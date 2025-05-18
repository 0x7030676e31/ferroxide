use std::io::Write;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{env, fmt, fs};

use super::get_path_to;
use super::tz_time;

const MAX_LINES: usize = 8192; // 2^13 lines
const MAX_LINES_THRESHOLD: usize = MAX_LINES + MAX_LINES / 2; // Threshold at which to truncate
const FILE: &str = "logs.txt";

struct Padded<T> {
    value: T,
    width: usize,
}

impl<T: fmt::Display> fmt::Display for Padded<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{: <width$}", self.value, width = self.width)
    }
}

static MAX_MODULE_WIDTH: AtomicUsize = AtomicUsize::new(0);
static LINE_COUNT: AtomicUsize = AtomicUsize::new(0);

fn max_target_width(target: &str) -> usize {
    let max_width = MAX_MODULE_WIDTH.load(Ordering::Relaxed);
    if max_width < target.len() {
        MAX_MODULE_WIDTH.store(target.len(), Ordering::Relaxed);
        target.len()
    } else {
        max_width
    }
}

/// Initializes the global application logger.
///
/// This function sets up a pretty-printed log output to stderr using
/// `pretty_env_logger`, and concurrently appends all log records to a
/// persistent file named `logs.txt` in the application’s base directory
/// (via `get_path_to(FILE)`).
///
/// On startup, it reads the existing file to initialize the line counter,
/// then writes a timestamped header. Each subsequent log record is formatted
/// with aligned level and module target fields, emitted to stderr, and
/// appended to the file. When the total lines exceed `MAX_LINES_THRESHOLD`,
/// the file is truncated to retain only the most recent `MAX_LINES` entries.
///
/// # Panics
///
/// - If creating or opening the log file on startup fails.  
/// - If the logger fails to initialize (`try_init()` error).  
///
/// # Examples
///
/// ```
/// // Must be called early in `main` before any `log` macros
/// logger::init();
/// log::info!("Application started");
/// ```
pub fn init() {
    let mut builder = pretty_env_logger::formatted_builder();
    let lock = Mutex::new(());

    let log_file = get_path_to(FILE);
    if let Ok(count) = fs::read_to_string(&log_file).map(|s| s.lines().count()) {
        LINE_COUNT.store(count + 1, Ordering::Relaxed);
    }

    let mut file = match fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
    {
        Ok(file) => file,
        Err(err) => panic!("Failed to open log file: {}", err),
    };

    let date = tz_time().format("%Y-%m-%d %H:%M:%S").to_string();
    let _ = writeln!(
        file,
        "=============================[ {} ]=============================",
        date
    );

    let res = builder
        .parse_filters(&env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .format(move |buf, record| {
            let target = record.target();
            let max_width = max_target_width(target);

            let style = buf.default_level_style(record.level());
            let level = style.value(Padded {
                value: record.level(),
                width: 5,
            });

            let mut style = buf.style();
            let target = style.set_bold(true).value(Padded {
                value: target,
                width: max_width,
            });

            let res = writeln!(buf, " {} {} > {}", level, target, record.args());

            let _lock = lock.lock().unwrap();
            let file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_file);

            let mut file = match file {
                Ok(file) => file,
                Err(err) => {
                    log::error!("Failed to open log file: {}", err);
                    return res;
                }
            };

            let date = tz_time().format("%Y-%m-%d %H:%M:%S").to_string();
            let _ = writeln!(file, "[{} {}] {} > {}", level, date, target, record.args());

            let line_count = LINE_COUNT.fetch_add(1, Ordering::Relaxed);
            if line_count < MAX_LINES_THRESHOLD {
                return res;
            }

            let lines = fs::read_to_string(&log_file);
            let lines = match lines {
                Ok(lines) => lines,
                Err(err) => {
                    log::error!("Failed to read log file: {}", err);
                    return res;
                }
            };

            let lines = lines
                .lines()
                .skip(line_count - MAX_LINES + 1)
                .chain(Some(""))
                .collect::<Vec<_>>();

            fs::write(&log_file, lines.join("\n")).unwrap_or_else(|err| {
                log::error!("Failed to write to log file: {}", err);
            });

            LINE_COUNT.store(MAX_LINES, Ordering::Relaxed);
            res
        })
        .try_init();

    if let Err(err) = res {
        panic!("Failed to initialize logger: {}", err);
    }
}

use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

static BASE_PATH: OnceLock<PathBuf> = OnceLock::new();
const BASE_PATH_NAME: &str = "ferroxide";

fn os_specific_path() -> PathBuf {
    let path = match env::consts::OS {
        "windows" => format!("{}\\{BASE_PATH_NAME}", env::var("USERPROFILE").unwrap()),
        "linux" | "macos" => format!("{}/.{BASE_PATH_NAME}", env::var("HOME").unwrap()),
        _ => panic!("Unsupported OS"),
    };
    PathBuf::from(path)
}

/// Returns the global base directory for the application.
///
/// On the first call, this function computes an OS-specific path:
/// - Windows: `%USERPROFILE%\ferroxide`  
/// - Linux/macOS: `$HOME/.ferroxide`  
///
/// If the directory does not already exist, it will be created. Any failure
/// during directory creation is logged at error level and causes a panic.
///
/// Subsequent calls simply return a reference to the same initialized `PathBuf`.
///
/// # Returns
///
/// A `'static` reference to the initialized `PathBuf` containing the base path.
///
/// # Panics
///
/// - If `env::consts::OS` is not one of `windows`, `linux`, or `macos`.
/// - If creating the directory fails.
///
/// # Examples
///
/// ```
/// let base = get_base_path();
/// println!("Application data directory: {}", base.display());
/// assert!(base.exists());
/// ```
pub fn get_base_path() -> &'static PathBuf {
    BASE_PATH.get_or_init(|| {
        let path = os_specific_path();
        if !path.exists() {
            if let Err(err) = fs::create_dir_all(&path) {
                log::error!("Failed to create base path: {}", err);
                panic!("Failed to create base path: {}", err);
            }
        }
        path
    })
}

/// Constructs a path under the global base directory.
///
/// This function retrieves the application's base directory via [`get_base_path`],
/// trims any leading `'/'` characters from the provided path fragment, and then
/// joins it to the base path.
///
/// # Parameters
///
/// * `path` – A string-like, relative path. Leading slashes will be ignored.
///
/// # Returns
///
/// A [`PathBuf`] pointing to the given resource under the application’s base directory.
///
/// # Examples
///
/// ```
/// // Suppose the base directory is "/home/user/.ferroxide"
/// let config = get_path_to("config/settings.toml");
/// assert_eq!(
///     config,
///     get_base_path().join("config/settings.toml")
/// );
/// ```
pub fn get_path_to<T: AsRef<str>>(path: T) -> PathBuf {
    let base_path = get_base_path();
    let path = path.as_ref().trim_start_matches('/');
    base_path.join(path)
}

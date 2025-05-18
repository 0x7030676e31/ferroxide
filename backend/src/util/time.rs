use chrono::Utc;
use chrono_tz::Tz;

const TIMEZONE: Tz = Tz::Europe__Warsaw;

/// Returns the current timestamp in seconds for the configured timezone.
///
/// This function obtains the current UTC time, converts it to the
/// `Europe/Warsaw` timezone, and then returns the Unix timestamp
/// (seconds since the Unix epoch) as a `u64`.
///
/// # Returns
///
/// A `u64` representing the current time in seconds in the `Europe/Warsaw` timezone.
///
/// # Examples
///
/// ```
/// let now_secs = tz_time_s();
/// assert!(now_secs > 0);
/// ```
pub fn tz_time_s() -> u64 {
    let utc = Utc::now().with_timezone(&TIMEZONE);
    utc.timestamp() as u64
}

/// Returns the current timestamp in milliseconds for the configured timezone.
///
/// This function obtains the current UTC time, converts it to the
/// `Europe/Warsaw` timezone, and then returns the Unix timestamp
/// (milliseconds since the Unix epoch) as a `u64`.
///
/// # Returns
///
/// A `u64` representing the current time in milliseconds in the `Europe/Warsaw` timezone.
///
/// # Examples
///
/// ```
/// let now_millis = tz_time_ms();
/// assert!(now_millis > 0);
/// ```
pub fn tz_time_ms() -> u64 {
    let utc = Utc::now().with_timezone(&TIMEZONE);
    utc.timestamp_millis() as u64
}

/// Returns the current date and time in the configured timezone.
///
/// This function obtains the current UTC time and converts it to the
/// `Europe/Warsaw` timezone, returning a `chrono::DateTime<Tz>` instance
/// for further manipulation or formatting.
///
/// # Returns
///
/// A `chrono::DateTime<Tz>` representing the current local time in the `Europe/Warsaw` timezone.
///
/// # Examples
///
/// ```
/// let local_dt = tz_time();
/// println!("Current local time: {}", local_dt);
/// ```
pub fn tz_time() -> chrono::DateTime<Tz> {
    Utc::now().with_timezone(&TIMEZONE)
}

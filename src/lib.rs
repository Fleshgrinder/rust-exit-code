//! Common exit codes for applications.
//!
//! # Example
//! ```
//! extern crate exit_code;
//!
//! ::std::process::exit(exit_code::SUCCESS);
//! ```

/// Successful termination.
pub const SUCCESS: i32 = 0;

/// Unsuccessful termination. This is a catch-all error code that should only
/// be used by processes if the reason for the failure is unknown.
pub const FAILURE: i32 = 1;

/// The command was used incorrectly, e.g., with the wrong number of arguments,
/// a bad flag, a bad syntax in a parameter, or whatever.
pub const USAGE_ERROR: i32 = 64;

/// The input data was incorrect in some way. This should only be used for
/// user’s data and not system file.
pub const DATA_ERROR: i32 = 65;

/// An input file (not a system file) did not exist or was not readable. This
/// could also include errors like “No message” to a mailer (if it cared to
/// catch it).
pub const NO_INPUT: i32 = 66;

/// The user specified did not exist. This might be used for mail addresses or
/// remote logins.
pub const NO_USER: i32 = 67;

/// The host specified did not exist. This is used in mail addresses of network
/// requests.
pub const NO_HOST: i32 = 68;

/// A service is unavailable. This can occur if a support program or file does
/// not exist. This can also be used as a catchall message when something you
/// wanted to do doesn’t work, but you don’t know why.
pub const SERVICE_UNAVAILABLE: i32 = 69;

/// An internal software error has been detected. This should be limited to
/// non-operating system related errors.
pub const SOFTWARE_ERROR: i32 = 70;

/// An operating system error has been detected. This is intended to be used
/// for such things as “cannot fork”, “cannot create pipe”, or the like. It
/// includes things like `getuid` returning a user that does not exist in the
/// `passwd` file.
pub const OS_ERROR: i32 = 71;

/// Some system file (e.g., `/etc/passwd`, `/var/run/utmp`, etc.) does not
/// exist, cannot be opened, or has some sort of error (e.g., syntax error).
pub const OS_FILE_ERROR: i32 = 72;

/// A (user specified) output file cannot be created.
pub const CANNOT_CREATE: i32 = 73;

/// An error occurred while doing I/O on some file.
pub const IO_ERROR: i32 = 74;

/// Temporary failure, indicating something that is not really an error. In
/// `sendmail`, this means that a mailer (e.g.) could not create a connection,
/// and the request should be reattempted later.
pub const TEMPORARY_FAILURE: i32 = 75;

/// The remote system returned something that was “not possible” during a
/// protocol exchanged.
pub const PROTOCOL_ERROR: i32 = 76;

/// Insufficient permissions to perform an operation. This is not intended for
/// file system problems, which should use `NO_INPUT` or `CANNOT_CREATE`, but
/// rather for higher level permissions.
pub const NO_PERMISSION: i32 = 77;

/// Something was found in an unconfigured or misconfigured state.
pub const CONFIG_ERROR: i32 = 78;

/// Check if the given exit code is reserved.
///
/// Reserved exit codes are the ones that have a special meaning in a shell:
///
/// |-----------|----------------------------------------------------------|
/// | Exit Code | Meaning                                                  |
/// |-----------|----------------------------------------------------------|
/// | 0         | Success                                                  |
/// | 1         | Catchall for general errors                              |
/// | 2         | Misuse of shell built-ins                                |
/// | 64        | Usage Error                                              |
/// | 65        | Data Error                                               |
/// | 66        | No Input                                                 |
/// | 67        | No User                                                  |
/// | 68        | No Host                                                  |
/// | 69        | Service Unavailable                                      |
/// | 70        | Software Error                                           |
/// | 71        | OS Error                                                 |
/// | 72        | OS File Error                                            |
/// | 73        | Cannot Create                                            |
/// | 74        | IO Error                                                 |
/// | 75        | Temporary Failure                                        |
/// | 76        | Protocol Error                                           |
/// | 77        | No Permission                                            |
/// | 78        | Config Error                                             |
/// | 126       | Command invoked cannot execute                           |
/// | 127       | Command not found                                        |
/// | 128       | Invalid argument to `exit`                               |
/// | 128–137   | Fatal error signal (`kill -n` where `n` is added to 128) |
/// | 256+      | Exit status out of range                                 |
/// |-----------|----------------------------------------------------------|
pub fn is_reserved(c: i32) -> bool {
    (0 <= c && c <= 2) || (64 <= c && c <= 78) || (126 <= c && c <= 137)
}

/// Check if the given exit code is valid.
///
/// Valid exit codes are in [0..256].
pub fn is_valid(c: i32) -> bool {
    0 <= c && c <= 255
}

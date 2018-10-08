# Rust Exit Codes
Commonly used exit codes for usage in applications.

Using appropriate exit codes greatly helps users of applications to assert them
in their own scripts and take appropriate actions based on the returned code.
Simply using the catchall exit code _1_ all the time does not help others in
any way. Unless there is only a single source of error in the complete
application, which is very unlikely.

## Installation
Add the following to your `Cargo.toml` file:

```toml
[dependencies]
exit-code = "1.0.0"
```

## Usage
```rust
extern crate exit_code;

fn main() {
    use std::process::exit;
    use exit_code::SUCCESS;

    println!("Hello, World!");
    exit(SUCCESS);
}
```

This crate provides two additional functions that allow validation of exit
codes:

- `is_reserved(exit_code) -> bool` can be used to check if a given exit code
  has a reserved meaning in a shell, refer to the API documentation for the
  meaning of the reserved exit codes.
- `is_valid(exit_code) -> bool` can be used to check if a given exit code is
  within the range `[0..256]` (anything below or beyond silently overflows).

## References
- Advanced Bash-Scripting Guide: [Appendix E. Exit Codes With Special Meanings](http://tldp.org/LDP/abs/html/exitcodes.html)
- FreeBSD Man Pages: [`man sysexits(3)`](https://www.freebsd.org/cgi/man.cgi?query=sysexits)

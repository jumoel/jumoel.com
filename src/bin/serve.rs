use std::process::Command;

fn main() -> std::io::Result<()> {
    let status = Command::new(env!("CARGO_BIN_FILE_MINISERVE"))
        .args(std::env::args_os().skip(1))
        .status()?;

    std::process::exit(status.code().unwrap_or(1))
}

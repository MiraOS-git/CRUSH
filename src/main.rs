use std::ffi::OsString;
use std::process::ExitCode;

mod shell;
mod repl;

fn main() -> ExitCode {
	let args: Vec<OsString> = std::env::args_os().collect();
	let mut shell = shell::Shell::new();
	ExitCode::from(shell.run(&args) as u8)
}

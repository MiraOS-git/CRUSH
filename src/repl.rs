use std::io::{self, BufRead, Write};

use crate::shell::Shell;

pub fn run(shell: &mut Shell) -> i32 {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    loop {
	print!("CRUSH$ ");
	io::stdout().flush().unwrap();
	let mut line: Vec<u8> = Vec::new();
	let n = match stdin.read_until(b'\n', &mut line) {
	   Ok(n) => n,
	   Err(e) => {
		eprintln!("crush: read error: {e}");
		return 1
	   }
	};
	if n == 0 {
	   println!();
	   return shell.exitcode();
	}
	if line.last() == Some(&b'\n') {
		line.pop();
	}
	println!("got: {}", String::from_utf8_lossy(&line));
    }
}

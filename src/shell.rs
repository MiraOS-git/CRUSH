use std::collections::{HashMap, HashSet};
use std::env::vars_os;
use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;

pub struct Shell {
	exitcode: i32,
	vars: HashMap<Vec<u8>, Vec<u8>>,
	exported: HashSet<Vec<u8>>,
	params: Vec<Vec<u8>>,
}

impl Shell {
	pub fn new() -> Self {
		let mut vars: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
		let mut exported: HashSet<Vec<u8>> = HashSet::new();
		for (name, value) in vars_os() {
			let name = name.into_vec();
			let value = value.into_vec();
			exported.insert(name.clone());
			vars.insert(name, value);
		}
		Shell {
			exitcode: 0,
			vars,
			exported,
			params: Vec::new(),
		}
	}
	pub fn run(&mut self, args: &[OsString]) -> i32 {
		self.params = args.iter().cloned().map(OsStringExt::into_vec).collect();
		crate::repl::run(self)
	}
	pub fn exitcode(&self) -> i32 {
		self.exitcode
	}
}

impl Default for Shell {
	fn default() -> Self {
		Self::new()
	}
}

#![crate_id = "js.rs"]
#![comment = "Javascript parsing and execution command line tool"]
#![license = "MIT"]
#![crate_type = "bin"]

#![deny(non_uppercase_statics)]
#![deny(missing_doc)]
#![deny(unnecessary_parens)]
#![deny(unrecognized_lint)]
#![deny(unreachable_code)]
#![deny(unnecessary_allocation)]
#![deny(unnecessary_typecast)]
#![deny(unnecessary_allocation)]
#![deny(uppercase_variables)]
#![deny(non_camel_case_types)]
#![deny(unused_must_use)]

//! A Javascript execution command line tool

extern crate js;
extern crate syntax = "js_syntax";
extern crate getopts;
extern crate collections;
/// Interactive mode
pub mod interactive;
/// Unit test mode
pub mod tests;
/// Script runner mode
pub mod runner;
/// The main function
pub fn main() {
	let opts = [
		getopts::optflag("h", "help", "Show this message"),
		getopts::optflag("v", "verbose", "Enable verbose output"),
		getopts::optflag("t", "tests", "Run tests"),
		getopts::optflag("i", "interactive", "Run in interactive mode"),
		getopts::optopt("s", "source-code", "Run some Javascript code", "The path to the source code")
	];
	let m = getopts::getopts(std::os::args().as_slice(), opts).unwrap();
	match m.opt_str("s") {
		Some(path) => {
			runner::Runner::new(m).run(path)
		},
		None if m.opt_present("h") => {
			println!("{}", getopts::usage("Usage: js.rs [OPTIONS] [INPUT]", opts));
		},
		None if m.opt_present("t") || (m.free.len() >= 2 && m.free.get(1).as_slice() == "test") => {
			tests::Tests::new(m).run();
		},
		None if m.opt_present("i") || (m.free.len() >= 2 && m.free.get(1).as_slice() == "interactive") => {
			interactive::Interactive::new(m).run();
		},
		None if m.free.len() >= 2 => {
			runner::Runner::new(m.clone()).run(m.free.get(1).clone());
		},
		None => {
			println!("{}", getopts::short_usage("Usage: js.rs [OPTIONS] [INPUT]", opts));
		}
	}
}
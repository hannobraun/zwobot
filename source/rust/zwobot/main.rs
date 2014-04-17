extern crate libc;

extern crate inotify;


use std::io::Process;
use std::os;

use inotify::INotify;


fn main() {
	let args = os::args();

	if args.len() < 2 {
		print!("Usage: {} COMMAND FILE [FILE...]\n", args[0]);
		return;
	}

	let command = args[1].to_owned();
	let files   = args.slice_from(2);

	let inotify = match INotify::init() {
		Ok(inotify) => inotify,
		Err(error)  => fail!(error)
	};

	for file in files.iter() {
		match inotify.add_watch(*file, inotify::ffi::IN_MODIFY) {
			Ok(watch)  => watch,
			Err(error) => fail!(error)
		};
	}

	loop {
		match inotify.event() {
			Ok(_)      => run_command(command.words().map(|x| x.to_owned()).collect()),
			Err(error) => {
				print!("{}", error);
				break;
			}
		}
	}

	match inotify.close() {
		Ok(_)      => (),
		Err(error) => fail!(error)
	}
}

fn run_command(command: ~[~str]) {
	let executable = command.head().unwrap();
	let args       = command.tail();

	let mut process = match Process::new(*executable, args) {
		Ok(process) => process,
		Err(error)  => fail!("{}", error)
	};

	assert!(process.wait().success());

	print!("{}", process.stdout.take().expect("no stdout").read_to_str().unwrap());
	print!("{}", process.stderr.take().expect("no stderr").read_to_str().unwrap());
}

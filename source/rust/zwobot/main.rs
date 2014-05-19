extern crate libc;
extern crate time;

extern crate inotify;


use std::io::Process;
use std::os;

use inotify::INotify;


fn main() {
	let args = os::args();

	if args.len() < 2 {
		print!("Usage: {} COMMAND FILE [FILE...]\n", args.get(0));
		return;
	}

	let command = args.get(1).to_owned();
	let files   = args.slice_from(2);

	let command_words: Vec<~str> =
		command.words().map(|x| x.to_owned()).collect();

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
			Ok(_) => {
				print!("\n\n\n=== {} START {}\n",
					time::now().rfc3339(),
					command);
				run_command(&command_words);
				print!("=== {} FINISH {}\n",
					time::now().rfc3339(),
					command);
			},
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

fn run_command(command: &Vec<~str>) {
	let executable = command.get(0);
	let args       = command.tail();

	let mut process = match Process::new(*executable, args) {
		Ok(process) => process,
		Err(error)  => fail!("{}", error)
	};

	let _ = process.wait();

	print!("{}", process.stdout.take().expect("no stdout").read_to_str().unwrap());
	print!("{}", process.stderr.take().expect("no stderr").read_to_str().unwrap());
}

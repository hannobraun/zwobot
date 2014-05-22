extern crate libc;
extern crate time;

extern crate inotify;


use std::os;

use inotify::INotify;


mod runner;


fn main() {
	let args = os::args();

	if args.len() < 2 {
		print!("Usage: {} COMMAND FILE [FILE...]\n", args.get(0));
		return;
	}

	let command = args.get(1).to_owned();
	let files   = args.slice_from(2);

	let runner = runner::new(command.clone());

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
			Ok(event) => {
				print!("\n\n\n=== {} START {}\n",
					time::now().rfc3339(),
					command);
				runner.send(event);
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

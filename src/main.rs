extern crate libc;
extern crate time;

extern crate inotify;


use std::io;
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
		match inotify.add_watch(file.as_slice(), inotify::ffi::IN_MODIFY) {
			Ok(watch)  => watch,
			Err(error) => fail!(error)
		};
	}

	runner.send(());

	read_manual_input(runner.clone());

	loop {
		match inotify.event() {
			Ok(_) =>
				runner.send(()),

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

fn read_manual_input(runner: Sender<()>) {
	spawn(proc() {
		for _ in io::stdin().lines() {
			runner.send(())
		}
	})
}

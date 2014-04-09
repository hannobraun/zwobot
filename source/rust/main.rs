extern crate libc;

use std::os;

use inotify::INotify;

mod inotify;


fn main() {
	let args = os::args();

	let command = args[1].to_owned();
	let files   = args.slice_from(2);

	print_files(files);
	print_command(command);

	let inotify = match INotify::init() {
		Ok(inotify) => inotify,
		Err(error)  => fail!(error)
	};

	let mut last_watch = 0;
	for file in files.iter() {
		last_watch = match inotify.add_watch(*file, 1) {
			Ok(watch)      => watch,
			Err(error) => fail!(error)
		};
	}

	match inotify.rm_watch(last_watch) {
		Ok(_)      => (),
		Err(error) => fail!(error)
	}

	loop {
		match inotify.event() {
			Ok(_)      => (),
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

fn print_command(command: &str) {
	print!("-> {}\n", command);
}

fn print_files(files: &[~str]) {
	for file in files.iter() {
		print!("{}\n", *file);
	}
}

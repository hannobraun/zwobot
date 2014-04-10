extern crate libc;

extern crate inotify;


use std::os;

use inotify::INotify;


fn main() {
	let args = os::args();

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
			Ok(event)  => print!("{}\n", event),
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

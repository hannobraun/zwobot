extern crate libc;
extern crate time;

extern crate inotify;


use std::io;
use std::io::fs;
use std::os;

use inotify::INotify;


mod runner;


fn main() {
	let args = os::args();

	if args.len() < 2 {
		print!("Usage: {} COMMAND FILE [FILE...]\n", args.get(0));
		return;
	}

	let command = args.get(1).to_str();
	let files   = args.slice_from(2);

	let runner = runner::new(command.clone());

	let inotify = match INotify::init() {
		Ok(inotify) => inotify,
		Err(error)  => fail!(error)
	};

	let paths: Vec<Path> = files
		.iter()
		.map(|file|
			Path::new(file.as_slice()))
		.collect();

	add_files(&inotify, paths.as_slice());

	runner.send(());

	read_manual_input(runner.clone());

	loop {
		match inotify.event() {
			Ok(_) =>
				runner.send(()),

			Err(error) =>
				fail!("Error retrieving inotify event: {}", error)
		}
	}
}

fn add_files(inotify: &INotify, files: &[Path]) {
	for path in files.iter() {
		if path.is_dir() {
			match fs::readdir(path) {
				Ok(paths)  => add_files(inotify, paths.as_slice()),
				Err(error) => fail!(error)
			}
		}
		else {
			match inotify.add_watch(path, inotify::ffi::IN_MODIFY) {
				Ok(_)      => (),
				Err(error) => fail!(error)
			};
		}
	}
}

fn read_manual_input(runner: Sender<()>) {
	spawn(proc() {
		for _ in io::stdin().lines() {
			runner.send(())
		}
	})
}

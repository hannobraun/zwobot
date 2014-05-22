use std::io::Command;

use inotify::ffi::inotify_event;


pub fn new(command_str: ~str) -> Sender<inotify_event> {
	let command_words: Vec<~str> =
		command_str.words().map(|x| x.to_owned()).collect();

	let executable = command_words.get(0).clone();
	let args       = command_words.tail();

	let mut command = Command::new(executable);
	command.args(args);

	let (sender, receiver) = channel();

	spawn(proc() {
		loop {
			let _ = receiver.recv();

			let mut process = match command.spawn() {
				Ok(process) => process,
				Err(error)  => fail!("{}", error)
			};

			let _ = process.wait();

			print!("{}", process.stdout.take().expect("no stdout").read_to_str().unwrap());
			print!("{}", process.stderr.take().expect("no stderr").read_to_str().unwrap());
		}
	});

	sender
}

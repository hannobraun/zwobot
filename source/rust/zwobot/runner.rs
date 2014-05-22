use std::comm::{
	Disconnected,
	Empty
};
use std::io::Command;
use std::io::timer;

use inotify::ffi::inotify_event;


pub fn new(command_str: ~str) -> Sender<inotify_event> {
	let command_words: Vec<~str> =
		command_str.words().map(|x| x.to_owned()).collect();

	let executable = command_words.get(0).clone();
	let args       = command_words.tail().to_owned();

	let (sender, receiver) = channel();

	spawn(proc() {
		let mut waiting_command = None;

		loop {
			let _ = receiver.recv();

			match waiting_command {
				Some(sender) => drop(sender),
				None         => ()
			}

			waiting_command = Some(run(executable.clone(), args));
		}
	});

	sender
}

fn run(executable: ~str, args: &[~str]) -> Sender<()> {
	let mut command = Command::new(executable);
	command.args(args);

	let (sender, receiver) = channel();

	spawn(proc() {
		// Don't execute the command right away. Another event that triggers the
		// same command might follow almost immediately. In that case, this run
		// is going to be cancelled.
		timer::sleep(50);

		match receiver.try_recv() {
			Ok(_) => (),

			Err(error) => match error {
				Empty        => (),
				Disconnected => {
					// We're cancelled! Return!
					return;
				}
			}
		}

		let mut process = match command.spawn() {
			Ok(process) => process,
			Err(error)  => fail!("{}", error)
		};

		let _ = process.wait();

		print!("{}", process.stdout.take().expect("no stdout").read_to_str().unwrap());
		print!("{}", process.stderr.take().expect("no stderr").read_to_str().unwrap());
	});

	sender
}

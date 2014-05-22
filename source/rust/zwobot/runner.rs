use std::comm::{
	Disconnected,
	Empty
};
use std::io::{
	BufferedReader,
	Command,
	PipeStream
};
use std::io::timer;
use time;

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

		print!("\n\n\n=== {} START {}\n", time::now().rfc3339(), command);

		printer(process.stdout.take().expect("no stdout"));
		printer(process.stderr.take().expect("no stderr"));

		let _ = process.wait();

		print!("=== {} FINISH {}\n", time::now().rfc3339(), command);
	});

	sender
}

fn printer(pipe: PipeStream) {
	spawn(proc() {
		let mut reader = BufferedReader::new(pipe);
		for l in reader.lines() {
			match l {
				Ok(line)   => print!("{}", line),
				Err(error) => fail!("{}", error)
			}
		}
	});
}

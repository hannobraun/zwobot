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


pub fn new(command_str: ~str) -> Sender<()> {
	let command_words: Vec<~str> =
		command_str.words().map(|x| x.to_owned()).collect();

	let executable = command_words.get(0).clone();
	let args       = command_words.tail().to_owned();

	let runner = runner(executable, args);
	deduplicator(runner)
}

fn deduplicator(runner: Sender<()>) -> Sender<()> {
	let (sender, receiver) = channel();

	spawn(proc() {
		loop {
			let _ = receiver.recv();

			// Don't pass the event on immediately. There might be another one
			// directly after that. We're only going to pass one of them.
			loop {
				timer::sleep(50);

				match receiver.try_recv() {
					Ok(_) => {
						// Another event. Do nothing, wait again.
						()
					},

					Err(error) => match error {
						Empty => {
							// No event followed. break out of the loop and pass
							// a single event on.
							break;
						},
						Disconnected =>
							fail!("Channel unexpectedly disconnected")
					}
				}
			}

			runner.send(());
		}
	});

	sender
}

fn runner(executable: ~str, args: &[~str]) -> Sender<()> {
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

			print!("\n\n\n=== {} START {}\n", time::now().rfc3339(), command);

			print(
				"stdout".to_owned(),
				process.stdout.take().expect("no stdout"));
			print(
				"stderr".to_owned(),
				process.stderr.take().expect("no stderr"));

			let _ = process.wait();

			print!("=== {} FINISH {}\n", time::now().rfc3339(), command);
		}
	});

	sender
}

fn print(prefix: ~str, pipe: PipeStream) {
	spawn(proc() {
		let mut reader = BufferedReader::new(pipe);
		for l in reader.lines() {
			match l {
				Ok(line)   => print!("[{}] {}", prefix, line),
				Err(error) => fail!("{}", error)
			}
		}
	});
}

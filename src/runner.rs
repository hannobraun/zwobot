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

use printer::printer;


pub fn new(command_str: String) -> Sender<()> {
	let command_words: Vec<String> =
		command_str.as_slice().words().map(|x| x.to_str()).collect();

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

fn runner(executable: String, args: &[String]) -> Sender<()> {
	let mut command = Command::new(executable);
	command.args(args);

	let (sender, receiver) = channel();

	let printer = printer();

	spawn(proc() {
		loop {
			// If multiple events have accumulated during the run, it should
			// only trigger one additional run.
			// Please note that this is different from deduplication:
			// - Deduplication makes sure that subsequent events with little
			//   delay between them only trigger a single run.
			// - Another event that occurs during a run _should_ trigger a
			//   second run.
			// - However, multiple events during a run should only trigger a
			//   second run, not a third, fourth etc.
			let mut no_events_received = true;
			loop {
				match receiver.try_recv() {
					Ok(_) =>
						no_events_received = false,

					Err(error) => match error {
						Empty =>
							break,
						Disconnected =>
							fail!("Channel unexpectedly disconnected")
					}
				}
			}

			if no_events_received {
				let _ = receiver.recv();
			}

			let mut process = match command.spawn() {
				Ok(process) => process,
				Err(error)  => fail!("{}", error)
			};

			printer.send(format!(
				"\n\n\n=== {} START {}\n", time::now().rfc3339(), command));

			print(
				"stdout".to_str(),
				process.stdout.take().expect("no stdout"),
				printer.clone());
			print(
				"stderr".to_str(),
				process.stderr.take().expect("no stderr"),
				printer.clone());

			let _ = process.wait();

			printer.send(format!(
				"=== {} FINISH {}\n", time::now().rfc3339(), command));
		}
	});

	sender
}

fn print(prefix: String, pipe: PipeStream, printer: Sender<String>) {
	spawn(proc() {
		let mut reader = BufferedReader::new(pipe);
		for l in reader.lines() {
			match l {
				Ok(line)   => printer.send(format!("[{}] {}", prefix, line)),
				Err(error) => fail!("{}", error)
			}
		}
	});
}

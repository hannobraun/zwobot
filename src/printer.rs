use std::comm::channel;


pub fn printer() -> Sender<String> {
	let (sender, receiver) = channel();

	spawn(proc() {
		loop {
			let text = receiver.recv();

			print!("{}", text);
		}
	});

	sender
}

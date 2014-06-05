use std::comm::channel;
use term;


pub struct Output(pub Vec<term::attr::Attr>, pub String);


pub fn printer() -> Sender<Output> {
	let (sender, receiver) = channel::<Output>();

	spawn(proc() {
		let mut stdout = match term::stdout() {
			Some(terminal) =>
				terminal,
			None =>
				fail!("Failed to retrieve stdout terminal")
		};

		loop {
			let Output(attributes, text) = receiver.recv();

			for &attr in attributes.iter() {
				let _ = stdout.attr(attr);
			}
			let _ = write!(stdout, "{}", text);
			let _ = stdout.reset();
		}
	});

	sender
}

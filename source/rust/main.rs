use std::os;

mod inotify;


fn main() {
	let args = os::args();

	let command = args[1].to_owned();
	let files   = args.slice_from(2);

	print_files(files);
	print_command(command);
}

fn print_command(command: &str) {
	print!("-> {}\n", command);
}

fn print_files(files: &[~str]) {
	for file in files.iter() {
		print!("{}\n", *file);
	}
}

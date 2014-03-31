use std::c_str::CString;
use std::libc;
use std::libc::{
	c_char,
	c_int,
	c_void,
	size_t,
	ssize_t,
	uint32_t };
use std::mem;
use std::os;
use std::ptr;


extern {
	fn inotify_init1(flags: c_int) -> c_int;
	fn inotify_add_watch(fd: c_int, pathname: *c_char, mask: uint32_t) -> c_int;
	fn inotify_rm_watch(fd: c_int, wd: c_int) -> c_int;
	fn read(fd: c_int, buf: *c_void, count: size_t) -> ssize_t;
	fn close(fd: c_int) -> c_int;
}


pub struct INotify {
	fd: c_int
}

type Watch = c_int;

struct Event {
	wd    : c_int,
	mask  : uint32_t,
	cookie: uint32_t,
	len   : uint32_t,
	name  : *c_char
}

impl INotify {
	pub fn init() -> Result<INotify, ~str> {
		INotify::init_with_flags(0)
	}

	pub fn init_with_flags(flags: int) -> Result<INotify, ~str> {
		let fd = unsafe { inotify_init1(flags as c_int) };

		match fd {
			-1 => Err(last_error()),
			_  => Ok(INotify { fd: fd })
		}
	}

	pub fn add_watch(&self, path_name: &str, mask: u32) -> Result<Watch, ~str> {
		let wd = unsafe {
			let c_path_name = path_name.to_c_str().unwrap();
			inotify_add_watch(self.fd, c_path_name, mask)
		};

		match wd {
			-1 => Err(last_error()),
			_  => Ok(wd)
		}
	}

	pub fn rm_watch(&self, watch: Watch) -> Result<(), ~str> {
		let result = unsafe { inotify_rm_watch(self.fd, watch) };
		match result {
			0  => Ok(()),
			-1 => Err(last_error()),
			_  => Err(format!(
				"unexpected return code from inotify_rm_watch ({})", result))
		}
	}

	pub fn event(&self) -> Result<Event, ~str> {
		let event = Event {
			wd    : 0,
			mask  : 0,
			cookie: 0,
			len   : 0,
			name  : ptr::null()
		};

		let event_size = mem::size_of::<Event>();

		let result = unsafe {
			read(
				self.fd,
				&event as *Event as *c_void,
				event_size as u64)
		};

		print!("{} {}\n", result, event_size);

		match result {
			0  => Err(~"end of file"),
			-1 => Err(last_error()),
			_  => Ok(event)
		}
	}

	pub fn close(&self) -> Result<(), ~str> {
		let result = unsafe { close(self.fd) };
		match result {
			0 => Ok(()),
			_ => Err(last_error())
		}
	}
}


fn last_error() -> ~str {
	unsafe {
		let c_error = libc::strerror(os::errno() as i32);
		CString::new(c_error, false)
			.as_str()
			.expect("failed to convert C error message into Rust string")
			.to_owned()
	}
}

use libc::{
	c_char,
	c_int,
	c_void,
	size_t,
	ssize_t,
	uint32_t };


extern {
	pub fn inotify_init1(flags: c_int) -> c_int;
	pub fn inotify_add_watch(fd: c_int, pathname: *c_char, mask: uint32_t) -> c_int;
	pub fn inotify_rm_watch(fd: c_int, wd: c_int) -> c_int;
	pub fn read(fd: c_int, buf: *c_void, count: size_t) -> ssize_t;
	pub fn close(fd: c_int) -> c_int;
}


pub static IN_CLOEXEC : c_int = 0o2000000;
pub static IN_NONBLOCK: c_int = 0o4000;


#[allow(non_camel_case_types)]
#[allow(raw_pointer_deriving)]
#[deriving(Show)]
pub struct inotify_event {
	pub wd    : c_int,
	pub mask  : uint32_t,
	pub cookie: uint32_t,
	pub len   : uint32_t,
	pub name  : *c_char
}

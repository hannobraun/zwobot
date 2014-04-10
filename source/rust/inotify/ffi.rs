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

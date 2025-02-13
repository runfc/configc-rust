use crate::manager::Manager;
use crate::errors::Error;

use libc::{
    c_char,
    c_int,
    c_uint,
    mode_t,
};
use std::ffi::CString;

#[repr(C)]
pub struct file_t {
    pub path: *const c_char,
    pub desired: *mut c_char,
    pub mode_t: c_uint,

    opts: c_uint,
    act_value: *mut c_char,
}

extern "C" {
    pub fn file_init(path: *const c_char, content: *const c_char, mode: mode_t, opts: c_uint) -> *mut file_t;
    pub fn file_get(file: *mut file_t) -> c_int;
    pub fn file_diff(file: *mut file_t) -> c_int;
    pub fn file_apply(file: *mut file_t) -> c_int;
    pub fn file_free(file: *mut file_t);
}

#[derive(Debug)]
pub struct File {
    path: String,
    desired: String,
    mode: mode_t,
    options: u32,
}

impl File {
    pub fn new(path: &str, desired: &str, mode: u32, options: u32) -> Self {
	Self{
	    path: path.to_string(),
	    desired: desired.to_string(),
	    mode: mode as mode_t,
	    options: options,
	}
    }
}

/*
 * Explain why this is "safe"?
 */
unsafe impl Send for File {}
unsafe impl Sync for File {}

impl Manager for File {

    /*
     * ensure attempts to apply the desired state of the object is
     * applied to system.
     */
    fn ensure(&self) -> Result<(), Error> {
	let cpath = CString::new(self.path.clone()).expect("CString::new failed for key");
	let ccontent = CString::new(self.desired.clone()).expect("CString::New failed for value");

	unsafe {
	    let file = file_init(cpath.as_ptr(), ccontent.as_ptr(), self.mode, self.options);
	    if file_get(file) < 0 {
		file_free(file);

		let errmsg = format!("Error when getting information from file: {}", self.path);
		return Err(Error::File(errmsg));
	    }

	    if file_diff(file) != 0 {
		if file_apply(file) < 0 {
		    file_free(file);

		    let errmsg = format!("Unable to apply information for file: {}", self.path);
		    return Err(Error::File(errmsg));
		}
	    }
	}

	Ok(())
    }

    fn has_drifted(&self) -> Result<bool, Error> {
	let cpath = CString::new(self.path.clone()).expect("CString::new failed for key");
	let ccontent = CString::new(self.desired.clone()).expect("CString::New failed for value");

	let is_diff = unsafe {
	    let file = file_init(cpath.as_ptr(), ccontent.as_ptr(), self.mode, self.options);

	    if file_get(file) < 0 {
		file_free(file);

		let errmsg = format!("Unable to get information for file: {}", self.path);
		return Err(Error::File(errmsg));
	    }

	    let diff = file_diff(file) != 0;
	    diff
	};

	Ok(is_diff)
    }
}

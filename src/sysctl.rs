use crate::manager::Manager;
use crate::errors::Error;

use libc::{
    c_char,
    c_int,
};
use std::ffi::CString;

#[repr(C)]
pub struct sysctl_t {
    pub key: *const c_char,
    pub desired: *const c_char,

    path: *mut c_char,
    act_value: *mut c_char,
}

extern "C" {
    pub fn sysctl_init(key: *const c_char, value: *const c_char) -> *mut sysctl_t;
    pub fn sysctl_get(sysctl: *mut sysctl_t) -> c_int;
    pub fn sysctl_diff(sysctl: *mut sysctl_t) -> c_int;
    pub fn sysctl_apply(sysctl: *mut sysctl_t) -> c_int;
    pub fn sysctl_free(sysctl: *mut sysctl_t);
}

#[derive(Debug)]
pub struct Sysctl {
    key: String,
    value: String,
}

impl Sysctl {
    pub fn new(key: &str, value: &str) -> Self {
	Self{
	    key: key.to_string(),
	    value: value.to_string()
	}
    }
}

/*
 * Explain why this is "safe"?
 */
unsafe impl Send for Sysctl {}
unsafe impl Sync for Sysctl {}

impl Manager for Sysctl {

    /*
     * ensure attempts to apply the desired state of the object is
     * applied to system.
     */
    fn ensure(&self) -> Result<(), Error> {
	let ckey = CString::new(self.key.clone()).expect("CString::new failed for key");
	let cvalue = CString::new(self.value.clone()).expect("CString::New failed for value");

	unsafe {
	    let sysctl = sysctl_init(ckey.as_ptr(), cvalue.as_ptr());
	    if sysctl_get(sysctl) < 0 {
		sysctl_free(sysctl);

		let errmsg = format!("Unable to get information from sysctl: {}", self.key);
		return Err(Error::Sysctl(errmsg));
	    }

	    if sysctl_diff(sysctl) != 0 {
		if sysctl_apply(sysctl) < 0 {
		    sysctl_free(sysctl);

		    let errmsg = format!("Unable to apply sysctl: {}", self.key);
		    return Err(Error::Sysctl(errmsg));
		}
	    }
	}
	Ok(())
    }

    fn has_drifted(&self) -> Result<bool, Error> {
	let ckey = CString::new(self.key.clone()).expect("CString::new failed for key");
	let cvalue = CString::new(self.value.clone()).expect("CString::New failed for value");

	let is_diff = unsafe {
	    let sysctl = sysctl_init(ckey.as_ptr(), cvalue.as_ptr());

	    let rc = sysctl_get(sysctl);
	    if rc < 0 {
		sysctl_free(sysctl);

		let errmsg = format!("Unable to get information for sysctl {} with error: {}", self.key, rc);
		return Err(Error::Sysctl(errmsg));
	    }

	    sysctl_diff(sysctl) != 0
	};
	Ok(is_diff)
    }
}

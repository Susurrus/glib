// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::str;
use std::ffi::CStr;
use std::fmt::{self, Formatter, Debug, Display};
use std::error;
use glib_ffi::{self, GQuark};
use glib_container::GlibContainer;
use translate::ToGlibPtr;

pub struct Error {
    pointer: *mut glib_ffi::GError
}

impl Error {
    pub fn new_literal(domain: GQuark, code: i32, message: &str) -> Option<Error> {
        let tmp_pointer = unsafe {
            glib_ffi::g_error_new_literal(domain, code, message.to_glib_none().0)
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(Error{pointer: tmp_pointer})
        }
    }

    pub fn release(&mut self) -> () {
        if !self.pointer.is_null() {
            unsafe { glib_ffi::g_error_free(self.pointer) };
            self.pointer = ::std::ptr::null_mut();
        }
    }

    pub fn matches(&self, domain: GQuark, code: i32) -> bool {
        match unsafe { glib_ffi::g_error_matches(self.pointer, domain, code) } {
            glib_ffi::GFALSE => false,
            _ => true
        }
    }

    pub fn set(&mut self, domain: GQuark, code: i32, message: &str) -> () {
        unsafe {
            glib_ffi::g_set_error_literal(&mut self.pointer, domain, code, message.to_glib_none().0)
        }
    }

    pub fn propagate(&mut self, other: &Error) -> () {
        unsafe { glib_ffi::g_propagate_error(&mut self.pointer, other.pointer) }
    }

    pub fn message(&self) -> &str {
        let c_str = unsafe { CStr::from_ptr((*self.pointer).message) };
        str::from_utf8(c_str.to_bytes()).unwrap()
    }
}

impl Clone for Error {
    fn clone(&self) -> Error {
        let tmp_pointer = unsafe { glib_ffi::g_error_copy(self.pointer) };

        if tmp_pointer.is_null() {
            Error {
                pointer: ::std::ptr::null_mut()
            }
        } else {
            GlibContainer::wrap(tmp_pointer)
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.message()
    }
}

impl Drop for Error {
    fn drop(&mut self) {
        self.release();
    }
}

impl GlibContainer<*mut glib_ffi::GError> for Error {
    fn wrap(pointer: *mut glib_ffi::GError) -> Error {
        Error {
            pointer: pointer
        }
    }

    fn unwrap(&self) -> *mut glib_ffi::GError {
        self.pointer
    }
}

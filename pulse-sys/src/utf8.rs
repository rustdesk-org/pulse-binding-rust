// Copyright 2017 Lyndon Brown
//
// This file is part of the PulseAudio Rust language linking library.
//
// Licensed under the MIT license or the Apache license (version 2.0), at your option. You may not
// copy, modify, or distribute this file except in compliance with said license. You can find copies
// of these licenses either in the LICENSE-MIT and LICENSE-APACHE files, or alternatively at
// <http://opensource.org/licenses/MIT> and <http://www.apache.org/licenses/LICENSE-2.0>
// respectively.
//
// Portions of documentation are copied from the LGPL 2.1+ licensed PulseAudio C headers on a
// fair-use basis, as discussed in the overall project readme (available in the git repository).

//! UTF-8 validation functions.

use crate::ffi;
use std::os::raw::c_char;

pub unsafe fn pa_utf8_valid(str: *const c_char) -> *const c_char {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_utf8_valid)(str)
    } else {
        std::ptr::null()
    }
}

pub unsafe fn pa_ascii_valid(str: *const c_char) -> *mut c_char {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_ascii_valid)(str)
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn pa_utf8_filter(str: *const c_char) -> *mut c_char {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_utf8_filter)(str)
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn pa_ascii_filter(str: *const c_char) -> *mut c_char {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_ascii_filter)(str)
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn pa_utf8_to_locale(str: *const c_char) -> *mut c_char {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_utf8_to_locale)(str)
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn pa_locale_to_utf8(str: *const c_char) -> *mut c_char {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_locale_to_utf8)(str)
    } else {
        std::ptr::null_mut()
    }
}

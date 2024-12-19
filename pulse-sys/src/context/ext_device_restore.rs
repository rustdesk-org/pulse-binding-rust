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

//! Routines for controlling module-device-restore.

use super::{pa_context, pa_context_success_cb_t};
use crate::{def::pa_device_type_t, format::pa_format_info, operation::pa_operation};
use std::os::raw::c_void;

#[repr(C)]
pub struct pa_ext_device_restore_info {
    pub dtype: pa_device_type_t,
    pub index: u32,
    pub n_formats: u8,
    pub formats: *mut *mut pa_format_info,
}

#[rustfmt::skip]
pub type pa_ext_device_restore_test_cb_t = Option<extern "C" fn(c: *mut pa_context, version: u32, userdata: *mut c_void)>;

#[rustfmt::skip]
pub type pa_ext_device_restore_subscribe_cb_t = Option<extern "C" fn(c: *mut pa_context, type_: pa_device_type_t, idx: u32, userdata: *mut c_void)>;

#[rustfmt::skip]
pub type pa_ext_device_restore_read_device_formats_cb_t = Option<extern "C" fn(c: *mut pa_context, info: *const pa_ext_device_restore_info, eol: i32, userdata: *mut c_void)>;

pub unsafe fn pa_ext_device_restore_test(
    c: *mut pa_context,
    cb: pa_ext_device_restore_test_cb_t,
    userdata: *mut c_void,
) -> *mut pa_operation {
    if let Some(functions) = crate::ffi::get_functions() {
        (functions.pa_ext_device_restore_test)(c, cb, userdata)
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn pa_ext_device_restore_subscribe(
    c: *mut pa_context,
    enable: i32,
    cb: pa_context_success_cb_t,
    userdata: *mut c_void,
) -> *mut pa_operation {
    if let Some(functions) = crate::ffi::get_functions() {
        (functions.pa_ext_device_restore_subscribe)(c, enable, cb, userdata)
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn pa_ext_device_restore_set_subscribe_cb(
    c: *mut pa_context,
    cb: pa_ext_device_restore_subscribe_cb_t,
    userdata: *mut c_void,
) {
    if let Some(functions) = crate::ffi::get_functions() {
        (functions.pa_ext_device_restore_set_subscribe_cb)(c, cb, userdata)
    }
}

pub unsafe fn pa_ext_device_restore_read_formats_all(
    c: *mut pa_context,
    cb: pa_ext_device_restore_read_device_formats_cb_t,
    userdata: *mut c_void,
) -> *mut pa_operation {
    if let Some(functions) = crate::ffi::get_functions() {
        (functions.pa_ext_device_restore_read_formats_all)(c, cb, userdata)
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn pa_ext_device_restore_read_formats(
    c: *mut pa_context,
    type_: pa_device_type_t,
    idx: u32,
    cb: pa_ext_device_restore_read_device_formats_cb_t,
    userdata: *mut c_void,
) -> *mut pa_operation {
    if let Some(functions) = crate::ffi::get_functions() {
        (functions.pa_ext_device_restore_read_formats)(c, type_, idx, cb, userdata)
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn pa_ext_device_restore_save_formats(
    c: *mut pa_context,
    type_: pa_device_type_t,
    idx: u32,
    n_formats: u8,
    formats: *const *mut pa_format_info,
    cb: pa_context_success_cb_t,
    userdata: *mut c_void,
) -> *mut pa_operation {
    if let Some(functions) = crate::ffi::get_functions() {
        (functions.pa_ext_device_restore_save_formats)(
            c, type_, idx, n_formats, formats, cb, userdata,
        )
    } else {
        std::ptr::null_mut()
    }
}

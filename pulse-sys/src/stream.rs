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

//! Audio streams for input, output and sample upload.

use crate::def::{pa_buffer_attr, pa_free_cb_t, pa_timing_info};
use crate::ffi;
use crate::proplist::{pa_proplist, pa_update_mode_t};
use crate::sample::{pa_sample_spec, pa_usec_t};
use crate::{channelmap::pa_channel_map, context::pa_context, format::pa_format_info};
use crate::{operation::pa_operation, volume::pa_cvolume};
use num_derive::{FromPrimitive, ToPrimitive};
use std::os::raw::{c_char, c_void};

/// An opaque stream for playback or recording.
#[repr(C)]
pub struct pa_stream {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum pa_stream_state_t {
    Unconnected,
    Creating,
    Ready,
    Failed,
    Terminated,
}

pub const PA_STREAM_UNCONNECTED: pa_stream_state_t = pa_stream_state_t::Unconnected;
pub const PA_STREAM_CREATING: pa_stream_state_t = pa_stream_state_t::Creating;
pub const PA_STREAM_READY: pa_stream_state_t = pa_stream_state_t::Ready;
pub const PA_STREAM_FAILED: pa_stream_state_t = pa_stream_state_t::Failed;
pub const PA_STREAM_TERMINATED: pa_stream_state_t = pa_stream_state_t::Terminated;

/// Checks if the passed state is one of the connected states (returns `true` if so).
#[inline(always)]
pub fn pa_stream_is_good(state: pa_stream_state_t) -> bool {
    state == pa_stream_state_t::Creating || state == pa_stream_state_t::Ready
}

/// Stream direction.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum pa_stream_direction_t {
    /// Invalid.
    Invalid,
    /// Playback.
    Playback,
    /// Record.
    Record,
    /// Upload.
    Upload,
}

pub const PA_STREAM_NODIRECTION: pa_stream_direction_t = pa_stream_direction_t::Invalid;
pub const PA_STREAM_PLAYBACK: pa_stream_direction_t = pa_stream_direction_t::Playback;
pub const PA_STREAM_RECORD: pa_stream_direction_t = pa_stream_direction_t::Record;
pub const PA_STREAM_UPLOAD: pa_stream_direction_t = pa_stream_direction_t::Upload;

pub type pa_stream_flags_t = u32;

pub use self::flags::*;

/// Some special flags for stream connections.
mod flags {
    use super::pa_stream_flags_t;

    pub const PA_STREAM_NOFLAGS: pa_stream_flags_t = 0;
    pub const PA_STREAM_START_CORKED: pa_stream_flags_t = 1 << 0;
    pub const PA_STREAM_INTERPOLATE_TIMING: pa_stream_flags_t = 1 << 1;
    pub const PA_STREAM_NOT_MONOTONIC: pa_stream_flags_t = 1 << 2;
    pub const PA_STREAM_AUTO_TIMING_UPDATE: pa_stream_flags_t = 1 << 3;
    pub const PA_STREAM_NO_REMAP_CHANNELS: pa_stream_flags_t = 1 << 4;
    pub const PA_STREAM_NO_REMIX_CHANNELS: pa_stream_flags_t = 1 << 5;
    pub const PA_STREAM_FIX_FORMAT: pa_stream_flags_t = 1 << 6;
    pub const PA_STREAM_FIX_RATE: pa_stream_flags_t = 1 << 7;
    pub const PA_STREAM_FIX_CHANNELS: pa_stream_flags_t = 1 << 8;
    pub const PA_STREAM_DONT_MOVE: pa_stream_flags_t = 1 << 9;
    pub const PA_STREAM_VARIABLE_RATE: pa_stream_flags_t = 1 << 10;
    pub const PA_STREAM_PEAK_DETECT: pa_stream_flags_t = 1 << 11;
    pub const PA_STREAM_START_MUTED: pa_stream_flags_t = 1 << 12;
    pub const PA_STREAM_ADJUST_LATENCY: pa_stream_flags_t = 1 << 13;
    pub const PA_STREAM_EARLY_REQUESTS: pa_stream_flags_t = 1 << 14;
    pub const PA_STREAM_DONT_INHIBIT_AUTO_SUSPEND: pa_stream_flags_t = 1 << 15;
    pub const PA_STREAM_START_UNMUTED: pa_stream_flags_t = 1 << 16;
    pub const PA_STREAM_FAIL_ON_SUSPEND: pa_stream_flags_t = 1 << 17;
    pub const PA_STREAM_RELATIVE_VOLUME: pa_stream_flags_t = 1 << 18;
    pub const PA_STREAM_PASSTHROUGH: pa_stream_flags_t = 1 << 19;
}

/// Seek mode.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum pa_seek_mode_t {
    /// Seek relatively to the write index.
    Relative = 0,
    /// Seek relatively to the start of the buffer queue.
    Absolute = 1,
    /// Seek relatively to the read index.
    RelativeOnRead = 2,
    /// Seek relatively to the current end of the buffer queue.
    RelativeEnd = 3,
}

pub const PA_SEEK_RELATIVE: pa_seek_mode_t = pa_seek_mode_t::Relative;
pub const PA_SEEK_ABSOLUTE: pa_seek_mode_t = pa_seek_mode_t::Absolute;
pub const PA_SEEK_RELATIVE_ON_READ: pa_seek_mode_t = pa_seek_mode_t::RelativeOnRead;
pub const PA_SEEK_RELATIVE_END: pa_seek_mode_t = pa_seek_mode_t::RelativeEnd;

pub const PA_STREAM_EVENT_REQUEST_CORK: &str = "request-cork";
pub const PA_STREAM_EVENT_REQUEST_UNCORK: &str = "request-uncork";
pub const PA_STREAM_EVENT_FORMAT_LOST: &str = "format-lost";

#[rustfmt::skip]
pub type pa_stream_success_cb_t = Option<extern "C" fn(s: *mut pa_stream, success: i32, userdata: *mut c_void)>;

#[rustfmt::skip]
pub type pa_stream_request_cb_t = Option<extern "C" fn(p: *mut pa_stream, nbytes: usize, userdata: *mut c_void)>;

#[rustfmt::skip]
pub type pa_stream_notify_cb_t = Option<extern "C" fn(p: *mut pa_stream, userdata: *mut c_void)>;

#[rustfmt::skip]
pub type pa_stream_event_cb_t = Option<extern "C" fn(p: *mut pa_stream, name: *const c_char, pl: *mut pa_proplist, userdata: *mut c_void)>;

pub unsafe fn pa_stream_connect_upload(s: *mut pa_stream, length: usize) -> i32 {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_connect_upload)(s, length)
    } else {
        -1
    }
}

pub unsafe fn pa_stream_finish_upload(s: *mut pa_stream) -> i32 {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_finish_upload)(s)
    } else {
        -1
    }
}

pub unsafe fn pa_stream_new(
    c: *mut pa_context,
    name: *const c_char,
    ss: *const pa_sample_spec,
    map: *const pa_channel_map,
) -> *mut pa_stream {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_new)(c, name, ss, map)
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn pa_stream_new_with_proplist(
    c: *mut pa_context,
    name: *const c_char,
    ss: *const pa_sample_spec,
    map: *const pa_channel_map,
    p: *mut pa_proplist,
) -> *mut pa_stream {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_new_with_proplist)(c, name, ss, map, p)
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn pa_stream_new_extended(
    c: *mut pa_context,
    name: *const c_char,
    formats: *const *const pa_format_info,
    n_formats: u32,
    p: *mut pa_proplist,
) -> *mut pa_stream {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_new_extended)(c, name, formats, n_formats, p)
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn pa_stream_unref(s: *mut pa_stream) {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_unref)(s)
    }
}

pub unsafe fn pa_stream_ref(s: *mut pa_stream) -> *mut pa_stream {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_ref)(s)
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn pa_stream_get_state(s: *const pa_stream) -> pa_stream_state_t {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_get_state)(s)
    } else {
        pa_stream_state_t::Failed
    }
}

pub unsafe fn pa_stream_get_context(s: *const pa_stream) -> *mut pa_context {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_get_context)(s)
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn pa_stream_get_index(s: *const pa_stream) -> u32 {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_get_index)(s)
    } else {
        std::u32::MAX
    }
}

pub unsafe fn pa_stream_get_device_index(s: *const pa_stream) -> u32 {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_get_device_index)(s)
    } else {
        std::u32::MAX
    }
}

pub unsafe fn pa_stream_get_device_name(s: *const pa_stream) -> *const c_char {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_get_device_name)(s)
    } else {
        std::ptr::null()
    }
}

pub unsafe fn pa_stream_is_suspended(s: *const pa_stream) -> i32 {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_is_suspended)(s)
    } else {
        -1
    }
}

pub unsafe fn pa_stream_is_corked(s: *const pa_stream) -> i32 {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_is_corked)(s)
    } else {
        -1
    }
}

pub unsafe fn pa_stream_connect_playback(
    s: *mut pa_stream,
    dev: *const c_char,
    attr: *const pa_buffer_attr,
    flags: pa_stream_flags_t,
    volume: *const pa_cvolume,
    sync_stream: *mut pa_stream,
) -> i32 {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_connect_playback)(s, dev, attr, flags, volume, sync_stream)
    } else {
        -1
    }
}

pub unsafe fn pa_stream_connect_record(
    s: *mut pa_stream,
    dev: *const c_char,
    attr: *const pa_buffer_attr,
    flags: pa_stream_flags_t,
) -> i32 {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_connect_record)(s, dev, attr, flags)
    } else {
        -1
    }
}

pub unsafe fn pa_stream_disconnect(s: *mut pa_stream) -> i32 {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_disconnect)(s)
    } else {
        -1
    }
}

pub unsafe fn pa_stream_begin_write(
    p: *mut pa_stream,
    data: *mut *mut c_void,
    nbytes: *mut usize,
) -> i32 {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_begin_write)(p, data, nbytes)
    } else {
        -1
    }
}

pub unsafe fn pa_stream_cancel_write(p: *mut pa_stream) -> i32 {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_cancel_write)(p)
    } else {
        -1
    }
}

pub unsafe fn pa_stream_write(
    p: *mut pa_stream,
    data: *const c_void,
    nbytes: usize,
    free_cb: pa_free_cb_t,
    offset: i64,
    seek: pa_seek_mode_t,
) -> i32 {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_write)(p, data, nbytes, free_cb, offset, seek)
    } else {
        -1
    }
}

#[cfg(any(doc, feature = "pa_v6"))]
#[cfg_attr(docsrs, doc(cfg(feature = "pa_v6")))]
pub unsafe fn pa_stream_write_ext_free(
    p: *mut pa_stream,
    data: *const c_void,
    nbytes: usize,
    free_cb: pa_free_cb_t,
    free_cb_data: *mut c_void,
    offset: i64,
    seek: pa_seek_mode_t,
) -> i32 {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_write_ext_free)(p, data, nbytes, free_cb, free_cb_data, offset, seek)
    } else {
        -1
    }
}

pub unsafe fn pa_stream_peek(
    p: *mut pa_stream,
    data: *mut *const c_void,
    nbytes: *mut usize,
) -> i32 {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_peek)(p, data, nbytes)
    } else {
        -1
    }
}

pub unsafe fn pa_stream_drop(p: *mut pa_stream) -> i32 {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_drop)(p)
    } else {
        -1
    }
}

pub unsafe fn pa_stream_writable_size(p: *const pa_stream) -> usize {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_writable_size)(p)
    } else {
        0
    }
}

pub unsafe fn pa_stream_readable_size(p: *const pa_stream) -> usize {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_readable_size)(p)
    } else {
        0
    }
}

pub unsafe fn pa_stream_drain(
    p: *mut pa_stream,
    cb: pa_stream_success_cb_t,
    userdata: *mut c_void,
) -> *mut pa_operation {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_drain)(p, cb, userdata)
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn pa_stream_update_timing_info(
    p: *mut pa_stream,
    cb: pa_stream_success_cb_t,
    userdata: *mut c_void,
) -> *mut pa_operation {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_update_timing_info)(p, cb, userdata)
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn pa_stream_set_state_callback(
    s: *mut pa_stream,
    cb: pa_stream_notify_cb_t,
    userdata: *mut c_void,
) {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_set_state_callback)(s, cb, userdata)
    }
}

pub unsafe fn pa_stream_set_write_callback(
    s: *mut pa_stream,
    cb: pa_stream_request_cb_t,
    userdata: *mut c_void,
) {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_set_write_callback)(s, cb, userdata)
    }
}

pub unsafe fn pa_stream_set_read_callback(
    s: *mut pa_stream,
    cb: pa_stream_request_cb_t,
    userdata: *mut c_void,
) {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_set_read_callback)(s, cb, userdata)
    }
}

pub unsafe fn pa_stream_set_overflow_callback(
    s: *mut pa_stream,
    cb: pa_stream_notify_cb_t,
    userdata: *mut c_void,
) {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_set_overflow_callback)(s, cb, userdata)
    }
}

pub unsafe fn pa_stream_get_underflow_index(s: *const pa_stream) -> i64 {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_get_underflow_index)(s)
    } else {
        0
    }
}

pub unsafe fn pa_stream_set_underflow_callback(
    s: *mut pa_stream,
    cb: pa_stream_notify_cb_t,
    userdata: *mut c_void,
) {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_set_underflow_callback)(s, cb, userdata)
    }
}

pub unsafe fn pa_stream_set_started_callback(
    s: *mut pa_stream,
    cb: pa_stream_notify_cb_t,
    userdata: *mut c_void,
) {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_set_started_callback)(s, cb, userdata)
    }
}

pub unsafe fn pa_stream_set_latency_update_callback(
    s: *mut pa_stream,
    cb: pa_stream_notify_cb_t,
    userdata: *mut c_void,
) {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_set_latency_update_callback)(s, cb, userdata)
    }
}

pub unsafe fn pa_stream_set_moved_callback(
    s: *mut pa_stream,
    cb: pa_stream_notify_cb_t,
    userdata: *mut c_void,
) {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_set_moved_callback)(s, cb, userdata)
    }
}

pub unsafe fn pa_stream_set_suspended_callback(
    s: *mut pa_stream,
    cb: pa_stream_notify_cb_t,
    userdata: *mut c_void,
) {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_set_suspended_callback)(s, cb, userdata)
    }
}

pub unsafe fn pa_stream_set_event_callback(
    s: *mut pa_stream,
    cb: pa_stream_event_cb_t,
    userdata: *mut c_void,
) {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_set_event_callback)(s, cb, userdata)
    }
}

pub unsafe fn pa_stream_set_buffer_attr_callback(
    s: *mut pa_stream,
    cb: pa_stream_notify_cb_t,
    userdata: *mut c_void,
) {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_set_buffer_attr_callback)(s, cb, userdata)
    }
}

pub unsafe fn pa_stream_cork(
    s: *mut pa_stream,
    b: i32,
    cb: pa_stream_success_cb_t,
    userdata: *mut c_void,
) -> *mut pa_operation {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_cork)(s, b, cb, userdata)
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn pa_stream_flush(
    s: *mut pa_stream,
    cb: pa_stream_success_cb_t,
    userdata: *mut c_void,
) -> *mut pa_operation {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_flush)(s, cb, userdata)
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn pa_stream_prebuf(
    s: *mut pa_stream,
    cb: pa_stream_success_cb_t,
    userdata: *mut c_void,
) -> *mut pa_operation {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_prebuf)(s, cb, userdata)
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn pa_stream_trigger(
    s: *mut pa_stream,
    cb: pa_stream_success_cb_t,
    userdata: *mut c_void,
) -> *mut pa_operation {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_trigger)(s, cb, userdata)
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn pa_stream_set_name(
    s: *mut pa_stream,
    name: *const c_char,
    cb: pa_stream_success_cb_t,
    userdata: *mut c_void,
) -> *mut pa_operation {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_set_name)(s, name, cb, userdata)
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn pa_stream_get_time(p: *mut pa_stream, r_usec: *mut pa_usec_t) -> i32 {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_get_time)(p, r_usec)
    } else {
        -1
    }
}

pub unsafe fn pa_stream_get_latency(
    p: *mut pa_stream,
    r_usec: *mut pa_usec_t,
    negative: *mut i32,
) -> i32 {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_get_latency)(p, r_usec, negative)
    } else {
        -1
    }
}

pub unsafe fn pa_stream_get_timing_info(p: *mut pa_stream) -> *const pa_timing_info {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_get_timing_info)(p)
    } else {
        std::ptr::null()
    }
}

pub unsafe fn pa_stream_get_sample_spec(p: *mut pa_stream) -> *const pa_sample_spec {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_get_sample_spec)(p)
    } else {
        std::ptr::null()
    }
}

pub unsafe fn pa_stream_get_channel_map(p: *mut pa_stream) -> *const pa_channel_map {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_get_channel_map)(p)
    } else {
        std::ptr::null()
    }
}

pub unsafe fn pa_stream_get_format_info(p: *const pa_stream) -> *const pa_format_info {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_get_format_info)(p)
    } else {
        std::ptr::null()
    }
}

pub unsafe fn pa_stream_get_buffer_attr(p: *mut pa_stream) -> *const pa_buffer_attr {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_get_buffer_attr)(p)
    } else {
        std::ptr::null()
    }
}

pub unsafe fn pa_stream_set_buffer_attr(
    p: *mut pa_stream,
    attr: *const pa_buffer_attr,
    cb: pa_stream_success_cb_t,
    userdata: *mut c_void,
) -> *mut pa_operation {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_set_buffer_attr)(p, attr, cb, userdata)
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn pa_stream_update_sample_rate(
    p: *mut pa_stream,
    rate: u32,
    cb: pa_stream_success_cb_t,
    userdata: *mut c_void,
) -> *mut pa_operation {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_update_sample_rate)(p, rate, cb, userdata)
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn pa_stream_proplist_update(
    p: *mut pa_stream,
    mode: pa_update_mode_t,
    proplist: *mut pa_proplist,
    cb: pa_stream_success_cb_t,
    userdata: *mut c_void,
) -> *mut pa_operation {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_proplist_update)(p, mode, proplist, cb, userdata)
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn pa_stream_proplist_remove(
    p: *mut pa_stream,
    keys: *const *const c_char,
    cb: pa_stream_success_cb_t,
    userdata: *mut c_void,
) -> *mut pa_operation {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_proplist_remove)(p, keys, cb, userdata)
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn pa_stream_set_monitor_stream(s: *mut pa_stream, sink_input_idx: u32) -> i32 {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_set_monitor_stream)(s, sink_input_idx)
    } else {
        -1
    }
}

pub unsafe fn pa_stream_get_monitor_stream(s: *const pa_stream) -> u32 {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_stream_get_monitor_stream)(s)
    } else {
        std::u32::MAX
    }
}

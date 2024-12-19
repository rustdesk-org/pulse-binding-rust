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

//! Monotonic clock utilities.

use crate::ffi;
use crate::sample::pa_usec_t;

pub unsafe fn pa_rtclock_now() -> pa_usec_t {
    if let Some(functions) = ffi::get_functions() {
        (functions.pa_rtclock_now)()
    } else {
        0
    }
}

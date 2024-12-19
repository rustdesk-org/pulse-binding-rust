use crate::channelmap::{pa_channel_map, pa_channel_position_mask_t, pa_channel_position_t};
use crate::context::{pa_context, pa_context_state_t};
use crate::def::{pa_buffer_attr, pa_spawn_api};
use crate::mainloop::api::{pa_mainloop_api, pa_mainloop_api_once_cb};
use crate::mainloop::{pa_mainloop, pa_poll_func};
use crate::operation::pa_operation;
use crate::proplist::{pa_proplist, pa_update_mode_t};
use crate::sample::{pa_sample_spec, pa_usec_t};
use crate::stream::{
    pa_seek_mode_t, pa_stream, pa_stream_event_cb_t, pa_stream_flags_t, pa_stream_notify_cb_t,
    pa_stream_request_cb_t, pa_stream_state_t, pa_stream_success_cb_t,
};
use crate::volume::{pa_cvolume, pa_volume_t};
use crate::{
    pa_card_info_cb_t, pa_channel_map_def_t, pa_client_info_cb_t, pa_context_event_cb_t,
    pa_context_flags_t, pa_context_index_cb_t, pa_context_notify_cb_t, pa_context_play_sample_cb_t,
    pa_context_subscribe_cb_t, pa_context_success_cb_t, pa_device_type_t, pa_direction_t,
    pa_encoding_t, pa_ext_device_manager_read_cb_t, pa_ext_device_manager_subscribe_cb_t,
    pa_ext_device_manager_test_cb_t, pa_ext_device_restore_read_device_formats_cb_t,
    pa_ext_device_restore_subscribe_cb_t, pa_ext_device_restore_test_cb_t,
    pa_ext_stream_restore_info, pa_ext_stream_restore_read_cb_t,
    pa_ext_stream_restore_subscribe_cb_t, pa_ext_stream_restore_test_cb_t, pa_format_info,
    pa_free_cb_t, pa_module_info_cb_t, pa_operation_notify_cb_t, pa_operation_state_t,
    pa_prop_type_t, pa_sample_format_t, pa_sample_info_cb_t, pa_server_info_cb_t, pa_signal_cb_t,
    pa_signal_destroy_cb_t, pa_signal_event, pa_sink_info_cb_t, pa_sink_input_info_cb_t,
    pa_source_info_cb_t, pa_source_output_info_cb_t, pa_stat_info_cb_t, pa_subscription_mask_t,
    pa_threaded_mainloop, pa_time_event, pa_time_event_cb_t, pa_timing_info,
};
use libc::timeval;
use libloading::Library;
use once_cell::sync::OnceCell;
use std::ffi::c_ulong;
use std::os::raw::{c_char, c_void};
use std::sync::{Arc, Mutex};

pub struct PulseFunctions {
    // Stream 相关函数
    pub pa_stream_connect_upload: unsafe extern "C" fn(*mut pa_stream, length: usize) -> i32,
    pub pa_stream_finish_upload: unsafe extern "C" fn(*mut pa_stream) -> i32,
    pub pa_stream_new: unsafe extern "C" fn(
        *mut pa_context,
        *const c_char,
        *const pa_sample_spec,
        *const pa_channel_map,
    ) -> *mut pa_stream,
    pub pa_stream_new_with_proplist: unsafe extern "C" fn(
        *mut pa_context,
        *const c_char,
        *const pa_sample_spec,
        *const pa_channel_map,
        *mut pa_proplist,
    ) -> *mut pa_stream,
    pub pa_stream_new_extended: unsafe extern "C" fn(
        *mut pa_context,
        *const c_char,
        *const *const pa_format_info,
        size: u32,
        *mut pa_proplist,
    ) -> *mut pa_stream,
    pub pa_stream_unref: unsafe extern "C" fn(*mut pa_stream),
    pub pa_stream_ref: unsafe extern "C" fn(*mut pa_stream) -> *mut pa_stream,
    pub pa_stream_get_state: unsafe extern "C" fn(*const pa_stream) -> pa_stream_state_t,
    pub pa_stream_get_context: unsafe extern "C" fn(*const pa_stream) -> *mut pa_context,
    pub pa_stream_get_index: unsafe extern "C" fn(*const pa_stream) -> u32,
    pub pa_stream_get_device_index: unsafe extern "C" fn(*const pa_stream) -> u32,
    pub pa_stream_get_device_name: unsafe extern "C" fn(*const pa_stream) -> *const c_char,
    pub pa_stream_is_suspended: unsafe extern "C" fn(*const pa_stream) -> i32,
    pub pa_stream_is_corked: unsafe extern "C" fn(*const pa_stream) -> i32,
    pub pa_stream_connect_playback: unsafe extern "C" fn(
        *mut pa_stream,
        *const c_char,
        *const pa_buffer_attr,
        flags: pa_stream_flags_t,
        *const pa_cvolume,
        *mut pa_stream,
    ) -> i32,
    pub pa_stream_connect_record: unsafe extern "C" fn(
        *mut pa_stream,
        *const c_char,
        *const pa_buffer_attr,
        flags: pa_stream_flags_t,
    ) -> i32,
    pub pa_stream_disconnect: unsafe extern "C" fn(*mut pa_stream) -> i32,
    pub pa_stream_begin_write:
        unsafe extern "C" fn(*mut pa_stream, *mut *mut c_void, *mut usize) -> i32,
    pub pa_stream_cancel_write: unsafe extern "C" fn(*mut pa_stream) -> i32,
    pub pa_stream_write: unsafe extern "C" fn(
        s: *mut pa_stream,
        data: *const c_void,
        nbytes: usize,
        free_cb: pa_free_cb_t,
        offset: i64,
        seek: pa_seek_mode_t,
    ) -> i32,
    #[cfg(any(doc, feature = "pa_v6"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "pa_v6")))]
    pub pa_stream_write_ext_free: unsafe extern "C" fn(
        s: *mut pa_stream,
        data: *const c_void,
        nbytes: usize,
        free_cb: pa_free_cb_t,
        free_cb_data: *mut c_void,
        offset: i64,
        seek: pa_seek_mode_t,
    ) -> i32,
    pub pa_stream_peek: unsafe extern "C" fn(
        s: *mut pa_stream,
        data: *mut *const c_void,
        nbytes: *mut usize,
    ) -> i32,
    pub pa_stream_drop: unsafe extern "C" fn(*mut pa_stream) -> i32,
    pub pa_stream_writable_size: unsafe extern "C" fn(*const pa_stream) -> usize,
    pub pa_stream_readable_size: unsafe extern "C" fn(*const pa_stream) -> usize,
    pub pa_stream_drain: unsafe extern "C" fn(
        *mut pa_stream,
        cb: pa_stream_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_stream_update_timing_info: unsafe extern "C" fn(
        *mut pa_stream,
        cb: pa_stream_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_stream_set_state_callback:
        unsafe extern "C" fn(*mut pa_stream, cb: pa_stream_notify_cb_t, userdata: *mut c_void),
    pub pa_stream_set_write_callback:
        unsafe extern "C" fn(*mut pa_stream, cb: pa_stream_request_cb_t, userdata: *mut c_void),
    pub pa_stream_set_read_callback:
        unsafe extern "C" fn(*mut pa_stream, cb: pa_stream_request_cb_t, userdata: *mut c_void),
    pub pa_stream_set_overflow_callback:
        unsafe extern "C" fn(*mut pa_stream, cb: pa_stream_notify_cb_t, userdata: *mut c_void),
    pub pa_stream_get_underflow_index: unsafe extern "C" fn(*const pa_stream) -> i64,
    pub pa_stream_set_underflow_callback:
        unsafe extern "C" fn(*mut pa_stream, cb: pa_stream_notify_cb_t, userdata: *mut c_void),
    pub pa_stream_set_started_callback:
        unsafe extern "C" fn(*mut pa_stream, cb: pa_stream_notify_cb_t, userdata: *mut c_void),
    pub pa_stream_set_latency_update_callback:
        unsafe extern "C" fn(*mut pa_stream, cb: pa_stream_notify_cb_t, userdata: *mut c_void),
    pub pa_stream_set_moved_callback:
        unsafe extern "C" fn(*mut pa_stream, cb: pa_stream_notify_cb_t, userdata: *mut c_void),
    pub pa_stream_set_suspended_callback:
        unsafe extern "C" fn(*mut pa_stream, cb: pa_stream_notify_cb_t, userdata: *mut c_void),
    pub pa_stream_set_event_callback:
        unsafe extern "C" fn(*mut pa_stream, cb: pa_stream_event_cb_t, userdata: *mut c_void),
    pub pa_stream_set_buffer_attr_callback:
        unsafe extern "C" fn(*mut pa_stream, cb: pa_stream_notify_cb_t, userdata: *mut c_void),
    pub pa_stream_cork: unsafe extern "C" fn(
        *mut pa_stream,
        b: i32,
        cb: pa_stream_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_stream_flush: unsafe extern "C" fn(
        *mut pa_stream,
        cb: pa_stream_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_stream_prebuf: unsafe extern "C" fn(
        *mut pa_stream,
        cb: pa_stream_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_stream_trigger: unsafe extern "C" fn(
        *mut pa_stream,
        cb: pa_stream_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_stream_set_name: unsafe extern "C" fn(
        *mut pa_stream,
        name: *const c_char,
        cb: pa_stream_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_stream_get_time: unsafe extern "C" fn(*mut pa_stream, r_usec: *mut pa_usec_t) -> i32,
    pub pa_stream_get_latency:
        unsafe extern "C" fn(*mut pa_stream, r_usec: *mut pa_usec_t, negative: *mut i32) -> i32,
    pub pa_stream_get_timing_info: unsafe extern "C" fn(*mut pa_stream) -> *const pa_timing_info,
    pub pa_stream_get_sample_spec: unsafe extern "C" fn(*mut pa_stream) -> *const pa_sample_spec,
    pub pa_stream_get_channel_map: unsafe extern "C" fn(*mut pa_stream) -> *const pa_channel_map,
    pub pa_stream_get_format_info: unsafe extern "C" fn(*const pa_stream) -> *const pa_format_info,
    pub pa_stream_get_buffer_attr: unsafe extern "C" fn(*mut pa_stream) -> *const pa_buffer_attr,
    pub pa_stream_set_buffer_attr: unsafe extern "C" fn(
        *mut pa_stream,
        attr: *const pa_buffer_attr,
        cb: pa_stream_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_stream_update_sample_rate: unsafe extern "C" fn(
        *mut pa_stream,
        rate: u32,
        cb: pa_stream_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_stream_proplist_update: unsafe extern "C" fn(
        *mut pa_stream,
        mode: pa_update_mode_t,
        p: *mut pa_proplist,
        cb: pa_stream_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_stream_proplist_remove: unsafe extern "C" fn(
        *mut pa_stream,
        keys: *const *const c_char,
        cb: pa_stream_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_stream_set_monitor_stream:
        unsafe extern "C" fn(*mut pa_stream, sink_input_idx: u32) -> i32,
    pub pa_stream_get_monitor_stream: unsafe extern "C" fn(*const pa_stream) -> u32,
    // Timeval 相关函数
    pub pa_gettimeofday: unsafe extern "C" fn(*mut timeval) -> *mut timeval,
    pub pa_timeval_diff: unsafe extern "C" fn(*const timeval, *const timeval) -> pa_usec_t,
    pub pa_timeval_cmp: unsafe extern "C" fn(*const timeval, *const timeval) -> i32,
    pub pa_timeval_age: unsafe extern "C" fn(*const timeval) -> pa_usec_t,
    pub pa_timeval_add: unsafe extern "C" fn(*mut timeval, v: i64) -> *mut timeval,
    pub pa_timeval_sub: unsafe extern "C" fn(*mut timeval, v: i64) -> *mut timeval,
    pub pa_timeval_store: unsafe extern "C" fn(*mut timeval, t: i64) -> *mut timeval,
    pub pa_timeval_load: unsafe extern "C" fn(*const timeval) -> pa_usec_t,
    // UTF8 相关函数
    pub pa_utf8_valid: unsafe extern "C" fn(*const c_char) -> *mut c_char,
    pub pa_ascii_valid: unsafe extern "C" fn(*const c_char) -> *mut c_char,
    pub pa_utf8_filter: unsafe extern "C" fn(*const c_char) -> *mut c_char,
    pub pa_ascii_filter: unsafe extern "C" fn(*const c_char) -> *mut c_char,
    pub pa_utf8_to_locale: unsafe extern "C" fn(*const c_char) -> *mut c_char,
    pub pa_locale_to_utf8: unsafe extern "C" fn(*const c_char) -> *mut c_char,
    // Utility functions
    pub pa_get_user_name: unsafe extern "C" fn(*mut c_char, l: usize) -> *mut c_char,
    pub pa_get_host_name: unsafe extern "C" fn(*mut c_char, l: usize) -> *mut c_char,
    pub pa_get_fqdn: unsafe extern "C" fn(*mut c_char, l: usize) -> *mut c_char,
    pub pa_get_home_dir: unsafe extern "C" fn(*mut c_char, l: usize) -> *mut c_char,
    pub pa_get_binary_name: unsafe extern "C" fn(*mut c_char, l: usize) -> *mut c_char,
    pub pa_path_get_filename: unsafe extern "C" fn(*const c_char) -> *mut c_char,
    pub pa_msleep: unsafe extern "C" fn(t: c_ulong) -> i32,
    #[cfg(any(doc, feature = "pa_v13"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "pa_v13")))]
    pub pa_thread_make_realtime: unsafe extern "C" fn(rtprio: i32) -> i32,
    // Version related functions
    pub pa_get_library_version: unsafe extern "C" fn() -> *const c_char,
    // Volume related functions
    pub pa_cvolume_equal: unsafe extern "C" fn(*const pa_cvolume, *const pa_cvolume) -> i32,
    pub pa_cvolume_init: unsafe extern "C" fn(*mut pa_cvolume) -> *mut pa_cvolume,
    pub pa_cvolume_set:
        unsafe extern "C" fn(*mut pa_cvolume, channels: u32, v: pa_volume_t) -> *mut pa_cvolume,
    pub pa_cvolume_snprint:
        unsafe extern "C" fn(*mut c_char, l: usize, c: *const pa_cvolume) -> *mut c_char,
    pub pa_sw_cvolume_snprint_dB:
        unsafe extern "C" fn(*mut c_char, l: usize, c: *const pa_cvolume) -> *mut c_char,
    pub pa_cvolume_snprint_verbose: unsafe extern "C" fn(
        *mut c_char,
        l: usize,
        c: *const pa_cvolume,
        map: *const pa_channel_map,
        print_dB: i32,
    ) -> *mut c_char,

    pub pa_volume_snprint:
        unsafe extern "C" fn(*mut c_char, l: usize, v: pa_volume_t) -> *mut c_char,
    pub pa_sw_volume_snprint_dB:
        unsafe extern "C" fn(*mut c_char, l: usize, v: pa_volume_t) -> *mut c_char,
    pub pa_volume_snprint_verbose:
        unsafe extern "C" fn(*mut c_char, l: usize, v: pa_volume_t, print_dB: i32) -> *mut c_char,

    pub pa_cvolume_avg: unsafe extern "C" fn(*const pa_cvolume) -> pa_volume_t,
    pub pa_cvolume_avg_mask: unsafe extern "C" fn(
        *const pa_cvolume,
        *const pa_channel_map,
        mask: pa_channel_position_mask_t,
    ) -> pa_volume_t,

    pub pa_cvolume_max: unsafe extern "C" fn(*const pa_cvolume) -> pa_volume_t,
    pub pa_cvolume_max_mask: unsafe extern "C" fn(
        *const pa_cvolume,
        *const pa_channel_map,
        mask: pa_channel_position_mask_t,
    ) -> pa_volume_t,
    pub pa_cvolume_min: unsafe extern "C" fn(*const pa_cvolume) -> pa_volume_t,
    pub pa_cvolume_min_mask: unsafe extern "C" fn(
        *const pa_cvolume,
        *const pa_channel_map,
        mask: pa_channel_position_mask_t,
    ) -> pa_volume_t,
    pub pa_cvolume_valid: unsafe extern "C" fn(*const pa_cvolume) -> i32,
    pub pa_cvolume_channels_equal_to:
        unsafe extern "C" fn(*const pa_cvolume, v: pa_volume_t) -> i32,

    pub pa_sw_volume_multiply: unsafe extern "C" fn(a: pa_volume_t, b: pa_volume_t) -> pa_volume_t,
    pub pa_sw_cvolume_multiply: unsafe extern "C" fn(
        dest: *mut pa_cvolume,
        a: *const pa_cvolume,
        b: *const pa_cvolume,
    ) -> *mut pa_cvolume,

    pub pa_sw_cvolume_multiply_scalar: unsafe extern "C" fn(
        dest: *mut pa_cvolume,
        a: *const pa_cvolume,
        b: pa_volume_t,
    ) -> *mut pa_cvolume,
    pub pa_sw_volume_divide: unsafe extern "C" fn(a: pa_volume_t, b: pa_volume_t) -> pa_volume_t,
    pub pa_sw_cvolume_divide: unsafe extern "C" fn(
        dest: *mut pa_cvolume,
        a: *const pa_cvolume,
        b: *const pa_cvolume,
    ) -> *mut pa_cvolume,
    pub pa_sw_cvolume_divide_scalar: unsafe extern "C" fn(
        dest: *mut pa_cvolume,
        a: *const pa_cvolume,
        b: pa_volume_t,
    ) -> *mut pa_cvolume,
    pub pa_sw_volume_from_dB: unsafe extern "C" fn(f: f64) -> pa_volume_t,
    pub pa_sw_volume_to_dB: unsafe extern "C" fn(v: pa_volume_t) -> f64,
    pub pa_sw_volume_from_linear: unsafe extern "C" fn(v: f64) -> pa_volume_t,
    pub pa_sw_volume_to_linear: unsafe extern "C" fn(v: pa_volume_t) -> f64,

    pub pa_cvolume_remap: unsafe extern "C" fn(
        v: *mut pa_cvolume,
        from: *const pa_channel_map,
        to: *const pa_channel_map,
    ) -> *mut pa_cvolume,
    pub pa_cvolume_compatible:
        unsafe extern "C" fn(*const pa_cvolume, ss: *const pa_sample_spec) -> i32,
    pub pa_cvolume_compatible_with_channel_map:
        unsafe extern "C" fn(*const pa_cvolume, cm: *const pa_channel_map) -> i32,
    pub pa_cvolume_get_balance:
        unsafe extern "C" fn(*const pa_cvolume, map: *const pa_channel_map) -> f32,
    pub pa_cvolume_set_balance: unsafe extern "C" fn(
        v: *mut pa_cvolume,
        map: *const pa_channel_map,
        new_balance: f32,
    ) -> *mut pa_cvolume,
    pub pa_cvolume_get_fade:
        unsafe extern "C" fn(*const pa_cvolume, map: *const pa_channel_map) -> f32,
    pub pa_cvolume_set_fade: unsafe extern "C" fn(
        v: *mut pa_cvolume,
        map: *const pa_channel_map,
        new_fade: f32,
    ) -> *mut pa_cvolume,

    #[cfg(any(doc, feature = "pa_v8"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "pa_v8")))]
    pub pa_cvolume_get_lfe_balance:
        unsafe extern "C" fn(*const pa_cvolume, map: *const pa_channel_map) -> f32,
    #[cfg(any(doc, feature = "pa_v8"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "pa_v8")))]
    pub pa_cvolume_set_lfe_balance: unsafe extern "C" fn(
        v: *mut pa_cvolume,
        map: *const pa_channel_map,
        new_balance: f32,
    ) -> *mut pa_cvolume,
    pub pa_cvolume_scale:
        unsafe extern "C" fn(v: *mut pa_cvolume, max: pa_volume_t) -> *mut pa_cvolume,
    pub pa_cvolume_scale_mask: unsafe extern "C" fn(
        v: *mut pa_cvolume,
        max: pa_volume_t,
        cm: *const pa_channel_map,
        mask: pa_channel_position_mask_t,
    ) -> *mut pa_cvolume,

    pub pa_cvolume_set_position: unsafe extern "C" fn(
        cv: *mut pa_cvolume,
        map: *const pa_channel_map,
        t: pa_channel_position_t,
        v: pa_volume_t,
    ) -> *mut pa_cvolume,
    pub pa_cvolume_get_position: unsafe extern "C" fn(
        cv: *const pa_cvolume,
        map: *const pa_channel_map,
        t: pa_channel_position_t,
    ) -> pa_volume_t,
    pub pa_cvolume_merge: unsafe extern "C" fn(
        dest: *mut pa_cvolume,
        a: *const pa_cvolume,
        b: *const pa_cvolume,
    ) -> *mut pa_cvolume,
    pub pa_cvolume_inc_clamp: unsafe extern "C" fn(
        v: *mut pa_cvolume,
        inc: pa_volume_t,
        limit: pa_volume_t,
    ) -> *mut pa_cvolume,
    pub pa_cvolume_inc:
        unsafe extern "C" fn(v: *mut pa_cvolume, inc: pa_volume_t) -> *mut pa_cvolume,
    pub pa_cvolume_dec:
        unsafe extern "C" fn(v: *mut pa_cvolume, dec: pa_volume_t) -> *mut pa_cvolume,

    // Memory allocation functions
    pub pa_xmalloc: unsafe extern "C" fn(l: usize) -> *mut c_void,
    pub pa_xmalloc0: unsafe extern "C" fn(l: usize) -> *mut c_void,
    pub pa_xrealloc: unsafe extern "C" fn(ptr: *mut c_void, size: usize) -> *mut c_void,
    pub pa_xfree: unsafe extern "C" fn(p: *mut c_void),
    pub pa_xstrdup: unsafe extern "C" fn(s: *const c_char) -> *mut c_char,
    pub pa_xstrndup: unsafe extern "C" fn(s: *const c_char, l: usize) -> *mut c_char,
    pub pa_xmemdup: unsafe extern "C" fn(p: *const c_void, l: usize) -> *mut c_void,

    // Sample related functions
    pub pa_bytes_per_second: unsafe extern "C" fn(spec: *const pa_sample_spec) -> usize,
    pub pa_frame_size: unsafe extern "C" fn(spec: *const pa_sample_spec) -> usize,
    pub pa_sample_size: unsafe extern "C" fn(spec: *const pa_sample_spec) -> usize,
    pub pa_sample_size_of_format: unsafe extern "C" fn(f: pa_sample_format_t) -> usize,
    pub pa_bytes_to_usec:
        unsafe extern "C" fn(length: u64, spec: *const pa_sample_spec) -> pa_usec_t,
    pub pa_usec_to_bytes: unsafe extern "C" fn(t: pa_usec_t, spec: *const pa_sample_spec) -> usize,
    pub pa_sample_spec_init: unsafe extern "C" fn(spec: *mut pa_sample_spec) -> *mut pa_sample_spec,
    pub pa_sample_format_valid: unsafe extern "C" fn(format: u32) -> i32,
    pub pa_sample_rate_valid: unsafe extern "C" fn(rate: u32) -> i32,
    pub pa_channels_valid: unsafe extern "C" fn(channels: u8) -> i32,
    pub pa_sample_spec_valid: unsafe extern "C" fn(spec: *const pa_sample_spec) -> i32,
    pub pa_sample_spec_equal:
        unsafe extern "C" fn(a: *const pa_sample_spec, b: *const pa_sample_spec) -> i32,
    pub pa_sample_format_to_string: unsafe extern "C" fn(f: pa_sample_format_t) -> *const c_char,
    pub pa_parse_sample_format: unsafe extern "C" fn(format: *const c_char) -> pa_sample_format_t,
    pub pa_sample_spec_snprint:
        unsafe extern "C" fn(s: *mut c_char, l: usize, spec: *const pa_sample_spec) -> *mut c_char,
    pub pa_bytes_snprint: unsafe extern "C" fn(s: *mut c_char, l: usize, v: u32) -> *mut c_char,
    pub pa_sample_format_is_le: unsafe extern "C" fn(f: pa_sample_format_t) -> i32,
    pub pa_sample_format_is_be: unsafe extern "C" fn(f: pa_sample_format_t) -> i32,

    // RtClock related functions
    pub pa_rtclock_now: unsafe extern "C" fn() -> pa_usec_t,

    // Proplist related functions
    pub pa_proplist_new: unsafe extern "C" fn() -> *mut pa_proplist,
    pub pa_proplist_free: unsafe extern "C" fn(p: *mut pa_proplist),
    pub pa_proplist_key_valid: unsafe extern "C" fn(key: *const c_char) -> i32,
    pub pa_proplist_sets:
        unsafe extern "C" fn(p: *mut pa_proplist, key: *const c_char, value: *const c_char) -> i32,
    pub pa_proplist_setp: unsafe extern "C" fn(p: *mut pa_proplist, pair: *const c_char) -> i32,
    // pub pa_proplist_setf: unsafe extern "C" fn(
    //     p: *mut pa_proplist,
    //     key: *const c_char,
    //     format: *const c_char,
    //     ...
    // ) -> i32,
    pub pa_proplist_set: unsafe extern "C" fn(
        p: *mut pa_proplist,
        key: *const c_char,
        data: *const c_void,
        nbytes: usize,
    ) -> i32,
    pub pa_proplist_gets:
        unsafe extern "C" fn(p: *const pa_proplist, key: *const c_char) -> *const c_char,
    pub pa_proplist_get: unsafe extern "C" fn(
        p: *const pa_proplist,
        key: *const c_char,
        data: *mut *const c_void,
        nbytes: *mut usize,
    ) -> i32,
    pub pa_proplist_update: unsafe extern "C" fn(
        p: *mut pa_proplist,
        mode: pa_update_mode_t,
        other: *const pa_proplist,
    ),
    pub pa_proplist_unset: unsafe extern "C" fn(p: *mut pa_proplist, key: *const c_char) -> i32,
    pub pa_proplist_unset_many:
        unsafe extern "C" fn(p: *mut pa_proplist, keys: *const *const c_char) -> i32,
    pub pa_proplist_iterate:
        unsafe extern "C" fn(p: *const pa_proplist, state: *mut *mut c_void) -> *const c_char,
    pub pa_proplist_to_string: unsafe extern "C" fn(p: *const pa_proplist) -> *mut c_char,
    pub pa_proplist_to_string_sep:
        unsafe extern "C" fn(p: *const pa_proplist, sep: *const c_char) -> *mut c_char,
    pub pa_proplist_from_string: unsafe extern "C" fn(s: *const c_char) -> *mut pa_proplist,
    pub pa_proplist_contains:
        unsafe extern "C" fn(p: *const pa_proplist, key: *const c_char) -> i32,
    pub pa_proplist_clear: unsafe extern "C" fn(p: *mut pa_proplist),
    pub pa_proplist_copy: unsafe extern "C" fn(p: *const pa_proplist) -> *mut pa_proplist,
    pub pa_proplist_size: unsafe extern "C" fn(p: *const pa_proplist) -> u32,
    pub pa_proplist_isempty: unsafe extern "C" fn(p: *const pa_proplist) -> i32,
    pub pa_proplist_equal:
        unsafe extern "C" fn(a: *const pa_proplist, b: *const pa_proplist) -> i32,

    // Format related functions
    pub pa_encoding_to_string: unsafe extern "C" fn(e: pa_encoding_t) -> *const c_char,
    #[cfg(any(doc, feature = "pa_v12"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "pa_v12")))]
    pub pa_encoding_from_string: unsafe extern "C" fn(encoding: *const c_char) -> pa_encoding_t,

    pub pa_format_info_new: unsafe extern "C" fn() -> *mut pa_format_info,
    pub pa_format_info_copy:
        unsafe extern "C" fn(src: *const pa_format_info) -> *mut pa_format_info,
    pub pa_format_info_free: unsafe extern "C" fn(f: *mut pa_format_info),
    pub pa_format_info_valid: unsafe extern "C" fn(f: *const pa_format_info) -> i32,
    pub pa_format_info_is_pcm: unsafe extern "C" fn(f: *const pa_format_info) -> i32,
    pub pa_format_info_is_compatible:
        unsafe extern "C" fn(first: *const pa_format_info, second: *const pa_format_info) -> i32,
    pub pa_format_info_snprint:
        unsafe extern "C" fn(s: *mut c_char, l: usize, f: *const pa_format_info) -> *mut c_char,
    pub pa_format_info_from_string: unsafe extern "C" fn(s: *const c_char) -> *mut pa_format_info,
    pub pa_format_info_from_sample_spec: unsafe extern "C" fn(
        ss: *const pa_sample_spec,
        map: *const pa_channel_map,
    ) -> *mut pa_format_info,
    pub pa_format_info_to_sample_spec: unsafe extern "C" fn(
        f: *const pa_format_info,
        ss: *mut pa_sample_spec,
        map: *mut pa_channel_map,
    ) -> i32,
    pub pa_format_info_get_prop_type:
        unsafe extern "C" fn(f: *const pa_format_info, key: *const c_char) -> pa_prop_type_t,
    pub pa_format_info_get_prop_int:
        unsafe extern "C" fn(f: *const pa_format_info, key: *const c_char, v: *mut i32) -> i32,
    pub pa_format_info_get_prop_int_range: unsafe extern "C" fn(
        f: *const pa_format_info,
        key: *const c_char,
        min: *mut i32,
        max: *mut i32,
    ) -> i32,
    pub pa_format_info_get_prop_int_array: unsafe extern "C" fn(
        f: *const pa_format_info,
        key: *const c_char,
        values: *mut *mut i32,
        n_values: *mut i32,
    ) -> i32,
    pub pa_format_info_get_prop_string: unsafe extern "C" fn(
        f: *const pa_format_info,
        key: *const c_char,
        v: *mut *mut c_char,
    ) -> i32,
    pub pa_format_info_get_prop_string_array: unsafe extern "C" fn(
        f: *const pa_format_info,
        key: *const c_char,
        values: *mut *mut *mut c_char,
        n_values: *mut i32,
    ) -> i32,
    pub pa_format_info_free_string_array:
        unsafe extern "C" fn(values: *mut *mut c_char, n_values: i32),
    #[cfg(any(doc, feature = "pa_v13"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "pa_v13")))]
    pub pa_format_info_get_sample_format:
        unsafe extern "C" fn(f: *const pa_format_info, sf: *mut pa_sample_format_t) -> i32,
    #[cfg(any(doc, feature = "pa_v13"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "pa_v13")))]
    pub pa_format_info_get_rate:
        unsafe extern "C" fn(f: *const pa_format_info, rate: *mut u32) -> i32,
    #[cfg(any(doc, feature = "pa_v13"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "pa_v13")))]
    pub pa_format_info_get_channels:
        unsafe extern "C" fn(f: *const pa_format_info, channels: *mut u8) -> i32,
    #[cfg(any(doc, feature = "pa_v13"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "pa_v13")))]
    pub pa_format_info_get_channel_map:
        unsafe extern "C" fn(f: *const pa_format_info, map: *const pa_channel_map) -> i32,
    pub pa_format_info_set_prop_int:
        unsafe extern "C" fn(f: *mut pa_format_info, key: *const c_char, value: i32),
    pub pa_format_info_set_prop_int_array: unsafe extern "C" fn(
        f: *mut pa_format_info,
        key: *const c_char,
        values: *const i32,
        n_values: i32,
    ),
    pub pa_format_info_set_prop_int_range:
        unsafe extern "C" fn(f: *mut pa_format_info, key: *const c_char, min: i32, max: i32),
    pub pa_format_info_set_prop_string:
        unsafe extern "C" fn(f: *mut pa_format_info, key: *const c_char, value: *const c_char),
    pub pa_format_info_set_prop_string_array: unsafe extern "C" fn(
        f: *mut pa_format_info,
        key: *const c_char,
        values: *const *const c_char,
        n_values: i32,
    ),
    pub pa_format_info_set_sample_format:
        unsafe extern "C" fn(f: *mut pa_format_info, sf: pa_sample_format_t),
    pub pa_format_info_set_rate: unsafe extern "C" fn(f: *mut pa_format_info, rate: i32),
    pub pa_format_info_set_channels: unsafe extern "C" fn(f: *mut pa_format_info, channels: i32),
    pub pa_format_info_set_channel_map:
        unsafe extern "C" fn(f: *mut pa_format_info, map: *const pa_channel_map),

    // Error related functions
    pub pa_strerror: unsafe extern "C" fn(error: i32) -> *const c_char,

    // Direction related functions
    #[cfg(any(doc, feature = "pa_v6"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "pa_v6")))]
    pub pa_direction_valid: unsafe extern "C" fn(direction: pa_direction_t) -> i32,
    #[cfg(any(doc, feature = "pa_v6"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "pa_v6")))]
    pub pa_direction_to_string: unsafe extern "C" fn(direction: pa_direction_t) -> *const c_char,

    // Channelmap related functions
    pub pa_channel_map_init: unsafe extern "C" fn(m: *mut pa_channel_map) -> *mut pa_channel_map,
    pub pa_channel_map_init_mono:
        unsafe extern "C" fn(m: *mut pa_channel_map) -> *mut pa_channel_map,
    pub pa_channel_map_init_stereo:
        unsafe extern "C" fn(m: *mut pa_channel_map) -> *mut pa_channel_map,
    pub pa_channel_map_init_auto: unsafe extern "C" fn(
        m: *mut pa_channel_map,
        channels: u32,
        def: pa_channel_map_def_t,
    ) -> *mut pa_channel_map,
    pub pa_channel_map_init_extend: unsafe extern "C" fn(
        m: *mut pa_channel_map,
        channels: u32,
        def: pa_channel_map_def_t,
    ) -> *mut pa_channel_map,
    pub pa_channel_position_to_string:
        unsafe extern "C" fn(pos: pa_channel_position_t) -> *const c_char,
    pub pa_channel_position_from_string:
        unsafe extern "C" fn(s: *const c_char) -> pa_channel_position_t,
    pub pa_channel_position_to_pretty_string:
        unsafe extern "C" fn(pos: pa_channel_position_t) -> *const c_char,
    pub pa_channel_map_snprint:
        unsafe extern "C" fn(s: *mut c_char, l: usize, map: *const pa_channel_map) -> *mut c_char,
    pub pa_channel_map_parse:
        unsafe extern "C" fn(map: *mut pa_channel_map, s: *const c_char) -> *mut pa_channel_map,
    pub pa_channel_map_equal:
        unsafe extern "C" fn(a: *const pa_channel_map, b: *const pa_channel_map) -> i32,
    pub pa_channel_map_valid: unsafe extern "C" fn(map: *const pa_channel_map) -> i32,
    pub pa_channel_map_compatible:
        unsafe extern "C" fn(map: *const pa_channel_map, ss: *const pa_sample_spec) -> i32,
    pub pa_channel_map_superset:
        unsafe extern "C" fn(a: *const pa_channel_map, b: *const pa_channel_map) -> i32,
    pub pa_channel_map_can_balance: unsafe extern "C" fn(map: *const pa_channel_map) -> i32,
    pub pa_channel_map_can_fade: unsafe extern "C" fn(map: *const pa_channel_map) -> i32,
    #[cfg(any(doc, feature = "pa_v8"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "pa_v8")))]
    pub pa_channel_map_can_lfe_balance: unsafe extern "C" fn(map: *const pa_channel_map) -> i32,
    pub pa_channel_map_to_name: unsafe extern "C" fn(map: *const pa_channel_map) -> *const c_char,
    pub pa_channel_map_to_pretty_name:
        unsafe extern "C" fn(map: *const pa_channel_map) -> *const c_char,
    pub pa_channel_map_has_position:
        unsafe extern "C" fn(map: *const pa_channel_map, p: pa_channel_position_t) -> i32,
    pub pa_channel_map_mask:
        unsafe extern "C" fn(map: *const pa_channel_map) -> pa_channel_position_mask_t,

    // Threaded mainloop related functions
    pub pa_threaded_mainloop_new: unsafe extern "C" fn() -> *mut pa_threaded_mainloop,
    pub pa_threaded_mainloop_free: unsafe extern "C" fn(m: *mut pa_threaded_mainloop),
    pub pa_threaded_mainloop_start: unsafe extern "C" fn(m: *mut pa_threaded_mainloop) -> i32,
    pub pa_threaded_mainloop_stop: unsafe extern "C" fn(m: *mut pa_threaded_mainloop),
    pub pa_threaded_mainloop_lock: unsafe extern "C" fn(m: *mut pa_threaded_mainloop),
    pub pa_threaded_mainloop_unlock: unsafe extern "C" fn(m: *mut pa_threaded_mainloop),
    pub pa_threaded_mainloop_wait: unsafe extern "C" fn(m: *mut pa_threaded_mainloop),
    pub pa_threaded_mainloop_signal:
        unsafe extern "C" fn(m: *mut pa_threaded_mainloop, wait_for_accept: i32),
    pub pa_threaded_mainloop_accept: unsafe extern "C" fn(m: *mut pa_threaded_mainloop),
    pub pa_threaded_mainloop_get_retval:
        unsafe extern "C" fn(m: *const pa_threaded_mainloop) -> i32,
    pub pa_threaded_mainloop_get_api:
        unsafe extern "C" fn(m: *const pa_threaded_mainloop) -> *const pa_mainloop_api,
    pub pa_threaded_mainloop_in_thread: unsafe extern "C" fn(m: *mut pa_threaded_mainloop) -> i32,
    pub pa_threaded_mainloop_set_name:
        unsafe extern "C" fn(m: *mut pa_threaded_mainloop, name: *const c_char),
    #[cfg(any(doc, feature = "pa_v13"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "pa_v13")))]
    pub pa_threaded_mainloop_once_unlocked: unsafe extern "C" fn(
        m: *mut pa_threaded_mainloop,
        callback: extern "C" fn(m: *mut pa_threaded_mainloop, userdata: *mut c_void),
        userdata: *mut c_void,
    ),

    // Standard mainloop related functions
    pub pa_mainloop_new: unsafe extern "C" fn() -> *mut pa_mainloop,
    pub pa_mainloop_free: unsafe extern "C" fn(m: *mut pa_mainloop),
    pub pa_mainloop_prepare: unsafe extern "C" fn(m: *mut pa_mainloop, timeout: i32) -> i32,
    pub pa_mainloop_poll: unsafe extern "C" fn(m: *mut pa_mainloop) -> i32,
    pub pa_mainloop_dispatch: unsafe extern "C" fn(m: *mut pa_mainloop) -> i32,
    pub pa_mainloop_get_retval: unsafe extern "C" fn(m: *const pa_mainloop) -> i32,
    pub pa_mainloop_iterate:
        unsafe extern "C" fn(m: *mut pa_mainloop, block: i32, retval: *mut i32) -> i32,
    pub pa_mainloop_run: unsafe extern "C" fn(m: *mut pa_mainloop, retval: *mut i32) -> i32,
    pub pa_mainloop_get_api: unsafe extern "C" fn(m: *const pa_mainloop) -> *const pa_mainloop_api,
    pub pa_mainloop_quit: unsafe extern "C" fn(m: *mut pa_mainloop, retval: i32),
    pub pa_mainloop_wakeup: unsafe extern "C" fn(m: *mut pa_mainloop),
    pub pa_mainloop_set_poll_func:
        unsafe extern "C" fn(m: *mut pa_mainloop, poll_func: pa_poll_func, userdata: *mut c_void),

    // Signal related functions
    pub pa_signal_init: unsafe extern "C" fn(api: *const pa_mainloop_api) -> i32,
    pub pa_signal_done: unsafe extern "C" fn(),
    pub pa_signal_new: unsafe extern "C" fn(
        sig: i32,
        callback: pa_signal_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_signal_event,
    pub pa_signal_free: unsafe extern "C" fn(e: *mut pa_signal_event),
    pub pa_signal_set_destroy:
        unsafe extern "C" fn(e: *mut pa_signal_event, callback: pa_signal_destroy_cb_t),

    // Mainloop API related functions
    pub pa_mainloop_api_once: unsafe extern "C" fn(
        m: *const pa_mainloop_api,
        callback: pa_mainloop_api_once_cb,
        userdata: *mut c_void,
    ),

    // Subscribe related functions
    pub pa_context_subscribe: unsafe extern "C" fn(
        c: *mut pa_context,
        m: pa_subscription_mask_t,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_set_subscribe_callback: unsafe extern "C" fn(
        c: *mut pa_context,
        cb: pa_context_subscribe_cb_t,
        userdata: *mut c_void,
    ),

    // Sample cache related functions
    pub pa_context_remove_sample: unsafe extern "C" fn(
        c: *mut pa_context,
        name: *const c_char,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,

    pub pa_context_play_sample: unsafe extern "C" fn(
        c: *mut pa_context,
        name: *const c_char,
        dev: *const c_char,
        volume: pa_volume_t,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,

    pub pa_context_play_sample_with_proplist: unsafe extern "C" fn(
        c: *mut pa_context,
        name: *const c_char,
        dev: *const c_char,
        volume: pa_volume_t,
        proplist: *const pa_proplist,
        cb: pa_context_play_sample_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,

    // Context related functions
    pub pa_context_new: unsafe extern "C" fn(
        mainloop: *const pa_mainloop_api,
        name: *const c_char,
    ) -> *mut pa_context,
    pub pa_context_new_with_proplist: unsafe extern "C" fn(
        mainloop: *const pa_mainloop_api,
        name: *const c_char,
        proplist: *const pa_proplist,
    ) -> *mut pa_context,
    pub pa_context_unref: unsafe extern "C" fn(c: *mut pa_context),
    pub pa_context_ref: unsafe extern "C" fn(c: *mut pa_context) -> *mut pa_context,
    pub pa_context_set_state_callback:
        unsafe extern "C" fn(c: *mut pa_context, cb: pa_context_notify_cb_t, userdata: *mut c_void),
    pub pa_context_set_event_callback:
        unsafe extern "C" fn(p: *mut pa_context, cb: pa_context_event_cb_t, userdata: *mut c_void),
    pub pa_context_errno: unsafe extern "C" fn(c: *const pa_context) -> i32,
    pub pa_context_is_pending: unsafe extern "C" fn(c: *const pa_context) -> i32,
    pub pa_context_get_state: unsafe extern "C" fn(c: *const pa_context) -> pa_context_state_t,
    pub pa_context_connect: unsafe extern "C" fn(
        c: *mut pa_context,
        server: *const c_char,
        flags: pa_context_flags_t,
        api: *const pa_spawn_api,
    ) -> i32,
    pub pa_context_disconnect: unsafe extern "C" fn(c: *mut pa_context),
    pub pa_context_drain: unsafe extern "C" fn(
        c: *mut pa_context,
        cb: pa_context_notify_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_exit_daemon: unsafe extern "C" fn(
        c: *mut pa_context,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_set_default_sink: unsafe extern "C" fn(
        c: *mut pa_context,
        name: *const c_char,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_set_default_source: unsafe extern "C" fn(
        c: *mut pa_context,
        name: *const c_char,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_is_local: unsafe extern "C" fn(c: *const pa_context) -> i32,
    pub pa_context_set_name: unsafe extern "C" fn(
        c: *mut pa_context,
        name: *const c_char,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_get_server: unsafe extern "C" fn(c: *const pa_context) -> *const c_char,
    pub pa_context_get_protocol_version: unsafe extern "C" fn(c: *const pa_context) -> u32,
    pub pa_context_get_server_protocol_version: unsafe extern "C" fn(c: *const pa_context) -> u32,
    pub pa_context_proplist_update: unsafe extern "C" fn(
        c: *mut pa_context,
        mode: pa_update_mode_t,
        p: *const pa_proplist,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_proplist_remove: unsafe extern "C" fn(
        c: *mut pa_context,
        keys: *const *const c_char,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_get_index: unsafe extern "C" fn(s: *const pa_context) -> u32,
    pub pa_context_rttime_new: unsafe extern "C" fn(
        c: *const pa_context,
        usec: pa_usec_t,
        cb: pa_time_event_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_time_event,
    pub pa_context_rttime_restart:
        unsafe extern "C" fn(c: *const pa_context, e: *mut pa_time_event, usec: pa_usec_t),
    pub pa_context_get_tile_size:
        unsafe extern "C" fn(c: *const pa_context, ss: *const pa_sample_spec) -> usize,
    pub pa_context_load_cookie_from_file:
        unsafe extern "C" fn(c: *mut pa_context, cookie_file_path: *const c_char) -> i32,

    // Introspect related functions - Sink
    pub pa_context_get_sink_info_by_name: unsafe extern "C" fn(
        c: *mut pa_context,
        name: *const c_char,
        cb: pa_sink_info_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_get_sink_info_by_index: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        cb: pa_sink_info_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_get_sink_info_list: unsafe extern "C" fn(
        c: *mut pa_context,
        cb: pa_sink_info_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_set_sink_volume_by_index: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        volume: *const pa_cvolume,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_set_sink_volume_by_name: unsafe extern "C" fn(
        c: *mut pa_context,
        name: *const c_char,
        volume: *const pa_cvolume,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_set_sink_mute_by_index: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        mute: i32,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_set_sink_mute_by_name: unsafe extern "C" fn(
        c: *mut pa_context,
        name: *const c_char,
        mute: i32,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_suspend_sink_by_name: unsafe extern "C" fn(
        c: *mut pa_context,
        sink_name: *const c_char,
        suspend: i32,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_suspend_sink_by_index: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        suspend: i32,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_set_sink_port_by_index: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        port: *const c_char,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_set_sink_port_by_name: unsafe extern "C" fn(
        c: *mut pa_context,
        name: *const c_char,
        port: *const c_char,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,

    // Introspect related functions - Source
    pub pa_context_get_source_info_by_name: unsafe extern "C" fn(
        c: *mut pa_context,
        name: *const c_char,
        cb: pa_source_info_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_get_source_info_by_index: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        cb: pa_source_info_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_get_source_info_list: unsafe extern "C" fn(
        c: *mut pa_context,
        cb: pa_source_info_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_set_source_volume_by_index: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        volume: *const pa_cvolume,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_set_source_volume_by_name: unsafe extern "C" fn(
        c: *mut pa_context,
        name: *const c_char,
        volume: *const pa_cvolume,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_set_source_mute_by_index: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        mute: i32,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_set_source_mute_by_name: unsafe extern "C" fn(
        c: *mut pa_context,
        name: *const c_char,
        mute: i32,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_suspend_source_by_name: unsafe extern "C" fn(
        c: *mut pa_context,
        source_name: *const c_char,
        suspend: i32,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_suspend_source_by_index: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        suspend: i32,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_set_source_port_by_index: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        port: *const c_char,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_set_source_port_by_name: unsafe extern "C" fn(
        c: *mut pa_context,
        name: *const c_char,
        port: *const c_char,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,

    // Introspect related functions - Server
    pub pa_context_get_server_info: unsafe extern "C" fn(
        c: *mut pa_context,
        cb: pa_server_info_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,

    // Introspect related functions - Module
    pub pa_context_get_module_info: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        cb: pa_module_info_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_get_module_info_list: unsafe extern "C" fn(
        c: *mut pa_context,
        cb: pa_module_info_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_load_module: unsafe extern "C" fn(
        c: *mut pa_context,
        name: *const c_char,
        argument: *const c_char,
        cb: pa_context_index_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_unload_module: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,

    #[cfg(any(doc, feature = "pa_v15"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "pa_v15")))]
    pub pa_context_send_message_to_object: unsafe extern "C" fn(
        c: *mut pa_context,
        recipient_name: *const c_char,
        message: *const c_char,
        message_parameters: *const c_char,
        cb: pa_context_string_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,

    // Introspect related functions - Client
    pub pa_context_get_client_info: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        cb: pa_client_info_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_get_client_info_list: unsafe extern "C" fn(
        c: *mut pa_context,
        cb: pa_client_info_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_kill_client: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,

    // Introspect related functions - Card
    pub pa_context_get_card_info_by_index: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        cb: pa_card_info_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_get_card_info_by_name: unsafe extern "C" fn(
        c: *mut pa_context,
        name: *const c_char,
        cb: pa_card_info_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_get_card_info_list: unsafe extern "C" fn(
        c: *mut pa_context,
        cb: pa_card_info_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_set_card_profile_by_index: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        profile: *const c_char,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_set_card_profile_by_name: unsafe extern "C" fn(
        c: *mut pa_context,
        name: *const c_char,
        profile: *const c_char,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_set_port_latency_offset: unsafe extern "C" fn(
        c: *mut pa_context,
        card_name: *const c_char,
        port_name: *const c_char,
        offset: i64,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,

    // Introspect related functions - Sink Input
    pub pa_context_get_sink_input_info: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        cb: pa_sink_input_info_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_get_sink_input_info_list: unsafe extern "C" fn(
        c: *mut pa_context,
        cb: pa_sink_input_info_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_move_sink_input_by_name: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        sink_name: *const c_char,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_move_sink_input_by_index: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        sink_idx: u32,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_set_sink_input_volume: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        volume: *const pa_cvolume,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_set_sink_input_mute: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        mute: i32,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_kill_sink_input: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,

    // Introspect related functions - Source Output
    pub pa_context_get_source_output_info: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        cb: pa_source_output_info_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_get_source_output_info_list: unsafe extern "C" fn(
        c: *mut pa_context,
        cb: pa_source_output_info_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_move_source_output_by_name: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        source_name: *const c_char,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_move_source_output_by_index: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        source_idx: u32,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_set_source_output_volume: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        volume: *const pa_cvolume,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_set_source_output_mute: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        mute: i32,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_kill_source_output: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,

    // Introspect related functions - Statistics
    pub pa_context_stat: unsafe extern "C" fn(
        c: *mut pa_context,
        cb: pa_stat_info_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,

    // Introspect related functions - Sample
    pub pa_context_get_sample_info_by_name: unsafe extern "C" fn(
        c: *mut pa_context,
        name: *const c_char,
        cb: pa_sample_info_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_get_sample_info_by_index: unsafe extern "C" fn(
        c: *mut pa_context,
        idx: u32,
        cb: pa_sample_info_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_context_get_sample_info_list: unsafe extern "C" fn(
        c: *mut pa_context,
        cb: pa_sample_info_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,

    // Extension: Stream Restore
    pub pa_ext_stream_restore_test: unsafe extern "C" fn(
        c: *mut pa_context,
        cb: pa_ext_stream_restore_test_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_ext_stream_restore_read: unsafe extern "C" fn(
        c: *mut pa_context,
        cb: pa_ext_stream_restore_read_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_ext_stream_restore_write: unsafe extern "C" fn(
        c: *mut pa_context,
        mode: pa_update_mode_t,
        data: *const pa_ext_stream_restore_info,
        n: u32,
        apply_immediately: i32,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_ext_stream_restore_delete: unsafe extern "C" fn(
        c: *mut pa_context,
        s: *const *const c_char,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_ext_stream_restore_subscribe: unsafe extern "C" fn(
        c: *mut pa_context,
        enable: i32,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_ext_stream_restore_set_subscribe_cb: unsafe extern "C" fn(
        c: *mut pa_context,
        cb: pa_ext_stream_restore_subscribe_cb_t,
        userdata: *mut c_void,
    ),

    // Extension: Device Restore
    pub pa_ext_device_restore_test: unsafe extern "C" fn(
        c: *mut pa_context,
        cb: pa_ext_device_restore_test_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_ext_device_restore_subscribe: unsafe extern "C" fn(
        c: *mut pa_context,
        enable: i32,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_ext_device_restore_set_subscribe_cb: unsafe extern "C" fn(
        c: *mut pa_context,
        cb: pa_ext_device_restore_subscribe_cb_t,
        userdata: *mut c_void,
    ),
    pub pa_ext_device_restore_read_formats_all: unsafe extern "C" fn(
        c: *mut pa_context,
        cb: pa_ext_device_restore_read_device_formats_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_ext_device_restore_read_formats: unsafe extern "C" fn(
        c: *mut pa_context,
        type_: pa_device_type_t,
        idx: u32,
        cb: pa_ext_device_restore_read_device_formats_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_ext_device_restore_save_formats: unsafe extern "C" fn(
        c: *mut pa_context,
        type_: pa_device_type_t,
        idx: u32,
        n_formats: u8,
        formats: *const *mut pa_format_info,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,

    // Extension: Device Manager function loading
    pub pa_ext_device_manager_test: unsafe extern "C" fn(
        c: *mut pa_context,
        cb: pa_ext_device_manager_test_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_ext_device_manager_read: unsafe extern "C" fn(
        c: *mut pa_context,
        cb: pa_ext_device_manager_read_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_ext_device_manager_set_device_description: unsafe extern "C" fn(
        c: *mut pa_context,
        device: *const c_char,
        description: *const c_char,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    )
        -> *mut pa_operation,
    pub pa_ext_device_manager_delete: unsafe extern "C" fn(
        c: *mut pa_context,
        s: *const *const c_char,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_ext_device_manager_enable_role_device_priority_routing:
        unsafe extern "C" fn(
            c: *mut pa_context,
            enable: i32,
            cb: pa_context_success_cb_t,
            userdata: *mut c_void,
        ) -> *mut pa_operation,
    pub pa_ext_device_manager_reorder_devices_for_role: unsafe extern "C" fn(
        c: *mut pa_context,
        role: *const c_char,
        devices: *const *const c_char,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    )
        -> *mut pa_operation,
    pub pa_ext_device_manager_subscribe: unsafe extern "C" fn(
        c: *mut pa_context,
        enable: i32,
        cb: pa_context_success_cb_t,
        userdata: *mut c_void,
    ) -> *mut pa_operation,
    pub pa_ext_device_manager_set_subscribe_cb: unsafe extern "C" fn(
        c: *mut pa_context,
        cb: pa_ext_device_manager_subscribe_cb_t,
        userdata: *mut c_void,
    ),

    // Operation related functions
    pub pa_operation_ref: unsafe extern "C" fn(o: *mut pa_operation) -> *mut pa_operation,
    pub pa_operation_unref: unsafe extern "C" fn(o: *mut pa_operation),
    pub pa_operation_cancel: unsafe extern "C" fn(o: *mut pa_operation),
    pub pa_operation_get_state:
        unsafe extern "C" fn(o: *const pa_operation) -> pa_operation_state_t,
    pub pa_operation_set_state_callback: unsafe extern "C" fn(
        o: *mut pa_operation,
        cb: pa_operation_notify_cb_t,
        userdata: *mut c_void,
    ),
}

impl PulseFunctions {
    pub(crate) unsafe fn load(lib: &libloading::Library) -> Result<Arc<Self>, libloading::Error> {
        Ok(Arc::new(Self {
            pa_stream_connect_upload: *lib.get(b"pa_stream_connect_upload\0")?,
            pa_stream_finish_upload: *lib.get(b"pa_stream_finish_upload\0")?,
            pa_stream_new: *lib.get(b"pa_stream_new\0")?,
            pa_stream_new_with_proplist: *lib.get(b"pa_stream_new_with_proplist\0")?,
            pa_stream_new_extended: *lib.get(b"pa_stream_new_extended\0")?,
            pa_stream_unref: *lib.get(b"pa_stream_unref\0")?,
            pa_stream_ref: *lib.get(b"pa_stream_ref\0")?,
            pa_stream_get_state: *lib.get(b"pa_stream_get_state\0")?,
            pa_stream_get_context: *lib.get(b"pa_stream_get_context\0")?,
            pa_stream_get_index: *lib.get(b"pa_stream_get_index\0")?,
            pa_stream_get_device_index: *lib.get(b"pa_stream_get_device_index\0")?,
            pa_stream_get_device_name: *lib.get(b"pa_stream_get_device_name\0")?,
            pa_stream_is_suspended: *lib.get(b"pa_stream_is_suspended\0")?,
            pa_stream_is_corked: *lib.get(b"pa_stream_is_corked\0")?,
            pa_stream_connect_playback: *lib.get(b"pa_stream_connect_playback\0")?,
            pa_stream_connect_record: *lib.get(b"pa_stream_connect_record\0")?,
            pa_stream_disconnect: *lib.get(b"pa_stream_disconnect\0")?,
            pa_stream_begin_write: *lib.get(b"pa_stream_begin_write\0")?,
            pa_stream_cancel_write: *lib.get(b"pa_stream_cancel_write\0")?,
            pa_stream_write: *lib.get(b"pa_stream_write\0")?,
            #[cfg(any(doc, feature = "pa_v6"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "pa_v6")))]
            pa_stream_write_ext_free: *lib.get(b"pa_stream_write_ext_free\0")?,
            pa_stream_peek: *lib.get(b"pa_stream_peek\0")?,
            pa_stream_drop: *lib.get(b"pa_stream_drop\0")?,
            pa_stream_writable_size: *lib.get(b"pa_stream_writable_size\0")?,
            pa_stream_readable_size: *lib.get(b"pa_stream_readable_size\0")?,
            pa_stream_drain: *lib.get(b"pa_stream_drain\0")?,
            pa_stream_update_timing_info: *lib.get(b"pa_stream_update_timing_info\0")?,
            pa_stream_set_state_callback: *lib.get(b"pa_stream_set_state_callback\0")?,
            pa_stream_set_write_callback: *lib.get(b"pa_stream_set_write_callback\0")?,
            pa_stream_set_read_callback: *lib.get(b"pa_stream_set_read_callback\0")?,
            pa_stream_set_overflow_callback: *lib.get(b"pa_stream_set_overflow_callback\0")?,
            pa_stream_get_underflow_index: *lib.get(b"pa_stream_get_underflow_index\0")?,
            pa_stream_set_underflow_callback: *lib.get(b"pa_stream_set_underflow_callback\0")?,
            pa_stream_set_started_callback: *lib.get(b"pa_stream_set_started_callback\0")?,
            pa_stream_set_latency_update_callback: *lib
                .get(b"pa_stream_set_latency_update_callback\0")?,
            pa_stream_set_moved_callback: *lib.get(b"pa_stream_set_moved_callback\0")?,
            pa_stream_set_suspended_callback: *lib.get(b"pa_stream_set_suspended_callback\0")?,
            pa_stream_set_event_callback: *lib.get(b"pa_stream_set_event_callback\0")?,
            pa_stream_set_buffer_attr_callback: *lib
                .get(b"pa_stream_set_buffer_attr_callback\0")?,
            pa_stream_cork: *lib.get(b"pa_stream_cork\0")?,
            pa_stream_flush: *lib.get(b"pa_stream_flush\0")?,
            pa_stream_prebuf: *lib.get(b"pa_stream_prebuf\0")?,
            pa_stream_trigger: *lib.get(b"pa_stream_trigger\0")?,
            pa_stream_set_name: *lib.get(b"pa_stream_set_name\0")?,
            pa_stream_get_time: *lib.get(b"pa_stream_get_time\0")?,
            pa_stream_get_latency: *lib.get(b"pa_stream_get_latency\0")?,
            pa_stream_get_timing_info: *lib.get(b"pa_stream_get_timing_info\0")?,
            pa_stream_get_sample_spec: *lib.get(b"pa_stream_get_sample_spec\0")?,
            pa_stream_get_channel_map: *lib.get(b"pa_stream_get_channel_map\0")?,
            pa_stream_get_format_info: *lib.get(b"pa_stream_get_format_info\0")?,
            pa_stream_get_buffer_attr: *lib.get(b"pa_stream_get_buffer_attr\0")?,
            pa_stream_set_buffer_attr: *lib.get(b"pa_stream_set_buffer_attr\0")?,
            pa_stream_update_sample_rate: *lib.get(b"pa_stream_update_sample_rate\0")?,
            pa_stream_proplist_update: *lib.get(b"pa_stream_proplist_update\0")?,
            pa_stream_proplist_remove: *lib.get(b"pa_stream_proplist_remove\0")?,
            pa_stream_set_monitor_stream: *lib.get(b"pa_stream_set_monitor_stream\0")?,
            pa_stream_get_monitor_stream: *lib.get(b"pa_stream_get_monitor_stream\0")?,
            // Timeval 相关函数加载
            pa_gettimeofday: *lib.get(b"pa_gettimeofday\0")?,
            pa_timeval_diff: *lib.get(b"pa_timeval_diff\0")?,
            pa_timeval_cmp: *lib.get(b"pa_timeval_cmp\0")?,
            pa_timeval_age: *lib.get(b"pa_timeval_age\0")?,
            pa_timeval_add: *lib.get(b"pa_timeval_add\0")?,
            pa_timeval_sub: *lib.get(b"pa_timeval_sub\0")?,
            pa_timeval_store: *lib.get(b"pa_timeval_store\0")?,
            pa_timeval_load: *lib.get(b"pa_timeval_load\0")?,
            // UTF8 相关函数加载
            pa_utf8_valid: *lib.get(b"pa_utf8_valid\0")?,
            pa_ascii_valid: *lib.get(b"pa_ascii_valid\0")?,
            pa_utf8_filter: *lib.get(b"pa_utf8_filter\0")?,
            pa_ascii_filter: *lib.get(b"pa_ascii_filter\0")?,
            pa_utf8_to_locale: *lib.get(b"pa_utf8_to_locale\0")?,
            pa_locale_to_utf8: *lib.get(b"pa_locale_to_utf8\0")?,
            // Utility function loading
            pa_get_user_name: *lib.get(b"pa_get_user_name\0")?,
            pa_get_host_name: *lib.get(b"pa_get_host_name\0")?,
            pa_get_fqdn: *lib.get(b"pa_get_fqdn\0")?,
            pa_get_home_dir: *lib.get(b"pa_get_home_dir\0")?,
            pa_get_binary_name: *lib.get(b"pa_get_binary_name\0")?,
            pa_path_get_filename: *lib.get(b"pa_path_get_filename\0")?,
            pa_msleep: *lib.get(b"pa_msleep\0")?,
            #[cfg(any(doc, feature = "pa_v13"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "pa_v13")))]
            pa_thread_make_realtime: *lib.get(b"pa_thread_make_realtime\0")?,
            // Version related function loading
            pa_get_library_version: *lib.get(b"pa_get_library_version\0")?,
            // Volume related function loading
            pa_cvolume_equal: *lib.get(b"pa_cvolume_equal\0")?,
            pa_cvolume_init: *lib.get(b"pa_cvolume_init\0")?,
            pa_cvolume_set: *lib.get(b"pa_cvolume_set\0")?,
            pa_cvolume_snprint: *lib.get(b"pa_cvolume_snprint\0")?,
            pa_sw_cvolume_snprint_dB: *lib.get(b"pa_sw_cvolume_snprint_dB\0")?,
            pa_cvolume_snprint_verbose: *lib.get(b"pa_cvolume_snprint_verbose\0")?,
            pa_volume_snprint: *lib.get(b"pa_volume_snprint\0")?,
            pa_sw_volume_snprint_dB: *lib.get(b"pa_sw_volume_snprint_dB\0")?,
            pa_volume_snprint_verbose: *lib.get(b"pa_volume_snprint_verbose\0")?,
            pa_cvolume_avg: *lib.get(b"pa_cvolume_avg\0")?,
            pa_cvolume_avg_mask: *lib.get(b"pa_cvolume_avg_mask\0")?,

            pa_cvolume_max: *lib.get(b"pa_cvolume_max\0")?,
            pa_cvolume_max_mask: *lib.get(b"pa_cvolume_max_mask\0")?,
            pa_cvolume_min: *lib.get(b"pa_cvolume_min\0")?,
            pa_cvolume_min_mask: *lib.get(b"pa_cvolume_min_mask\0")?,
            pa_cvolume_valid: *lib.get(b"pa_cvolume_valid\0")?,
            pa_cvolume_channels_equal_to: *lib.get(b"pa_cvolume_channels_equal_to\0")?,
            pa_sw_volume_multiply: *lib.get(b"pa_sw_volume_multiply\0")?,
            pa_sw_cvolume_multiply: *lib.get(b"pa_sw_cvolume_multiply\0")?,
            pa_sw_cvolume_multiply_scalar: *lib.get(b"pa_sw_cvolume_multiply_scalar\0")?,
            pa_sw_volume_divide: *lib.get(b"pa_sw_volume_divide\0")?,
            pa_sw_cvolume_divide: *lib.get(b"pa_sw_cvolume_divide\0")?,
            pa_sw_cvolume_divide_scalar: *lib.get(b"pa_sw_cvolume_divide_scalar\0")?,
            pa_sw_volume_from_dB: *lib.get(b"pa_sw_volume_from_dB\0")?,
            pa_sw_volume_to_dB: *lib.get(b"pa_sw_volume_to_dB\0")?,
            pa_sw_volume_from_linear: *lib.get(b"pa_sw_volume_from_linear\0")?,
            pa_sw_volume_to_linear: *lib.get(b"pa_sw_volume_to_linear\0")?,

            pa_cvolume_remap: *lib.get(b"pa_cvolume_remap\0")?,
            pa_cvolume_compatible: *lib.get(b"pa_cvolume_compatible\0")?,
            pa_cvolume_compatible_with_channel_map: *lib
                .get(b"pa_cvolume_compatible_with_channel_map\0")?,
            pa_cvolume_get_balance: *lib.get(b"pa_cvolume_get_balance\0")?,
            pa_cvolume_set_balance: *lib.get(b"pa_cvolume_set_balance\0")?,
            pa_cvolume_get_fade: *lib.get(b"pa_cvolume_get_fade\0")?,
            pa_cvolume_set_fade: *lib.get(b"pa_cvolume_set_fade\0")?,

            #[cfg(any(doc, feature = "pa_v8"))]
            pa_cvolume_get_lfe_balance: *lib.get(b"pa_cvolume_get_lfe_balance\0")?,
            #[cfg(any(doc, feature = "pa_v8"))]
            pa_cvolume_set_lfe_balance: *lib.get(b"pa_cvolume_set_lfe_balance\0")?,
            pa_cvolume_scale: *lib.get(b"pa_cvolume_scale\0")?,
            pa_cvolume_scale_mask: *lib.get(b"pa_cvolume_scale_mask\0")?,

            pa_cvolume_set_position: *lib.get(b"pa_cvolume_set_position\0")?,
            pa_cvolume_get_position: *lib.get(b"pa_cvolume_get_position\0")?,
            pa_cvolume_merge: *lib.get(b"pa_cvolume_merge\0")?,
            pa_cvolume_inc_clamp: *lib.get(b"pa_cvolume_inc_clamp\0")?,
            pa_cvolume_inc: *lib.get(b"pa_cvolume_inc\0")?,
            pa_cvolume_dec: *lib.get(b"pa_cvolume_dec\0")?,

            // Memory allocation function loading
            pa_xmalloc: *lib.get(b"pa_xmalloc\0")?,
            pa_xmalloc0: *lib.get(b"pa_xmalloc0\0")?,
            pa_xrealloc: *lib.get(b"pa_xrealloc\0")?,
            pa_xfree: *lib.get(b"pa_xfree\0")?,
            pa_xstrdup: *lib.get(b"pa_xstrdup\0")?,
            pa_xstrndup: *lib.get(b"pa_xstrndup\0")?,
            pa_xmemdup: *lib.get(b"pa_xmemdup\0")?,

            // Sample related function loading
            pa_bytes_per_second: *lib.get(b"pa_bytes_per_second\0")?,
            pa_frame_size: *lib.get(b"pa_frame_size\0")?,
            pa_sample_size: *lib.get(b"pa_sample_size\0")?,
            pa_sample_size_of_format: *lib.get(b"pa_sample_size_of_format\0")?,
            pa_bytes_to_usec: *lib.get(b"pa_bytes_to_usec\0")?,
            pa_usec_to_bytes: *lib.get(b"pa_usec_to_bytes\0")?,
            pa_sample_spec_init: *lib.get(b"pa_sample_spec_init\0")?,
            pa_sample_format_valid: *lib.get(b"pa_sample_format_valid\0")?,
            pa_sample_rate_valid: *lib.get(b"pa_sample_rate_valid\0")?,
            pa_channels_valid: *lib.get(b"pa_channels_valid\0")?,
            pa_sample_spec_valid: *lib.get(b"pa_sample_spec_valid\0")?,
            pa_sample_spec_equal: *lib.get(b"pa_sample_spec_equal\0")?,
            pa_sample_format_to_string: *lib.get(b"pa_sample_format_to_string\0")?,
            pa_parse_sample_format: *lib.get(b"pa_parse_sample_format\0")?,
            pa_sample_spec_snprint: *lib.get(b"pa_sample_spec_snprint\0")?,
            pa_bytes_snprint: *lib.get(b"pa_bytes_snprint\0")?,
            pa_sample_format_is_le: *lib.get(b"pa_sample_format_is_le\0")?,
            pa_sample_format_is_be: *lib.get(b"pa_sample_format_is_be\0")?,

            // RtClock related function loading
            pa_rtclock_now: *lib.get(b"pa_rtclock_now\0")?,

            // Proplist related function loading
            pa_proplist_new: *lib.get(b"pa_proplist_new\0")?,
            pa_proplist_free: *lib.get(b"pa_proplist_free\0")?,
            pa_proplist_key_valid: *lib.get(b"pa_proplist_key_valid\0")?,
            pa_proplist_sets: *lib.get(b"pa_proplist_sets\0")?,
            pa_proplist_setp: *lib.get(b"pa_proplist_setp\0")?,
            // pa_proplist_setf: *lib.get(b"pa_proplist_setf\0")?,
            pa_proplist_set: *lib.get(b"pa_proplist_set\0")?,
            pa_proplist_gets: *lib.get(b"pa_proplist_gets\0")?,
            pa_proplist_get: *lib.get(b"pa_proplist_get\0")?,
            pa_proplist_update: *lib.get(b"pa_proplist_update\0")?,
            pa_proplist_unset: *lib.get(b"pa_proplist_unset\0")?,
            pa_proplist_unset_many: *lib.get(b"pa_proplist_unset_many\0")?,
            pa_proplist_iterate: *lib.get(b"pa_proplist_iterate\0")?,
            pa_proplist_to_string: *lib.get(b"pa_proplist_to_string\0")?,
            pa_proplist_to_string_sep: *lib.get(b"pa_proplist_to_string_sep\0")?,
            pa_proplist_from_string: *lib.get(b"pa_proplist_from_string\0")?,
            pa_proplist_contains: *lib.get(b"pa_proplist_contains\0")?,
            pa_proplist_clear: *lib.get(b"pa_proplist_clear\0")?,
            pa_proplist_copy: *lib.get(b"pa_proplist_copy\0")?,
            pa_proplist_size: *lib.get(b"pa_proplist_size\0")?,
            pa_proplist_isempty: *lib.get(b"pa_proplist_isempty\0")?,
            pa_proplist_equal: *lib.get(b"pa_proplist_equal\0")?,

            // Format related function loading
            pa_encoding_to_string: *lib.get(b"pa_encoding_to_string\0")?,
            #[cfg(any(doc, feature = "pa_v12"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "pa_v12")))]
            pa_encoding_from_string: *lib.get(b"pa_encoding_from_string\0")?,

            pa_format_info_new: *lib.get(b"pa_format_info_new\0")?,
            pa_format_info_copy: *lib.get(b"pa_format_info_copy\0")?,
            pa_format_info_free: *lib.get(b"pa_format_info_free\0")?,
            pa_format_info_valid: *lib.get(b"pa_format_info_valid\0")?,
            pa_format_info_is_pcm: *lib.get(b"pa_format_info_is_pcm\0")?,
            pa_format_info_is_compatible: *lib.get(b"pa_format_info_is_compatible\0")?,
            pa_format_info_snprint: *lib.get(b"pa_format_info_snprint\0")?,
            pa_format_info_from_string: *lib.get(b"pa_format_info_from_string\0")?,
            pa_format_info_from_sample_spec: *lib.get(b"pa_format_info_from_sample_spec\0")?,
            pa_format_info_to_sample_spec: *lib.get(b"pa_format_info_to_sample_spec\0")?,
            pa_format_info_get_prop_type: *lib.get(b"pa_format_info_get_prop_type\0")?,
            pa_format_info_get_prop_int: *lib.get(b"pa_format_info_get_prop_int\0")?,
            pa_format_info_get_prop_int_range: *lib.get(b"pa_format_info_get_prop_int_range\0")?,
            pa_format_info_get_prop_int_array: *lib.get(b"pa_format_info_get_prop_int_array\0")?,
            pa_format_info_get_prop_string: *lib.get(b"pa_format_info_get_prop_string\0")?,
            pa_format_info_get_prop_string_array: *lib
                .get(b"pa_format_info_get_prop_string_array\0")?,
            pa_format_info_free_string_array: *lib.get(b"pa_format_info_free_string_array\0")?,
            #[cfg(any(doc, feature = "pa_v13"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "pa_v13")))]
            pa_format_info_get_sample_format: *lib.get(b"pa_format_info_get_sample_format\0")?,
            #[cfg(any(doc, feature = "pa_v13"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "pa_v13")))]
            pa_format_info_get_rate: *lib.get(b"pa_format_info_get_rate\0")?,
            #[cfg(any(doc, feature = "pa_v13"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "pa_v13")))]
            pa_format_info_get_channels: *lib.get(b"pa_format_info_get_channels\0")?,
            #[cfg(any(doc, feature = "pa_v13"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "pa_v13")))]
            pa_format_info_get_channel_map: *lib.get(b"pa_format_info_get_channel_map\0")?,
            pa_format_info_set_prop_int: *lib.get(b"pa_format_info_set_prop_int\0")?,
            pa_format_info_set_prop_int_array: *lib.get(b"pa_format_info_set_prop_int_array\0")?,
            pa_format_info_set_prop_int_range: *lib.get(b"pa_format_info_set_prop_int_range\0")?,
            pa_format_info_set_prop_string: *lib.get(b"pa_format_info_set_prop_string\0")?,
            pa_format_info_set_prop_string_array: *lib
                .get(b"pa_format_info_set_prop_string_array\0")?,
            pa_format_info_set_sample_format: *lib.get(b"pa_format_info_set_sample_format\0")?,
            pa_format_info_set_rate: *lib.get(b"pa_format_info_set_rate\0")?,
            pa_format_info_set_channels: *lib.get(b"pa_format_info_set_channels\0")?,
            pa_format_info_set_channel_map: *lib.get(b"pa_format_info_set_channel_map\0")?,

            // Error related function loading
            pa_strerror: *lib.get(b"pa_strerror\0")?,

            // Direction related function loading
            #[cfg(any(doc, feature = "pa_v6"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "pa_v6")))]
            pa_direction_valid: *lib.get(b"pa_direction_valid\0")?,
            #[cfg(any(doc, feature = "pa_v6"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "pa_v6")))]
            pa_direction_to_string: *lib.get(b"pa_direction_to_string\0")?,

            // Channelmap related function loading
            pa_channel_map_init: *lib.get(b"pa_channel_map_init\0")?,
            pa_channel_map_init_mono: *lib.get(b"pa_channel_map_init_mono\0")?,
            pa_channel_map_init_stereo: *lib.get(b"pa_channel_map_init_stereo\0")?,
            pa_channel_map_init_auto: *lib.get(b"pa_channel_map_init_auto\0")?,
            pa_channel_map_init_extend: *lib.get(b"pa_channel_map_init_extend\0")?,
            pa_channel_position_to_string: *lib.get(b"pa_channel_position_to_string\0")?,
            pa_channel_position_from_string: *lib.get(b"pa_channel_position_from_string\0")?,
            pa_channel_position_to_pretty_string: *lib
                .get(b"pa_channel_position_to_pretty_string\0")?,
            pa_channel_map_snprint: *lib.get(b"pa_channel_map_snprint\0")?,
            pa_channel_map_parse: *lib.get(b"pa_channel_map_parse\0")?,
            pa_channel_map_equal: *lib.get(b"pa_channel_map_equal\0")?,
            pa_channel_map_valid: *lib.get(b"pa_channel_map_valid\0")?,
            pa_channel_map_compatible: *lib.get(b"pa_channel_map_compatible\0")?,
            pa_channel_map_superset: *lib.get(b"pa_channel_map_superset\0")?,
            pa_channel_map_can_balance: *lib.get(b"pa_channel_map_can_balance\0")?,
            pa_channel_map_can_fade: *lib.get(b"pa_channel_map_can_fade\0")?,
            #[cfg(any(doc, feature = "pa_v8"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "pa_v8")))]
            pa_channel_map_can_lfe_balance: *lib.get(b"pa_channel_map_can_lfe_balance\0")?,
            pa_channel_map_to_name: *lib.get(b"pa_channel_map_to_name\0")?,
            pa_channel_map_to_pretty_name: *lib.get(b"pa_channel_map_to_pretty_name\0")?,
            pa_channel_map_has_position: *lib.get(b"pa_channel_map_has_position\0")?,
            pa_channel_map_mask: *lib.get(b"pa_channel_map_mask\0")?,

            // Threaded mainloop related function loading
            pa_threaded_mainloop_new: *lib.get(b"pa_threaded_mainloop_new\0")?,
            pa_threaded_mainloop_free: *lib.get(b"pa_threaded_mainloop_free\0")?,
            pa_threaded_mainloop_start: *lib.get(b"pa_threaded_mainloop_start\0")?,
            pa_threaded_mainloop_stop: *lib.get(b"pa_threaded_mainloop_stop\0")?,
            pa_threaded_mainloop_lock: *lib.get(b"pa_threaded_mainloop_lock\0")?,
            pa_threaded_mainloop_unlock: *lib.get(b"pa_threaded_mainloop_unlock\0")?,
            pa_threaded_mainloop_wait: *lib.get(b"pa_threaded_mainloop_wait\0")?,
            pa_threaded_mainloop_signal: *lib.get(b"pa_threaded_mainloop_signal\0")?,
            pa_threaded_mainloop_accept: *lib.get(b"pa_threaded_mainloop_accept\0")?,
            pa_threaded_mainloop_get_retval: *lib.get(b"pa_threaded_mainloop_get_retval\0")?,
            pa_threaded_mainloop_get_api: *lib.get(b"pa_threaded_mainloop_get_api\0")?,
            pa_threaded_mainloop_in_thread: *lib.get(b"pa_threaded_mainloop_in_thread\0")?,
            pa_threaded_mainloop_set_name: *lib.get(b"pa_threaded_mainloop_set_name\0")?,
            #[cfg(any(doc, feature = "pa_v13"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "pa_v13")))]
            pa_threaded_mainloop_once_unlocked: *lib
                .get(b"pa_threaded_mainloop_once_unlocked\0")?,

            // Standard mainloop related function loading
            pa_mainloop_new: *lib.get(b"pa_mainloop_new\0")?,
            pa_mainloop_free: *lib.get(b"pa_mainloop_free\0")?,
            pa_mainloop_prepare: *lib.get(b"pa_mainloop_prepare\0")?,
            pa_mainloop_poll: *lib.get(b"pa_mainloop_poll\0")?,
            pa_mainloop_dispatch: *lib.get(b"pa_mainloop_dispatch\0")?,
            pa_mainloop_get_retval: *lib.get(b"pa_mainloop_get_retval\0")?,
            pa_mainloop_iterate: *lib.get(b"pa_mainloop_iterate\0")?,
            pa_mainloop_run: *lib.get(b"pa_mainloop_run\0")?,
            pa_mainloop_get_api: *lib.get(b"pa_mainloop_get_api\0")?,
            pa_mainloop_quit: *lib.get(b"pa_mainloop_quit\0")?,
            pa_mainloop_wakeup: *lib.get(b"pa_mainloop_wakeup\0")?,
            pa_mainloop_set_poll_func: *lib.get(b"pa_mainloop_set_poll_func\0")?,

            // Signal related function loading
            pa_signal_init: *lib.get(b"pa_signal_init\0")?,
            pa_signal_done: *lib.get(b"pa_signal_done\0")?,
            pa_signal_new: *lib.get(b"pa_signal_new\0")?,
            pa_signal_free: *lib.get(b"pa_signal_free\0")?,
            pa_signal_set_destroy: *lib.get(b"pa_signal_set_destroy\0")?,

            // Mainloop API related function loading
            pa_mainloop_api_once: *lib.get(b"pa_mainloop_api_once\0")?,

            // Subscribe related function loading
            pa_context_subscribe: *lib.get(b"pa_context_subscribe\0")?,
            pa_context_set_subscribe_callback: *lib.get(b"pa_context_set_subscribe_callback\0")?,

            // Sample cache related function loading
            pa_context_remove_sample: *lib.get(b"pa_context_remove_sample\0")?,
            pa_context_play_sample: *lib.get(b"pa_context_play_sample\0")?,
            pa_context_play_sample_with_proplist: *lib
                .get(b"pa_context_play_sample_with_proplist\0")?,

            // Context related function loading
            pa_context_new: *lib.get(b"pa_context_new\0")?,
            pa_context_new_with_proplist: *lib.get(b"pa_context_new_with_proplist\0")?,
            pa_context_unref: *lib.get(b"pa_context_unref\0")?,
            pa_context_ref: *lib.get(b"pa_context_ref\0")?,
            pa_context_set_state_callback: *lib.get(b"pa_context_set_state_callback\0")?,
            pa_context_set_event_callback: *lib.get(b"pa_context_set_event_callback\0")?,
            pa_context_errno: *lib.get(b"pa_context_errno\0")?,
            pa_context_is_pending: *lib.get(b"pa_context_is_pending\0")?,
            pa_context_get_state: *lib.get(b"pa_context_get_state\0")?,
            pa_context_connect: *lib.get(b"pa_context_connect\0")?,
            pa_context_disconnect: *lib.get(b"pa_context_disconnect\0")?,
            pa_context_drain: *lib.get(b"pa_context_drain\0")?,
            pa_context_exit_daemon: *lib.get(b"pa_context_exit_daemon\0")?,
            pa_context_set_default_sink: *lib.get(b"pa_context_set_default_sink\0")?,
            pa_context_set_default_source: *lib.get(b"pa_context_set_default_source\0")?,
            pa_context_is_local: *lib.get(b"pa_context_is_local\0")?,
            pa_context_set_name: *lib.get(b"pa_context_set_name\0")?,
            pa_context_get_server: *lib.get(b"pa_context_get_server\0")?,
            pa_context_get_protocol_version: *lib.get(b"pa_context_get_protocol_version\0")?,
            pa_context_get_server_protocol_version: *lib
                .get(b"pa_context_get_server_protocol_version\0")?,
            pa_context_proplist_update: *lib.get(b"pa_context_proplist_update\0")?,
            pa_context_proplist_remove: *lib.get(b"pa_context_proplist_remove\0")?,
            pa_context_get_index: *lib.get(b"pa_context_get_index\0")?,
            pa_context_rttime_new: *lib.get(b"pa_context_rttime_new\0")?,
            pa_context_rttime_restart: *lib.get(b"pa_context_rttime_restart\0")?,
            pa_context_get_tile_size: *lib.get(b"pa_context_get_tile_size\0")?,
            pa_context_load_cookie_from_file: *lib.get(b"pa_context_load_cookie_from_file\0")?,

            // Introspect related function loading - Sink
            pa_context_get_sink_info_by_name: *lib.get(b"pa_context_get_sink_info_by_name\0")?,
            pa_context_get_sink_info_by_index: *lib.get(b"pa_context_get_sink_info_by_index\0")?,
            pa_context_get_sink_info_list: *lib.get(b"pa_context_get_sink_info_list\0")?,
            pa_context_set_sink_volume_by_index: *lib
                .get(b"pa_context_set_sink_volume_by_index\0")?,
            pa_context_set_sink_volume_by_name: *lib
                .get(b"pa_context_set_sink_volume_by_name\0")?,
            pa_context_set_sink_mute_by_index: *lib.get(b"pa_context_set_sink_mute_by_index\0")?,
            pa_context_set_sink_mute_by_name: *lib.get(b"pa_context_set_sink_mute_by_name\0")?,
            pa_context_suspend_sink_by_name: *lib.get(b"pa_context_suspend_sink_by_name\0")?,
            pa_context_suspend_sink_by_index: *lib.get(b"pa_context_suspend_sink_by_index\0")?,
            pa_context_set_sink_port_by_index: *lib.get(b"pa_context_set_sink_port_by_index\0")?,
            pa_context_set_sink_port_by_name: *lib.get(b"pa_context_set_sink_port_by_name\0")?,

            // Introspect related function loading - Source
            pa_context_get_source_info_by_name: *lib
                .get(b"pa_context_get_source_info_by_name\0")?,
            pa_context_get_source_info_by_index: *lib
                .get(b"pa_context_get_source_info_by_index\0")?,
            pa_context_get_source_info_list: *lib.get(b"pa_context_get_source_info_list\0")?,
            pa_context_set_source_volume_by_index: *lib
                .get(b"pa_context_set_source_volume_by_index\0")?,
            pa_context_set_source_volume_by_name: *lib
                .get(b"pa_context_set_source_volume_by_name\0")?,
            pa_context_set_source_mute_by_index: *lib
                .get(b"pa_context_set_source_mute_by_index\0")?,
            pa_context_set_source_mute_by_name: *lib
                .get(b"pa_context_set_source_mute_by_name\0")?,
            pa_context_suspend_source_by_name: *lib.get(b"pa_context_suspend_source_by_name\0")?,
            pa_context_suspend_source_by_index: *lib
                .get(b"pa_context_suspend_source_by_index\0")?,
            pa_context_set_source_port_by_index: *lib
                .get(b"pa_context_set_source_port_by_index\0")?,
            pa_context_set_source_port_by_name: *lib
                .get(b"pa_context_set_source_port_by_name\0")?,

            // Introspect related function loading - Server
            pa_context_get_server_info: *lib.get(b"pa_context_get_server_info\0")?,

            // Introspect related function loading - Module
            pa_context_get_module_info: *lib.get(b"pa_context_get_module_info\0")?,
            pa_context_get_module_info_list: *lib.get(b"pa_context_get_module_info_list\0")?,
            pa_context_load_module: *lib.get(b"pa_context_load_module\0")?,
            pa_context_unload_module: *lib.get(b"pa_context_unload_module\0")?,

            #[cfg(any(doc, feature = "pa_v15"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "pa_v15")))]
            pa_context_send_message_to_object: *lib.get(b"pa_context_send_message_to_object\0")?,

            // Introspect related function loading - Client
            pa_context_get_client_info: *lib.get(b"pa_context_get_client_info\0")?,
            pa_context_get_client_info_list: *lib.get(b"pa_context_get_client_info_list\0")?,
            pa_context_kill_client: *lib.get(b"pa_context_kill_client\0")?,

            // Introspect related function loading - Card
            pa_context_get_card_info_by_index: *lib.get(b"pa_context_get_card_info_by_index\0")?,
            pa_context_get_card_info_by_name: *lib.get(b"pa_context_get_card_info_by_name\0")?,
            pa_context_get_card_info_list: *lib.get(b"pa_context_get_card_info_list\0")?,
            pa_context_set_card_profile_by_index: *lib
                .get(b"pa_context_set_card_profile_by_index\0")?,
            pa_context_set_card_profile_by_name: *lib
                .get(b"pa_context_set_card_profile_by_name\0")?,
            pa_context_set_port_latency_offset: *lib
                .get(b"pa_context_set_port_latency_offset\0")?,

            // Introspect related function loading - Sink Input
            pa_context_get_sink_input_info: *lib.get(b"pa_context_get_sink_input_info\0")?,
            pa_context_get_sink_input_info_list: *lib
                .get(b"pa_context_get_sink_input_info_list\0")?,
            pa_context_move_sink_input_by_name: *lib
                .get(b"pa_context_move_sink_input_by_name\0")?,
            pa_context_move_sink_input_by_index: *lib
                .get(b"pa_context_move_sink_input_by_index\0")?,
            pa_context_set_sink_input_volume: *lib.get(b"pa_context_set_sink_input_volume\0")?,
            pa_context_set_sink_input_mute: *lib.get(b"pa_context_set_sink_input_mute\0")?,
            pa_context_kill_sink_input: *lib.get(b"pa_context_kill_sink_input\0")?,

            // Introspect related function loading - Source Output
            pa_context_get_source_output_info: *lib.get(b"pa_context_get_source_output_info\0")?,
            pa_context_get_source_output_info_list: *lib
                .get(b"pa_context_get_source_output_info_list\0")?,
            pa_context_move_source_output_by_name: *lib
                .get(b"pa_context_move_source_output_by_name\0")?,
            pa_context_move_source_output_by_index: *lib
                .get(b"pa_context_move_source_output_by_index\0")?,
            pa_context_set_source_output_volume: *lib
                .get(b"pa_context_set_source_output_volume\0")?,
            pa_context_set_source_output_mute: *lib.get(b"pa_context_set_source_output_mute\0")?,
            pa_context_kill_source_output: *lib.get(b"pa_context_kill_source_output\0")?,

            // Introspect related function loading - Statistics
            pa_context_stat: *lib.get(b"pa_context_stat\0")?,

            // Introspect related function loading - Sample
            pa_context_get_sample_info_by_name: *lib
                .get(b"pa_context_get_sample_info_by_name\0")?,
            pa_context_get_sample_info_by_index: *lib
                .get(b"pa_context_get_sample_info_by_index\0")?,
            pa_context_get_sample_info_list: *lib.get(b"pa_context_get_sample_info_list\0")?,

            // Extension: Stream Restore
            pa_ext_stream_restore_test: *lib.get(b"pa_ext_stream_restore_test\0")?,
            pa_ext_stream_restore_read: *lib.get(b"pa_ext_stream_restore_read\0")?,
            pa_ext_stream_restore_write: *lib.get(b"pa_ext_stream_restore_write\0")?,
            pa_ext_stream_restore_delete: *lib.get(b"pa_ext_stream_restore_delete\0")?,
            pa_ext_stream_restore_subscribe: *lib.get(b"pa_ext_stream_restore_subscribe\0")?,
            pa_ext_stream_restore_set_subscribe_cb: *lib
                .get(b"pa_ext_stream_restore_set_subscribe_cb\0")?,

            // Extension: Device Restore
            pa_ext_device_restore_test: *lib.get(b"pa_ext_device_restore_test\0")?,
            pa_ext_device_restore_subscribe: *lib.get(b"pa_ext_device_restore_subscribe\0")?,
            pa_ext_device_restore_set_subscribe_cb: *lib
                .get(b"pa_ext_device_restore_set_subscribe_cb\0")?,
            pa_ext_device_restore_read_formats_all: *lib
                .get(b"pa_ext_device_restore_read_formats_all\0")?,
            pa_ext_device_restore_read_formats: *lib
                .get(b"pa_ext_device_restore_read_formats\0")?,
            pa_ext_device_restore_save_formats: *lib
                .get(b"pa_ext_device_restore_save_formats\0")?,

            // Extension: Device Manager function loading
            pa_ext_device_manager_test: *lib.get(b"pa_ext_device_manager_test\0")?,
            pa_ext_device_manager_read: *lib.get(b"pa_ext_device_manager_read\0")?,
            pa_ext_device_manager_set_device_description: *lib
                .get(b"pa_ext_device_manager_set_device_description\0")?,
            pa_ext_device_manager_delete: *lib.get(b"pa_ext_device_manager_delete\0")?,
            pa_ext_device_manager_enable_role_device_priority_routing: *lib
                .get(b"pa_ext_device_manager_enable_role_device_priority_routing\0")?,
            pa_ext_device_manager_reorder_devices_for_role: *lib
                .get(b"pa_ext_device_manager_reorder_devices_for_role\0")?,
            pa_ext_device_manager_subscribe: *lib.get(b"pa_ext_device_manager_subscribe\0")?,
            pa_ext_device_manager_set_subscribe_cb: *lib
                .get(b"pa_ext_device_manager_set_subscribe_cb\0")?,

            // Operation related function loading
            pa_operation_ref: *lib.get(b"pa_operation_ref\0")?,
            pa_operation_unref: *lib.get(b"pa_operation_unref\0")?,
            pa_operation_cancel: *lib.get(b"pa_operation_cancel\0")?,
            pa_operation_get_state: *lib.get(b"pa_operation_get_state\0")?,
            pa_operation_set_state_callback: *lib.get(b"pa_operation_set_state_callback\0")?,
        }))
    }
}

static LIBRARY: OnceCell<Mutex<Option<Library>>> = OnceCell::new();
static FUNCTIONS: OnceCell<Option<Arc<PulseFunctions>>> = once_cell::sync::OnceCell::new();

pub fn get_functions() -> Option<Arc<PulseFunctions>> {
    init().ok()?;
    FUNCTIONS.get().map(|x| x.clone()).flatten()
}

fn get_library() -> &'static Mutex<Option<Library>> {
    LIBRARY.get_or_init(|| {
        let paths = ["libpulse.so.0", "libpulse.so"];

        for path in &paths {
            if let Ok(lib) = unsafe { Library::new(path) } {
                return Mutex::new(Some(lib));
            }
        }
        Mutex::new(None)
    })
}

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let functions = FUNCTIONS.get_or_init(|| {
        let lib = get_library().lock().ok()?;
        let lib = lib.as_ref()?;
        let functions = unsafe { PulseFunctions::load(&lib) };
        match functions {
            Ok(functions) => Some(functions),
            Err(e) => {
                eprintln!("Failed to load pulse functions: {}", e);
                None
            }
        }
    });
    if functions.is_some() {
        return Ok(());
    }
    Err("Failed to load pulse library".into())
}

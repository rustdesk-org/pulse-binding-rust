use crate::pa_simple;
use libloading::Library;
use libpulse_sys::{
    channelmap::pa_channel_map,
    def::pa_buffer_attr,
    sample::{pa_sample_spec, pa_usec_t},
    stream::pa_stream_direction_t,
};
use once_cell::sync::OnceCell;
use std::{
    os::raw::{c_char, c_void},
    sync::{Arc, Mutex},
};

pub struct PulseSimpleFunctions {
    pub pa_simple_new: unsafe extern "C" fn(
        server: *const c_char,
        name: *const c_char,
        dir: pa_stream_direction_t,
        dev: *const c_char,
        stream_name: *const c_char,
        ss: *const pa_sample_spec,
        map: *const pa_channel_map,
        attr: *const pa_buffer_attr,
        error: *mut i32,
    ) -> *mut pa_simple,
    pub pa_simple_free: unsafe extern "C" fn(*mut pa_simple),
    pub pa_simple_write:
        unsafe extern "C" fn(*mut pa_simple, *const c_void, usize, *mut i32) -> i32,
    pub pa_simple_drain: unsafe extern "C" fn(*mut pa_simple, *mut i32) -> i32,
    pub pa_simple_read: unsafe extern "C" fn(*mut pa_simple, *mut c_void, usize, *mut i32) -> i32,
    pub pa_simple_get_latency: unsafe extern "C" fn(*mut pa_simple, *mut i32) -> pa_usec_t,
    pub pa_simple_flush: unsafe extern "C" fn(*mut pa_simple, *mut i32) -> i32,
}

impl PulseSimpleFunctions {
    pub(crate) unsafe fn load(lib: &libloading::Library) -> Result<Arc<Self>, libloading::Error> {
        Ok(Arc::new(Self {
            pa_simple_new: *lib.get(b"pa_simple_new\0")?,
            pa_simple_free: *lib.get(b"pa_simple_free\0")?,
            pa_simple_write: *lib.get(b"pa_simple_write\0")?,
            pa_simple_drain: *lib.get(b"pa_simple_drain\0")?,
            pa_simple_read: *lib.get(b"pa_simple_read\0")?,
            pa_simple_get_latency: *lib.get(b"pa_simple_get_latency\0")?,
            pa_simple_flush: *lib.get(b"pa_simple_flush\0")?,
        }))
    }
}

static LIBRARY: OnceCell<Mutex<Option<Library>>> = OnceCell::new();
static FUNCTIONS: OnceCell<Option<Arc<PulseSimpleFunctions>>> = OnceCell::new();

pub fn get_functions() -> Option<Arc<PulseSimpleFunctions>> {
    init().ok()?;
    FUNCTIONS.get().map(|x| x.clone()).flatten()
}

fn get_library() -> &'static Mutex<Option<Library>> {
    // get_or_init: init once even failed
    // get_or_try_init: init until success
    LIBRARY.get_or_init(|| {
        let paths = ["libpulse-simple.so.0", "libpulse-simple.so"];
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
        let functions = unsafe { PulseSimpleFunctions::load(&lib) };
        match functions {
            Ok(functions) => Some(functions),
            Err(e) => {
                eprintln!("Failed to load pulse-simple functions: {}", e);
                None
            }
        }
    });
    if functions.is_some() {
        return Ok(());
    }
    Err("Failed to load pulse-simple library".into())
}

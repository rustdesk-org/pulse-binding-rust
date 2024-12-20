use crate::pa_glib_mainloop;
use glib_sys::GMainContext;
use libloading::Library;
use libpulse_sys::mainloop::api::pa_mainloop_api;
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};

pub struct PulseGlibFunctions {
    pub pa_glib_mainloop_new: unsafe extern "C" fn(*mut GMainContext) -> *mut pa_glib_mainloop,
    pub pa_glib_mainloop_free: unsafe extern "C" fn(*mut pa_glib_mainloop),
    pub pa_glib_mainloop_get_api:
        unsafe extern "C" fn(*const pa_glib_mainloop) -> *const pa_mainloop_api,
}

impl PulseGlibFunctions {
    pub(crate) unsafe fn load(lib: &libloading::Library) -> Result<Arc<Self>, libloading::Error> {
        Ok(Arc::new(Self {
            pa_glib_mainloop_new: *lib.get(b"pa_glib_mainloop_new\0")?,
            pa_glib_mainloop_free: *lib.get(b"pa_glib_mainloop_free\0")?,
            pa_glib_mainloop_get_api: *lib.get(b"pa_glib_mainloop_get_api\0")?,
        }))
    }
}

static LIBRARY: OnceCell<Mutex<Option<Library>>> = OnceCell::new();
static FUNCTIONS: OnceCell<Option<Arc<PulseGlibFunctions>>> = OnceCell::new();

pub fn get_functions() -> Option<Arc<PulseGlibFunctions>> {
    init().ok()?;
    FUNCTIONS.get().map(|x| x.clone()).flatten()
}

fn get_library() -> &'static Mutex<Option<Library>> {
    LIBRARY.get_or_init(|| {
        let paths = ["libpulse-mainloop-glib.so.0", "libpulse-mainloop-glib.so"];
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
        let functions = unsafe { PulseGlibFunctions::load(&lib) };
        match functions {
            Ok(functions) => Some(functions),
            Err(e) => {
                eprintln!("Failed to load libpulse-mainloop-glib functions: {}", e);
                None
            }
        }
    });
    if functions.is_some() {
        return Ok(());
    }
    Err("Failed to load ibpulse-mainloop-glib library".into())
}

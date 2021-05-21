use libc::{c_int, c_void, sighandler_t, signal, SIGABRT, SIGILL};

extern "C" fn handler(_: c_int) {
    eprintln!("Abort: Unexpected error.");
    std::process::exit(127);
}

fn get_handler() -> sighandler_t {
    handler as extern "C" fn(c_int) as *mut c_void as sighandler_t
}

pub fn setup() {
    unsafe {
        signal(SIGABRT, get_handler());
        signal(SIGILL, get_handler());
    }
}

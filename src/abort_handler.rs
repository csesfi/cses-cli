use libc::{c_int, c_void, sighandler_t};

extern "C" fn handler(_: c_int) {
    eprintln!("Abort: Unexpected error.");
    std::process::exit(127);
}

#[allow(unused)]
fn get_handler() -> sighandler_t {
    handler as *mut c_void as sighandler_t
}

pub fn setup() {
    #[cfg(unix)]
    unsafe {
        use libc::{sigaction, sigemptyset, sigset_t, SIGABRT, SIGILL};
        use std::{mem, ptr};
        let mut descriptor: sigaction = mem::zeroed();
        descriptor.sa_sigaction = get_handler();
        sigemptyset(&mut descriptor.sa_mask as *mut sigset_t);
        sigaction(SIGABRT, &mut descriptor as *mut sigaction, ptr::null_mut());
        sigaction(SIGILL, &mut descriptor as *mut sigaction, ptr::null_mut());
    }
}

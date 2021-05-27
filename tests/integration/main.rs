//! # Integration tests
//!
//! Integration tests are run in a clean folder (`crate/tmp`) alongside the API test server.
//!
//! ## Running integration tests
//!
//! Integration tests are run among other tests with `cargo test`.
//!
//! Filter to only integration tests:
//! ```
//! cargo test integration
//! ```
//!
//! Filter to a specific test:
//! ```
//! cargo test help_works
//! ```
//!
//! In order to show output from the test server:
//! ```
//! cargo test integration -- --nocapture
//! ```
//!
//! If you wish to **not** run integration tests at all (only unit tests), do:
//! ```
//! cargo test --bins
//! ```

mod common;
mod help;

use common::TESTS;

use std::net;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};

struct TestServer {
    child: Arc<Mutex<Child>>,
}

impl TestServer {
    fn start(capture: bool) -> Self {
        let mut child = Command::new("poetry")
            .current_dir(["api", "mock_server"].iter().collect::<PathBuf>())
            .args(&["run", "python", "app.py"])
            .stdin(Stdio::null())
            .stdout(if capture {
                Stdio::null()
            } else {
                Stdio::inherit()
            })
            .stderr(if capture {
                Stdio::null()
            } else {
                Stdio::inherit()
            })
            .spawn()
            .unwrap();
        for _try in 0..5 {
            let res = net::TcpStream::connect_timeout(
                &"127.0.0.1:4010".parse().unwrap(),
                std::time::Duration::from_millis(1000),
            );
            match res {
                Ok(_stream) => {
                    let child = Arc::new(Mutex::new(child));
                    let child_copy = Arc::clone(&child);
                    ctrlc::set_handler(move || {
                        let _ = child_copy.lock().unwrap().kill();
                    })
                    .unwrap();
                    return Self { child };
                }
                Err(_) => {
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                }
            }
        }
        let _ = child.kill();
        panic!("Unable to connect to test server");
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        let _ = self.child.lock().unwrap().kill();
    }
}

fn main() {
    println!();
    let filter = std::env::args().nth(1);
    let capture = !std::env::args().any(|arg| arg == "--nocapture");
    println!("starting test server");
    let _test_server = TestServer::start(capture);
    let project_dir = std::env::current_dir().unwrap();
    let mut runtime_dir = project_dir.clone();
    runtime_dir.push("tmp");
    for test in TESTS {
        let mut name = "?".to_owned();
        backtrace::resolve(
            unsafe { (*test as *mut std::os::raw::c_void).offset(1) },
            |symbol| {
                name = symbol.name().unwrap().to_string();
            },
        );
        if let Some(filter) = &filter {
            if !name.contains(filter) {
                continue;
            }
        }
        println!("test {}", name);
        if runtime_dir.exists() {
            std::fs::remove_dir_all(&runtime_dir).unwrap();
        }
        std::fs::create_dir(&runtime_dir).unwrap();
        std::env::set_current_dir(&runtime_dir).unwrap();
        test();
        std::env::set_current_dir(&project_dir).unwrap();
    }
    println!("success");
    println!();
}

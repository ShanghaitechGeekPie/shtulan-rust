extern crate hyper;
extern crate hyper_native_tls;
extern crate url;
extern crate rustc_serialize;
mod http;
use http::Auth;
use std::time::Duration;
use std::thread;

fn main() {
    let auth = Auth::new("chenww", "123456");
    loop {
        auth.login();
        thread::sleep(Duration::from_secs(10));
    }
}

extern crate nix;

use nix::unistd::getpid;
use std::env::set_var;
use std::net::TcpListener;
use std::os::unix::io::AsRawFd;
use std::os::unix::process::CommandExt;
use std::process::Command;

fn main() {
    println!("Hello, world!");
    // TODO: make the address configurable
    let listener = TcpListener::bind("0.0.0.0:0").unwrap();

    let addr = listener.local_addr().unwrap();

    // TODO: we are assuming that this will bind to fd=3 which might not be true
    let fd = listener.as_raw_fd();

    println!("addr={} fd={}", addr, fd);

    // TODO: actually select which command is going to be running
    let status = Command::new("sh")
        .arg("-c")
        .arg("echo world")
        .before_exec(|| {
            set_var("LISTEN_FDS", "1");
            set_var("LISTEN_PID", format!("{}", getpid()));
            Ok(())
        })
        .status().unwrap();
}

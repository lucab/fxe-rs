#[macro_use]
extern crate error_chain;
extern crate nix;

use std::{env, ffi, fs};
use std::os::unix::io::IntoRawFd;
use nix::{sched, unistd};

error_chain!{
    foreign_links {
        Ffi(std::ffi::NulError);
        Io(std::io::Error);
        Nix(nix::Error);
    }
}
quick_main!(run);

fn run() -> Result<()> {
    // Get a FD to the `busybox` binary.
    let bb_path = "/bin/busybox";
    let exe = fs::File::open(&bb_path)
        .chain_err(|| format!("failed to open {}", bb_path))?
        .into_raw_fd();

    // Get a FD to the target mount-namespace.
    let ns_path = env::args().nth(1).ok_or("missing mount-ns path")?;
    let ns = fs::File::open(&ns_path)
        .chain_err(|| format!("failed to open {}", ns_path))?
        .into_raw_fd();

    // Move to the target mount-namespace.
    sched::setns(ns, sched::CLONE_NEWNS)
        .chain_err(|| "setns failed")?;
    unistd::close(ns)
        .chain_err(|| "closing descriptor failed")?;

    // Execute `modinfo` in the target.
    let args = vec![
        ffi::CString::new("modinfo")?,
        ffi::CString::new("crc16")?,
    ];
    let env = vec![];
    unistd::fexecve(exe, args.as_slice(), env.as_slice())
        .chain_err(|| "fexecve failed")?;

    Ok(())
}

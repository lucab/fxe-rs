# fxe

> Binary execution across Linux mount-namespaces

`fxe` is a small, pure-Rust Linux program which demonstrates how to execute binaries across mount-namespaces.

This technique is suitable for several usecases, as it allows to ship minimal containers with specialized binaries and then to run them in namespaces where they are not available.
For example, a bare-minimal [ContainerLinux][CL] OS can augmented with a `mount-foo` container to mount `foo` volumes directly on the host. 

This program is provided for illustrative purpose only, it is not supposed to be run as-is in production.

## How this works

As the name suggests, `fxe` core functionality is built around [`fexecve(3)`][fexecve]. Short description from its [manpage][man] says:

```
fexecve() performs the same task as execve(), with the difference that the file to be executed is specified
via a file descriptor rather than via a pathname.
```

This allows `fxe` to get an handle to a binary available inside its container (i.e. mount-namespace), move to a different target, and execute the binary there.

## Demo

This repository contains a demo program which runs a `modinfo crc16` using the `busybox` container.
However, the directory containing kernel modules is not available inside the container; instead the process changes its mount-namespace to the target one (e.g. host) and runs the `modinfo` binary there.

A pre-built binary is available as a Docker image at `quay.io/lucab/fxe`.
To try it, simply do a `make run`:

```
$ make run

docker run --privileged --pid=host -v /proc/1/ns/:/ns quay.io/lucab/fxe:latest

filename:       /lib/modules/4.11.0-1-amd64/kernel/lib/crc16.ko
description:    CRC16 calculations
license:        GPL
depends:        
intree:         Y
vermagic:       4.11.0-1-amd64 SMP mod_unload modversions 
```

This will use `/proc/1/ns/mnt` as the host mount-namespace target, which should be bind-mounted inside the container.

The `--privileged` flag is a shortcut to add `CAP_SYS_ADMIN` and `CAP_SYS_CHROOT` (required by `setns(2)`) and to prevent the default SECCOMP filter to block it. Both can be allowed with finer granularity settings (this is left as an exercise).

The `--pid=host` flag is required for proper `fexecve()` execution. It can be changed to any arbitrary target, here it is set to `host` only for demonstration purpose.

## Caveats

Due to how `setns(2)` and `fexecve(3)` are implemented on Linux, there are some conditions imposed on the running environment:
 1. setns: `CAP_SYS_ADMIN` and `CAP_SYS_CHROOT` are required
 1. setns: the target mount-namespace must be available as a file descriptor
 1. setns: to be allowed to change mount-namespace, the process must be single-thread
 1. fexecve: `/proc` must be available
 1. fexecve: source and target processes must be running in the same PID-namespace
 1. fexecve: scripts and dynamic binaries resources must be available in the target

See notes in both manpages for further details and explanations.

## Compilation

The demo in this repository can be quickly built via `make`.

Pre-requisites are:
 * `make` 
 * a stable rustc/cargo toolchain for the `x86_64-unknown-linux-musl` target (available via [rustup]) 
 * `docker run` available to the current user

This currently depends on a [pending PR to nix][nix-727].

[CL]: https://coreos.com/os/docs/latest
[fexecve]: http://pubs.opengroup.org/onlinepubs/9699919799/functions/fexecve.html
[man]: http://man7.org/linux/man-pages/man3/fexecve.3.html
[nix-727]: https://github.com/nix-rust/nix/pull/727
[rustup]: https://www.rustup.rs/ 

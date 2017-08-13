FROM busybox

MAINTAINER "Luca Bruno <lucab@debiab.org>"

COPY target/x86_64-unknown-linux-musl/release/fxe /

CMD ["/fxe", "/ns/mnt"]

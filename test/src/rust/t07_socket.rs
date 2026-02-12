#![allow(non_camel_case_types)]

type c_long = i64;
type c_int = i32;
type size_t = usize;

extern "C" {
    fn syscall(num: c_long, ...) -> c_long;
}

const SYS_SOCKETPAIR: c_long = 53;
const SYS_SENDTO: c_long = 44;
const SYS_RECVFROM: c_long = 45;
const SYS_SHUTDOWN: c_long = 48;
const SYS_CLOSE: c_long = 3;

const AF_UNIX: c_int = 1;
const SOCK_STREAM: c_int = 1;
const SHUT_RDWR: c_int = 2;

fn main() {
    unsafe {
        let mut fds = [0i32; 2];

        let _ = syscall(SYS_SOCKETPAIR, AF_UNIX, SOCK_STREAM, 0, fds.as_mut_ptr());

        let mut buf = [b'A'; 1];
        
        let _ = syscall(
            SYS_SENDTO,
            fds[0] as c_int,
            buf.as_ptr(),
            1 as size_t,
            0 as c_int,
            0 as *const core::ffi::c_void,
            0 as c_int,
        );

        let _ = syscall(
            SYS_RECVFROM,
            fds[1] as c_int,
            buf.as_mut_ptr(),
            1 as size_t,
            0 as c_int,
            0 as *mut core::ffi::c_void,
            0 as *mut c_int,
        );

        let _ = syscall(SYS_SHUTDOWN, fds[0] as c_int, SHUT_RDWR as c_int);
        
        let _ = syscall(SYS_CLOSE, fds[0] as c_int);
        let _ = syscall(SYS_CLOSE, fds[1] as c_int);
    }
}

#![allow(non_camel_case_types)]

type c_long = i64;

extern "C" {
    fn syscall(num: c_long, ...) -> c_long;
}

const SYS_GETPID: c_long = 39;
const SYS_GETPPID: c_long = 110;
const SYS_GETTID: c_long = 186;
const SYS_UNAME: c_long = 63;
const SYS_SCHED_YIELD: c_long = 24;

const SYS_GETUID: c_long = 102;
const SYS_GETEUID: c_long = 107;
const SYS_GETGID: c_long = 104;
const SYS_GETEGID: c_long = 108;
const SYS_UMASK: c_long = 95;

fn exists_only() {
    unsafe {
        let _ = syscall(SYS_GETUID);
        let _ = syscall(SYS_GETEUID);
        let _ = syscall(SYS_GETGID);
        let _ = syscall(SYS_GETEGID);
        let _ = syscall(SYS_UMASK, 0o22);
    }
}

#[used]
static SINK: [fn(); 1] = [exists_only];

fn main() {
    unsafe {
        let _ = syscall(SYS_GETPID);
        let _ = syscall(SYS_GETPPID);
        let _ = syscall(SYS_GETTID);

        #[repr(C)]
        struct utsname {
            sysname: [u8; 65],
            nodename: [u8; 65],
            release: [u8; 65],
            version: [u8; 65],
            machine: [u8; 65],
            domainname: [u8; 65],
        }
        let mut u = utsname {
            sysname: [0; 65],
            nodename: [0; 65],
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65],
            domainname: [0; 65],
        };
        let _ = syscall(SYS_UNAME, &mut u as *mut _);

        let _ = syscall(SYS_SCHED_YIELD);
    }

    if std::env::var("RUN_EXISTS_ONLY").ok().as_deref() == Some("1") {
        (SINK[0])();
    }
}

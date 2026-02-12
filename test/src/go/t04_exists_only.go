package main

import (
	"os"
	"runtime"
	"syscall"
	"time"
	"unsafe"
)

func existsOnly() {
	_, _, _ = syscall.RawSyscall(syscall.SYS_GETUID, 0, 0, 0)
	_, _, _ = syscall.RawSyscall(syscall.SYS_GETEUID, 0, 0, 0)
	_, _, _ = syscall.RawSyscall(syscall.SYS_GETGID, 0, 0, 0)
	_, _, _ = syscall.RawSyscall(syscall.SYS_GETEGID, 0, 0, 0)
	_, _, _ = syscall.RawSyscall(syscall.SYS_UMASK, 022, 0, 0)
}

var sink = []func(){existsOnly}

func main() {

	_ = os.Getpid()
	_ = os.Getppid()
	var u syscall.Utsname
	_ = syscall.Uname(&u)
	time.Sleep(13 * time.Millisecond)
	runtime.Gosched()



	if os.Getenv("RUN_EXISTS_ONLY") == "1" {

		sink[0]()
	}

	_ = unsafe.Pointer(nil)
}

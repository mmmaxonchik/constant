package main

import (
	"syscall"
	"unsafe"
)

func main() {
	var fds [2]int32

	_, _, _ = syscall.RawSyscall6(
		syscall.SYS_SOCKETPAIR,
		1, 1, 0,
		uintptr(unsafe.Pointer(&fds[0])),
		0, 0,
	)
	buf := []byte{'A'}
	_, _, _ = syscall.RawSyscall6(
		syscall.SYS_SENDTO,
		uintptr(fds[0]),
		uintptr(unsafe.Pointer(&buf[0])),
		1,
		0, 0, 0,
	)

	_, _, _ = syscall.RawSyscall6(
		syscall.SYS_RECVFROM,
		uintptr(fds[1]),
		uintptr(unsafe.Pointer(&buf[0])),
		1,
		0, 0, 0,
	)

	_, _, _ = syscall.RawSyscall(syscall.SYS_SHUTDOWN, uintptr(fds[0]), 2, 0)

	_, _, _ = syscall.RawSyscall(syscall.SYS_CLOSE, uintptr(fds[0]), 0, 0)
	_, _, _ = syscall.RawSyscall(syscall.SYS_CLOSE, uintptr(fds[1]), 0, 0)
}

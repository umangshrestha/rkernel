install:
	cargo bootimage --target x86_64.json  -Z build-std=core -Z build-std-features=compiler-builtins-mem
	qemu-system-x86_64 -drive format=raw,file=target/x86_64/debug/bootimage-rkernel.bin 

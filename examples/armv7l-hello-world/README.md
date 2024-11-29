# Inspect binary

```console

objdump -D -S target/x86_64-unknown-none/debug/deos-example-x86
```

# Putting ELF to

```console
cd /desk
bin/putapp -targetIPAddr=127.0.0.1 ~/documents/projects/deos-rust/deos-example-x86/target/x86_64-unknown-none/release/deos-example-x86
bin/link_deos /usr/local/cross-compilers/gcc-11.2.0/ x86_64 exe ~/documents/projects/deos-rust/deos-example-x86/target/x86_64-unknown-none/release/deos-example-x86
```

# Linking

```
/desk/bin/link_deos /usr/local/cross-compilers/gcc-11.2.0 arm exe  -rpath-link /desk/arm/appbin -nostdlib -Map ./hello-world.map  -Ttext-segment=0x04000000  -o hello-world.exe  ~/documents/projects/deos-rust/deos-example-armv7l/target/armv7a-none-eabihf/release/libdeos_example_x86.a -L/desk/arm/appbin -l kernel -l video -l vfile -l ansi --show-command
```

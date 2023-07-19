#! /bin/sh
set -e

#rm -f librust_app.a
#cp ../../../../rust-app/target/thumbv7m-none-eabi/debug/librust_app.a .
#cp /home/shiro/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/thumbv7m-none-eabi/lib/libcore-*.rlib librust_core.a
#cp /home/shiro/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/thumbv7m-none-eabi/lib/liballoc-*.rlib librust_alloc.a
#cp /home/shiro/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/thumbv7m-none-eabi/lib/librustc_std_workspace_core-*.rlib librustc_std_workspace_core.a

#cp /home/shiro/.rustup/toolchains/stage1/lib/rustlib/thumbv7m-none-eabi/lib/libcore-*.rlib librust_core.a
#cp /home/shiro/.rustup/toolchains/stage1/lib/rustlib/thumbv7m-none-eabi/lib/liballoc-*.rlib librust_alloc.a
#cp /home/shiro/.rustup/toolchains/stage1/lib/rustlib/thumbv7m-none-eabi/lib/librustc_std_workspace_core-*.rlib librustc_std_workspace_core.a


#ar d librust_app.a $(ar t librust_app.a | grep compiler_builtins)
make
qemu-system-arm -machine mps2-an385 -cpu cortex-m3 -kernel output/RTOSDemo.out -monitor none -nographic -serial stdio -s

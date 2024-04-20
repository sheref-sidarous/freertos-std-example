#! /bin/bash
TOOLCHAIN_NAME="local-1.76-unwind"
set -e

if [[ $(rustup toolchain list | grep $TOOLCHAIN_NAME) ]]
then
echo "ERROR: There is already a toolchain linked under the name $TOOLCHAIN_NAME"
echo "Uninstall it via \"rustup uninstall $TOOLCHAIN_NAME\" and retry"
exit 1
fi

# Clone and patch the Rust repo
if [[ -e rust ]]
then
cd rust
if [[ $(git describe)=1.76.0 && -z $(git diff | diff --ignore-all-space ../setup-unwind-for-thumbv7m-target.patch -) ]]
then
echo "Rust repo is already cloned and patched, skipping this step"
else
echo "Unexpected contents in rust subfolder, delete it and retry"
exit 1
fi
else
echo "Cloning and Patching Rust repo"
git clone --depth=1 --branch=1.76.0 https://github.com/rust-lang/rust.git
cd rust
git apply ../setup-unwind-for-thumbv7m-target.patch
fi

# Build the toolchain
echo "Building the toolchain"
./x build --config ../config.toml --target thumbv7m-none-eabi

# link it
rustup toolchain link $TOOLCHAIN_NAME build/host/stage1
echo "The built toolchain is linked as $TOOLCHAIN_NAME, you can now use it for building"
echo "example: cargo +$TOOLCHAIN_NAME build"

#! /bin/sh
set -e

if [ $1 = "--debug" ]
then
DEBUG_MSG="Will halt and wait for gdb attach on port 1234"
DEBUG_ARG="-S"
else
DEBUG_MSG="If you want to use gdb, call this script with --debug"
DEBUG_ARG=""
fi

echo "Buidling the image ..."
make

echo "Running using Qemu. $DEBUG_MSG"
qemu-system-arm -machine mps2-an385 \
                -cpu cortex-m3 \
                -kernel output/RTOSDemo.out \
                -monitor none \
                -nographic \
                -serial stdio \
                -s \
                --semihosting-config enable=on,target=native \
                $DEBUG_ARG

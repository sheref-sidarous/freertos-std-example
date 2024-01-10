import subprocess

def build_and_run(test_function):
    subprocess.run("RUST_APP_ROOT=../tests/rust make", cwd="../FreeRTOS-Qemu-Demo", shell=True)
    #subprocess.run("make", cwd="../FreeRTOS-Qemu-Demo", shell=True)
    subprocess.run("qemu-system-arm -machine mps2-an385 "
                "-cpu cortex-m3 "
                "-kernel output/RTOSDemo.out "
                "-monitor none "
                "-nographic "
                "-serial stdio "
                "-s "
                "--semihosting-config enable=on,target=native ", cwd="../FreeRTOS-Qemu-Demo", shell=True)


def test_addition():
    assert 1 + 2 == 3


build_and_run("")
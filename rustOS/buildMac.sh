echo "What do you want to do:"
echo "1. Build"
echo "2. Run target on QEMU"

read value
echo $value
if [[ $value -eq 1 ]]; then
  cargo rustc -- -C link-args="-e __start -static -nostartfiles"
fi
if [[ $value -eq 2 ]]; then
  # qemu-system-x86_64 -drive format=raw,file=target/x86-64_rustOS/debug/bootimage-rust_os.bin
  cargo run
fi

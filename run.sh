cd hello
cbindgen . -o hello.h --lang c
cargo build
cp target/debug/libhello.so ../lib/
cd ..
go build -ldflags="-r lib" main.go
./main


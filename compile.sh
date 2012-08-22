rm *.so
rustc --lib opus.rs
rustc opus_test.rs -L .

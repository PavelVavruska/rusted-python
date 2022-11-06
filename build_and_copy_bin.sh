cd pyext-rustlib/
cargo build --release
cd ..
cp pyext-rustlib/target/release/librustlib.so rustlib.so
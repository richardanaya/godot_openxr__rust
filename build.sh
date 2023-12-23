echo "Building Rust OpenXR"
cd rust_openxr  && cargo build
cd ..
pwd
rm ./lib/rust_openxr.dll
cp ./rust_openxr/target/debug/rust_openxr.dll ./lib/rust_openxr.dll
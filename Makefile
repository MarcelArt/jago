build:
	@cargo build --release

build-win:
	@cargo build --release --target x86_64-pc-windows-gnu

build-web:
	@cargo +nightly build -Zbuild-std --target wasm32-unknown-emscripten
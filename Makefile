build:
	@cargo build --release

build-win:
	@cargo build --release --target x86_64-pc-windows-gnu

build-android:
	@export CLANG_PATH=/home/marcel/Android/Sdk/ndk/28.1.13356709/toolchains/llvm/prebuilt/linux-x86_64/bin/clang && \
	echo "Using clang at $$CLANG_PATH" && \
	export CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER=/home/marcel/Android/Sdk/ndk/28.1.13356709/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android35-clang && \
	export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=/home/marcel/Android/Sdk/ndk/28.1.13356709/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android35-clang && \
	echo "Using linker at $$CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER and $$CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER" && \
	cargo build --target=aarch64-linux-android --release && \
	cargo build --target=x86_64-linux-android --release

keystore-gen:
	@keytool -genkey -v -keystore godot/.keystore/cafe-on-wheel.keystore -keyalg RSA -keysize 2048 -validity 10000 -alias cafe-on-wheel
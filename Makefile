install:
	cargo build --release
	install -Dm755 target/release/iin-generator /usr/local/bin/iin-generator


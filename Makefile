all: install

install:
	cargo build --release
	sudo cp target/release/cow-translator /usr/bin/cow-translator

uninstall:
	sudo rm -f /usr/bin/cow-translator
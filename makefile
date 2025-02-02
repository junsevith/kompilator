all:
	cargo build --release

req:
	sudo apt install rustup
	rustup default stable
install-rust-menu:
	cargo build --release
	cp target/release/rust-menu ~/.local/bin/

install-menu-sh:
	cp menu.sh ~/.local/bin/omni-menu

install:
	make install-rust-menu
	make install-menu-sh


all: src/lib.rs src/ic.rs
	cargo build --release

test: src/lib.rs src/ic.rs
	cargo test --release

src/lib.rs: make.py make/bitpack.py make/vint.py make/vp4.py make/vsimple.py
	python3 make.py lib > $@

src/ic.rs: make.py make/bitpack.py make/vint.py make/vp4.py make/vsimple.py
	python3 make.py ic > $@

clean:
	rm -f src/ic.rs src/lib.rs

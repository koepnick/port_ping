all: release

release:
	cargo build --release

debug:
	cargo build

clean:
	rm -rf target/

R=0

ifeq ($(R), 1)
    FLAGS=--release
endif

build:
	cargo fmt
	cargo build $(FLAGS)

clean:
	cargo clean

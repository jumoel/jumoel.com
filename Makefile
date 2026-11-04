INPUTS := Cargo.toml Cargo.lock $(shell find src _posts _templates images -type f -o -type d)

.PHONY: serve site build

site: _build/.stamp

_build/.stamp: $(INPUTS)
	cargo run
	cp -r images _build/images
	touch $@

serve:
	cargo run --bin serve -- ./_build --index index.html --port 3000

build:
	cargo build
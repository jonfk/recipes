default: update-contents

.PHONY: clean update-contents build

RUST_FILES = $(shell find . -path 'target' -prune -o -type f -name '*.rs')

build: recipe_index/target/release/recipe_index

recipe_index/target/release/recipe_index: $(RUST_FILES)
	cd recipe_index && cargo build --release

clean:
	cd recipe_index && cargo clean

update-contents:
	./index-recipes > contents.md

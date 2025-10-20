set shell := ["bash", "-lc"]

alias default := update-contents

update-contents:
    ./index-recipes > contents.md

build:
    cd recipe_index && cargo build --release

clean:
    cd recipe_index && cargo clean

fmt:
    pnpm run fmt
    cd recipe_index && cargo fmt

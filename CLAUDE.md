# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

This is a personal recipe collection stored in YAML format, organized by category (baking, meals, desserts, drinks, salads). The repository includes a Rust-based indexing tool (`recipe_index`) that generates a table of contents and JSON index from the recipe files.

## Common Commands

### Building and Running the Indexer

```bash
# Build the recipe indexer
cd recipe_index && cargo build --release

# Or use the Makefile target
make build
```

### Updating the Table of Contents

```bash
# Generate/update contents.md and recipes.json from all recipe YAML files
make update-contents

# Or run directly (requires building first)
./index-recipes > contents.md
```

The `update-contents` target is the default make target, so you can also just run `make`.

### Cleaning Build Artifacts

```bash
make clean
```

## Recipe Structure

Recipes are stored as YAML files with the following structure (see `template.yaml`):

```yaml
name: Recipe Name
description: Optional description
source: Author or source name (can be string or list)
links:
  - URL references
ingredients:
  - name: ingredient name
    quantity: amount
    optional: true/false (optional field)
timings:
  - time: duration
    for: what the time is for
notes:
  - Notes and tips
instructions:
  - Step by step instructions
times_made:
  count: number
  dates:
    - YYYY-MM-DD
```

### Recipe Organization

- `baking/` - Breads, pizza doughs, pastries, cookies, cakes
- `meals/` - Main dishes organized by cuisine/protein (beef, chicken, pasta, korean, japanese, etc.)
- `desserts/` - Sweet treats and desserts
- `drinks/` - Beverage recipes
- `salads/` - Salad recipes
- `ramen/` - Ramen-specific recipes
- `unsuccessful/` - Recipes that didn't work out
- `untested/` - Recipes not yet tested

## Recipe Indexer Architecture

The `recipe_index` Rust tool (`recipe_index/`) builds a directory tree structure and generates outputs:

### Key Modules

- `lib.rs` - Main entry point that walks directories and builds the recipe tree
- `model.rs` - Defines `IdTree`, `Dir`, and `Entry` structures representing the recipe hierarchy
- `serialization.rs` - YAML parsing using serde, defines `Recipe`, `Ingredient`, `Timing`, etc.
- `output.rs` - Generates markdown table of contents and JSON index
- `git.rs` - Extracts git metadata (creation/modification dates) for recipes
- `filters.rs` - File filtering utilities (YAML detection, hidden file exclusion)

### How It Works

1. Walks specified directories (baking, desserts, meals, drinks, salads, unsuccessful)
2. For each YAML file, parses the recipe structure and extracts git metadata
3. Builds a tree structure (`IdTree`) mirroring the directory hierarchy
4. Generates two outputs:
   - `contents.md` - Hierarchical markdown table of contents with links to recipes
   - `recipes.json` - Flat JSON array of all recipes with metadata

The indexer creates entries in `contents.md` with the recipe name (title-cased) and optional description. Git timestamps are stored in `recipes.json` but not displayed in the markdown output.

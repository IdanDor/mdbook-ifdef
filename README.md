# Mdbook ifdef

This package is for clearing specific sections/chapters from an mdbook according to "compilation flags".
It amounts to ifdef like behavior with the added feature of a file-wide ifdef for removing a chapter and its sub chapters.

You should also consider using [mdbook-private](https://github.com/RealAtix/mdbook-private) instead.

## Installation

- [Install `mdbook`](https://rust-lang.github.io/mdBook/guide/installation.html)
- Install the latest version of this repository `cargo install --git https://github.com/IdanDor/mdbook-ifdef.git mdbook-ifdef`

## Usage

Add the `ifdef` preprocessor to your `book.toml` file.

```diff
[book]
title = "Multiple Consumer Book"

+ [preprocessor.ifdef]
+ command = "mdbook-ifdef -f flags.txt"
```

Create the file `flags.txt` with your chosen build flags.

Now running `mdbook build` will strip out sections and files depending on your given flags.

### Commandline arguments

The supported commandline arguments can be seen using `--help` on the binary.
The following are useful optional flags:

- `-f, --flags-file <FLAGS_FILE>` for setting the flags file.
- `-e, --extra-flags <EXTRA_FLAGS>` for giving extra flags, argument can be repeated and supports `,` delimeter for easier usage (`-e a,b,c` is the same as `-e a,b -e c`).

### Flags file

Currently the format is a straightforward textual format of flags seperated by `,`.

### As cli

The binary can be run manually as a cli for debugging purposes which prints the output after censoring.
To manually see the expected result of a file `a.md` one can run:

```bash
mdbook-ifdef -f flags.txt -e extra_flag manual a.md
```

## Features

The package supports two ifdef patterns:

- A filewide ifdef `@file_abc` which will remove the entire file - and the reference from `SUMMARY.md` - from the output if the flag `abc` is not set.
  - Links to the file are broken, but remain. You can use the second pattern to help in such a thing.
- A `@if_abc abc @elif_dfe dfe @else else @end` pattern for selective text.
  - The `elif` section is optional, and can also be repeated as many times as necessary.
  - Multiline text between each section is supported.
  - Both patterns can be nested within this and will only be evaulted if the given branch is selected.

Patterns within `backticks` and code sections are ignored.

## Examples

See the `examples` subdirectory for some example books.

# Supported features demonstration

## a.md

Will always be shown.

## b.md

A subchapter of `a.md` which will only be shown if `flag_b` is set - else it will be removed.

## c.md

A subchapter of `a.md` which includes a branching section depending on which of the flags `flag1`/`flag2`/`flag3` are set.
The `@file_flag5` - which removes `c.md` and its sub chapter if `flag5` is not given - is only relevant if one of the branches leading to it are set.
Specifically it happens only if `flag2` is set or none of the flags `flag1`/`flag2`/`flag3` are set.

## d.md

A subchapter of `c.md`, which will be removed if `c.md` is removed, also demonstrates nested `if/elif/else` patterns depending on `flag1` and `flag2`.

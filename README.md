# Mdbook ifdef

This package is for clearing specific sections/chapters from an mdbook according to "compilation flags".
It amounts to ifdef like behavior with the added feature of a file-wide ifdef for removing a chapter and its sub chapters.

## Example usage

We can have the following files:
SUMMARY

```markdown
# some header
* [main](./a.md)
 * [subchapter](./b.md)
 * [subchapter2](./c.md)
  * [subsubchapter](./d.md)
```

a.md

```markdown
This is some regular markdown, note that we can use `@if @else @elif @end @file` without problems because it is in backticks.
Similarily, they are ignored if they are in a code section.
```

b.md

```markdown
This chapter will only appear if @file_flag_b `flag_b` is set, else it will be removed and all its subchapters.
```

c.md

```markdown
This will have the relevant text depending if flag1/flag2/flag3 are given.
The `@file_<flag>` is only relevant if it is within the chosen if/elif/else path. 

@if_flag1
text1
@elif_flag2
text2 @file_flag5
@elif_flag3
text3
@else
text4 @file_flag5
@end
```

d.md

```markdown
@if_flag1
Here we can see that ifs work within other ifs
@if_flag2
This will only appear if both `flag1` and `flag2` are set.
@end
@else
This the the other optional text.
@end
```

## Safety features

After processing all the input, if any `@if` or other markings are still somewhere, a panic will occur to prevent accidental information leakage.

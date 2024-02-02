# c.md file

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

use core::panic;
use std::collections::HashSet;

use pest_derive::Parser;
use pest::{Parser, iterators::Pair};


#[derive(Parser)]
#[grammar = "fake_md.pest"]
pub struct FakeMarkdownParser<'a> {
    contents: String,
    ctx: &'a HashSet<String>
}

impl<'a> FakeMarkdownParser<'a> {
    const fn new(ctx: &'a HashSet<String>) -> FakeMarkdownParser {
        Self{contents: String::new(), ctx}
    }

    fn check_file_flag(&mut self, node: Pair<'_, Rule>) -> Option<()> {
        // file_flag only has identifier and unwanted whitespace.
        let iden = node.into_inner().next().unwrap().as_str();

        if self.ctx.contains(iden) {
            Some(())
        } else {  // Flag not in ctx, clear the file!
            None
        }
    }

    fn recursive_flag_statement_parser(&mut self, node: Pair<'_, Rule>) -> Option<()>  {
        let mut add_next = false;

        for inner_pair in node.into_inner() {
            let rule = inner_pair.as_rule();
            match rule {
                Rule::flag_if | Rule::flag_elif => {
                    // file_flag only has identifier and unwanted whitespace.
                    let iden = inner_pair.into_inner().next().unwrap().as_str();
                    if self.ctx.contains(iden) {
                        add_next = true;
                    }
                },
                Rule::flag_else => {add_next = true},
                Rule::flag_end => {},
                Rule::markdown => {
                    if add_next {
                        self.recursive_markdown_parser(inner_pair)?;
                        return Some(());
                    }
                },
                _ => {panic!("Unexpected type {rule:?} inside flage_statement")}
            };
        }

        Some(())
    }

    fn recursive_markdown_parser(&mut self, node: Pair<'_, Rule>) -> Option<()>  {
        for inner_pair in node.into_inner() {
            let rule = inner_pair.as_rule();
            match rule {
                Rule::flag_statement => {
                    self.recursive_flag_statement_parser(inner_pair)?;
                },
                Rule::flag_file => {
                    self.check_file_flag(inner_pair)?;
                },
                Rule::code_section | Rule::code_snippet | Rule::text => {
                    self.contents.push_str(inner_pair.as_str());
                },
                _ => {panic!("Unexpected type {rule:?} inside markdown")}
            };
        }

        Some(())
    }

    #[must_use]
    pub fn fake_markdown_parse_and_clean(string: &str, ctx: &'a HashSet<String>) -> Option<String> {
        let file_node = Self::parse(Rule::markdown_file, string)
            .map_err(|e| {eprintln!("Error when processing file: {e:?}"); e}).ok()?
            .next()?
            .into_inner().next()?;

        let mut ctx = Self::new(ctx);
        ctx.recursive_markdown_parser(file_node)?;

        Some(ctx.contents)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::grammer::FakeMarkdownParser;

    fn default_ctx() -> HashSet<String> {
        vec!["abc".to_owned(), "bbb".to_owned()].into_iter().collect::<HashSet<_>>()
    }

    #[test]
    fn check_iden_usage() {
        let input = r#"@if_a 111 @elif_b 222 @else 333 @file_c @end"#;
        let empty_ctx = HashSet::default();
        assert_eq!(FakeMarkdownParser::fake_markdown_parse_and_clean(input, &empty_ctx), None);
        assert_eq!(FakeMarkdownParser::fake_markdown_parse_and_clean(input, &vec!["a".to_owned()].into_iter().collect::<HashSet<_>>()), Some("111 ".to_owned()));
        assert_eq!(FakeMarkdownParser::fake_markdown_parse_and_clean(input, &vec!["b".to_owned()].into_iter().collect::<HashSet<_>>()), Some("222 ".to_owned()));
        assert_eq!(FakeMarkdownParser::fake_markdown_parse_and_clean(input, &vec!["c".to_owned()].into_iter().collect::<HashSet<_>>()), Some("333 ".to_owned()));
    }


    #[test]
    fn backticks_escape_works() {
        let input = r#"
            I can write lots of stuff.
            Special Sym@bols, and even more special symbols inside code section.
            ```
            @if_abc
            option1
            @elif_aaa
            option2
            @else
            option3 and @file_aaaaa
            @end
            ```
            And also the same in code snippets `@file_you_dont_have_this` and `@if_aaaa ss @else ssf @end`.
        "#;
        assert_eq!(FakeMarkdownParser::fake_markdown_parse_and_clean(input, &default_ctx()), Some(input.to_owned()));
    }

    #[test]
    fn file_flag_clear() {
        let input = r#"
            Some text!
            @file_no_way_you_have_this
        "#;
        assert_eq!(FakeMarkdownParser::fake_markdown_parse_and_clean(input, &default_ctx()), None);
    }

    #[test]
    fn file_flag_doesnt_clear() {
        let input = r#"
            Some text!
            @file_abc
        "#;
        let exp = r#"
            Some text!
            "#;
        assert_eq!(FakeMarkdownParser::fake_markdown_parse_and_clean(input, &default_ctx()), Some(exp.to_owned()));
    }

    #[test]
    fn test_if() {
        let input = r#"
            @if_bbb
            aaa
            @end
        "#;
        let exp = r#"aaa
            "#;
        assert_eq!(FakeMarkdownParser::fake_markdown_parse_and_clean(input.trim(), &default_ctx()), Some(exp.to_owned()));
    }

    #[test]
    fn test_elff() {
        let input = r#"
            @if_not_here
            111
            @elif_bbb
            aaa
            @else
            222
            @end
        "#;
        let exp = r#"aaa
            "#;
        assert_eq!(FakeMarkdownParser::fake_markdown_parse_and_clean(input.trim(), &default_ctx()), Some(exp.to_owned()));
    }

    #[test]
    fn test_else() {
        let input = r#"
            @if_not_here
            111
            @elif_not_here
            222
            @else
            aaa
            @end
        "#;
        let exp = r#"aaa
            "#;
        assert_eq!(FakeMarkdownParser::fake_markdown_parse_and_clean(input.trim(), &default_ctx()), Some(exp.to_owned()));
    }

    #[test]
    fn test_flag_only_in_selected() {
        let input = r#"
            @if_not_here
            111
            @elif_not_here
            222
            @file_not_here
            @else
            aaa
            @end
        "#;
        let exp = r#"aaa
            "#;
        assert_eq!(FakeMarkdownParser::fake_markdown_parse_and_clean(input.trim(), &default_ctx()), Some(exp.to_owned()));
        let input = r#"
            @if_not_here
            111
            @elif_not_here
            222
            @else
            aaa
            @file_not_here
            @end
        "#;
        assert_eq!(FakeMarkdownParser::fake_markdown_parse_and_clean(input.trim(), &default_ctx()), None);
    }

    #[test]
    fn test_missing_else_not_parsed() {
        let input = r#"
            @if_not_here
            111
            @elif_not_here
            222
            @file_not_here
            @else
        "#;
        assert_eq!(FakeMarkdownParser::fake_markdown_parse_and_clean(input.trim(), &default_ctx()), None);
    }

    #[test]
    fn test_missing_if_not_parsed() {
        let input = r#"
            @elif_not_here
            222
            @file_not_here
            @else
            aaa
            @end
        "#;
        assert_eq!(FakeMarkdownParser::fake_markdown_parse_and_clean(input.trim(), &default_ctx()), None);
    }

    #[test]
    fn test_case_insensitive_if() {
        let input = r#"
            @IF_not_here
            @eLiF_not_here
            222
            @fiLL_not_here
            @elSE
            aaa
            @End
        "#;
        let exp = r#"aaa
            "#;
        assert_eq!(FakeMarkdownParser::fake_markdown_parse_and_clean(input.trim(), &default_ctx()), Some(exp.to_owned()));
    }
}

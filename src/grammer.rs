use core::panic;

use regex::Regex;
use once_cell::sync::OnceCell;

use pest_derive::Parser;
use pest::{Parser, iterators::Pair};


fn static_regex() -> &'static Regex {
    static FLAG_REGEX: OnceCell<Regex> = OnceCell::new();
    FLAG_REGEX.get_or_init(|| {
        Regex::new(r"@(if|elif|else|end|file)").unwrap()
    })
}


#[derive(Parser)]
#[grammar = "fake_md.pest"]
pub struct FakeMarkdownParser {
    contents: String
}

impl FakeMarkdownParser {
    fn new() -> FakeMarkdownParser {
        FakeMarkdownParser{contents: String::new()}
    }

    fn check_file_flag(&mut self, node: Pair<'_, Rule>) -> Option<()> {
        println!("flag {:?}", node.as_str());
        // file_flag only has identifier and unwanted whitespace.
        let iden = node.into_inner().next().unwrap().as_str();

        if iden == "abc" {
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
                Rule::flag_if => {
                    // file_flag only has identifier and unwanted whitespace.
                    let iden = inner_pair.into_inner().next().unwrap().as_str();
                    if iden == "bbb" {
                        add_next = true;
                    }
                },
                Rule::flag_elif => {
                    // file_flag only has identifier and unwanted whitespace.
                    let iden = inner_pair.into_inner().next().unwrap().as_str();
                    if iden == "bbb" {
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
                    self.recursive_flag_statement_parser(inner_pair)?
                },
                Rule::flag_file => {
                    self.check_file_flag(inner_pair)?
                },
                Rule::code_section | Rule::code_snippet | Rule::text => {
                    if rule == Rule::text {
                        // To prevent accidental misusage of the fake markdown tools we which to check if the text includes problematic stuff.
                        for m in static_regex().find_iter(inner_pair.as_str()) {
                            // We have found a match for probably accidental misusage of our api flags.
                            eprintln!("Found suspected misusage of preparser stuff {:?}, skipping this section. If this is intentional, use `around it`", m.as_str());
                            return None;
                        }
                    }
                    self.contents.push_str(inner_pair.as_str())
                },
                _ => {panic!("Unexpected type {rule:?} inside markdown")}
            };
        }

        Some(())
    }

    pub fn fake_markdown_parse_and_clean(string: &str) -> Option<String> {
        let file_node = Self::parse(Rule::markdown_file, string)
            .unwrap_or_else(|e| panic!("{:?}", e))
            .next().unwrap()
            .into_inner().next().unwrap();

        let mut ctx = Self::new();
        ctx.recursive_markdown_parser(file_node)?;

        Some(ctx.contents)
    }
}

#[cfg(test)]
mod tests {
    use crate::grammer::FakeMarkdownParser;

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
        assert_eq!(FakeMarkdownParser::fake_markdown_parse_and_clean(input), Some(input.to_owned()));
    }

    #[test]
    fn file_flag_clear() {
        let input = r#"
            Some text!
            @file_no_way_you_have_this
        "#;
        assert_eq!(FakeMarkdownParser::fake_markdown_parse_and_clean(input), None);
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
        assert_eq!(FakeMarkdownParser::fake_markdown_parse_and_clean(input), Some(exp.to_owned()));
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
        assert_eq!(FakeMarkdownParser::fake_markdown_parse_and_clean(input.trim()), Some(exp.to_owned()));
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
        assert_eq!(FakeMarkdownParser::fake_markdown_parse_and_clean(input.trim()), Some(exp.to_owned()));
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
        assert_eq!(FakeMarkdownParser::fake_markdown_parse_and_clean(input.trim()), Some(exp.to_owned()));
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
        assert_eq!(FakeMarkdownParser::fake_markdown_parse_and_clean(input.trim()), Some(exp.to_owned()));
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
        assert_eq!(FakeMarkdownParser::fake_markdown_parse_and_clean(input.trim()), None);
    }

    #[test]
    #[should_panic]
    fn test_misusage_protection_cut_if() {
        let input = r#"
            @if_not_here
            111
            @elif_not_here
            222
            @file_not_here
            @else
        "#;
        FakeMarkdownParser::fake_markdown_parse_and_clean(input.trim());
    }

    #[test]
    #[should_panic]
    fn test_misusage_protection_missing_if() {
        let input = r#"
            @elif_not_here
            222
            @file_not_here
            @else
            aaa
            @end
        "#;
        FakeMarkdownParser::fake_markdown_parse_and_clean(input.trim());
    }
}

use core::panic;

use pest_derive::Parser;
use pest::{Parser, iterators::Pair};


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
                Rule::flag_statement => {self.recursive_flag_statement_parser(inner_pair)?},
                Rule::flag_file => {self.check_file_flag(inner_pair)?},
                Rule::code_section | Rule::text => {self.contents.push_str(inner_pair.as_str())},
                _ => {panic!("Unexpected type {rule:?} inside markdown")}
            };
        }

        Some(())
    }

    pub fn test_fake_markdown_parser(string: &str) -> Option<String> {
        let file_node = FakeMarkdownParser::parse(Rule::markdown_file, string)
            .unwrap_or_else(|e| panic!("{:?}", e))
            .next().unwrap()
            .into_inner().next().unwrap();

        let mut ctx = FakeMarkdownParser::new();
        ctx.recursive_markdown_parser(file_node)?;

        Some(ctx.contents)
    }
}

#[cfg(test)]
mod tests {
    use crate::grammer::FakeMarkdownParser;

    #[test]
    fn code_section_works() {
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
        "#;
        assert_eq!(FakeMarkdownParser::test_fake_markdown_parser(input), Some(input.to_owned()));
    }

    #[test]
    fn file_flag_clear() {
        let input = r#"
            Some text!
            @file_no_way_you_have_this
        "#;
        assert_eq!(FakeMarkdownParser::test_fake_markdown_parser(input), None);
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
        assert_eq!(FakeMarkdownParser::test_fake_markdown_parser(input), Some(exp.to_owned()));
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
        assert_eq!(FakeMarkdownParser::test_fake_markdown_parser(input.trim()), Some(exp.to_owned()));
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
        assert_eq!(FakeMarkdownParser::test_fake_markdown_parser(input.trim()), Some(exp.to_owned()));
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
        assert_eq!(FakeMarkdownParser::test_fake_markdown_parser(input.trim()), Some(exp.to_owned()));
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
        assert_eq!(FakeMarkdownParser::test_fake_markdown_parser(input.trim()), Some(exp.to_owned()));
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
        assert_eq!(FakeMarkdownParser::test_fake_markdown_parser(input.trim()), None);
    }
}

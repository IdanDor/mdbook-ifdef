#![deny(
    warnings,
    rustdoc::all,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]
use std::collections::HashSet;

use mdbook::BookItem;
use mdbook::book::{Book, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{PreprocessorContext, Preprocessor};

use regex::Regex;

pub mod grammer;
use grammer::FakeMarkdownParser;

pub struct CensorProcessor {
    flags: HashSet<String>,
}


impl Default for CensorProcessor {
    fn default() -> Self {
        CensorProcessor{flags: HashSet::new()}
    }
}

impl CensorProcessor {
    pub fn new(flags: Vec<String>) -> Self {
        CensorProcessor{flags: HashSet::from_iter(flags.into_iter())}
    }

    fn process_chapter(&self, mut chapter: Chapter) -> Option<Chapter>{
        // @file_<flag> requires `flag` to be set for the chapter to stay.
        let re = Regex::new(r"@file_(\w*)").unwrap();

        for m in re.find_iter(&chapter.content) {
            if !self.flags.contains(m.as_str()) {
                // We are missing a flag, remove chapter.
                return None;
            }
        }

        // TODO: cleanup flags? -> move file flags to pest.

        // Handle if/else
        FakeMarkdownParser::fake_markdown_parse_and_clean("a b");

        // Handle sub items recursively.
        chapter.sub_items = chapter.sub_items.into_iter().map(
            |item| self.process_item(item)
        ).flatten().collect();

        // Return the chapter.
        Some(chapter)
    }

    fn process_item(&self, section: BookItem) -> Option<BookItem> {
        Some(match section {
            BookItem::Chapter(chapter) => BookItem::Chapter(self.process_chapter(chapter)?),
            other => other,
        })
    }
}

impl Preprocessor for CensorProcessor {
    fn name(&self) -> &str {
        "censor"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> { 
        book.sections = book.sections.into_iter().map(
            |item| self.process_item(item)
        ).flatten().collect();

        Ok(book)
    }
}
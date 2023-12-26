#![deny(
    warnings,
    rustdoc::all,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]
use std::collections::HashSet;

use mdbook::book::{Book, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::BookItem;

pub mod grammer;
use grammer::FakeMarkdownParser;

#[derive(Default)]
pub struct IfdefProcessor {
    flags: HashSet<String>,
}

impl IfdefProcessor {
    #[must_use]
    pub const fn new(flags: HashSet<String>) -> Self {
        Self { flags }
    }

    #[must_use]
    pub fn from_vec(flags: Vec<String>) -> Self {
        Self {
            flags: flags.into_iter().collect::<HashSet<_>>(),
        }
    }

    fn process_chapter(&self, mut chapter: Chapter) -> Option<Chapter> {
        // Process contents, if None is returned - skip chapter and subsections.
        chapter.content =
            FakeMarkdownParser::fake_markdown_parse_and_clean(&chapter.content, &self.flags)?;

        // Handle sub items recursively.
        chapter.sub_items = chapter
            .sub_items
            .into_iter()
            .filter_map(|item| self.process_item(item))
            .collect();

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

impl Preprocessor for IfdefProcessor {
    fn name(&self) -> &str {
        "ifdef"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.sections = book
            .sections
            .into_iter()
            .filter_map(|item| self.process_item(item))
            .collect();

        Ok(book)
    }
}

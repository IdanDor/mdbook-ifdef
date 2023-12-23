#![deny(
    warnings,
    rustdoc::all,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]
use mdbook::BookItem;
use mdbook::book::{Book, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{PreprocessorContext, Preprocessor};

pub mod grammer;
use grammer::FakeMarkdownParser;

pub mod flags;
use flags::FlagsHolder;

pub struct IfdefProcessor {
    flags: FlagsHolder,
}


impl Default for IfdefProcessor {
    fn default() -> Self {
        IfdefProcessor{flags: FlagsHolder::default()}
    }
}

impl IfdefProcessor {
    pub fn new(flags: FlagsHolder) -> Self {
        IfdefProcessor{flags}
    }

    pub fn from_vec(flags: Vec<String>) -> Self {
        IfdefProcessor{flags: FlagsHolder::new(flags)}
    }

    fn process_chapter(&self, mut chapter: Chapter) -> Option<Chapter> {
        // Process contents, if None is returned - skip chapter and subsections.
        chapter.content = FakeMarkdownParser::fake_markdown_parse_and_clean(&chapter.content, &self.flags)?;

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

impl Preprocessor for IfdefProcessor {
    fn name(&self) -> &str {
        "ifdef"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> { 
        book.sections = book.sections.into_iter().map(
            |item| self.process_item(item)
        ).flatten().collect();

        Ok(book)
    }
}

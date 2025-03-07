use clap::ValueEnum;
use tabled::{Table, settings::Style};

#[derive(Debug, ValueEnum, Clone)]
pub enum TableStyle {
    Ascii,
    AsciiRounded,
    Blank,
    Dots,
    Empty,
    Extended,
    Markdown,
    Modern,
    ModernRounded,
    Psql,
    ReStructuredText,
    Rounded,
    Sharp,
}

pub struct TablePadding {
    pub left: usize,
    pub right: usize,
}

pub struct RenderConfig {
    pub style: TableStyle,
    pub padding: TablePadding,
}

impl TableStyle {
    pub fn apply_to(self, data: &mut Table) -> &mut Table {
        match self {
            TableStyle::Ascii => data.with(Style::ascii()),
            TableStyle::AsciiRounded => data.with(Style::ascii_rounded()),
            TableStyle::Blank => data.with(Style::blank()),
            TableStyle::Dots => data.with(Style::dots()),
            TableStyle::Empty => data.with(Style::empty()),
            TableStyle::Extended => data.with(Style::extended()),
            TableStyle::Markdown => data.with(Style::markdown()),
            TableStyle::Modern => data.with(Style::modern()),
            TableStyle::ModernRounded => data.with(Style::modern_rounded()),
            TableStyle::Psql => data.with(Style::psql()),
            TableStyle::ReStructuredText => data.with(Style::re_structured_text()),
            TableStyle::Rounded => data.with(Style::rounded()),
            TableStyle::Sharp => data.with(Style::sharp()),
        }
    }
}

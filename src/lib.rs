use clap::ValueEnum;
use tabled::{
    builder::Builder,
    settings::{Alignment, Style},
    Table,
};

pub struct Attribute {
    pub key: String,
    pub value: String,
}
pub fn get_row_vec(row: &str, delimiter: &str, num_cols: usize) -> Vec<String> {
    let mut items: Vec<&str> = row.split(delimiter).collect();
    if items.len() < num_cols {
        for _ in 0..num_cols - items.len() {
            items.push("")
        }
    }

    let items_slice = &items[0..num_cols];

    items_slice.iter().map(|s| s.trim().to_string()).collect()
}

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

impl TableStyle {
    fn apply_to(self, data: &mut Table) -> &mut Table {
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

pub fn get_output(data: &[Vec<String>], style: TableStyle) -> String {
    let mut builder = Builder::default();

    data.iter().for_each(|d| {
        builder.push_record(d);
    });

    let mut b = builder.build();

    b.with(Alignment::left());
    b.with(Style::sharp());

    style.apply_to(&mut b);

    b.to_string()
}

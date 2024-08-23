use tabled::{
    builder::Builder,
    settings::{Alignment, Style},
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

pub fn get_output(data: &[Vec<String>]) -> String {
    let mut builder = Builder::default();

    data.iter().for_each(|d| {
        builder.push_record(d);
    });

    builder
        .build()
        .with(Style::sharp())
        .with(Alignment::left())
        .to_string()
}

use clap::ValueEnum;
use tabled::{
    builder::Builder,
    settings::{Alignment, Padding, Style},
    Table,
};

pub struct Attribute {
    pub key: String,
    pub value: String,
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

pub struct TablePadding {
    pub left: usize,
    pub right: usize,
}

pub struct RenderConfig {
    pub style: TableStyle,
    pub padding: TablePadding,
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

pub fn get_output(data: &[Vec<String>], config: RenderConfig) -> String {
    let mut builder = Builder::default();

    data.iter().for_each(|d| {
        builder.push_record(d);
    });

    let mut b = builder.build();

    b.with(Alignment::left());
    b.with(Style::sharp());
    b.with(Padding::new(
        config.padding.left,
        config.padding.right,
        0,
        0,
    ));

    config.style.apply_to(&mut b);

    b.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_data() -> Vec<Vec<String>> {
        let data = vec![
            ["row1col1", "row1col2", "row1col3"],
            ["row2col1", "row2col2", "row2col3"],
            ["row3col1", "row3col2", "row3col3"],
        ];
        data.into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>()
    }

    #[test]
    fn get_row_vec_works_with_defaults() {
        // GIVEN
        let row_data = "row1col1,row1col2,row1col3";

        // WHEN
        let got = get_row_vec(row_data, ",", 3);

        // THEN
        let expected = ["row1col1", "row1col2", "row1col3"]
            .into_iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>();

        assert_eq!(got, expected);
    }

    #[test]
    fn get_row_vec_works_with_custom_delimiter() {
        // GIVEN
        let row_data = "row1col1|row1col2|row1col3";

        // WHEN
        let got = get_row_vec(row_data, "|", 3);

        // THEN
        let expected = ["row1col1", "row1col2", "row1col3"]
            .into_iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>();

        assert_eq!(got, expected);
    }

    #[test]
    fn get_row_vec_trims_cells() {
        // GIVEN
        let row_data = "row1col1 , row1col2,    row1col3   ";

        // WHEN
        let got = get_row_vec(row_data, ",", 3);

        // THEN
        let expected = ["row1col1", "row1col2", "row1col3"]
            .into_iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>();

        assert_eq!(got, expected);
    }

    #[test]
    fn get_row_vec_limits_output_to_num_cols() {
        // GIVEN
        let row_data = "row1col1,row1col2,row1col3";

        // WHEN
        let got = get_row_vec(row_data, ",", 2);

        // THEN
        let expected = ["row1col1", "row1col2"]
            .into_iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>();

        assert_eq!(got, expected);
    }

    #[test]
    fn get_row_vec_creates_empty_cells_if_asked() {
        // GIVEN
        let row_data = "row1col1,row1col2,row1col3";

        // WHEN
        let got = get_row_vec(row_data, ",", 5);

        // THEN
        let expected = ["row1col1", "row1col2", "row1col3", "", ""]
            .into_iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>();

        assert_eq!(got, expected);
    }

    #[test]
    fn renders_defaults_correctly() {
        // GIVEN
        let data = generate_data();
        let config = RenderConfig {
            style: TableStyle::Sharp,
            padding: TablePadding { left: 1, right: 1 },
        };

        // WHEN
        let got = get_output(&data, config);

        // THEN
        let expected = "
┌──────────┬──────────┬──────────┐
│ row1col1 │ row1col2 │ row1col3 │
├──────────┼──────────┼──────────┤
│ row2col1 │ row2col2 │ row2col3 │
│ row3col1 │ row3col2 │ row3col3 │
└──────────┴──────────┴──────────┘
";
        assert_eq!(got, expected.trim());
    }

    #[test]
    fn renders_ascii_style_correctly() {
        // GIVEN
        let data = generate_data();
        let config = RenderConfig {
            style: TableStyle::Ascii,
            padding: TablePadding { left: 1, right: 1 },
        };

        // WHEN
        let got = get_output(&data, config);

        // THEN
        let expected = "
+----------+----------+----------+
| row1col1 | row1col2 | row1col3 |
+----------+----------+----------+
| row2col1 | row2col2 | row2col3 |
+----------+----------+----------+
| row3col1 | row3col2 | row3col3 |
+----------+----------+----------+
";
        assert_eq!(got, expected.trim());
    }

    #[test]
    fn renders_padding_correctly() {
        // GIVEN
        let data = generate_data();
        let config = RenderConfig {
            style: TableStyle::Sharp,
            padding: TablePadding { left: 1, right: 2 },
        };

        // WHEN
        let got = get_output(&data, config);

        // THEN
        let expected = "
┌───────────┬───────────┬───────────┐
│ row1col1  │ row1col2  │ row1col3  │
├───────────┼───────────┼───────────┤
│ row2col1  │ row2col2  │ row2col3  │
│ row3col1  │ row3col2  │ row3col3  │
└───────────┴───────────┴───────────┘
";
        assert_eq!(got, expected.trim());
    }
}

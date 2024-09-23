use crate::config::RenderConfig;
use tabled::{
    builder::Builder,
    settings::{Alignment, Padding, Style},
};

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
    use crate::config::{RenderConfig, TablePadding, TableStyle};

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

    fn to_string_vec(str_vec: Vec<&str>) -> Vec<String> {
        str_vec.into_iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn get_row_vec_works_with_defaults() {
        // GIVEN
        let row_data = "row1col1,row1col2,row1col3";

        // WHEN
        let got = get_row_vec(row_data, ",", 3);

        // THEN
        let expected = to_string_vec(vec!["row1col1", "row1col2", "row1col3"]);

        assert_eq!(got, expected);
    }

    #[test]
    fn get_row_vec_works_with_custom_delimiter() {
        // GIVEN
        let row_data = "row1col1|row1col2|row1col3";

        // WHEN
        let got = get_row_vec(row_data, "|", 3);

        // THEN
        let expected = to_string_vec(vec!["row1col1", "row1col2", "row1col3"]);

        assert_eq!(got, expected);
    }

    #[test]
    fn get_row_vec_trims_cells() {
        // GIVEN
        let row_data = "row1col1 , row1col2,    row1col3   ";

        // WHEN
        let got = get_row_vec(row_data, ",", 3);

        // THEN
        let expected = to_string_vec(vec!["row1col1", "row1col2", "row1col3"]);

        assert_eq!(got, expected);
    }

    #[test]
    fn get_row_vec_limits_output_to_num_cols() {
        // GIVEN
        let row_data = "row1col1,row1col2,row1col3";

        // WHEN
        let got = get_row_vec(row_data, ",", 2);

        // THEN
        let expected = to_string_vec(vec!["row1col1", "row1col2"]);

        assert_eq!(got, expected);
    }

    #[test]
    fn get_row_vec_creates_empty_cells_if_asked() {
        // GIVEN
        let row_data = "row1col1,row1col2,row1col3";

        // WHEN
        let got = get_row_vec(row_data, ",", 5);

        // THEN
        let expected = to_string_vec(vec!["row1col1", "row1col2", "row1col3", "", ""]);

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

use crate::config::RenderConfig;
use csv::StringRecord;
use tabled::{
    builder::Builder,
    settings::{Alignment, Padding, Style},
};

pub fn get_output(data: &[StringRecord], config: RenderConfig) -> String {
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

    fn generate_data() -> Vec<StringRecord> {
        let data = vec![
            ["row1col1", "row1col2", "row1col3"],
            ["row2col1", "row2col2", "row2col3"],
            ["row3col1", "row3col2", "row3col3"],
        ];
        data.into_iter()
            .map(|row| {
                StringRecord::from(
                    row.into_iter()
                        .map(|item| item.to_string())
                        .collect::<Vec<String>>(),
                )
            })
            .collect::<Vec<StringRecord>>()
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

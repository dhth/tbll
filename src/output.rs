use crate::config::RenderConfig;
use crate::types::Cols;
use csv::StringRecord;
use tabled::{
    builder::Builder,
    settings::{Alignment, Padding, Style},
};

pub fn get_output(
    data: &[StringRecord],
    config: RenderConfig,
    cols: Option<Cols>,
) -> Option<String> {
    let mut builder = Builder::default();

    match cols {
        Some(indices) => {
            if indices.include {
                data.iter().for_each(|record| {
                    let vals = record
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| indices.values.contains(i))
                        .map(|(_, s)| s)
                        .collect::<Vec<_>>();
                    if !vals.is_empty() {
                        builder.push_record(vals);
                    }
                });
            } else {
                data.iter().for_each(|record| {
                    let vals = record
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| !indices.values.contains(i))
                        .map(|(_, s)| s)
                        .collect::<Vec<_>>();
                    if !vals.is_empty() {
                        builder.push_record(vals);
                    }
                });
            }
        }
        None => {
            data.iter().for_each(|record| {
                builder.push_record(record);
            });
        }
    }

    let mut b = builder.build();
    if b.count_rows() == 0 {
        return None;
    }

    b.with(Alignment::left());
    b.with(Style::sharp());
    b.with(Padding::new(
        config.padding.left,
        config.padding.right,
        0,
        0,
    ));

    config.style.apply_to(&mut b);

    Some(b.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{RenderConfig, TablePadding, TableStyle};
    use insta::assert_snapshot;

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
        let got = get_output(&data, config, None).expect("a string should've been returned");

        // THEN
        assert_snapshot!(got, @r"
        ┌──────────┬──────────┬──────────┐
        │ row1col1 │ row1col2 │ row1col3 │
        ├──────────┼──────────┼──────────┤
        │ row2col1 │ row2col2 │ row2col3 │
        │ row3col1 │ row3col2 │ row3col3 │
        └──────────┴──────────┴──────────┘
        ");
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
        let got = get_output(&data, config, None).expect("a string should've been returned");

        // THEN
        assert_snapshot!(got, @r"
        +----------+----------+----------+
        | row1col1 | row1col2 | row1col3 |
        +----------+----------+----------+
        | row2col1 | row2col2 | row2col3 |
        +----------+----------+----------+
        | row3col1 | row3col2 | row3col3 |
        +----------+----------+----------+
        ");
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
        let got = get_output(&data, config, None).expect("a string should've been returned");

        // THEN
        assert_snapshot!(got, @r"
        ┌───────────┬───────────┬───────────┐
        │ row1col1  │ row1col2  │ row1col3  │
        ├───────────┼───────────┼───────────┤
        │ row2col1  │ row2col2  │ row2col3  │
        │ row3col1  │ row3col2  │ row3col3  │
        └───────────┴───────────┴───────────┘
        ");
    }

    #[test]
    fn selects_indices_correctly() {
        // GIVEN
        let data = generate_data();
        let config = RenderConfig {
            style: TableStyle::Sharp,
            padding: TablePadding { left: 1, right: 1 },
        };

        // WHEN
        let got = get_output(&data, config, Some(Cols::include(vec![0, 2])))
            .expect("a string should've been returned");

        // THEN
        assert_snapshot!(got, @r"
        ┌──────────┬──────────┐
        │ row1col1 │ row1col3 │
        ├──────────┼──────────┤
        │ row2col1 │ row2col3 │
        │ row3col1 │ row3col3 │
        └──────────┴──────────┘
        ");
    }

    #[test]
    fn skips_indices_correctly() {
        // GIVEN
        let data = generate_data();
        let config = RenderConfig {
            style: TableStyle::Sharp,
            padding: TablePadding { left: 1, right: 1 },
        };

        // WHEN
        let got = get_output(&data, config, Some(Cols::skip(vec![0, 2])))
            .expect("a string should've been returned");

        // THEN
        assert_snapshot!(got, @r"
        ┌──────────┐
        │ row1col2 │
        ├──────────┤
        │ row2col2 │
        │ row3col2 │
        └──────────┘
        ");
    }

    #[test]
    fn returns_nothing_if_indices_are_incorrect() {
        // GIVEN
        let data = generate_data();
        let config = RenderConfig {
            style: TableStyle::Sharp,
            padding: TablePadding { left: 1, right: 1 },
        };

        // WHEN
        let got = get_output(&data, config, Some(Cols::include(vec![5, 8])));

        // THEN
        assert!(got.is_none());
    }
}

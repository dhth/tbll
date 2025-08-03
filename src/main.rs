mod config;
mod output;
mod types;

use anyhow::Context;
use clap::Parser;
use config::{RenderConfig, TablePadding, TableStyle};
use csv::StringRecord;
use output::get_output;
use std::fs::File;
use std::io::BufReader;
use types::Cols;

const ROW_DELIMITER: &str = ",";

/// tbll outputs data in tabular format
#[derive(Parser, Debug)]
#[command(about, long_about=None)]
struct Args {
    /// Input file path; tbll will read from stdin if this is not provided
    #[arg(short = 'p', long = "input-path", value_name = "STRING")]
    input_file_path: Option<String>,
    /// Delimiter to use
    #[arg(short = 'd', long = "delimiter", value_name = "STRING")]
    #[clap(default_value = ROW_DELIMITER)]
    delimiter: char,
    /// Command separated list of headers
    #[arg(long = "headers", value_name = "STRING,STRING...")]
    headers: Option<String>,
    /// Indices of columns (starting from zero) to display
    #[arg(
        short = 'c',
        long = "cols",
        value_name = "NUMBER,NUMBER...",
        value_delimiter = ','
    )]
    cols: Option<Vec<usize>>,
    /// Indices of columns (starting from zero) to skip
    #[arg(
        short = 'C',
        long = "skip-cols",
        value_name = "NUMBER,NUMBER...",
        value_delimiter = ','
    )]
    skip_cols: Option<Vec<usize>>,
    /// Border Style
    #[arg(short = 's', long = "style", value_name = "STRING")]
    #[clap(value_enum, default_value = "sharp", value_name = "STRING")]
    style: TableStyle,
    /// Left padding for cells
    #[arg(short = 'l', long = "left-pad", value_name = "NUMBER")]
    #[clap(default_value = "1")]
    left_pad: usize,
    /// Right padding for cells
    #[arg(short = 'r', long = "right-pad", value_name = "NUMBER")]
    #[clap(default_value = "1")]
    right_pad: usize,
    /// Trim whitespace from cells
    #[arg(short = 't', long = "trim")]
    trim: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut data: Vec<StringRecord> = Vec::new();

    let maybe_cols = match (args.cols, args.skip_cols) {
        (Some(_), Some(_)) => Err(anyhow::anyhow!(
            "--cols and --skip-cols cannot be used at the same time"
        )),
        (None, None) => Ok(None),
        (Some(c), None) => Ok(Some(Cols::include(c))),
        (None, Some(c)) => Ok(Some(Cols::skip(c))),
    }?;

    if let Some(headers) = &args.headers {
        let headers_vec: Vec<&str> = headers.split(",").collect();
        data.push(StringRecord::from(
            headers_vec
                .into_iter()
                .map(|s| {
                    if args.trim {
                        s.trim().to_string()
                    } else {
                        s.to_string()
                    }
                })
                .collect::<Vec<String>>(),
        ));
    }

    match args.input_file_path {
        None => {
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .delimiter(args.delimiter as u8)
                .from_reader(std::io::stdin());

            for result in reader.records() {
                let record = result.context("couldn't read row from stdin")?;
                if args.trim {
                    data.push(get_trimmed_record(record));
                } else {
                    data.push(record);
                }
            }
        }
        Some(path) => {
            let file = File::open(path)?;
            let file_reader = BufReader::new(file);

            let mut reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .delimiter(args.delimiter as u8)
                .from_reader(file_reader);

            for result in reader.records() {
                let record = result.context("couldn't read row in file")?;
                if args.trim {
                    data.push(get_trimmed_record(record));
                } else {
                    data.push(record);
                }
            }
        }
    };

    let padding = TablePadding {
        left: args.left_pad,
        right: args.right_pad,
    };

    let config = RenderConfig {
        style: args.style,
        padding,
    };

    if let Some(output) = get_output(&data, config, maybe_cols) {
        println!("{output}");
    }

    Ok(())
}

fn get_trimmed_record(record: StringRecord) -> StringRecord {
    StringRecord::from(
        record
            .iter()
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>(),
    )
}

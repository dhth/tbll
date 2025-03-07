mod config;
mod output;

use anyhow::Context;
use clap::Parser;
use config::{RenderConfig, TablePadding, TableStyle};
use csv::StringRecord;
use output::get_output;
use std::fs::File;
use std::io::BufReader;

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
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut data: Vec<StringRecord> = Vec::new();

    if let Some(headers) = &args.headers {
        let headers_vec: Vec<&str> = headers.split(",").collect();
        data.push(StringRecord::from(
            headers_vec
                .into_iter()
                .map(|s| s.trim().to_string())
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
                data.push(record);
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
                data.push(record);
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

    let output = get_output(&data, config);

    println!("{output}");

    Ok(())
}

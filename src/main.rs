mod config;
mod output;

use anyhow::Context;
use clap::Parser;
use config::{RenderConfig, TablePadding, TableStyle};
use output::{get_output, get_row_vec};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const ROW_DELIMITER: &str = ",";

/// tbll outputs data in tabular format
#[derive(Parser, Debug)]
#[command(about, long_about=None)]
struct Args {
    /// Row elements
    #[arg(short = 'r', long = "row", value_name = "STRING:::STRING...")]
    row: Option<Vec<String>>,
    /// Input file path
    #[arg(short = 'p', long = "input-path", value_name = "STRING")]
    input_file_path: Option<String>,
    /// Whether to read input from stdin
    #[arg(short = 's', long = "read-stdin")]
    read_stdin: bool,
    /// Delimiter to use
    #[arg(short = 'd', long = "delimiter", value_name = "STRING")]
    #[clap(default_value = ROW_DELIMITER)]
    delimiter: String,
    /// Number of columns to output
    #[arg(short = 'n', long = "num-cols", value_name = "NUMBER")]
    #[clap(default_value = "1")]
    num_cols: usize,
    /// Command separated list of headers; overrides --num-cols when provided
    #[arg(long = "headers", value_name = "STRING,STRING...")]
    headers: Option<String>,
    /// Border Style
    #[arg(long = "style", value_name = "STRING")]
    #[clap(value_enum, default_value = "sharp", value_name = "STRING")]
    style: TableStyle,
    /// Left padding for cells
    #[arg(long = "left-pad", value_name = "NUMBER")]
    #[clap(default_value = "1")]
    left_pad: usize,
    /// Right padding for cells
    #[arg(long = "right-pad", value_name = "NUMBER")]
    #[clap(default_value = "1")]
    right_pad: usize,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut data: Vec<Vec<String>> = Vec::new();

    let mut num_cols = args.num_cols;

    if let Some(headers) = args.headers {
        let headers_vec: Vec<&str> = headers.split(",").collect();
        num_cols = headers_vec.len();
        data.push(
            headers_vec
                .into_iter()
                .map(|s| s.trim().to_string())
                .collect(),
        );
    }

    if let Some(attributes_input) = args.row {
        for attr_inp in attributes_input {
            let attr = get_row_vec(&attr_inp, &args.delimiter, num_cols);
            data.push(attr)
        }
    }

    match (args.read_stdin, args.input_file_path) {
        (false, None) => {
            return Err(anyhow::anyhow!(
                "a source needs to be provided (either a file or stdin)"
            ));
        }
        (true, Some(_)) => {
            return Err(anyhow::anyhow!(
                "only one source (either a file or stdin) can be used at a time"
            ));
        }
        (true, None) => {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                let line = line.context("couldn't read line from stdin")?;
                let attr = get_row_vec(&line, &args.delimiter, num_cols);
                data.push(attr)
            }
        }
        (false, Some(path)) => {
            let file = File::open(path)?;
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let line = line.context("couldn't read line from stdin")?;
                let attr = get_row_vec(&line, &args.delimiter, num_cols);
                data.push(attr)
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

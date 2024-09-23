mod config;
mod output;

use clap::Parser;
use config::{RenderConfig, TablePadding, TableStyle};
use output::{get_output, get_row_vec};
use std::io::{self, BufRead};
use std::process;

const ROW_DELIMITER: &str = ",";

/// tbll outputs data in tabular format
#[derive(Parser, Debug)]
#[command(about, long_about=None)]
struct Args {
    /// Row elements
    #[arg(short = 'r', long = "row", value_name = "STRING:::STRING...")]
    row: Option<Vec<String>>,
    /// Whether to read row elements from stdin
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

fn main() {
    let args = Args::parse();

    let mut data: Vec<Vec<String>> = Vec::new();

    let mut num_cols = args.num_cols;
    let style = args.style;

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

    if args.read_stdin {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line.unwrap_or_else(|err| {
                eprintln!("Error: could not read line from stdin: {}", err);
                process::exit(1);
            });

            let attr = get_row_vec(&line, &args.delimiter, num_cols);
            data.push(attr)
        }
    }

    let padding = TablePadding {
        left: args.left_pad,
        right: args.right_pad,
    };

    let config = RenderConfig { style, padding };

    let output = get_output(&data, config);

    println!("{output}");
}

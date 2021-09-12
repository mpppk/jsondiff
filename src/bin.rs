use anyhow::Result;
use jsondiff::{diff, open_file};
use serde_json::Value;
use std::path::PathBuf;
use structopt::StructOpt;

/// A tool for outputs semantic difference of json
#[derive(StructOpt, Debug)]
#[structopt(name = "jsondiff")]
struct Opt {
    /// Generate diffs with <n> lines of context
    #[structopt(short = "U", default_value = "3")]
    unified: usize,

    /// Outputs normalized json as "normalized1.json" and "normalized2.json"
    #[structopt(long = "output-normalized-json", short = "n")]
    output_normalized_json: bool,

    file_path1: PathBuf,
    file_path2: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let file1 = open_file(opt.file_path1)?;
    let file2 = open_file(opt.file_path2)?;

    let v1: Value = serde_json::from_reader(file1)?;
    let v2: Value = serde_json::from_reader(file2)?;
    println!("{}", diff(v1, v2, opt.unified, opt.output_normalized_json)?);
    Ok(())
}

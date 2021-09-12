use anyhow::{Result};
use jsondiff::{diff, normalize_value, open_file, normalize_from_file_path};
use serde_json::Value;
use std::path::PathBuf;
use structopt::StructOpt;

/// CLI for prints json diff
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(subcommand)]
    pub sub: Sub,
}

#[derive(StructOpt, Debug)]
#[structopt()]
enum Sub {
    /// Normalize json
    Normalize { file_path: PathBuf },
    /// Prints diff of json files
    Diff {
        #[structopt(short = "U", default_value = "3")]
        unified: usize,
        file_path1: PathBuf,
        file_path2: PathBuf,
    },
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    match opt.sub {
        Sub::Diff {
            unified,
            file_path1,
            file_path2,
        } => {
            let file1 = open_file(file_path1)?;
            let file2 = open_file(file_path2)?;

            let v1: Value = serde_json::from_reader(file1)?;
            let v2: Value = serde_json::from_reader(file2)?;
            println!("{}", diff(v1, v2, unified));
            Ok(())
        }
        Sub::Normalize { file_path } => {
            let v = normalize_from_file_path(file_path);
            let pretty_json = serde_json::to_string_pretty(&normalize_value(v, true)).unwrap();
            println!("{}", pretty_json);
            Ok(())
        }
    }
}

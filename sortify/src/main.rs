mod sorter;
mod file_utils;

use clap::Parser;

/// Sort files in a directory by size using merge sort
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Path to the directory
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    let files = file_utils::collect_files(&args.path);
    let sorted = sorter::merge_sort(files);

    for file in sorted {
        if let Ok(metadata) = file.metadata() {
            println!("{} - {} bytes", file.path().display(), metadata.len());
        }
    }
}
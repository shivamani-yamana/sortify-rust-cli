use walkdir::{WalkDir,DirEntry};

pub fn collect_files(path : &str) -> Vec<DirEntry> {
    WalkDir::new(path).max_depth(1).into_iter().filter_map(Result::ok).filter(|e| e.file_type().is_file()).collect()
}
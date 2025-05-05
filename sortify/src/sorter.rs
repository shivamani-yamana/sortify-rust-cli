use walkdir::DirEntry;

pub fn merge_sort(mut files:Vec<DirEntry>) -> Vec<DirEntry> {
    if files.len()<=1 {
        return files;
    }
    let mid: usize = files.len()/2;

    let left: Vec<DirEntry> = merge_sort(files.drain(..mid).collect());
    let right: Vec<DirEntry> = merge_sort(files);

    return merge(left,right);
}

fn merge(mut left:Vec<DirEntry>,mut right:Vec<DirEntry>) -> Vec<DirEntry> {
    let mut result: Vec<DirEntry> = Vec::new();

    while !left.is_empty() && !right.is_empty() {
        let l_size: u64 = left[0].metadata().map(|m| m.len()).unwrap_or(0);
        let r_size: u64 = right[0].metadata().map(|m| m.len()).unwrap_or(0);

        if l_size <= r_size {
            result.push(left.remove(0));
        } else {
            result.push(right.remove(0));
        }
    }

    result.extend(left);
    result.extend(right);

    result
}
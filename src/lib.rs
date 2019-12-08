use std::fs;
use std::io::Read;
use std::io;

pub struct File {
    pub path: String,
    pub length: u64,
    pub duplications: Vec<File>,
}

impl File {
    fn copy(&self) -> File {
        File { 
            path: self.path.clone(),
            length: self.length,
            duplications: Vec::new()
        }
    }
}

impl PartialEq for File {
    fn eq(&self, other: &File) -> bool {
        if self.length == other.length && self.path == other.path {
            return true;
        } else {
            return false;
        }
    }
}

fn are_files_equal(file1: &File, file2: &File) -> bool {
    if file1.length == 0 && file2.length == 0 {
        return true;
    }

    let mut f1 = fs::File::open(&file1.path).expect("Couldn't open file").bytes();
    let mut f2 = fs::File::open(&file2.path).expect("Couldn't open file").bytes();

    let mut next_byte1 = f1.next().unwrap().expect("Error in reading byte");
    let mut next_byte2 = f2.next().unwrap().expect("Error in reading byte");
    while &next_byte1 == &next_byte2 {
        let b1 = f1.next();
        match b1 {
            Some(value) => {
                next_byte1 = value.expect("Error in reading byte");
            },
            None => return true,
        }
        next_byte2 = f2.next().unwrap().expect("No next byte");
    }

    return false
}

pub fn find_dups(files: Vec<File>) -> io::Result<Vec<File>> {
    let mut dups: Vec<File> = Vec::new();

    for c_item in &files {
        let mut duplicated_file = File {
            path: c_item.path.clone(),
            length: c_item.length,
            duplications: Vec::new()
        };
        for file in &files {
            if c_item.path != file.path && c_item.length == file.length && are_files_equal(&c_item, &file) {
                duplicated_file.duplications.push(file.copy());
            }
        }
        if duplicated_file.duplications.len() > 0 {
            dups.push(duplicated_file)
        }
    }
    
    Ok(dups)
}

pub fn collect_files(directory: &str, recursive: bool) -> io::Result<Vec<File>> {
    let mut files: Vec<File> = Vec::new();

    for entry in fs::read_dir(directory)? {
        let dir = entry?;
        let file_metadata = dir.metadata()?;
        if file_metadata.is_file() {
            let path = dir.path();
            let path_str = path.to_str()
                .expect("File has no name");
            let length = file_metadata.len();
            files.push(File { path: path_str.to_string(), length, duplications: Vec::new() });
        } else if recursive {
            let path = dir.path();
            let path_str = path.to_str()
                .expect("File has no name");
            let mut subdirectory_files = collect_files(path_str, recursive).expect("Can't read subdir");
            files.append(&mut subdirectory_files);
        }
    }
    Ok(files)
}
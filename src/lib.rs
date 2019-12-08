use std::fs;
use std::io::Read;
use std::io;
// use std::error::Error;

pub struct File {
    pub path: String,
    pub length: u64,
    pub duplications: Vec<String>,
    pub skip: bool
}

impl File {
    fn copy(&self) -> File {
        File { 
            path: self.path.clone(),
            length: self.length,
            duplications: Vec::new(),
            skip: false
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

fn are_files_equal(file1: &File, file2: &File) -> io::Result<bool> {
    if file1.length == 0 && file2.length == 0 {
        return Ok(true);
    }

    let mut f1 = match fs::File::open(&file1.path) {
        Ok(f) => f.bytes(),
        Err(_) => {
            eprintln!("Couldn't open file {}", file1.path);
            return Ok(false)
        }
    };
    let mut f2 = match fs::File::open(&file2.path) {
        Ok(f) => f.bytes(),
        Err(_) => {
            eprintln!("Couldn't open file {}", file2.path);
            return Ok(false)
        }
    };

    let mut next_byte1 = match f1.next().unwrap() {
        Ok(b) => b,
        Err(_) => {
            eprintln!("There was an error reading the file {}", file1.path);
            return Ok(false)
        }
    };
    let mut next_byte2 = match f2.next().unwrap() {
        Ok(b) => b,
        Err(_) => {
            eprintln!("There was an error reading the file {}", file2.path);
            return Ok(false)
        }
    };

    while &next_byte1 == &next_byte2 {
        let b1 = f1.next();
        match b1 {
            Some(value) => {
                next_byte1 = match value {
                    Ok(b) => b,
                    Err(_) => {
                        eprintln!("There was an error reading the file {}", file1.path);
                        return Ok(false)
                    }
                };
            },
            None => return Ok(true),
        }
        next_byte2 = match f2.next().unwrap() {
            Ok(b) => b,
            Err(_) => {
                eprintln!("End of file {}", file1.path);
                return Ok(false)
            }
        };
    }

    return Ok(false)
}

pub fn find_dups(files: &mut Vec<File>) -> io::Result<Vec<File>> {
    let mut dups: Vec<File> = Vec::new();

    for i in 0..files.len() {
        files[i].skip = true;
        let mut duplicated_file = files[i].copy();
        for j in 0..files.len() {
            if !files[j].skip && files[i].path != files[j].path && files[i].length == files[j].length {
                match are_files_equal(&files[i], &files[j]) {
                    Ok(is_equal) => {
                        if is_equal {
                            files[j].skip = true;
                            duplicated_file.duplications.push(files[j].path.clone());
                        }
                    },
                    Err(_) => {
                        eprintln!("Can't parse file {}, skipping file...", &files[j].path)
                    }
                }
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
            let path_str = match path.to_str() {
                Some(p) => p,
                None => {
                    eprintln!("Can't read file path, skipping file...");
                    continue
                }
            };
                
            let length = file_metadata.len();
            files.push(File { path: path_str.to_string(), length, duplications: Vec::new(), skip: false });
        } else if recursive {
            let path = dir.path();
            let path_str = match path.to_str() {
                Some(d) => d,
                None => {
                    eprintln!("Can't read directory path, skipping directory...");
                    continue
                }
            };
            let mut subdirectory_files = match collect_files(path_str, recursive) {
                Ok(s) => s,
                Err(_) => {
                    eprintln!("Can't process directory, skipping directory...");
                    continue
                }
            };
            files.append(&mut subdirectory_files);
        }
    }
    Ok(files)
}
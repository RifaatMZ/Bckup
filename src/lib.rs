use std::error::Error;
use std::path::Path;
use std::{fs, env};

pub struct Config {
    pub source: String,
    pub destination: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let source = match args.next() {
            Some(args) => args,
            None => return Err("Didn't get a folder name to backup"),
        };
        let destination = match args.next() {
            Some(args) => args,
            None => return  Err("Didn't get backup directory"),            
        };
        Ok(Config{
            source,
            destination,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    copy_dir(config.source, config.destination)?;
    Ok(())
}

fn copy_dir(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<(), Box<dyn Error>>{
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ft = entry.file_type()?;
        if ft.is_dir() {
            copy_dir(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            let src_file = entry.path();
            let dst_file = dst.as_ref().join(entry.file_name());
            if dst_file.exists() {
                let modified = is_modified(&src_file, &dst_file).unwrap();
                if modified {
                    fs::copy(src_file, dst_file)?;
                }
            } else {
                fs::copy(src_file, dst_file)?;
            }          
        }
    }
    Ok(())
}

fn is_modified(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<bool, Box<dyn Error>> {
    let src_metadata = fs::metadata(src)?;
    let dst_metadata = fs::metadata(dst)?;

    let src_time = src_metadata.modified().unwrap();
    let dst_time = dst_metadata.modified().unwrap();

    Ok(src_time != dst_time)
}

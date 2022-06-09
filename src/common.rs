use std::path::PathBuf;
use std::path::Path;

use dirs::home_dir;

pub fn base_absolute_path() -> String {
    let home = home_dir().unwrap();
    let home = home.to_str().unwrap();
    String::from(home) + "/.local/bcp"
}

pub fn get_abs_path(relative_path:&str) -> PathBuf {
    match Path::new(relative_path).canonicalize() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{} does not exsist.",relative_path);
            // panic!("error: {}",e.kind()) // errorの種類を調べたいなら
            std::process::exit(120)
        }
    }
}
use std::path::PathBuf;
use std::path::Path;

use dirs::home_dir;
use lazy_static::lazy_static;
use regex::Regex;


pub fn base_absolute_path() -> String {
    let home = home_dir().unwrap();
    let home = home.to_str().unwrap();
    String::from(home) + "/.local/bcp/"
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

pub fn check_filename_format(fname:&str) -> bool {
    lazy_static! {
        static ref RE : Regex = Regex::new(r"^.*_[0-9]{8}-[0-9]{6}_.*$").unwrap();
    }
    RE.is_match(fname)
}

#[cfg(test)]
mod common_test {
    use crate::common::check_filename_format;


    #[test]
    fn test_regex() {
        let fname = "abcd_20221231-231140_nocomment";
        assert!(check_filename_format(fname));
        let fname = "abcd_20221231-231140_Why2022-12-1";
        assert!(check_filename_format(fname));
        let fname = "abcd_2022123112-231140_Why2022-12-1";
        assert!(!check_filename_format(fname));
        let fname = "20221231-240000-_2022123112-231140_Why2022-12-1";
        assert!(!check_filename_format(fname));
        let fname = "abcd_20221231-231140";
        assert!(!check_filename_format(fname));
        let fname = "foo.rs_20220609-230838_nocomment";
        assert!(check_filename_format(fname));
    }
}
use clap::ArgMatches;

use std::fs::{read_dir,metadata, DirEntry};

use crate::common ;

pub fn restore(matches : &ArgMatches) {
    let target = matches.value_of("src").unwrap();
    let abs_src = common::get_abs_path(target);
    let src_type = metadata(abs_src.clone()).unwrap();
    let abs_target = common::base_absolute_path();
    let (path_to_src,file_name) = abs_src.to_str().unwrap().rsplit_once("/").unwrap();
    let vec = match read_dir(abs_target + path_to_src) {
        Err(_) => {std::process::exit(124)},
        // やばすぎ。後でなんとかしたい
        Ok(iter) => {
            let mut v = Vec::new();
            for entry  in iter.filter(|r| r.is_ok())
                .map(|r| r.unwrap())
                .filter(|entry| {  
                    let ft = entry.file_type().unwrap();
                        (ft.is_dir() && src_type.is_dir()) ||
                        (ft.is_file() && src_type.is_file()) ||
                        (ft.is_symlink() && src_type.is_symlink()) 
                }) {
                    v.push(entry)
                }
            v
        }
    };
    // file_nameでさらに絞る
    // dir entryから候補を探す
    // restore
    todo!()
}
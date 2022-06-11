use std::fs::metadata;

use clap::ArgMatches;

use crate::common;

pub fn show(matches:&ArgMatches) {
    let src_value = matches.value_of("src").unwrap();
    let abs_src = common::get_abs_path(src_value);
    let abs_base = common::base_absolute_path();
    let src_type = metadata(abs_src.clone()).unwrap();
    let (path_to_src,file_name) = abs_src.to_str().unwrap().rsplit_once("/").unwrap();
    let vec = common::get_target_list(abs_base.clone() + path_to_src, file_name, src_type);
    for entry in vec {
        println!("Found : {:?}",entry.file_name())
    }
}
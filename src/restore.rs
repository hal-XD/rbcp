use clap::ArgMatches;

use std::fmt::Debug;
use std::fs::{read_dir,metadata, File,};
use std::io::{Read,Write};
use std::path::Path;

use crate::common;


pub fn restore(matches : &ArgMatches) {
    let src_value = matches.value_of("src").unwrap();
    let abs_src = common::get_abs_path(src_value);
    let src_type = metadata(abs_src.clone()).unwrap();
    let abs_base = common::base_absolute_path();
    let (path_to_src,file_name) = abs_src.to_str().unwrap().rsplit_once("/").unwrap();
    let vec = common::get_target_list(abs_base.clone() + path_to_src, file_name, src_type);
    // restore
    let mut tmp= String::from("");
    for entry in vec {
        let fname = String::from(entry.file_name().to_str().unwrap());
        if compare_timestamp(tmp.as_str(), fname.as_str(),file_name) {
            tmp = fname;
        };
    }
    println!("{}",tmp);
    replace_file(abs_base + path_to_src + "/" + tmp.as_str(),abs_src)
}

fn replace_file<A,B>(src: A,dest: B)
where
    A : AsRef<Path> + Debug,
    B : AsRef<Path> + Debug,
{
    println!("src {:?}\ndest {:?}",src,dest);
    let sfd = File::open(src).unwrap();
    let mut dfd = File::create(dest).unwrap();
    for b in sfd.bytes() {
        dfd.write(&[b.unwrap()]);
    }
}

fn compare_timestamp(first:&str, second:&str, filename: &str) -> bool {
    // _yyyymmdd-HHSSMM_ 16
    if !common::check_filename_format(first) {
        return true;
    }
    if !common::check_filename_format(second) {
        return false;
    }
    let first = first.replace(filename, "");
    let (s_date,_) = first.split_at(16);
    let second = second.replace(filename, "");
    let (d_date,_) = second.split_at(16); 
    d_date.replace("_","")
        .replace("-", "").parse::<u64>().unwrap()
    > s_date.replace("_", "")
        .replace("-", "").parse::<u64>().unwrap()
}


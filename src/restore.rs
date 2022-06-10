use clap::ArgMatches;

use std::fs::{read_dir,metadata, DirEntry};

use crate::common::{self, base_absolute_path} ;


pub fn restore(matches : &ArgMatches) {
    let target = matches.value_of("src").unwrap();
    let abs_src = common::get_abs_path(target);
    let src_type = metadata(abs_src.clone()).unwrap();
    let abs_target = common::base_absolute_path();
    let (path_to_src,file_name) = abs_src.to_str().unwrap().rsplit_once("/").unwrap();
    let vec = match read_dir(abs_target.clone() + path_to_src) {
        Err(_) => {std::process::exit(124)},
        Ok(iter) => {
            // DirEntryはcollectが使えないのでvecに詰める
            let mut v = Vec::new();
            for entry  in iter
                .filter(|r| r.is_ok())
                .map(|r| r.unwrap())
                .filter(|entry| {  
                    let ft = entry.file_type().unwrap();
                        (ft.is_dir() && src_type.is_dir()) ||
                        (ft.is_file() && src_type.is_file()) ||
                        (ft.is_symlink() && src_type.is_symlink()) 
                })
                .filter(|entry| {
                    entry.file_name().to_str().unwrap().starts_with(file_name)
                })
            {
                v.push(entry)
            }
            v
        }
    };
    // restore
    let mut tmp= String::from("");
    for entry in vec {
        let fname = String::from(entry.file_name().to_str().unwrap());
        //println!("candidate=[{}]",fname);
        if compare_timestamp(tmp.as_str(), fname.as_str(),file_name) {
            tmp = fname;
        };
    }
    //println!("target   =[{}]",tmp);
    println!("{}",tmp);
    replace_file(abs_src.to_str().unwrap(), abs_target + path_to_src + tmp.as_str())
}

fn replace_file(src: &str,dest: String)
{
    let mut y = dest.clone();
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


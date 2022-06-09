use crate::common;

use std::{
    fs,path::Path,
    io::{Read,Write}
};

use chrono::Local;
use clap::ArgMatches;



pub fn backup(matches : &ArgMatches) {
    println!("backup"); 
    let src = matches.value_of("src").unwrap();
    let abs_src = common::get_abs_path(src);

    let abs_base_path = common::base_absolute_path();
    // ~/.local/bcpがなければ作成
    match fs::metadata(abs_base_path.as_str()) {
        Err(_) => {
            // 無理ならパニック
            fs::create_dir_all(abs_base_path.as_str()).unwrap();
        },
        _ => {}
    }

    // 保存先が無ければ作成
    let (path_to_src,fine_name) = abs_src.to_str().unwrap().rsplit_once("/").unwrap();
    let repo = abs_base_path + path_to_src;
    let p = match Path::new(repo.clone().as_str()).canonicalize() {
        Err(_) => {
            // 無理ならパニック
            fs::create_dir_all(repo.as_str()).unwrap();
            Path::new(repo.as_str()).canonicalize().unwrap()
        },
        Ok(p) => {p}
    };

    // srcにサフィックスをつけて保存
    let suffix = {
        let date = Local::now().format("_%Y%m%d-%H%M%S").to_string();
        match matches.value_of("comment") {
            Some(c) => { date + "_" + c},
            None => { date + "_nocomment" } , 
        }
    };
    let saved_file = String::from(p.to_str().unwrap()) + "/" + fine_name + suffix.as_str() ;
    println!("saved_file=[{}]",saved_file);
    let mut f = std::fs::File::create(saved_file).unwrap();
    let src_file = std::fs::File::open(abs_src).unwrap();
    for buf in src_file.bytes() {
        f.write(&[buf.unwrap()]);
    }
}
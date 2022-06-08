use chrono::Local;
use clap::{Command,Arg, ArgMatches};
use dirs::home_dir;

use std::{fs::{self, }, path::Path, io::{Write, Read}};

fn main() {
    let matches = Command::new("back up copy")
        .version("0.0.1")
        .author("hal")
        .about("back up copy command")
        .subcommand(
            Command::new("restore")
                    .about("restore backup file.") // clapの問題で分けないと src subcommandになる。解決できるか不明？
                    .arg(
                        Arg::new("src")
                            .required(true)    
                            .help("specified restored backup file")
                    )
        )
        .subcommand(
            Command::new("backup")
                    .about("backup file")
                    .arg(
                        Arg::new("src")
                            .required(true)    
                            .help("specified backup file")
                    )
                    .arg(
                        Arg::new("comment")
                            .help("commnet to back up file")
                            .short('c')
                            .long("comment")
                            .takes_value(true)
                    )
        )   
        .get_matches();
    match matches.subcommand() {
        Some(("backup",s_matches)) => {backup(&s_matches)},
        Some(("restore",s_matches)) => {restore(&s_matches)},
        Some((_,_)) => {unreachable!("if specified no exsiting subcommand, error occurs by clap.")},
        None => {unreachable!("why?")},
    }
}

fn backup(matches : &ArgMatches) {
    println!("backup"); 
    let sa = matches.value_of("src").unwrap();
    let abs_src = match Path::new(sa).canonicalize() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{} does not exsist.",sa);
            // panic!("error: {}",e.kind()) // errorの種類を調べたいなら
            std::process::exit(120)
        }
    };

    let abs_base_path = base_absolute_path();
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

fn restore(matches : &ArgMatches) {
    println!("restore");
}

fn base_absolute_path() -> String {
    let home = home_dir().unwrap();
    let home = home.to_str().unwrap();
    String::from(home) + "/.local/bcp"
}

#[cfg(test)]
mod Test {

}
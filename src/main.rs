use clap::{Command,Arg, ArgMatches};
use std::{fs};

const BASE_DIR_PATH : &str = "~/.local/bcp";
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
        )   
        .get_matches();
    match matches.subcommand() {
        Some(("backup",s_matches)) => {backup(&s_matches)},
        Some(("restore",s_matches)) => {restore(&s_matches)},
        Some((_,_)) => {unreachable!("if specified no exsiting subcommand, error occurs by clap.")},
        None => {unreachable!("why?")},
    }
}

// 保存先の取得 or　作成
// 保存元
// 保存
fn backup(matches : &ArgMatches) {
    println!("backup"); 
    let src = matches.value_of("src");
    println!("{}",src.unwrap());
    let metadata = match fs::metadata(BASE_DIR_PATH) {
        Ok(metadeta) => {metadeta},
        Err(e) if e.raw_os_error().unwrap() == 2 => {
            println!("error = {}",e);
            println!("create dir");
            // 無理なら諦めてパニックしておく。
            fs::create_dir_all(BASE_DIR_PATH).unwrap();
            fs::metadata(BASE_DIR_PATH).unwrap()
        },
        Err(e) => {
            println!("error = {}",e);
            std::process::exit(121)
        }
    };
    assert!(metadata.is_dir());
}

fn restore(matches : &ArgMatches) {
    println!("restore");
}

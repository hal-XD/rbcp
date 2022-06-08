use clap::{Command,Arg, ArgMatches};

mod backup;
mod common;

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
        Some(("backup",s_matches)) => {backup::backup(&s_matches)},
        Some(("restore",s_matches)) => {restore(&s_matches)},
        Some((_,_)) => {unreachable!("if specified no exsiting subcommand, error occurs by clap.")},
        None => {unreachable!("why?")},
    }
}



fn restore(matches : &ArgMatches) {
    println!("restore");
}
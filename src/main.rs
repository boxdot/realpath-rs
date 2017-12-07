use std::fs;
use std::env;

fn main() {
    let mut args = env::args();
    args.next(); // skip command name
    match args.next() {
        Some(path) => {
            let path = fs::canonicalize(path).unwrap_or_else(|err| {
                println!("{}", err);
                ::std::process::exit(1);
            });
            let path_str = path.to_str().unwrap_or_else(|| {
                println!("Could not convert the path to string");
                ::std::process::exit(1);
            });
            println!("{}", path_str);
        }
        None => {
            println!("USAGE: realpath <PATH>");
            ::std::process::exit(1);
        }
    }
}

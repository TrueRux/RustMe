use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    /*
    dbg!(rs.err());
    dbg!(config);
    let contents =
    fs::read_to_string(&config.file_path).expect("Should have been able to read the file");
    println!("With text:\n {}", config.file_path);
    println!("With text:\n {}", config.query);
    let _ = run(config);
    */

    if let Err(e) = minigrep::run(config) {
        println!("Application error:{e}");
        process::exit(1);
    }
}

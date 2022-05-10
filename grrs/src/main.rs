#![allow(unused)]
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf
}


fn main() {
    let args = Cli::parse();

    // let content = std::fs::read_to_string(&args.path)
    //     .expect("could not read file");

    let resulut = std::fs::read_to_string("test.txt");
    let content = match resulut {
        Ok(content) => {content},
        Err(error) => {panic!("Cant't deal with {}, just exit here", error);}
    };
    println!("file content: {}", content);

    for line in content.lines() {
        if line.contains(&args.pattern){
            println!("{}", line);
            
        }
    }
}


// let pattern = std::env::args().nth(1).expect("no pattern given");
// let path = std::env::args().nth(2).expect("no path given");

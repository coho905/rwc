use std::fs::File;
//use std::io::prelude::*;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::env;

struct FileStatistics {
    file_path: String,
    word_count: usize,
    byte_count: u64,
    line_count: usize,
}

fn main()  -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("Missing filepaths. Please do rwc [filepaths]");
        return Ok(());
    }
    let mut total_stats = FileStatistics {
        file_path: "Total".to_string(),
        word_count: 0,
        byte_count: 0,
        line_count: 0,
    };
    for arg in &args[1..] {
        //println!("{arg}");
        let path = Path::new(arg.as_str());
        let metadata = std::fs::metadata(&path)?;
        let file_size = metadata.len();
        let file = match File::open(&path) {
            Err(why) => {
                println!("couldn't open file: {}", why);
                return Ok(()); // Or handle the error differently
            },
            Ok(file) => file,
        };
        //let mut file = File::open(&path);
        let mut file_stats = FileStatistics {
            file_path: arg.to_string(),
            word_count: 0,
            byte_count: file_size,
            line_count: 0,
        };
        let reader = BufReader::new(file);
        for line in reader.lines(){
            match line{
                Ok(line) => {
                    file_stats.line_count = file_stats.line_count+1;
                    //let bytes = line.len();
                    //file_stats.byte_count = file_stats.byte_count+bytes;
                    //let words = line.split_whitespace().collect::<Vec<_>>().len();
                    file_stats.word_count = file_stats.word_count + line.split_whitespace().count();
                }
                Err(err) => return Err(err),
            }
        }
        total_stats.line_count = total_stats.line_count + file_stats.line_count;
        total_stats.word_count = total_stats.word_count + file_stats.word_count;
        total_stats.byte_count = total_stats.byte_count + file_stats.byte_count;
        println!(
            "File: {}\nLines: {}   Words: {}   Bytes: {}",
            file_stats.file_path, file_stats.line_count, file_stats.word_count, file_stats.byte_count
        );
    }
    println!(
        "\nTotal\tLines: {}   Words: {}   Bytes: {}",
        total_stats.line_count, total_stats.word_count, total_stats.byte_count
    );
    Ok(())
    //println!("Hello, world!");
}

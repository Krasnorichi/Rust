use std::fs::File;
use std::io::{self,BufRead};

fn main(){
    if let Ok(file) = File::open("src/стихи.txt") {
        let reader = io::BufReader::new(file);

        for (line_num, line) in reader.lines().enumerate() {

            if let Ok(line_content) = line {
                if line_content.contains("стая") {
                    println!("Номер строки {}: {}", line_num + 1, line_content);
                }
            }
        }
    } else {
        println!("Файл не открывается")
    }
}


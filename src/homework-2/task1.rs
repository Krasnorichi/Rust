use std::fs::File;
use std::io::{self,BufRead};

fn find_word_file(file_name: &str, key_word: &str) -> i8 {
    if let Ok(file) = File::open(&file_name) {
        let reader = io::BufReader::new(file);

        for (line_num, line) in reader.lines().enumerate() {

            if let Ok(line_content) = line {
                if line_content.contains(&key_word) {
                    println!("Номер строки {}: {}", line_num + 1, line_content);
                    return 0;
                }
            }
        };
        println!("Слово не найдено");
        return 2;
    } else {
        println!("Файл не открывается");
        return 1;
    }
}

fn main() {
    let file_name: String = "src/homework-2/data/text.txt".to_string();
    let key_word: String = "agony".to_string();
    find_word_file(&file_name, &key_word); 
    
    
}

#[cfg(test)]
mod tests {
    use crate::find_word_file;
    #[test]
    fn no_file_assert() {
        let file_name: String = "src/homework-2/data/empty.txt".to_string();
        let key_word: String = "agony".to_string();
        assert_eq!(find_word_file(&file_name, &key_word), 1);
    }

    #[test]
    fn no_word_assert() {
        let file_name: String = "src/homework-2/data/text.txt".to_string();
        let key_word: String = "empty".to_string();
        assert_eq!(find_word_file(&file_name, &key_word), 2);
    }

    #[test]
    fn keyword_assert() {
        let file_name: String = "src/homework-2/data/text.txt".to_string();
        let key_word: String = "agony".to_string();
        assert_eq!(find_word_file(&file_name, &key_word), 0);
    }
}

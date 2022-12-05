use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String

        let mut result = 0;
        let mut elf1: String = "".to_string();
        let mut elf2: String = "".to_string();

        for line in lines {
        if let Ok(ip) = line {
            let first_half = false;
            if first_half {

                let (mut first, mut second) = ip.split_at(ip.len()/2);
                let mut chars = first.chars();

                while let Some(c) = chars.next() {
                    if second.contains(c) {
                        result += char_to_val(c);
                        break;
                    }
                }
            } else {
                if elf1 == "" {
                    elf1 = ip;
                } else if elf2 == "" {
                    elf2 = ip;
                } else {

                    // Check for every char in elf 3 if its contained in the other elfs
                    let mut chars = ip.chars();
                    while let Some(c) = chars.next() {
                        if elf1.contains(c) && elf2.contains(c) {
                            result += char_to_val(c);
                            break; // important due to badges that may occur multiple times.
                        }
                    }
                    
                    elf1 = "".to_string();
                    elf2 = "".to_string();
                }
            }
        }
        }
        println!("{}", result);
    }
}

fn char_to_val(c: char) -> i32 {
    let ascii_val = c as i32;
        if ascii_val <= 90 && ascii_val >= 65 { // upper case
            return ascii_val - 38; // because math
        }
        if ascii_val <= 122 && ascii_val >= 97 { // lower case
            return ascii_val - 96; // because again math
        }
        return -1;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {


    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {

        let mut elf = 0;
        let mut calories = 0;
        let mut c2 = 0;
        let mut c3 = 0;
        let mut current_elf = 0;
        let mut columative = 0;
        println!("test");
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    if columative > calories {
                        elf = current_elf;
                        c3 = c2;
                        c2 = calories;
                        calories = columative;
                    }
                    else {
                    if columative > c2 {
                        c3 = c2;
                        c2 = columative;
                    }
                    else{
                    if columative > c3 {
                        c3 = columative;
                    }}}
                    current_elf += 1;
                    columative = 0;
                } else {
                    columative += ip.parse::<i32>().unwrap();
                } 
            }
        }
        println!("{}, {}, {}, total: {}", calories, c2, c3, calories + c2 + c3);
    }


}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

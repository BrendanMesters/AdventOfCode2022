use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String

        let mut result = 0;

        for line in lines {
            if let Ok(ip) = line {
                let mut iterator = ip.split(",");
                if let Some(first) = iterator.next() {
                if let Some(second) = iterator.next() {
                    println!("{} ; {}", first, second);
                    let mut it1 = first.split("-");
                    let mut it2 = second.split("-");
                    if let Some(vs1) = it1.next() {
                    if let Some(vs2) = it2.next() {
                    if let Some(ve1) = it1.next() {
                    if let Some(ve2) = it2.next() {
                        println!("{}, {}, {}, {}", vs1, ve1, vs2, ve2);
                        let to_int = |v: &str| v.parse::<i32>().unwrap();
                        let part_one = false;

                        if part_one {
                            if to_int(vs1) >= to_int(vs2) && to_int(ve1) <= to_int(ve2) {
                                println!("came to 1");
                                result += 1;
                            } else if to_int(vs1) <= to_int(vs2) && to_int(ve1) >= to_int(ve2) {
                                println!("came to 2");
                                result += 1;
                            }
                        } else {
                            if to_int(vs1) >= to_int(vs2) && to_int(vs1) <= to_int(ve2) {
                                println!("came to 1");
                                result += 1;
                            } else if to_int(ve1) >= to_int(vs2) && to_int(ve1) <= to_int(ve2) {
                                println!("came to 2");
                                result += 1;
                            } else if to_int(vs2) >= to_int(vs1) && to_int(vs2) <= to_int(ve1) {
                                result += 1;
                            } else if to_int(ve2) >= to_int(vs1) && to_int(ve2) <= to_int(ve1) {
                                result += 1;
                            }
                        }
                        println!("result = {}", result);
                    }
                    }   
                    }
                    }
                    //if let Some(parse::<i32>().unwrap())    
                } 
                } 
            }
        }
        println!("{}", result);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
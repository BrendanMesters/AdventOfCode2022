use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut vals = ['-','-','-','-'];
        let mut som = ['-','-','-','-','-','-','-','-','-','-','-','-','-','-'];
        let mut counter = 0;
        let mut message_counter = 0;
        let mut hit_four = false;
        let mut any_problems = false;
        let mut total_count = 0;
        let mut looking_for_message = false;

        for line in lines {
            if let Ok(ip) = line {
                //println!("{}", ip);
                for c in ip.chars() {

                    if message_counter == 14 {
                        message_counter = 0;
                    }
                    if counter == 4 {
                        counter = 0;
                        hit_four = true;
                    }

                    vals[counter] = c;
                    som[message_counter] = c;

                    if hit_four && !looking_for_message {
                        any_problems = false;
                        for i in 0..4 {
                            for j in (i+1)..4 {
                                if vals[i] == vals[j] {
                                    any_problems = true;
                                }
                            }
                        }
                        if !any_problems {
                            println!("package start at {}", total_count + 1);
                            looking_for_message = true;
                        }
                    }
                    if looking_for_message {
                        //println!("test");
                        any_problems = false;
                        for i in 0..14 {
                            for j in (i+1)..14 {
                                if som[i] == som[j] {
                                    //println!("{}, {}, {}, {}", i, j, som[i], som[j]);
                                    any_problems = true;
                                }
                            }
                        }
                        if !any_problems {
                            println!("message start at {}", total_count + 1);
                            looking_for_message = true;
                            break;
                        }
 
                    }
                    total_count += 1;
                    counter += 1;
                    message_counter += 1;
                    
                }
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


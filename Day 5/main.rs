use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::any::type_name;


fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String

        let mut find_boxes = true;
        let mut early_lines: Vec<String> = vec![];
        let mut stacks: [Vec<char>; 9] = [vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![]];
        let to_int = |v: &str| v.parse::<u32>().unwrap();
        let first_part = false;

        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {continue;} 
                if find_boxes {
                    if ip == " 1   2   3   4   5   6   7   8   9 " {
                       stacks = read_boxes(early_lines);
                       early_lines = vec![];
                        println!("here ");
                       find_boxes = false; // boxes have been found.
                    } else {
                        early_lines.push(ip);
                    }
                } else {
                    let string_seperated_by_spaces = ip.replace("move ", "").replace("from ", "").replace("to ", "");
                    let mut iter = string_seperated_by_spaces.split_whitespace();
                    if let Some(amount) = iter.next() {
                    if let Some(from) = iter.next() {
                    if let Some(to) = iter.next() {
                        let (amount_num, from_num, to_num) = (to_int(amount), to_int(from)-1, to_int(to)-1);
                        if first_part {
                            for i in 0..amount_num {
                                println!("{}", i);
                                let mut val = stacks[from_num as usize].pop().unwrap();
                                stacks[to_num as usize].push(val);
                            }
                        } else {
                            let mut temp = vec![];
                            for i in 0..amount_num {
                                let mut val = stacks[from_num as usize].pop().unwrap();
                                temp.push(val);
                            } 
                            for i in 0..amount_num {
                                let mut val = temp.pop().unwrap();
                                stacks[to_num as usize].push(val);
                            } 

                        }
                        
                    
                    }
                    }
                    }

                }
            }
        }

        for mut st in stacks {
            if let Some(mut val) = st.pop() {
                println!("{}", val);
            }
        }
    }
}


fn read_boxes(lines: Vec<String>) -> [Vec<char>; 9] {
    let mut result: [Vec<char>; 9] = [vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![]];
    
    let mut i = 0;

    //lines.reverse();
    for line in lines {
        let mut lon = line.chars();
        i = 0;
        loop {
            lon.next();
            if let Some(c) = lon.next() {
                lon.next();
                lon.next();
                println!("{}",c);
                if !(c == ' ') {
                    result[i].push(c);
                }
                i += 1;
            } else {
                break;
            }
        }
    }
     
    let mut actual_result: [Vec<char>; 9] = [vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![]];
    for i in 0..9 {
        while !result[i].is_empty() {
            actual_result[i].push(result[i].pop().unwrap());
        }
    }
    
    /*
    println!("--------------");
    for mut i in actual_result {
        let mut test = i.pop().unwrap();
        println!("{}", test);
    }
    */
    return actual_result;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


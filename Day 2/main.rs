use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


// a ; x = rock
// b ; y = paper
// c ; z = scisors
fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut result = 0;
        for line in lines {
            if let Ok(ip) = line {
                let mut temp = ip.split_whitespace();
                if let Some(opponent) = temp.next() {
                if let Some(me) = temp.next() {
                    println!("{}, {}, {}", ip, opponent, me);
                    let part_one = false;
                    let x_index = ["A", "B", "C"].iter().position(|&x| x == opponent).unwrap();
                    let y_index = ["X", "Y", "Z"].iter().position(|&x| x == me).unwrap();
                    if part_one {
                        result += [[4, 8, 3], [1, 5, 9], [7, 2, 6]][x_index][y_index];
                       /* 
                       // My origional solution
                        if me == "X" {
                            result += 1;
                            if opponent == "A" {result += 3;}
                            if opponent == "C" {result += 6;}
                        }
                        if me == "Y" {
                            result += 2;
                            if opponent == "B" {result += 3;}
                            if opponent == "A" {result += 6;}
                        }
                        if me == "Z" {
                            result += 3;
                            if opponent == "C" {result += 3;}
                            if opponent == "B" {result += 6;}
                        }
                       */ 
                    } else {
                        result += [[3, 4, 8], [1, 5, 9], [2, 6, 7]][x_index][y_index];
                       /* 
                        // My origional solution
                        if me == "X" {
                            result += 0;
                            if opponent == "A" {result += 3;}
                            if opponent == "B" {result += 1;}
                            if opponent == "C" {result += 2;}
                        }                        
                        if me == "Y" {
                            result += 3;
                            if opponent == "A" {result += 1;}
                            if opponent == "B" {result += 2;}
                            if opponent == "C" {result += 3;}
                        }  
                        if me == "Z" {
                            result += 6;
                            if opponent == "A" {result += 2;}
                            if opponent == "B" {result += 3;}
                            if opponent == "C" {result += 1;}
                        } 
                       */ 
                    }
                }} 
            }
            println!("{}", result)
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

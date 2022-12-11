use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::collections::{linked_list, LinkedList};



const AT_MOST_SIZE: i32 = 100_000;
const TOTAL_SPACE: i32 = 70_000_000;
const REQUIRED_SPACE: i32 = 30_000_000;

fn main() {
    let mut file: LinkedList::<String> = LinkedList::from([]);

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines{
            if let Ok(mut ip) = line {
                file.push_back(ip);
            }
        }
   } 

   let partone = false;

    if partone {
        let (test, answer, _) = file_system_walker1(file);
        
        println!("Part 1: total size of `/`: {}, the answer: {}", test, answer);
    } else {
        let (total_used, mut cumul_list, _) = file_system_walker2(file, String::from("/"));

        let space_to_be_deleted = REQUIRED_SPACE - (TOTAL_SPACE - total_used);

        let (mut smallest_viable, mut name) = (TOTAL_SPACE, String::from("error"));
        while !cumul_list.is_empty() {
            if let Some((s, n)) = cumul_list.pop_back() {
                if s >= space_to_be_deleted && s <= smallest_viable {
                    smallest_viable = s;
                    name = n;
                }
            }
        }
        println!("The smallest viable directory is {} with size of {}", name, smallest_viable);

    }


}

/*
This function walks over the file, recursively calling itself to calculate
the size of directories.

@input: file, a linked list holding the strings representing the input file
@output: a triple tuple of the size of the directory that this itteration
represents, a cumulative size of all directories at most 10_000 in size and
the rest of the file object which was the input

Note, the current dir size gets added to cumul before return, not uppon comming
back to the calling function.
*/
fn file_system_walker1(mut file: LinkedList::<String>) -> (i32, i32, LinkedList::<String>) {

    let cd = Regex::new(r"\$ cd *").unwrap();
    let ls = Regex::new(r"\$ ls *").unwrap();

    let mut dir_size: i32 = 0;
    let mut cumul_size: i32 = 0;

    while !file.is_empty() {
        if let Some(line) = file.pop_front(){
            if cd.is_match(line.as_str()) {
                let (_, cmd) = line.split_at(5); 
                

                // either it wants to go up one dir, or it wants to go down one dir.
                // The case where it wants to go to the root dit (`/`) only happens as cmd #1 
                if cmd == ".." {
                    if dir_size <= AT_MOST_SIZE {
                        cumul_size += dir_size;
                    }
                    return (dir_size, cumul_size, file);
                } else {
                    let (new_cumul, new_dir): (i32, i32);
                    (new_dir, new_cumul, file) = file_system_walker1(file);

                    dir_size += new_dir;
                    cumul_size += new_cumul;
                }
                
            } else if ls.is_match(line.as_str()) {
                // This case can basically be skipped
            } else {
                // Now we're in the case where we're reading files and their sizes.
                if let Some(size) = line.split_ascii_whitespace().next() {
                    if let Ok(i) = size.parse::<i32>() { 
                        dir_size += i;
                    }
                }
            }
        }
        



    }
    return (dir_size, cumul_size, file);

    //return (-1, -1, LinkedList::from([String::from("error")]));
}


fn file_system_walker2(mut file: LinkedList::<String>, name: String) -> (i32, LinkedList::<(i32, String)>, LinkedList::<String>) {



    let cd = Regex::new(r"\$ cd *").unwrap();
    let ls = Regex::new(r"\$ ls *").unwrap();

    let mut dir_size: i32 = 0;
    let mut cumul_list: LinkedList::<(i32, String)> = LinkedList::from([]);

    while !file.is_empty() {
        if let Some(line) = file.pop_front(){
            if cd.is_match(line.as_str()) {
                let (_, cmd) = line.split_at(5); 
                

                // either it wants to go up one dir, or it wants to go down one dir.
                // The case where it wants to go to the root dit (`/`) only happens as cmd #1 
                if cmd == ".." {
                    cumul_list.push_back((dir_size, name));
                    return (dir_size, cumul_list, file);
                } else {
                    let (mut new_cumul, new_dir): (LinkedList::<(i32, String)>, i32);
                    (new_dir, new_cumul, file) = file_system_walker2(file, cmd.to_string());

                    dir_size += new_dir;
                    cumul_list.append(&mut new_cumul);
                }
                
            } else if ls.is_match(line.as_str()) {
                // This case can basically be skipped
            } else {
                // Now we're in the case where we're reading files and their sizes.
                if let Some(size) = line.split_ascii_whitespace().next() {
                    if let Ok(i) = size.parse::<i32>() { 
                        dir_size += i;
                    }
                }
            }
        }
        



    }
    return (dir_size, cumul_list, file);

    //return (-1, -1, LinkedList::from([String::from("error")]));
}




// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



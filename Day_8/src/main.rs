use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn main() {
    // file[y][x];
    let mut file: Vec::<Vec::<i32>> = Vec::from([]);

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines{
            if let Ok(ip) = line {
                if ip.is_empty() {continue;}
                let mut line_list: Vec::<i32> = Vec::from([]);
                for c in ip.as_str().chars() {
                    line_list.push((c as i32) - 48);
                }
                file.push(line_list);
            }
        }
    } 

    let width: usize = file[0 as usize].len();
    let height: usize = file.len();

    let mut visible_count = 0;
    let mut scene_score = 0;

    // part 1
    for h in 0..height {
        'outer: for w in 0..width {
            let tree = file[h][w];
            if h == 0 || w == 0 || h == height -1 || w == width -1 { // Literal edge case
                visible_count += 1;
                continue 'outer;
            }
            for y in (0..h).rev() {
                if file[y][w] >= tree { // Ocluded from top
                    break;
                }
                if y == 0 { // Seen from top
                    visible_count += 1;
                    continue 'outer;
                }
            }
            for y in (h + 1)..height {
                if file[y][w] >= tree { // Ocluded from bot
                    break;
                }
                if y == height-1 { // Seen from bot
                    visible_count += 1;
                    continue 'outer;
                }
            }
            for x in (0..w).rev() {
                if file[h][x] >= tree { // Ocluded from left
                    break;
                }
                if x == 0 { // Seen from left
                    visible_count += 1;
                    continue 'outer;
                }
            }
            for x in (w + 1)..width {
                if file[h][x] >= tree { // Ocluded from right
                    break;
                }
                if x == width-1 { // Seen from right
                    visible_count += 1;
                    continue 'outer;
                }
            }
            println!("y: {}, x: {} with height {} fully occluded", h, w, tree);
            //visible_count += 1;
        }
    }
 
    //return;
    for h in 0..height {
        'outer: for w in 0..width {
            let tree = file[h][w];
            let mut acum_scene_score = 1;

            'top: for y in (0..h).rev() {
                if file[y][w] >= tree { // Ocluded from top
                    acum_scene_score *= h-y;
                    break 'top;
                }
                if y == 0 { // Seen from top
                    acum_scene_score *= h-y;
                    break 'top;
                }
            }
            'bot: for y in (h + 1)..height {
                if file[y][w] >= tree { // Ocluded from bot
                    acum_scene_score *= (y - h);
                    break 'bot;
                }
                if y == height-1 { // Seen from bot
                    acum_scene_score *= (y - h);
                    break 'bot;
                }
            }
            'left: for x in (0..w).rev() {
                if file[h][x] >= tree { // Ocluded from left
                    acum_scene_score *= w - x;
                    break 'left;
                }
                if x == 0 { // Seen from left
                    acum_scene_score *= w - x;
                    break 'left;
                }
            }
            'right: for x in (w + 1)..width {
                if file[h][x] >= tree { // Ocluded from right
                    acum_scene_score *= (x - w);
                    break 'right;
                }
                if x == width-1 { // Seen from right
                    acum_scene_score *= (x - w);
                    break 'right;
                }
            }
            if scene_score < acum_scene_score {
                scene_score = acum_scene_score;
            }
        }
    }
    println!("random info {} {}", width, height);
    println!("visibility = {}", visible_count);
    println!("scene score = {}", scene_score);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufReader;
use std::iter::FromIterator;


fn main() {
    // Create a path to the desired file
    let path = Path::new("input");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
   let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let readable_file = BufReader::new(&file);
    let mut p1_valid_count = 0;
    let mut p1_invalid_count = 0;
    let mut p2_valid_count = 0;
    let mut p2_invalid_count = 0;
    for line in readable_file.lines() {
        let l = String::from(line.unwrap());
        let str_vec: Vec<char> = l.chars().collect();
        let dash_position = str_vec.iter().position(|&x| x == '-').unwrap();
        let break_position = str_vec.iter().position(|&x| x == ' ').unwrap();
        let min_bound = String::from_iter(&str_vec[0..dash_position]).parse::<i32>().unwrap();
        let max_bound = String::from_iter(&str_vec[dash_position+1..break_position]).parse::<i32>().unwrap();
        let char_to_find = &str_vec[break_position+1];
        // Problem 1
        let count_of_char = str_vec[break_position+4..].iter().filter(|&x| x == char_to_find).count() as i32;
       if min_bound <= count_of_char && max_bound >= count_of_char {
            p1_valid_count += 1;
       } else {
            p1_invalid_count += 1;
       }
       //problem 2
       let new_arr = &str_vec[break_position+4..];
       let first_pos = &new_arr[(min_bound-1) as usize];
       let second_pos = &new_arr[(max_bound-1) as usize];
       let char_count = vec![first_pos,second_pos].iter().filter(|&x| *x == char_to_find).count() as i32;
        if char_count == 1 {
            p2_valid_count += 1;
        } else {
            p2_invalid_count += 1;
        }


    }
    println!("Valid count! {}", p1_valid_count);
    println!("Invalid count :( {}", p1_invalid_count);

    println!("Valid count 2! {}", p2_valid_count);
    println!("Invalid count 2:( {}", p2_invalid_count);
}
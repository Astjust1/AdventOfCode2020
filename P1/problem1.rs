use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;

fn main() {
    // Create a path to the desired file
    let path = Path::new("p1input");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    }
    const IMPORTANT_NUMBER:i32 = 2020;

    let x: HashSet<i32> = s.lines().map(|a| a.parse::<i32>().unwrap()).collect();

    let y: HashSet<i32> = x.iter().map(&|a| (IMPORTANT_NUMBER - a)).collect();

    let first_set: HashSet<_> = x.intersection(&y).cloned().collect();
  
    let first_answer: i32 = first_set.iter().cloned().product();

    println!("{:#?}",first_answer);

    // Part 2 since I cant do matrix subtraction in rust without a library it seems :(
    let mut second_set = HashSet::new();

    for unmodded in x.iter() {
        for subbed in y.iter() {
            second_set.insert(subbed - unmodded);
        }
    }

    let final_intersection: HashSet<_> = x.intersection(&second_set).cloned().collect();

    //println!("{:#?}", final_intersection);

    let final_answer: i32 = final_intersection.iter().cloned().product();
    println!("{}",final_answer);
}
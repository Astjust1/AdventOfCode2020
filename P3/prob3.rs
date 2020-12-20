use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;

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
    let mut forest: Vec<Vec<char>> = Vec::new();
    
    for line in readable_file.lines() {
        let l = String::from(line.unwrap());
        let str_vec: Vec<char> = l.chars().collect();
        forest.push(str_vec);
        println!("{}", l);
    }
    let mut count_vec:Vec<i64> = Vec::new();
    count_vec.push(count_trees(1, 1, &forest));
    count_vec.push(count_trees(1, 3, &forest));
    count_vec.push(count_trees(1, 5, &forest));
    count_vec.push(count_trees(1, 7, &forest));
    count_vec.push(count_trees(2, 1, &forest));
    println!("{:#?}",count_vec);
    let final_answer:i64 = count_vec.iter().product() ;
    println!("Final answer:{}", final_answer);

}

fn count_trees(depth_step:i32 , width_step:i32, forest:&Vec<Vec<char>>) -> i64{
    let mut x_position: i32 = 0;
    let mut y_position: i32 = 0;
    let mut num_trees: i64 = 0;
    const tree_char: char = '#';
    let depth = forest.len() as i32;
    let width = forest[0].len() as i32 -1;
    while (y_position < depth) {
       let row = &forest[y_position as usize];
        if row[x_position as usize] == tree_char {
            num_trees += 1;
        }
        x_position += width_step;
        if  x_position > width {
            x_position = x_position - width - 1;
            
        }
        y_position += depth_step;
    }
    return num_trees;
}
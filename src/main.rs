// use std::env;
use std::io::prelude::*;
use std::fs::File;
// use std::io::BufReader;
use std::fs::read_to_string;
use std::io::{self, BufReader};
use std::path::Path;

fn string_to_int(num: String) -> i32 {

    let new_line = num.trim();
    let mut return_val: i32 = 0;
    match new_line.parse::<i32>() {
        Ok(n) => return_val = n,
        Err(e) => println!("error: {}", e),
    }
    return_val
}

// naive way
// fn read_lines(filename: &str) -> Vec<String> {
//     let mut result = Vec::new();

//     for line in read_to_string(filename).unwrap().lines() {
//         result.push(line.to_string());
//     }
//     result
// }

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[allow(dead_code)]
fn parse_file(file_path: &str) -> std::io::Result<()> {
    let mut m: i32;
    let mut n: i32;
    let mut p: i32;

    let f = File::open(file_path)?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut len = reader.read_line(&mut line)?;
    // m = reader.read_line(&mut line)?;
    // n = reader.read_line(&mut line)?;
    // p = reader.read_line(&mut line)?;

    m = string_to_int(line.to_string());
    len = reader.read_line(&mut line)?;
    n = string_to_int(line.to_string());
    len = reader.read_line(&mut line)?;
    p = string_to_int(line.to_string());

    println!("{} {} {}", m, n, p);

    Ok(())
}


fn main() -> std::io::Result<()> {
    // let mut mat1 = vec<i16>;
    let mut mat1 = vec![0i32; 5 * 2];
    println!("array is {:?}", mat1);
    println!("vector is this long: {}", mat1.len());

    let file_path = "src/test1.txt";

    let f = File::open(file_path)?;
    let mut reader = BufReader::new(f);
    println!("in file {}", file_path);
    let mut line = String::new();
    let mut len = reader.read_line(&mut line)?;
    println!("first line is {} bytes long", len);
    println!("{}", line);
    // len = reader.read_line(&mut line)?;
    // let new_line = line.to_string();
    // let temp = new_line.trim();

    // match temp.parse::<i32>() {
    //     Ok(n) => println!("number: {}", n),
    //     Err(e) => println!("error: {}", e),
    // }
    // let stringnum = "32";
    // let new_num = string_to_int(stringnum.to_string());
    // println!("{}", new_num);

    parse_file(file_path);

    if let Ok(lines) = read_lines(file_path) {
        // let mut index: i32 = 1;
        for line in lines {
            if let Ok(ip) = line {
                println!("{}: {}", index, ip);
                // index += 1;
            }
        }
    }

    Ok(())
}

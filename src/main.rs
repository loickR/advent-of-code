use std::{fs::File, io::Read};

fn main() {
    check_puzzle();
}

fn check_puzzle() {
    let mut file = match File::open("puzzle.txt") {
        Ok(f) => f,
        Err(e) => panic!("could not open file : {e}")
    };
    
    let mut contents = String::new();
    let _result = match file.read_to_string(&mut contents) {
        Ok(_) => println!("file read"),
        Err(e) => println!("{e}")
    };
}

fn _check_list_1() {
    let mut file = match File::open("liste.txt") {
        Ok(f) => f,
        Err(e) => panic!("could not open file : {e}")
    };
    
    let mut contents = String::new();
    let _result = match file.read_to_string(&mut contents) {
        Ok(_) => println!("file read"),
        Err(e) => println!("{e}")
    };

    let iter = contents.split_ascii_whitespace();
    let mut first_list : Vec<i32> = Vec::new();
    let mut second_list : Vec<i32> = Vec::new();

    for (i, val_str) in iter.enumerate() {
        let val = match val_str.parse::<i32>() {
            Ok(r) => r,
            Err(e) => panic!("not a number : {e}")
        };
        
        if i%2 == 0 {
            first_list.push(val);
        } else {
            second_list.push(val);
        }
    }
    
    assert_eq!(first_list.len(), second_list.len());
    
    let mut result = 0;
    for (i, _v) in first_list.iter().enumerate() {
        let a = first_list[i];
        let b: i32 = second_list[i];
        let res: i32 = (a - b).abs();
        println!("{a} - {b} = {res}");
        result = result + res;
    }

    println!("result = {result}")
}

fn _check_list_2() {
    let mut file = match File::open("liste.txt") {
        Ok(f) => f,
        Err(e) => panic!("could not open file : {e}")
    };
    
    let mut contents = String::new();
    let _result = match file.read_to_string(&mut contents) {
        Ok(_) => println!("file read"),
        Err(e) => println!("{e}")
    };

    let iter = contents.split_ascii_whitespace();
    let mut first_list : Vec<i32> = Vec::new();
    let mut second_list : Vec<i32> = Vec::new();

    for (i, val_str) in iter.enumerate() {
        let val = match val_str.parse::<i32>() {
            Ok(r) => r,
            Err(e) => panic!("not a number : {e}")
        };
        
        if i%2 == 0 {
            first_list.push(val);
        } else {
            second_list.push(val);
        }
    }
    
    assert_eq!(first_list.len(), second_list.len());
    
    let mut result = 0;
    for (i, _v) in first_list.iter().enumerate() {
        let a = first_list[i];
        let count: i32 = second_list.iter().filter(|x| **x == a).count() as i32;
        let res: i32 = a * count as i32;
        println!("{a} * {count} = {res}");

        result = result + res;
    }

    println!("result = {result}")
}


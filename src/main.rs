use std::{fs::File, io::Read};

fn main() {
    check_puzzle_day2_part1();
}

fn _check_puzzle_day2_part2() {
}

fn check_puzzle_day2_part1() {
    let mut file = match File::open("puzzle.txt") {
        Ok(f) => f,
        Err(e) => panic!("could not open file : {e}")
    };
    
    let mut contents = String::new();
    let _result = match file.read_to_string(&mut contents) {
        Ok(_) => println!("file read"),
        Err(e) => println!("{e}")
    };

    let res1 = contents.lines()
            .map(|l| l.split_ascii_whitespace())
            .enumerate();

    let mut v1 : Vec<i64> = Vec::new();
    let mut safe_report_count = 0;
    for (i, val) in res1 {
        print!("Ligne {i} - ");
        for (_j, v) in val.enumerate() {
            let a = v.parse::<i64>().unwrap();
            v1.push(a);
        }

        let mut is_safe = true;
        for (k, _value) in v1.iter().enumerate() {

            let np1 = k + 1;
            let nm1 = k as i32 - 1;
            if np1 >= v1.len() {
                break;
            } else if nm1 < 0 {
                continue;
            }

            let a = v1[k - 1];
            let b = v1[k];
            let c = v1[np1];
            let res_ab = (a - b).abs();
            let res_bc = (b - c).abs();

            is_safe = is_safe & (res_ab.abs() >= 1 && res_ab.abs() <= 3 && res_bc.abs() <= 3 && res_bc.abs() >= 1) && (res_ab.abs() != 0) && (res_bc != 0);

            if a < b && b > c {
                is_safe = false;
            } else if a > b && b < c {
                is_safe = false;
            }
        }

        if is_safe {
            println!("Safe");
            safe_report_count = safe_report_count + 1;
        } else {
            println!("Unsafe")
        }

        v1.clear();
    }

    println!("{safe_report_count} reports are safe");
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


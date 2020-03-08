#![feature(duration_as_u128)]
extern crate rand;

use std::{env, io};
use std::fs::File;
use std::io::Write;
use std::ops::Add;
use std::time::Instant;
use rand::*;
use std::str::SplitWhitespace;
use list2::*;
use list2::stat::Stat;

fn u32_insert_sort(mut a: &mut Vec<u32>, b: fn(&u32, &u32) -> bool) -> Stat {
    insert_sort(&mut a, b)
}

fn u32_merge_sort(mut a: &mut Vec<u32>, b: fn(&u32, &u32) -> bool) -> Stat {
    merge_sort(&mut a, b)
}

fn u32_quick_sort(mut a: &mut Vec<u32>, b: fn(&u32, &u32) -> bool) -> Stat {
    quick_sort(&mut a, b)
}

fn u32_double_sort(mut a: &mut Vec<u32>, b: fn(&u32, &u32) -> bool) -> Stat {
    double_sort(&mut a, b)
}

fn u32_mix_sort(mut a: &mut Vec<u32>, b: fn(&u32, &u32) -> bool) -> Stat {
    mix_sort(&mut a, b)
}

fn test_function(function: fn(& mut Vec<u32>, fn(&u32, &u32) -> bool) -> Stat,
                 ord: fn(&u32, &u32) -> bool, n: i32) -> (i32, i32, u128)
{
    let mut data: Vec<u32> = Vec::new();
    let mut rng = rand::thread_rng();
    for i in 0..n {
        data.push(rng.gen());
    }
    let now = Instant::now();
    let stat = function(&mut data, ord);
    let time = now.elapsed().as_nanos();
    (stat.comp, stat.swap, time)
}

fn test_and_save(function: fn(& mut Vec<u32>, fn(&u32, &u32) -> bool) -> Stat,
                 ord: fn(&u32, &u32) -> bool, mut file: &File) {
    for i in 1..=100 {
        let (c, s, t) = test_function(function, ord, i * 100);
        let to_write = format!("{};{};{};{}\n", i * 100, c, s, t);
        file.write(to_write.as_ref());
    }
}

fn get_order_function(s : &str) -> (fn(&u32, &u32) -> bool){
     match s {
        ">=" => {
            return |&x, &y| x >= y;
        }
        "<=" => {
            return |&x, &y| x <= y;
        }
        _ => panic!("Unknown order type"),
    }
}

fn get_line() -> Vec<u32>{
    let mut res: Vec<u32> = vec![];
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    for s in input_text.trim().split_whitespace(){
        match s.parse::<u32>() {
            Ok(number) => {
                res.push(number);
            }
            Err(..) => panic!("Is not a number {}", s),
        };
    }
    res
}

fn find_in_vec<A>(v: &Vec<A>, to_find: A) -> Option<usize>
    where A: PartialEq
{
    for i in 0..v.len()
    {
        if v[i] == to_find{
            return Some(i);
        }
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        5 => {
            let s_arg_pos = find_in_vec(&args, "--sort".parse().unwrap()).expect("none --sort param");
            let o_arg_pos = find_in_vec(&args, "--ord".parse().unwrap()).expect("none --ord param");
            let mut input = get_line();
            let mut array: Vec<u32> = Vec::new();
            let n: u32 = input[0];
            input.remove(0);
            let mut i = 0;
            loop {
                for x in input.iter(){
                    array.push(x.clone());
                    i += 1;
                }
                if i < n {
                    input = get_line();
                } else {
                    break;
                }
            }
            let ord: fn(&u32, &u32) -> bool = get_order_function(args[o_arg_pos+1].as_str());
            println!("Sorting");
            match args[s_arg_pos+1].as_str() {
                "merge" => {
                    merge_sort(&mut array, ord);
                }
                "insert" => {
                    insert_sort(&mut array, ord);
                }
                "quick" => {
                    quick_sort(&mut array, ord);
                }
                "double" => {
                    double_sort(&mut array, ord);
                }
                "mix" => {
                    mix_sort(&mut array, ord);
                }
                _ => panic!("Unknown sort type"),
            }
            println!("{:?}", array);
        }
        8 => {
            let s_arg_pos = find_in_vec(&args, "--sort".parse().unwrap()).expect("none --sort param");
            let o_arg_pos = find_in_vec(&args, "--ord".parse().unwrap()).expect("none --ord param");
            let st_arg_pos = find_in_vec(&args, "--stat".parse().unwrap()).expect("none --stat param");
            let mut file = File::create(args[st_arg_pos+1].as_str()).unwrap();
            let k = args[st_arg_pos+2].as_str().parse::<u32>().unwrap();
            let ord: fn(&u32, &u32) -> bool = get_order_function(args[o_arg_pos+1].as_str());
            println!("Creating stats");
            match args[s_arg_pos+1].as_str() {
                "merge" => {
                    for i in 0..k {
                        test_and_save(u32_merge_sort, ord, &mut file);
                    }
                }
                "insert" => {
                    for i in 0..k {
                        test_and_save(u32_insert_sort, ord, &mut file);
                    }
                }
                "quick" => {
                    for i in 0..k {
                        test_and_save(u32_quick_sort, ord, &mut file);
                    }
                }
                "double" => {
                    for i in 0..k {
                        test_and_save(u32_double_sort, ord, &mut file);
                    }
                }
                "mix" => {
                    for i in 0..k {
                        test_and_save(u32_mix_sort, ord, &mut file);
                    }
                }
                _ => panic!("Unknown sort type"),
            }
        }
        _ => println!("Wrong number of arguments"),
    }
}

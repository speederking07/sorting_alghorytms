extern crate rand;

use std::{env, io};
use std::fs::File;
use std::io::Write;
use std::time::Instant;

use rand::*;

use list2::double_sort::double_sort;
use list2::insert_sort::insert_sort;
use list2::merge_sort::merge_sort;
use list2::mix_sort::mix_sort;
use list2::quick_sort::quick_sort;
use list2::stat::Stat;
use list2::radix_sort::radix_sort;
use std::borrow::Borrow;
use core::mem;
use list2::improved_quick_sort::improved_quick_sort;

//Funkcje mojej chańby (nie ograrnołem do końca typów w Rust'ie)
fn u32_improved_quick_sort(mut a: &mut Vec<u32>, b: fn(&u32, &u32) -> bool) -> Stat {
    improved_quick_sort(&mut a, b)
}

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

fn u32_radix_sort(mut a: &mut Vec<u32>, b: fn(&u32, &u32) -> bool) -> Stat {
    radix_sort(&mut a, b((1 as u32).borrow(), (2 as u32).borrow()))
}

//Przeprowadza pojedyńczy test na n tablicy i zwraca wynik
fn test_function(function: fn(&mut Vec<u32>, fn(&u32, &u32) -> bool) -> Stat,
                 ord: fn(&u32, &u32) -> bool, n: i32) -> (u128, u128, u128, u128)
{
    let mut data: Vec<u32> = Vec::new();
    let mut rng = rand::thread_rng();
    for _i in 0..n {
        data.push(rng.gen());
    }
    let now = Instant::now();
    let stat = function(&mut data, ord);
    let time = now.elapsed().as_millis();
    //eprintln!("{}", stat.mem);
    (stat.comp as u128, stat.swap as u128, stat.mem as u128, time)
}

//Przeprowaadza cykl testów i zapisuje wyniki do pliku
fn test_and_save(function: fn(&mut Vec<u32>, fn(&u32, &u32) -> bool) -> Stat,
                 ord: fn(&u32, &u32) -> bool, k: u32, mut file: &File) {
    let mut i = 10;
    let mut five = true;
    while i <= 100000 {
        let mut sum_c = 0;
        let mut sum_s = 0;
        let mut sum_m = 0;
        let mut sum_t: u128= 0;
        for _j in 0..k {
            let (c, s, m, t) = test_function(function, ord, i);
            sum_c += c/ k as u128;
            sum_s += s/ k as u128;
            sum_m += m/ k as u128 + i as u128 * mem::size_of::<u32>() as u128;
            sum_t += t/ k as u128;
        }
        let to_write = format!("{};{};{};{};{}\n", i, sum_c, sum_s, sum_m, sum_t);
        match file.write(to_write.as_ref()) {
            Ok(_o) => (),
            Err(err) => panic!("{}", err)
        }
        if five{
            five = false;
            i *= 5;
        } else {
            five = true;
            i *= 2;
        }
    }
}

//Zwraca odpowiednią funkcję porządku
fn get_order_function(s: &str) -> fn(&u32, &u32) -> bool {
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

//Zwraca listę liczb wpisanych do kosnoli
fn get_line() -> Vec<u32> {
    let mut res: Vec<u32> = vec![];
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    for s in input_text.trim().split_whitespace() {
        match s.parse::<u32>() {
            Ok(number) => {
                res.push(number);
            }
            Err(..) => panic!("Is not a number {}", s),
        };
    }
    res
}

//Sprawdza gdzie w Vectorze znajduje się element
fn find_in_vec<A>(v: &Vec<A>, to_find: A) -> Option<usize>
    where A: PartialEq
{
    for i in 0..v.len()
    {
        if v[i] == to_find {
            return Some(i);
        }
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        //Sortowanianie
        5 => {
            let s_arg_pos = find_in_vec(&args, "--sort".parse().unwrap()).expect("none --sort param");
            let o_arg_pos = find_in_vec(&args, "--comp".parse().unwrap()).expect("none --comp param");
            let mut input = get_line();
            let mut array: Vec<u32> = Vec::new();
            let n: u32 = input[0];
            input.remove(0);
            let mut i = 0;
            loop {
                for x in input.iter() {
                    array.push(x.clone());
                    i += 1;
                }
                if i < n {
                    input = get_line();
                } else {
                    break;
                }
            }
            let ord: fn(&u32, &u32) -> bool = get_order_function(args[o_arg_pos + 1].as_str());
            println!("Sorting");
            match args[s_arg_pos + 1].as_str() {
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
                "radix" => {
                    u32_radix_sort(&mut array, ord);
                }
                "improved_quick" => {
                    u32_improved_quick_sort(&mut array, ord);
                }
                _ => panic!("Unknown sort type"),
            }
            println!("{:?}", array);
        }
        //Statystyki
        8 => {
            let s_arg_pos = find_in_vec(&args, "--sort".parse().unwrap()).expect("none --sort param");
            let o_arg_pos = find_in_vec(&args, "--comp".parse().unwrap()).expect("none --comp param");
            let st_arg_pos = find_in_vec(&args, "--stat".parse().unwrap()).expect("none --stat param");
            let mut file = File::create(args[st_arg_pos + 1].as_str()).unwrap();
            let k = args[st_arg_pos + 2].as_str().parse::<u32>().unwrap();
            let ord: fn(&u32, &u32) -> bool = get_order_function(args[o_arg_pos + 1].as_str());
            println!("Creating stats");
            match args[s_arg_pos + 1].as_str() {
                "merge" => {
                    test_and_save(u32_merge_sort, ord, k, &mut file);
                }
                "insert" => {
                    test_and_save(u32_insert_sort, ord, k, &mut file);
                }
                "quick" => {
                    test_and_save(u32_quick_sort, ord, k, &mut file);
                }
                "double" => {
                    test_and_save(u32_double_sort, ord, k, &mut file);
                }
                "mix" => {
                    test_and_save(u32_mix_sort, ord, k, &mut file);
                }
                "radix" => {
                    test_and_save(u32_radix_sort, ord, k, &mut file);
                }
                "improved_quick" => {
                    test_and_save(u32_improved_quick_sort, ord, k, &mut file);
                }
                _ => panic!("Unknown sort type"),
            }
        }
        _ => println!("Wrong number of arguments"),
    }
}

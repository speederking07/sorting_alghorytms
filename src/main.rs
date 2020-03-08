#![feature(duration_as_u128)]
extern crate rand;

use std::{env, io};
use std::fs::File;
use std::io::Write;
use std::ops::Add;
use std::time::Instant;
use rand::*;
use std::str::SplitWhitespace;

struct Stat {
    comp: i32,
    swap: i32,
}

impl Stat {
    fn new() -> Self {
        Stat {
            comp: 0,
            swap: 0,
        }
    }

    fn swap(&mut self) {
        self.swap += 1;
    }

    fn comp(&mut self) {
        self.comp += 1;
    }
}

impl Add for Stat {
    type Output = Stat;

    fn add(self, other: Stat) -> Stat {
        Self { comp: self.comp + other.comp, swap: self.swap + other.swap }
    }
}

fn merge<A>(arr1: &[A], arr2: &[A], ret: &mut [A], ord: fn(&A, &A) -> bool) -> Stat
    where A: Clone
{
    let mut stat = Stat::new();
    let mut left = 0;
    let mut right = 0;
    let mut i = 0;
    while left < arr1.len() && right < arr2.len() {
        stat.comp();
        if ord(&arr1[left], &arr2[right]) {
            stat.swap();
            ret[i] = arr1[left].clone();
            i += 1;
            left += 1;
        } else {
            stat.swap();
            ret[i] = arr2[right].clone();
            i += 1;
            right += 1;
        }
    }
    while left < arr1.len() {
        stat.swap();
        ret[i] = arr1[left].clone();
        i += 1;
        left += 1;
    }
    while right < arr2.len() {
        stat.swap();
        ret[i] = arr2[right].clone();
        i += 1;
        right += 1;
    }
    stat
}

fn merge_sort<X, A>(mut array: X, ord: fn(&A, &A) -> bool) -> Stat
    where X: AsMut<[A]>, A: Clone
{
    let arr = array.as_mut();
    let mid = arr.len() / 2;
    if mid == 0 {
        return Stat::new();
    }

    let s1 = merge_sort(&mut arr[..mid], ord);
    let s2 = merge_sort(&mut arr[mid..], ord);
    let mut ret = arr.to_vec(); //temporary vector
    let s3 = merge(&arr[..mid], &arr[mid..], &mut ret[..], ord);
    let mut i = 0;
    for e in ret {
        arr[i] = e;
        i += 1;
    }
    s1 + s2 + s3
}

fn mix_merge_sort<X, A>(mut array: X, ord: fn(&A, &A) -> bool) -> Stat
    where X: AsMut<[A]>, A: Clone
{
    let arr = array.as_mut();
    let mid = arr.len() / 2;
    if mid == 0 {
        return Stat::new();
    }

    let s1 = mix_sort(&mut arr[..mid], ord);
    let s2 = mix_sort(&mut arr[mid..], ord);
    let mut ret = arr.to_vec(); //temporary vector
    let s3 = merge(&arr[..mid], &arr[mid..], &mut ret[..], ord);
    let mut i = 0;
    for e in ret {
        arr[i] = e;
        i += 1;
    }
    s1 + s2 + s3
}

fn insert_sort<X, A>(mut array: X, ord: fn(&A, &A) -> bool) -> Stat
    where X: AsMut<[A]>
{
    let mut stat = Stat::new();
    let arr = array.as_mut();
    for i in 1..arr.len() {
        for j in (1..=i).rev() {
            stat.comp();
            if ord(&arr[j - 1], &arr[j]) {
                break;
            } else {
                stat.swap();
                arr.swap(j - 1, j);
            }
        }
    }
    stat
}

fn partition<A>(arr: &mut [A], ord: fn(&A, &A) -> bool) -> (usize, Stat)
    where A: Clone
{
    let mut stat = Stat::new();
    let pivot = arr[arr.len() - 1].clone();
    let mut i: usize = 0;
    for j in 0..arr.len() {
        stat.comp();
        if !ord(&pivot, &arr[j]) {
            stat.swap();
            arr.swap(i, j);
            i += 1;
        }
    }
    stat.swap();
    arr.swap(i, arr.len() - 1);
    return (i, stat);
}

fn quick_sort<X, A>(mut array: X, ord: fn(&A, &A) -> bool) -> Stat
    where X: AsMut<[A]>, A: Clone
{
    let arr = array.as_mut();
    if arr.len() > 1 {
        let (p, s1) = partition(arr, ord);
        let s2 = quick_sort(&mut arr[..p], ord);
        let s3 = quick_sort(&mut arr[p + 1..], ord);
        return s1 + s2 + s3;
    }
    Stat::new()
}

fn double_partition<A>(arr: &mut [A], ord: fn(&A, &A) -> bool) -> (usize, usize, Stat)
    where A: Clone
{
    let mut stat = Stat::new();
    stat.comp();
    if !ord(&arr[0], &arr[arr.len() - 1]) {
        stat.swap();
        arr.swap(0, arr.len() - 1);
    }
    let l = arr[0].clone();
    let h = arr[arr.len() - 1].clone();
    let mut i = 1;
    let mut low_end = 1;
    let mut high_end = arr.len() - 2;
    while i <= high_end {
        stat.comp();
        if !ord(&l, &arr[i]) {
            stat.swap();
            arr.swap(low_end, i);
            low_end += 1;
        } else if !ord(&arr[i], &h) {
            stat.comp();
            while !ord(&arr[high_end], &h) && i < high_end {
                stat.comp();
                high_end -= 1;
            }
            stat.comp();
            stat.swap();
            arr.swap(i, high_end);
            high_end -= 1;
            stat.comp();
            if !ord(&l, &arr[i]) {
                stat.swap();
                arr.swap(low_end, i);
                low_end += 1;
            }
        }
        i += 1;
    }
    stat.swap();
    arr.swap(low_end - 1, 0);
    stat.swap();
    arr.swap(high_end + 1, arr.len() - 1);
    return (low_end - 1, high_end + 1, stat);
}

fn double_sort<X, A>(mut array: X, ord: fn(&A, &A) -> bool) -> Stat
    where X: AsMut<[A]>, A: Clone
{
    let arr = array.as_mut();
    if arr.len() > 1 {
        let (p, q, s1) = double_partition(arr, ord);
        let s2 = double_sort(&mut arr[..p], ord);
        let s3 = double_sort(&mut arr[p + 1..q], ord);
        let s4 = double_sort(&mut arr[q + 1..], ord);
        return s1 + s2 + s3 + s4;
    }
    Stat::new()
}

fn mix_double_sort<X, A>(mut array: X, ord: fn(&A, &A) -> bool) -> Stat
    where X: AsMut<[A]>, A: Clone
{
    let arr = array.as_mut();
    if arr.len() > 1 {
        let (p, q, s1) = double_partition(arr, ord);
        let s2 = mix_sort(&mut arr[..p], ord);
        let s3 = mix_sort(&mut arr[p + 1..q], ord);
        let s4 = mix_sort(&mut arr[q + 1..], ord);
        return s1 + s2 + s3 + s4;
    }
    Stat::new()
}

fn mix_sort<X, A>(mut array: X, ord: fn(&A, &A) -> bool) -> Stat
    where X: AsMut<[A]>, A: Clone
{
    let arr = array.as_mut();
    return if arr.len() <= 4 {
        insert_sort(array, ord)
    } else if arr.len() <= 100 {
        mix_double_sort(array, ord)
    } else {
        mix_merge_sort(array, ord)
    }
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
    /*let mut t = ["dasdasdadsasdas", "dsad", "dasdsad", "dasdasd"];
    mix_sort(&mut t, |&a, &b| a.len() <= b.len());
    println!("{:?}", t);
    let ord: fn(&u32, &u32) -> bool = |&a, &b| a <= b;
    //test_function(mix_sort::<&mut Vec<u32>, u32>, ord, 1);
    for i in 0..10000 {
        let (nc, ns, nt) = test_function(u32_mix_sort, ord, i);
        let (mc, ms, mt) = test_function(u32_merge_sort, ord, i);
        let (qc, qs, qt) = test_function(u32_quick_sort, ord, i);
        let (dc, ds, dt) = test_function(u32_double_sort, ord, i);
        let (ic, is, it) = test_function(u32_insert_sort, ord, i);
        println!("{} - {} {} {} {} {}", i, nc, mc, qc, dc, ic);
    }*/
    //let f: fn(& mut Vec<u32>, fn(&u32, &u32) -> bool) -> Stat = merge_sort;
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

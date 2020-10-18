use crate::stat::Stat;
use std::mem::size_of;

const COUNT_SORT_PARTITION: usize = 10;

pub fn count_sort(data: &mut Vec<u32>, f: Box<dyn Fn(u32) -> usize>) -> Stat{
    let mut stat = Stat::new();
    let mut partition = vec![vec![]; COUNT_SORT_PARTITION];
    for e in data.iter() {
        stat.comp();
        stat.swap();
        partition[f(*e)].push(*e);
    }
    stat.add_mem((data.len() * size_of::<u32>() + size_of::<u32>() * COUNT_SORT_PARTITION + 12) as i32);
    data.clear();
    for mut c in partition {
        stat.swap += c.len() as u64;
        data.append(c.as_mut())
    }

    stat
}

fn pos_fn(n: u32, asc: bool) -> Box<dyn Fn(u32) -> usize> {
    let d: u32 = (COUNT_SORT_PARTITION as u32).pow(n);
    let m: u32 = COUNT_SORT_PARTITION as u32;
    return if asc {
        Box::new(move |x| ((x / d) % m) as usize)
    } else {
        Box::new(move |x| (COUNT_SORT_PARTITION as u32 - 1 - (x / d) % m) as usize)
    };
}

pub fn radix_sort(array: &mut Vec<u32>, asc: bool) -> Stat
{
    let mut stat = Stat::new();
    let max = (array.iter().cloned().fold(0, u32::max) as f32).log(COUNT_SORT_PARTITION as f32).ceil() as u32;
    for i in 0..max{
        stat = stat + count_sort(array, pos_fn(i, asc));
    }
    stat.add_mem(12 as i32);
    stat
}
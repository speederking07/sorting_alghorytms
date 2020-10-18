use crate::stat::Stat;
use crate::select::good_pivot;
use crate::double_sort::double_partition;
use crate::quick_sort::partition;

pub fn improved_quick_sort<X, A>(mut array: X, ord: fn(&A, &A) -> bool) -> Stat
    where X: AsMut<[A]>, A: Clone
{
    let arr = array.as_mut();
    if arr.len() > 1 {
        let s0 = good_pivot(arr, ord);
        let (p, s1) = partition(arr, ord);
        let s2 = improved_quick_sort(&mut arr[..p], ord);
        let s3 = improved_quick_sort(&mut arr[p + 1..], ord);
        let mut stat = s1 + s2 + s3 + s0;
        stat.add_mem((20) as i32);
        return stat;
    }
    let mut s = Stat::new();
    s.add_mem((16) as i32);
    return s;
}

pub fn improved_double_sort<X, A>(mut array: X, ord: fn(&A, &A) -> bool) -> Stat
    where X: AsMut<[A]>, A: Clone
{
    let arr = array.as_mut();
    if arr.len() > 1 {
        let s0 = good_pivot(arr, ord);
        arr.swap(0, arr.len()-1);
        arr.swap(arr.len()-2, arr.len()-1);
        let (p, q, s2) = double_partition(arr, ord);
        let s3 = improved_double_sort(&mut arr[..p], ord);
        let s4 = improved_double_sort(&mut arr[p + 1..q], ord);
        let s5 = improved_double_sort(&mut arr[q + 1..], ord);
        return s0 + s2 + s3 + s4 + s5;
    }
    Stat::new()
}
use crate::stat::Stat;
use crate::insert_sort::insert_sort;
use crate::double_sort::double_partition;
use crate::merge_sort::merge;

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

pub fn mix_sort<X, A>(mut array: X, ord: fn(&A, &A) -> bool) -> Stat
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
use crate::stat::Stat;

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

pub fn quick_sort<X, A>(mut array: X, ord: fn(&A, &A) -> bool) -> Stat
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
use crate::stat::Stat;

pub fn partition<A>(arr: &mut [A], ord: fn(&A, &A) -> bool) -> (usize, Stat)
    where A: Clone
{
    let mut stat = Stat::new();
    let pivot = arr[arr.len() - 1].clone();
    let mut i: usize = 0;
    for j in 0..arr.len() {
        stat.comp();
        //jeśli arr[j] > pivot
        if !ord(&pivot, &arr[j]) {
            stat.swap();
            arr.swap(i, j);
            i += 1;
        }
    }
    stat.swap();
    //ustawianie pivata na sowim miejscu
    arr.swap(i, arr.len() - 1);
    stat.add_mem(20 as i32);
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
        let mut stat = s1 + s2 + s3;
        stat.add_mem((20) as i32);
        return stat;
    }
    let mut s = Stat::new();
    s.add_mem((16) as i32);
    return s;
}
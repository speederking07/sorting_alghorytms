/* Double pivot sort */
use crate::stat::Stat;

pub fn double_partition<A>(arr: &mut [A], ord: fn(&A, &A) -> bool) -> (usize, usize, Stat)
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
        //jeśli &arr[i] > mały pivot
        if !ord(&l, &arr[i]) {
            stat.swap();
            arr.swap(low_end, i);
            low_end += 1;
        //jeśli &arr[i] < duży pivot
        } else if !ord(&arr[i], &h) {
            stat.comp();
            //szukanie elementu do zamiany
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
    //ustawianne pivotów na swoje miejsca
    stat.swap();
    arr.swap(low_end - 1, 0);
    stat.swap();
    arr.swap(high_end + 1, arr.len() - 1);
    return (low_end - 1, high_end + 1, stat);
}

pub fn double_sort<X, A>(mut array: X, ord: fn(&A, &A) -> bool) -> Stat
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
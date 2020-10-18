use crate::stat::Stat;
use crate::quick_sort::partition;

fn deterministic_selection<X, A>(mut data: X, k: usize, ord: fn(&A, &A) -> bool) -> (usize, Stat)
    where X: AsMut<[A]>, A: Clone{
    let arr = data.as_mut();
    let p_comp = good_pivot(arr, ord);
    let (pivot, comp) = partition(arr, ord);
    return if pivot == k {
        (k, comp)
    } else if pivot > k {
        let (pos, c) = deterministic_selection(&mut arr[..pivot], k, ord);
        (pos, c + comp + p_comp)
    } else {
        let (pos, c) = deterministic_selection(&mut arr[pivot + 1..], k - pivot - 1, ord);
        (pos + pivot + 1, c + comp + p_comp)
    }
}

fn five_median<A>(a: &mut [A], ord: fn(&A, &A) -> bool) -> Stat {
    let mut stat = Stat::new();
    if ord(&a[0], &a[1]) {
        stat.swap();
        a.swap(0, 1);
    }
    if ord(&a[2], &a[3]) {
        stat.swap();
        a.swap(2, 3);
    }
    if ord(&a[0], &a[2]) {
        stat.swap();
        a.swap(0, 2);
    }
    if ord(&a[1], &a[2]) {
        stat.swap();
        a.swap(1, 2);
    }
    if ord(&a[3], &a[4]) {
        stat.swap();
        a.swap(3, 4);
    }
    if ord(&a[1], &a[3]) {
        stat.swap();
        a.swap(1, 3);
    }
    if ord(&a[2], &a[3]) {
        stat.swap();
        a.swap(2, 3);
    }
    if ord(&a[2], &a[4]) {
        stat.swap();
        a.swap(2, 4);
    }
    stat.comp+=8;
    stat
}

pub fn good_pivot<A>(a: &mut [A], ord: fn(&A, &A) -> bool) -> Stat where A: Clone {
    let mut last = a.len() - 1;
    if last < 4{
        return Stat::new();
    }
    let mut stat = Stat::new();
    for i in (1..=a.len() / 5_usize).rev() {
        stat = stat + five_median(&mut a[5 * (i - 1)..5 * i], ord);
        stat.swap();
        a.swap(last, 5 * i - 3);
        last -= 1;
    }
    let len = a.len();
    let (_, s_stat) = deterministic_selection(&mut a[(last + 1)..], len / 10_usize, ord);
    a[(last + 1)..].swap(len / 10_usize, len - last - 2);
    return stat + s_stat;
}
use crate::stat::Stat;
use std::mem;

pub fn merge<A>(arr1: &[A], arr2: &[A], ret: &mut [A], ord: fn(&A, &A) -> bool) -> Stat
    where A: Clone
{
    let mut stat = Stat::new();
    let mut left = 0;
    let mut right = 0;
    let mut i = 0;
    //łączenie arr1 i arr2 tablic w arr3
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
    //Uzupełnianie pozostałych elementów
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
    stat.add_mem((20) as i32);
    stat
}

pub fn merge_sort<X, A>(mut array: X, ord: fn(&A, &A) -> bool) -> Stat
    where X: AsMut<[A]>, A: Clone
{
    let arr = array.as_mut();
    let mid = arr.len() / 2;
    if mid == 0 {
        return Stat::new();
    }
    let s1 = merge_sort(&mut arr[..mid], ord);
    let s2 = merge_sort(&mut arr[mid..], ord);
    let mut ret = arr.to_vec(); //tymczasowy vektor
    let s3 = merge(&arr[..mid], &arr[mid..], &mut ret[..], ord);
    let mut i = 0;
    //kopiowanie vectora tymczasowego do właściwego
    let mut stat = s1 + s2 + s3;
    stat.add_mem((ret.len() * mem::size_of::<A>() + 16) as i32);
    for e in ret {
        arr[i] = e;
        i += 1;
    }
    stat
}

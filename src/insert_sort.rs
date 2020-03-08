use crate::stat::Stat;

pub fn insert_sort<X, A>(mut array: X, ord: fn(&A, &A) -> bool) -> Stat
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
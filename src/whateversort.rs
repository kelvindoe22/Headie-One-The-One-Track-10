use super::*;

pub struct WhateverSort;

fn whatever<T: Ord>(slice: &mut [T]){
    match slice.len() {
        0 => {return;},
        1 => {return;},
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        },
        _ =>{}
    }
    let mid = slice.len()/2;
    let (left, right) = slice.split_at_mut(mid);
    whatever(left);
    whatever(right);

    if slice[0] > slice[slice.len() - 1]{
        slice.rotate_left(mid);
        return;
    }

    let mut j_offset = 0_usize;

    for i in (0..mid).rev(){
        for j in (mid..slice.len()).rev(){
            if slice[i] > slice[j - j_offset]{
                slice[i..=(j-j_offset)].rotate_left(mid - j_offset - i);
                j_offset += 1;
                break;
            }
        }
        if i+1 == mid && j_offset == 0{
            break;
        }
    }

}

impl Sorter for WhateverSort{
    fn sort<T:Ord>(slice: &mut [T]){
        whatever(slice);
    }
}

#[test]
fn whatever_works(){
    let mut a = [28, 24, 14, 31, 5, 6, 18, 19, 35, 26, 29, 16, 20, 10, 23, 2, 38, 32, 34, 17, 40, 9, 45, 33, 48, 44, 43, 11, 49, 15, 3, 8, 21, 47, 1, 41, 13, 37, 12, 36, 25, 27, 39, 22, 7, 46, 30, 42, 4];
    sort::<_,WhateverSort>(&mut a);
    assert_eq!(a,[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49] );
}
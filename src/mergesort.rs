use super::*;

struct MergeSort;

fn mergesort<T:Ord + Copy + Debug>(slice: &mut[T]){
    match slice.len(){
        0 => return,
        1 => return,
        2 => {
            if slice[0] > slice[1]{
                slice.swap(0, 1);
                return;
            }
        },
        _ => {}
    }
    let mut y = slice.to_vec();
    let mid = slice.len()/2;
    let (left, right) = slice.split_at_mut(mid);
    mergesort(left);
    mergesort(right);
    merge(&left,&right, &mut y[..]);
    println!("{:?}",y);
    slice.copy_from_slice(&y[..]);
}

fn merge<T:Ord + Copy + Debug>(left: &[T], right: &[T], control: &mut [T]){
    let (mut c, mut l, mut r) = (0_usize,0_usize,0_usize);
    while l < left.len() && r < right.len(){
        if left[l] < right[r]{
            control[c] = left[l];
            c+=1;
            l+=1
        }else{
            control[c] = right[r];
            c += 1;
            r += 1;
        }
    }
    if l < left.len(){
        control[c..].copy_from_slice(&left[l..]);
    }else{
        control[c..].copy_from_slice(&right[r..])
    }
}

impl Sorter for MergeSort {
    fn sort<T>(slice: &mut [T])
    where T: Ord {
        drop(slice)
    }
}

impl CopySorter for MergeSort {
    fn sort<T:Ord + Copy + Debug>(slice: &mut[T]){
        mergesort(slice);
    }


} 















#[test]
fn mergesort_works(){
    let mut a = [10,3,5,4,1,0];
    copy_sort::<_,MergeSort>(&mut a);
    assert_eq!(a,[0,1,3,4,5,10]);
}
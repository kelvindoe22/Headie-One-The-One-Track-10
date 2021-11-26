use super::Sorter;


pub struct QuickSort;

fn quicksort<T:Ord>(slice: &mut [T]){
    match slice.len(){
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1]{
                slice.swap(0, 1);
                return;
            }
        }
        _ => {}

    }


    let (pivot,rest) = slice.split_first_mut().expect("empty slice");

    let mut left = 0;
    let mut right = rest.len()-1;

    while left <= right {
        if &rest[left as usize] <= pivot{
            left += 1;
        }
        else if &rest[right as usize] > pivot {
            if right ==0 {
                break;
            }
            right -= 1;
        }
        else {

            rest.swap(left as usize,right as usize);
            left += 1;
            if right == 0{
                continue;
            }
            right -= 1
        }
    }

    let left = (left + 1) as usize; 


    slice.swap(0, left-1);
    let (left,right) = slice.split_at_mut(left-1);
    assert!(left.last() <= right.first());
    quicksort(left);
    quicksort(&mut right[1..]);
}

impl Sorter for QuickSort {
    fn sort<T>(slice:&mut [T])
    where T: Ord
    {
        quicksort(slice)
    }
}

#[test]
fn quicksort_works(){
    let mut a = [6,7,4,1,3];
    super::sort::<_,QuickSort>(&mut a);
    assert_eq!(&a, &[1,3,4,6,7]);
}
#[test]
fn histest(){
    let mut a = [5,4,3,2,1];
    super::sort::<_,QuickSort>(&mut a);
    assert_eq!(&a,&[1,2,3,4,5]);
}


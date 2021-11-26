use super::Sorter;


pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(slice:&mut [T])
    where T: Ord
    {
        for unsorted in 0..slice.len(){
            let mut smallest_in_rest = unsorted;

            for i in (unsorted+1)..slice.len(){
                if slice[i] < slice[smallest_in_rest]{
                    smallest_in_rest = i;
                }
            }

            let (mut smallest_in_rest2,_) =  slice[unsorted..]
            .iter()
            .enumerate()
            .min_by_key(|&(_,v)| v)
            .expect("whoops empty");

            smallest_in_rest2 = smallest_in_rest2+unsorted;

            assert_eq!(smallest_in_rest2,smallest_in_rest);

            if smallest_in_rest != unsorted{
                slice.swap(smallest_in_rest, unsorted);
            }
        }
        
    }
}

#[test]
fn selectionsort_works(){
    let mut a = [4,3,1,5,0];
    super::sort::<_,SelectionSort>(&mut a);
    assert_eq!(&a, &[0,1,3,4,5]);
}
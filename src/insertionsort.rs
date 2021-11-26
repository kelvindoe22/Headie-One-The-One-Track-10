use super::Sorter;


pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T>(slice:&mut [T])
    where T: Ord
    {
        for unsorted in 1..slice.len(){

            let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                Ok(i)=>i,
                Err(i)=> i  
            };
            &slice[i..=unsorted].rotate_right(1);
        }
    }
}

#[test]
fn insertionsort_works(){
    let mut a = [4,3,1,5,0,99];
    super::sort::<_,InsertionSort>(&mut a);
    assert_eq!(&a, &[0,1,3,4,5,99]);
}
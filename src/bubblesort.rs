use super::Sorter;


pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(slice:&mut [T])
    where T: Ord{
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 0..(slice.len()-1){
                if slice[i] > slice[i+1]{
                    slice.swap(i,i+1);
                    swapped = true;
                }
            }
            // println!("{:?}",&slice)
        }
    }
}

#[test]
    fn bubblesort_works(){
        let mut a = [1,9,0,5,8,2,10];
        super::sort::<_,BubbleSort>(&mut a);
        assert_eq!(&a, &[0,1,2,5,8,9,10])
    }

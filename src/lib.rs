pub trait Sorter{
    fn sort<T>(slice: &mut [T])
    where T: Ord;
}

pub trait CopySorter : Sorter {
    fn sort<T: Ord + Copy + Debug>(slice: &mut[T]);
}

pub fn sort<T,U>(slice: &mut [T])
where T: Ord , U: Sorter
{
    U::sort(slice)
}

pub fn copy_sort<T,U>(slice: &mut [T])
where T: Ord + Copy + Debug, U: CopySorter
{
   <U as CopySorter>::sort(slice);
}

mod bubblesort;
mod insertionsort;
mod selectionsort;
mod quicksort;
mod whateversort;
mod mergesort;

use std::fmt::Debug;

pub use bubblesort::BubbleSort;
pub use insertionsort::InsertionSort;
pub use selectionsort::SelectionSort;
pub use quicksort::QuickSort;

#[cfg(test)]
mod tests{
}
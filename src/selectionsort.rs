use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T]) where T: Ord {
        // [sorted | unsorted]
        for unsorted in 0..slice.len() {
            // let (smallest_in_rest, _) = slice[unsorted..].iter()
            //     .enumerate()
            //     .min_by_key(|&(_, v)| v)  // Dereference the tuple and fetch out the v, as opposed to returning a reference to the second element of the tuple.  t
            //     .expect("sice is non-empty");
            // smallest_in_rest = unsorted + smallest_in_rest;

            let mut smallest_in_rest = unsorted;
            for i in (unsorted + 1)..slice.len() {
                if slice[i] < slice[smallest_in_rest] {
                    smallest_in_rest = i
                }
            }
            if unsorted != smallest_in_rest {
                slice.swap(unsorted, smallest_in_rest);

            }
        }
    }
}

#[test]
fn selectionsort_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    SelectionSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}

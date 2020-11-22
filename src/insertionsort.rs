use super::Sorter;

pub struct InsertionSort {
    pub smart: bool,
}

impl Sorter for InsertionSort {
    fn sort<T>(&self, slice: &mut [T]) where T: Ord {
        // [ sorted | unsorted ]
        for unsorted in 1..slice.len() {
            if !self.smart {
                // slice[unsorted..] is unsorted
                // take slice[unsorted] and place in sorted location in slice[..=unsorted]
                // [ 1 3 4 | 2 ] - requires shifting
                // easist way is to keep moving the element selected to the left until the next left element is smaller.
                let mut i = unsorted;
                while i > 0 && slice[i-1] > slice[i] {
                    slice.swap(i-1, i);
                    i -= 1;
                }
            } else {
                // binary search tells you where the element *should* go on such a list.
                let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    Ok(i) => i,
                    Err(i) => i,
                };
                // can insert on a vector, not a slice
                //slice.insert(i, &slice[unsorted]);
                // rotate everything right starting at the insert location and ending at the element
                // to be inserted.  Since rotate_right **wraps around**, you are moving the unsorted
                // element into the ith position.
                slice[i..=unsorted].rotate_right(1);
            }
            
        }
    }
}

#[test]
fn insertionsort_works_smart() {
    let mut things = vec![4, 2, 5, 3, 1];
    InsertionSort{smart: true}.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
#[test]
fn insertionsort_works_dumb() {
    let mut things = vec![4, 2, 5, 3, 1];
    InsertionSort{smart: false}.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
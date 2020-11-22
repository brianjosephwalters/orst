use super::Sorter;

pub struct QuickSort;

fn quicksort<T: Ord>(slice: &mut [T]) {
    // Base case: list so small its trivial to sort
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1)
            }
            return;
        },
        _ => {}
    }
    // Get a reference to the pivot and still be allowed to modify the slice.
    // if we took an immutable refence to the pivot, then we wouldn't be allowed
    // to modify the slice.  once you have a reference to the slice, you can't also modify
    // that slice.
    let (pivot_slice, rest) = slice.split_at_mut(1);
    let pivot = &pivot_slice[0];
    // Everything below the left index is on the left side
    // Everything above the right index is on the right side.
    let mut left = 0;
    let mut right = rest.len() - 1;

    // Can't use for-loop because once we do a swap, we need to look at the element again.
    while right != usize::MAX && left <= right {
        if &rest[left] <= pivot {
            // already on correct side.
            left += 1;
        } else if &rest[right] > pivot {
            // right already on the correct side
            // avoid unnecessary swaps back and forth
            right = right.wrapping_sub(1);
        } else {
            // left holds a right and right holds a left. swap them.
            rest.swap(left, right);
            left += 1;
            right = right.wrapping_sub(1);
            // And we recheck element at left again (what was just moved there).
        }
    }

    // Realign left to account for the pivot at 0
    left = left + 1;

    // Place the pivot in its final location
    slice.swap(0, left - 1);

    let (left_side, right_side) = slice.split_at_mut(left - 1);
    quicksort(left_side);
    quicksort(&mut right_side[1..]);
}

impl Sorter for QuickSort {
    fn sort<T>(&self, slice: &mut [T]) where T: Ord {
        // [unsorted | pivot | unsorted]
        quicksort(slice);
    }
}

#[test]
fn quicksort_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    QuickSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}

#[test]
fn quicksort_works_base() {
    let mut things = vec![2, 1];
    QuickSort.sort(&mut things);
    assert_eq!(things, &[1, 2]);
}
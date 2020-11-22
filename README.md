# Sorting Algorithms in Rust


## Simple Setup
* Create a Sorter trait - an instance method since sorters themselves won't have any state.
    * A Sorter takes a mutable slice of T
* Create a public function to which you can pass a slice
    * It uses the generic type to determine which Sorter to apply?
* Create a test that utilizes this structure with sort from the rust libraries.

## Bubble Sort
* Note that BubbleSort is a unit struct.  It doesn't have state.
* slice.swap() internally uses mem-swap.  (Isn't creating a variable to hold references while swapping.)
* Why require the _ in `super::sort::<_, BubbleSort>(&mut things);`?
    * We have two type parameters to this function.
    * Since we are trying to name the sorting type so we have to put place holders for all of the other generic types.

## Insertion Sort
* Why not `SorterName::sort(&mut things)` direclty without the freestanding function `super::sort::<_, BubbleSort>(&mut things);`?
    * Downside is that you need to have the `Sorter` trait in scope. (Is this right?)
* Can't use `PartialEq` because PartialEq doesn't let you order things.
* Can't use `PartialOrd` if some elements can say "I don't know what the order should be here."
* Everything beyond our threshold is unsorted, everything before it is sorted.
    * Start with 1 because a list of 1 element is always sorted.
* InsertionSort works best if you know it's a linked list.  Slices require shifting
* Another way:
    * [1 3 4 2 5]
      hold the 2 in a variable
      find where 2 goes
      shift to the right by one, from there to where 2 was
    * both end up being two swaps.
* Why does binary search return `Ok()` and `Error()`?
    * when returning index where the thing was found, uses Ok().
    * when returning index where it should have been found, uses Err().

### Refactor:
* Change from `::` to `.` you can think of it like a change from class method to instance method.
    * an associated method of the type vs. a method of the type.

## Selection Sort
* Don't have to store any additional elements (i.e., references while doing swaps, etc.)
* Pattern:
    * Frame the list: find the smallest element of that frame, and stick it in the front.
    * Frame the remainder of the list: find the smallest element of that frame and stick it in the front.
    * Smallest of remainder is always going to be the largest of the thing that' sorted.
        * need to make sure the first element is the smallest.
```
let (smallest_in_rest, _) = slice[unsorted..].iter()
    .enumerate()
    .min_by_key(|&(_, v)| v)  reference to the second element of the tuple.  t
    .expect("sice is non-empty");
let smallest_in_rest = unsorted + smallest_in_rest;
```
* Why use `|&(_, v)| v` and not `|(_,v)| v`?
    * We need to dereference the tuple and fetch out the v, as opposed to returning a reference to the select element of the tuple.
    * The lifetime of tuple is only for the lifetime of the `iter()`. Since we are returning v, it needs the lifetime of the value, not the tuple's reference to the value. The bad way is equivalent to `.min_by_key(|t| &t.1)`.  The corret way really says `min_by_key(|t| t.1)`
        * an & in a pattern is the same as the removal of an & in the value. 
* `smallest_in_rest` refers to an index starting at unsorted, so index on the slice needs to add the two.

## Quick Sort
Pick an element at random from the list, walk the list.  Put everything smaller to one side and everything larget to the other side.  Do this recursively down until all are sorted.
* Allocating and in-place implementations.
### Allocating
* Base case is when the slice is trivial to sort, either 0 or 1 elements.
* Otherwise, we find a pivot point, and put everything less that the pivot into one bucket, and everything greater into another.
* Then quicksort each bucket,
* finally put the two buckets back together with the pivot, left being less than right.
```
fn quicksort<T: Ord>(slice: &mut [T]) {
    // Base case: list so small its trivial to sort
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
            slice.swap(0, 1)
            }
        },
        _ => {}
    }
    let pivot = &slice[0];

    // ineffecient allocations
    let mut left = vec![];
    let mut right = vec![];
    // Errors trying to move out of a slice.  Need tricks
    for i in slice {
        if slice[i] <= pivot {
            left.push(slice[i]);
        } else {
            right.push(slice[i]);
        }
    }
    quicksort(left);
    quicksort(right);
    // merge together
    ..
}
```
* Difficult to implement with rust - need tricks for allocating and moving around slices.  Borrow-checker
### In Place
* Start at 1 - if pivot always goes to the left, don't need to pivot.
* Base case 2 - pivot or not if the list is of size 2.

* Otherwise, similar recursive idea, but:
    * use slice.split_at_mut(index) to get two mutable portions of the original slice.
        * This is really just getting the pivot from the 0th element without upsetting the borrow checker.
    * create two indexes that move inward from the beginning and end of the slice.
        * everything to the left of the left index should be lower than the pivot
        * everything to the right of the right index should be higher than the pivot.
        * track the left index until it is equal to the right.

* when always choosing pivot to be the same element, and it is the largest element, then left will never shrink. So you get infinite recursion.

# Benchmarking
To Run: `cargo run --release`
* Implement `PartialEq` manually for the `SortEvaluator<T>`s because we want to consider only the T and not the cmps.
    * cmps is a counter so that every time we compare an element we increment the counter.
    * we are testing Ord so we only increment cmps with the Ord trait.  cmp forwards to partial_cmp.
* Instead of using:
    ```
    use std::sync::{
        Arc, 
        atomic::{AtomicUsize, Ordering::Relaxed}
    };
    ```
    We can use 
    ```
    use std::cell::Cell;
    use std::rc::Rc;
    ```
    Because we don't need it to be threadsafe.
* Can't use a closure because Sorter is not object safe, so change
    ```
    // closure: for each test, reset the counter and sort.
    let bench = |sorter: &dyn Sorter| {
        let mut values = values.clone();
        counter.set(0);
        sorter.sort(&mut values);
        counter.get()
    };
    ```
    Instead use:
    ```
    fn bench<T: Ord + Clone, S: Sorter>(sorter: S, values: &[T], counter: &Cell<usize>) -> usize {
        let mut values: Vec<_>= values.into_iter().cloned().collect();
        counter.set(0);
        sorter.sort(&mut values);
        counter.get()
    }
    ```
    * The sorter trait is not object safe is because we have a method that is generic in it.
    * We cannot box the sorter because the trait is not object safe because it has a generic method on it.

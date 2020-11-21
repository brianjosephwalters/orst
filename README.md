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
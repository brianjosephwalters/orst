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
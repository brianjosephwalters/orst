# Sorting Algorithms in Rust


## Simple Setup
* Create a Sorter trait - an instance method since sorters themselves won't have any state.
    * A Sorter takes a mutable slice of T
* Create a public function to which you can pass a slice
    * It uses the generic type to determine which Sorter to apply?
* Create a test that utilizes this structure with sort from the rust libraries.

## Bubblesort
* Note that BubbleSort is a unit struct.  It doesn't have state.
* slice.swap() internally uses mem-swap.  (Isn't creating a variable to hold references while swapping.)
* Why require the _ in `super::sort::<_, BubbleSort>(&mut things);`?
    * We have two type parameters to this function.
    * Since we are trying to name the sorting type so we have to put place holders for all of the other generic types.

# Sorting Algorithms in Rust



## Bubblesort
* Create a Sorter trait - an instance method since sorters themselves won't have any state.
    * A Sorter takes a mutable slice of T
* Create a public function to which you can pass a slice
    * It uses the generic type to determine which Sorter to apply?
* Create a test that utilizes this structure with sort from the rust libraries.

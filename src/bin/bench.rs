use orst::*;
use rand::prelude::*;
use std::cmp::Ordering;
use std::cell::Cell;
use std::rc::Rc;
#[derive(Clone)]
struct SortEvaluator<T> {
    t: T,
    cmps: Rc<Cell<usize>>
}

impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmps.set(self.cmps.get() + 1);
        self.t == other.t
    }
}
impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmps.set(self.cmps.get() + 1);
        self.t.partial_cmp(&other.t)
    }
}

impl<T: Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmps.set(self.cmps.get() + 1);
        self.t.cmp(&other.t)
    }
}

impl<T: Eq> Eq for SortEvaluator<T> {}

fn bench<T: Ord + Clone, S: Sorter>(sorter: S, values: &[SortEvaluator<T>], counter: &Cell<usize>) -> usize {
    let mut values: Vec<_>= values.to_vec();
    counter.set(0);
    sorter.sort(&mut values);
    counter.get()
}

fn main() {
    let counter = Rc::new(Cell::new(0));
    let mut rand = rand::thread_rng();

    for &n in &[0, 1, 10, 100, 1000, 10000] {
        // Run on the same array of values..
        let mut values = Vec::with_capacity(n);
        for _ in 0..n {
            values.push(SortEvaluator {
                t: rand.gen::<usize>(),
                cmps: Rc::clone(&counter),
            })
        }
        for _ in 0..10 {
            // Use the same values but shuffle their order for each of the 10 runs.
            values.shuffle(&mut rand);
            // For each one, reset the counter and sort;
            let took = bench(BubbleSort, &values, &counter);
            println!("{} {} {}", "bubble", n, took);
            let took = bench(InsertionSort {smart: true}, &values, &counter);
            println!("{} {} {}", "insertion-smart", n, took);
            let took = bench(InsertionSort {smart: false}, &values, &counter);
            println!("{} {} {}", "insertion-dumb", n, took);
            let took = bench(QuickSort, &values, &counter);
            println!("{} {} {}", "quick", n, took);
            let took = bench(SelectionSort, &values, &counter);
            println!("{} {} {}", "selection", n, took);
        }
    }
}
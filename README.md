# rust-sort : A collection of sorting algorithms written in Rust.

## Different sorting algorithms :
- Bubble sort
- Bucket sort
- Counting sort
- Heap sort
- Insertion sort
- Merge sort
- Quick sort
- Selection sort

## Examples
Bubble Sort : 
```rust
let mut a = [3, 7, 4, 2, 5];
rust_sort::bubble_sort::bubble_sort(&mut a);
println!("{:?}", a);
assert_eq!(a, [2, 3, 4, 5, 7]);
```
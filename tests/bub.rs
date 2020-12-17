extern crate rust_sort;

#[cfg(test)]
mod tests {
    #[test]
    fn bubble_test() {
        let mut a = [3, 7, 4, 2, 5];
        rust_sort::bubble_sort::bubble_sort(&mut a);
        println!("{:?}", a);
        assert_eq!(a, [2, 3, 4, 5, 7]);
    }
}
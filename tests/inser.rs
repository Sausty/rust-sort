extern crate rust_sort;

#[cfg(test)]
mod tests {
    #[test]
    fn insertion_test() {
        let mut a = [3, 7, 4, 2, 5];
        rust_sort::insertion_sort::insertion_sort(&mut a);
        println!("{:?}", a);
        assert_eq!(a, [2, 3, 4, 5, 7]);
    }
}
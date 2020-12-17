extern crate rust_sort;

#[cfg(test)]
mod tests {
    #[test]
    fn counting_test() {
        let mut a = [3, 7, 4, 2, 5];
        rust_sort::counting_sort::counting_sort(&mut a, 1, 10, |int| *int as usize);
        println!("{:?}", a);
        assert_eq!(a, [2, 3, 4, 5, 7]);
    }
}
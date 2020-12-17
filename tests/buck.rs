extern crate rust_sort;

#[cfg(test)]
mod tests {
    #[test]
    fn bucket_test() {
        let mut a = [3, 7, 4, 2, 5];
        rust_sort::bucket_sort::bucket_sort(&mut a, |int| int / 4);
        println!("{:?}", a);
        assert_eq!(a, [2, 3, 4, 5, 7]);
    }
}

pub fn counting_sort<F, T>(arr: &mut [T], min: usize, max: usize, key: F)
where
    F: Fn(&T) -> usize,
    T: Clone,
{
    let mut prefix_sums = {
        let len = max - min;
        let mut count_arr = Vec::with_capacity(len);
        count_arr.resize(len, 0);

        for value in arr.iter() {
            count_arr[key(value)] += 1;
        }

        count_arr
            .into_iter()
            .scan(0, |state, x| {
                *state += x;
                Some(*state - x)
            })
            .collect::<Vec<usize>>()
    };

    for value in arr.to_vec().iter() {
        let index = key(value);
        arr[prefix_sums[index]] = value.clone();
        prefix_sums[index] += 1;
    }
}
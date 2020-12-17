
pub fn bucket_sort<H, F, T>(arr: &mut [T], hasher: F)

where
    H: Ord,
    F: Fn(&T) -> H,
    T: Ord + Clone,
{
    let mut buckets: Vec<Bucket<H, T>> = Vec::new();

    for value in arr.iter() {
        let hash = hasher(&value);

        let value = value.clone();
        match buckets.binary_search_by(|bucket| bucket.hash.cmp(&hash)) {
            Ok(index) => buckets[index].values.push(value),
            Err(index) => buckets.insert(index, Bucket::new(hash, value)),
        }
    }

    let ret = buckets
        .into_iter()
        .flat_map(|mut bucket| {
            bucket.values.sort();
            bucket.values
        })
        .collect::<Vec<T>>();

    // 4. Clone back to original array.
    arr.clone_from_slice(&ret);
}

/// Bucket to store elements.

struct Bucket<H, T> {
    hash: H,
    values: Vec<T>,
}

impl<H, T> Bucket<H, T> {
    pub fn new(hash: H, value: T) -> Bucket<H, T> {
        Bucket {
            hash,
            values: vec![value],
        }
    }
}
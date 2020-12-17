
pub fn merge_sort(arr: &mut [i32]) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }

    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut ret = arr.to_vec();

    merge(&arr[..mid], &arr[mid..], &mut ret[..]);

    arr.copy_from_slice(&ret);
}

pub fn mergesort_bottom_up(arr: &mut [i32]) {
    let mut width = 1;
    let mut ret = arr.to_vec();
    let len = arr.len();

    while width < len {
        let mut i = 0;
        while i < len {
            let upper = ::std::cmp::min(i + 2 * width, len);
            let mid = ::std::cmp::min(i + width, len);

            merge(&arr[i..mid], &arr[mid..upper], &mut ret[i..upper]);

            arr[i..upper].copy_from_slice(&ret[i..upper]);

            i += 2 * width;
        }
        width *= 2;
    }
}

fn merge(arr1: &[i32], arr2: &[i32], ret: &mut [i32]) {
    let mut left = 0; 
    let mut right = 0;
    let mut index = 0;

    while left < arr1.len() && right < arr2.len() {
        if arr1[left] <= arr2[right] {
            ret[index] = arr1[left];
            index += 1;
            left += 1;
        } else {
            ret[index] = arr2[right];
            index += 1;
            right += 1;
        }
    }

    if left < arr1.len() {
        ret[index..].copy_from_slice(&arr1[left..]);
    }
    if right < arr2.len() {
        ret[index..].copy_from_slice(&arr2[right..]);
    }
}
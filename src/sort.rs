// Bubble Sort
pub fn bubble_sort(vec: &mut Vec<i32>) {
    let n = vec.len();
    for _i in 0..n {
        for j in 0..n - _i - 1 {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
            }
        }
    }
}

// Merge Sort
pub fn merge_sort(vec: &[i32]) -> Vec<i32> {
    if vec.len() <= 1 {
        return vec.to_vec();
    }

    let mid = vec.len() / 2;
    let left = merge_sort(&vec[..mid]);
    let right = merge_sort(&vec[mid..]);

    merge(&left, &right)
}

fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut l = 0;
    let mut r = 0;

    while l < left.len() && r < right.len() {
        if left[l] <= right[r] {
            result.push(left[l]);
            l += 1;
        } else {
            result.push(right[r]);
            r += 1;
        }
    }

    while l < left.len() {
        result.push(left[l]);
        l += 1;
    }

    while r < right.len() {
        result.push(right[r]);
        r += 1;
    }

    return result;
}

// Counting Sort
pub fn counting_sort(vec: &Vec<i32>) -> Vec<i32> {
    Vec::new()
}

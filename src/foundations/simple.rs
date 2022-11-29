#[allow(unused)]
fn insertion_sort(mut array: Vec<i64>) {
    let mut i: usize;
    let mut key: i64;
    for j in 2..array.len() {
        key = array[j];
        // Insert A[j] into the sorted sequence A[1..j-1]
        i = j - 1;
        while i > 0 && array[i] > key {
            array[i + 1] = array[i];
            i -= 1;
        }
        array[i + 1] = key;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_insertion_sort() {
        let _test_array = vec![10,4,5,29,2,52,34,4];
    }
}
use std::thread;


struct MergeSort<T> {
    in_vector: Vec<T>,
}

impl<T: std::cmp::PartialOrd + std::marker::Send> MergeSort<T> {
    pub fn merge_sort(in_vector: &mut Vec<T>) {
        let start = 0;
        let finish = in_vector.len() - 1;
        MergeSort::merge(in_vector, start, finish);
    }

    fn merge(to_sort: &mut Vec<T>, start: usize, end: usize) {
        // Base case
        if start == end {
            return
        }

        let middle = (start + end) / 2;
        let first_half = thread::spawn(|| {
            MergeSort::merge(to_sort, start, middle);
        });
        let second_half = thread::spawn(|| {
            MergeSort::merge(to_sort, middle + 1, end);
        });
        first_half.join().unwrap();
        second_half.join().unwrap();

        let mut temp: Vec<T> = Vec::new();

        let mut left_index = start;
        let mut right_index = middle + 1;

        while left_index <= middle && right_index <= end {
            if to_sort[left_index] <= to_sort[right_index] {
                temp.push(to_sort[left_index]);
                left_index += 1;
            } else {
                temp.push(to_sort[right_index]);
                right_index += 1;
            }
        }

        while left_index <= middle {
            temp.push(to_sort[left_index]);
            left_index += 1;
        }

        while right_index <= end {
            temp.push(to_sort[right_index]);
            right_index += 1;
        }

        for i in 0..temp.len() {
            to_sort[i + start] = temp[i];
        }
    }
}
pub mod sort {
    pub fn bubble_sort(source: &mut Vec<usize>) {
        let length = source.len();

        for _ in 0..length {
            for j in 1..(length - 1) {
                if source[j - 1] > source[j] {
                    source.swap(j - 1, j);
                };
            }
        }
    }

    pub fn selection_sort(source: &mut Vec<usize>) {
        let length = source.len();
        let mut min;
        for i in 0..length {
            min = i;
            for j in (i + 1)..length {
                if source[j] < source[min] {
                    min = j;
                };
            }

            source.swap(min, i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithm;

    #[test]
    fn it_tests_bubble_sort() {
        algorithm::sort::selection_sort(&mut vec![1, 4, 2, 3, 7]);
        assert_eq!(source, vec![1, 2, 3, 4, 7]);
    }

    #[test]
    fn it_tests_selection_sort() {
        algorithm::sort::selection_sort(&mut vec![1, 5, 2, 3, 7]);
        assert_eq!(source, vec![1, 2, 3, 5, 7]);
    }
}

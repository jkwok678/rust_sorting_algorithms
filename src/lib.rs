pub fn bubble_sort_v1(array: &mut Vec<i16>) {
    for i in 0..array.len() {
        for j in 0..array.len() - i - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
}

pub fn bubble_sort_v2(array: &mut Vec<i16>) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..array.len() {
            if array[i - 1] > array[i] {
                array.swap(i - 1, i);
                swapped = true;
            }
        }
    }
}

pub fn bubble_sort_v3(array: &mut Vec<i16>) {
    for i in 0..array.len() {
        let mut swapped = false;
        for j in 0..array.len() - i - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

pub fn selection_sort(array: &mut Vec<i16>) {
    for i in 0..array.len() {
        let mut min_index = i;
        for j in i..array.len() {
            if array[j] < array[min_index] {
                min_index = j;
            }
        }
        array.swap(i, min_index);
    }
}

pub fn insertion_sort(array: &mut Vec<i16>) {
    for i in 0..array.len() {
        let key = array[i];
        let mut j = i;
        while (j >= 0) & (key < array[j]) {
            array[j + 1] = array[j];
            j -= 1;
            array[j + 1] = key;
        }
    }
}

pub fn check_in_order(array: &Vec<i16>) -> bool {
    return array.windows(2).all(|x| x[0] <= x[1]);
}

#[cfg(test)]
mod tests {
    use crate::{bubble_sort_v1, check_in_order, selection_sort};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn bubble_sort_1_works() {
        let mut array: Vec<i16> = vec![20, 15, 2, 1, 4, 9, 11, 14, 17, 16];
        bubble_sort_v1(&mut array);
        assert_eq!(check_in_order(&array), true);
    }

    #[test]
    fn selection_sort_works() {
        let mut array: Vec<i16> = vec![20, 15, 2, 1, 4, 9, 11, 14, 17, 16];
        selection_sort(&mut array);
        assert_eq!(check_in_order(&array), true);
    }
}

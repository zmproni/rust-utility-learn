use utility_learn::array::search::binary_search::search;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let elem = 5;
        assert_eq!(Some(4), search(&arr, &elem));
    }

    #[test]
    fn test_binary_search_right() {
        let arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let elem = 10;
        assert_eq!(Some(9), search(&arr, &elem));
    }

    #[test]
    fn test_binary_search_left() {
        let arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let elem = 1;
        assert_eq!(Some(0), search(&arr, &elem));
    }

    #[test]
    fn test_binary_search_mid_left() {
        let arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let elem = 2;
        assert_eq!(Some(1), search(&arr, &elem));
    }

    #[test]
    fn test_binary_search_mid_right() {
        let arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let elem = 8;
        assert_eq!(Some(7), search(&arr, &elem));
    }

    #[test]
    fn test_binary_search_off_mid() {
        let arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let elem = 6;
        assert_eq!(Some(5), search(&arr, &elem));
    }

    #[test]
    fn test_binary_search_not_present_greater() {
        let arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let elem = 11;
        assert_eq!(None, search(&arr, &elem));
    }


    #[test]
    fn test_binary_search_not_present_smaller() {
        let arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let elem = 0;
        assert_eq!(None, search(&arr, &elem));
    }


    #[test]
    fn test_binary_search_not_present_between() {
        let arr: [i32; 9] = [1, 2, 3, 4, 6, 7, 8, 9, 10];
        let elem = 5;
        assert_eq!(None, search(&arr, &elem));
    }

}


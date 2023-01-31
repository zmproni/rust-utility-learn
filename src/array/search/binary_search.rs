use std::cmp::Ordering;

pub fn search<T:Eq+Ord>(arr: &[T], elem: &T) -> Option<usize> {    
    let size = arr.len();
    let mut rindex = size as i32 - 1;
    let mut lindex = 0;
    
    while lindex <= rindex {
        let mindex = ((rindex - lindex) / 2) + lindex;
        let index = mindex as usize;

        match &elem.cmp(&arr[index]) {
            Ordering::Equal => return Some(index),
            Ordering::Greater => lindex = mindex + 1,
            Ordering::Less => rindex = mindex - 1,
        }
    }

    return None;
}
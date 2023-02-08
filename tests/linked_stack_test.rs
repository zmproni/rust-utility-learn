use utility_learn::data_structures::linked_stack::List;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_list_and_pop() {
        let mut list = List::<i32>::new();
        
        list.push(1);
        list.push(3);
        list.push(5);
        
        assert_eq!(list.pop(), Option::Some(5));
        assert_eq!(list.pop(), Option::Some(3));
        assert_eq!(list.pop(), Option::Some(1));
        assert_eq!(list.pop(), Option::None); 
    } 
}
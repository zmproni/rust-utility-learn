use utility_learn::data_structures::better_linked_stack::List;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peek_empty() {
        let list = List::<i32>::new();

        assert_eq!(list.peek(), Option::None);
    }
    
    #[test]
    fn peek_one_element() {
        let mut list = List::<i32>::new();    
        list.push(5);
        
        assert_eq!(list.peek(), Option::Some(&5));
    }

    #[test]
    fn peek_many_elements() {
        let mut list = List::<i32>::new();    
        list.push(5);
        list.push(1);
        list.push(229);
        list.push(53);
        list.push(15);

        
        assert_eq!(list.peek(), Option::Some(&15));
    }
}
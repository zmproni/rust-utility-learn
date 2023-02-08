// https://rust-unofficial.github.io/too-many-lists/first-layout.html
use std::mem;
struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

impl <T> List<T> {

    pub fn new() -> Self {
        List { head: Link::None }
    }

    pub fn push(&mut self, elem: T) {
        let node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::None)
        });
        self.head = Link::Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        return match mem::replace(&mut self.head, Link::None) {
            Link::None => Option::None,
            Link::Some(node) => {
                self.head = node.next;
                Some(node.elem)
            },
        }
    }
}

impl <T>Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::None);
        while let Link::Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::None);
        }
    }
}
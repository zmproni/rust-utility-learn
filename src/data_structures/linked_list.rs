// https://rust-unofficial.github.io/too-many-lists/first-layout.html
use std::mem;
struct Node<T> {
    elem: T,
    next: Link<T>,
}

enum Link<T> {
    Empty,
    More(Box<Node<T>>)
}

pub struct List<T> {
    head: Link<T>,
}

impl <T> List<T> {

    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: T) {
        let node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty)
        });
        self.head = Link::More(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        return match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => Option::None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            },
        }
    }
}
use std::{fmt::Debug, usize};

// I think that must be crazy!
type YLink<T> = Option<Box<YListNode<T>>>;

#[derive(Clone, Debug)]
pub struct YListNode<T> {
    val: T,
    next: YLink<T>,
}

// TODO
impl<T> YListNode<T> {
    fn len(&self) -> usize {
        match self.next {
            None => 1,
            Some(ref next) => 1 + next.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct YLinkedList<T> {
    head: YLink<T>,
}

impl<T> YLinkedList<T>
where
    T: Debug,
{
    pub fn new() -> Self {
        YLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        match self.head {
            None => 0,
            Some(ref head) => head.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    //头插
    pub fn append(&mut self, elem: T) {
        let new_head = Box::new(YListNode {
            val: elem,
            next: self.head.take(),
        });
        self.head = Some(new_head);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }

    pub fn print(self) {
        let node = self.head;
        let mut cur = node;
        let mut vec = Vec::new();
        while let Some(mut tmp) = cur.take() {
            let next = tmp.next.take();
            vec.push(tmp.val);
            cur = next;
        }
        vec.reverse();
        // for item in vec.iter(){
        //     println!("{:#?}",item);
        // }
        println!("{:?}", vec);
    }
}

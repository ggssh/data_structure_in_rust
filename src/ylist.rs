use std::usize;

type YNextNode<T> = Option<Box<YListNode<T>>>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct YListNode<T> {
    pub val: T,
    // pub next: Option<Box<YListNode<T>>>,
    pub next: YNextNode<T>,
}

impl<T> YListNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        YListNode { next: None, val }
    }

    fn get<'a>(&'a mut self, index: usize) -> Option<&'a mut Self> {
        if index == 0 {
            return Some(self);
        }
        if let Some(x) = self.get_next() {
            x.get(index - 1)
        } else {
            None
        }
    }

    pub fn get_next<'a>(&'a mut self) -> Option<&'a mut Self> {
        if let Some(ref mut x) = self.next {
            return Some(x);
        }
        None
    }

    pub fn set_next(&mut self, node: Option<Self>) {
        self.next = None;
        if let Some(x) = node {
            self.next = Some(Box::new(x));
        }
    }

    pub fn get_last<'a>(&'a mut self) -> &'a mut Self {
        if let Some(ref mut x) = self.next {
            x.get_last()
        } else {
            self
        }
    }

    fn get_last_immutable<'a>(&'a self) -> &'a Self {
        if let Some(ref x) = self.next {
            x.get_last_immutable()
        } else {
            self
        }
    }

    pub fn get_value(&self) -> &T {
        let value = self.get_value();
        value
    }

    pub fn push(&mut self, val: T) {
        let new_node = YListNode::new(val);
        self.get_last().set_next(Some(new_node));
    }
}

#[derive(Debug, Clone)]
pub struct YLinkedList<T> {
    len: usize,
    // pub next: Option<Box<YListNode<T>>>,
    pub next: YNextNode<T>,
}

impl<T> YLinkedList<T> {
    pub fn new() -> Self {
        YLinkedList { len: 0, next: None }
    }

    fn get<'a>(&'a mut self, index: usize) -> Option<&'a mut YListNode<T>> {
        if index > self.len || index == 0 {
            return None;
        }
        let node = self.get_next()?;
        if index == 1 {
            return Some(node);
        }
        node.get(index - 1)
    }
    pub fn get_next<'a>(&'a mut self) -> Option<&'a mut YListNode<T>> {
        if let Some(ref mut x) = self.next {
            return x.get_next();
        }
        None
    }

    fn get_last<'a>(&'a mut self) -> Option<&'a mut YListNode<T>> {
        if let Some(ref mut x) = self.next {
            Some(x.get_last())
        } else {
            None
        }
    }

    fn get_last_immutable<'a>(&'a self) -> Option<&'a YListNode<T>> {
        if let Some(ref x) = self.next {
            Some(x.get_last_immutable())
        } else {
            None
        }
    }

    pub fn get_last_value(&self) -> Option<&T> {
        match self.len {
            0 => {
                return None;
            }
            _ => Some(self.get_last_immutable()?.get_value()),
        }
    }

    pub fn push(&mut self, val: T) {
        match self.len {
            0 => {
                self.next = Some(Box::new(YListNode::new(val)));
            }
            _ => {
                if let Some(ref mut x) = self.get_last() {
                    x.push(val);
                }
            }
        }
        self.len += 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn pop(&mut self) {
        match self.len() {
            0 => {
                return ();
            }
            _ => {
                self.len -= 1;
                return ();
            }
        }
    }
}

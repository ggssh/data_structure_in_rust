mod ylist;

use std::collections::LinkedList;
#[cfg(test)]
mod tests {
    use crate::ylist::YLinkedList;
    #[test]
    fn list_test() {
        let mut list = YLinkedList::<i32>::new();
        for i in 0..5 {
            list.append(i + 20);
        }
        list.print();
    }
}

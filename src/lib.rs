mod ylist;

#[cfg(test)]
mod tests {
    use crate::ylist::*;
    #[test]
    fn list_test() {
        let mut l : YLinkedList<i32> = YLinkedList::new();
        l.push(32);
        println!("{:#?}",&l.get_next().get_value()?);

    }
}

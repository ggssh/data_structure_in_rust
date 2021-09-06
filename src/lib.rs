pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    //获取一个新结点
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    //返回最后一个结点的可变借用
    fn get_last_mut(&mut self) -> &mut Self {
        if let Some(ref mut boxNode) = self.next  {
            return boxNode.get_last_mut();
        }else {
            return self;
        }
    }

    //追加结点
    pub fn append(&mut self, val: i32){
        let newNode = ListNode::new(val);
        self.get_last_mut().next = Some(Box::new(newNode));
    }

    //删除结点
    pub fn delete(&mut self,val: i32){
        let tempHead = self;
        if let val = tempHead.val {
            self = tempHead.next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_new() {
        let val = 3;
        assert_eq!(val, ListNode::new(val).val);
    }
}

// use std::cell::RefCell;
// use std::error::Error;
// use std::rc::Rc;
// use std::boxed::Box;
//
// use anyhow::Ok;
//
// #[derive(Clone)]
// struct TreeNode {
//     val: Option<i32>,
//     left: Option<Rc<RefCell<TreeNode>>>,
//     right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     fn new() -> Self {
//         Self { val: None, left: None, right: None }
//     }
//
//     fn new_by_val(val: i32) -> Self {
//         Self { val: Some(val), left: None, right: None }
//     }
//
//     fn get_immutable_rf(&self) -> &Self {
//         self
//     }
//
//     // insert left < mid < right
//     fn insert(&mut self, val: i32) -> anyhow::Result<()> {
//         if self.val.is_none() {
//             self.val = Some(val);
//             return Ok(())
//         }
//
//         let mut curr = Rc::new(RefCell::new(self.get_immutable_rf().clone()));
//         loop {
//             if val < curr.borrow().val.unwrap() {
//                 match curr.borrow().left {
//                     Some(_) => {
//                         curr = curr.borrow().left.unwrap();
//                     }
//                     None => {
//                         curr.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new_by_val(val))));
//                     }
//                 }
//             } else {
//                 todo!()
//             }
//         }
//
//         Ok(())
//     }
//     
//     fn search() -> anyhow::Result<()> {
//         todo!()
//     }
//     
//     fn delete() -> anyhow::Result<()> {
//         todo!()
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_insert_node() {
//
//     }
//     
//     #[test]
//     fn test_search_node() {
//
//     }
//     
//     #[test]
//     fn test_delete_node() {
//
//     }
// }

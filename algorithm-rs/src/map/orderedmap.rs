use std::{cell::{Ref, RefCell}, collections::HashMap, hash::Hash, ops::Deref, rc::Rc};

#[derive(Debug)]
struct OrderedMap<T> {
    ordered: Vec<Rc<RefCell<T>>>,
    data: HashMap<String, Rc<RefCell<T>>>
}

impl<T> OrderedMap<T> {
    fn new() -> Self {
        Self {
            ordered: vec![],
            data: HashMap::new(),
        }
    }

    fn insert(&mut self, key: impl Into<String>, elem: T) -> anyhow::Result<()> {
        let key = key.into();
        match self.data.get(&key) {
            Some(old_elem) => {
                *old_elem.borrow_mut() = elem;
            }
            None => {
                let elem = Rc::new(RefCell::new(elem));
                self.ordered.push(elem.clone());
                self.data.insert(key, elem);
            }
        } 

        Ok(())
    }

    fn get(&self, key: impl Into<String>) -> Option<Ref<T>> {
        if let Some(elem) = self.data.get(&key.into()) {
            Some(elem.borrow())
        } else {
            None
        }
    }

    fn to_vec(&self) -> Vec<Ref<T>> {
        if self.ordered.len() == 0 {
            return vec![];
        }
        
        let mut array = Vec::with_capacity(self.ordered.len());
        for elem in self.ordered.iter() {
            array.push(elem.borrow());
        }
        array
    }
}

#[cfg(test)]
mod tests {
    use super::OrderedMap;

    #[test]
    fn test_insert() {
        let mut m = OrderedMap::new();
        m.insert("lily", 18).unwrap();
        m.insert("green", 19).unwrap();
        m.insert("white", 17).unwrap();

        for item in m.to_vec() {
            println!("{:?}", item);
        }
    }
}

struct NodeItem<'a, V:'a + Ord> {
  value : V,
  next : Box<Option<NodeItem<'a,V>>>
}

impl <'a, V:'a + Ord> NodeItem<'a,V> {

  fn new(value : V) -> NodeItem<'a,V> {
    NodeItem { value : value, next : box None }
  }

  fn add(&mut self, value : V) {

    match self.value.cmp(&value) {
      Less => {
        self.next = box Some(NodeItem {value: self.value, next : self.next });
        self.value = value;
      },
      Equal | Greater => {
        match *self.next {
          Some(ref mut next) => next.add(value),
          None => self.next = box Some(NodeItem::new(value)),
        }
      },
    }

  }

}

#[cfg(test)]
mod test {



}

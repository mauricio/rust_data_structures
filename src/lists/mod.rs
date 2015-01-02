enum Node<'a, V:'a + PartialOrd> {
  Cons( V, Box<Node<'a, V>>),
  Nil,
}

impl <'a, V:'a + PartialOrd> Node<'a,V> {

  fn add(&mut self, element : V) {

    match self {

    }

  }

}

pub struct LinkedList<'a, T: 'a + PartialOrd> {
  head : Node<'a,T>
}

impl <'a,T : 'a + PartialOrd> LinkedList<'a,T> {

  fn new() -> LinkedList<'a,T> {
    LinkedList{ head : Node::Nil }
  }

  fn add(&mut self, element : T) -> &mut LinkedList<'a,T> {




    self
  }

}

#[cfg(test)]
mod test {



}

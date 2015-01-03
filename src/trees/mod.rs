struct PrefixNode<'a, T: 'a> {
  item : char,
  value : Box<Option<T>>,
}

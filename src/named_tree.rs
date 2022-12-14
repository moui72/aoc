pub struct NamedNode<T> {
    name: String,
    value: u32,
    children: Vec<NamedNode>,
    parent: Option<NamedNode>,
}

impl<T> NamedNode<T> {
    pub fn new(name: String, value: i32, parent: Option<NamedNode>) -> Self {
        NamedNode {
            name: name,
            value: value,
            parent: parent,
        }
    }
}

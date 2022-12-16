use std::{collections::HashMap, rc::Rc};

pub struct NamedNode {
    name: String,
    value: u32,
    children: HashMap<String, NamedNode>,
}

impl NamedNode {
    pub fn new(name: String, value: u32) -> Self {
        let children = HashMap::new();
        NamedNode {
            name,
            value,
            children,
        }
    }

    pub fn add_child(&mut self, name: String, value: u32) {
        let mut node: NamedNode = NamedNode::new(name.clone(), value);
        self.children.insert(name.clone(), node);
    }

    pub fn get_total_size(&self) -> u32 {
        let mut to_visit = Vec::from([self]);
        let mut total_size: u32 = 0;
        while to_visit.len() > 0 {
            let node: &NamedNode = to_visit.pop().unwrap();
            for (_name, child) in &node.children {
                to_visit.push(&child)
            }
            total_size += node.value;
        }
        return total_size;
    }

    pub fn count(&self) -> u32 {
        let mut to_visit = Vec::from([self]);
        let mut count: u32 = 0;
        while to_visit.len() > 0 {
            let node: &NamedNode = to_visit.pop().unwrap();
            for (_name, child) in &node.children {
                to_visit.push(&child)
            }
            count += 1;
        }
        return count;
    }

    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    pub fn increment_value(&mut self, amount: u32) {
        self.value += amount
    }

    pub fn peek_all(&self, path_to_parent: Option<String>) -> Vec<String> {
        let mut paths = vec![];
        let path_to_node;
        match path_to_parent {
            Some(path) => {
                path_to_node = path.clone() + "/" + &self.name;
            }
            None => {
                path_to_node = self.name.clone();
            }
        }
        paths.push(path_to_node.clone());
        for (_, child) in &self.children {
            paths.append(&mut child.peek_all(Some(path_to_node.clone())))
        }
        return paths;
    }

    pub fn get_by_path(
        &mut self,
        path_to_match: &str,
        path_to_parent: Option<String>,
    ) -> Option<&mut NamedNode> {
        let mut path_to_node: String = path_to_parent.clone().unwrap_or("".to_string()).to_owned();
        if &self.name != "~" {
            path_to_node += "/"
        }
        path_to_node += &self.name;
        println!("Visited {}", path_to_node);
        if path_to_match == path_to_node {
            return Some(self);
        } else {
            for (_, child) in &mut self.children {
                match child.get_by_path(path_to_match, Some(path_to_node.clone())) {
                    Some(node) => return Some(node),
                    None => {}
                };
            }
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_make_tree() {
        let mut root = NamedNode::new("~".to_string(), 0);
        assert!(!root.has_children());
        root.add_child("file".to_string(), 100);
        assert_eq!(root.get_total_size(), 100);
        root.children
            .get_mut("file")
            .unwrap()
            .add_child("file2".to_string(), 10);
        assert_eq!(root.get_total_size(), 110);
        root.add_child("file3".to_string(), 20);
        assert_eq!(root.get_total_size(), 130);
        assert_eq!(root.get_by_path("~/file/file2", None).unwrap().value, 10);
        println!("---");
        assert_eq!(root.get_by_path("~/file", None).unwrap().value, 100);
        println!("---");
        assert_eq!(root.get_by_path("~/file3", None).unwrap().value, 20);
        println!("---");
        assert_eq!(root.get_by_path("~", None).unwrap().value, 0);
        assert_eq!(
            root.peek_all(None).sort(),
            vec!["~", "~/file3", "~/file", "~/file/file2"].sort()
        );
        assert_eq!(root.count(), 4);
    }
}

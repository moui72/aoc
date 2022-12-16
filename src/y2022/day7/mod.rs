use itertools::join;

use crate::named_tree;
use crate::util;

fn up_one(cwd: &str) -> String {
    let segments: Vec<&str> = cwd.split('/').collect();
    match segments.len() {
        2.. => {
            return join(segments.split_last().unwrap_or((&"", &[""])).1.iter(), "/");
        }
        _ => {
            println!("above root?");
            return "/".to_string();
        }
    }
}

fn cd(cwd: &str, input: &str) -> String {
    if input.starts_with('/') {
        return input.clone().to_owned();
    } else if input.starts_with('.') {
        return up_one(cwd);
    }
    if cwd.ends_with('/') {
        return cwd.to_owned() + input;
    }
    cwd.to_owned() + "/" + input
}

fn add_file(root: &mut named_tree::NamedNode, cwd: &String, name: &str, size: u32) {
    let pref;
    if cwd.starts_with("/") {
        pref = "~".to_string();
    } else {
        pref = "~/".to_string();
    }
    let path;
    if cwd.ends_with('/') {
        path = pref.clone() + cwd + name;
    } else {
        path = pref.clone() + cwd + "/" + name;
    }
    println!("prefix {}\ncwd {}\ngetting path {}", &pref, cwd, &path);
    let mut node: &mut named_tree::NamedNode = root.get_by_path(&path, None).unwrap();
    node.add_child(name.clone().to_string(), size)
}

pub fn solve() {
    let mut root: named_tree::NamedNode = named_tree::NamedNode::new("~".to_string(), 0);
    let input = util::load_input_file("src/y2022/day6/input.txt");
    println!("Recieved {} characters", input.len());
    let mut cwd = "".to_string();
    for line in input.split('\n') {
        match line.split(' ').collect::<Vec<&str>>().split_first() {
            Some((&"$", cmd)) => match cmd.len() {
                2 => {
                    cwd = cd(&cwd, cmd[1]);
                }
                _ => {}
            },
            Some((size, name)) => add_file(
                &mut root,
                &cwd,
                name[0],
                size.parse::<u32>().or::<Result<u32, u32>>(Ok(0)).unwrap(),
            ),
            None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_cd() {
        let mut cwd = "/".to_string();
        cwd = cd(&cwd, "/");
        assert_eq!(cwd, "/".to_string());
        cwd = cd(&cwd, "potato");
        assert_eq!(cwd, "/potato".to_string());
        cwd = cd(&cwd, "fizz");
        assert_eq!(&cwd, &"/potato/fizz".to_string());
        cwd = cd(&cwd, "/new");
        assert_eq!(&cwd, &"/new".to_string());
        cwd = cd(&cwd, "/a/b/c/d");
        assert_eq!(&cwd, &"/a/b/c/d".to_string());
        cwd = cd(&cwd, "..");
        assert_eq!(&cwd, &"/a/b/c".to_string());
        cwd = cd(&cwd, "/");
        cwd = cd(&cwd, "..");
        cwd = cd(&cwd, "..");
        assert_eq!(cwd, "/".to_string());
    }

    #[test]
    fn test_add_file() {
        let mut root: named_tree::NamedNode = named_tree::NamedNode::new("~".to_string(), 0);
        println!("{:?}", root.peek_all(None));

        let mut cwd = "/".to_string();
        add_file(&mut root, &cwd, "a", 0);
        println!("{:?}", root.peek_all(None).sort(),);
        cwd = cd(&cwd, "/a");
        add_file(&mut root, &cwd, "c.txt", 300);
        assert_eq!(root.get_total_size(), 300)
    }
}

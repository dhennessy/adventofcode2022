use regex::Regex;
use std::{rc::Rc, str::FromStr, cell::RefCell};

static INPUT: &str = include_str!("../../input.txt");

#[derive(Clone, Debug, PartialEq)]
struct Node {
    name: String,
    size: Option<usize>,
    parent: Option<Rc<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>
}

impl Node {
    fn new() -> Node {
        Node { name: "/".to_string(), size: None, parent: None, children: Vec::new() }
    }

    fn add_child(&mut self, child: Rc<RefCell<Node>>) {
        // println!("ADD {} += {}", self.name, child.borrow().name);
        // child.borrow_mut().parent = Rc::clone(&self);
        self.children.push(child);
    }
}

impl FromStr for Node {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (s, n) = input.split_once(" ").unwrap();
        let size = s.parse::<usize>().ok();
        Ok(Node { name: n.to_string(), size: size, parent: None, children: Vec::new() })
    }
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let tree = parse_tree(input);
    let mut size = 0;
    for dir in all_dirs(tree) {
        let dir_size = node_size(dir);
        // println!("SIZE {} => {}", node.borrow().name, dir_size);
        if dir_size <= 100000 { 
            size += dir_size;
        }
    }
    size
}

fn part2(input: &str) -> usize {
    let tree = parse_tree(input);
    let needed = 70000000 - 30000000;
    let used = node_size(Rc::clone(&tree));

    let mut sizes: Vec<usize> = all_dirs(Rc::clone(&tree))
        .iter()
        .map(|n| node_size(Rc::clone(&n)))
        .collect();
    sizes.sort();

    for s in sizes {
        // println!("SIZE: {}", s);
        if used - s < needed {
            return s;
        }
    }
    0
}

fn dump_node(node: Rc<RefCell<Node>>, indent: &str) {
    let info = match node.borrow().size {
        Some(s) => s,
        None => 0
    };
    println!("{} {} {}", indent, node.borrow().name, info);
    let next_indent = &format!("  {}", indent);
    for child in node.borrow().children.clone() {
        dump_node(Rc::clone(&child), next_indent);
    }
}

fn node_size(node: Rc<RefCell<Node>>) -> usize {
    let mut size = 0;
    for child in node.borrow().children.clone() {
        // println!("CONSIDER: {}", child.borrow().name);
        size += match child.borrow().size {
            Some(s) => s,
            None => node_size(Rc::clone(&child))
        }
    }
    // println!("SIZE: {} = {}", node.borrow().name, size);
    size
}

fn all_dirs(node: Rc<RefCell<Node>>) -> Vec<Rc<RefCell<Node>>> {
    let mut vec = Vec::new();
    // println!("ENUM: {}", node.borrow().name);
    vec.push(Rc::clone(&node));
    for child in node.borrow().children.clone() {
        if let None = child.borrow().size {
            vec.append(&mut all_dirs(Rc::clone(&child)));
        }
    }
    vec
}

fn parse_tree(input: &str) -> Rc<RefCell<Node>> {
    let root = Rc::new(RefCell::new(Node::new()));
    let mut cwd = Rc::clone(&root);

    let re = Regex::new(r"\$ ([a-z]+) ?(.*)").unwrap();

    for line in input.lines() {
        if line.starts_with("$") {
            // println!("CMD: {}", line);
            let c = re.captures(line).unwrap();
            match &c[1] {
                "ls" => (),
                "cd" => {
                    let _cwd: Rc<RefCell<Node>> = Rc::clone(&cwd);
                    match &c[2] {
                        "/" => { 
                            cwd = Rc::clone(&root) 
                        },
                        ".." => { 
                            cwd = Rc::clone(_cwd.borrow().parent.as_ref().unwrap()) 
                        },
                        &_ => {
                            for child in _cwd.borrow().children.clone() {
                                if child.borrow().name == &c[2] {
                                    cwd = Rc::clone(&child);
                                    break;
                                }
                            }
                        }
                    }    
                    // println!("CWD: {}", cwd.borrow().name);    
                    // dump_node(Rc::clone(&cwd), ">");            
                },
                &_ => todo!()
            }
        } else {
            // println!("PARSE: {}", line);
            let mut n = Node::from_str(line).unwrap();
            n.parent = Some(Rc::clone(&cwd));
            cwd.borrow_mut().add_child(Rc::new(RefCell::new(n)));
        }
        // dump_node(Rc::clone(&root), "");
    }
    root
}

#[test]
fn test_parse_node() {
    let n = Node::from_str("42 towel").unwrap();
    assert_eq!(n.name, "towel");
    assert_eq!(n.size, Some(42));

    let mut root = Node::from_str("dir /").unwrap();
    root.add_child(Rc::new(RefCell::new(n)));
    assert_eq!(root.children.len(), 1);
}

#[cfg(test)]
static SAMPLE: &str = include_str!("../../sample.txt");

#[test]
fn test_parse_tree() {
    let root = parse_tree(SAMPLE);
    assert_eq!(root.borrow().name, "/");
    assert_eq!(root.borrow().children.len(), 4);
}

#[test]
fn test_node_size() {
    let root = parse_tree(SAMPLE);
    dump_node(Rc::clone(&root), "");
    assert_eq!(node_size(root), 48381165);
}

#[test]
fn test_part1() {
    assert_eq!(part1(SAMPLE), 95437);
    assert_eq!(part1(INPUT), 1491614);
}

#[test]
fn test_part2() {
    assert_eq!(part2(SAMPLE), 24933642);
    assert_eq!(part2(INPUT), 6400111);
}
use std::fmt::Debug;

#[derive(Debug)]
pub struct Tree<T>(Option<Box<Node<T>>>);

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    h: i8,
    left: Tree<T>,
    right: Tree<T>,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn height(&self) -> i8 {
        match self.0 {
            Some(ref n) => n.h,
            None => 0,
        }
    }

    pub fn set_height(&mut self) {
        if let Some(ref mut n) = self.0 {
            n.h = 1 + std::cmp::max(n.left.height(), n.right.height());
        }
    }

    pub fn rot_left(&mut self) {
        self.0 = self.0.take().map(|n| n.rot_left());
    }

    pub fn rot_right(&mut self) {
        self.0 = self.0.take().map(|n| n.rot_right());
    }
}

impl<T: PartialOrd> Tree<T> {
    pub fn add_sorted(&mut self, data: T) {
        let rot_dir = match self.0 {
            Some(ref mut n) => {
                if data < n.data {
                    n.left.add_sorted(data);
                    if n.left.height() - n.right.height() > 1 {
                        1
                    } else {
                        0
                    }
                } else {
                    n.right.add_sorted(data);
                    if n.right.height() - n.left.height() > 1 {
                        -1
                    } else {
                        0
                    }
                }
            }
            None => {
                self.0 = Some(Box::new(Node::new(data)));
                0
            }
        };
        match rot_dir {
            1 => self.rot_right(),
            -1 => self.rot_left(),
            _ => self.set_height(),
        }
    }
}

impl<T: Debug> Tree<T> {
    pub fn print_sorted(&self, depth: i32) {
        if let Some(ref n) = self.0 {
            n.left.print_sorted(depth + 1);
            let mut dp_str = "".to_string();
            for _ in 0..depth {
                dp_str.push('.');
            }
            println!("{}{:?}", dp_str, n.data);
            n.right.print_sorted(depth + 1);
        }
    }
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            h: 0,
            left: Tree::new(),
            right: Tree::new(),
        }
    }

    pub fn rot_left(mut self) -> Box<Self> {
        let mut res = match self.right.0.take() {
            Some(n) => n,
            None => return Box::new(self),
        };
        self.right = Tree(res.left.0.take());
        self.right.set_height();
        res.left = Tree(Some(Box::new(self)));
        res.left.set_height();
        res.h = 1 + std::cmp::max(res.left.height(), res.right.height());
        res
    }

    pub fn rot_right(mut self) -> Box<Self> {
        let mut res = match self.left.0.take() {
            Some(n) => n,
            None => return Box::new(self),
        };
        self.left = Tree(res.right.0.take());
        self.left.set_height();
        res.right = Tree(Some(Box::new(self)));
        res.right.set_height();
        res.h = 1 + std::cmp::max(res.left.height(), res.right.height());
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut t = Tree::new();
        for v in 1..1000 {
            t.add_sorted(v);
        }
        t.print_sorted(0);
    }

    #[test]
    fn test_node_rot_left() {
        let mut t = Tree::new();
        for v in vec![5, 3, 4, 2, 7, 6, 8] {
            t.add_sorted(v);
        }
        t.print_sorted(0);
    }
}

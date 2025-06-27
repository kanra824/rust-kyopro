use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Clone, Debug)]
pub struct Trie {
    cnt: i64,
    end: i64,
    child: Vec<Option<Rc<RefCell<Trie>>>>,
    par: Option<Weak<RefCell<Trie>>>,
    val: usize,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            cnt: 0,
            end: 0,
            child: vec![None; 26],
            par: None,
            val: usize::MAX,
        }
    }

    pub fn new_child() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Trie::new()))
    }

    pub fn add_recursive(node: Rc<RefCell<Self>>, s: &Vec<char>, depth: usize) {
        node.borrow_mut().cnt += 1;
        if depth >= s.len() {
            node.borrow_mut().end += 1;
            return;
        }
        
        let idx = s[depth] as usize - 'a' as usize;
        if node.borrow().child[idx].is_none() {
            let new_child = Self::new_child();
            new_child.borrow_mut().val = idx;
            new_child.borrow_mut().par = Some(Rc::downgrade(&node));
            node.borrow_mut().child[idx] = Some(new_child);
        }
        
        let child = node.borrow().child[idx].as_ref().unwrap().clone();
        Self::add_recursive(child, s, depth + 1);
    }
    
    pub fn has_nth(&self, idx: usize) -> bool {
        !self.child[idx].is_none()
    }

    pub fn nxt(now: Rc<RefCell<Trie>>, idx: usize) -> Option<Rc<RefCell<Trie>>> {
        now.borrow().child[idx].clone()
    }

    // 最小共通接頭辞の長さの総和
    pub fn calc_lcp_sum(&self, s: &Vec<char>) -> i64 {
        let mut now = Rc::new(RefCell::new(self.clone()));
        let mut res = 0;
        let mut depth = 0;
        let mut prevcnt = now.borrow().cnt;
        for i in 0..s.len() {
            let idx = s[i] as usize - 'a' as usize;
            if now.borrow().child[idx].is_none() {
                res += now.borrow().cnt * depth;
                return res;
            }
            let nxt = now.borrow_mut().child[idx].as_ref().unwrap().clone();
            now = nxt;
            res += (prevcnt - now.borrow().cnt) * depth;
            depth += 1;
            prevcnt = now.borrow().cnt;
        }
        res += now.borrow().cnt * depth;
        res
    }
}
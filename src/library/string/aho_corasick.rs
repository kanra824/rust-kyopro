use std::collections::{HashMap, VecDeque};
// https://naoya-2.hatenadiary.org/entry/20090405/aho_corasick

#[derive(Clone, Debug)]
struct State {
    id: usize,
    next: HashMap<usize, usize>,
    pattern: Option<usize>,
}

impl State {
    fn new(id: usize) -> Self {
        State {
            id,
            next: HashMap::new(),
            pattern: None,
        }
    }

    fn has_key(&self, x: usize) -> bool {
        self.next.contains_key(&x)
    }
}

#[derive(Clone, Debug)]
pub struct AhoCorasick {
    node: Vec<State>,
    failure: Vec<usize>,
}

impl AhoCorasick {
    pub fn new(patterns: &Vec<Vec<usize>>) -> Self {
        let node = vec![State::new(0)];
        let mut ahocora = AhoCorasick {
            node,
            failure: vec![],
        };
        ahocora.make_goto(patterns);
        ahocora.make_failure();
        ahocora
    }

    fn make_goto(&mut self, patterns: &Vec<Vec<usize>>) {
        // Trie 木をつくる
        for i in 0..patterns.len() {
            let mut cur = self.node[0].id;
            for &x in &patterns[i] {
                if !self.node[cur].has_key(x) {
                    let mut new_node = State::new(self.node.len());
                    self.node[cur].next.insert(x, new_node.id);
                    self.node.push(new_node);
                }
                cur = self.node[cur].next[&x];
            }
            self.node[cur].pattern = Some(i);
        }
    }

    fn make_failure(&mut self) {
        let mut failure = vec![0; self.node.len()];
        let mut que = VecDeque::new();
        que.push_back(0);
        while !que.is_empty() {
            let s = que.pop_front().unwrap();
            for &x in self.node[s].next.keys() {
                let nxt = self.goto(s, x);
                if let Some(nxt) = nxt {
                    que.push_back(nxt);
                    if s != 0 {
                        let mut f = failure[s];
                        while self.goto(f, x).is_none() {
                            f = failure[f];
                        }
                        let res = self.goto(f, x);
                        if let Some(val) = res {
                            failure[nxt] = val;
                        }
                    }
                }
            }
        }
        self.failure = failure;
    }

    pub fn goto(&self, s: usize, x: usize) -> Option<usize> {
        if self.node[s].next.contains_key(&x) {
            Some(self.node[s].next[&x])
        } else {
            if s == 0 {
                Some(0)
            } else {
                None
            }
        }
    }

    pub fn transition(&mut self, s: usize, x: usize) -> usize {
        let mut now = s;
        let mut from = vec![];
        while let None = self.goto(now, x) {
            from.push(now);
            now = self.failure[now];
        }
        let to = self.goto(now, x).unwrap();
        for e in from {
            self.node[e].next.insert(x, to);
        }
        to
    }

    pub fn query(&mut self, query: &Vec<usize>) -> std::collections::BTreeSet<usize> {
        let mut now = 0;
        let mut st = std::collections::BTreeSet::new();
        for i in 0..query.len() {
            now = self.transition(now, query[i]);
            if let Some(val) = self.node[now].pattern {
                st.insert(val);
            }
        }
        st
    }
}
use std::collections::{HashMap, VecDeque};
// https://naoya-2.hatenadiary.org/entry/20090405/aho_corasick

#[derive(Clone, Debug)]
struct State {
    id: usize,
    next: HashMap<usize, usize>,
}

impl State {
    fn new(id: usize) -> Self {
        State {
            id,
            next: HashMap::new(),
        }
    }

    fn has_key(&self, x: usize) -> bool {
        self.next.contains_key(&x)
    }
}

pub struct AhoCorasick {
    node: Vec<State>,
    output: Vec<Vec<Vec<usize>>>,
    failure: Vec<usize>,
}

impl AhoCorasick {
    pub fn new(patterns: Vec<Vec<usize>>) -> Self {
        let node = vec![State::new(0)];
        AhoCorasick {
            node,
            output: vec![vec![]; 1],
            failure: vec![],
        }
    }

    fn make_goto(&mut self, patterns: &Vec<Vec<usize>>) {
        // Trie 木をつくる
        for pattern in patterns {
            let mut cur = self.node[0].id;
            for &x in pattern {
                if !self.node[cur].has_key(x) {
                    let mut new_node = State::new(self.node.len());
                    self.node[cur].next.insert(x, new_node.id);
                    self.node.push(new_node);
                    self.output.push(vec![]);
                }
                cur = self.node[cur].next[&x];
            }

            // output の処理
            // self.output[cur].push(pattern.clone())
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
                            // こわれてる self.output[nxt].extend(self.output[failure[nxt]].clone().iter());
                        }
                    }
                }
            }
        }
        self.failure = failure;
    }

    fn goto(&self, s: usize, x: usize) -> Option<usize> {
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

    fn query(&self, query: Vec<usize>) {
        let mut now = 0;
        for i in 0..query.len() {
            while self.goto(now, query[i]).is_none() {
                now = self.failure[now];
            }
            now = self.goto(now, query[i]).unwrap();
        }
    }

}
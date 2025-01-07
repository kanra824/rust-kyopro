#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trie {
    cnt: i64,
    end: i64,
    child: Vec<Option<Box<Trie>>>,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            cnt: 0,
            end: 0,
            child: vec![None; 26],
        }
    }

    pub fn new_child() -> Option<Box<Self>> {
        Some(Box::new(Trie {
            cnt: 0,
            end: 0,
            child: vec![None; 26],
        }))
    }

    pub fn add(&mut self, s: &Vec<char>) {
        let mut now = self;
        now.cnt += 1;
        for i in 0..s.len() {
            let idx = s[i] as usize - 'a' as usize;
            if now.child[idx].is_none() {
                now.child[idx] = Self::new_child();
            }
            now = now.child[idx].as_mut().unwrap().as_mut();
            now.cnt += 1;
        }
        now.end += 1;
    }

    pub fn has_nth(&self, idx: usize) -> bool {
        !self.child[idx].is_none()
    }

    pub fn nxt(mut self: &mut Trie, idx: usize) {
        self = self.child[idx].as_mut().unwrap().as_mut();
    }

    // 最小共通接頭辞の長さの総和
    pub fn calc_lcp_sum(&mut self, s: &Vec<char>) -> i64 {
        let mut now = self;
        let mut res = 0;
        let mut depth = 0;
        let mut prevcnt = now.cnt;
        for i in 0..s.len() {
            let idx = s[i] as usize - 'a' as usize;
            if now.child[idx].is_none() {
                res += now.cnt * depth;
                return res;
            }
            now = now.child[idx].as_mut().unwrap().as_mut();
            res += (prevcnt - now.cnt) * depth;
            depth += 1;
            prevcnt = now.cnt;
        }
        res += now.cnt * depth;
        res
    }
}

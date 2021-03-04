pub struct Solution {}

impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let n = envelopes.len();
        if n == 0 {
            return 0;
        }

        let mut envelopes = envelopes.into_iter().map(|env_vec| Envelope(env_vec)).collect::<Vec<Envelope>>();
        envelopes.sort_unstable();

        let mut max_lens = vec![1; n];

        for i in 1..n {
            for j in 0..i {
                if envelopes[i].0[1] > envelopes[j].0[1] {
                    max_lens[i] = max_lens[i].max(max_lens[j] + 1);
                }
            }
        }

        max_lens.into_iter().max().unwrap()
    }
}

struct Envelope(Vec<i32>);

impl PartialEq for Envelope {
    fn eq(&self, other: &Self) -> bool {
        self.0[0] == other.0[0] && self.0[1] == other.0[1]
    }
}

impl Eq for Envelope {}

impl Ord for Envelope {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.0[0].cmp(&other.0[0]) {
            same if same == std::cmp::Ordering::Equal => {
                return self.0[1].cmp(&other.0[1]).reverse();
            }
            diff => diff,
        }
    }
}

impl PartialOrd for Envelope {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

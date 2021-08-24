pub struct Solution {}

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if graph.len() < 2 {
            return vec![];
        }

        let mut rec_pack = RecPack {
            trace: vec![0],
            ans: vec![],
        };

        rec_pack = Solution::backtrace_rec(&graph, rec_pack);

        return rec_pack.ans;
    }

    fn backtrace_rec(graph: &Vec<Vec<i32>>, mut rec: RecPack) -> RecPack {
        let target = (graph.len() - 1) as i32;
        let src = *rec.trace.last().unwrap();
        if src == target {
            return rec;
        }

        for &dest in graph[src as usize].iter() {
            rec.trace.push(dest);

            if dest == target {
                rec.ans.push(rec.trace.clone());
            }

            rec = Solution::backtrace_rec(graph, rec);
            rec.trace.pop();
        }

        return rec;
    }
}

struct RecPack {
    trace: Vec<i32>,
    ans: Vec<Vec<i32>>,
}

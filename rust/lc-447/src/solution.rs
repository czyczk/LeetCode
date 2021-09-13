pub struct Solution {}

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;
        for point in points.iter() {
            let mut dist_cnt_map = std::collections::HashMap::new();
            for other in points.iter() {
                let dist = (other[0] - point[0]).pow(2) + (other[1] - point[1]).pow(2);
                *dist_cnt_map.entry(dist).or_insert(0) += 1;
            }

            for (_, v) in dist_cnt_map {
                ret += v * (v - 1);
            }
        }

        ret
    }
}

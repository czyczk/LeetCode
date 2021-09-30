pub struct Solution {}

impl Solution {
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        let (cx1, cy1, cx2, cy2) = (ax1.max(bx1), ay1.max(by1), ax2.min(bx2), ay2.min(by2));

        let overlapped_area = if cx1 >= cx2 || cy1 >= cy2 {
            0
        } else {
            (cx2 - cx1) * (cy2 - cy1)
        };

        (ax2 - ax1) * (ay2 - ay1) + (bx2 - bx1) * (by2 - by1) - overlapped_area
    }
}

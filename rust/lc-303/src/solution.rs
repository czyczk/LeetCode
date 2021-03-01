pub struct NumArray {
    arr: Vec<i32>,
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        NumArray {
            arr: nums,
        }
    }

    pub fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.arr[i as usize..=j as usize].iter().sum()
    }
}


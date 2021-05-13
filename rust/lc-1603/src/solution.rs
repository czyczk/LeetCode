pub struct ParkingSystem {
    spaces: [i32; 4],
}

impl ParkingSystem {
    pub fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            spaces: [0, big, medium, small],
        }
    }

    pub fn add_car(&mut self, car_type: i32) -> bool {
        let car_type = car_type as usize;
        if self.spaces[car_type] > 0 {
            self.spaces[car_type] -= 1;
            return true;
        }

        false
    }
}

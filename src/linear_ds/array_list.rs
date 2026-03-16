#[derive(Debug)]
pub struct ArrayList {
    nums: Box<[i32]>,
    count: usize,
}

impl ArrayList {
    pub fn new(length: usize) -> Self {
        return Self {
            nums: vec![0; length].into_boxed_slice(),
            count: 0,
        };
        /*
         * use of vec may defies the purpose of creating arraylist.
         * but to focus on the actual logic of arraylist data structure,
         * I used vec to simplify/abstract the complexity.
         * because vec requires low level understanding of raw poiuter and other stuffs
         */
    }
}

impl ArrayList {
    pub fn insert(&mut self, item: i32) {
        if self.nums.len() == self.count {
            let mut new_array = vec![0; self.count * 2].into_boxed_slice();

            for i in 0..self.nums.len() {
                new_array[i] = self.nums[i];
            }
            self.nums = new_array;
        }
        self.nums[self.count] = item;
        self.count += 1;
    }
}

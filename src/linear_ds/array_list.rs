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

    pub fn remove_at(&mut self, index: usize) {
        if index >= self.count {
            panic!("illegal argument");
        }
        for i in index..self.count - 1 {
            self.nums[i] = self.nums[i + 1];
        }
        self.count -= 1;
    }

    pub fn to_string(&self) -> String {
        return format!("{:?}", &self.nums[..self.count]);
    }
}

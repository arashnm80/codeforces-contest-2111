use std::io::{self, BufRead};

fn func(mut nums: Vec<i32>, x: i32) -> Vec<i32> {
    nums.sort();
    nums[0] = std::cmp::min(2 * nums[1] + 1, x);
    nums
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let x: i32 = lines.next().unwrap().parse().unwrap();
        let mut nums = vec![0, 0, 0];
        let mut counter = 0;

        while nums != vec![x, x, x] {
            nums = func(nums, x);
            counter += 1;
        }

        println!("{}", counter);
    }
}

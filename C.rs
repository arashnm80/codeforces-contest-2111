use std::io::{self, BufRead};
use std::cmp;

fn solve() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t_str = lines.next().unwrap().unwrap();
    let t: i32 = t_str.parse().unwrap();

    for _ in 0..t {
        let n_str = lines.next().unwrap().unwrap();
        let n: i32 = n_str.parse().unwrap();

        let nums_str = lines.next().unwrap().unwrap();
        let nums: Vec<i64> = nums_str
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut min_cost: i64 = 5 * 100_000 * n as i64;
        let mut l_max: usize = 0;
        let mut r_min: usize = 0;

        for j in 0..n as usize {
            // In Rust, we handle the first number implicitly.
            // On the first iteration (j=0), l_max and r_min are already 0.
            if j > 0 {
                // if it's not the first number
                if nums[j] == nums[j - 1] {
                    // is equal with previous one
                    r_min = j; // shift r_min one step to right
                } else {
                    // shift both to right
                    l_max = j;
                    r_min = j;
                }
            }
            
            // The length of the subarray of equal numbers is (r_min - l_max + 1)
            // The number of elements to change is n - that length.
            let found_cost = (n as i64 - (r_min - l_max + 1) as i64) * nums[r_min];
            min_cost = cmp::min(min_cost, found_cost);
        }
        println!("{}", min_cost);
    }
}

fn main() {
    solve();
}

// pub fn main() {
//     println!("{0:?}", two_sum(vec![2, 7, 11, 15], 9));
//     println!("{0:?}", two_sum(vec![3, 2, 4], 6));
//     println!("{0:?}", two_sum(vec![3, 3], 6));
// }

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, &element) in nums.iter().enumerate() {
        for (j, &second) in nums.iter().enumerate().skip(i + 1) {
            println!("{0}, {1}", element, second);
            if element + second == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn example_cases() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
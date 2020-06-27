pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let len = nums.len();

    for i in 0..len-1 {
        for j in i+1..len {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32]
            }
        }
    }
    vec![]
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let vec = vec![3,2,4];

        let result = super::two_sum(vec, 6);

        assert_eq!(result, vec![1,2])
    }

    #[test]
    fn case2() {
        let vec = vec![3,1,2,4,3];

        let result = super::two_sum(vec, 6);

        assert_eq!(result, vec![0,4])
    }

    #[test]
    fn case3() {
        let vec = vec![3,0,3,3,3];

        let result = super::two_sum(vec, 6);

        assert_eq!(result, vec![0,2])
    }

    #[test]
    fn case4() {
        let vec = vec![3,0,3,3,3];

        let result = super::two_sum(vec, 5);

        assert_eq!(result, vec![])
    }
}
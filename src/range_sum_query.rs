pub struct NumArray {
    numbers: Vec<i32>
}

impl NumArray {
    pub fn new(mut numbers: Vec<i32>) -> Self {
        let mut sum = 0;
        for n in numbers.iter_mut() {
            sum += *n;
            *n = sum;
        }
        NumArray {
            numbers
        }
    }

    pub fn sum_range(&self, i: i32, j: i32) -> i32 {
        let top = self.numbers[j as usize];
        let bottom = if i == 0 {
            0
        } else {
            self.numbers[i as usize - 1]
        };

        top - bottom
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let vec = vec![-2, 0, 3, -5, 2, -1];
        let numbers = super::NumArray::new(vec);

        let result = numbers.sum_range(0, 2);

        assert_eq!(result, 1)
    }

    #[test]
    fn case2() {
        let vec = vec![-2, 0, 3, -5, 2, -1];
        let numbers = super::NumArray::new(vec);

        let result = numbers.sum_range(2, 5);

        assert_eq!(result, -1)
    }

    #[test]
    fn case3() {
        let vec = vec![-2, 0, 3, -5, 2, -1];
        let numbers = super::NumArray::new(vec);

        let result = numbers.sum_range(0, 5);

        assert_eq!(result, -3)
    }
}
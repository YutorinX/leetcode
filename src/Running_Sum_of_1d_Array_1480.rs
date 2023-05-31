impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut output = Vec::new();

        for (index, num) in nums.iter().enumerate() {
            let last_value = if (index == 0) { 0 } else { output[index - 1] };
            output.push(last_value + num);
        }

        return output;
    }
}

impl Solution2 {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut output = vec![nums[0]];

        for index in 1..nums.len() {
            let last_value = output[index - 1];
            output.push(last_value + nums[index]);
        }

        return output;
    }
}

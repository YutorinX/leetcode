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

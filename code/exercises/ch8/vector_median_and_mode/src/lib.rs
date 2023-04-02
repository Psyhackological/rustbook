use std::collections::HashMap;

// Calculate the median of a slice of i32 values.
pub fn calculate_median(nums: &[i32]) -> f64 {
    let mut sorted_nums = nums.to_vec();
    // Use an unstable sort for better performance, as the order of equal elements doesn't matter.
    sorted_nums.sort_unstable();

    let len = sorted_nums.len();
    // If the length is even, average the two middle values.
    if len % 2 == 0 {
        (sorted_nums[len / 2 - 1] as f64 + sorted_nums[len / 2] as f64) / 2.0
    } else {
        // If the length is odd, return the middle value.
        sorted_nums[len / 2] as f64
    }
}

// Calculate the mode of a slice of i32 values.
pub fn calculate_mode(nums: &[i32]) -> Option<i32> {
    // Create a HashMap to store the counts of each number in the input slice.
    let mut counts = HashMap::new();

    // Iterate through the input slice and update the counts for each number.
    for &num in nums {
        *counts.entry(num).or_insert(0) += 1;
    }

    // Find the maximum count among all the numbers.
    let max_count = counts.values().cloned().max().unwrap_or(0);

    // If the maximum count is less than or equal to 1, it means that all numbers
    // occur the same number of times, so we return None.
    if max_count <= 1 {
        None
    } else {
        // If there is a mode, find the number with the maximum count and return it.
        // The map() function is used to extract just the number (num) from the tuple (num, count).
        counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(num, _)| num)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_median() {
        let nums = vec![1, 2, 3, 4, 4, 5, 5, 5, 6, 7, 7];
        assert_eq!(calculate_median(&nums), 5.0);

        let nums = vec![1, 9, 3, 4, 4, 5, 5, 5, 6, 7, 7, 8, 9, 9, 9];
        assert_eq!(calculate_median(&nums), 6.0);

        let nums = vec![1, 3, 4, 5, 8, 10];
        assert_eq!(calculate_median(&nums), 4.5);
    }

    #[test]
    fn test_calculate_mode() {
        let nums = vec![1, 2, 3, 4, 4, 5, 5, 5, 6, 7, 7];
        assert_eq!(calculate_mode(&nums), Some(5));

        let nums = vec![1, 9, 3, 4, 4, 5, 5, 5, 6, 7, 7, 8, 9, 9, 9];
        assert_eq!(calculate_mode(&nums), Some(9));

        let nums = vec![1, 3, 4, 5, 8, 10];
        assert_eq!(calculate_mode(&nums), None);
    }
}

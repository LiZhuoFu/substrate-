
pub fn sum_u32(values: &[u32]) -> Option<u32> {
    let mut sum = 0u32;
    for value in values {
        match sum.checked_add(*value) {
            Some(new_sum) => sum = new_sum,
            None => return None,
        }
    }
    Some(sum)
}

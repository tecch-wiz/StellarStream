// Optimized math functions with inlining for better performance

/// Calculate unlocked amount with linear vesting
/// Inlined for performance in hot paths
#[inline(always)]
#[allow(dead_code)]
pub fn calculate_unlocked_amount(
    total_amount: i128,
    start_time: u64,
    end_time: u64,
    current_time: u64,
) -> i128 {
    if current_time < start_time {
        return 0;
    }

    if current_time >= end_time {
        return total_amount;
    }

    let elapsed_time = (current_time - start_time) as i128;
    let total_duration = (end_time - start_time) as i128;

    (total_amount * elapsed_time) / total_duration
}

/// Calculate withdrawable amount
/// Inlined for performance
#[inline(always)]
#[allow(dead_code)]
pub fn calculate_withdrawable_amount(unlocked_amount: i128, withdrawn_amount: i128) -> i128 {
    unlocked_amount - withdrawn_amount
}

/// Calculate unlocked amount with cliff support
/// Inlined for performance in withdraw function (most frequently called)
#[inline(always)]
pub fn calculate_unlocked(total_amount: i128, start: u64, cliff: u64, end: u64, now: u64) -> i128 {
    // Early returns for common cases
    if now < cliff {
        return 0;
    }
    if now >= end {
        return total_amount;
    }

    let elapsed = (now - start) as i128;
    let total_duration = (end - start) as i128;

    (total_amount * elapsed) / total_duration
}

/// Fast fee calculation using bit operations where possible
/// For fee_bps (basis points), we divide by 10000
/// Inlined for performance
#[inline(always)]
pub fn calculate_fee(amount: i128, fee_bps: u32) -> i128 {
    if fee_bps == 0 {
        return 0;
    }
    (amount * fee_bps as i128) / 10000
}

/// Check if a value is a power of 2 (useful for optimization checks)
#[inline(always)]
pub fn is_power_of_two(n: u32) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_math_logic() {
        let total = 1000_i128;
        let start = 100;
        let end = 200;

        assert_eq!(calculate_unlocked_amount(total, start, end, 50), 0);
        assert_eq!(calculate_unlocked_amount(total, start, end, 100), 0);
        assert_eq!(calculate_unlocked_amount(total, start, end, 150), 500);
        assert_eq!(calculate_unlocked_amount(total, start, end, 200), 1000);
        assert_eq!(calculate_unlocked_amount(total, start, end, 250), 1000);
    }

    #[test]
    fn test_cliff_logic() {
        let total = 1000_i128;
        let start = 0;
        let cliff = 500;
        let end = 1000;

        assert_eq!(calculate_unlocked(total, start, cliff, end, 250), 0);
        assert_eq!(calculate_unlocked(total, start, cliff, end, 500), 500);
        assert_eq!(calculate_unlocked(total, start, cliff, end, 750), 750);
        assert_eq!(calculate_unlocked(total, start, cliff, end, 1000), 1000);
    }
}

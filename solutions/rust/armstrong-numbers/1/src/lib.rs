pub fn is_armstrong_number(num: u32) -> bool {
    let mut sum: u32 = 0u32;
    let mut n: u32 = num;
    let mut digit: u32 = 0u32;
    while n > 0 {
        n /= 10;
        digit += 1;
    }
    n = num;
    while n > 0 {
        sum += (n%10).pow(digit);
        n /= 10;
    }
    sum == num
}

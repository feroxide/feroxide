/// Calculate greatest common divisor
pub fn gcd(x: i32, y: i32) -> i32 {
    // Store the highest in a, the lowest in b
    let (mut a, mut b) = {
        if x > y {
            (x, y)
        } else {
            (y, x)
        }
    };

    // Use Euclides' algorithm
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }

    a
}

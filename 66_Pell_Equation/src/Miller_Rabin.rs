// miller Rabin primality test
fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    for mut x in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41] {
        if n == x {
            return true;
        }
        let mut flag: bool = true;
        let mut r: i64 = 1;
        let mut t: i64 = 1;
        let temp = (n - 1) >> (n - 1).trailing_zeros();
        while r <= temp {
            if r & temp != 0 {
                t = ((t as i128 * x as i128) % n as i128) as i64;
            }
            x = (((x as i128) * x as i128) % n as i128) as i64;
            r <<= 1;
        }
        if t == 1 || t == n - 1 {
            flag = false;
        }
        for _ in 0..(n - 1).trailing_zeros() {
            t = ((t as i128 * t as i128) % n as i128) as i64;
            if t == n - 1 {
                flag = false;
            }
        }
        if flag {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    fn check_array(nums: Vec<i64>, res: bool) {
        for n in nums {
            assert_eq!(is_prime(n), res);
        }
    }
    #[test]
    fn check_small_primes() {
        let primes = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
        ];
        check_array(primes.to_vec(), true);
    }
    #[test]
    fn check_small_composite() {
        let composites = [
            1, 4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21, 22, 24, 25, 26, 27, 28, 30, 32, 33, 34,
            35, 36, 38, 39, 40, 42, 44, 45, 46, 48, 49, 50, 51, 52, 54, 55, 56, 57, 58, 60, 62, 63,
            64, 65, 66, 68, 69, 70, 72, 74, 75, 76, 77, 78, 80, 81, 82, 84, 85, 86, 87, 88, 90, 91,
            92, 93, 94, 95, 96, 98, 99, 100,
        ];
        check_array(composites.to_vec(), false);
    }
    #[test]
    fn check_large_primes() {
        let primes = [
            1000000007,
            1000000009,
            1000000021,
            1000000033,
            1000000087,
            1000000093,
            1000000097,
            1000000103,
            1000000123,
            1000000181,
            1000000207,
            1000000223,
            1000000241,
            1000000271,
            1000000289,
            1000000297,
            1000000321,
            1000000349,
            1000000363,
            1000000403,
            1000000409,
            1000000411,
            1000000427,
            1000000433,
            1000000439,
            1000000447,
            1000000453,
            1000000459,
            1000000483,
            1000000513,
            1000000531,
            1000000579,
            1000000607,
            1000000613,
            1000000637,
            1000000663,
            1000000711,
            1000000753,
            1000000787,
            1000000801,
            1000000829,
            1000000861,
            1000000871,
            1000000891,
            1000000901,
            1000000919,
            1000000931,
            1000000933,
            1000000993,
            1000001011,
            1000001021,
            1000001053,
            1000001087,
            1000001099,
            1000001137,
            1000001161,
            1000001203,
            1000001213,
            1000001237,
            1000001263,
            1000001269,
            1000001273,
            1000001279,
            1000001311,
            1000001329,
            1000001333,
            1000001351,
            1000001371,
            1000001393,
            1000001413,
            1000001447,
            1000001449,
            1000001491,
            1000001501,
            1000001531,
            1000001537,
            1000001539,
            1000001581,
            1000001617,
            1000001621,
            999999999959,
            999999999961,
            999999999989,
            1000000000039,
            1000000000061,
        ];
        check_array(primes.to_vec(), true);
    }
}
fn main() {
    println!("{}", is_prime(5));
}

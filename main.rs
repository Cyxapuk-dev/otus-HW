struct Doubler;

impl Doubler {
    fn double_int32(input: i32) -> i32 {
        input.wrapping_mul(2)
    }

    fn double_int64(input: i32) -> i64 {
        (input as i64) * 2
    }

    fn double_float32(input: f32) -> f32 {
        input * 2.0
    }

    fn double_float64(input: f32) -> f64 {
        (input as f64) * 2.0
    }
}

struct Summator;

impl Summator {
    fn int_plus_float_to_float(a: i32, b: f32) -> f64 {
        (a as f64) + (b as f64)
    }

    fn int_plus_float_to_int(a: i32, b: f32) -> i64 {
        (a as f64 + b as f64) as i64
    }

    fn tuple_sum(input: (i32, i32)) -> i32 {
        input.0 + input.1
    }

    fn array_sum(input: [i32; 3]) -> i32 {
        input.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_int32() {
        assert_eq!(Doubler::double_int32(15), 30);
    }

    #[test]
    fn test_double_int64() {
        assert_eq!(Doubler::double_int64(15), 30);
    }

    #[test]
    fn test_double_float32() {
        assert_eq!(Doubler::double_float32(2.5), 5.0);
    }

    #[test]
    fn test_double_float64() {
        assert_eq!(Doubler::double_float64(2.5), 5.0);
    }

    #[test]
    fn test_int_plus_float_to_float() {
        assert_eq!(Summator::int_plus_float_to_float(10, 2.5), 12.5);
    }

    #[test]
    fn test_int_plus_float_to_int() {
        assert_eq!(Summator::int_plus_float_to_int(10, 2.5), 12);
    }

    #[test]
    fn test_tuple_sum() {
        assert_eq!(Summator::tuple_sum((1, 2)), 3);
    }

    #[test]
    fn test_array_sum() {
        assert_eq!(Summator::array_sum([1, 2, 3]), 6);
    }
}

fn main() {
    println!("{}", Doubler::double_int32(15));
    println!("{}", Doubler::double_int64(15));
    println!("{}", Doubler::double_float32(2.5));
    println!("{}", Doubler::double_float64(2.5));
    println!("{}", Summator::int_plus_float_to_float(10, 2.5));
    println!("{}", Summator::int_plus_float_to_int(10, 2.5));
    println!("{}", Summator::tuple_sum((1, 2)));
    println!("{}", Summator::array_sum([1, 2, 3]));
}

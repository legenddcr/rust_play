#[cfg(test)]
mod tests {
    fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
        f(value)
    }

    fn square(value: i32) -> i32 {
        value * value
    }

    fn cube(value: i32) -> i32 {
        value * value * value
    }

    #[test]
    fn test_apply() {
        assert_eq!(4, apply(2, square));
        assert_eq!(8, apply(2, cube));
    }

    fn pi() -> f64 {
        3.1415926
    }

    fn not_pi() {
        3.1415926;
    }

    #[test]
    fn test_ret_type() {
        //cargo t -p ch03_grammar -- --show-output
        // uint if last statement ends with ';'
        let is_pi = pi();
        let is_uint1 = not_pi();
        let is_uint2 = {
            pi();
        };

        println!("is_pi: {:?}, is_uint1: {:?}, is_unit2: {:?}", is_pi, is_uint1, is_uint2)
    }
}

const PI: f32 = std::f32::consts::PI;

fn count_to_5() -> i32 {
    let mut foo = 0;
    loop {
        if foo > PI as i32 && foo > 5 {
            break;
        }
        foo += 1;
    }
    5
}

fn main() {
    println!("I can count to {}", count_to_5());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_counting() {
        assert_eq!(count_to_5(), 5);
    }
}

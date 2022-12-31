pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn it_works() {
        let random_num = rand::thread_rng().gen_range(-100..=100);
        assert_eq!(random_num + 1, add_one(random_num));
    }
}

pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    let mut vec = vec![0, 1];

    for i in 2..=n {
        vec.push(vec[i as usize - 1] + vec[i as usize - 2]);
    }
    vec[n as usize]
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirthieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}

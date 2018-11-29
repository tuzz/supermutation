struct Util {

}

impl Util {
    fn factorial(n: usize) -> usize {
        match n {
            1 => 1,
            _ => n * Self::factorial(n - 1),
        }
    }
}

#[cfg(test)]
mod test;

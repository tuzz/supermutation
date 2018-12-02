pub struct Interface {

}

impl Interface {
    pub fn ask_for_symbols() -> usize {
        if cfg!(feature = "four_symbols") {
            return 4;
        }

        if cfg!(test) {
            return 5;
        }

        5 // TODO: get user input
    }
}

#[cfg(test)]
mod test;

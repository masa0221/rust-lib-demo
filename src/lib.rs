mod generator;

pub fn print_randam_number() {
    let n = generator::gen_ran();
    println!("Ramdom u8: {}", n);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

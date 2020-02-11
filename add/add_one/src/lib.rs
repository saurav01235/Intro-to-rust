pub fn add_1(x :u32) ->u32 {
    x+1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_1(2));
    }
}
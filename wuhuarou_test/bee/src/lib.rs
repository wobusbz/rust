pub mod beego;
#[cfg(test)]
mod tests {

    use crate::beego::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn usecat() {
        cat::hello();
        assert_eq!(true, cat::is_cat());
    }

    #[test]
    fn us_dog(){
        assert_eq!(true, dog::is_dog());
    }
}

//! my Create
//! 
//! 'my_create' is a collection of utilites to make performing certain calcuations more convenicnt
//! 
/// 给一个数加1
/// ```
/// let five = 5;
/// assert_eq!(6, libs::add_one(5));
/// ``` 
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

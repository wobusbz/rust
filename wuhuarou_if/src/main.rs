fn main() {


    let x = 1;
    if x == 1{
        println!("{}", x)
    }else {
        println!("x != {}", x)
    }

    let condition = true;
    let x = if condition{
        5
    } else {
        6
    };
    println!("x = {}", x);
    let mut i: u32 = 0;
    loop {
        println!("in loop");
        if i == 10 {
            break;
        }
        i += 1;
    };

    let result = loop {
        i += 1;
        if i == 20 {
            break i * 2;
        }
    };
    println!("{}", result);

    let mut count: u32 = 0;
    while count != 30{
        count += 1;
    };
    println!("count = {}", count);


    let arr: [u32; 5] = [1,2,3,4,5];
    
    for value in &arr {
        println!("value = {} ", value);
    };
}

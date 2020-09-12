fn main() {

   // bool
   let is_true: bool = true;
   println!("{}", is_true);

   let is_true = false;
   println!("{}", is_true);

   // char
   let b = '你';
   println!("{}", b);

   println!("{}", usize::max_value());
   println!("{}", isize::min_value());

   // 数组
   let arr: [u32; 3] = [1,2,3];
   println!("arr[0] = {}",arr[0]);
   shows(arr);

   // 元祖

   let tup: (u32, f32, char) = (1, 2.2, '我');
   println!("{}", tup.0);
   println!("{}", tup.1);
   println!("{}", tup.2);

   let tups = (1, 0.001, "wo");
   println!("{}", tups.0);
   println!("{}", tups.1);
   println!("{}", tups.2);
}


fn shows(arr: [u32; 3])  {
   for i in &arr {
      println!("{}",i);
   }
}
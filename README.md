##### 变量绑定

1. 声明变量--- 
   ```rust 
   let x = 5; // 变量绑定
   // Patterns 声明多个变量
   let (x, y) = (1, 2);
   // 类型注解 声明变量类型 称变量绑定 x 变量绑定为i32类型
   let x: i32 = 5;
   // **********以上变量不可变*********** //

   let mut x = 5; // 声明可变变量

   ```

#### 有符号和无符号
整型有两种变体：有符号和无符号。为了理解它们的区别，让我们考虑一个 4 比特大小的数字。一个有符号，4 比特数字你可以储存-8到+7的数字。有符号数采用“二进制补码”表示。一个无符号 4 比特的数字，因为它不需要储存负数，可以出储存0到+15的数字。

#### 固定大小类型
固定大小类型在其表现中有特定数量的位。有效的位大小是8，16，32和64。那么，u32是无符号的，32 位整型，而i64是有符号，64 位整型。

#### 可变大小类型
Rust 也提供了依赖底层机器指针大小的类型。这些类型拥有“size”分类，并有有符号和无符号变体。它有两个类型：isize和usize。

#### 浮点类型
Rust 也有两个浮点类型：f32和f64。它们对应 IEEE-754 单精度和双精度浮点数。


#### 主要命令
cargo  new project_name --bin                     # 如果你想写一个普通的项目</br>
cargo new lib_name --lib    --vcs none            # 如果你想写一个库</br>
cargo build                                       # 如果你想编译，默认会编译到target/</br>
debug/project_name下
cargo run                                         # 如果你想编译并运行</br>
cargo build --release                             # 如果你想发布，这会做很多优化，并编译到target/release/</br>
project_name下
cargo update                                      # 如果你想修改Cargo.lock文件的话，运行它</br>
cargo update  -p rand                             # 如果你只是想更新rand版本的话，运行它</br>
cargo test abc                                    # 如果你想做test，运行它</br>

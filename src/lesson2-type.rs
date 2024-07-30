fn main() {
    // integer types 整型  => 有符号 无符号  按平台
    // 有符号 i8 i6 i32 i64 i64 根据大小来的
    // 无符号 u8 u6 u32 u64 u64 根据大小来的
    // 按平台 usize isize
    let a1: i32 = -125;
    let a2: i32 = 0xff;
    let a3: i32 = 0o13;
    let a4: i32 = 0b10;
    println!("u32 max: {}", u32::MAX);
    println!("u32 max: {}", u32::MIN);
    //Float Types   f32 f64
    //尽量使用f64 除非你很需要空间小  知道自己的边界

    //Boolean  true false

    //Character Types
    //Rust支持 unicode 字符😊
    let x = "123";
    //表示char类型的时候可以使用单引号
    let y = '1';
}

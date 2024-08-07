//static 一般在文件头
static _XXXX: i32 = 23;
static mut MUT_XXXX: i32 = 23;

fn main() {
    // 默认会进行类型推导，也可以自己去定义类型
    //如果类型推导不出来或者有问题编译器会直接报错
    // let value1 = 123;
    // 变量名支持 蛇形 和 pascal,所以不能使用驼峰去命名一个变量
    // let Value2 = 234;
    // 也支持强制类型转换   as i32

    // print变量的方式
    // printIn!("")

    // rust的变量是默认不可变的  const
    //发生了自动推导
    let _temp_x = 123;
    let _temp_x_2: i64 = 123;
    // 可以使用 mut 来让变量可变 let mut y = 20;
    let _mut_temp_x = 123;
    // rust 可以声明一个现有变量同名的变量。
    // 但是他不是重新赋值，只是隐藏
    let x = 2;
    {
        // 建立一个新的作用域？还是命名空间
        let x = 10;
        print!("inner x : {x}");
    }
    print!("outer x : {x}");
    let x = "hello world"; //在这个地方会发生覆盖;
    print!("outer new x : {x}");
    let mut x = "mut string"; //可修改变量的 可变性
    print!("outer mut new x : {x}");
    x = "change";
    print!("outer mut new x : {x}");

    //常量的值会被直接嵌入到生成的底层机器代码里。而不是简单的字符替换
    //常量默认是全大写  MAX_XXX
    const SECOND_HOURS: usize = 3_600;
    const SECOND_DAY: usize = SECOND_HOURS * 24; //编译时就确定了/类型/值
    print!("outer mut second : {SECOND_HOURS}-{SECOND_DAY}");
    //static静态变量  实在运行时才分配内存
    //它是可修改的  在unsafe 代码段里修改
    // 它的生命周期是程序运行的时间？（ps 这个我不太懂是什么意思）
    unsafe {
        MUT_XXXX = 34; //不要写unsafe
        println!("outer mut new x : {MUT_XXXX}");
    }
}

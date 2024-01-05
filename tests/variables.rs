#[cfg(test)]
mod variables {
    #[test]
    fn base()
    {
        let num1: i8 = -1;
        let num2: u8 = 1;
        let num3 = 1_000_00;//自动推断类型,还可以用下划线格式化
        let num4 = 100i16;//i16 表示类型
        let _num = 100u16;//使用下划线开头忽略未使用的变量

        println!("numbers: {} {} {} {}", num1, num2, num3, num4);

        let f1 = 1.1;
        let f2 = 1.1f32;
        println!("float: {} {}", f1, f2);

        let str = "hello";
        println!("str: {}", str);
    }

    #[test]
    fn mutable() {
        let a = 123;
        let mut b = 356;
        println!("{} {}", a, b);

        // a = -1;
        b = -1;
        println!("{} {}", a, b)
    }

    #[test]
    fn const_var() {
        const C: i32 = 123;
        println!("{}", C);
    }

    #[test]
    fn string() {
        let a = "hello";
        let b = 'a';
        println!("{} {}", a, b);
    }

    #[test]
    //解构
    fn deconstruction() {
        let (a, mut b): (bool, bool) = (true, false);
        // a = true,不可变; b = false，可变
        println!("a = {:?}, b = {:?}", a, b);

        b = true;
        assert_eq!(a, b);
    }
    // #[test]
    // fn deconstruction2() {
    //     let (a, b, c, d);
    //
    //     (a, b) = (1, 2);
    //     // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    //     [c, .., d, _] = [1, 2, 3, 4, 5];
    //     // println!("{} {} {} {} {}", a, b, c, d, e);
    // }
    #[test]
    fn shadowing() {
        //1. 定义的变量没有被使用
        let a = "hello";
        let a = a.len();
        println!("{}", a);
        //2. 不同作用域
        let x = 10;
        {
            let x = x*2;
            println!("{}", x);
        }
    }
}
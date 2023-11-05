struct S<'a>{
    r: &'a i32
}

/// D使用S作为类型引用，必须提供对应的生命周期
struct D<'a>{
    s: S<'a>
}

/// 拥有不同生命周期参数的结构体，放松生命周期限制
struct S1<'a, 'b>{
    x: &'a i32,
    y: &'b i32,
}

fn main() {
    let s;
    {
        let x = 10;
        s = S { r: &x };
        assert_eq!(*s.r, 10);
    }
    // s的生命周期被x所限制
    // assert_eq!(*s.r, 10);

    print_different_lifetime_struct();
}

fn print_different_lifetime_struct(){
    let x = 10;
    let r;
    {
        let y = 20;
        let s = S1 {x: &x, y: &y};
        r = s.x;
    }
    println!("{}", r);
}



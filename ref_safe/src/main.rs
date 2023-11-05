fn main() {
    error_demo();
    success_demo();

    let x = 1;
    g(&x);
    // x的生命周期不能和static一样长
    // _g1(&x);
}

fn error_demo(){
    let _r;
    {
        let x = 1;
        _r = &x;
    }
    // 超出x的作用域，调用出错
    //assert_eq!(*r, 1);
}

fn success_demo(){
    // r的生命周期涵盖在x的生命周期
    let x = 1;
    {
        let r  = &x;
        assert_eq!(*r, 1);
    }
}


fn g<'a>(p: &'a i32){
    println!("{}", p);
}

fn _g1(p: &'static i32){
    println!("{}", p);
}
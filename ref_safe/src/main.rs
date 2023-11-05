fn main() {
    println!("Hello, world!");
    error_demo();
    success_demo();
}

fn error_demo(){
    let r;
    {
        let x = 1;
        r = &x;
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
fn main() {
    let v = vec!["101".to_string(),"102".to_string(),"103".to_string()];
    for mut s in v {
        s.push('!');
        println!("{}", s);
    }
    // 不允许使用转移掉所有权的值
    // println!("{:?}", v);

    let v1 = vec!["101".to_string(),"102".to_string(),"103".to_string()];
    // 深拷贝值使用，不动用之前的所有权
    for mut s in v1.clone() {
        s.push('!');
        println!("{}", s);
    }
    println!("{:?}", v1);

    
}

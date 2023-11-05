fn main() {
    let v = vec!["101".to_string(),"102".to_string(),"103".to_string()];
    for mut s in v {
        s.push('!');
        println!("{}", s);
    }
    // 不允许使用转移掉所有权的值
    // println!("{:?}", v);
    println!(r###"ownership Move [println!("", v)] is error"###);

    let v1 = vec!["101".to_string(),"102".to_string(),"103".to_string()];
    // 深拷贝值使用，不动用之前的所有权
    for mut s in v1.clone() {
        s.push('!');
        println!("{}", s);
    }
    println!("CLONE complex Type{:?}", v1);

    // copy类型
    let v2 = vec![101,102,103];
    // 整数等简单类型直接会Copy，所有权还在自身
    for s in &v2 {
        let p = s.to_string() + "!";
        println!("{}", p);
    }
    println!("COPY ---- {:?}", v2);
}

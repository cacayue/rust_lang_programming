use std::env;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");

    // 初始化数组向量
    let mut numbers = Vec::new();

    // 循环获取的命令行参数，并转换为u64整数后放入numbers
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
        .expect("error parsing argument"));
    }

    // 如果数组长度为0， 提示错误并退出程序
    if numbers.len() == 0 {
        eprint!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    // 获取第一个数字，并循环执行最大公约数函数
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m)
    }

    // 打印最大公约数
    print!("The greatest common divisor of {:?} is {}", numbers, d);

    let _max = gcd(1, 4);
    
}

/// 最大公约数函数
fn gcd(mut n: u64, mut m:u64) -> u64{
    assert!(n != 0 && m!= 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}


#[test]
fn test_gcd(){
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
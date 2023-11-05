fn main() {
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a:[f64; 4] = [0.0, 0.707, 1.0, 0.707];

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    print_slice(sv);
    println!("-------------");
    print_slice(sa);
}   

/// 切片打印
fn print_slice(n: &[f64]){
    for elt in n {
        println!("{}", elt);
    }
}

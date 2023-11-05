use std::rc::Rc;

fn main() {
    let s = Rc::new("shirataki".to_string());
    let t = s.clone();
    let u = s.clone();
    println!("{:?}",t);
    println!("{:?}",u);
}

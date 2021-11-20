fn main() {
    fn fn_immut<F>(f: F) where F: Fn() -> String {
        println!("calling Fn closure from fn, {}", f());
    }
    
    let a = "Fn".to_string();
    // fn_immut(|| a);
    // 报错：move occurs because `a` has type `String`, which does not implement the `Copy` trait
    fn_immut(|| a.clone()); // 闭包返回一个字符串

    // let f = || a;
    // fn_immut(f);
    // error：
    // 11 |     let f = || a;
    // |             ^^^-
    // |             |  |
    // |             |  closure is `FnOnce` because it moves the variable `a` out of its environment
    // |             this closure implements `FnOnce`, not `Fn`
    // 12 |     fn_immut(f);
    // |     -------- the requirement to implement `Fn` derives from here
}
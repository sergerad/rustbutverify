fn main() {
    // fnMut: mutate captured vec
    let mut mutate_me: Vec<String> = vec!["c".into(), "d".into()];
    let fn_mut = |s: String| {
        mutate_me.push(s.clone());
        s
    };

    // fn: no capture
    let fn_ = |s: String| -> String { s.clone() };

    // use closures in iterator
    let l: Vec<String> = vec!["a".into(), "b".into()];
    let ll = l.into_iter().map(fn_).map(fn_mut).collect::<Vec<_>>();
    println!("ll: {:?}", ll);
    println!("r: {:?}", mutate_me);

    // fnOnce: move captured String
    let move_me = "x".to_string();
    let fn_once = || move_me;
    let moved_me = Some("s".to_string()).unwrap_or_else(fn_once);
    println!("moved_x: {:?}", moved_me);
}

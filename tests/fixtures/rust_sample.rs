fn simple() {
    println!("hello");
}

fn with_if(x: i32) {
    if x > 0 {
        println!("pos");
    } else {
        println!("non-pos");
    }
}

fn with_match(x: i32) {
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("other"),
    }
}

fn nested() {
    let _closure = |y: i32| -> i32 {
        if y > 0 { 1 } else { 0 }
    };
}

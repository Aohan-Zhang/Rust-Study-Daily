#[test]
fn mut_variable() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

struct _Struct {
    e: i32,
}

/// 解构
#[test]
fn deconstruct() {
    let (mut a, b): (bool, bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);
    a = false;
    assert_eq!(a, b);


    let (a, b, c, d, e);
    (a, b) = (1, 2);
    [c, .., d, _] = [1, 2, 3, 4, 5];
    _Struct { e, .. } = _Struct { e: 5 };

    assert_eq!([a, b, c, d, e], [1, 2, 1, 4, 5])
}

#[test]
fn shadowing() {
    let x = 1;
    let x = x + 1;
    {
        let x = 12;
        println!("inner block x value: {}", x);
    }
    println!("x value: {}", x);


    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces.len() = {}", spaces)
}

#[test]
fn reference_1()
{
    let x = 5;
    let p = &x;
    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}
#[test]
fn reference_2()
{
    let x = 5;
    let y = &x;
    assert_eq!(5, *y);
    println!("Success!");
}
#[test]
fn reference_3()
{
    let mut s = String::from("hello, ");
    borrow_object(&s);
    println!("Success!");
    fn borrow_object(s: &String) {}
}
#[test]
fn reference_4()
{
    let mut s = String::from("hello, ");
    push_str(&mut s);
    println!("Success!");
    fn push_str(s: &mut String) {
        s.push_str("world")
    }
}
#[test]
fn reference_5()
{
    let mut s = String::from("hello, ");
    let mut p = Box::new(&mut s);
    p.push_str("world");
    println!("Success!");
}
#[test]
fn reference_6()
{
    let c = '中';
    let r1 = &c;
    let ref r2 = c;
    assert_eq!(*r1, *r2);
    assert_eq!(get_addr(r1), get_addr(r2));
    println!("Success!");
    fn get_addr(r: &char) -> String {
        format!("{:p}", r)
    }
}
#[test]
fn reference_7()
{
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}", r2);
    println!("Success!");
}
#[test]
fn reference_8()
{
    // Fix error by modifying this line
    let mut s = String::from("hello, ");
    borrow_object(&mut s);
    println!("Success!");
    fn borrow_object(s: &mut String) {}
}
#[test]
fn reference_9()
{
    // This code has no errors!
    let mut s = String::from("hello, ");
    borrow_object(&s);
    s.push_str("world");
    println!("Success!");
    fn borrow_object(s: &String) {}
}
#[test]
fn reference_10()
{
    // Comment one line to make it work
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    //println!("{}",r1);
}
#[test]
fn reference_11()
{
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}",r1)
    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time
}
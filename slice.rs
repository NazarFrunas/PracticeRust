#[test]
fn slice_1()
{
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];
    let s2: &str = "hello, world";
    println!("Success!");
}
#[test]
fn slice_2()
{
    let arr: [char; 3] = ['中', '国', '人'];
    let slice = &arr[..2];
    assert!(std::mem::size_of_val(&slice) == 16);
    println!("Success!");
}
#[test]
fn slice_3()
{
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);
    println!("Success!");
}
#[test]
fn slice_4()
{
    let s = String::from("hello");
    let slice1 = &s[0..2];
    let slice2 = &s[..2];
    assert_eq!(slice1, slice2);
    println!("Success!");
}
#[test]
fn slice_5()
{
    let s = "你好，世界";
    // Modify this line to make the code work
    let slice = &s[0..3];
    assert!(slice == "你");
    println!("Success!");
}
#[test]
fn slice_6()
{
    let mut s = String::from("hello world");
    let letter = first_letter(&s);
    s.clear(); // error!
    println!("the first letter is: {}", letter);
    fn first_letter(s: &str) -> char {
        s.chars().next().unwrap()
    }
}

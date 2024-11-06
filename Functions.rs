#[test]
fn functions_1()
{
    let (x, y) = (1, 2);
    let s = sum(x, y);
    assert_eq!(s, 3);
    println!("Success!");
    fn sum(x: i32, y: i32) -> i32
    {
        x + y
    }
}
#[test]
fn functions_2()
{
    print();
    fn print() -> () {
        println!("Success!");
    }
}
#[test]
fn functions_3()
{
    never_return();
    fn never_return() -> !
    {
        loop {}
    }
}
#[test]
fn functions_4()
{
    get_option(0);
    println!("Success!");
    fn get_option(tp: u8) -> Option<i32>
    {
        match tp
        {
            1 => {
                Some(10)
            }
            _ => {
                never_return_fn()
            }
        }
    }
    fn never_return_fn() -> !
    {
        loop{}
    }
}
#[test]
fn functions_5()
{
    // FILL in the blank
    let b = false;
    let _v = match b {
        true => 1,

        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };
    println!("Exercise Failed if printing out this line!");
}
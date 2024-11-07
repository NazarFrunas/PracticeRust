#[test]
fn array_1()
{
    // Fill the blank with proper array type
    let arr:[i32; 5] = [1, 2, 3, 4, 5];
    assert!(arr.len() == 5);
    println!("Success!");
}
#[test]
fn array_2()
{
    
    let arr0 = [1, 2, 3];
    let arr: [char; 3] = ['a', 'b', 'c'];
    assert!(std::mem::size_of_val(&arr) == 12);
    println!("Success!");
}
#[test]
fn array_3()
{
    // Fill the blank
    let list: [i32; 100] = [1; 100];
    assert!(list[0] == 1);
    assert!(list.len() == 100);
    println!("Success!");
}
#[test]
fn array_4()
{
    // Fix the error
    let _arr = [1, 2, 3];
    println!("Success!");
}
#[test]
fn array_5()
{
    let arr = ['a', 'b', 'c'];
    let ele = arr[0]; // Only modify this line to make the code work!
    assert!(ele == 'a');
    println!("Success!");
}
#[test]
fn array_6()
{
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    // `Get` returns an Option<T>, it's safe to use
    let name0 = names.get(0).unwrap();
    // But indexing is not safe
    let _name1 = &names[1];
    println!("Success!");
}

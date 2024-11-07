#[test]
fn struct_1()
{
    struct Person {
        name: String,
        age: u8,
        hobby: String
    }
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age: age,
        hobby: String::from("Chilling")
    };
    println!("Success!");
}
#[test]
fn struct_2()
{
    struct Unit;
    trait SomeTrait {
    }
    impl SomeTrait for Unit {  }
    let u = Unit;
    do_something_with_unit(u);
    println!("Success!");
    fn do_something_with_unit(u: Unit) {   }
}
#[test]
fn struct_3()
{
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let v = Color(0, 127, 255);
    check_color(v);
    println!("Success!");
    fn check_color(p: Color) {
        let (x, y, z) = (p.0, p.1, p.2);
        assert_eq!(x, 0);
        assert_eq!(p.1, 127);
        assert_eq!(z, 255);
    }
}
#[test]
fn struct_4()
{
    struct Person {
        name: String,
        age: u8,
    }
    let age = 18;
    let mut p = Person {
        name: String::from("sunface"),
        age,
    };
    p.age = 30;
    p.name = String::from("sunfei");
    println!("Success!");
}
#[test]
fn struct_5()
{
    struct Person
    {
        name: String,
        age: u8,
    }
    println!("Success!");
    fn build_person(name: String, age: u8) -> Person
    {
        Person
        {
            age,
            name
        }
    }
}
#[test]
fn struct_6()
{
    struct User
    {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    let u1 = User
    {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };
    let u2 = set_email(u1);
    println!("Success!");
    fn set_email(u: User) -> User
    {
        User
        {
            email: String::from("contact@im.dev"),
            username: u.username,
            active: u.active,
            sign_in_count: u.sign_in_count,
        }
    }
}
#[test]
fn struct_7()
{
    #[derive(Debug)]
    struct Rectangle
    {
        width: u32,
        height: u32,
    }
    let scale = 2;
    let rect1 = Rectangle
    {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
    println!("{:?}", rect1);
#[test]
fn struct_8()
{
    #[derive(Debug)]
    struct File
    {
        name: String,
        data: String,
    }
    let f = File
    {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };
    let _name = f.name;
    println!("{}, {}, {:?}", _name, f.data, File { name: _name.clone(), data: f.data.clone() });
}
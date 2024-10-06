#[derive(Debug)]
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//tuple struct
#[derive(Debug)]
struct Color(i32, i32, i32);

struct Rectangle{
    width: u32,
    height: u32
}

//methods
impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }
}


fn main() {
    println!("Creating User");

    let mut user = User{
        active: true,
        username: String::from("Anjana"),
        email: String::from("anjana@gmail.com"),
        sign_in_count: 10,
    };

    user.username = String::from("anjana2002@gmail.com");
    println!("user {:?}", user);

    let user2 = create_user("Arjun".to_string(), "arjun@gmail.com".to_string());
    println!("user2 {:?}", user2);

    let user3 = User{
        username: String::from("Amal"),
        ..user
    };
    println!("user3 {:?}", user3); // now we can't use user 
    
    //tuple struct 
    let color = Color(2,5,1);
    println!("Color: {:?}", color);

    let rectangle = Rectangle{
        width:10,
        height:12,
    };

    println!("Area of rectangle: {}", rectangle_area(&rectangle));
    println!("Area of rectangle using method: {}", rectangle.area());
}

fn rectangle_area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}

fn create_user(username: String, email: String) -> User  {
    User { active: true, username,  email, sign_in_count: 1 }
}
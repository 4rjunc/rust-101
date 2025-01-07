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

    fn can_fit(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
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

    let rectangle2 = Rectangle{
        width: 15,
        height: 20
    };
    println!("Size fit: {}", rectangle2.can_fit(&rectangle));

    // Matrix
    let matrix1 = Matrix{e11: 12,e12: 6, e21: 7,e22: 8};
    let matrix2 = Matrix{e11: 3,e12: 4, e21: 1,e22: 0};

    let sum  = matrix1.add(&matrix2);
    sum.display();
}


fn rectangle_area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}

fn create_user(username: String, email: String) -> User  {
    User { active: true, username,  email, sign_in_count: 1 }
}

#[derive(Debug)]
struct Matrix{
    e11: i32,
    e12: i32,
    e21: i32,
    e22: i32,
}

impl Matrix {
    fn add(&self, other: &Matrix) -> Matrix{
        Matrix{
            e11: self.e11 + other.e11,
            e12: self.e12 + other.e12,
            e21: self.e21 + other.e21,
            e22: self.e22 + other.e22,
        }
    }

    fn display(&self){
        println!("[{} {}]", &self.e11, &self.e12);
        println!("[{} {}]", &self.e21, &self.e22);
    }

    
}

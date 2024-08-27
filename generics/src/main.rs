// struct generics
struct Point<T>{
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T{
        &self.x
    }
}

impl Point<f64>  {
    fn y(&self, scale: f64) -> f64{
        return &self.y * scale;
    }
}


fn main() {
    let num_list = vec![67,45,69,101,56,3];
    let largest_number = get_largest(num_list);
    println!("Largest Number: {}", largest_number);

    let char_list = vec!["a", "b", "Z", "I", "o"];
    let largest_char = get_largest(char_list);
    println!("Largest Char: {}", largest_char);

    let point = Point{x:5, y:6};
    println!("Point impl: {}", point.x());

    let point_2 = Point{x:5.6, y:6.5};
    println!("Point impl: {}", point_2.y(100.0));


}

// T is the generic to represnt any type 
// PartialOrd + Copy are traits
// T has to be ordered and copied 
fn get_largest<T: PartialOrd + Copy>(num_list: Vec<T>) -> T { 
    let mut largest = num_list[0];
    for num in num_list{
        if num > largest{
            largest = num
        }
    }
    return largest;
}

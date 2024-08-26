fn main() {

    // pushing items
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);

    // fetching items
    let va = vec![2,3,4,5,6,7];

    //method 1
    let third: &i32 = &va[2];
    println!("Third item in {}", third);

    //method 2
    match va.get(2) {
        Some(third) => println!("Third element is {}", third),
        None => println!("No third element"),
    }

    //iteration
    let vb= vec![199, 89, 13, 56];
    for i in &vb{
        println!("iteration: {}", i)
    }

    //iteration and mutability
    let mut vc = vec![5, 6 ,7 ,8, 9];
    for i in &mut vc{
        *i +=5;
    }
    println!("iteration and mutability: {:?}", vc);

    // iteration method 2
    let vb = vec!["apple", "orange", "grapes", "pineapple"];
    for (index, item) in vb.iter().enumerate(){
        println!("{item} at index {index}");
    }
}

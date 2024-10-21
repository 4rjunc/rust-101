enum IpAddr{
    V4(u8,u8,u8,u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn message_handle(&self){
        match self{
            Message::Write(text) => println!("Message::Write: {}", text),
            _=>println!("Nothing to print")
        }
    }
}

#[derive(Debug)]
enum States {
    Kerala,
    Karnataka,
    TamilNadu,
    Goa,
    Telugana
}

enum Food{
    Idly,
    Dosa,
    Poha,
    Upma(States)
}

fn main() {
    let loopback = IpAddr::V6(String::from("::1"));
    let locahost = IpAddr::V4(127,0,0,1);

    let sys_message = Message::Write(String::from("System Hacked"));
    sys_message.message_handle();


    route(locahost);
    route(loopback);

    //option enum
    let marks: Option<i32> = Some(32);  
    let score: Option<i32> = None;
    let num: i32 = 5;

    let sum = marks.unwrap_or(0) + num; // mark can be None or it can have value hence .unwrap_or
    // is used
    println!("SUM: {}", sum);
   
    desi_data(Food::Upma(States::TamilNadu));
}

fn desi_data(data: Food){
    match data {
        Food::Dosa => println!("DOSA"),
        Food::Poha => println!("Poha"),
        Food::Upma(state) => println!("State Food from : {:?}", state),
        _ => println!("You are  Idly guy")
    }
}

fn route(ip_kind: IpAddr){
    match ip_kind {
        IpAddr::V4(one, two, three, four) => println!("This is V4"),
        IpAddr::V6(test) => println!("This is V6")
    }
}

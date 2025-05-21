use crossbeam;

fn main() {
    // Create a channel of unbounded capacity.
    let (s, r) = crossbeam::channel::unbounded();

    // Send a message into the channel.
    s.send("Hello, world!").unwrap();

    // Receive the message from the channel.
    assert_eq!(r.recv(), Ok("Hello, world!"));

    print!("r:{:?}", r.recv());
}

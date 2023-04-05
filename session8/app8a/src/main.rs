use std::time::Duration;

fn main() {
    std::thread::spawn(|| {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line);
        println!("Got the name {}", line);
    });
    loop {
        println!("Hello there!");
        std::thread::sleep(Duration::from_secs(1));
    }
}

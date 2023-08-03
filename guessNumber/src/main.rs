use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数!");
    println!("请猜一个数!");

    let secret_number: u32 = rand::thread_rng().gen_range(0..=60);
    println!("神秘数字是：x!请猜猜看");
    loop {
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你猜测的数是{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("--------------小了！---------------"),
            Ordering::Greater => println!("--------------大了！--------------"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        };
    }
}

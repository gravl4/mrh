fn main() {
    println!("Hello, world!");
    //let output = "Hello World";
    println!("\x1b[32m Вывод моего цвета и формата \x1b[0m");

    use console::style;
    //use indicatif::ProgressBar;

    println!("This is {} neat", style("quitesd").cyan().bold());

    let mut se = String::from("string hello");
    println!("{}", se);

    se.push_str(", nneemm!");
    println!("{}", style(se).green().bold());

    let mt1: u32 = 124;
    println!("{}", mt1);

    let my_string = String::from("hellohj fkhgworld");
    let word = first_word(&my_string[..]).0;
    println!("{}", word);

    let a = 3;
    let b = 1 + 2;
    assert_eq!(a, b);

    assert_eq!(a, b, "we are testing addition with {} and {}", a, b);

    let plus_one = |x: i32| x + 1;
    let plus_2 = |x: i32| -> i32 { x + 2 };
    println!("замыкания {}, {}", plus_one(3), plus_2(6));

    {
        let num = 5;
        let owns_num = move |x: i32| x + num;

        //println!("замыкания move {}", add_num(6));
        //owns_num(6);
        println!("замыкания move {}", owns_num(6));
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle::square(12);
    println!("rect1 is {:?}", rect1);
    println!("rect3 is {:?}", rect3);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let x_state = UsState::Alaska;
    let x_coin = Coin::Quarter(x_state);
    let z_coin = x_coin.clone();
    value_in_cents(x_coin);
    println!("State quarter from {:?}!", z_coin);
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn first_word(s: &str) -> (&str, usize) {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return (&s[0..i], i);
        }
    }

    (&s[..], s.len())
}

#[derive(Debug)] // включить вывод содержимого
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)] // отключаем предупреждение о неиспользуемом коде
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

#[derive(Debug)] // so we can inspect the state in a minute // разрешить выводить
#[allow(dead_code)]
// не выводить предупреждение при неиспользуемом перечисления
#[derive(Clone)] // разрешение клонирования перечисления
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
#[allow(dead_code)]
#[derive(Clone)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

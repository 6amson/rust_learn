// use regex::Regex;

#[derive(Debug)]
// Define a tuple struct
struct KeyPress(String, char);
//to make structs and enums loggable with specail formatting, use [derive()debug]
#[derive(Debug)]
// Define a classic struct
struct MouseClick {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct KeyNames {
    name: String,
    email: u32,
}

#[derive(Debug)]
enum WebEvent {
    WELoad(bool),
    WEClick(MouseClick),
    WEKeys(KeyPress),
}
//enum is used for properties that have a limited expression e.g height: tall/short
// struct acts like an object in javascript

fn main() {
    let _number: &str = "workaholic";
    let _madness = ('E', true, 4u32);
    println!(
        "Hello, world, are you ready!, youre {} years old",
        _madness.2
    );

    // Redefine the enum variants to use the data from the new structs
    // Update the page Load variant to have the boolean type
    let keys = KeyPress(String::from("Ctrl+"), 'N');
    println!("The letter {} is contained here", keys.1);

    let keynames = KeyNames {
        name: String::from("Samson"),
        email: 675,
    };
    println!(
        "My name is {}, and here is my email: {}",
        keynames.name, keynames.email
    );

    let click = MouseClick { x: 100, y: 250 };
    println!("Mouse click location: {}, {}", click.x, click.y);

    let we_load = WebEvent::WELoad(true);
    let we_key = WebEvent::WEKeys(keys);

    println!("{:#?}", we_key);

    //Vectors
    let numbers1 = vec![5, 6, 7];
    println!("numbers1 array: {:?}", numbers1[2]);

    let mut fruit = Vec::new();
    fruit.push("mango");

    println!("fruit: {} fruit array: {:?}", fruit[0], fruit);

    let popped = fruit.pop();
    println!("popped value: {:?} fruit arrray: {:?}", popped, fruit);

    //using lifetimes
    fn workit<'a>(x: &'a mut String) -> &String {
        x.push('!');
        x
    }

    //traits
    const PI: f64 = 3.142;

    trait Area {
        fn area(&self) -> f64;
        fn circum(&self) -> f64;
    }

    struct Rect {
        length: f64,
        width: f64,
    }

    struct Circle {
        radius: f64,
    }

    let circle = Circle { radius: 5.0 };

    let rect = Rect {
        length: 10.0,
        width: 2.00,
    };

    impl Area for Rect {
        fn area(&self) -> f64 {
            // format!("{}", self.width * self.length)
            self.length * self.width
        }

        fn circum(&self) -> f64 {
            self.length + self.width
        }
    }

    impl Area for Circle {
        fn area(&self) -> f64 {
            f64::powf(self.radius, 2.0) * PI
        }

        fn circum(&self) -> f64 {
            todo!("fix implementation later")
        }
    }

    println!(
        "rectArea {} \nCircleArea {} \nRectCircum {}",
        rect.area(),
        circle.area(),
        rect.circum()
    );

    //Iterators
    struct Groups<T> {
        inner: Vec<T>,
    }

    impl<T> Groups<T> {
        fn new(inner: Vec<T>) -> Self {
            Groups { inner }
        }
    }

    impl<T: PartialEq> Iterator for Groups<T> {
        type Item = Vec<T>;

        // TODO: Write the rest of this implementation.
        fn next(&mut self) -> Option<Self::Item> {
            if self.inner.is_empty() {
                return None;
            }

            let mut cursor = 1;
            let first = &self.inner[0];
            for element in &self.inner[1..] {
                if element == first {
                    cursor += 1;
                } else {
                    break;
                }
            }
            let items = self.inner.drain(0..cursor).collect();

            Some(items)
        }
    }

    fn main2() {
        let data = vec![4, 1, 1, 2, 1, 3, 3, -2, -2, -2, 5, 5];
        // groups:     |->|---->|->|->|--->|----------->|--->|
        assert_eq!(
            Groups::new(data).into_iter().collect::<Vec<Vec<_>>>(),
            vec![
                vec![4],
                vec![1, 1],
                vec![2],
                vec![1],
                vec![3, 3],
                vec![-2, -2, -2],
                vec![5, 5],
            ]
        );

        let data2 = vec![1, 2, 2, 1, 1, 2, 2, 3, 4, 4, 3];
        // groups:      |->|---->|---->|----|->|----->|->|
        assert_eq!(
            Groups::new(data2).into_iter().collect::<Vec<Vec<_>>>(),
            vec![
                vec![1],
                vec![2, 2],
                vec![1, 1],
                vec![2, 2],
                vec![3],
                vec![4, 4],
                vec![3],
            ]
        )
    }

    trait AsJSON {
        fn as_json(&self) -> String;
    }

    mod text_processing {

        pub mod letters {
            pub fn count_letters(text: &str) -> usize {
                text.chars().filter(|ref c| c.is_alphabetic()).count()
            }
        }

        pub mod numbers {
            pub fn count_numbers(text: &str) -> usize {
                text.chars().filter(|ref c| c.is_numeric()).count()
            }
        }
    }

    fn count_letters_and_numbers(text: &str) -> (usize, usize) {
        let number_of_letters = text_processing::letters::count_letters(text);
        let number_of_numbers = text_processing::numbers::count_numbers(text);
        (number_of_letters, number_of_numbers)
    }

    fn main() {
        assert_eq!(count_letters_and_numbers("221B Baker Street"), (12, 3));
        assert_eq!(count_letters_and_numbers("711 Maple Street"), (11, 3));
        assert_eq!(count_letters_and_numbers("4 Privet Drive"), (11, 1));
    }
}

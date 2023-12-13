#[derive(Debug)]
    // Define a tuple struct
    struct KeyPress(String, char);
//to make structs and enums loggable with specail formatting, use [derive()debug]
#[derive(Debug)]
// Define a classic struct
struct MouseClick { x: i64, y: i64 }


#[derive(Debug)]
struct KeyNames{name: String, email: u32}


#[derive(Debug)]
enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress) }
//enum is used for properties that have a limited expression e.g height: tall/short
// struct acts like an object in javascript


fn main() {
    let _number:  &str = "workaholic";
    let _madness = ('E', true, 4u32);
    println!("Hello, world, are you ready!, youre {} years old", _madness.2);

    // Redefine the enum variants to use the data from the new structs
    // Update the page Load variant to have the boolean type
    let keys = KeyPress(String::from("Ctrl+"), 'N');
    println!("The letter {} is contained here", keys.1);

    let keynames = KeyNames{name: String::from("Samson"), email: 675};
    println!("My name is {}, and here is my email: {}", keynames.name, keynames.email );

    let click = MouseClick { x: 100, y: 250 };
    println!("Mouse click location: {}, {}", click.x, click.y);


    let we_load = WebEvent::WELoad(true);
    let we_key = WebEvent::WEKeys(keys);

    println!("{:#?}", we_key);

    //Vectors
    let numbers1 = vec![5,6,7];
    println!("numbers1 array: {:?}", numbers1[2]);

    let mut fruit = Vec::new();
    fruit.push("mango");

    println!("fruit: {} fruit array: {:?}", fruit[0], fruit);

    let popped = fruit.pop();
    println!("popped value: {:?} fruit arrray: {:?}", popped, fruit);

}

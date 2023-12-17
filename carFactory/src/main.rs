#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    NEW,
    USED,
}

// #[derive(PartialEq, Debug)]
// enum Option<T>{
//     None,
//     Some<String::from("Corn")>
// }

const Manual: Transmission = Transmission::Manual;
const SemiAuto: Transmission = Transmission::SemiAuto;
const Automatic: Transmission = Transmission::Automatic;
const new_age: Age = Age::NEW;
const used_age: Age = Age::USED;

use std::collections::HashMap;

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    }
}

fn car_quality(miles: u32) -> (Age, u32) {
    if miles > 0 {
        let quality = (Age::USED, miles);

        quality
    } else {
        let quality = (Age::NEW, miles);

        quality
    }
}

fn main() {
    let colors = ["Blue", "Green", "Red", "Silver"];

    //you can only use "let" and mutable variablesinside a function
    //HASHMAP
    let mut orders: HashMap<i32, Car> = HashMap::new();

    // println!("{:?}", new_age);
    // let mut car: Car;
    let car = car_factory(String::from(colors[1]), Manual, true, 230);
    println!(
        "Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // println!("{:?}", car)
    let order: i32 = 3;
    // orders(order, car);

    orders.insert(order, car);
    println!("car order {}: {:?}", order, orders.get(&order));

    let vehicles = vec!["acura", "benz", "toyota", "nissan"];
    for vehicle in vehicles.iter() {
        // println!("this is a {} kinda car", vehicle);
    }

    for &index in [0, 2, 99].iter() {
        match vehicles.get(index) {
            Some(&"acura") => println!("Acura is the best"),
            Some(veh) => println!("{} is a pretty car", veh),
            // _ => {},
            None => println!("no car here"),
        }
    }

    let someNumber: Option<u8> = Some(5);
    match someNumber {
        Some(8) => println!("got the wanted number"),
        _ => {}
    }

    let gift = Some("Candy");
    assert_eq!(gift.unwrap(), "Candy");

    let gift2: Option<i8> = None;
    // let gift2 : Option::None;
    assert_eq!(gift2.unwrap(), 4);
}

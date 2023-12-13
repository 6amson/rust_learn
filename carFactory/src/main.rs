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
enum Transmission { Manual, SemiAuto, Automatic }


#[derive(PartialEq, Debug)]
enum Age {NEW, USED}

const Manual:Transmission = Transmission::Manual;
const SemiAuto:Transmission = Transmission::SemiAuto;
const Automatic:Transmission = Transmission::Automatic;
const new_age:Age = Age::NEW;
const used_age:Age = Age::USED;


fn car_factory (color: String, motor: Transmission, roof: bool, miles: u32) -> Car{
   
    // if car.age.0 == USED {
        
    // }
    
    Car{
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    }

}

fn car_quality (miles: u32) -> (Age, u32){
    if miles > 0 {
        let quality = (Age::USED, miles);

        quality
    }else{
        let quality = (Age::NEW, miles);

        quality
    }
    
}

fn main() {

    let colors = ["Blue", "Green", "Red", "Silver"];

    // println!("{:?}", new_age);
    // let mut car: Car;
    let car = car_factory(String::from(colors[1]), Manual, true, 230);
    // println!("{:?}", car)
    println!("Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);
}




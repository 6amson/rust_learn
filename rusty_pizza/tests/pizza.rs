use rusty_pizza::Pizza;

#[test]
fn make_pepperoni (){
    let pep = Pizza::pepperoni(15);
    assert_eq!(pep.get_inches(), 15);
    assert_eq!(pep.topping, "pepperoni");
}
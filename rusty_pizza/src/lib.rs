
pub struct Pizza {
    inches: usize,
    pub topping: String
}

impl Pizza {
    pub fn pepperoni (inches: usize) -> Self{
        Pizza::bake("pepperoni", inches)
    }

    pub fn mozzarella (inches: usize) -> Self{
        Pizza::bake("mozzarella", inches)
    }

    pub fn get_inches(&self) -> usize{
        self.inches
    }

    fn bake (toppings: &str, inches: usize) -> Self{
        Pizza{
            topping: String::from(toppings),
            inches: inches
        }
    }
}
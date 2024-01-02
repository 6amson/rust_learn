pub struct Car_honk{
    MANUAL: String,
    pub AUTO: String,
    pub SEMI_AUTO: String,
}

impl Car_honk{
    pub fn new (manual: String, auto: String, semi_auto: String) -> Car_honk{
        Car_honk {
            MANUAL: manual,
            AUTO: auto,
            SEMI_AUTO: semi_auto
        }
    }

    pub fn get_manual_honk(&self) -> &String{
        &self.MANUAL
    }
}

// let mut Manual = Car_honk::new();
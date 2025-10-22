pub struct Counter {
    value: u64,
    max: u64,
}

impl Counter {
    //Create a new object with the default value of 0 and specified max.
    pub fn new(max: u64) -> Self {
        Self { value: 0, max }
    }

    //Read the value
    pub fn value(&self) -> u64 {
        self.value
    }

    //Increment the value
    pub fn inc(&mut self) -> Result<(), &'static str> {
        if self.value >= self.max {
            return Err("cap reached");
        }
        self.value += 1;
        Ok(())
    }

    //Reset the value
    pub fn reset(&mut self) {
        self.value = 0;
    }
}

fn main(){  
    let mut counter = Counter::new(1);
    
    let count_value = counter.value();

    println!("Initial counter value: {}", count_value);

    //Let's increment the counter.
    match counter.inc() {
        Ok(()) => println!("Incremented successfully!"),
        Err(e) => println!("Error: {}", e),
    }

    println!("Counter value after increment: {}", counter.value());

    counter.reset();

    println!("Counter value after reset: {}", counter.value());
}

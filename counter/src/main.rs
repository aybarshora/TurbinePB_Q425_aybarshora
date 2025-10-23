use std::io;

pub struct Counter {
    value: u64,
    max: u64,
}

impl Counter {
    //Create a new object with the default value of 0 and specified max.
    pub fn new(max: u64) -> Self {
        Self { value: 0, max }
    }

    pub fn current_value(&self) -> u64 {
        self.value
    }

    pub fn max_value(&self) -> u64 {
        self.max
    }

    pub fn inc(&mut self) -> Result<(), &str> {
        if self.value >= self.max {
            return Err("Max cap reached");
        }
        self.value += 1;
        Ok(())
    }

     pub fn dec(&mut self) -> Result<(), &'static str> {
        if self.value <= 0 {
            return Err("Min cap reached");
        }
        self.value -= 1;
        Ok(())
    }

    pub fn reset(&mut self) {
        self.value = 0;
    }
}


fn main(){  
    let mut counter = Counter::new(5);

    println!("\n Counter created with the max value {}", counter.max_value());

    //We show menu default once at the beggining so it doesn't fill the whole CLI 
    let menu = "\n=== CLI Counter App Menu ===
    1. View Count
    2. Increment Counter
    3. Decrement Counter
    4. Reset Counter
    5. Exit 
    6. Show Menu
    ==============================";
    
    println!("{}", menu);

    loop{
        println!("6 to open menu");

        println!("Please enter your choice: ");

        let mut user_choice = String::new();
        
        //Read from the std input
        io::stdin()
        .read_line(&mut user_choice)
        .expect("Failed to read line"); 

        //Converting String to u32
        let user_choice: u32 = match user_choice.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
        
        match user_choice {
           1 => println!("Current counter value: {}", counter.current_value()),
           2 => match counter.inc() {
            Ok(()) => println!("Counter Incremented!"),
            Err(e) => println!("Error: {}", e),
           }
           3 => match counter.dec() {
            Ok(()) => println!("Counter Decremented!"),
            Err(e) => println!("Error: {}", e),
           }
           4 => { 
            println!("Counter reset");
            counter.reset();
           }
           5 => {
            println!("Exiting. Final count: {}", counter.current_value());
            break;
           }
           6 => println!("{}", menu),
           _ => {
            println!("Invalid input. Please enter a number from 1 to 5.");
           }      
        }
    }
}

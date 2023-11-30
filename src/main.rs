use std::error::Error;
use input_macro::input;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    loop {
        let mut line = input!().split_whitespace().collect::<String>().as_str();
        
        //for 
    }

    Ok(())
}

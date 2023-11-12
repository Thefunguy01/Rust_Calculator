//Imports Dependencies
use std::io::prelude::*;
use std::io;

//Main function
fn main() {

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
        
    stdout.flush().unwrap();
    
    println!("Press 1, 2, 3 or 4 (1 = + , 2 = - , 3 = x , 4 = ÷)");
    let mut Mode = String::new();
    io::stdin().read_line(&mut Mode);
    let my_Mode = Mode.trim().parse::<i32>().unwrap();

    //Addition
    if my_Mode == 1 {
        
        println!("What is the 1st number?");
        let mut OneN = String::new();

        io::stdin().read_line(&mut OneN);

        println!("What is the 2nd number?");
        let mut TwoN = String::new();

        io::stdin().read_line(&mut TwoN);

        let my_OneN = OneN.trim().parse::<f32>().unwrap();
        let my_TwoN = TwoN.trim().parse::<f32>().unwrap();

        let Q = my_OneN + my_TwoN;

        println!("The aswer is {} ", Q );

    }

    //Subtraction
    if my_Mode == 2 {
        
        println!("What is the 1st number?");
        let mut OneN = String::new();

        io::stdin().read_line(&mut OneN);

        println!("What is the 2nd number?");
        let mut TwoN = String::new();

        io::stdin().read_line(&mut TwoN);

        let my_OneN = OneN.trim().parse::<f32>().unwrap();
        let my_TwoN = TwoN.trim().parse::<f32>().unwrap();

        let Q = my_OneN - my_TwoN;

        println!("The aswer is {} ", Q );

    }

    //Multiplication
    if my_Mode == 3 {
        
        println!("What is the 1st number?");
        let mut OneN = String::new();

        io::stdin().read_line(&mut OneN);

        println!("What is the 2nd number?");
        let mut TwoN = String::new();

        io::stdin().read_line(&mut TwoN);

        let my_OneN = OneN.trim().parse::<f32>().unwrap();
        let my_TwoN = TwoN.trim().parse::<f32>().unwrap();

        let Q = my_OneN * my_TwoN;

        println!("The aswer is {} ", Q );

    }

    //Devision
    if my_Mode == 4 {
        
        println!("What is the divident?");
        let mut Divident = String::new();

        io::stdin().read_line(&mut Divident);

        println!("What is the diviser?");
        let mut Diviser = String::new();

        io::stdin().read_line(&mut Diviser);

        let my_Divident = Divident.trim().parse::<f32>().unwrap();
        let my_Diviser = Diviser.trim().parse::<f32>().unwrap();

        let Q = my_Divident / my_Diviser;

        println!("The aswer is {} ", Q );

    }

    //Close when done
    {  
        write!(stdout, "Press any key to close...").unwrap();
        stdout.flush().unwrap();

        let _ = stdin.read(&mut [0u8]).unwrap();
    }
}

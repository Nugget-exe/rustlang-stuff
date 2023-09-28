fn main() {
    let mut number = 1;
    
    for number in 1...100 {
        if number % 5 == 0 & number % 3 == 0 {
            println!("fizzbuzz"); 
        } else if number % 5 == 0 {
            println!("Buzz");
        } else if number % 3 == 0 {
            println!("Fizz");
        }
            
        
    }
}

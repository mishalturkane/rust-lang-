use std::io;

fn main() {
    let mut input = String::new();
    
    println!("Enter your investment amount in USD:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let investment: f64 = input.trim().parse().expect("Please enter a valid number");
    input.clear();
    
    println!("Enter the price per SOL at purchase:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let buy_price: f64 = input.trim().parse().expect("Please enter a valid number");
    input.clear();
    
    println!("Enter the price per SOL at selling:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let sell_price: f64 = input.trim().parse().expect("Please enter a valid number");
    
    // Calculate the number of SOL purchased
    let sol_quantity = investment / buy_price;

    // Calculate the total selling amount
    let selling_amount = sol_quantity * sell_price;

    // Calculate profit
    let profit = selling_amount - investment;

    println!("Investment: ${}", investment);
    println!("SOL Purchased: {:.4} SOL", sol_quantity);
    println!("Selling Amount: ${:.2}", selling_amount);
    println!("Profit: ${:.2}", profit);
}

fn main() {
    let amounts: [f64; 5] = [
        450000.0,
        1500000.0,
        750000.0,
        2850000.0,
        250000.0,
    ];

    let mut sum: f64 = 0.0;

    for amount in amounts {
        sum = sum + amount;
    }

    let average = sum / 5.0;

    println!("Total Sales = ₦{}", sum);
    println!("Average Sales = ₦{}", average);
}
    



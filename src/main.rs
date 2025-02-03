use compute_pi::compute_pi_str;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    // Specify the number of digits of Pi you want to compute
    let digits = 500000;
    {
      // Compute Pi
      //let pi = compute_pi_str(digits);
      compute_pi_str(digits);

      // Print calculated decimal
      //println!("Pi to {} decimal places: {}", digits, pi);
    }

    let elapsed = now.elapsed();
    //println!("Computed {} decimals - Elapsed: {:.2?}", digits, elapsed);
    println!("{}",elapsed.as_millis());
}

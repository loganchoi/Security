mod eve;

fn main() {
    // Purpose:    Driver for DH problems
    // Parameters: None
    // User Input: If no args, input dec numbers
    // Prints:     If no args, then print result
    // Returns:    Nothing
    // Modifies:   Nothing
    // Calls:      ?
    // Tests:      arg_tests/ and stdio_tests/
    // Status:     Student does this
    use std::io::Write;
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        let mut aliceb = String::new();
        let mut bobb = String::new();
        let mut key = String::new();
        std::io::stdin().read_line(&mut aliceb);
        std::io::stdin().read_line(&mut bobb);
        std::io::stdin().read_line(&mut key);
        let num_aliceb: u64 = aliceb.trim().parse().expect("");
        let num_bobb: u64 = bobb.trim().parse().expect("");
        let num_key: u64 = key.trim().parse().expect("");
        let result: [u64; 3] = crate::eve::baby_eve(num_aliceb, num_bobb, num_key);
        println!("{} {} {}", result[0], result[1], result[2]);
    } else {
        let file = std::fs::read_to_string(&args[1]).expect("Failed");
        let nums: Vec<&str> = file.split_whitespace().collect();
        let num_aliceb: u64 = nums[0].trim().parse().expect("");
        let num_bobb: u64 = nums[1].trim().parse().expect("");
        let num_key: u64 = nums[2].trim().parse().expect("");
        let result: [u64; 3] = crate::eve::baby_eve(num_aliceb, num_bobb, num_key);
        let mut out = std::fs::File::create(&args[2]).unwrap();
        let mut ans = String::new();
        ans = format!("{} {} {}", result[0], result[1], result[2]);
        write!(&mut out, "{}", ans);
    }
}

pub fn baby_eve(alice_broadcasts: u64, bob_broadcasts: u64, public_base: u64) -> [u64; 3] {
    // Purpose:     Crack baby DH
    // Parameters:  alice's broadcast, bob's broadcast, and the public base
    // User-input:  None
    // Prints:      Nothing
    // Returns:     Should return an array of 3 ints:
    //              Alice's secret, Bob's secret, shared secret
    // Modifies:    Nothing
    // Calls:       ?
    // Tests:       unit_test_babydh.rs
    // Status:      Done, correct, but is it ideal?
    let alice = (alice_broadcasts as f64)
        .powf(1.0 / (public_base as f64))
        .round();
    let bob = (bob_broadcasts as f64)
        .powf(1.0 / (public_base as f64))
        .round();
    let secret = (alice_broadcasts as f64).powf(bob);
    [(alice as u64), (bob as u64), (secret as u64)]
}

// This function is bonus (see the README.md for details)
// pub fn big_eve(
//     alice_broadcasts: i64,
//     bob_broadcasts: i64,
//     public_base: i64,
//     public_modulus: i64,
// ) -> [u64; 3] {
//     // Purpose:      Crack real DH (albeit not with huge numbers)
//     // Parameters:   Alice's broadcast, Bob's broadcast, the public base and modulus of DH.
//     // User-input:   None
//     // Prints:       Nothing
//     // Returns:      Should return an array of 3 ints:
//     //               Alice's secret, Bob's secret, shared secret
//     // Modifies:     Nothing
//     // Calls:        ?
//     // Test:         ./unit_tests/unit_test_babydh.rs
//     // Status:       TODO delete the 0 placeholders, and replace with real computations
//     let alice_secret = 0;
//     let bob_secret = 0;
//     let mutual_secret = 0;
//     [alice_secret as i64, bob_secret as i64, mutual_secret as i64]
// }

fn main() {
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    let mut mantissa: f32 = 1.0;

    // in an `f32`, the mantissa occupies bits 0..=22
    for i in 0..23 {
        // create a mask for a bit position based on `i`
        let mask = 1 << i;
        // filter the bits from the original number
        let one_at_bit_i = n_bits & mask;
        // if the bit at position `i` is non-zero, continue the decoding process
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            // When `i` is 0, we're rasing 2.0 to the power of -23, which is ~0.0000005
            let weight = 2_f32.powf(i_ - 23.0);
            println!("Weight: {}", &weight);
            // By adding the weight
            mantissa += weight;
            println!("Mantissa: {}\n", &mantissa);
        }
    }

    println!("Decoded mantissa: {mantissa}");
}

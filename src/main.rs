const EXPONENT_BIAS: i32 = 127;

fn main() {
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    // Shifts exponent's 8 bits to right, overwriting the mantissa
    let exponent_ = n_bits >> 23;
    // Passes `exponent_` through an [AND bit mask](https://en.wikipedia.org/wiki/Mask_(computing)
    // to set all but the first 8 bits to 0, which effectively removes the sign bit
    let exponent_ = exponent_ & 0xff;
    // Casts the exponent and applied the exponent bias to arrive at the final value
    let exponent = (exponent_ as i32) - EXPONENT_BIAS;

    println!("{exponent}");
}

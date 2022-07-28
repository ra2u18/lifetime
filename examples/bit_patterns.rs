use std::intrinsics::transmute;

const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn main (){
    // endianness();
    // println!("{:?}", get_sign_bit(-42.42));
    // println!("{:?}", 1 << 2);

    let n: f32 = 42.42;
    let (sign, exp, frac) = to_parts(n);
    let (sign_, exp_, mant) = decode(sign, exp, frac);

    let n_ = from_parts(sign_, exp_, mant);
    
    println!("{} -> {}", n, n_);
    println!("field | as bits | as real number");
    println!("sign | {:01b} | {}", sign, sign_);
    println!("exponent | {:08b} | {}", exp, exp_);
    println!("mantissa | {:023b} | {}", frac, mant);
}

fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32{
    sign * exponent * mantissa
}

fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();

    let sign = (bits >> 31) & 1;
    let exponent = (bits >> 23) & 0xff;
    let fraction = bits & 0x7fffff;

    (sign, exponent, fraction)
}

fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    let mut mantissa: f32 = 1.0;

    // Coverts the sign bit to 1.0 or -1.0. Paratheses are required around -1.0
    // to clarify operator precedence as method calls rank higher than unary minus
    let signed_1 = (-1.0_f32).powf(sign as f32);

    let exponent = (exponent as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i =  fraction & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }

    (signed_1, exponent, mantissa)
}

// fn endianness() {
//     let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
//     let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

//     let a: i32 = unsafe { transmute(big_endian) };
//     let b: i32 = unsafe { transmute(little_endian) };

//     println!("{} vs {}", a, b);
// }

// fn get_sign_bit(n: f32) -> u32 {
//     let n_bits: u32 = n.to_bits();
//     n_bits >> 31
// }

// fn get_exponent_bit(n: f32) -> i32 {
//     let n_bits = n.to_bits();
//     let exponent_ = n_bits >> 23; // bring exponent to the right
//     let exponent_ = exponent_ & 0xff; // remove exponent_bit
//     (exponent_ as i32) - 127 // interpret as i32 and subtract the bias
// }

// fn get_mantissa(n: f32) -> f32 {
//     let n_bits: u32 = n.to_bits();
//     let mut mantissa: f32 = 1.0;

//     for i in 0..23 {
//         let mask = 1 << i;
//         let one_at_bit_i = n_bits & mask;

//         if one_at_bit_i != 0 {
//             let i_ = i as f32;
//             let weight = 2_f32.powf(i_ - 23.0);
//             mantissa += weight;
//         }
//     }

//     mantissa
// }
/// The Q format is a fixed-point number that uses a single byte.
/// It was create for embedded devices.
/// 
/// There are 7 bits available for the represented number plus 1 sign bit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Q7(i8);

impl From<f64> for Q7 {
    fn from(n: f64) -> Self {
        if n >= 1.0 {
            Q7(127)
        } else if n <= -1.0 {
            Q7(-128)
        } else {
            Q7((n* 128.0) as i8)
        }
    }
}

fn main(){

}
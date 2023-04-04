mod convert_temperature;
mod fibonacci;

fn main() {
    for i in 0..11 {
        println!("fibonacci {} {}", i, fibonacci::fib(i));
    }

    println!("10 in F {:?}", convert_temperature::convert(10.0, 'c'));
    println!("10 in C {:?}", convert_temperature::convert(10.0, 'f'));
}

use std::num::NonZeroU32;

fn main() {
    if let Some(n) = NonZeroU32::new(5) {
        println!("The number is part of the natural numbers set: {}", n);
    } else {
        println!("The number is not part of the natural numbers set");
    }

    if let Some(n) = NonZeroU32::new(0) {
        println!("The number is part of the natural numbers set: {}", n);
    } else {
        println!("The number is not part of the natural numbers set");
    }
}

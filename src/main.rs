mod md5;

fn main() {
    let user_input = std::env::args()
        .nth(1)
        .expect("Expected input value: Hint 'cargo run -- input-value'");
    let mut user_input_8bits: Vec<u8> = user_input.bytes().collect();
    user_input_8bits = md5::Md5::pad_input(&user_input_8bits);

    let mut hasher = md5::Md5::new();

    for block in user_input_8bits.chunks(64) {
        hasher.compress(block.try_into().unwrap());
    }

    let hash = hasher.finalise();

    for byte in hash {
        print!("{:02x}", byte);
    }
    println!();
}

pub fn main() {
    let gift_message = String::from("Merry Christmas! Enjoy your gift!");
    attach_message_to_present(&gift_message);

    // The solution for the tests to pass is to clone the 'gift_message:
    // attach_message_to_present(gift_message.clone());

    // To use a reference is better and more idiomatic.
    // The gift_message is not going to be changed, so there's no need to
    // do an expensive .clone().

    println!("{}", gift_message);
}

pub fn attach_message_to_present(message: &String) {
    println!("The present now has this message: {}", message);
}

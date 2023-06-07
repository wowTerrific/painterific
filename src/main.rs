use painterific::{Colors, paint};

fn main() {
    paint(Colors::Black, "BLACK:").unwrap();
    println!(" This is the black line.\n");

    paint(Colors::Red, "RED:").unwrap();
    println!(" This is the red line.\n");

    paint(Colors::Green, "GREEN:").unwrap();
    println!(" This is the green line.\n");

    paint(Colors::Yellow, "YELLOW:").unwrap();
    println!(" This is the yellow line.\n");

    paint(Colors::Blue, "BLUE:").unwrap();
    println!(" This is the blue line.\n");

    paint(Colors::Magenta, "MAGENTA:").unwrap();
    println!(" This is the magenta line.\n");

    paint(Colors::Cyan, "CYAN:").unwrap();
    println!(" This is the cyan line.\n");

    paint(Colors::White, "WHITE:").unwrap();
    println!(" This is the White line.\n");
}
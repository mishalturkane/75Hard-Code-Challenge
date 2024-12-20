// Basic enum with no data
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let move_direction = Direction::Up;

    match move_direction {
        Direction::Up => println!("Moving up!"),
        Direction::Down => println!("Moving down!"),
        Direction::Left => println!("Moving left!"),
        Direction::Right => println!("Moving right!"),
    }
}

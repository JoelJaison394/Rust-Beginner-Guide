enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main(){

    let player_direction:Direction = Direction::Up;


//it is similar to how the swicth works
    match  player_direction {
        Direction::Up => print!("zthe player is moving in upward direction"),
        Direction::Down => print!("zthe player is moving in downward direction"),
        Direction::Left => print!("zthe player is moving in leftwards direction"),
        Direction::Right => print!("zthe player is moving in rightwards direction"),
    }

}

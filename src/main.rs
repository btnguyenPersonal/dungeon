use std::env;
mod dungeon;

fn parse_args() -> Vec<String> {
    let args: Vec<String> = env::args().skip(1).collect();
    return args;
}

fn main() {
    let args = parse_args();
    let dun = dungeon::Dungeon::new(80, 24);
    dun.print_board();
    return ();
}

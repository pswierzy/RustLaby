enum Chessman {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

enum Color {
    White, Black,
}

struct Position {
    x: u8,
    y: u8,
}

struct ChessPawn {
    type: Chessman,
    position: Position,
    color: Color,
}

impl ChessPawn {
    fn move(&mut self, new_position: Position) -> bool {
        match self.type {
            Pawn if self.color == Color::White => {
                if new_position.x == self.position.x {
                    let range = {
                        if self.position.y == 2 {
                            2
                        } else {
                            1
                        }
                    }
                    if new_position.y <= self.position.y + range && new_position.y > self.position.y {
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

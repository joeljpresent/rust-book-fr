// ANCHOR: here
enum Piece {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn valeur_en_centimes(piece: Piece) -> u8 {
    match piece {
        Piece::Penny => 1,
        Piece::Nickel => 5,
        Piece::Dime => 10,
        Piece::Quarter => 25,
    }
}
// ANCHOR_END: here

fn main() {}

enum Piece {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// ANCHOR: here
fn valeur_en_centimes(piece: Piece) -> u8 {
    match piece {
        Piece::Penny => {
            println!("Un centime porte-bonheur !");
            1
        }
        Piece::Nickel => 5,
        Piece::Dime => 10,
        Piece::Quarter => 25,
    }
}
// ANCHOR_END: here

fn main() {}

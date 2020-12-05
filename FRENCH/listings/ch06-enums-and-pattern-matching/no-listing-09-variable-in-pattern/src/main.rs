#[derive(Debug)]
enum EtatUs {
    Alabama,
    Alaska,
    // -- partie masquée ici --
}

enum Piece {
    Penny,
    Nickel,
    Dime,
    Quarter(EtatUs),
}

// ANCHOR: here
fn valeur_en_centimes(piece: Piece) -> u8 {
    match piece {
        Piece::Penny => 1,
        Piece::Nickel => 5,
        Piece::Dime => 10,
        Piece::Quarter(etat) => {
            println!("Il s'agit d'un quarter de l'État de {:?} !", etat);
            25
        },
    }
}
// ANCHOR_END: here

fn main() {
    valeur_en_centimes(Piece::Quarter(EtatUs::Alaska));
}

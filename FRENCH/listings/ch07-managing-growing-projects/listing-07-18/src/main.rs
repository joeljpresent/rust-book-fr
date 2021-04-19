use rand::Rng;
// ANCHOR: here
// -- partie masquée ici --
use std::{cmp::Ordering, io};
// -- partie masquée ici --
// ANCHOR_END: here

fn main() {
    println!("Devinez le nombre !");

    let nombre_secret = rand::thread_rng().gen_range(1..101);

    println!("Le nombre secret est : {}", nombre_secret);

    println!("Veuillez entrer un nombre.");

    let mut supposition = String::new();

    io::stdin()
        .read_line(&mut supposition)
        .expect("Échec de la lecture de l'entrée utilisateur");

    let supposition: u32 = supposition.trim().parse().expect("Veuillez saisir un nombre !");

    println!("Votre nombre : {}", supposition);

    match supposition.cmp(&nombre_secret) {
        Ordering::Less => println!("C'est plus !"),
        Ordering::Greater => println!("C'est moins !"),
        Ordering::Equal => println!("Vous avez gagné !"),
    }
}

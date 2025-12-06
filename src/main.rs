// creation des livres
use std::io::{self, Write};

struct Livre {
    titre: String,
    auteur: String,
    annee: u32,
    disponible: bool,
}

fn ajouter_livre(biblio: &mut Vec<Livre>) { 
    println!("=== Ajouter un livre ===");

    print!("Titre : ");
    io::stdout().flush().unwrap();
    let titre = lire_ligne().trim().to_string();

    print!("Auteur : ");
    io::stdout().flush().unwrap();
    let auteur = lire_ligne().trim().to_string();

    print!("Année de publication : ");
    io::stdout().flush().unwrap();
    let annee_str = lire_ligne();

    let annee: u32 = match annee_str.trim().parse() {
    Ok(val) => val,
        Err(_) => {
            println!("Année invalide, le livre n'a pas été ajouté.");
            return;
        }
    };

        let livre = Livre {
        titre,
        auteur,
        annee,
        disponible: true,
    };

    biblio.push(livre);
    println!("Livre ajouté avec succès !");
 }

fn emprunter_livre(biblio: &mut Vec<Livre>) { 

 }

fn retourner_livre(biblio: &mut Vec<Livre>) { 

 }

fn afficher_tous(biblio: &Vec<Livre>) { 
    println!("=== Tous les livres ===");

    if biblio.is_empty() {
        println!("Aucun livre dans la bibliothèque pour le moment.");
        return;
    }

    for (index, livre) in biblio.iter().enumerate() {
        let statut = if livre.disponible { "Disponible" } else { "Emprunté" };

        println!(
            "{}. \"{}\" de {} ({}) - {}",
            index + 1,
            livre.titre,
            livre.auteur,
            livre.annee,
            statut
        );
    }
 }

fn afficher_disponibles(biblio: &Vec<Livre>) { 

 }

fn menu() {
    println!();
    println!("--- Bibliothèque ---");
    println!("1. Ajouter un livre");
    println!("2. Emprunter un livre");
    println!("3. Retourner un livre");
    println!("4. Afficher tous les livres");
    println!("5. Afficher les livres disponibles");
    println!("6. Quitter");
}

fn lire_ligne() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur lors de la lecture de l'entrée");
    input
}

fn main() {
    let mut bibliotheque: Vec<Livre> = Vec::new();

    loop {
        menu();
        print!("Votre choix : ");
        io::stdout().flush().unwrap();

        let choix = lire_ligne();

        match choix.trim() {
            "1" => ajouter_livre(&mut bibliotheque),
            "2" => emprunter_livre(&mut bibliotheque),
            "3" => retourner_livre(&mut bibliotheque),
            "4" => afficher_tous(&bibliotheque),
            "5" => afficher_disponibles(&bibliotheque),
            "6" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide."),
        }
    }
}
let mut bibliotheque: Vec<Livre> = Vec::new();

struct Livre {
    titre: String,
    auteur: String,
    annee: u32,
    disponible: bool,
}

fn ajouter_livre(biblio: &mut Vec<Livre>) { ... }
fn emprunter_livre(biblio: &mut Vec<Livre>) { ... }
fn retourner_livre(biblio: &mut Vec<Livre>) { ... }
fn afficher_tous(biblio: &Vec<Livre>) { ... }
fn afficher_disponibles(biblio: &Vec<Livre>) { ... }

println!("--- Bibliothèque ---");
println!("1. Ajouter un livre");
println!("2. Emprunter un livre");
println!("3. Retourner un livre");
println!("4. Afficher tous les livres");
println!("5. Afficher les livres disponibles");
println!("6. Quitter");

if biblio.iter().any(|l| l.titre == titre) {
    println!("Un livre avec ce titre existe déjà !");
    return;
}

match biblio.iter_mut()
    .find(|l| l.titre == titre && l.disponible) {
        Some(livre) => livre.disponible = false,
        None => println!("Livre introuvable ou indisponible"),
}

match biblio.iter_mut()
    .find(|l| l.titre == titre && l.disponible) {
        Some(livre) => livre.disponible = true,
        None => println!("Livre introuvable ou indisponible"),
}

fn menu() {
    println!("--- Bibliothèque ---");
    println!("1. Ajouter un livre");
    println!("2. Emprunter un livre");
    println!("3. Retourner un livre");
    println!("4. Afficher tous les livres");
    println!("5. Afficher les livres disponibles");
    println!("6. Quitter");
}

fn main() {
    
}
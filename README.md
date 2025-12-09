# Mini Système de Gestion de Bibliothèque (Rust)

Ce projet est une application en ligne de commande écrite en Rust qui permet de gérer une petite bibliothèque de livres.  
L’application fonctionne via un menu texte interactif et manipule une collection de livres stockés en mémoire.

Ce projet s’inscrit dans le cadre d’un TP noté de Rust.

---

## 🎯 Objectifs

- Manipuler des **structures (`struct`)**
- Utiliser un **`Vec<T>`** pour stocker des données en mémoire
- Gérer les entrées utilisateur en console (`stdin`, `stdout`)
- Mettre en place un **menu interactif** dans une boucle (`loop` + `match`)
- Pratiquer les notions de base de Rust :
  - emprunts (`&` / `&mut`)
  - mutabilité
  - `match` sur les conversions (`parse`)
  - gestion simple d’erreurs avec `expect` et `match`

---

## 🧱 Structure principale du code


Pour l’instant, tout le code est dans un seul fichier :

- `src/main.rs`

Le type principal est la structure `Livre` :

```rust
struct Livre {
    titre: String,
    auteur: String,
    annee: u32,
    disponible: bool,
}
```

📜 Fonctionnalités
1. Ajouter un livre

Fonction : ajouter_livre(biblio: &mut Vec<Livre>)

2. Emprunter un livre
Si la bibliothèque est vide, affiche un message et quitte la fonction.

Affiche la liste des livres avec leur numéro, leur titre, auteur, année.

Demande à l’utilisateur :

Numéro du livre à emprunter :

Convertit le numéro saisi en usize avec parse.

Récupère le livre correspondant dans le vecteur (biblio[choix_num - 1]).

Si le livre est disponible :

le passe à disponible = false

affiche un message de confirmation.

Sinon, affiche que le livre est déjà emprunté.

3. Retourner un livre

Cette fonction fait l’inverse de l’emprunt :

Vérifie si la bibliothèque est vide.

Affiche uniquement les livres empruntés (disponible == false) avec un numéro.

Demande à l’utilisateur :

Numéro du livre à rendre :

Convertit la saisie en usize.

Récupère le livre correspondant dans le vecteur.

Si le livre est actuellement emprunté (disponible == false), il est remis à true et un message confirme le retour.

Si le livre est déjà disponible, un message indique qu’il était déjà rendu.

4. Afficher tous les livres

Affiche un en-tête : "=== Tous les livres ===".

Si la bibliothèque est vide, affiche un message et retourne.

Sinon, parcourt le vecteur avec :

```rust
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
```
Chaque livre est affiché avec :

numéro (index + 1)

titre

auteur

année

statut (Disponible ou Emprunté)

5. Afficher uniquement les livres disponibles

Si la bibliothèque est vide, affiche un message.

Parcourt tous les livres.

N’affiche que ceux pour lesquels livre.disponible == true.

```rust
for (index, livre) in biblio.iter().enumerate() {
    if livre.disponible {
        println!(
            "{}. \"{}\" de {} ({})",
            index + 1,
            livre.titre,
            livre.auteur,
            livre.annee,
        );
    }
}
```
⌨️ Gestion des entrées utilisateur

```rust
fn lire_ligne() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur lors de la lecture de l'entrée");
    input
}
```
La fonction lire_ligne() centralise la lecture depuis le clavier :

lire les choix du menu

lire le titre, l’auteur, l’année

lire les numéros de livre à emprunter / retourner

▶️ Lancer le projet
Prérequis

Rust installé (via rustup
)

Toolchain Windows configurée (MSVC ou GNU)

Commandes

Dans le dossier du projet :

cargo run
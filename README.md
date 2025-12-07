# Mini Système de Gestion de Bibliothèque (Rust)

Ce projet est une application en ligne de commande écrite en Rust qui permet de gérer une petite bibliothèque de livres.  
L’application fonctionne via un menu texte interactif et manipule une collection de livres stockés en mémoire.

Ce projet s’inscrit dans le cadre d’un TP noté de Rust.

---

## 🎯 Objectifs

- Manipuler des **structures (`struct`)**
- Utiliser un **`Vec<T>`** pour stocker des données
- Gérer les entrées utilisateur en console
- Mettre en place un menu dans une **boucle** (`loop` + `match`)
- Pratiquer les notions de base de Rust : emprunts, mutabilité, `match`, gestion simple d’erreurs, etc.

---

## 🧱 Structure principale du code

Tout le code est pour l’instant dans un seul fichier :

- `src/main.rs` 

📜 Fonctionnalités
1. Ajouter un livre

Fonction : ajouter_livre(biblio: &mut Vec<Livre>)

Demande à l’utilisateur :

le titre du livre

l’auteur

l’année de publication

Convertit l’année saisie (String) en u32

Crée une instance de Livre avec disponible = true

Ajoute le livre au vecteur bibliotheque

Affiche un message de confirmation

Points Rust utilisés :

io::stdout().flush().expect(...) pour forcer l’affichage du prompt

lire_ligne() pour lire une saisie utilisateur

match sur parse::<u32>() pour valider l’année


2. Afficher tous les livres

Fonction : afficher_tous(biblio: &Vec<Livre>)

Affiche un en-tête

Si la bibliothèque est vide, affiche un message dédié

Sinon, parcourt tous les livres avec for (index, livre) in biblio.iter().enumerate()

Affiche pour chaque livre :

son numéro (index + 1)

le titre

l’auteur

l’année

le statut : Disponible ou Emprunté

Fonction : afficher_disponibles(biblio: &Vec<Livre>)

Vérifie si la bibliothèque est vide

Parcourt tous les livres

N’affiche que ceux pour lesquels livre.disponible == true

Affiche :

numéro

titre

auteur

année

3. Afficher uniquement les livres disponibles

Fonction : afficher_disponibles(biblio: &Vec<Livre>)

Vérifie si la bibliothèque est vide

Parcourt tous les livres

N’affiche que ceux pour lesquels livre.disponible == true

Affiche :

numéro

titre

auteur

année
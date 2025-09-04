# 🤖 Robozzle - Test Technique 

Un jeu de puzzle de programmation visuelle développé avec Rust et Bevy, inspiré du jeu Robozzle original. Guidez un robot à travers différents niveaux en programmant ses mouvements pour collecter toutes les étoiles !
Ce projet a été développé afin d'évaluer des candidats dans un test technique qui allie réflexion, algorithme et logique.


## 📋 Prérequis

- **Rust** (version 1.70 ou supérieure)
- **Cargo** (inclus avec Rust)
- Système d'exploitation : Windows, macOS ou Linux


## 🚀 Installation

### 1. Installer Rust

Si Rust n'est pas installé sur votre système :

```bash
1. Installer rust (Windows/macOS/Linux)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

2. Cloner le projet
git clone [https://github.com/Onsraa/robozzle]

cd robozzle

3. Compiler et lancer
cargo run --release
```

## 🎮 Instructions

→ : Avancer d'une case
↶ : Tourner à gauche (90°)
↷ : Tourner à droite (90°)
F1, F2... : Appeler une fonction

Conditions de couleur
Les instructions peuvent être conditionnelles selon la couleur de la case :

🔴 Rouge : L'instruction s'exécute seulement sur une case rouge
🟢 Vert : L'instruction s'exécute seulement sur une case verte
🔵 Bleu : L'instruction s'exécute seulement sur une case bleue


## Boutons

▶ Start : Lancer l'exécution du programme
⏸ Pause : Mettre en pause l'exécution
⏭ Step : Exécuter une instruction à la fois
Reset : Réinitialiser le robot à sa position de départ
Clear : Effacer toutes les instructions
⚡ Vitesse : Changer la vitesse d'exécution (Normal/Fast/V.Fast)
Clic droit : Efface l'instruction et sa couleur survolée


## 📁 Structure des niveaux
Les niveaux sont stockés dans des fichiers .txt dans les dossiers et peuvent être créés à souhait :

src/levels/tutorials/ : Niveaux tutoriel (1.txt, 2.txt, 3.txt...)
src/levels/ : Niveaux principaux (1.txt, 2.txt, 3.txt...)


Format d'un niveau dans le fichier txt :

LEVEL Nom du niveau
SIZE largeur hauteur
ROBOT x y direction
FUNCTIONS limite_f1 limite_f2 ...

GRID:
.   *   G   B   R
X   G*  B*  R*  .


Légende :

. : Case grise (neutre)
G : Case verte
B : Case bleue
R : Case rouge
X : Case vide (trou)
* : Étoile à collecter (ex: G* = case verte avec étoile)

Direction du robot :

NORTH ou N : Nord (haut)
EAST ou E : Est (droite)
SOUTH ou S : Sud (bas)
WEST ou W : Ouest (gauche)

Exemple de niveau simple
LEVEL Premier pas
SIZE 4 1
ROBOT 0 0 EAST
FUNCTIONS 3

GRID:
.   .   .   *


## Infos complémentaires

Les résultats sont sauvegardés automatiquement dans results_NOM_Prenom.txt
Le timer ne s'applique qu'aux niveaux principaux, pas aux tutoriels
Vous devez compléter chaque niveau tutoriel pour passer au suivant


Bon courage et amusez-vous bien avec les puzzles ! 🚀

## Todo list
- Améliorer l'ui - Couleur de sélection des fonctions à corriger
- Faire un build compatible avec la sauvegarde des résultats
- Ajouter un système de rollback (Ctrl+Z)
- Ajouter du feedback lors de la résolution ou l'échec des niveaux

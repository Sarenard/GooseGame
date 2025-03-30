use super::player::Player;

#[derive(Debug)]
pub struct Grille {
    dimension: i8,
}

const GRILLE_CLASSIQUE: [u64; 64] = [
     0,  1,  2,  3,  4,  5,  6,  7,
    27, 28, 29, 30, 31, 32, 33,  8,
    26, 47, 48, 49, 50, 51, 34,  9,
    25, 46, 59, 60, 61, 52, 35, 10,
    24, 45, 58, 63, 62, 53, 36, 11,
    23, 44, 57, 56, 55, 54, 37, 12,
    22, 43, 42, 41, 40, 39, 38, 13,
    21, 20, 19, 18, 17, 16, 15, 14,
];

impl Grille {
    pub fn new() -> Grille {
        Grille {
            dimension: 0,
        }
    }

    pub fn avancer(&self, player: &mut Player, nb: u8) -> bool {
        let new_position = player.get_case() + nb;

        if new_position == 63 { // on a gagné !
            player.set_case(63);
            return true; // we won !
        } else if new_position > 63 {
            let excess = new_position - 63; // Calculer combien le joueur a dépassé
            player.set_case(63 - excess); // Le joueur revient en arrière
        } else {
            player.set_case(new_position);; // Déplace le joueur normalement
        }
        return false; // we didnt win
    }

    pub fn affiche_grille(&self, player: &Player) {
        for i in 0..8 {
            for _ in 0..8 {
                print!("*-**-");
            }
            println!("*");

            // Affichage des cases de la grille
            for j in 0..8 {
                let index = i * 8 + j;
                if GRILLE_CLASSIQUE[index] == player.get_case().into() {
                    print!("| XX "); // Position du joueur
                } else {
                    let val = GRILLE_CLASSIQUE[index];
                    if val < 10{
                        print!("| 0{} ", val); // Cases avec 1 chiffre
                    } else {
                        print!("| {} ", val); // Cases avec 2 chiffres ou plus
                    }
                }
            }
            println!("|");
        }

        for _ in 0..8 {
            print!("*-**-");
        }
        println!("*");
    }
}
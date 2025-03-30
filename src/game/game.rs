use crate::game::player::Player as Player;
use crate::game::grille::Grille as Grille;

use std::io::{self, Write};

use rand::{rngs::ThreadRng, Rng};

#[derive(Debug)]
pub struct Game {
    player: Player,
    grille: Grille,
    random: ThreadRng,
    won: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player: Player::new(
                "Unintialised".to_string(), 
                0
            ),
            grille: Grille::new(),
            random: rand::thread_rng(),
            won: false,
        }
    }

    pub fn init(&mut self) {
        let mut name = String::new();
        print!("Quel est ton nom ?\n> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut name).expect("Échec de la lecture de l'entrée");
        name = name.trim().to_string();
        self.player.set_name(name);

        println!("Bienvenue dans ce jeu de l'oie {}.", &self.player.get_name());
        println!("Ton but est de trouver l'une des nombreuses fins possibles.");

        println!();
    }

    pub fn step(&mut self) {
        // show the current game
        self.print_game();
        println!();

        // what should we do ?
        let mut action = String::new();
        print!("Que veux-tu faire ?\n> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut action).expect("Échec de la lecture de l'entrée");
        action = action.trim().to_string();
        match action.as_str() {
            "d" => {
                println!("Lancé de dé !");
                self.dice();
            }
            "e" => {
                println!("Informations !");
            }
            _ => {
                println!("Commande inconnue !")
            }
        }
    }

    pub fn last(&mut self) {
        println!();
        println!("Félicitations {}, tu as gagné !", self.player.get_name());
    }

    pub fn run(&mut self) {
        while !self.won {
            self.step();
        }
    }

    fn dice(&mut self) {
        let nb = self.random.gen_range(1..=6);
        println!("Le nombre {} est sorti !", nb);
        if self.grille.avancer(&mut self.player, nb) {
            self.won = true;
        }
    }

    pub fn print_game(&self) {
        &self.grille.affiche_grille(&self.player);
    }
}

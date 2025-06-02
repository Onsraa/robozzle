use std::collections::HashMap;
use std::fs;
use bevy::prelude::*;
use crate::resources::player::PlayerInfo;
use crate::structs::level::{LevelData, ProblemState};

#[derive(Resource)]
pub struct LevelManager {
    current_level_id: usize,
    levels: Vec<LevelData>,                          // Tous les niveaux chargés
    level_states: HashMap<usize, ProblemState>,      // État de chaque problème
}

impl LevelManager {
    pub fn new() -> Self {
        Self {
            current_level_id: 0,
            levels: Vec::new(),
            level_states: HashMap::new(),
        }
    }

    pub fn load_levels_from_directory(path: &str) -> Result<Self, String> {
        let mut manager = Self::new();

        // Lit tous les fichiers .txt du répertoire et les trie par numéro
        let paths = fs::read_dir(path)
            .map_err(|e| format!("Erreur lecture répertoire: {}", e))?;

        let mut level_files = Vec::new();

        for path in paths {
            let path = path.map_err(|e| format!("Erreur path: {}", e))?;
            let file_path = path.path();

            if file_path.extension().and_then(|s| s.to_str()) == Some("txt") {
                if let Some(file_name) = file_path.file_stem().and_then(|s| s.to_str()) {
                    // Essaie de parser le nom de fichier comme un nombre
                    if let Ok(level_num) = file_name.parse::<usize>() {
                        level_files.push((level_num, file_path.to_string_lossy().to_string()));
                    }
                }
            }
        }

        // Trie par numéro de niveau
        level_files.sort_by_key(|(num, _)| *num);

        // Charge chaque niveau dans l'ordre
        for (index, (level_num, file_path)) in level_files.iter().enumerate() {
            match LevelData::from_file(file_path, index) {
                Ok(level_data) => {
                    let problem_state = ProblemState::new(level_data.function_limits.len());

                    manager.levels.push(level_data);
                    manager.level_states.insert(index, problem_state);

                    info!("Niveau {} chargé depuis {}", index + 1, file_path);
                }
                Err(e) => {
                    error!("Erreur chargement niveau {}: {}", file_path, e);
                    return Err(format!("Erreur dans le fichier {}: {}", file_path, e));
                }
            }
        }

        info!("Chargement terminé: {} niveaux", manager.levels.len());
        Ok(manager)
    }

    pub fn get_current_level(&self) -> Option<&LevelData> {
        self.levels.get(self.current_level_id)
    }

    pub fn switch_to_level(&mut self, level_id: usize) {
        if level_id < self.levels.len() {
            self.current_level_id = level_id;
        }
    }

    pub fn get_problem_state(&self, level_id: usize) -> Option<&ProblemState> {
        self.level_states.get(&level_id)
    }

    pub fn get_problem_state_mut(&mut self, level_id: usize) -> Option<&mut ProblemState> {
        self.level_states.get_mut(&level_id)
    }

    // Nouvelle méthode pour accéder à la liste des niveaux
    pub fn get_levels(&self) -> &Vec<LevelData> {
        &self.levels
    }

    // Méthode pour obtenir le nombre de niveaux
    pub fn get_levels_count(&self) -> usize {
        self.levels.len()
    }

    // Génère le rapport final de scoring
    pub fn generate_final_report(&self, player: &PlayerInfo) -> String {
        let mut report = format!("{} {}\n", player.last_name, player.first_name);

        for (i, level) in self.levels.iter().enumerate() {
            if let Some(state) = self.level_states.get(&i) {
                let status = if state.is_completed { "Réussi" } else { "Echoué" };
                report.push_str(&format!(
                    "Problème {} : {} ({}/{})\n",
                    i + 1,
                    status,
                    state.stars_collected,
                    level.total_stars
                ));
            }
        }

        report
    }

    // Sauvegarde le rapport dans un fichier
    pub fn save_final_report(&self, player: &PlayerInfo) -> Result<(), String> {
        let filename = format!("results_{}_{}.txt",
                               player.last_name.replace(" ", "_"),
                               player.first_name.replace(" ", "_"));

        let report = self.generate_final_report(player);

        fs::write(&filename, report)
            .map_err(|e| format!("Erreur sauvegarde: {}", e))?;

        Ok(())
    }
}
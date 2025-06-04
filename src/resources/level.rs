use std::collections::HashMap;
use std::fs;
use bevy::prelude::*;
use crate::resources::player::PlayerInfo;
use crate::structs::level::{LevelData, ProblemState};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LevelType {
    Tutorial,
    Normal,
}

#[derive(Resource)]
pub struct LevelManager {
    current_level_id: usize,
    current_level_type: LevelType,
    tutorial_levels: Vec<LevelData>,                    // Niveaux tutoriel
    normal_levels: Vec<LevelData>,                      // Niveaux normaux
    tutorial_states: HashMap<usize, ProblemState>,      // État des tutoriels
    normal_states: HashMap<usize, ProblemState>,        // État des niveaux normaux
}

impl LevelManager {
    pub fn new() -> Self {
        Self {
            current_level_id: 0,
            current_level_type: LevelType::Tutorial,
            tutorial_levels: Vec::new(),
            normal_levels: Vec::new(),
            tutorial_states: HashMap::new(),
            normal_states: HashMap::new(),
        }
    }

    pub fn load_tutorial_levels_from_directory(path: &str) -> Result<Vec<LevelData>, String> {
        Self::load_levels_from_path(path, 0)
    }

    pub fn load_normal_levels_from_directory(path: &str) -> Result<Vec<LevelData>, String> {
        Self::load_levels_from_path(path, 0)
    }

    fn load_levels_from_path(path: &str, start_id: usize) -> Result<Vec<LevelData>, String> {
        let mut levels = Vec::new();

        // Lit tous les fichiers .txt du répertoire et les trie par numéro
        let paths = fs::read_dir(path)
            .map_err(|e| format!("Erreur lecture répertoire {}: {}", path, e))?;

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
        for (index, (_, file_path)) in level_files.iter().enumerate() {
            match LevelData::from_file(file_path, start_id + index) {
                Ok(level_data) => {
                    levels.push(level_data);
                    info!("Niveau {} chargé depuis {}", index + 1, file_path);
                }
                Err(e) => {
                    error!("Erreur chargement niveau {}: {}", file_path, e);
                    return Err(format!("Erreur dans le fichier {}: {}", file_path, e));
                }
            }
        }

        info!("Chargement terminé: {} niveaux", levels.len());
        Ok(levels)
    }

    pub fn set_tutorial_levels(&mut self, levels: Vec<LevelData>) {
        for level in &levels {
            let problem_state = ProblemState::new(level.function_limits.len());
            self.tutorial_states.insert(level.id, problem_state);
        }
        self.tutorial_levels = levels;
    }

    pub fn set_normal_levels(&mut self, levels: Vec<LevelData>) {
        for level in &levels {
            let problem_state = ProblemState::new(level.function_limits.len());
            self.normal_states.insert(level.id, problem_state);
        }
        self.normal_levels = levels;
    }

    pub fn get_current_level(&self) -> Option<&LevelData> {
        match self.current_level_type {
            LevelType::Tutorial => self.tutorial_levels.get(self.current_level_id),
            LevelType::Normal => self.normal_levels.get(self.current_level_id),
        }
    }

    pub fn switch_to_level(&mut self, level_id: usize) {
        self.current_level_id = level_id;
    }

    pub fn switch_level_type(&mut self, level_type: LevelType) {
        self.current_level_type = level_type;
        self.current_level_id = 0; // Reset à 0 quand on change de type
    }

    pub fn get_problem_state(&self, level_id: usize) -> Option<&ProblemState> {
        match self.current_level_type {
            LevelType::Tutorial => self.tutorial_states.get(&level_id),
            LevelType::Normal => self.normal_states.get(&level_id),
        }
    }

    pub fn get_problem_state_mut(&mut self, level_id: usize) -> Option<&mut ProblemState> {
        match self.current_level_type {
            LevelType::Tutorial => self.tutorial_states.get_mut(&level_id),
            LevelType::Normal => self.normal_states.get_mut(&level_id),
        }
    }

    // Méthode pour accéder à la liste des niveaux
    pub fn get_levels(&self) -> &Vec<LevelData> {
        match self.current_level_type {
            LevelType::Tutorial => &self.tutorial_levels,
            LevelType::Normal => &self.normal_levels,
        }
    }

    // Méthode pour obtenir le nombre de niveaux
    pub fn get_levels_count(&self) -> usize {
        match self.current_level_type {
            LevelType::Tutorial => self.tutorial_levels.len(),
            LevelType::Normal => self.normal_levels.len(),
        }
    }

    pub fn get_current_level_type(&self) -> LevelType {
        self.current_level_type
    }

    // Vérifie si on peut passer au niveau suivant (pour le tutoriel)
    pub fn can_proceed_to_next(&self) -> bool {
        if let Some(current_level) = self.get_current_level() {
            if let Some(state) = self.get_problem_state(current_level.id) {
                return state.is_completed;
            }
        }
        false
    }

    // Passe au niveau suivant si possible
    pub fn try_next_level(&mut self) -> Option<usize> {
        if self.current_level_type == LevelType::Tutorial && !self.can_proceed_to_next() {
            return None; // Bloqué tant que pas complété
        }

        let next_id = self.current_level_id + 1;
        if next_id < self.get_levels_count() {
            self.current_level_id = next_id;
            Some(next_id)
        } else {
            None
        }
    }

    // Vérifie si tous les tutoriels sont complétés
    pub fn are_all_tutorials_completed(&self) -> bool {
        if self.current_level_type != LevelType::Tutorial {
            return true;
        }

        self.tutorial_levels.iter().all(|level| {
            self.tutorial_states
                .get(&level.id)
                .map(|state| state.is_completed)
                .unwrap_or(false)
        })
    }

    // Vérifie si tous les niveaux normaux sont complétés
    pub fn are_all_levels_completed(&self) -> bool {
        self.normal_levels.iter().all(|level| {
            self.normal_states
                .get(&level.id)
                .map(|state| state.is_completed)
                .unwrap_or(false)
        })
    }

    // Génère le rapport final de scoring
    pub fn generate_final_report(&self, player: &PlayerInfo) -> String {
        let mut report = format!("{} {}\n\n", player.last_name, player.first_name);

        // Rapport seulement pour les niveaux normaux
        for (i, level) in self.normal_levels.iter().enumerate() {
            if let Some(state) = self.normal_states.get(&level.id) {
                let status = if state.is_completed { "Réussi" } else { "Echoué" };
                let time_str = if let Some(time) = state.completion_time {
                    format!(" - Temps: {:.1}s", time)
                } else {
                    String::new()
                };

                report.push_str(&format!(
                    "Problème {} : {} ({}/{}) - {}{}\n",
                    i + 1,
                    level.name,
                    state.stars_collected,
                    level.total_stars,
                    status,
                    time_str
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
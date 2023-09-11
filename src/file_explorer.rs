use anyhow::{Error, Result};
use egui::{Button, Color32, ScrollArea, Ui};
use std::fs::{self, DirEntry};
use std::path::PathBuf;

pub struct FileExplorer {
    pub current_dir: PathBuf,
    pub selected_file: PathBuf,
    dir_vec: Vec<PathBuf>,
    file_vec: Vec<PathBuf>,
    dirnames: Vec<String>,
    filenames: Vec<String>,
    pub err: Result<()>,
}

impl FileExplorer {
    pub fn new(_: &eframe::CreationContext) -> Self {
        let mut fe = Self {
            current_dir: std::env::current_dir().unwrap(),
            selected_file: std::path::PathBuf::new(),
            dir_vec: Vec::new(),
            file_vec: Vec::new(),
            dirnames: Vec::new(),
            filenames: Vec::new(),
            err: Ok(()),
        };
        fe.err = fe.update_paths();
        fe
    }

    pub fn update_paths(&mut self) -> Result<()> {
        let dir_content = std::fs::read_dir(&self.current_dir)?
            .filter_map(|dir| dir.ok())
            .collect::<Vec<DirEntry>>()
            .iter()
            .map(|dir| dir.path())
            .collect::<Vec<PathBuf>>();

        self.dir_vec.clear();
        self.file_vec.clear();
        self.filenames.clear();
        self.dirnames.clear();

        for path in dir_content.into_iter() {
            match path {
                _ if path.is_dir() => {
                    self.dir_vec.push(path.clone());
                    if let Some(dirname) = path.file_name() {
                        let str_dirname = dirname.to_string_lossy().to_string();
                        self.dirnames.push(str_dirname);
                    }
                }
                _ if path.is_file() => {
                    self.file_vec.push(path.clone());
                    if let Some(filename) = path.file_name() {
                        let str_filename = filename.to_string_lossy().to_string();
                        self.filenames.push(str_filename);
                    }
                }
                _ => (),
            }
        }
        Ok(())
    }

    pub fn get_filename(&self) -> String {
        self.selected_file
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    }

    pub fn add_dir_ui<G>(ui: &mut Ui, label: &str, r_size: f32, f: G)
    where
        G: FnOnce(),
    {
        // let size = []
        if ui
            // .add_sized(ui.available_size(), Button::new(label))
            // .clicked()
            .button(label)
            .clicked()
        {
            f()
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        // Affichez le chemin actuel en tant qu'en-tête.
        ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Current Path:");
                ui.monospace(self.current_dir.display().to_string());
            });
            ui.horizontal(|ui| {
                if ui.button("<<<").clicked() {
                    self.current_dir.pop();
                    self.err = self.update_paths();
                };
                if ui.button("Update").clicked() {
                    self.err = self.update_paths();
                };
            });

            let mut should_update = false;
            for (dirname, dir_path) in self.dirnames.iter().zip(self.dir_vec.iter()) {
                if ui.button(dirname).clicked() {
                    self.current_dir = dir_path.clone();
                    should_update = true;
                };
            }
            if should_update {
                self.err = self.update_paths();
            }

            for (filename, file_path) in self.filenames.iter().zip(self.file_vec.iter()) {
                if ui.selectable_label(false, filename).clicked() {
                    self.selected_file = file_path.clone();
                };
            }
        });
    }
}
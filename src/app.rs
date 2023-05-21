use std::{io::Read, path::PathBuf};

use egui::*;

use crate::{file_selector::FileSelector, maze::Maze, maze_view::MazeView};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
   maze: Option<Maze>,
   maze_path: Option<PathBuf>,
}

impl Default for App {
   fn default() -> Self {
      Self {
         maze: None,
         maze_path: None,
      }
   }
}

impl App {
   pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
      if let Some(storage) = cc.storage {
         return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
      }
      Default::default()
   }
}

impl eframe::App for App {
   fn save(&mut self, storage: &mut dyn eframe::Storage) {
      eframe::set_value(storage, eframe::APP_KEY, self);
   }

   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
      egui::CentralPanel::default().show(ctx, |ui| {
         let size = ui.available_height().min(ui.available_width());
         ui.horizontal_top(|ui| {
            let mut maze_view = MazeView::new(size);
            if let Some(maze) = &self.maze {
               maze_view.set_maze(maze);
            }
            ui.add(maze_view);

            Grid::new("file_selectors")
               .num_columns(2)
               .spacing([0., ui.spacing().item_spacing.y])
               .show(ui, |ui| {
                  ui.label("Maze");
                  if ui.add(FileSelector::new(&mut self.maze_path)).changed() {
                     if let Some(path) = &self.maze_path {
                        let mut buf = String::new();
                        std::fs::File::open(path)
                           .unwrap()
                           .read_to_string(&mut buf)
                           .unwrap();
                        self.maze = Some(Maze::from_str(&buf));
                     }
                  }
                  ui.end_row();
               });
         });
      });
   }
}

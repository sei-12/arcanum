use std::{
    collections::HashMap,
    hash::Hash,
    sync::{Arc, RwLock},
};

use eframe::egui::{self, Color32, ColorImage};
use game_core::{chars::StaticCharId, enemy_ai::StaticEnemyId};
use image::GenericImageView;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum ImgId {
    CharIcon(StaticCharId),
    EnemyIcon(StaticEnemyId),
}

impl ImgId {
    fn to_path(self) -> String {
        match self {
            Self::CharIcon(id) => format!("./assets/char/icons/{}.jpg", id),
            Self::EnemyIcon(id) => format!("./assets/enemy/icons/{}.jpg", id),
        }
    }
}

#[derive(Debug)]
pub struct ImageLoader {
    cache: RwLock<HashMap<ImgId, Arc<egui::ColorImage>>>,
}

impl ImageLoader {
    pub fn new() -> Self {
        ImageLoader {
            cache: RwLock::new(HashMap::new()),
        }
    }
    pub fn get_char_icon(&self, id: StaticCharId) -> Arc<egui::ColorImage> {
        self.get_or_load(ImgId::CharIcon(id))
    }

    pub fn get_enemy_icon(&self, id: StaticEnemyId) -> Arc<egui::ColorImage> {
        self.get_or_load(ImgId::EnemyIcon(id))
    }

    fn get_or_load(&self, img_id: ImgId) -> Arc<egui::ColorImage> {
        let cache = self.cache.read().expect("failed to read image cache");

        if let Some(img) = cache.get(&img_id) {
            return img.clone();
        };

        let img_data = Arc::new(load_image(&img_id.to_path()));

        drop(cache);

        let mut cache = self.cache.write().expect("failed to write image cache");

        cache.insert(img_id, img_data.clone());

        img_data
    }
}

fn load_image(path: &str) -> ColorImage {
    use image::ImageReader;

    let img = ImageReader::open(path)
        .unwrap_or_else(|err| panic!("failed to read file: path={} : {:?}", path, err))
        .decode()
        .expect("failed to decode image");

    let pixels = img
        .pixels()
        .map(|(_, _, p)| Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
        .collect();

    egui::ColorImage {
        size: [img.width() as usize, img.height() as usize],
        pixels,
    }
}

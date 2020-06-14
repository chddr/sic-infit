use std::collections::HashMap;
use amethyst::{
    assets::{AssetStorage, Loader, ProgressCounter},
    ecs::prelude::World,
    prelude::WorldExt,
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteSheetFormat, SpriteSheetHandle},
        SpriteSheet, Texture,
    },
};


#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum AssetType {
    BackgroundForest,
    Character,
    Platforms,
    Collectables
}

#[derive(Default)]
pub struct SpriteSheetList {
    sprite_sheets: HashMap<AssetType, SpriteSheetHandle>,
}


impl SpriteSheetList {
    pub fn insert(&mut self, asset_type: AssetType, sprite_sheet_handle: SpriteSheetHandle) {
        self.sprite_sheets.insert(asset_type, sprite_sheet_handle);
    }

    pub fn get(&self, asset_type: AssetType) -> Option<&SpriteSheetHandle> {
        self.sprite_sheets.get(&asset_type)
    }
}


pub fn load_assets(world: &mut World, asset_type_list: Vec<AssetType>) -> ProgressCounter {
    let mut sprite_sheet_list = SpriteSheetList::default();
    let mut progress_counter = ProgressCounter::new();

    for &asset_type in asset_type_list.iter() {
        let (texture_path, ron_path) = match asset_type {
            AssetType::BackgroundForest => ("textures/background_forest.png", "prefabs/background_forest.ron"),
            AssetType::Character => ("textures/character.png", "prefabs/character.ron"),
            AssetType::Platforms => ("textures/obstacles/platforms.png", "prefabs/obstacles/platforms.ron"),
            AssetType::Collectables => ("textures/collectables.png", "prefabs/collectables.ron"),
        };
        let sprite_sheet_handle = get_sprite_sheet_handle(world, texture_path, ron_path, &mut progress_counter);
        sprite_sheet_list.insert(asset_type, sprite_sheet_handle);
    }
    world.insert(sprite_sheet_list);
    progress_counter
}

pub fn get_sprite_sheet_handle(world: &World, texture_path: &str, ron_path: &str, progress_counter: &mut ProgressCounter, ) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = &world.read_resource::<Loader>();
        let texture_storage = &world.read_resource::<AssetStorage<Texture>>();
        loader.load(texture_path, ImageFormat::default(), (), &texture_storage)
    };
    let loader = &world.read_resource::<Loader>();
    let sprite_sheet_store = &world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat(texture_handle),
        progress_counter,
        &sprite_sheet_store,
    )
}


use dominari::dominari::GameConfig;
use crate::*;

#[derive(Deserialize, Debug, Clone)]
pub struct Game {
    pub transformation: Transformation,
    pub config: GameConfig,
    pub map: MapConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Transformation {
    pub starting_cards: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MapConfig {
    pub cost_per_tile: u64,
    pub mapmeta: ComponentMapMeta,
    pub features: Vec<Feature>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Feature {
    pub x: u8,
    pub y: u8,
    pub feature: String,
}

pub struct Client {
    pub id01: Keypair,
    pub rpc: WasmClient,
    pub universe: Universe,
    pub world: World,
    pub dominari: Dominari
}
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct Component{
    
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ComponentMetadata{
    pub name: String,
    pub entity_type: String,
    pub world_instance: Pubkey
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ComponentMapMeta{
    pub max_x: u8,
    pub max_y: u8,
    pub play_phase: bool, // False = Build Phase, True = Play Phase
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ComponentLocation {
    pub x: u8,
    pub y: u8
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ComponentFeature{
    pub feature_id: Pubkey // Entity ID
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ComponentOwner{
    pub owner: Pubkey,    // Keypair for Tile Owner
    pub player: Pubkey    // Entity ID for Tile Owner's Player
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ComponentValue{
    pub value: u64, // Could be currency if it's a feature, could be score you'll get for killing the unit, etc
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ComponentOccupant{
    pub occupant_id: Pubkey
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ComponentPlayerStats{
    pub score: u64,
    pub kills: u64,
    pub cards: Vec<Pubkey>, // Blueprints for Unit/Mod entities. Dynamically Realloc space when adding/removing a card
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ComponentLastUsedSlot{
    pub last_used: u64, // Slot last used in
    pub recovery: u64 // How many slots til it can be used again
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ComponentRank{
    pub rank: u8,
    pub max_rank: u8,                  
    pub cost_for_use_ladder: Vec<u64>, // how much it costs at every rank to use the feature
    pub link_rank_ladder: Vec<String>, //"small_healer.png", "medium_healer.png", etc
    pub name_rank_ladder: Vec<String>, //"small_healer", "medium_healer", etc 
    pub per_rank_stat_increase: u64    // Can be interpretted for one stat or many
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ComponentRange{
    pub range: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ComponentDropTable{
    pub drop_table: Vec<Pubkey> // Links to a Blueprint(Card) Pubkey that's dropped
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ComponentUses{
    pub uses_left: u64,
    pub max_uses: u64
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ComponentHealingPower{
    pub heals: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ComponentHealth{
    pub health: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ComponentDamage{
    pub min_damage: u64,
    pub max_damage: u64,
    pub modifier_infantry: i32,
    pub modifier_armor: i32,
    pub modifer_aircraft: i32
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ComponentTroopClass{
    pub class: TroopClass,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub enum TroopClass {
    Infantry,
    Armor,
    Aircraft
}
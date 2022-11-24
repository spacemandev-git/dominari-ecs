# Dominari Design Doc

## ECS 
    - The Universe Program. Keeps tracks of Worlds
    - Keeps track of Entity PDAs as well
        - The same Token could have multiple Entities (One per World Instance), which hold different components

## DominariWorld
    - Keeps track of First Party ComponentBlueprints for DominariWorld  
        - Third party should be deployed as a separate program
    - Keeps track of Systems for Dominari World
    - Keeps track of what Systems are allowed to edit what Components
    - Keeps track of Governance for who's allowed to add Systems and Components to the World
    - Keeps track of Instances of the World and their Update Authority

## DominariSystems: A set of first party systems for game "Dominari"
    - A set of systems to define maps, players, etc

### Key Assumptions
    - Components and Systems are UNIQUE to a world. Even if they have the same code across games, this requires a deployment of a different World Package
    - You can easily create new *instances* of a World with the same code base however


## Entities (Rows) & Components (Columns)

|             | Metadata   | MapMeta    | Location | Feature | Owner | Occupant | P. Stats | Active | Cost |
|:------------| :--------- | :--------- | :------- | :------ | :---- | :------- | :------- | :----- | :--- |
| Map         |     X      |     x      |          |         |       |          |          |        |      |
| Tile        |     x      |            |    x     |    x    |   x   |    x     |          |        |  x   |
| Feature     |     x      |            |    x     |         |   x   |          |          |   x    |  x   |
| Unit        |     x      |            |          |         |   x   |          |          |   x    |      |
| Player      |     x      |            |          |         |       |          |    x     |        |      |


// Features can be mix'd and matched between these three as well, for example, a Damage feature might be found on a Feature that's a static turret
Last Used (because it has entity specific 'recovery' can be used on Features/Units)
Feature Components: Rank, Range, Drop Table, Uses, Healer
Unit Components: Damage, Health, Troop Class,  
Card Components: Card Stats (Blueprint)

## Scripts
    -> Deploy & Register
        - Deploy Universe, World, Systems
        -> Initalize World with Universe
        -> Register Components to Dominari World
        -> Instance a World
        -> Register DominariSystems for Each of the Registered Components

    -> Setup Map
        -> Instance a map of a given grid size
            -> Create Empty Map Entity
            -> Initalize Map Entity & Add Compnoents
        -> Initalize X*Y Tiles
            -> Create Empty Tile Entity
            -> Initialize Tile(x,y) Entity & Add Components

    -> Setup Features, Units, Mods
        -> Register Blueprints as Accounts on DominariSystems for each Feature, Unit, Mod
        -> Register Blueprint for Starting Card

    -> Register Player
        -> Create Player Entity
        -> Init Player by giving them a starting Unit Blueprint as a card

    -> Build Phase Sim 01
        -> Two players buy and build various features on locations

    -> Phase Phase Sim 02
        -> Two players spawn units and use features while attempting to kill other player off

## Systems
    -> RegisterBlueprint()
        -> Blueprint is a Collection of Preset Components
    -> RegisterPlayer()
    -> InitMap()
    -> InitTile()
    -> BuyTile()
    -> BuildFeature()
    -> SwitchPhases()
    -> MoveUnit()
    -> UseFeature() *** (Will be quite a few systems)
    -> PlayCard()
    -> AttackUnit()

## SDK: JS or Rust?
    -> How to get all entities owned by an address?
    -> How to get all Tiles that belong to a Map?
    -> How to get all Features on a Map? 
    -> 

### Key Considerations
    -> Entity Owners for a Game *must* be a PDA so it can modify/fetch them them!!
        -> Game Entities are Maps, Tiles, Blueprints
        -> 

### TODO
-> Account Compression for Entities
-> xNFT for Entity Viewership
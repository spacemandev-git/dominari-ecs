# Programs

## ECS 
    - The Universe Program. Keeps tracks of Worlds
    - Keeps track of Entity PDAs as well
        - The same Token could have multiple Entities (One per World Instance), which hold different components

## DominariWorld
    - Keeps track of ComponentBlueprints for DominariWorld
    - Keeps track of Systems for Dominari World
    - Keeps track of what Systems are allowed to edit what Components
    - Keeps track of Governance for who's allowed to add Systems and Components to the World
    - Keeps track of Instances of the World and their Update Authority

## ObjectiveSystem: Score
    - A set of systems that make up the "Score" based Dominari Game


#### Key Assumptions
    - Components and Systems are UNIQUE to a world. Even if they have the same code across games, this requires a deployment of a different World Package
    - You can easily create new *instances* of a World with the same code base however

# Play Phase
-> Move PlayPhase out of mapmeta and into instance index
-> Check for play phase before spawning/moving/attacking with units or doing any other action
-> In game setup, start the game by moving playphase to "Play" from "Lobby"

# Score & Kills
-> Actually give players score and kills when defeating opponents

# REPL: Attack
-> Allow attacking features
-> Allow playing mods on existing units

# REPL: Printing
-> Tile Info print should have full feature print support
-> Dunno why TileAttacked events aren't being listened to but movement events are. Might just be really bad solana logs thingy?
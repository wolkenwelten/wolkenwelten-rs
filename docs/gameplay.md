# Gameplay notes
You wake up on an island with just your clothes, as you get up you see something moving, it's coming towards you, ready to attack! It seems food and shelter are the least of your worries.

## Weapons
You start with just your fists for your first enemies.

### Tier 1 (Stone age)
These can be foraged/crafted from nature.
- Club = Medium range/speed/damage
- Rock = Short range, high speed/damage
- Stones/Sling = Ranged, very slow, little damage

### Tier 2 (Iron age)
These will mostly be scavenged from monsters
- Sword = Medium range/speed/damage
- Knives = Short range, high speed, medium damage
- Bow = Ranged, very slow, medium damage

### Tier 3 (Golden age)
These can be found in the ancient ruins, or dropped by mini bosses
- Katana
- Katar
- Crossbow

## Armor
Armor slows a player down, especially shield slow down a player considerably, but also make defending a lot easier. Basically a player has to choose whether they want to evade or defend.
- Shields (Wood, Iron, Titanium) = Slows down, but increases defense, especially if held high

## Items
- Grenade/Bomb = Thrown on use, long fuse, big explosion afterwards.
- Food / Beef Jerky = Heals slowly, when attacked buff disappears
- Power potion = Boost all stats for a short while, very rare

## Enemies

### Shores / Jungle
- Slimes = Splits in two when killed, until minimum size reached ( boring enemy, but seems to be the simplest enemy one could implement)
- Coconut crabs
- Kobold = Slings / Clubs

### Goblin Village
- Goblin Hunter = Bow
- Goblin Guard = Sword
- Goblin Villager = Knife

### Ancient Ruins
- Wolf
- Goblin warrior = Sword + Shield
- Goblin apprentice = Slow Fireballs

### Boss
- Goblin witcher = Calls/Resurrects Goblins / Fireballs / Teleports

## Leveling / Experience / Skills
Killing enemies gains XP, giving a level up after grinding for long enough. No attributes, instead a skill tree.
Each skill depends on the one before it. Max level is 6 (so 5 skills). No Mana, instead each skill has a cooldown.

### Rookie
The rookie has 3 branches, melee, ranged, magic, with 3 skills each.

#### Rookie melee
- Bash = Charge up, massive strike that knocks back an enemy ~10-30 Blocks
- Smash = Smash the ground, which cracks the earth, damaging enemies
- Dash = Quickly dash into a specific direction while swinging your weapon

#### Rookie ranged
- Power shot = Charge up, projectile pierces through enemies, and later blocks
- Multi shot = Fire ~3-9 projectiles at once in an arc
- Climb = Allows climbing up walls/trees for a little bit

#### Rookie magic
- Telekinesis = Move everything in front of the wizard into a specific direction (WASD)
- Blindness = Enemies afflicted just swing their weapons around, maybe even hitting each other
- Blink = Throw a (somewhat slow) projectile and exchange positions with what it hit (mainly blocks or enemies)

## Permadeath
Gonna orient myself after ToME4 here, where you have a couple of tries per run, which seems like the best compromise I've seen so far. This way most games will end, either in death or victory.

## Character Gen
For the first run the shouldn't be anything to choose from, only the base combination. More can then be unlocked with each run.
While this complicates things, I feel that it is important so that permadeath doesn't feel as punishing. Since otherwise a failed run would be just a failure, with unlocks however each unlock brings one closer to winning the game.

## Singleplayer only
Multiplayer complicates things A LOT, focus on making a short, fun, singleplayer experience.

## Winning
Killing the boss means ending/winning the game. There should be a sort of score, like time elapsed, damage dealt, damage taken etc. This should be simple to implement since we need to keep track if that anyways for achievements.

## Healing
To heal you have to eat and rest for a bit.
# Stragglers
---
## Team Members:
### Enemy AI:
* Ricky Conrad
* Kiernan Devane
* Miko Miller
* Sally Zhang
### Procedural Generation:
* Helen Fleming
* Rachael Jan
* Leo Liang
* Andre Ukattah
---
## Game Description:
* L's Labyrinth is a exploration and combat based game where a player navigates through a maze to encounter enemies and level up to eventually escape. Players will have a variety of combat options in an encounter (attack, magic, heal, escape) and are able to boost different abilities which impact their stats--Health, Strength, Agility, Magic--by progressing through the skill tree.
![image](/images/skill-tree-V2.jpg)
---
## Procedural Generation
* The player will be able to navigate through a procedurally generated maze structure with starting and ending rooms present in each generation. The procedural generation of the maze will be implemented using Wilson’s algorithm. Start and end rooms will be created with pathways procedurally generated between.
---
## Enemy AI
* The npc enemies will be scaled with different class strengths to resemble a advanced player. Combat will be turn based with advanced enemies able to choose attacks based on the player's current ability scores and the enemy's own ability scores. Starter enemies will choose attacks based on random probability. Advanced enemies will follow resource assignment algorithym (state machine implementation) which will track outcome of previous attacks and assigns values to actions based on the battle log.
---
## Gameplay
* The player will progress through the maze, searching for enemies. Defeating enemies awards the player points towards their ability tree. Advancing in the ability tree will allow the player to gain new attacks and improve their stats and ability scores to eventually defeat all enemies and escape the labyrinth.
* The main overworld screen is a top-down view with the player centered in the screen, being able to move left, right, up, down, and diagonally. The secondary screen for combat encounters will include the player sprite, enemy sprite, and attack options for the player. A third screen, available by pressing a designated key, brings up the skill tree and ability score overview which can be navigated by mouse.
* The Player will begin the game as a basic fighter with physical attacks, progressing through the skill tree will unlock magic attacks and speed boosts.
*
* ### Ability Scores/Skill tree
* Before begining the game, player is given 12 points to distribute between the four ability scores: Magic, Health, Strength, Agility
* Magic: affects base magical damage and magical resistance
* Health: affects base hit points
* Strength: affects base physical damage and physical resistance
* Agility: affects number of actions per turn
* Players will gain additional ability points to increase their ability scores when progressing through the skill tree.
* 
* ### Combat
* Turn-based with agility determining order and frequency of actions
* Combat options:
* Attack (physical damage)
* Single attack
* Status effects
* Magic (magical damage)
* Single attack
* Status effects
* Heal (basic heal available at start; more efficient heal available if player has unlocked heal spells)
* Escape (dependent on agility; no skill points gained)
* ### Enemy encounters
* Gain skill points
* Stock enemies with modifiers added based on player progression
---
## Procedural Generation
### Midterm Goals:
* Static map is created in void with a start and end room.
### Final Goals:
* Maze will have a procedurally generated layout with starting and ending rooms present in each generation.
* Player can move through the map with collisions in place (not moving through walls, running into nothing, etc).
* Generate start and end rooms with 10 rooms between them where the pathways between are procedurally generated
* Textures are added to the map and generated rooms vary in size
* Pathways between rooms contain enemies whose locations are procedurally generated
---
## Enemy AI
### Midterm Goals:
* Player is able to play through functional battle UI
* Enemies will randomly select an attack or healing option which will affect the player
* Enemies will be affected by player attacks.
### Final Goals:
* Secondary enemy type is implemented which will select calculated attack and heal options
* Enemies will switch their attack/heal type if the player switches their attack/heal type
---
Final Goals | Weight
--- | ---
Procedural Generation:<BR>  -  8%: Maze pathways and entity locations within them are procedurally generated using Wison's algorithm<BR>  -  7%: Maze contains begin room, end room, and eight other rooms throughout the maze<BR>  -  5%: Each generation of the map is significantly different compared to the last | 20%
Enemy AI:<BR>  -  7%: The player is able to engage in full turn-based combat encounters that play to completion with individual enemies<BR>  -  7%: Enemies adjust combat actions according to player actions & status effects using resource assignment algorithm.<BR>  -  6%: 2 different enemy types are implemented, one with random attacks and another with calculated attacks that follow our ai implementation  | 20%
The player character can move up, down, left, right, and diagonally throughout the map with the camera centered on them and interact with entities/the world where gameplay physics are consistent (player is restricted to walls, can’t move through objects. etc) | 10%
The player is able to customize their character by putting skill points into different abilities that affect the gameplay/combat. At start of game, player recieves 12 ability points to attribute to either Health, Strength, Agility, Magic.  | 10%
All game scenes/menu are present and triggered at appropriate times<BR>  -  2%: Welcome scene where players atribute their starting ability scores<BR>  -  2%: Skill tree UI which includes the four abaility scores<BR>  -  2%: Gameplay map where the player is moving/exploring<BR>  -  2%: Combat scene including player sprite, enemy sprite, and attack choices<BR>  -  2%: End scene when the player escapes the maze/dies in combat | 10%
After enemies are defeated, the player recieves skill points that can be used to unlock nodes in the skill tree UI | 10%
---
## Stretch Goal:
* Implement two additional classes the player can choose from for a total of three: Fighter, Mage, Rogue. Each additional class includes an additional associated skill tree. Stat modifiers, ability score improvements, and abilities earned from different skill trees will stack.
* Addition of third enemy type: Boss, which will have more health and a "smarter" AI than regular enemies during combat, as well as offer more skill points upon defeat.

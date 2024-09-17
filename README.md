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
* L's Labyrinth is a exploration and combat based game where a player navigates through a maze to encounter enemies and level up to eventually escape. Players have a variety of combat options wherein they choose one of three classes to start and are able to boost different abilities- Health, Strength, Agility, Magic- by progressing through each class's skill tree.
![image](/Tree.png)
### Classes
* Rogue - higher speed
* Mage - higher magic
* Fighter - higher strength
### Ability Scores/Stats
* Set base stats at beginning; 12 points to distribute between the following below
* Magic, Health, Strength, Agility
* Agility: affects number of actions per turn
* inspired by polyrhythmic music (sally and ricky)
* Strength: affects base physical damage and physical resistance
* Magic: affects base magical damage and magical resistance
* Health: affects base hit points
* If implementing mp system, health will impact mp as well
* Can gain additional ability score points to distribute to any stat through skill tree & level up (maybe)
### Skill Tree
* 3 trees, one for each class
* Player progresses through skill tree with points acquired from skill point bar (like exp bar) which is filled by combat encounters
* Skill tree branches/upgrades:
#### Rogue
* Agility ability score & minor health increases
* Magic & physical damage modifier
* Magic & physical resistance modifier
* Magic & physical attack unlock
#### Mage
* Magic ability score increases
* Magic damage modifier
* Magic resistance modifier
* Magic attack unlock/healing spells unlock
#### Fighter
* Strength ability score increases
* Physical damage modifier
* Physical resistance modifier
* Physical attack unlock
* Ability score points to apply to any ability score can be unlocked through any tree
### Combat
* Turn-based with agility determining order and frequency of actions
* Combat options:
* Attack (physical damage)
* Single attack
* Status effects
* Magic (magical damage)
* Single attack
* AOE
* Status effects
* Heal (basic heal available at start?; more efficient heal available if player has unlocked heal spells from mage skill tree)
* Escape (dependent on agility; no exp or skill points gained)
### Exp/Level up
* Health increases
* Gain ability score points every x levels (maybe)
### Enemy encounters
* Gain exp & skill points (both are a points/bar fill system)
* Stock enemies with modifiers added based on player progression
---
## Procedural Generation
* The player will be able to navigate through a procedurally generated maze structure with starting and ending rooms present in each generation. The procedural generation of the maze will be implemented using Wilson’s algorithm. Start and end rooms will be created with pathways procedurally generated between.
---
## Enemy AI
* The npc enemies will be scaled with different class strengths to resemble a advanced player. Combat will be turn based with advanced enemies able to choose attacks based on the player's current ability scores and the enemy's own ability scores. Starter enemies will choose attacks based on random probability
---
## Gameplay
* The player will progress through the maze, searching for enemies and power ups. Defeating enemies awards the player points towards their ability tree. Advancing in the ability tree will allow the player to gain new attacks and improve their ability score to eventually defeat boss rooms.
* Game will be top-down single player with the player always centered in the screen
---
## Procedural Generation
### Midterm Goals:
* Maze will have a procedurally generated layout with starting and ending rooms present in each generation.
* Player can move through the map with collisions in place (not moving through walls, running into nothing, etc).
* Generate start and end rooms with 5 rooms between them where the pathways between are procedurally generated
### Final Goals:
* Procedurally generated npc/enemy locations.
* Textures are added to the map.
* 10 rooms that are populated with entities (helpful and harmful) 
* Pathways between rooms contain enemies whose locations are procedurally generated
* Generated rooms for eney encounters vary in size
---
## Enemy AI
### Midterm Goals:
* Can fight one enemy at a time, with attacks and spells.
* Enemies will have a basic ai and choose combat actions based on current conditions & player's skill tree/level progression.
* Two different types of enemies to fight.
### Final Goals:
* Can fight one enemy at a time, full encounter (includes escape combat option, which enemies may respond to).
* Enemies adjust combat actions according to player actions & status effects.
* Three different enemies implemented(including a boss).
---
Final Goals | Weight
--- | ---
Procedurally generated npc/enemy locations. | 20%
Textures are added to the map. | 10%
Full combat encounter implemented. | 20%
Enemies adjust combat actions according to player actions & status effects. | 15%
Three different enemies implemented(including a boss). | 15%
---
## Stretch Goal:
* Combat involving multiple enemies.
* Friendly npcs with additional dialogue options to gain skill tree points.

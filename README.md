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
![image](/images/Tree.png)
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
* ### Classes
* Rogue - higher speed
* Mage - higher magic
* Fighter - higher strength
* ### Ability Scores/Stats
* Set base stats at beginning; 12 points to distribute between the following below
* Magic, Health, Strength, Agility
* Agility: affects number of actions per turn
* inspired by polyrhythmic music (sally and ricky)
* Strength: affects base physical damage and physical resistance
* Magic: affects base magical damage and magical resistance
* Health: affects base hit points
* If implementing mp system, health will impact mp as well
* Can gain additional ability score points to distribute to any stat through skill tree & level up (maybe)
* ### Skill Tree
* 3 trees, one for each class
* Player progresses through skill tree with points acquired from skill point bar (like exp bar) which is filled by combat encounters
* Skill tree branches/upgrades:
* #### Rogue
* Agility ability score & minor health increases
* Magic & physical damage modifier
* Magic & physical resistance modifier
* Magic & physical attack unlock
* #### Mage
* Magic ability score increases
* Magic damage modifier
* Magic resistance modifier
* Magic attack unlock/healing spells unlock
* #### Fighter
* Strength ability score increases
* Physical damage modifier
* Physical resistance modifier
* Physical attack unlock
* Ability score points to apply to any ability score can be unlocked through any tree
* ### Combat
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
* ### Exp/Level up
* Health increases
* Gain ability score points every x levels (maybe)
* ### Enemy encounters
* Gain exp & skill points (both are a points/bar fill system)
* Stock enemies with modifiers added based on player progression
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
Procedural Generation:<BR>  -  5%: Beginning and Ending rooms with 10 rooms populated with entities (helpful and harmful). The pathways to these rooms and entity locations within them are procedurally generated<BR>  -  5%: Textures and sprites are present and consistent throughout the map and within each generation.<BR>  -  5%: Each generation of the map is significantly different compared to the last | 15%
Enemy AI:<BR>  -  5%: The player is able to engage in full turn-based combat encounters that play to completion with individual enemies includes an escape combat option, which enemies may respond to by following the character<BR>  -  5%: Enemies adjust combat actions according to player actions & status effects.<BR>  -  5%: 3 different enemy types are implemented (including a boss whose fighting style reacts more accurately to the player compared to the other types)  | 15%
The player character can move throughout the map with the camera centered on them and interact with entities/the world where gameplay physics are consistent (player is restricted to walls, can’t move through objects. etc) | 15%
The player is able to customize their character by putting skill points into different abilities that affect the gameplay/combat  | 15%
All game scenes/menu are present and triggered at appropriate times<BR>  -  Welcome scene, skill tree, gameplay map where the player is moving/exploring, combat scene, end scene when the player escapes the maze/dies in combat | 10%
Checkpoints are present for if a player dies; they return to the last room cleared with all of the skill progress from that point | 10%
---
## Stretch Goal:
* Combat involving multiple enemies.
* Friendly npcs with additional dialogue options to gain skill tree points.

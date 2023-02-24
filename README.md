# Grandstand Gauntlet

Game's Links:

- [Submission Brackeys 2023.1](https://fabinistere.itch.io/grandstand-gauntlet)
- [Latest Build](https://fabinistere.github.io/grandstand-gauntlet)
<!-- TODO: Changelog -->

GameJam [Brackeys 2023.1](https://itch.io/jam/brackeys-9), for a week (12/02->19/02).
The Theme is

![An end is a new beginning](https://img.itch.zone/aW1nLzExMzAyNDc0LnBuZw==/original/maBIE1.png)

2 person game, created for the game jam Brackeys 2023 (1week limit)

Developed by:

- [Morgan aka Elzapat](https://github.com/Elzapat)
- [Olf aka Wabtey](https://github.com/Wabtey)

## Game Ideas

Fighters in arenas or galleries.
Death makes us take control of a surrounding normie entity,
like a spectator

### Concept

- Fight only in populated areas.
- Build arenas to defeat the enemy.
- Create a cult, the members follow us to act as a receptacle.
- Don't overuse the reappearance or the Boss will start a killing spree in the crowd.
- At the end of the fight, we can take possession of the enemy and resume the symmetrical fight
or leave to face other stronger enemy, in the next arena.
One can decide to cut the cycle by refusing the transfer, and say:
"Everything has an end".

## Current Release

- Parallax map
  - [***Eder Munizz***](https://edermunizz.itch.io/) - Parallax
    - [Free Pixel Art Hill](https://edermunizz.itch.io/free-pixel-art-hill)
      ![Free Pixel Art Hill](https://img.itch.zone/aW1hZ2UvMTc4ODM2LzgzNzU3Mi5wbmc=/original/t0jyLw.png)
- Characters
  - [***Penusbmic***](https://penusbmic.itch.io/) - Scifi Character Pack
    - [Character Pack 10 - Ball and Chain Bot](https://penusbmic.itch.io/sci-fi-character-pack-10)

      ![Ball and Chain Bot](https://img.itch.zone/aW1nLzQ4NzY4NzUucG5n/180x143%23c/fWIjAB.png "pack ten")
  
    - [Character Pack 11 - Toaster Bot](https://penusbmic.itch.io/sci-fi-character-pack-11)

      ![Toaster Bot](https://img.itch.zone/aW1nLzUyNzk4MzkucG5n/180x143%23c/RzE1WI.png "pack eleven")
- Player
  - Can *light slap* with, light press on `Left Click Mouse` or `Return`
  - Can *heavy slap* with, long press on `Left Click Mouse` or `Return`
  - Can *move left or right* with, `A D` or `Q D` or `Left Right`
  - Can *so long* with, `E`
- Boss
  - Can *stare* the player
  - Can *attack with a 'light' smash* the player if too close
  - Can *wonder about all the abilities they have been given without any chance to triggering them*
- After Two hits, the player dies and soul shift towards a nearby spectator
  - The spectator comes at the first plan
  - The Dead Body lays above the scene
  - The new player is fully functional

### Screenshots

![InGame 2](https://img.itch.zone/aW1hZ2UvMTkzMjU4MS8xMTM2OTgyNS5wbmc=/original/aO01KP.png)

## Requirements

### Must Have

- [x] Map
  - [x] side view
- [ ] Enemy Boss
  - [x] Attack the player
  - [x] Big HP
  - [ ] Move around
- [x] Player
  - [x] Attack
  - [x] Movement
- [x] Soul Shift
  - [x] Crowd
  - [x] When lethal dmg to the player -> Soul Shift
  - [x] Switch control to another unit

### Should Have

- [ ] Map
  - [ ] Grandstand somewhere
  - [ ] Confort Zone:
  6/8 in the center of the screen
  - [ ] Only move when leaving the confort zone
  - [ ] The Gap should always be between the player and the boss.
- [ ] Player
  - [x] Smooth Movement
  - [x] Charged Slap
  - [ ] Pary
  - [ ] Dodge / Dash
- [ ] SFX
  - [ ] Fight
    - [ ] Impact / Failed Pary
    - [ ] Successful Pary / Dodge
- [ ] Music
  - [ ] Fight

### Could Have

If Time exists in our dimension:

- [ ] Combat Map
  - [ ] The more HP missing the HP have, the more dark and red, the clouds are.
- [ ] Music
  - [ ] Start of Fight
  - [ ] End of Fight
  - [ ] Boss Entry Scene
  (Big badass, slow move: sick organ solo ?)
  [Fallen Angel - Second Part](https://youtu.be/QjV-f-Ew-Bw?t=2978)
  - [ ] Player Death (First Soul Shift)
  (Suspense, Tragic, Slow)
  [Fallen Angel - First Part](https://www.youtube.com/watch?v=QjV-f-Ew-Bw&t=2939~~s)
  ["He Is the Light in My Darkness" - First Part](https://www.youtube.com/watch?v=QjV-f-Ew-Bw&t=1476s)
  - [ ] Boss Fight - First Phase
  (intense, loop)
  [The Death of God's Will (incl. "Horns of Insurrection")](https://www.youtube.com/watch?v=QjV-f-Ew-Bw&t=3058s)
  - [ ] Boss Fight - Second Phase (angier + tp)
- [ ] SFX
  - [ ] Movement
  - [ ] Ambiance
    - [ ] Environment
    - [ ] Character
  - [ ] Crowd
    - [ ] Acclamation of the crowd
    - [ ] Suspense / Tension

### Won't Have

- [x] Stuff we can't produce

## Combat

### Boss AI

- [ ] Movement
  - [ ] Won't try to dodge
  - [x] just want to smash the player's skull
  - [ ] Move back (and charge) before doing a dash attack (?)
- [ ] can feint/fake
- [ ] Attacks
  - [x] Attack Player when nearby
  - [ ] "Just Die Already"
  One Time Attack - Phase 2 Transition
    - Slow motion Instant/Attac
    Meant to be paried
  - [ ] "Try to pary that!"
  after a certain number (10?) of pary
    - TP behind/infront of the player
    - feint 2 to 5 times before striking
    (Have to create hint or pattern (for ex: the anim being 50%+ done it's a real attack))
  - [ ] "Back Off"
  If too close
    - simple attack meant to knockback
  - [x] Fallen Angel - "BEHOLD" (yells it once - first time doing the attack)
    - Powerfull Fall Attack

### Player Mecs

MustHave

- [x] simple attack (1row)
- [x] charge
- [ ] when charged:
  - [x] powerfull attack
  - [x] 2rows (slap return)
  - [ ] Stun (if charged at least x sec)
  - [ ] dmg calculed by % of time charged
- [x] Hitted
  - [x] Invulnerablity Frame
  - [x] Cancel Current Action ?
  - [ ] Knockback?
- [x] run / Stop
  - [ ] ? Did we have them ? precise movement

Should Have

- [ ] Dash
  - [ ] Rapid Move
  - [ ] Invulnerablity
- [ ] Pary
  - [ ] Stun if pertectly timed
  - [ ] Dmg restitution

## Name Ideas

### Favorites

- ***The Grandstand Gauntlet***
- Soul Shift
- Shadow Saviors
- Boss Bender
- ~~Spectral~~ Standoff

### Others

- Another Chance
- One more chance
- **Power of the crowd**
- Take Control
- MITSUKI
- The end to a new beginning
- Fight to the end
- Ever After
- **Crowd Control**
- **Soul Shift**
- **The Grandstand Gauntlet**
- Keep Living
- Keep Fighting
- Sharp Soul
- Never Ever Stop
- Soul Switcher
- Dual Souls
- Boss Bender
- Beyond the Grave
- Revenant Rampage
- **Spectral Standoff**
- Shadow Saviors
- Afterlife Assault
- Dual Demise
- Spirit Showdown

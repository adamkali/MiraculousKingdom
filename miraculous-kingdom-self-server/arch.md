# main.rs
## actors 
### [actor].rs
- [ ] struct Actor 
- [ ] struct ActorPost 
- [ ] struct Actors
- [ ] GetActor 
    - Just get the data with an ID
    - /api/[actor]/[id]
- [ ] GetActorsFilter
    - Just get the data with an appropriate filter query parameter
    - /api/[actor]?active=true&...
- [ ] GetActors
    - Just get the data with of all actors 
    - /api/[actor]
- [ ] PostActor 
    - Just get the data with and a body 
    - /api/actor 
    - body=[ActorPost]
- [ ] DelteActor
    - Just delete the data with an ID
    - /api/[actor]/[id]
## data
### Classes
### Abilitis
### Events
## game management


## Sample Request
To get a character with id=00000000-0000-0000-000000000000 in a game?id=0 
`/api/game/0/character/00000000-0000-0000-000000000000`

## game
### Sovreign Name 
 - string
 - leader_name
### Country
 - country_name 
### password
 - GUID -> returned to the user to submit turns 
### Characters
 - Vec<Character> corresponds to each user's character.
### State
 - State

## State
### GameState
 - GameState -> Enum what should be happening
#### ClockResolution()
 - check if any of the server held clocks need to be resolved 
 - If there are any that need to be resolved go down the list based on the current turn order.
#### EventRolling(characterID: GUID)
 - based on if the current player with the corresponding turn
 - check if event is finished.
#### EpisodeTurn(characterID: GUID)
#### EventResolution()
### Clocks
### Characters

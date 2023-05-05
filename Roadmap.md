# Roadmap to 1.0.0 

## Backend

### Containerization
- [ ] Need to containerize the api and make sure i can get into it wih the self signed certificate.

### Updated Character
- [ ] Remove class from the Character Struct and pull up the class enum, the abilities into a deck struct, and the class passive.

### Hold current game.
- [ ] Hold the current game in an arc Layer.

### Deck and Hand and apropriate method.
- [ ] add methods and api endpoints for Abilities getting the hand and the deck.
- [ ] have every single ability to have an id and be stored in mongo db.

### More Classes
- [ ] Cardinal
- [ ] Royal Scientist

### Add More events
- [ ] add in enough events for 20 seasons at least.
- [ ] have the event be stored in the arc Layer

### Have better logging with axum.
- [ ] figure out how tower logging works 
- [ ] figure out bettier loggin to get all of the endpoints and sent byte count.

### Cors
- [x] Athorization and Accept

## Frontend 

### Get all the rules i can think of
- [ ] Might
- [ ] seasons
- [ ] Game
- [ ] Deck & Hand

### Character sheet
- [ ] develop the character sheet.
- [ ] have the character page listen for new events,
- [ ] have the side bar hold the current, events
- [ ] have a bottom bar that will hold the current hand.
- [ ] have a listener for when everyone is ready.

### Start game
- [x] Develop Start Game page.
- [x] Develop join Game page.
- [ ] Develop Play Game page.
- [x] Develop Create Character

### Containerize the Frontend
- [ ] make a docker container for the front end.
- [ ] have the certificates self signed.


## Testing
- [ ] test all endpoints.
- [ ] confirm everything works in a docker compose.
- [ ] try getting it on our home network as a home lab.
- [ ] see if we can play from the site!

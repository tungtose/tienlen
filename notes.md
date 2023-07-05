### 1-6-23
- Features
    + [x] Player can draw a card
    + [x] Switch turn after player draw a card
    + [x] Show card on the table


- Bug
    + [x] After turn update, the previous hand disapear
    + [x] After 4 turn, the game crash

### 4-6-23

- Features
    + [x] Skip turn button
    + [x] Show timer

- Bug
    + [] 10 J Q K A 2 -> should not allowed as a sequences
    + [] Cards in the table should be sorted
    + [] only move mouse around to get the update

### 5-6-23

- Features
    + [x] After skip turn, counter should be reset
    + [x] Decide if the turn can be play due to the last turn
    + [] correct the turn:
        + [] if all the player skip turn, the given card player can play another combo
        + [] if player skip turn, they will not be able to play until new turn/combo

- Bug
    + [x] 10 J Q K A 2 -> should not allowed as a sequences
    + [x] Only move mouse around to get the update (Why: de WinitSetting:desktop_app() plugin is the root cause, it's pause all the system if no focus to the app)
    + [x] Card should be in the initial position after the turn update
    + [x] 5 6 7 8 9 10 J Q -> should be playable
    + [x] Cards in the table should be sorted

### 6-6-2023

- Features
    + [] Add multiplayer
        [x] Choose one: websocket client/server, webrtc p2p -> webrtc p2p


### 8-6-2023

- Features
    + [x] Compile to WASM
    + [] Add multiplayer
        [x] Choose one: websocket client/server, webrtc p2p -> webrtc p2p
        [x] Integrate matchbox, ggrs
        [x] Create lobby
        [x] After amount of player join the room, move into the game
        [] Rewrite the determistic system
### 14-6-2023

[x] feat: Compile to WASM
[] feat: Add multiplayer
  [x] Choose one: websocket client/server, webrtc p2p -> webrtc p2p
  [x] Integrate matchbox, ggrs
  [x] Create lobby
  [x] After amount of player join the room, move into the game
  [] Rewrite the determistic system

[] bug: Crash after 2 people join


### 15-6 -> 22-6-2023

- Found that P2P is hard for the initial implement -> switch to server-client approach -> using naia
    [x] naia naia naia learning & init


### 23-6-2023

- [x] Feat: Host able to start the game
- [x] Feat: Server deal card to player


### 24-6-2023

- [x] Feat: Player able to select the card


### 25-6-2023

- [x] Feat: Player able to play the card
    + [x] Despawn the card on the player hand ui
    + [x] Show card on the global table


### 26-6-2023

- [x] Feat: Player able to play the card
    + [x] Despawn the card on the player hand ui
    + [x] Show card on the global table

- [x] Bug: Crash when play no card


### 27-6-2023
- [x] Feat: Turn System
    + [x] Highligh player turn (must rewrite later)
    + [x] Update player turn (must rewrite later)



### 5-7-2023
- [] Feat: Game system

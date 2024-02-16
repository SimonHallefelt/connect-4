# Connect 4
This is a rust project that implements a connect 4 board and a variety of different types of players.

## How to run
1. Install rust.
2. Install the project.
3. Use the command "cargo run --release" in a terminal.

## Players
There are multiple different players and they all should make their move under 5 seconds.

### Random 
Random is a player that always plays a random legal move.

### Human 
Human is a player that you the user controls what move will be played.

### Bot
Bot is a player that uses alpha-beta pruning and a heretic evaluation to decide it's next move.

### Bit Bot
Bit Bot is a player that uses alpha-beta pruning and a heretic evaluation to decide it's next move. Implemented using bit-boards.

### Bit Bot 2
Same as bit bot but implemented with a loop and preforms much worse.
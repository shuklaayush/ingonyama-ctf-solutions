# The Day Of Sagittarius IV

### Description

We are excited to announce the release of the latest version of the retro terminal game "The Day of Sagittarius IV". This new version is designed to be played peer to peer and does not require any servers to play. Instead, the game data is stored on the players' computers, creating a truly decentralized gaming experience.

To ensure that players adhere to the rules of the game, the new version utilizes a zero-knowledge virtual machine. This feature allows players to verify that their opponents are playing fairly without revealing any confidential information.

We are currently in the beta-testing phase, and we invite all gaming enthusiasts to try out the new version of "The Day of Sagittarius IV".

As part of the beta-launch, we are also excited to announce a special promotional event. Players who win a bot in the game on `52.7.211.188:6000` will be eligible for a prize.

We believe that the new peer-to-peer version of "The Day of Sagittarius IV" will revolutionize the world of retro gaming, and we are thrilled to share it with you. Join us in this exciting new adventure, and let's explore the universe together!

### How to play
1. Build the game from source
```bash
RISC0_SKIP_BUILD=1 cargo build --release
```  
The first build can take up to 5 minutes.

2. Run the game
```bash
./target/release/fairclient
```

3. You can either host or connect to another game

4. Once you connect to the game, you will be able to choose your board

5. After that, both you and the other player would need to provide a proof of generating a valid game board

6. Each round, you can either  
  a. Fire a single shot in the cell  
  b. Send scouts to reveal enemy spaceships (Only 1 charge)  
  c. Fire a claster charge, to hit from 2 to 4 random cells in an area  
7. Destroy all 4 enemy ships to win  

Cells:
 - M - miss
 - H - hit
 - R - reveal (that means there is a ship in this cell, but it is still alive)

### Note

ZK is a very young piece of technology so please be patient while waiting for the proofs from server

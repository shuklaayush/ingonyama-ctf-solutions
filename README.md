# Ingonyama CTF write-up

![King of the Jungle](https://github.com/shuklaayush/ingonyama-ctf-solutions/assets/27727946/451c2dfb-2188-460a-bef2-a2f0df64644f)

The [Ingonyama CTF](https://ctf.ingonyama.com/), a Capture-the-flag competition held from May 13-15, 2023, was primarily focused on Zero-knowledge (ZK) and general cryptography. The contest was organized by Ingonyama, a company building hardware accelerators for zero-knowledge proofs (the term Ingonyama translates to "Lion"). The competition was a mix of challenges aimed at hacking or deciphering a cryptographic system to acquire a flag.

![Challenges](https://github.com/shuklaayush/ingonyama-ctf-solutions/assets/27727946/50070111-fca7-4084-abd2-dabfc784541f)

There were a total of 13 challenges, including 2 fun tasks. Although I was part of an automatically assigned team (King of the Jungle), I couldn't connect with my teammates and ended up doing the contest alone. I managed to solve 10 challenges and secured the 5th position (with 1357 points) on the leaderboard, out of ~35 participating teams.

Challenges that I solved:
- Safe bn254
- Lost Funds
- ZokClub
- The Day of Sagittarius IV
- ZokClub - As it should've been
- It is over 9000!
- Operation ZK rescue
- A Tale of Two Keys
- Loki's Vault
- The Power of iNTEgers

Unsolved challenges:
- The Lost Relic
- ZK Rescue Part 2
- Umculo

Below, I'll analyze each challenge individually, outlining my approach, solution, and the reasoning behind it.

## Solved challenges

### Safe bn254

> doesn’t bn254 look safer now?
> 
> y^2=x^3+2023 if you are not afraid of generating curves and discrete logarithms, you could try looking for a flag in x, where res = x * curve_gen
> 
> The generators are given by (in affine coordinates)
> 
> curve_gen_x=14810849444223915365675197147935386463496555902363368947484943637353816116538 curve_gen_y=742647408428947575362456675910688304313089065515277648070767281175728054553
> 
> The result coordinates are res_x=5547094896230060345977898543873469282119259956812769264843946971664050560756 res_y=14961832535963026880436662513768132861653428490706468784706450723166120307238
> 
> you can use any language for finding the solution and convert the flag into text format
> 
> The prime modulus in GF(p) p=21888242871839275222246405745257275088696311157297823662689037894645226208583

We've been provided with a modified BN-254 elliptic curve, and our task is to calculate a discrete logarithm on this curve (ECDLP). 

First, I set up the problem in [Sagemath](https://www.sagemath.org/):

```python
import math
from sage.all import *

# BN-254 prime
p = 21888242871839275222246405745257275088696311157297823662689037894645226208583

# Generator
Gx = 14810849444223915365675197147935386463496555902363368947484943637353816116538
Gy = 742647408428947575362456675910688304313089065515277648070767281175728054553

# P = kG
Px = 5547094896230060345977898543873469282119259956812769264843946971664050560756
Py = 14961832535963026880436662513768132861653428490706468784706450723166120307238

F = GF(p)
E = EllipticCurve(F, [0, 2023])

print(E)
```

```bash
Elliptic Curve defined by y^2 = x^3 + 2023 over Finite Field of size 21888242871839275222246405745257275088696311157297823662689037894645226208583
```

The ECDLP problem, generally regarded as computationally infeasible, hinges on the order of the subgroup of the elliptic curve. The curve should have a large prime order subgroup, as operations are conducted in this group. If the subgroup's order is small, the [Pohlig-Hellman algorithm](https://en.wikipedia.org/wiki/Pohlig%E2%80%93Hellman_algorithm) can be used to divide the problem into smaller, solvable parts. These parts can then be pieced back together using the Chinese Remainder Theorem (CRT) to arrive at the final solution. 

To check this, I factorized the order of the subgroup containing $G$:

```python
G = E(Gx, Gy)
order = G.order()
factors = factor(order)
print(f"Order of G = {order}")
print(f"           = {factors}")

P = E(Px, Py)
```
```bash
Order of G = 2203960485148121921418603742825762020959382274778627105322
           = 2 * 3^2 * 13 * 19 * 29 * 37 * 613 * 983 * 11003 * 346501 * 6248149 * 405928799 * 79287328374952431757
```

The largest prime in the prime factorization of the order is huge, making it computationally infeasible to solve. Therefore, I ignored the largest prime while calculating the discrete log. This gives us a solution modulo the remaining factors.

```python
dlogs = []
moduli = [p ** e for p, e in factors]
# Ignore largest factor to make problem computationally feasible
for m in moduli[:-1]:
    t = order // m
    dlog = discrete_log(t * P, t * G, operation="+")
    dlogs.append(dlog)

k = crt(dlogs, moduli[:-1])
print(f"Secret k: {k} mod {math.prod(moduli[:-1])}")

flag = bytearray.fromhex(hex(k)[2:]).decode("utf-8")
print(f"flag: {flag}")
```

Decoding the secret reveals that it conforms to the flag format:

```bash
Secret k: 381276930664415168886989783656063357 mod 27797133921898830561267529521791838546
flag: IngoCTF{b67e8a}
```

So, we've successfully found the flag.

### Lost Funds

> So last night I was going to transfer some crypto to a friend of mine. But when I copy-pasted his address - it was replaced by the different one, which i didn't notice till it was too late! Seems like I had some kind of malware installed on my computer... Anyway, can you track down where my funds went? I heard it's possible in blockchains. https://goerli.etherscan.io/tx/0xf681ca8c2999a11dc41983657af5afd164c56322925227b88f5693f1de8130f7 is the transaction link (whatever that is)

Following the trail of transactions on Goerli etherscan, we see that the culprit bridged the funds to the zkSync Era testnet. They then proceeded to transfer them repeatedly from one wallet to another on ZkSync. In order to trace the funds, I wrote a small Python script. 

```python
# Part 1
block = 0
address = "0xa2041c55902585ca3295f034f18d9000ad07738d"
while True:
    print(address)
    txs = get_txs(address)

    for tx in txs:
        if int(tx["blockNumber"]) < block:
            continue
        if "transfer" not in tx:
            continue
        if (
            tx["transfer"]["tokenInfo"]["address"]
            != "0x0000000000000000000000000000000000000000"
        ):
            continue
        if tx["transfer"]["from"] != address:
            continue
        if tx["transfer"]["to"] == address:
            continue

        block = int(tx["blockNumber"])
        address = tx["transfer"]["to"]
        break
    else:
        break
print(address)
# 0xd7120a6b038ad942a2966c2769451d21d608006f
```

The funds lead to [this address](https://goerli.explorer.zksync.io/address/0xD7120a6B038aD942A2966c2769451d21d608006f) and are then transferred back to Goerli. After a couple of hops, they are sent back to the zkSync Era testnet and end up at this [final address](https://goerli.explorer.zksync.io/address/0x475e3f18be51a970a3079dff0774b96da9d22dbe), where they are used up on gas across multiple transactions. Weird.

I wrote a script to decode all these transactions. Cross-referencing the function signatures with similar functions on a contract database, I found out that these belong to contracts for an on-chain casino hosted at [zkasino.io](zkasino.io).

```python
# Part 2
address = "0x475E3f18Be51A970a3079Dff0774B96Da9d22dbE"
txs = get_txs(address)

for tx in txs:
    result = subprocess.run(
        ["cast", "4byte-decode", tx["data"]["calldata"]], capture_output=True, text=True
    )
    print(result.stdout)
```

![transactions](https://github.com/shuklaayush/ingonyama-ctf-solutions/assets/27727946/a589a52e-4a95-4143-acd5-be240fecc2dd)

I found the [profile](https://play.zkasino.io/profile?u=0x475e3f18be51a970a3079dff0774b96da9d22dbe) corresponding to the exploiter's address on zKasino. It had the this badge: 

![crypt0cr1m1nal zkasino](https://github.com/shuklaayush/ingonyama-ctf-solutions/assets/27727946/c44191d3-7139-4f00-830f-36266fe63ed9)

The username piqued my curiosity but at this point, I felt as though I'd hit a dead-end. After a while, I went back to this challenge again and searched for the username on various search engines. This led me to a [Twitter profile](https://twitter.com/crypt0cr1m1nal) with this tweet.

![crypt0cr1m1nal twitter](https://github.com/shuklaayush/ingonyama-ctf-solutions/assets/27727946/ac808f3c-9162-4ad6-a763-fad2f4d61853)

Hurray, we found the flag!

### ZokClub

> In the last night's operation, the Crypto Police have discovered the former meeting place of a notorious anonymous crypto criminal secret society known as the "ZokClub." The police raided the location after receiving a tip-off from an anonymous source and found nothing except for a single note.
> 
> Unfortunately, the note said nothing... The meaning behind the note remains a mystery, but it is suspected to be a code that only members of the ZokClub would understand.
> 
> The ZokClub has been known to operate in the shadows of the dark web, engaging in illegal activities such as money laundering and hacking. The group's anonymity and sophisticated use of blockchain technology have made it difficult for law enforcement agencies to track them down, despite the fact they have their own public web page: https://zokclub.ctf.ingonyama.com/
> 
> We need your help to understand what are they up to. Try to infiltrate their group and gather the secret information.

The website made it clear that only holders of the ZokClub NFT could access the secret information. The site featured a detailed diagram outlining the club's operational blueprint: each club member possesses a secret and a nullifier that are used to create a Merkle tree. Using the secret-nullifier combo and a given Zokrates program, they generate a zero-knowledge proof proving their club membership. By submitting this proof on another page, they could mint a club NFT. Once the NFT is in the wallet, the secret can be revealed.

As we're not give a secret or a nullifier, I guessed that the objective was to forge a proof using  arbitrary values. The Zokrates code, which generates the ZK proof, is as follows:

```rust
def main(u32[8] root, private MerkleProof merkleProof, field nullifierHash, private field nullifier, private field secret) -> bool {
    // Check that note hash is in the merkle tree
    assert(checkMerkleProof(root, merkleProof));

    // Check that the nullifier hash match with public one
    field trueNullifierHash = calculateNullifierHash(nullifier);
    assert(nullifierHash == trueNullifierHash);

    // Construct note from secret and nullifier
    u8[16] nullifierBytes = cast::<128, 16>(nullifier);
    u8[16] secretBytes = cast::<128, 16>(secret);
    u8[32] constraintPreimageBytes = [...nullifierBytes, ...secretBytes];
    u32[8] trueConstraint = sha256padded::<32>(constraintPreimageBytes);
    
    return trueConstraint == merkleProof.leaf;
}
```

On closer inspection, one can spot that there's no assert statement in the last line. Instead, the function merely returns a boolean indicating whether or not the nullifier+secret combo is contained in the tree. Even if the boolean is false, a proof can still be generated. So the exploit is to use an arbitrary nullifier, its hash, and the correct Merkle root and Merkle proof to generate a counterfeit proof.

Upon submitting the forged proof and minting the club NFT, we get access to the secret flag, thereby accomplishing our infiltration into the ZokClub.

### The Day of Sagittarius IV

> We are excited to announce the release of the latest version of the retro terminal game "The Day of Sagittarius IV". This new version is designed to be played peer to peer and does not require any servers to play. Instead, the game data is stored on the players' computers, creating a truly decentralized gaming experience.
> 
> To ensure that players adhere to the rules of the game, the new version utilizes a zero-knowledge virtual machine. This feature allows players to verify that their opponents are playing fairly without revealing any confidential information.
> 
> We are currently in the beta-testing phase, and we invite all gaming enthusiasts to try out the new version of "The Day of Sagittarius IV".
> 
> As part of the beta-launch, we are also excited to announce a special promotional event. Players who win a bot in the game on 52.7.211.188:6000 will be eligible for a prize.
> 
> We believe that the new peer-to-peer version of "The Day of Sagittarius IV" will revolutionize the world of retro gaming, and we are thrilled to share it with you. Join us in this exciting new adventure, and let's explore the universe together!
> 
> https://github.com/ingonyama-zk/ctf-day-of-sagittarius

This was one of the most intriguing challenges: a modified [battleship game](https://en.wikipedia.org/wiki/Battleship_(game)) implemented using Zero-Knowledge (ZK) proofs in RISC-Zero. Each round, you could either: 

1. Fire a single shot in the cell
2. Send scouts to reveal enemy spaceships
3. Fire a claster charge, to hit from 2 to 4 random cells in an area

The game was played on an 8x8 board like the one below

```
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃          Player Board            ┃
┣━━┳━━━┳━━━┳━━━┳━━━┳━━━┳━━━┳━━━┳━━━┫
┃  ┃ a ┃ b ┃ c ┃ d ┃ e ┃ f ┃ g ┃ h ┃
┣━━╋───┼───┼───┼───┼───┼───┼───┼───┨
┃ 1┃ A │ A │ A │ A │   │   │   │   ┃
┣━━╋───┼───┼───┼───┼───┼───┼───┼───┨
┃ 2┃   │   │ D │ D │ B │ B │ B │   ┃
┣━━╋───┼───┼───┼───┼───┼───┼───┼───┨
┃ 3┃   │   │   │   │ C │   │   │   ┃
┣━━╋───┼───┼───┼───┼───┼───┼───┼───┨
┃ 4┃   │   │   │   │ C │   │   │   ┃
┣━━╋───┼───┼───┼───┼───┼───┼───┼───┨
┃ 5┃   │   │   │   │ C │   │   │   ┃
┣━━╋───┼───┼───┼───┼───┼───┼───┼───┨
┃ 6┃   │   │   │   │   │   │   │   ┃
┣━━╋───┼───┼───┼───┼───┼───┼───┼───┨
┃ 7┃   │   │   │   │   │   │   │   ┃
┣━━╋───┼───┼───┼───┼───┼───┼───┼───┨
┃ 8┃   │   │   │   │   │   │   │   ┃
┗━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┛
```

After playing (and losing) the game a few times against the bot, it became clear that the bot had some unfair advantage. It was hitting the target at each shot. Given that the game was based on a ZK design and  supposed to keep the state private, it wasn't immediately clear to me how the bot was cheating. I felt that either a backdoor was present, or the proofs were leaking information.

I started digging into the code to figure out where the bug was. In RISC Zero, [a proof](https://www.risczero.com/docs/explainers/proof-system/) comprises of a zk-SNARK and a journal entry that includes the public outputs of the computation. My hunch was that the journals were leaking state information so I started looking into the journals of all the actions. The cluster bomb one was interesting since it contained many fields:

```rust
pub struct ClusterCommit {
    pub old_state_digest: Digest,
    pub new_state_digest: Digest,
    pub config: ClusterBombParams,
    pub shots: alloc::vec::Vec<Position>,
    pub hits: alloc::vec::Vec<HitType>,
}
```

Everything seemed fine except for the config variable. Checking the definition of `ClusterBombParams`, we see that it contained the `GameState`, which should have been private. Voila!

```rust
pub struct ClusterBombParams {
    pub state: GameState,
    pub upper_left_coordinates: Position,
    pub down_right_coordinates: Position,
    pub seed: u8,
}
```
After finding this, I wrote a small piece of code that took this game state, rendered it and dumped it into a file. Now, all I had to do was start the game with a cluster charge, get the opponent's board state and use it to plan optimal moves. 

Both us and the bot play optimally now. But since our turn is first and the bot wastes a turn sending a scout, then we'd almost surely win. After winning, we are awarded with the the flag.

*Shoutout to another team who skipped all of this and instead tackled this challenge in true hacking spirit by writing a script to make the bot play itself and win.*

### ZokClub - As it should've been

> After our last infiltration ZokClub layed low and moved to another website: http://52.7.211.188:4000/
> 
> Crypto Police have discovered another former meeting place of a secret society. The police once again found nothing except for a single note.
> 
> **This time the note said: `S: ukEaZLLyQhfoKiKD, N: OCjw4gAbjXtTiJWJ`**
> 
> We looked through their website briefly, the only thing changed was that weird archive at GPI.zip
> 
> We again need your help to understand what are they up to. Try to infiltrate their group and gather the secret information.

This challenge was an extension to the previous ZokClub one. It introduced a bug fix, now checking that the boolean output is `true` within the NFT smart contract. This time, we're also given a valid secret and nullifier. However, the nullifier had already been used to mint an NFT and trying to use it again would be rejected by the NFT contract.

So I went back to the Zokrates code:

```rust
def main(u32[8] root, private MerkleProof merkleProof, field nullifierHash, private field nullifier, private field secret) -> bool {
    // Check that note hash is in the merkle tree
    assert(checkMerkleProof(root, merkleProof));

    // Check that the nullifier hash match with public one
    field trueNullifierHash = calculateNullifierHash(nullifier);
    assert(nullifierHash == trueNullifierHash);

    // Construct note from secret and nullifier
    u8[16] nullifierBytes = cast::<128, 16>(nullifier);
    u8[16] secretBytes = cast::<128, 16>(secret);
    u8[32] constraintPreimageBytes = [...nullifierBytes, ...secretBytes];
    u32[8] trueConstraint = sha256padded::<32>(constraintPreimageBytes);
    
    return trueConstraint == merkleProof.leaf;
}
```

Inspecting further, specifically at the casting function, I found that higher order bytes were ignored during the casting of the nullifier:

```rust
def cast<N, P>(field input) -> u8[P] {
    bool[FIELD_SIZE_IN_BITS] bits = unpack(input);
    bool[N] bits_input = bits[FIELD_SIZE_IN_BITS-N..];
    assert(N == 8 * P);
    u8[P] mut r = [0; P];
    for u32 i in 0..P {
        r[i] = u8_from_bits(bits_input[i * 8..(i + 1) * 8]);
    }
    return r;
}
```

Knowing this, I realized it was possible to take the given nullifier (which was known to be in the merkle tree), prepend some bytes to it, compute the nullifier hash, and generate a valid proof. Since the new nullifier differed from the old one, it wasn't rejected by the smart contract.

So we again generate a fake proof, mint the NFT, and obtain the secret.

### It is over 9000!

> The Saiyans have landed on planet earth. Our great defenders Krillin, Piccolo, Tien and Gohan have to hold on till Goku arrives on the scene.
> 
> Vegeta and Nappa have scouters that indicate our heroes power levels and sadly we are not doing too well.
> 
> Somehow, Gohan has raised his power level to `p_4(X) = 9000`, but it is not good enough. Piccolo `p_3(X)` can help but he is still regenerating, and Krillin `p_2(X)` and Tien `p_1(X)` are in bad shape. The total power of the team is computed as
> ```
> P = p_1(X) * 0 + p_2(X) * 0 + p_3(X) * 0 + p_4(X)
> ```
> At the current moment, the X is equal to `42`.
> 
> Suddenly Gohan, and Piccolo recieve a message from Bulma that the scouters verify the sensed power level of individual enemies using KZG and for multiple enemies with batched KZG method. Vegeta knows for sure that the power level of Gohan is `p_4(X) = 9000`, so he will know if we change that. If only the team had a way to trick their opponents to believe that their total power level is `P > 9000` - then the enemies will surely flee.
> 
> To run
> ```bash
> cargo run --release
> ```
> Verification
> Vegeta is running the scouters from `52.7.211.188:8000`.
> 
> https://github.com/ingonyama-zk/ctf-over-9000

The key to solving this challenge is understanding how batching works in KZG commitments. Looking into the code reveals two important functions, `open_batch` and `verify_batch`. The crucial insight comes from realizing that the `upsilon` parameter has to be random. Otherwise, it can be used to make the verification ignore some of the polynomials.

```rust
fn open_batch(
    &self,
    x: &FieldElement<F>,
    ys: &[FieldElement<F>],
    polynomials: &[Polynomial<FieldElement<F>>],
    upsilon: &FieldElement<F>,
) -> Self::Commitment {
    let acc_polynomial = polynomials
        .iter()
        .rev()
        .fold(Polynomial::zero(), |acc, polynomial| {
            acc * upsilon.to_owned() + polynomial
        });

    let acc_y = ys
        .iter()
        .rev()
        .fold(FieldElement::zero(), |acc, y| acc * upsilon.to_owned() + y);

    self.open(x, &acc_y, &acc_polynomial)
}

fn verify_batch(
    &self,
    x: &FieldElement<F>,
    ys: &[FieldElement<F>],
    p_commitments: &[Self::Commitment],
    proof: &Self::Commitment,
    upsilon: &FieldElement<F>,
) -> bool {
    let acc_commitment =
        p_commitments
            .iter()
            .rev()
            .fold(P::G1Point::neutral_element(), |acc, point| {
                acc.operate_with_self(upsilon.to_owned().representative())
                    .operate_with(point)
            });

    let acc_y = ys
        .iter()
        .rev()
        .fold(FieldElement::zero(), |acc, y| acc * upsilon.to_owned() + y);
    self.verify(x, &acc_y, &acc_commitment, proof)
}
```

In the code, `u` (`upsilon`) can be selected by the prover. By hardcoding it to `0` instead of a random value, only `p1` is checked. Now since `p4` isn't checked, you can keep the polynomial the same so that Vegeta can verify the commitment to it, and confirm Gohan's power level. But you can change the `y4` value, which would never be verified in the `batch_verify` function. As a result, the sum of power levels appears to be over 9000, fooling the villains.

### Operation ZK rescue

> Message from HQ: High priority
> 
> **Agent Zulu has gone M.I.A.**
> 
> We have received reports that he has been kidnapped by the notorious Woe Jinx. Direct intervention without evidence is not an option. Our friendly enemy The Concierge of crime: Red, has managed to get one of his associates (The forger) infiltrate into Jinx's organization, who will be your point contact.
> 
> Your task is to send us a confirmation message that indeed Zulu is inside Jinx's base so we can rescue our man quietly.
> 
> Go to the Github Repository to get the full details on this challenge.
> 
> https://github.com/ingonyama-zk/operation_zk_rescue
> 

After executing the challenge using `cargo run`, you get an interactive story-telling prompt:

```
Agent Zulu has gone M.I.A.

We have received reports that he has been kidnapped by the notorious Woe Jinx.
Direct intervention without evidence is not an option.
Our friendly enemy The Concierge of crime: Red, has managed to get one of his associates
(The forger) infiltrate into Jinx's organization, who will be your point contact.

Your task is to send us a confirmation message that indeed Zulu is inside Jinx's base so we
can rescue our man quietly.

Jinx uses a sumcheck protocol that validates the sender's identity in the base,
when the sumcheck evaluates to zero.

The Forger has forged an identity for you in order to faciliate a one time message.
However, we ran some tests and found that it may not pass the validation.
We have no idea what game Red and the forger are playing here.

We do know that Woe Jinx protects his men from HQ by anonymizing the validation process, this basically
adds a random polynomial to the claimed polynomial. This is usually a real pain in the butt.
But, perhaps the anonymization can be used to your advantage this time. Just watch out that Jinx double checks the anonymization,
so if you use a constant polynomial for anonymization, you will get caught!

Once you have cleared the validation, we will use a security lapse window to activate recieving a one time message from you.
We have been told by Red that you will have to eventually find some of the information you need on your own.
U have got Big intELLIGENCE, be YOURSELF! We are expecting your message in 8 in the futURE.

Note that if Jinx learns the message during the validation, the probability you will live is pretty low.

One more thing, Red and the Forger cannot be trusted, there is always more to what meets the eye!
Watch out!  Good luck!! - HQ
```

I wasted a lot of time analyzing the weird capitalization in one of the sentences. This turned out to be a red-herring and actually, none of the prompt is important.

If you convert the polynomial evaluations in the code into ASCII, you get the following list:
```
INGONYAMA_THEö
              #@Åç"
PINOCCHIO
TinyRAM
GROTH16
BULLETPROOFS
STARK
SONIC
PLONK
TURBOPLONK
SPARTAN
HALO
AURORA
MARLIN
FRACTAL
LUNAR
VIRGO
SUPERSONIC
PLOOKUP
BRAKEDOWN
NOVA
PLONKY2
HALO2
GEMINI
CAULK
CAULK+
ORION
FLOOKUP
HYPERPLONK
BALOO
CQ
SUPERNOVA
CQlin
```

Apart from the first one, the others are all names of modern ZK proving systems. The first entry looks like a corrupted flag and gives away a hint that the flag is related to the evaluations somehow.

Looking into the sum-check code, I realized that this is exactly the [ZK-hack puzzle](https://gist.github.com/shuklaayush/b92e6b53b0ff8571c0e73d42b504f7e3) that I did a few months ago. It's a sum-check protocol where the masking is improper. We can select any masking polynomial whose sum over the domain is 0, and the check will pass. If we calculate this sum over domain and turn that number into ASCII, we get the flag `INGONYAMA_THE_LION_INSIDE`. Upon entering this flag when the code prompts, the code calculates its hash and confirms that this is indeed the solution.


### A Tale of Two Keys

> Alice deployed a Groth16 based system, and to convince everyone her system is secure and the secrets used in the setup are not exposed, she thought of a clever way - she would publish a few different circuits that share the same secrets, that don't have a valid solution. In this way, the only way malicious prover could create proofs would be by using the exposed secret. she put a large bounty in one of these to incentivize hackers to look at it.
> 
> See more details on Github: https://github.com/ingonyama-zk/TaleOfTwoKeys

(This challenge was updated during the CTF. I only solved the initial unmodified challenge.)

In the initial version, the problem posed was to find the square root of 15 and 17. Given that 15 is a quadratic residue in the BLS12-377 scalar field, calculating the square root was trivial. Thus, by simply generating a valid proof for the square root of 15 and submitting it, I was able to get the flag.

The revised challenge tweaked the parameters. The value 15 was replaced with 11, which is a non-residue. This turned the challenge into a seemingly more complex problem that I didn't have time to look into.

### Loki's Vault

> After years of careful investigation, you have reached the gate to Loki's vault in the icy mountains of Norway, where it is said that many great treasures and powerful weapons are hidden. The gate seems unbreakable, but you spot some ancient machinery with inscriptions in old runes.
> 
> Read more: https://github.com/ingonyama-zk/breaking_into_vault_of_loki

This challenge was one of the toughest and needed a good understanding of how KZG commitments work. We are provided with the following polynomial

$$
\begin{aligned}
p(x) &= 69 +78x + 32x^2 + 65x^3 + 82x^4 + 71x^5 + 69x^6 + 78x^7 + 84x^8 + 73x^9 \newline &+78x^{10} + 65x^{11} + 32x^{12} + 78x^{13} + 65x^{14}+ 67x^{15} + 73x^{16} + 32x^{17} \newline
&+ 84x^{18} + 73x^{19} + 69x^{20} + 82x^{21} + 82x^{22} + 65 x^{23} 
\end{aligned}
$$

Our objective is to generate a KZG proof that the polynomial equals $3$ at $x = 1$ within the BLS12-381 scalar field.

Given that the polynomial's value is clearly not $3$ at $x = 1$, it is clear that forging a proof is our only option. So I started looking into KZG commitments in detail. [Dankrad's article](https://dankradfeist.de/ethereum/2020/06/16/kate-polynomial-commitments.html) was a great resource in understanding how KZG commitments work. Essentially, a person can create a fraudulent proof if they know the secret "toxic waste" $s$ from the trusted setup. I looked into how trusted setups work by reading [Vitalik's article](https://vitalik.ca/general/2022/03/14/trustedsetup.html). The SRS string derived from the trusted setup has the following form:

$$
[G_1, G_1 * s, G_1 * s^2 ... G_1 * s^{n_1-1}]\\
[G_2, G_2 * s, G_2 * s^2 ... G_2 * s^{n_2-1}]\\
$$

The given SRS had $n_1 = 1024$ and $n_2 = 2$ with $G_1$ and $G_2$ representing the generators of the main and secondary group of the elliptic curve.

Analyzing the powers in the main group, I realized that they were repeating with a frequency of 64 i.e. $s^{64} = 1\mod p$. This implied that $s$ is a 64th root of unity. To determine $s$, I wrote a simple Sage script to calculate the 64 roots of unity and find the correct one:

```python
from sage.all import *

p = 0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab
K = GF(p)
a = K(0x00)
b = K(0x04)
E = EllipticCurve(K, (a, b))
G = E(0x17F1D3A73197D7942695638C4FA9AC0FC3688C4F9774B905A14E3A3F171BAC586C55E83FF97A1AEFFB3AF00ADB22C6BB, 0x08B3F481E3AAA0F1A09E30ED741D8AE4FCF5E095D5D00AF600DB18CB2C04B3EDD03CC744A2888AE40CAA232946C5E7E1)
E.set_order(0x73EDA753299D7D483339D80809A1D80553BDA402FFFE5BFEFFFFFFFF00000001 * 0x396C8C005555E1568C00AAAB0000AAAB)

sG = E(0xb45e08705bc9f96ddef642f24f7e6d326c5e450aefb21363fd8c6788591afca990680a8f862e8d43609430f54aca45f, 0x63e8c7cd26cee9463932fd15ddaac016f42d598bd1abedfdfc37bfeb9f326cd80e36ab003b5b4a79bb25c5695e291b)

q = G.order()
L = GF(q)
g = L.multiplicative_generator()

n = 64
s = 0
for i in range(n):
    s = pow(g, i * (q - 1) // n)
    if s*G == sG:
        print(f"Found s: {s}")
        break
```

Once $s$ is known, we can construct a fake proof using the formula:

$$
\pi_{fake} = \frac{1}{s - y} (C - yG) 
$$

Here, $s$ is the secret, $C$ is the commitment, and $y$ is the intended fake value for which the proof is to be generated.

Below is another Sage script to compute this:

```python
x = 1
ynew = 3

k = pow(L(s - x), -1)
C = E(0x1167e707d11074bef0ee02040d38c06e32d829341246af1ba03572a61a3d2052d687d5ebb5de356ff089006e6318bb8b, 0x1537d55fffdd6d31d1b831bb8ac2e24142084f122c830cddc117d31505d49becd3854df61ce8b7f7ca14aa8f3a0eb0c)

proof = k*(C - ynew*G)
print([hex(z) for z in proof])
```

As stated in the problem description, the $x$ coordinate of the fake proof is the solution.

### The Power of iNTEgers

> 59-213-402-213-964-402-213-149-310-534
> 
> Format: XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX (not specifically 3 letters, English)
> 
> **Hint**
> Greek: equal pebble

From the hint, it was clear that this challenge was based on Isopsephy ("equal pebble"), the Greek practice of adding up the numerical values of the letters in a word to form a total. I constructed an Isopsephy table for English letters to use:

| Key | Value |      | Key | Value |      | Key | Value |      | Key | Value |
| --- | ---   | ---  | --- | ---   | ---  | --- | ---   | ---  | --- | ---   |
| a   | 1     |      | k   | 20    |      | t   | 200   |      | y   | 700   |
| b   | 2     |      | l   | 30    |      | u   | 300   |      | z   | 800   |
| c   | 3     |      | m   | 40    |      | v   | 400   |      |     |       |
| d   | 4     |      | n   | 50    |      | w   | 500   |      |     |       |
| e   | 5     |      | o   | 60    |      | x   | 600   |      |     |       |
| f   | 6     |      | p   | 70    |      |     |       |      |     |       |
| g   | 7     |      | q   | 80    |      |     |       |      |     |       |
| h   | 8     |      | r   | 90    |      |     |       |      |     |       |
| i   | 9     |      | s   | 100   |      |     |       |      |     |       |
| j   | 10    |      |     |       |      |     |       |      |     |       |

The task was to identify meaningful words for each provided number that together also form a meaningful sentence/phrase. I used [NLTK](https://www.nltk.org/) library's Brown word list as my reference dictionary and wrote a small python script to find candidate words for each number. Here are the words that I found (ordered by decreasing frequency):

```bash
['in', 'end', 'aimed', 'media', 'cane']
['the', 'desire', 'homes', 'gate', 'helps', 'acted', 'orange', 'ribbon']
['guard', 'bus', 'urge', 'faster', 'jungle', 'crude']
['the', 'desire', 'homes', 'gate', 'helps', 'acted', 'orange', 'ribbon']
['mighty']
['guard', 'bus', 'urge', 'faster', 'jungle', 'crude']
['the', 'desire', 'homes', 'gate', 'helps', 'acted', 'orange', 'ribbon']
['fiscal', 'flesh', 'pink', 'ladies', 'ideals', 'lion', 'limp', 'shelf']
['not', 'father', 'facts', 'tragic', 'dates', 'sons', 'ton', 'pepper']
['liver', 'drums']
[59, 213, 402, 213, 964, 402, 213, 149, 310, 534]
```

Analyzing the potential word combinations and keeping in mind the CTF's lion-themed context, it was easy to figure out that "[in the jungle, the mighty jungle, the lion sleeps tonight](https://www.youtube.com/watch?v=OQlByoPdG6c)" was the solution. 

## Unsolved challenges

There were some challenges I wasn't able to solve within the timeframe of the competition. However, I revisited them after the competition concluded, sought hints and tried to solve them to gain further understanding.

### The Lost Relic

> During their quest to find the greatest treasure in the world, the One Piece, Luffy and his friends are wandering inside a subterranean maze. After many hours, they arrive at the door hiding an old relic, which can be instrumental to achieving their goal. The big problem is that it is made of sea stone and Luffy is unable to use his strength to break it. There are some inscriptions on the walls, which Nico Robin is able to translate.
> 
> It says: "If you can find the secret hidden among these texts, the door will open."
> 
> There are many input plaintexts and their corresponding ciphertexts, all of them encrypted using a custom MiMC algorithm under the same key. There are also many skeletons around, of all the people who have so far failed this test. Luckily, Usopp brought his computing device and will try to break the secret. What can he do to recover the secret?
> 
> https://github.com/ingonyama-zk/the_lost_relic

We're given a bunch of plaintext-ciphertext pairs. The block cipher function, a modified MiMC hash, for each round is given as:

$$
x_{n+1} = (x_n + k)^2
$$

Our goal is to find the secret key, $k$.

I spent a lot of time trying to devise an algebraic attack on the cipher, given its simplicity. However, I couldn't figure out a way. After the CTF was over, I was given the hint that one of the pairs was a slide pair and we had to do a [slide attack](https://en.wikipedia.org/wiki/Slide_attack). Below is the implementation of this attack in Sage:

```python
from sage.all import *
from itertools import permutations, product

# Stark252 prime field
p = 0x800000000000011000000000000000000000000000000000000000000000001
Fp = GF(p)

g = Fp.multiplicative_generator()
print(g.order())
print(factor(p - 1))

# Plaintext-ciphertext pairs
data = [
    (0x28b0063846b89588ca3685b89a541f0d928fe5a9cbe96d5abb7f7abb629d3e2, 0xce80a279e03ffd68f6394329f7d10991cc93950cddb7506485d8ff6ed7b8e2),
    (0xbf1ce8008359702c83d1bac70b7612c928687a3eeb1f753968d2e194868429, 0x45230772a45ccf3dd959fe19551b8e8a2d477bb9cc34ddada45f8b5e1df7c60),
    (0x7e2b171d99abe9032dfea6b45a926f5ee48f8e12b0ff78adcdf584c4c639bd1, 0x305f75862c31d4b39d39d153bbcfdc057ad3c1be8ea7ff4caf98c19b902063a),
    (0x2e15cb9cc16936d3ae45a48908ae64188ad3e54b6461d026de2e9119a0fb92c, 0x6a03ffde37a63f1d6f8457c32319095fdd66f0aaa2bcd11aae2c392c2b2a5bd),
    (0x5302462b15a1278251a78d55d73808c6baea5f97239698ec01ffe8a9ae08b9d, 0x6f6242d42fff75a04ff5616c1e8462885c9ae45823f66313f99f2e746df9add),
    (0x136d355d557ae436cb02f0b10b846771cfc84b9179043c846508844b14027f7, 0x4f052e8a02fc19db85484ab772b86e873d225077fc62050e42611d104283cae),
    (0x122fead16809905bdeca0792e650b0c14d217a2a6e1228621f51361594b4f06, 0x3f7755174d8812e217affe00a53cf48ecf507d0ba0f7ffd0a52eda877ae67d2),
    (0x77c1a014b96c0de113bd4594a98d5e15241969b88629ad1a18ca4a324af353b, 0x6f87bb1349583cf1d382f0ddf9987d21cbbdb54afe279e87df057ee8516318b),
    (0x77365982f9d84204a371ee8301edc2eff4a907094b2c81992727738758db99c, 0xc40395a16b69960867177ccad31c172dfc205fa4f305eaf7f4f44e08629da9),
    (0x5d46134ee8397c82949da3f15831730dae280e40d0ee4af4f9be056cd1ce05a, 0x3690fd96de5feadedb4a588eb8a27c4b279d384c3529a0b9eabd5546d79d331),
]
data = [(Fp(x), Fp(y)) for x, y in data]

# Find slide pair by looking at all permutations and calculate the secret key
for (x1, y1), (x2, y2) in permutations(data, 2):
    if pow(y2, (p-1)//2) == 1 and pow(x2, (p-1)//2) == 1:
        x2roots = [x2.sqrt(), -x2.sqrt()]
        y2roots = [y2.sqrt(), -y2.sqrt()]

        for (x2root, y2root) in product(x2roots, y2roots): 
            if y2root - x2root == y1 - x1:
                k = x2root - x1
                print(f"k: {k}")
                print(f"k: {hex(k)}")
                break
```

### ZK Rescue Part 2

> Hey it's me, Red. In today's fast-paced jungle, where communication can be as intricate as the pages of a book, the real art of understanding is to read between the lines, and not just hear what people say. I remember hearing this wisdom from an old man in India 31 years ago, just before I visited the Louvre in Paris. Much like the Louvre's many artworks, people's true meanings and intentions are often hidden beneath the surface, waiting to be uncovered. What you did to get the first flag can also be done in other places, just need to lookup.

This was a continuation of the ZK Rescue challenge. I had no idea how to solve this.

After the CTF, I came to know that the flag was "BALOO." Baloo, in addition to being a [ZK proof system](https://eprint.iacr.org/2022/1565.pdf), is also a character from the classic tale, The Jungle Book. Looking back, I think the subtle hints from Red's message, "jungle" and "book," were meant to lead us towards this answer. However, I'm still not entirely sure if this was supposed to be the only hint. Some teams managed to brute force their way to the solution by putting in all the decoded strings from the polynomial evaluations.

### Umculo

> ![Umculo](https://github.com/shuklaayush/ingonyama-ctf-solutions/assets/27727946/8508d5f2-a8ce-42a3-bd70-08afe1c2a0b7)
> Format: XXX XXX XXX (not specifically 3 letters,English)
> 
> **Hint**
> Within the many challenges, the songs' name is already familiar, Seek the singers' words, for the riddle to be figured.

I realized that the top-left image referred to Imagine Dragons but I couldn't figure out anything else. Even after the CTF is over and multiple hints, I still don't know what the solution is.

## Conclusion

Thank you to Ingonyama and all the sponsors for putting together such a fun competition. My main goal in participating was to get better at and test my knowledge of ZK cryptography. The contest was a great learning experience, and I realized how much progress I've made.

On the first day, I struggled to solve any challenges, but as I invested more time, I started figuring out how to approach them. This was a real confidence booster and a great source of motivation. Yes, it took up my entire weekend, but I'm very happy that I decided to take part. :)

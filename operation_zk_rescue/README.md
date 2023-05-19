# Operation ZK rescue

---
***Message from HQ: High priority***

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
U have got Big intELLIGENCE, be YOURSELF!

Note that if Jinx learns the message during the validation, the probability you will live is pretty low.

One more thing, Red and the Forger cannot be trusted, there is always more to what meets the eye!
Watch out!  Good luck!! - HQ

---
***Message from the FORGER***

Hi there this is the Forger, I have crafted a identity for you and it is in the form of a polynomial. 
Note that you cant change the given polynomial. You have to use your knowledge of the
univariate sumcheck protocol to complete the code in the `prover.rs` file.
You may want to consult the attached document in the docs folder for reference.

BTW: Red said 'You will have to find some of the information yourself', you know what it sounds like right? sigh..Goodluck and Godspeed.

---
***Note***

1. Clone the repository and keep it private. Use `cargo run --release` to run the puzzle.
2. If you complete the sumcheck challenge, you will get an opportunity to check your first flag.
3. The flag is of the format XXXX_XXXX_XXXX
4. Submit your flag in the corresponding challenge (Operation ZK Rescue) in [Ingonyama CTF](https://ctf.ingonyama.com)
5. Listen to Red for more clues.

---

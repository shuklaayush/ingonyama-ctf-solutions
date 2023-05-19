# A tale of two keys

---
Alice deployed a Groth16 based system, and to convince everyone her system is secure and the secrets used in the setup are not exposed, she thought of a clever way - she would publish a few different circuits that share the same secrets, that don't have a valid solution. in this way, the only way malicious prover could create proofs would be by using the exposed secret. she put a large bounty in one of these to incentivize hackers to look at it.

---

***Note**

The code in this repo will help you see that it is --impossible-- to create a proof with such configuration. If you somehow manage to generate a valid proof - please send it to me at totk.ctf.ingonyama.com:5000

`nc totk.ctf.ingonyama.com 5000`
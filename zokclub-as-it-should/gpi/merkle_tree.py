#!/usr/bin/env python3

import json
import hashlib
import random
import string

class MerkleTree:
    def __init__(self, lst):
        self.leaves = [hashlib.sha256(s.encode()).hexdigest() for s in lst]
        self.levels = [self.leaves]

        while len(self.levels[-1]) > 1:
            level = []
            for i in range(0, len(self.levels[-1]), 2):
                left = self.levels[-1][i]
                right = self.levels[-1][i+1] if i+1 < len(self.levels[-1]) else left
                combined = left + right
                level.append(hashlib.sha256(bytes.fromhex(combined)).hexdigest())
            self.levels.append(level)

    def get_root_hash(self):
        return self.levels[-1][0]

    def get_proof(self, leaf_index):
        proof_hashes = []
        proof_directions = []

        for level in self.levels:
            if len(level) == 1:
                break

            if leaf_index % 2 == 0:
                sibling_index = leaf_index + 1
                is_left = True
            else:
                sibling_index = leaf_index - 1
                is_left = False

            if sibling_index < len(level):
                proof_hashes.append(level[sibling_index])
                proof_directions.append(not is_left)

            leaf_index //= 2

        return proof_hashes, proof_directions
    
    def to_dict(self):
        return {
            "leaves": self.leaves,
            "levels": self.levels
        }

    @classmethod
    def from_dict(cls, dct):
        obj = cls.__new__(cls)
        obj.leaves = dct["leaves"]
        obj.levels = dct["levels"]
        return obj
    
if __name__ == "__main__":
    TREE_DEPTH = 5
    NUMBER_OF_LEAVES = 2 ** TREE_DEPTH

    # 16 byte secrets
    secrets = [''.join([random.choice(string.ascii_letters + string.digits) for _ in range(16)]) for _ in range(NUMBER_OF_LEAVES)]
    print("Secrets:", secrets)
    print()

    # 16 byte nullifiers
    nullifiers = [''.join([random.choice(string.ascii_letters + string.digits) for _ in range(16)]) for _ in range(NUMBER_OF_LEAVES)]
    print("Nullifiers:", nullifiers)
    print()

    # 32 byte notes
    notes = [n + s for n, s in zip(nullifiers, secrets)]
    print("Notes:", notes)
    print()

    # Generate tree
    tree = MerkleTree(notes)
    with open('tree.json', 'w') as f:
        json.dump(tree.to_dict(), f, indent=4)

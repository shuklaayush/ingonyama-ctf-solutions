#!/usr/bin/env python3

from merkle_tree import MerkleTree
from zok_poseidon import poseidon

import argparse
import hashlib
import json

def split_to_bytes32(value):
    """
    Split the given string into 8 integers of 4 bytes each.
    """

    if isinstance(value, str):
        value = int(value, 16).to_bytes(32, 'big')

    parts = [value[i:i+4] for i in range(0, len(value), 4)]
    return ["0x" + p.hex() for p in parts]

def compute_poseidon(value):
    return poseidon([value])

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Merkle Tree Proof Generator')
    parser.add_argument('secret', type=str, help='16-byte secret')
    parser.add_argument('nullifier', type=str, help='16-byte nullifier')
    args = parser.parse_args()

    with open('tree.json', 'r') as f:
        tree = MerkleTree.from_dict(json.load(f))

    note = args.nullifier + args.secret
    # commitment = hashlib.sha256(note.encode()).hexdigest()
    commitment = tree.leaves[0]

    try:
        comm_index = tree.leaves.index(commitment)
    except ValueError:
        print('Error: commitment not in tree')
        exit(1)

    proof, directions = tree.get_proof(comm_index)

    root_bytes = split_to_bytes32(tree.get_root_hash())
    leaf_bytes = split_to_bytes32(commitment)
    path_bytes = [split_to_bytes32(p) for p in proof]

    nullifier_number = int.from_bytes(args.nullifier.encode(), 'big')
    secret_number = int.from_bytes(args.secret.encode(), 'big')

    nullifier_hash = compute_poseidon(nullifier_number)

    data = [
        root_bytes,
        {
            'leaf': leaf_bytes,
            'directionSelector': directions,
            'path': path_bytes,
        },
        hex(nullifier_hash),
        hex(nullifier_number),
        hex(secret_number),
    ]

    with open('inputs.json', 'w') as f:
        json.dump(data, f, indent=4)

    print('Inputs for witness generation was put into "inputs.json"')

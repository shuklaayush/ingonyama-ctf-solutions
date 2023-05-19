# https://eprint.iacr.org/2019/458.pdf
# translated from https://github.com/Zokrates/ZoKrates/blob/develop/zokrates_stdlib/stdlib/hashes/poseidon/poseidon.zok
# with the help of Chat-GPT
# Correctness and perfomance might be bad

from zok_constants import POSEIDON_C, POSEIDON_M

P = 21888242871839275222246405745257275088548364400416034343698204186575808495617

def ark(state, c, it):
    for i in range(len(state)):
        state[i] = (state[i] + c[it + i]) % P
    return state

def sbox(state, f, p, r):
    state[0] = pow(state[0], 5, P)
    for i in range(1, len(state)):
        if r < f//2 or r >= f//2 + p:
            state[i] = pow(state[i], 5, P)
    return state

def mix(state, m):
    out = [0] * len(state)
    for i in range(len(state)):
        acc = 0
        for j in range(len(state)):
            acc = (acc + state[j] * m[i][j]) % P
        out[i] = acc
    return out

def poseidon(inputs):
    assert len(inputs) > 0 and len(inputs) <= 6

    t = len(inputs) + 1
    rounds_p = [56, 57, 56, 60, 60, 63, 64, 63]

    f = 8
    p = rounds_p[t - 2]

    c = POSEIDON_C[t - 2]
    m = POSEIDON_M[t - 2]

    state = [0] * t
    for i in range(1, t):
        state[i] = inputs[i - 1]

    for r in range(f + p):
        state = ark(state, c, r * t)
        state = sbox(state, f, p, r)
        state = mix(state, m)

    return state[0] % P
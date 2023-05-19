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

G = E(Gx, Gy)
order = G.order()
factors = factor(order)
print(f"Order of G = {order}")
print(f"           = {factors}")

P = E(Px, Py)

dlogs = []
moduli = [p ** e for p, e in factors]
# Ignore largest factor to make problem computationally feasible
for m in moduli[:-1]:
    t = order // m
    dlog = discrete_log(t * P, t * G, operation="+")
    dlogs.append(dlog)

k = crt(dlogs, moduli[:-1])

assert(k * G == P)
print(f"Secret k: {k} mod {math.prod(moduli[:-1])}")

flag = bytearray.fromhex(hex(k)[2:]).decode("utf-8")
print(f"flag: {flag}")

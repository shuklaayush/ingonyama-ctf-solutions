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

x = 1
ynew = 3

k = pow(L(s - x), -1)
C = E(0x1167e707d11074bef0ee02040d38c06e32d829341246af1ba03572a61a3d2052d687d5ebb5de356ff089006e6318bb8b, 0x1537d55fffdd6d31d1b831bb8ac2e24142084f122c830cddc117d31505d49becd3854df61ce8b7f7ca14aa8f3a0eb0c)

proof = k*(C - ynew*G)
print([hex(z) for z in proof])

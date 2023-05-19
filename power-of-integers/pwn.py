import nltk
from nltk.corpus import brown
from pprint import pprint
from collections import defaultdict
from itertools import combinations_with_replacement, permutations, product

cipher = "59-213-402-213-964-402-213-149-310-534"
cipher = [int(n) for n in cipher.split("-")]

table = {
    'a': 1,
    'b': 2,
    'c': 3,
    'd': 4,
    'e': 5,
    'f': 6,
    'g': 7,
    'h': 8,
    'i': 9,
    'j': 10,
    'k': 20,
    'l': 30,
    'm': 40,
    'n': 50,
    'o': 60,
    'p': 70,
    'q': 80,
    'r': 90,
    's': 100,
    't': 200,
    'u': 300,
    'v': 400,
    'w': 500,
    'x': 600,
    'y': 700,
    'z': 800,
}

def encode_word(w):
    return sum([table[c] for c in w])

def encode_sentence(s):
    return [encode_word(w) for w in s.split()]

inv = {v: k for k, v in table.items()}

# with open('/usr/share/dict/words', 'r') as f:
#     wordlist = set(f.read().splitlines())

nltk.download('brown')
words = brown.words()
word_frequency = nltk.FreqDist(words)

candidates = defaultdict(set)
for num_chars in range(1, 7):
    combs = list(combinations_with_replacement(table.values(), num_chars))

    for num in set(cipher):
        valid_combs = [comb for comb in combs if sum(comb) == num]

        for comb in valid_combs:
            chars = ''.join([inv[n] for n in comb])
            perms = [''.join(p) for p in permutations(chars)]
            
            for word in perms:
                if word_frequency[word] > 10:
                    candidates[num].add(word)

assert(len(candidates) == len(set(cipher)))

for n in cipher:
    print(sorted(candidates[n], key=lambda word: word_frequency[word], reverse=True))

print(encode_sentence("in the jungle the mighty jungle the lion sleeps tonight"))

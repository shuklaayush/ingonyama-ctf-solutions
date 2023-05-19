import requests
import subprocess
from collections import defaultdict

ENDPOINT = "https://zksync2-testnet-explorer.zksync.dev/transactions"


def get_txs(address, limit=50):
    r = requests.get(
        ENDPOINT, params={"limit": 50, "direction": "newer", "accountAddress": address}
    )
    r.raise_for_status()

    return r.json()["list"]


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

# Part 2
address = "0x475E3f18Be51A970a3079Dff0774B96Da9d22dbE"
txs = get_txs(address)

for tx in txs:
    result = subprocess.run(
        ["cast", "4byte-decode", tx["data"]["calldata"]], capture_output=True, text=True
    )
    print(result.stdout)

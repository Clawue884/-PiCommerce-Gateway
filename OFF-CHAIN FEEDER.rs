import time
import hashlib
import nacl.signing
import requests

PEG_VALUE = 314_159  # example
TIMESTAMP = int(time.time())

# load feeder private key
sk = nacl.signing.SigningKey(open("feeder.sk", "rb").read())

payload = (
    PEG_VALUE.to_bytes(16, "big") +
    TIMESTAMP.to_bytes(8, "big")
)

hash_bytes = hashlib.sha256(payload).digest()
signature = sk.sign(hash_bytes).signature

# send to blockchain (RPC / SDK)
print("peg:", PEG_VALUE)
print("timestamp:", TIMESTAMP)
print("signature:", signature.hex())

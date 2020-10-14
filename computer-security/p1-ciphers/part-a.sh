#!/usr/bin/env bash

set -eux
set -o errexit

PASSWORD="1234" # Openssl derives a key from a password, which is obviously not amazing but :-)

# -pbkdf2 = better key derivation

echo "Hola, mundo" > plaintext.txt
for base64 in "" "-base64"; do
  for mode in CBC CBF ECB OFB; do
    for algorithm in RC5 SEED CAMELLIA-128 AES-128; do
      openssl enc -e -in plaintext.txt -out plaintext.$algorithm-$mode$base64 -$algorithm-$mode -pass pass:$PASSWORD -pbkdf2 $base64
      openssl enc -d -in plaintext.$algorithm-$mode$base64 -out plaintext.$algorithm-$mode$base64.txt -$algorithm-$mode -pass pass:$PASSWORD -pbkdf2 $base64
    done
  done
done

for f in *.txt; do
  cat "$f"
done

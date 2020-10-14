#!/usr/bin/env bash

set -eux
set -o errexit

PASSWORD="1234" # Openssl derives a key from a password, which is obviously not amazing but :-)

# -pbkdf2 = better key derivation

# Get our plaintext that we're going to encrypt.
echo "Hola, mundo" > plaintext.txt

# For each of salted vs. not salted
for salt in "-salt" "-nosalt"; do
  # For each of binary vs. base64
  for base64 in "" "-base64"; do
    # For each block cipher mode.
    for mode in cbc cfb ecb ofb; do
      # For each of some interesting algorithms
      for algorithm in rc5 seed camellia-128 aes-128; do
        # Encrypt it, then decrypt it.
        openssl enc -e \
          -in plaintext.txt \
          -out plaintext.$algorithm-$mode$base64$salt \
          -$algorithm-$mode \
          -pass pass:$PASSWORD \
          -pbkdf2 $base64 $salt

        openssl enc -d \
          -in plaintext.$algorithm-$mode$base64$salt \
          -out plaintext.$algorithm-$mode$base64$salt.txt \
          -$algorithm-$mode \
          -pass pass:$PASSWORD \
          -pbkdf2 $base64 $salt
      done
    done
  done
done

# Output the decrypted text (should make sure it matches the plaintext).
for f in *.txt; do
  cat "$f"
done

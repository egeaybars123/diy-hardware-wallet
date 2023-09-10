extern "C" {
#include <ecdsa.h>
#include <bignum256.h>
}

void setup() {
  Serial.begin(9600);
}

void loop() {

  uint8_t hash[32];

  if (Serial.available()) {
    Serial.readBytes(hash, 32);
    delay(500);
    uint8_t r[32];
    uint8_t s[32];
    uint8_t sig[64];

    uint8_t privBytes[32] = { ENTER_PRIV_KEY_HERE };

    ecdsaSign((BigNum256)r, (BigNum256)s, (BigNum256)hash, (BigNum256)privBytes);

    memcpy(sig, r, 32);
    memcpy(sig + 32, s, 32);

    Serial.write(sig, 64);
  }
}

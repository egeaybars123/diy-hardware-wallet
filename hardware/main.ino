extern "C" {
    #include <ecdsa.h>
    #include <bignum256.h>
}

String privkey;

void printHex(uint8_t num) {
  char hexCar[2];

  sprintf(hexCar, "%02X", num);
  Serial.print(hexCar);
}

//signature byte array

uint8_t r[32];
uint8_t s[64];
int i;

void setup() {
  Serial.begin(9600);
  privkey = String("PRIVATE_KEY_HERE");

  uint8_t privBytes[32];
  uint8_t hash[32] =  {200, 215, 201, 225, 52, 171, 175, 175, 142, 42, 131, 206, 158, 34, 122, 14, 203, 193, 134, 242, 88, 247, 143, 196, 28, 14, 93, 150, 35, 218, 22, 86};
  uint8_t sig[64] = {0};
  
  uint8_t s[64];
  privkey.getBytes(privBytes, 32);
  ecdsaSign((BigNum256)r, (BigNum256)s, (BigNum256)hash, (BigNum256)privBytes);

}

void loop() {
  for(i=0; i<sizeof(r); i++){
    printHex(r[i]);
  }
  Serial.println();

  delay(10000);
}

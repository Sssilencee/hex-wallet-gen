#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum {
    Eth,
    Trx,
    Btc,
    Ltc,
    Sol,
    Apt,
    Sui,
} Network;

extern char* generate_keychain(Network);
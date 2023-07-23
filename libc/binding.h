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

extern void free_c_char(char*);
#include <stdint.h>

/* SHA-512 compress for ATmega328P using only 32-bit ops (no libgcc
 * __rotldi3 / __adddi3, which hung the previous 64-bit-C build).
 * W[] and the working state live in BSS. K halves come from progmem_k.c.
 * Algorithm matches rind's rolled SHA-512 loop. */

typedef struct { uint32_t lo, hi; } u64;

#ifdef SHA512_COMPRESS_HOST
static const uint32_t sha512_k_le[160] = {
    0xd728ae22u, 0x428a2f98u, 0x23ef65cdu, 0x71374491u,
    0xec4d3b2fu, 0xb5c0fbcfu, 0x8189dbbcu, 0xe9b5dba5u,
    0xf348b538u, 0x3956c25bu, 0xb605d019u, 0x59f111f1u,
    0xaf194f9bu, 0x923f82a4u, 0xda6d8118u, 0xab1c5ed5u,
    0xa3030242u, 0xd807aa98u, 0x45706fbeu, 0x12835b01u,
    0x4ee4b28cu, 0x243185beu, 0xd5ffb4e2u, 0x550c7dc3u,
    0xf27b896fu, 0x72be5d74u, 0x3b1696b1u, 0x80deb1feu,
    0x25c71235u, 0x9bdc06a7u, 0xcf692694u, 0xc19bf174u,
    0x9ef14ad2u, 0xe49b69c1u, 0x384f25e3u, 0xefbe4786u,
    0x8b8cd5b5u, 0x0fc19dc6u, 0x77ac9c65u, 0x240ca1ccu,
    0x592b0275u, 0x2de92c6fu, 0x6ea6e483u, 0x4a7484aau,
    0xbd41fbd4u, 0x5cb0a9dcu, 0x831153b5u, 0x76f988dau,
    0xee66dfabu, 0x983e5152u, 0x2db43210u, 0xa831c66du,
    0x98fb213fu, 0xb00327c8u, 0xbeef0ee4u, 0xbf597fc7u,
    0x3da88fc2u, 0xc6e00bf3u, 0x930aa725u, 0xd5a79147u,
    0xe003826fu, 0x06ca6351u, 0x0a0e6e70u, 0x14292967u,
    0x46d22ffcu, 0x27b70a85u, 0x5c26c926u, 0x2e1b2138u,
    0x5ac42aedu, 0x4d2c6dfcu, 0x9d95b3dfu, 0x53380d13u,
    0x8baf63deu, 0x650a7354u, 0x3c77b2a8u, 0x766a0abbu,
    0x47edaee6u, 0x81c2c92eu, 0x1482353bu, 0x92722c85u,
    0x4cf10364u, 0xa2bfe8a1u, 0xbc423001u, 0xa81a664bu,
    0xd0f89791u, 0xc24b8b70u, 0x0654be30u, 0xc76c51a3u,
    0xd6ef5218u, 0xd192e819u, 0x5565a910u, 0xd6990624u,
    0x5771202au, 0xf40e3585u, 0x32bbd1b8u, 0x106aa070u,
    0xb8d2d0c8u, 0x19a4c116u, 0x5141ab53u, 0x1e376c08u,
    0xdf8eeb99u, 0x2748774cu, 0xe19b48a8u, 0x34b0bcb5u,
    0xc5c95a63u, 0x391c0cb3u, 0xe3418acbu, 0x4ed8aa4au,
    0x7763e373u, 0x5b9cca4fu, 0xd6b2b8a3u, 0x682e6ff3u,
    0x5defb2fcu, 0x748f82eeu, 0x43172f60u, 0x78a5636fu,
    0xa1f0ab72u, 0x84c87814u, 0x1a6439ecu, 0x8cc70208u,
    0x23631e28u, 0x90befffau, 0xde82bde9u, 0xa4506cebu,
    0xb2c67915u, 0xbef9a3f7u, 0xe372532bu, 0xc67178f2u,
    0xea26619cu, 0xca273eceu, 0x21c0c207u, 0xd186b8c7u,
    0xcde0eb1eu, 0xeada7dd6u, 0xee6ed178u, 0xf57d4f7fu,
    0x72176fbau, 0x06f067aau, 0xa2c898a6u, 0x0a637dc5u,
    0xbef90daeu, 0x113f9804u, 0x131c471bu, 0x1b710b35u,
    0x23047d84u, 0x28db77f5u, 0x40c72493u, 0x32caab7bu,
    0x15c9bebcu, 0x3c9ebe0au, 0x9c100d4cu, 0x431d67c4u,
    0xcb3e42b6u, 0x4cc5d4beu, 0xfc657e2au, 0x597f299cu,
    0x3ad6faecu, 0x5fcb6fabu, 0x4a475817u, 0x6c44198cu,
};
static void sha512_k_load_halves(uint8_t i, uint32_t *lo, uint32_t *hi) {
    *lo = sha512_k_le[(unsigned)i * 2u];
    *hi = sha512_k_le[(unsigned)i * 2u + 1u];
}
#else
void sha512_k_load_halves(uint8_t i, uint32_t *lo, uint32_t *hi);
#endif

static u64 add64(u64 a, u64 b) {
    u64 r;
    r.lo = a.lo + b.lo;
    r.hi = a.hi + b.hi + (r.lo < a.lo);
    return r;
}

static u64 xor64(u64 a, u64 b) {
    u64 r;
    r.lo = a.lo ^ b.lo;
    r.hi = a.hi ^ b.hi;
    return r;
}

static u64 and64(u64 a, u64 b) {
    u64 r;
    r.lo = a.lo & b.lo;
    r.hi = a.hi & b.hi;
    return r;
}

static u64 not64(u64 a) {
    u64 r;
    r.lo = ~a.lo;
    r.hi = ~a.hi;
    return r;
}

static u64 rotr64(u64 x, uint8_t n) {
    u64 r;
    uint8_t m;
    if (n == 0) {
        return x;
    }
    if (n < 32) {
        m = 32 - n;
        r.lo = (x.lo >> n) | (x.hi << m);
        r.hi = (x.hi >> n) | (x.lo << m);
    } else if (n == 32) {
        r.lo = x.hi;
        r.hi = x.lo;
    } else {
        n = (uint8_t)(n - 32);
        m = 32 - n;
        r.lo = (x.hi >> n) | (x.lo << m);
        r.hi = (x.lo >> n) | (x.hi << m);
    }
    return r;
}

static u64 shr64(u64 x, uint8_t n) {
    u64 r;
    uint8_t m;
    if (n == 0) {
        return x;
    }
    if (n < 32) {
        m = 32 - n;
        r.lo = (x.lo >> n) | (x.hi << m);
        r.hi = x.hi >> n;
    } else if (n == 32) {
        r.lo = x.hi;
        r.hi = 0;
    } else {
        n = (uint8_t)(n - 32);
        r.lo = x.hi >> n;
        r.hi = 0;
    }
    return r;
}

static u64 ch(u64 x, u64 y, u64 z) {
    return xor64(and64(x, y), and64(not64(x), z));
}

static u64 maj(u64 x, u64 y, u64 z) {
    return xor64(xor64(and64(x, y), and64(x, z)), and64(y, z));
}

static u64 bsig0(u64 x) { return xor64(xor64(rotr64(x, 28), rotr64(x, 34)), rotr64(x, 39)); }
static u64 bsig1(u64 x) { return xor64(xor64(rotr64(x, 14), rotr64(x, 18)), rotr64(x, 41)); }
static u64 ssig0(u64 x) { return xor64(xor64(rotr64(x, 1), rotr64(x, 8)), shr64(x, 7)); }
static u64 ssig1(u64 x) { return xor64(xor64(rotr64(x, 19), rotr64(x, 61)), shr64(x, 6)); }

static u64 load_be64(const uint8_t *p) {
    u64 r;
    r.hi = ((uint32_t)p[0] << 24) | ((uint32_t)p[1] << 16) | ((uint32_t)p[2] << 8) | (uint32_t)p[3];
    r.lo = ((uint32_t)p[4] << 24) | ((uint32_t)p[5] << 16) | ((uint32_t)p[6] << 8) | (uint32_t)p[7];
    return r;
}

static u64 k_at(uint8_t t) {
    u64 r;
    sha512_k_load_halves(t, &r.lo, &r.hi);
    return r;
}

void sha512_compress_block_c(uint32_t *state, const uint8_t *block) {
    static u64 w[16];
    static u64 s[8];
    uint8_t t;
    for (t = 0; t < 16; t++) {
        w[t] = load_be64(block + (uint16_t)t * 8u);
    }
    for (t = 0; t < 8; t++) {
        s[t].lo = state[(uint8_t)(t * 2u)];
        s[t].hi = state[(uint8_t)(t * 2u + 1u)];
    }
    for (t = 0; t < 80; t++) {
        u64 wt, t1, t2;
        u64 a = s[0], b = s[1], c = s[2], d = s[3];
        u64 e = s[4], f = s[5], g = s[6], h = s[7];
        if (t < 16) {
            wt = w[t];
        } else {
            wt = add64(add64(ssig1(w[(t - 2) & 15]), w[(t - 7) & 15]),
                       add64(ssig0(w[(t - 15) & 15]), w[t & 15]));
            w[t & 15] = wt;
        }
        t1 = add64(add64(add64(add64(h, bsig1(e)), ch(e, f, g)), k_at(t)), wt);
        t2 = add64(bsig0(a), maj(a, b, c));
        s[0] = add64(t1, t2);
        s[1] = a;
        s[2] = b;
        s[3] = c;
        s[4] = add64(d, t1);
        s[5] = e;
        s[6] = f;
        s[7] = g;
    }
    for (t = 0; t < 8; t++) {
        u64 st;
        st.lo = state[(uint8_t)(t * 2u)];
        st.hi = state[(uint8_t)(t * 2u + 1u)];
        st = add64(st, s[t]);
        state[(uint8_t)(t * 2u)] = st.lo;
        state[(uint8_t)(t * 2u + 1u)] = st.hi;
    }
}

#ifdef SHA512_COMPRESS_HOST
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

static const u64 SHA512_IV[8] = {
    {0xf3bcc908u, 0x6a09e667u}, {0x84caa73bu, 0xbb67ae85u},
    {0xfe94f82bu, 0x3c6ef372u}, {0x5f1d36f1u, 0xa54ff53au},
    {0xade682d1u, 0x510e527fu}, {0x2b3e6c1fu, 0x9b05688cu},
    {0xfb41bd6bu, 0x1f83d9abu}, {0x137e2179u, 0x5be0cd19u},
};

static void oneshot(const uint8_t *msg, uint32_t len, uint8_t out[64]) {
    uint32_t state[16];
    uint8_t block[256];
    uint32_t i, full, rem;
    uint8_t t;
    for (t = 0; t < 8; t++) {
        state[t * 2] = SHA512_IV[t].lo;
        state[t * 2 + 1] = SHA512_IV[t].hi;
    }
    full = len / 128;
    rem = len % 128;
    for (i = 0; i < full; i++) {
        sha512_compress_block_c(state, msg + i * 128);
    }
    memset(block, 0, sizeof block);
    if (rem) memcpy(block, msg + full * 128, rem);
    block[rem] = 0x80;
    {
        uint64_t bits = (uint64_t)len * 8ull;
        uint8_t *lenp;
        if (rem + 17 > 128) {
            sha512_compress_block_c(state, block);
            memset(block, 0, 128);
        }
        lenp = block + 120;
        for (t = 0; t < 8; t++) lenp[t] = (uint8_t)(bits >> (56 - 8 * t));
        sha512_compress_block_c(state, block);
    }
    for (t = 0; t < 8; t++) {
        uint32_t hi = state[t * 2 + 1], lo = state[t * 2];
        out[t * 8 + 0] = (uint8_t)(hi >> 24);
        out[t * 8 + 1] = (uint8_t)(hi >> 16);
        out[t * 8 + 2] = (uint8_t)(hi >> 8);
        out[t * 8 + 3] = (uint8_t)hi;
        out[t * 8 + 4] = (uint8_t)(lo >> 24);
        out[t * 8 + 5] = (uint8_t)(lo >> 16);
        out[t * 8 + 6] = (uint8_t)(lo >> 8);
        out[t * 8 + 7] = (uint8_t)lo;
    }
}

static int check(const char *name, const uint8_t *msg, uint32_t len, const char *hex) {
    uint8_t out[64];
    char got[129];
    unsigned i;
    oneshot(msg, len, out);
    for (i = 0; i < 64; i++) sprintf(got + 2 * i, "%02x", out[i]);
    got[128] = 0;
    if (strcmp(got, hex) != 0) {
        fprintf(stderr, "FAIL %s\n got %s\n exp %s\n", name, got, hex);
        return 1;
    }
    printf("PASS %s\n", name);
    return 0;
}

int main(void) {
    int f = 0;
    f |= check("abc", (const uint8_t *)"abc", 3,
        "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a"
        "2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f");
    uint8_t pat[1350];
    uint32_t i;
    for (i = 0; i < 1350; i++) pat[i] = (uint8_t)i;
    f |= check("pattern32", pat, 32,
        "3d94eea49c580aef816935762be049559d6d1440dede12e6a125f1841fff8e6f"
        "a9d71862a3e5746b571be3d187b0041046f52ebd850c7cbd5fde8ee38473b649");
    f |= check("pattern1350", pat, 1350,
        "78c7ba319caa4be4fc5b307b044d6c4c34e9a5b6f27c0a04285f49851e9bcda2"
        "237c0c1b2346a4c8d2a6fa574754d6271bdddfc4cf0bef0b87651fff41bd1690");
    return f ? 1 : 0;
}
#endif

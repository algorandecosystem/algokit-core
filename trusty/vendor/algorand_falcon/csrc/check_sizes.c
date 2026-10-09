/* Compile-time checks that the constants hard-coded in src/det1024.rs and
 * src/ffi.rs match the C headers. */
#include "deterministic.h"

_Static_assert(FALCON_DET1024_LOGN == 10, "LOGN");
_Static_assert(FALCON_DET1024_PUBKEY_SIZE == 1793, "PUBLIC_KEY_SIZE");
_Static_assert(FALCON_DET1024_PRIVKEY_SIZE == 2305, "PRIVATE_KEY_SIZE");
_Static_assert(FALCON_DET1024_SIG_COMPRESSED_MAXSIZE == 1423, "SIGNATURE_MAX_SIZE");
_Static_assert(FALCON_DET1024_SIG_CT_SIZE == 1538, "CT_SIGNATURE_SIZE");
_Static_assert(FALCON_DET1024_CURRENT_SALT_VERSION == 0, "CURRENT_SALT_VERSION");
_Static_assert(sizeof(shake256_context) == 26 * sizeof(uint64_t), "Shake256Context");

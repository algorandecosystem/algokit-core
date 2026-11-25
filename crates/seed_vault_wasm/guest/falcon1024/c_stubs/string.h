/* Minimal libc header for building Falcon's C for wasm32-unknown-unknown,
 * which has no libc. Rust's compiler_builtins provides these symbols. */
#ifndef SEED_VAULT_WASM_STRING_H
#define SEED_VAULT_WASM_STRING_H
#include <stddef.h>
void *memcpy(void *dst, const void *src, size_t n);
void *memmove(void *dst, const void *src, size_t n);
void *memset(void *dst, int c, size_t n);
int memcmp(const void *a, const void *b, size_t n);
#endif

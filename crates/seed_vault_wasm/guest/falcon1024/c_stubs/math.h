/* Minimal libc header for wasm32: Falcon only needs sqrt and floor, which
 * are single WebAssembly instructions (f64.sqrt, f64.floor). */
#ifndef SEED_VAULT_WASM_MATH_H
#define SEED_VAULT_WASM_MATH_H
static inline double sqrt(double x) { return __builtin_sqrt(x); }
static inline double floor(double x) { return __builtin_floor(x); }
#endif

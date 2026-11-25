# C half of the hand-vendored `algorand-falcon` crate: the deterministic
# Falcon-1024 reference implementation, compiled with the same defines as
# upstream's `build.rs`. See `../rules.mk`.

LOCAL_DIR := $(GET_LOCAL_DIR)
FALCON_DIR := $(LOCAL_DIR)/../falcon
MODULE := $(LOCAL_DIR)

MODULE_SRCS := \
	$(FALCON_DIR)/codec.c \
	$(FALCON_DIR)/common.c \
	$(FALCON_DIR)/deterministic1024.c \
	$(FALCON_DIR)/falcon.c \
	$(FALCON_DIR)/fft.c \
	$(FALCON_DIR)/fpr.c \
	$(FALCON_DIR)/keygen.c \
	$(FALCON_DIR)/rng.c \
	$(FALCON_DIR)/shake.c \
	$(FALCON_DIR)/sign.c \
	$(FALCON_DIR)/vrfy.c \
	$(LOCAL_DIR)/../csrc/check_sizes.c

MODULE_INCLUDES := $(FALCON_DIR)

# Same as upstream build.rs: every key is seeded from caller-provided bytes,
# so disable all OS entropy sources (getentropy, /dev/urandom, CryptGenRandom)
# -- none exist in Trusty anyway.
#
# FALCON_FPEMU=1: use Falcon's integer-only IEEE-754 emulation instead of the
# hardware FPU. Results are bit-identical to native FP (required for
# deterministic signing), it's constant-time on every core, and it avoids
# relying on FPU/libm state inside the TEE.
MODULE_CFLAGS += \
	-DFALCON_RAND_GETENTROPY=0 \
	-DFALCON_RAND_URANDOM=0 \
	-DFALCON_RAND_WIN32=0 \
	-DFALCON_FPEMU=1 \
	-DFALCON_FPNATIVE=0 \
	-fomit-frame-pointer \
	-O3

include make/library.mk


#ifndef _KHASH_H
#define _KHASH_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

#ifdef __GNUC__
#define _deprecated(x) __attribute__((deprecated))
#else
#define _deprecated(x)
#endif

#define KHASH_ALGO_DEFAULT ((uint8_t)0)
#define KHASH_ALGO_CRC32 ((uint8_t)1)
#define KHASH_ALGO_CRC64 ((uint8_t)2)
#define KHASH_ALGO_SHA256 ((uint8_t)3)
#define KHASH_ALGO_SHA256_TRUNCATED ((uint8_t)4) /* SHA256 truncated to 64 bits */
  
  /// No salt
#define KHASH_SALT_TYPE_NONE ((uint8_t)0)
  /// The default static salt
#define KHASH_SALT_TYPE_DEFAULT ((uint8_t)1)
  /// A specific salt passed as `data` to `khash_new_salt`.
#define KHASH_SALT_TYPE_SPECIFIC ((uint8_t)2)
  /// A randomly generated salt.
#define KHASH_SALT_TYPE_RANDOM ((uint8_t)3)

  /// A valid salt for khash functions. Instantiated with `khash_new_salt`.
  typedef struct {
    uint8_t salt_type;
    uint32_t size;
    uint8_t* body;
  } khash_salt;

  /// A valid context for khash functinos. Instantiated with `khash_new_context`.
  typedef struct {
    uint8_t algo;
    uint64_t flags;
    khash_salt salt;
  } khash_ctx;

  /// Returned by all functions that succeed.
#define KHASH_SUCCESS ((int32_t)0)

  /// Reading into the hash failed
#define KHASH_ERROR_IO ((int32_t)1)
  /// Formatting the has failed
#define KHASH_ERROR_FORMAT ((int32_t)2)
  /// Bad hash length
#define KHASH_ERROR_LENGTH ((int32_t)3)
  /// Random number generation failed
#define KHASH_ERROR_RNG ((int32_t)4)
  /// Unknown error
#define KHASH_ERROR_UNKNOWN ((int32_t)-1)

  /// Create a new salt. `salt_type` is expected to be one of the above defined `KHASH_SALT_TYPE_*` macros.
  /// Depending on the type, `data` may be `NULL`.
  extern int32_t khash_new_salt(uint8_t salt_type, const void* data, size_t size, khash_salt* output) _deprecated("Use `khash_new_context` instead."); 
  /// Free a salt allocated with `khash_new_salt`. It is okay to call this multiple times.
  extern int32_t khash_free_salt(khash_salt* salt) _deprecated("Use `khash_free_context` instead.");
  /// Clone a salt allocated with `khash_new_salt`.
  extern int32_t khash_clone_salt(const khash_salt* src, khash_salt* dst) _deprecated("Use `khash_close_context` instead."); 

  /// Create a new context with the specified algorithm (one of the `KHASH_ALGO_*` macro constants), salt type (one of the `KHASH_SALT_TYPE_*` constants), optional salt `data` and salt length `size`, and output pointer `output`.
  /// `data` may be `NULL` if the corresponding `salt_type` does not require an input.
  extern int32_t khash_new_context(uint8_t algo, uint8_t salt_type, const void* data, size_t size, khash_ctx* output);
  /// Free a `khash_ctx` allocated with `khash_new_context`.
  extern int32_t khash_free_context(khash_ctx* ctx);
  /// Clone a `khash_ctx` allocated with `khash_new_context`. The clone is a newly allocated instance.
  extern int32_t khash_clone_context(const khash_ctx* src, khash_ctx* dst);
  
  /// Compute the length of hash required for the specified input.
  /// This function does not free `context` after it has been called.
  extern int32_t khash_length(const khash_ctx* context, const void* data, size_t size, size_t* length);
  /// Compute the hash and store it in `string`. Will write no more than `strlen` bytes into `string`.
  /// This function takes ownership of and frees `context` after it has been called.
  extern int32_t khash_do(khash_ctx* context, const void* data, size_t size, char* string, size_t strlen);

#ifdef __cplusplus
}
#endif

#endif /* _KHASH_H */

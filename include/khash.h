#ifndef _KHASH_H
#define _KHASH_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

  /// No salt
#define KHASH_SALT_TYPE_NONE ((uint8_t)0)
  /// The default static salt
#define KHASH_SALT_TYPE_DEFAULT ((uint8_t)1)
  /// A specific salt passed as `data` to `khash_new_salt`.
#define KHASH_SALT_TYPE_SPECIFIC ((uint8_t)2)
  /// A randomly generated salt.
#define KHASH_SALT_TYPE_RANDOM ((uint8_t)3)

  /// A valid salt for khash functions. Initialised with `khash_new_salt`.
  typedef struct {
    uint8_t salt_type;
    uint32_t size;
    uint8_t* body;
  } khash_salt;

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
  extern int32_t khash_new_salt(uint8_t salt_type, const void* data, size_t size, khash_salt* output);
  /// Free a salt allocated with `khash_new_salt`. It is okay to call this multiple times.
  extern int32_t khash_free_salt(khash_salt* salt);
  /// Compute the length of hash required for the specified input.
  extern int32_t khash_length(const void* data, size_t size, const khash_salt* salt, size_t* length);
  /// Compute the hash and store it in `string`. Will write no more than `strlen` bytes into `string`.
  extern int32_t khash_do(const void* data, size_t size, khash_salt* salt, char* string, size_t strlen);

#ifdef __cplusplus
}
#endif

#endif /* _KHASH_H */

#include <stdio.h>
#include <khash.h>
#include <assert.h>
#include <locale.h>
#include <alloca.h>
#include <string.h>

int main(void)
{
  setlocale(LC_ALL, "");
  const char* string = "hello world!";

  printf("input: %s\n", string);
  khash_salt salt;
  assert(khash_new_salt(KHASH_SALT_TYPE_RANDOM, NULL, 0, &salt) == KHASH_SUCCESS);
  printf("salt: %d\n", (int)salt.size);
  size_t length;
  assert(khash_length(string, strlen(string), &salt, &length) == KHASH_SUCCESS);
  printf("length: %d\n", (int)length);
  char* output = alloca(length+1);
  assert(khash_do(string, strlen(string), &salt, output,length) == KHASH_SUCCESS);

  printf("output: %s\n", output);
  return 0;
}

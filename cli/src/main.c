#include <stdio.h>
#include <assert.h>
#include <locale.h>
#include <alloca.h>
#include <string.h>

#include <khash.h>

#ifdef TEST
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
#else

#define KTRY(expr, msg) (assert(((expr) && (msg))== KHASH_SUCCESS))

void k_do(const char* input, const khash_salt* salt)
{
  khash_salt clone;
  KTRY(khash_clone_salt(salt, &clone), "khash: saltclone failed");

  size_t length;
  KTRY(khash_length(input, strlen(input), &clone, &length), "khash: hashlength failed");

  char* output =alloca(length+1);
  KTRY(khash_do(input,strlen(input), &clone, output, length), "khash: hashstring failed");
  output[length] = 0; //ensure no overflow.
  printf("%s\n", output);
}

void reseed_salt(khash_salt* salt, uint8_t type, const void* in_ptr, size_t ptr_sz)
{
  KTRY(khash_free_salt(salt), "khash: saltrefree failed");
  KTRY(khash_new_salt(type, in_ptr, ptr_sz, salt), "khash: saltreseed failed");
}

static int _main(int argc, char** argv, khash_salt salt)
{
  int look = 1;
  if (argc <= 1)
    {
      printf("try `%s --help`\n", argv[0]);
      return 1;
    }
  
  for(argv++;*argv;argv++)
    {
      if (!look) goto work;
      if (strcmp(*argv, "--help") == 0)
	{
	  printf("kana-hash cli\n");
	  printf("Usage: khash [--salt SALT-TYPE [<salt>]] [--] <input strings...>\n");
	  printf("  --salt: Specify the salt.\n");
	  printf("    SALT_TYPE: D: default embedded.\n");
	  printf("             : N: no salt.\n");
	  printf("             : S <salt>: specific salt.\n");
	  printf("  --: Stop reading args here.\n");
	  return 1;
	}
      else if (strcmp(*argv, "--") == 0)
	look = 0;
      else if (strcmp(*argv, "--salt")==0)
	{
	  if (argv[1])
	    {
	      switch (argv[1][0])
		{
		case 'd':
		case 'D':
		  reseed_salt(&salt, KHASH_SALT_TYPE_DEFAULT, NULL, 0);
		  break;
		case 'N':
		case 'n':
		  reseed_salt(&salt, KHASH_SALT_TYPE_NONE, NULL, 0);
		  break;
		case 'S':
		case 's':
		  if(argv[2])
		    reseed_salt(&salt, KHASH_SALT_TYPE_SPECIFIC, argv[2], strlen(argv[2]));
		  else {
		    fprintf(stderr, "SALT_TYPE `%c' expects a value.\n", *argv[1]);
		    return 1;
		  }
		  argv++;
		  break;
		default:
		  fprintf(stderr, "Unknown SALT_TYPE `%c'\n", *argv[1]);
		  return 1;
		}
	      argv++;
	    }
	  else {
	    fprintf(stderr, "--salt expects at least SALT_TYPE.\n");
	    return 1;
	  }
	}
      else {
      work:
	k_do(*argv, &salt);
      }
    }
  
  return 0;
}

int main(int argc, char** argv)
{
  
  khash_salt salt;
  KTRY(khash_new_salt(KHASH_SALT_TYPE_DEFAULT, NULL, 0, &salt), "khash: saltgen failed");

  int res = _main(argc, argv, salt);

  KTRY(khash_free_salt(&salt), "khash: saltfree failed");

  return res;
}
#endif

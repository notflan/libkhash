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
  khash_ctx ctx;
  assert(khash_new_context(KHASH_ALGO_SHA256, KHASH_SALT_TYPE_NONE, NULL, 0, &ctx) == KHASH_SUCCESS);
  printf("salt: %d\n", (int)ctx.salt.size);
  size_t length;
  assert(khash_length(&ctx, string, strlen(string), &length) == KHASH_SUCCESS);
  printf("length: %d\n", (int)length);
  char* output = alloca(length+1);
  assert(khash_do(&ctx, string, strlen(string), output,length) == KHASH_SUCCESS);
  output[length] = 0;
  printf("output: %s\n", output);
  return 0;
}
#else

#define KTRY(expr, msg) (assert(((expr) && (msg))== KHASH_SUCCESS))

void k_do(const khash_ctx* ctx, const char* input)
{
  khash_ctx clone;
  KTRY(khash_clone_context(ctx, &clone), "khash: ctxclone failed");

  size_t length;
  KTRY(khash_length(&clone, input, strlen(input), &length), "khash: hashlength failed");

  char* output =alloca(length+1);
  KTRY(khash_do(&clone, input,strlen(input), output, length), "khash: hashstring failed");
  output[length] = 0; //ensure no overflow.
  printf("%s\n", output);
}

void reseed_ctx(khash_ctx* ctx, uint8_t algo, uint8_t type, const void* in_ptr, size_t ptr_sz)
{
  KTRY(khash_free_context(ctx), "khash: ctxrefree failed");
  KTRY(khash_new_context(algo, type, in_ptr, ptr_sz, ctx), "khash: ctxreseed failed");
}

static int _main(int argc, char** argv, khash_ctx ctx)
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
	  printf("Usage: khash [--algo ALGO] [--salt SALT-TYPE [<salt>]] [--] <input strings...>\n");
	  printf("  --algo: Specify the algorithm. (default crc64)\n");
	  printf("    ALGO: 3: crc32.\n");
	  printf("    ALGO: 6: crc64.\n");
	  printf("    ALGO: s: sha256.\n");
	  printf("  --salt: Specify the salt.\n");
	  printf("    SALT_TYPE: D: default embedded.\n");
	  printf("             : N: no salt.\n");
	  printf("             : R: random salt.\n");
	  printf("             : S <salt>: specific salt.\n");
	  printf("  --: Stop reading args here.\n");
	  return 1;
	}
      else if (strcmp(*argv, "--") == 0)
	look = 0;
      else if (strcmp(*argv, "--algo")==0)
	{
	  if (argv[1])
	    {
	      switch(argv[1][0])
		{
		case '3':
		  ctx.algo = KHASH_ALGO_CRC32;
		  break;
		case '6':
		  ctx.algo = KHASH_ALGO_CRC64;
		  break;
		case 's':
		  ctx.algo = KHASH_ALGO_SHA256;
		  break;
		default:
		  fprintf(stderr, "ALGO: unknow algorithm key `%c'\n", *argv[1]);
		  return 1;
		}
	    }
	  argv++;
	}
      else if (strcmp(*argv, "--salt")==0)
	{
	  if (argv[1])
	    {
	      switch (argv[1][0])
		{
		case 'd':
		case 'D':
		  reseed_ctx(&ctx, ctx.algo, KHASH_SALT_TYPE_DEFAULT, NULL, 0);
		  break;
		case 'N':
		case 'n':
		  reseed_ctx(&ctx, ctx.algo, KHASH_SALT_TYPE_NONE, NULL, 0);
		  break;
		case 'S':
		case 's':
		  if(argv[2])
		    reseed_ctx(&ctx, ctx.algo, KHASH_SALT_TYPE_SPECIFIC, argv[2], strlen(argv[2]));
		  else {
		    fprintf(stderr, "SALT_TYPE `%c' expects a value.\n", *argv[1]);
		    return 1;
		  }
		  argv++;
		  break;
		case 'R':
		case 'r':
		  reseed_ctx(&ctx, ctx.algo, KHASH_SALT_TYPE_RANDOM, NULL, 0);
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
	k_do(&ctx, *argv);
      }
    }
  
  return 0;
}

int main(int argc, char** argv)
{
  setlocale(LC_ALL, "");

  khash_ctx context;
  KTRY(khash_new_context(KHASH_ALGO_DEFAULT, KHASH_SALT_TYPE_DEFAULT, NULL, 0, &context), "khash: ctxgen failed");

  int res = _main(argc, argv, context);

  KTRY(khash_free_context(&context), "khash: ctxfree failed");

  return res;
}
#endif

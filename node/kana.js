const ffi = require('ffi');
const struct = require('ref-struct');
const ref = require('ref');

const Salt = struct({
    'type': 'char',
    'size': 'int',
    'body': 'pointer',
});
const PSalt = ref.refType(Salt);

const Context = struct({
    'algo': 'char',
    'salt': Salt,
});
const PContext = ref.refType(Context);
const PLong = ref.refType(ref.types.long);

const lib = ffi.Library('libkhash', {
    'khash_new_context': ['int', ['char', 'char', 'string', 'long', PContext]],
    'khash_free_context': ['int', [PContext]],
    'khash_clone_context': ['int', [PContext, PContext]],

    'khash_length': ['int', [PContext, 'string', 'long', PLong]],
    'khash_do': ['int', [PContext, 'string', 'long', 'string', 'long']],
});

const ctx_create = (algo, salt, salt_ref, salt_sz) => {
    let ctx_ptr = ref.alloc(Context);
    lib.khash_new_context(algo, salt, salt_ref, salt_sz, ctx_ptr);
    return ctx_ptr;
};
const ctx_free = (ptr) => {
    lib.khash_free_context(ptr);
    return ptr.deref();
};
const ctx_clone = (src) => {
    let dst = ref.alloc(Context);
    lib.khash_clone_context(src,dst);
    return dst;
};
const khash_length = (ctx, jsstring) => {
    let string = ref.allocCString(jsstring);
    let len = ref.alloc('long');
    lib.khash_length(ctx, string, string.length, len);
    return len.deref();
};
const khash_do = (ctx, jsstring, len) => {
    let string = ref.allocCString(jsstring);
    let buffer = Buffer.alloc(len+1);
    lib.khash_do(ctx,string,string.length,buffer,len);
    return ref.readCString(buffer,0);
};

const get_salt_type = (salt) => {
    if (salt && salt.tag) {
	switch(salt.tag)
	{
	    case '__NONE':
	    return Kana.SALT_NONE;
	    case '__RANDOM':
	    return Kana.SALT_RANDOM;
	    case '__SPECIFIC':
	    return Kana.SALT_SPECIFIC;
	    default:
	    return Kana.SALT_DEFAULT;
	}
    }
    else return Kana.SALT_DEFAULT;
};

/// Create a new kana-hash context.
/// This context must be disposed of correctly either by calling `finish()` or `once()`.
/// `algo` is expected to be one of the `Kana.ALGO_*` constants, or `undefined`.
/// `salt` is expected to be either an object of `Salt` or `undefined`.
function Kana(algo, salt)
{
    if(algo && algo.ctx) {
	this.ctx = algo.ctx;
    } else {
	const stype = get_salt_type(salt);
	const fbuffer = salt ? salt.buffer || null : null;
	this.ctx = ctx_create(algo || 0, stype, fbuffer, fbuffer ? fbuffer.length : 0);
    }
}
const K = Kana.prototype;

/// Free the associated context.
/// The object is no longer valid after this call.
K.finish = function() {
    ctx_free(this.ctx);
    this.ctx = null;
};

/// Compute the kana-hash for `string` and then free the associated context.
K.once = function(string) {
    let len = khash_length(this.ctx, string);
    return khash_do(this.ctx, string, len);
};

/// Compute the kana-hash for `string`.
K.hash = function(string) {
    const ctx = ctx_clone(this.ctx);
    let len = khash_length(ctx, string);
    return khash_do(ctx, string, len);
};

/// Clone this kana-hash context.
K.clone = function() {
    const ctx = ctx_clone(this.ctx);
    return new Kana({ctx: ctx});
};

/// The default algorithm used. (sha256 truncated.)
Kana.ALGO_DEFAULT = 0;
/// CRC32 algorithm.
Kana.ALGO_CRC32 = 1;
/// CRC64 algorithm.
Kana.ALGO_CRC64 = 2;
/// SHA256 algorithm.
Kana.ALGO_SHA256 = 3;
/// SHA256 truncated to 64-bits.
Kana.ALGO_SHA256_TRUNCATED = 4;

// You don't need to reference these directly, use the `Salt` module instead.
Kana.SALT_NONE = 0;
Kana.SALT_DEFAULT = 1;
Kana.SALT_SPECIFIC = 2;
Kana.SALT_RANDOM = 3;

module.exports = Kana;

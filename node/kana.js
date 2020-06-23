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

function Kana(algo, salt)
{
    const stype = get_salt_type(salt);
    const fbuffer = salt ? salt.buffer || null : null;
    this.ctx = ctx_create(algo || 0, stype, fbuffer, fbuffer ? fbuffer.length : 0);
}
const K = Kana.prototype;

K.finish = function() {
    ctx_free(this.ctx);
};

K.once = function(string) {
    let len = khash_length(this.ctx, string);
    return khash_do(this.ctx, string, len);
};

Kana.ALGO_DEFAULT = 0;
Kana.ALGO_CRC32 = 1;
Kana.ALGO_CRC64 = 2;
Kana.ALGO_SHA256 = 3;

Kana.SALT_NONE = 0;
Kana.SALT_DEFAULT = 1;
Kana.SALT_SPECIFIC = 2;
Kana.SALT_RANDOM = 3;


module.exports = Kana;
/*
  const Kana = require('./kana');

  let buffer= ref.alloc('long');
  let sz = ref.alloc('long');
  console.log(libhkana._kana_length(buffer, 2, null, sz));
  console.log(sz.deref());

  let output = new Buffer(sz);
  console.log(libhkana._kana_do(buffer, 2, null, output, sz));
  console.log(ref.readCString(output, 0));*/

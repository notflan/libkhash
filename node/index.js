const ffi = require('ffi');
const struct = require('ref-struct');
const ref = require('ref');


const libhkana = ffi.Library('libkana_hash', {
    '_kana_new_salt': ['int', ['pointer', 'long', 'pointer']],
    '_kana_free_salt': ['int', ['pointer']],
    '_kana_do': ['int', ['pointer', 'long', 'pointer', 'pointer', 'long']],
    '_kana_length': ['int', ['pointer', 'long', 'pointer', 'pointer']],
});

const Kana = require('./kana');

let buffer= ref.alloc('long');
let sz = ref.alloc('long');
console.log(libhkana._kana_length(buffer, 2, null, sz));
console.log(sz.deref());

let output = new Buffer(sz);
console.log(libhkana._kana_do(buffer, 2, null, output, sz));
console.log(ref.readCString(output, 0));

const ffi = require('ffi');
const struct = require('ref-struct');
const ref = require('ref');

const Salt = module.exports.Salt = struct({
    'size': 'pointer',
    'body': 'pointer',
});
const PSalt = module.exports.PSalt = ref.refType(Salt);


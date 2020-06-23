const ffi = require('ffi');
const struct = require('ref-struct');
const ref = require('ref');

const Kana = require('./kana');
const Salt = require('./salt');

//console.log(new Kana(Kana.CRC64, new Salt("hello lolis")).once("hello worldノチそぬとね"));

module.exports.Kana = Kana;
module.exports.Salt = Salt;

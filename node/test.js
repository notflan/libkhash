const kana = require('./kana');

console.log(new kana(null, null).once("hello loli"));
console.log(kana.single(kana.ALGO_DEFAULT, null, "hello loli"));


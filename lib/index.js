var addon = require('../native');
let isScalar = addon.isScalar;
let setupRandom = addon.setupRandom;
let randomScalar = addon.randomScalar;

module.exports = addon;
function Scalar() { }
function Crypto() { }
function Key() { }

Scalar.setupRandom = (value) => {
  return setupRandom(value);
}

Scalar.checkBuffer = (scalar) => {
  return isScalar(scalar);
}

Scalar.checkHexString = (scalar) => {
  return isScalar(Buffer.from(scalar, 'hex'));
}

Scalar.random = () => {
  return Buffer.from(randomScalar());
}

Scalar.toHash = (scalar) => {
  return Buffer.from(addon.hashToScalar(scalar));
}

module.exports.Scalar = Scalar;
module.exports.Crypto = Crypto;
module.exports.Key = Key;
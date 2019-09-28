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
};

Scalar.checkBuffer = (scalar) => {
  return isScalar(scalar);
};

Scalar.checkHexString = (scalar) => {
  return isScalar(Buffer.from(scalar, 'hex'));
};

Scalar.random = () => {
  return Buffer.from(randomScalar());
};

Scalar.toHash = (scalar) => {
  return Buffer.from(addon.hashToScalar(scalar));
};

Key.generateKeys = () => {
  const keys = Buffer.from(addon.generateKeys());
  const publicKey = Buffer.alloc(32);
  const secretKey = Buffer.alloc(32);
  keys.copy(publicKey, 0, 0, 32);
  keys.copy(secretKey, 0, 32, 64);
  return {
    public: publicKey,
    secret: secretKey
  };
};

Key.check = (hash) => {
  return addon.checkKey(hash);
}

Key.secretToPublic = (secretKey) => {
  return Buffer.from(addon.secretKeyToPublicKey(secretKey));
}

Key.derivate = (publicKey, secretKey) => {
  return Buffer.from(addon.generateKeyDerivation(publicKey, secretKey));
}
Key.derivePublicKey = (derivation, publicKey, index) => {
  return Buffer.from(addon.derivePublicKey(derivation, publicKey, index));
}

module.exports.Scalar = Scalar;
module.exports.Crypto = Crypto;
module.exports.Key = Key;
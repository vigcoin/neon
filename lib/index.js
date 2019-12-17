var addon = require('../native');
let isScalar = addon.isScalar;
let setupRandom = addon.setupRandom;
let randomScalar = addon.randomScalar;

// module.exports = addon;

module.exports.Wallet = addon.Wallet;
module.exports.Difficulty = addon.Difficulty;

function Scalar() { }
function Key() { }
function Signature() { }
function Hash() { }
function Amount() { }

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

Scalar.fromHash = (hash) => {
  return Buffer.from(addon.hashToEC(hash));
};

Scalar.toPoint = (hash) => {
  return Buffer.from(addon.hashToPoint(hash));
}

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

Key.deriveSecretKey = (derivation, secretKey, index) => {
  return Buffer.from(addon.deriveSecretKey(derivation, secretKey, index));
}

Key.underivePublicKey = (derivation, publicKey, index) => {
  return Buffer.from(addon.underivePublicKey(derivation, publicKey, index));
}

Key.generateImage = (publicKey, secretKey) => {
  return Buffer.from(addon.generateKeyImage(publicKey, secretKey));
}

Signature.generate = (prefixHash, publickKey, secretKey) => {
  return Buffer.from(addon.generateSignature(prefixHash, publickKey, secretKey));
}

Signature.check = (prefixHash, publickKey, signature) => {
  return addon.checkSignature(prefixHash, publickKey, signature);
}

Signature.generateRing = (
  prefixHash, image, pubsv, pubsCount, secretKey, secretKeyIndex
) => {
  return Buffer.from(addon.generateRingSignature(
    prefixHash, image, pubsv, pubsCount, secretKey, secretKeyIndex
  ));
}

Signature.checkRing = (
  prefixHash, image, pubsv, pubsCount, signatures
) => {
  return addon.checkRingSignature(
    prefixHash, image, pubsv, pubsCount, signatures
  );
}

Hash.fast = (buffer) => {
  return addon.getFastHash(buffer);
}

Hash.slow = (buffer, variant) => {
  return addon.getSlowHash(buffer, variant);
}

Hash.check = (buffer, difficulty) => {
  return addon.checkHash(buffer, difficulty);
}

Hash.from = (buffer) => {
  return Buffer.from(Hash.fast(buffer), "hex");
};

Amount.getPenalized = (amount, medianSize, currentBlockSize) => {
  return addon.getPenalized(amount, medianSize, currentBlockSize);
}

module.exports.Scalar = Scalar;
module.exports.Key = Key;
module.exports.Signature = Signature;
module.exports.Hash = Hash;
module.exports.Amount = Amount;

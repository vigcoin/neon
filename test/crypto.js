let vigcoin = require("../lib");
let path = require("path");
const lineByLine = require('n-readlines');
let assert = require("assert");
let Scalar = vigcoin.Scalar;
let Key = vigcoin.Key;

describe("Test Crypto", () => {
  it("should check crypto", function () {
    const file = path.resolve(__dirname, "./tests.txt");
    const liner = new lineByLine(file);
    let line;
    let last;
    let executed = false;
    while (line = liner.next()) {
      const divs = String(line).split(" ");
      const command = divs[0];
      if (last != command) {
        console.log(command);
        last = command;
      }
      switch (command) {
        case 'check_scalar':
          {
            const checked = Scalar.checkHexString(divs[1]);
            const expected = divs[2] === 'true';
            assert(checked === expected);
          }
          break;
        case 'random_scalar':
          {
            if (!executed) {
              Scalar.setupRandom(42);
              executed = true;
            }
            const scalar = Scalar.random();
            const expected = Buffer.from(divs[1], 'hex');
            assert(scalar.equals(expected));
          }
          break;
        case 'hash_to_scalar':
          {
            let scalar = Buffer.alloc(0);
            if (divs[1] !== "x") {
              scalar = Buffer.from(divs[1], 'hex');
            }
            let hash = Scalar.toHash(scalar);
            let expected = Buffer.from(divs[2], 'hex');
            assert(hash.equals(expected));
          }
          break;

        case 'generate_keys': {
          const publicKey = Buffer.from(divs[1], "hex");
          const secretKey = Buffer.from(divs[2], "hex");
          const keys = Key.generateKeys();
          assert(publicKey.equals(keys.public));
          assert(secretKey.equals(keys.secret));
        }
          break;
        case 'check_key': {
          const publicKey = Buffer.from(divs[1], "hex");
          assert(publicKey.length === 32);
          let expected = divs[2] === "true";
          assert(Key.check(publicKey) === expected);
        }
          break;
        case 'secret_key_to_public_key': {
          const secretKey = Buffer.from(divs[1], "hex");
          const expected1 = divs[2] === 'true';
          const publicKey = Key.secretToPublic(secretKey);
          if (expected1) {
            const expected2 = Buffer.from(divs[3], "hex");
            assert(publicKey.equals(expected2));
          } else {
            assert(publicKey.equals(Buffer.alloc(32)));
          }
        }
          break;
        case 'generate_key_derivation': {
          const publicKey = Buffer.from(divs[1], "hex");
          const secretKey = Buffer.from(divs[2], "hex");
          const expected1 = divs[3] === 'true';

          const derived = Key.derivate(publicKey, secretKey);

          if (expected1) {
            const expected2 = Buffer.from(divs[4], "hex");
            assert(derived.equals(expected2));
          } else {
            assert(derived.equals(Buffer.alloc(32)));
          }
        }
          break;
        case 'derive_public_key': {
          let derivation = Buffer.from(divs[1], "hex");
          let index = parseInt(divs[2]);
          let publicKey = Buffer.from(divs[3], "hex");
          const expected1 = divs[4] === 'true';
          let derived = Key.derivePublicKey(derivation, publicKey, index);
          if (expected1) {
            let expected2 = Buffer.from(divs[5], "hex");
            assert(derived.equals(expected2));
          } else {
            assert(derived.equals(Buffer.alloc(32)));
          }
        }
          break;
      }
    }
  });
});

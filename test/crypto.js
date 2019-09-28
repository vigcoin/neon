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
      }
      // console.log(divs);
    }
  });
});

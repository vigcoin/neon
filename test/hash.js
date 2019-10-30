let vigcoin = require("../lib");
let path = require("path");
const lineByLine = require('n-readlines');
let assert = require("assert");
let Hash = vigcoin.Hash;

describe("Test Hash", () => {
  it("should test fast hash", function () {
    this.timeout(50000);
    const file = path.resolve(__dirname, "./tests-fast.txt");
    const liner = new lineByLine(file);
    let line;
    while (line = liner.next()) {
      const split = String(line).split(" ");
      const expected = Buffer.from(split[0], 'hex');
      let plain = split[1];
      if (plain === "x") {
        plain = ""
      }
      plain = Buffer.from(plain, "hex");
      assert(expected.equals(Buffer.from(Hash.fast(plain), 'hex')));
    }
  });

  it("should test slow hash", function () {
    this.timeout(50000);
    const file = path.resolve(__dirname, "./tests-slow.txt");
    const liner = new lineByLine(file);
    let line;
    while (line = liner.next()) {
      const split = String(line).split(" ");
      const expected = Buffer.from(split[0], 'hex');
      let plain = split[1];
      if (plain === "x") {
        plain = ""
      }
      plain = Buffer.from(plain, "hex");
      assert(expected.equals(Buffer.from(Hash.slow(plain, 0), 'hex')));
    }
  });
});

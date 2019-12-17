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

  it("should test check hash", function () {
    assert(Hash.check(Buffer.from([
      0, 223, 74, 253, 65, 221, 188, 172, 253, 50, 122, 246, 173, 212, 162, 103, 13, 174, 254,
      199, 175, 49, 254, 177, 181, 91, 56, 9, 98, 1, 0, 0
    ]), 10343869));
  });

  it("should test check hash", function () {
    assert(Hash.from(Buffer.from([
      0, 223, 74, 253, 65, 221, 188, 172, 253, 50, 122, 246, 173, 212, 162, 103, 13, 174, 254,
      199, 175, 49, 254, 177, 181, 91, 56, 9, 98, 1, 0, 0
    ])).length === 32);
  });
});

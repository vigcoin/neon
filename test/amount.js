let vigcoin = require("../lib");
let assert = require("assert");
let Amount = vigcoin.Amount;


describe("Test Amount", () => {
  it("should test amount", function () {
    assert(0 === Amount.getPenalized(0, 1, 2));
    assert(2 === Amount.getPenalized(2, 1, 1));
    assert(7 === Amount.getPenalized(10, 1000, 1500));
    assert(1 === Amount.getPenalized(2, 10, 11));
  });
});
let vigcoin = require("../lib/");
let path = require("path");
let assert = require("assert");

let Wallet = vigcoin.Wallet;
let getFastHash = vigcoin.getFastHash;
let isPublicKey = vigcoin.isPublicKey;

describe("Test Wallet", () => {
  it("should create Wallet", function () {
    let file = path.resolve(__dirname, "./vig.wallet");
    let wallet = new Wallet(file, "");
    let address = wallet.toAddress(0x3d);
    keys = wallet.getPrivateKeys();
    assert(
      address ==
      "BFrj6s15vg47Za5ipA46m8CjV59nsEeeNSSozZzs9WEo759Prf3zXke4caP22RESH5Yj2GJubQ6WPCDBR78MX3myNaHsWME"
    );

    let output = path.resolve(__dirname, "vig-new.wallet");
    let password = "newpassword";
    wallet.save(output, password);

    let wallet1 = new Wallet(output, password);
    let address1 = wallet1.toAddress(0x3d);
    assert(
      address1 ==
      "BFrj6s15vg47Za5ipA46m8CjV59nsEeeNSSozZzs9WEo759Prf3zXke4caP22RESH5Yj2GJubQ6WPCDBR78MX3myNaHsWME"
    );
  });

  it("should catch error file not found", function () {
    let file = path.resolve(__dirname, "./vig.wallet");
    let errored = false;
    try {
      let wallet = new Wallet(file + "1111", "");
    } catch (e) {
      assert(e.message.indexOf("File not found!") !== -1);
      errored = true;
    }
    assert(errored);
  });

  it("should catch wrong password error", function () {
    let file = path.resolve(__dirname, "./vig.wallet");
    let errored = false;
    try {
      let wallet = new Wallet(file, "aoaaoao");
    } catch (e) {
      assert(e.message.indexOf("Wrong spend keys!") !== -1);
      errored = true;
    }
    assert(errored);
  });

  it("should create Wallet from none", function () {
    let wallet = new Wallet("", "");
    let object = wallet.create(0x3d);
    assert(object.spend);
    assert(object.view);
    assert(object.address);
  });

  it("should import Wallet from keys", function () {
    let wallet = new Wallet("", "");
    let keys1 = wallet.getPrivateKeys();

    wallet.setPrivateKeys(
      0x3d,
      "32e4e5f72797c2fc0e2dda4e80e61bd0093934a305af08c9d3b942715844aa08",
      "95a27c683df6a73bfc238d78fc55f414c699735d60fad4e3a999806763cb340d"
    );

    let address = wallet.toAddress(0x3d);
    assert(
      address ==
      "BFrj6s15vg47Za5ipA46m8CjV59nsEeeNSSozZzs9WEo759Prf3zXke4caP22RESH5Yj2GJubQ6WPCDBR78MX3myNaHsWME"
    );
  });

  it("should get fast hash", function () {
    let hash = getFastHash(Buffer.from([1]));
    assert(hash.length === 64);

    hash = getFastHash(Buffer.from([2]));
    assert(hash.length === 64);
    let input = [
      0x01,
      0x3c,
      0x01,
      0xff,
      0x00,
      0x01,
      0x01,
      0x02,
      0x9b,
      0x2e,
      0x4c,
      0x02,
      0x81,
      0xc0,
      0xb0,
      0x2e,
      0x7c,
      0x53,
      0x29,
      0x1a,
      0x94,
      0xd1,
      0xd0,
      0xcb,
      0xff,
      0x88,
      0x83,
      0xf8,
      0x02,
      0x4f,
      0x51,
      0x42,
      0xee,
      0x49,
      0x4f,
      0xfb,
      0xbd,
      0x08,
      0x80,
      0x71,
      0x21,
      0x01,
      0xa9,
      0xa4,
      0x56,
      0x9f,
      0x7e,
      0x10,
      0x16,
      0x4a,
      0x32,
      0x32,
      0x4b,
      0x2b,
      0x87,
      0x8a,
      0xe3,
      0x2d,
      0x98,
      0xbe,
      0x09,
      0x49,
      0xce,
      0x6e,
      0x01,
      0x50,
      0xba,
      0x1d,
      0x7e,
      0x54,
      0xd6,
      0x09,
      0x69,
      0xe5
    ];
    hash = getFastHash(Buffer.from(input));
    assert(hash.length === 64);
    hash = Buffer.from(hash, "hex");
    const temp = Buffer.from([
      81,
      131,
      30,
      137,
      17,
      68,
      149,
      122,
      23,
      4,
      105,
      195,
      35,
      123,
      221,
      255,
      230,
      192,
      96,
      73,
      129,
      38,
      117,
      210,
      237,
      178,
      168,
      52,
      82,
      247,
      162,
      80
    ]);
    hash.equals(temp);
  });

  it("should check public key", () => {
    const key = Buffer.from([
      81, 76, 248, 201, 237, 192, 109, 39, 58, 159, 67, 13, 120, 203, 91, 70, 36, 216, 162,
      222, 0, 100, 243, 152, 32, 48, 89, 129, 252, 169, 180, 36
    ]);
    assert(isPublicKey(key));
  });
});

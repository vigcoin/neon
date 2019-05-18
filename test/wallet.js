let vigcoin = require("../lib/");
let path = require("path");
let assert = require("assert");

let Wallet = vigcoin.Wallet;

let keys;

describe("Test Wallet", () => {
  it("should create Wallet", function () {
    let file = path.resolve(__dirname, "./vig.wallet");
    let wallet = new Wallet(file, "");
    let address = wallet.toAddress(0x3d);
    keys = wallet.getPrivateKeys();
    console.log(wallet.getPrivateKeys());
    assert(address == "BFrj6s15vg47Za5ipA46m8CjV59nsEeeNSSozZzs9WEo759Prf3zXke4caP22RESH5Yj2GJubQ6WPCDBR78MX3myNaHsWME");

    let output = path.resolve(__dirname, "vig-new.wallet");
    let password = "newpassword";
    wallet.save(output, password);


    let wallet1 = new Wallet(output, password);
    let address1 = wallet1.toAddress(0x3d);
    assert(address1 == "BFrj6s15vg47Za5ipA46m8CjV59nsEeeNSSozZzs9WEo759Prf3zXke4caP22RESH5Yj2GJubQ6WPCDBR78MX3myNaHsWME");

  });

  it("should catch error file not found", function () {
    let file = path.resolve(__dirname, "./vig.wallet");
    let errored = false;
    try {
      let wallet = new Wallet(file + "1111", "");
    } catch (e) {
      assert(e.message.indexOf('File not found!') !== -1);
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
      assert(e.message.indexOf('Wrong spend keys!') !== -1);
      errored = true;
    }
    assert(errored);
  });


  it("should create Wallet from none", function () {
    let wallet = new Wallet("", "");
    let object = wallet.create(0x3d);
    console.log(object);
    assert(object.spend);
    assert(object.view);
    assert(object.address);
  });

  it("should import Wallet from keys", function () {
    let wallet = new Wallet("", "");
    let keys1 = wallet.getPrivateKeys();

    wallet.setPrivateKeys(0x3d, '32e4e5f72797c2fc0e2dda4e80e61bd0093934a305af08c9d3b942715844aa08',
    '95a27c683df6a73bfc238d78fc55f414c699735d60fad4e3a999806763cb340d');

    let address = wallet.toAddress(0x3d);
    let keys2 = wallet.getPrivateKeys();
    console.log(wallet.getPrivateKeys());
    assert(address == "BFrj6s15vg47Za5ipA46m8CjV59nsEeeNSSozZzs9WEo759Prf3zXke4caP22RESH5Yj2GJubQ6WPCDBR78MX3myNaHsWME");
  });

});
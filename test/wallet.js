let vigcoin = require("../lib/");
let path = require("path");
let assert = require("assert");

let Wallet = vigcoin.Wallet;


describe("Test Wallet", () => {
  it("should create Wallet", function() {
    let file = path.resolve(__dirname, "./vig.wallet");
    let wallet = new Wallet(file, "");
    let address = wallet.toAddress(0x3d);
    assert(address == "BFrj6s15vg47Za5ipA46m8CjV59nsEeeNSSozZzs9WEo759Prf3zXke4caP22RESH5Yj2GJubQ6WPCDBR78MX3myNaHsWME");

    let output = path.resolve(__dirname, "vig-new.wallet");
    let password = "newpassword";
    wallet.save(output, password);


    let wallet1 = new Wallet(output, password);
    let address1 = wallet1.toAddress(0x3d);
    assert(address1 == "BFrj6s15vg47Za5ipA46m8CjV59nsEeeNSSozZzs9WEo759Prf3zXke4caP22RESH5Yj2GJubQ6WPCDBR78MX3myNaHsWME");

  });

});
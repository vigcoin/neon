let vigcoin = require("../lib");
let path = require("path");
const lineByLine = require('n-readlines');
let assert = require("assert");
let Difficulty = vigcoin.Difficulty;


describe("Test Difficulty", () => {
  it("should check diffculty", function () {
    this.timeout(50000);
    const file = path.resolve(__dirname, "./difficulty.txt");
    const liner = new lineByLine(file);
    let line;

    const target = 120;
    const win = 720;
    const cut = 60;
    const lag =  15;

    const diff = new Difficulty(
      target, cut, lag, win
    );
    let cumulative_difficulty = 0;
    const timestampes = [];
    const diffculties = [];
    let n = 0;
    while (line = liner.next()) {
      const divs = String(line).split(" ");
      const timestamp = parseInt(divs[0], 10);
      const difficulty = parseInt(divs[1], 10);
      
      let begin;
      let end;
      if (n < win + lag ) {
        begin = 0;
        end = n > win ? win: n;
      } else {
        end = n - lag ;
        begin = end - win;
      }
      let ts = [];
      for (i = begin; i < end; i++) {
        ts.push(timestampes[i]);
      }
      let tempdiff = []
      for (i = begin; i < end; i++) {
        tempdiff.push(diffculties[i]);
      }
      let res = diff.next(
        ts,
        tempdiff
      );
      assert(parseInt(res, 10) == difficulty);
      timestampes.push(timestamp);
      cumulative_difficulty += difficulty;
      diffculties.push(cumulative_difficulty);
      n += 1;
    }
  });
});
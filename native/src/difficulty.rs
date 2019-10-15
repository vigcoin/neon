
use cryptonote_raw_crypto::{*};
use neon::prelude::*;
use util::*;

declare_types! {
    /// JS class wrapping Difficulty Functions
    pub class JsDifficulty for Difficulty {
        init(mut cx) {
          let diffstr = cx.argument::<JsString>(0)?.value();
          let diffu64 = diffstr.parse::<u64>().unwrap();
          Ok(Difficulty::from(&diffu64))
        }
        method next(mut cx) {
    let mut b: Handle<JsBuffer> = cx.argument(0).expect("Fail to get argument!");
    let times = cx.borrow(&mut b, |data| {
      let slice = data.as_slice::<u64>();
      slice
    });
        let mut b1: Handle<JsBuffer> = cx.argument(1).expect("Fail to get argument!");
    let difficulties = cx.borrow(&mut b1, |data| {
      let slice = data.as_slice::<u64>();
      slice
    });
    let mut timestamps : Vec<u64> = vec![];
    for i in 0..timestamps.len() {
      timestamps.push(times[i]);
    }

            let mut this = cx.this();
            let difficulty = 
            {
            let guard = cx.lock();
            let diff = this.borrow_mut(& guard);
            let next = diff.next(timestamps.as_mut_slice(), difficulties);
            next
            };
            Ok(cx.string(difficulty.to_string()).upcast())
        }
    }
}
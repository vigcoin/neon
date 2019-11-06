
use cryptonote_raw_crypto::{difficulty::Difficulty};
use neon::prelude::*;

declare_types! {
    /// JS class wrapping Difficulty Functions
    pub class JsDifficulty for Difficulty {
        init(mut cx) {
            let target = cx.argument::<JsNumber>(0)?.value() as u8;
            let cut = cx.argument::<JsNumber>(1)?.value() as u8;
            let lag = cx.argument::<JsNumber>(2)?.value() as u16;
            let window = cx.argument::<JsNumber>(3)?.value() as u32;
            Ok(Difficulty {
                target,
                window,
                cut,
                lag
            })
        }
        method next(mut cx) {
            let ba: Handle<JsArray> = cx.argument::<JsArray>(0).expect("Fail to parse Array!");
            let vec: Vec<Handle<JsValue>> = ba.to_vec(&mut cx).expect("Fail to parse vec!");
            let mut timestamps: Vec<u64> = vec![];
            // Iterate over each item in the array
            for i in 0..vec.len() {
                // Each item is a generic `JsValue`, downcast it to a `JsBuffer`
                let value = vec[i].downcast::<JsNumber>().unwrap().value() as u64;
                timestamps.push(value)

            }
            let ba1: Handle<JsArray> = cx.argument::<JsArray>(1).expect("Fail to parse Array!");
            let vec1: Vec<Handle<JsValue>> = ba1.to_vec(&mut cx).expect("Fail to parse vec!");
            let mut difficulties: Vec<u64> = vec![];
            // Iterate over each item in the array
            for i in 0..vec1.len() {
                // Each item is a generic `JsValue`, downcast it to a `JsBuffer`
                let value = vec1[i].downcast::<JsNumber>().unwrap().value() as u64;
                difficulties.push(value)
            }
            let mut this = cx.this();
            let difficulty = 
            {
                let guard = cx.lock();
                let diff = this.borrow_mut(& guard);
                let next = diff.next(timestamps.as_mut_slice(), difficulties.as_slice());
                next
            };
            Ok(cx.string(difficulty.to_string()).upcast())
        }
    }
}

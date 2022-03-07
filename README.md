# Mutex for NEAR

## Introduction
We met a special situation when developing near contracts. Before contract A.f1() making a call to contract contract B.g1(), we set a context in f1(). And in the process of B.g1(),  we will visit the context set before by calling A.get1(). Because the cross-contract call will be actually implemented in the next block, how can we make sure the context is not being changed by others(e.g. some others calls A.f1() in the same block after, so the context will be changed before the first callback related the first call has been implemented)?

So are there any methods, we can keep the status of the context along with the processing in B.g1()  and the callback? That's what we are looking for in this project. 
We will upgrade when getting any progress.

## Conclusion
If two invocations to a same interface happens in the same block, the situation above will make a mistake. This is because the cross-contract call will happen in next block in near protocol.

<img src="./image/without locker.png">

If we add a mechanism like locker, the problom will be solved.

<img src="./image/with locker.png">

### Full Process
1. I deployed contract mutex_near and test_mutex;
2.  In contract `test_mutex`:
- I organize  two cross-contract calls to `cross_call_mutex` in contract `mutex_near` in two transactions in the same block, the related interface is`test_mutex::cross_call_test`;
3. In contract `mutex_near`:
- Within `cross_call_mutex`, I set a state(`md: MyData`) with a lock operation , and then make a cross-contract call to `test_mutex::visit_state`;
- The **lock** operation can be execute either at the beginning or at the end of `cross_call_mutex` because of the **atomic of the transaction**;
- The result will be shown in the related call back function(`callback_and_unlock`), the **unlock** operation must be execute the first line;
4. In contract `test_mutex`:
- In `visit_state` called from `mutex_near`, make a cross-contract call to `mutex_near::getContext` to get the state setted in `mutex_near`;
- Organized a reply about the state, return to `mutex_near::callback_and_unlock`;
5. In contract `mutex_near`:
- Print the result in `callback_and_unlock`(In fact we can check the state in `test_mutex`).


## About Near Contract(Rust)

To get started with this template:

1. Click the "Use this template" button to create a new repo based on this template
2. Update line 2 of `Cargo.toml` with your project name
3. Update line 4 of `Cargo.toml` with your project author names
4. Set up the [prerequisites](https://github.com/near/near-sdk-rs#pre-requisites)
5. Begin writing your smart contract in `src/lib.rs`
6. Test the contract 

    `cargo test -- --nocapture`

8. Build the contract

    `RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release`

**Get more info at:**

* [Rust Smart Contract Quick Start](https://docs.near.org/docs/develop/contracts/rust/intro)
* [Rust SDK Book](https://www.near-sdk.io/)

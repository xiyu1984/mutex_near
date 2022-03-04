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
I'll add explain for the mutex and the test soon...

## Getting started

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

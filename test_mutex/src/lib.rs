use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, log};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::collections::{LookupMap};
use near_sdk::PanicOnDefault;

// #[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
// #[serde(crate = "near_sdk::serde")]
// pub struct MyData{
//     s: String,
//     i: i32,
//     v: Vec<i32>,
// }

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // SETUP CONTRACT STATE
    // lm: LookupMap<i32, String>,
}


#[near_bindgen]
impl Contract {
    // #[init]
    // pub fn new()-> Self{
    //     Self{
    //         lm: LookupMap::new(b"l"),
    //     }
    // } 

    // pub fn onERC721Received(&self, operator: String, from: String, token_id: String, data: String){
    //     log!("{} transfered {} from {} to me. Info: {}", operator, token_id, from, data);
    // }

    // // ADD CONTRACT METHODS HERE
    // pub fn say_hello() ->String {
    //     String::from("hello near!")
    // }

    // pub fn c_say_hello(&self) -> String{
    //     String::from("hello near!")
    // }

    // pub fn get_my_data(&self)-> MyData{
    //     MyData{
    //         s: "hello".to_string(),
    //         i: -73,
    //         v: vec![1,2,3],
    //     }
    // }

    // pub fn set_and_get_md(&mut self, md: MyData) ->MyData{
    //     let mut md = md;
    //     md.s = hex::encode(near_sdk::env::sha256(md.s.as_bytes()));
    //     md
    // }
    
    // // insert can modify the val related to the key directly when the key exist in contract call
    // pub fn inster_val(&mut self, k: i32, s: String){
    //     let mk = k;
    //     let ms = s;
    //     self.lm.insert(&mk, &ms);
    // }

    // pub fn get_val(&self, k: i32) -> Option<String>{
    //     self.lm.get(&k)
    // }

    // #[private]
    // pub fn p_say_hello(&self) -> String{
    //     String::from("hello near!")
    // }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use near_sdk::test_utils::{get_logs, VMContextBuilder};
//     use near_sdk::{testing_env, AccountId};

//     // part of writing unit tests is setting up a mock context
//     // provide a `predecessor` here, it'll modify the default context
//     fn get_context(predecessor: AccountId) -> VMContextBuilder {
//         let mut builder = VMContextBuilder::new();
//         builder.predecessor_account_id(predecessor);
//         builder
//     }

//     // TESTS HERE
//     #[test]
//     fn my_test0() {
//         assert_eq!("hello near!".to_string(), Contract::say_hello());
//     }

//     #[test]
//     fn my_test1(){
//         let mut lm = near_sdk::collections::LookupMap::new(b"a");
//         let rst0 = lm.insert(&73, &String::from("hello zero"));
//         // insert cannot modify the val related to the exsited key in test
//         let rst1 = lm.insert(&73, &String::from("hello one"));
//         if let Some(val) = rst1 {
//             assert_eq!(val, "hello zero".to_string(), "not the value!");
//         }else{
//             assert!(false, "none of the key!");
//         }
//         // assert!(lm.contains_key(&77), "none of the key!");
//     }
// }

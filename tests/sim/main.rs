pub use near_sdk::json_types::{Base64VecU8, U64, U128};
use near_sdk::serde_json::json;
use near_sdk::{AccountId};
use near_sdk_sim::{call, view, init_simulator, UserAccount, STORAGE_AMOUNT, DEFAULT_GAS};
use mutex_near::Contract as MutexContract;
use test_mutex::Contract as TestContract;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    MUTEX_BYTES => "res/mutex_near.wasm",
    TEST_BYTES => "res/test_mutex.wasm",
}

fn init() -> (UserAccount, UserAccount, UserAccount) {
    let root = init_simulator(None);


     // Deploy the compiled Wasm bytes
    // let mutex: ContractAccount<MutexContract> = deploy!(
    //     contract: MutexContract,
    //     contract_id: "mutex".to_string(),
    //     bytes: &MUTEX_BYTES,
    //     signer_account: root
    // );
    let mutex = root.deploy(&MUTEX_BYTES, "mutex".parse().unwrap(), STORAGE_AMOUNT);
    // println!("{:#?}", mutex);

    // Deploy the compiled Wasm bytes
    // let test: ContractAccount<TestContract> = deploy!(
    //     contract: TestContract,
    //     contract_id: "test".to_string(),
    //     bytes: &TEST_BYTES,
    //     signer_account: root
    // );
    let test = root.deploy(&TEST_BYTES, "test".parse().unwrap(), STORAGE_AMOUNT);
    // println!("{:#?}", test);

    (root, mutex, test)
}

#[test]
fn mytest(){
    let (root, mutex, test) = init();

    // init mutex
    let result = root.call(test.account_id(), "new", &json!({}).to_string().into_bytes(), DEFAULT_GAS, 0);

    assert!(result.is_ok());
    // println!("{:?}", result.promise_results());

    // let result = root.view(test.account_id(), "c_say_hello", &json!({}).to_string().into_bytes());
    // println!("{}", String::from_utf8(result.unwrap()).unwrap());

    let result = root.call(mutex.account_id(), "new", &json!({}).to_string().into_bytes(), DEFAULT_GAS, 0);
    // println!("{:?}", result.promise_results());
    assert!(result.is_ok());

    // let result1 = root.call(mutex.account_id(), "cross_call_mutex", &json!({"account_id": test.account_id(), "method_name": "c_say_hello", "args": ""}).to_string().into_bytes(), DEFAULT_GAS, 0);
    // println!("result1 {:?}", result1.promise_results());
    // let result2 = root.call(mutex.account_id(), "getContext", &json!({}).to_string().into_bytes(), DEFAULT_GAS, 0);
    // println!("result2 {:?}", result2.promise_results());

    let seconde_call_args = json!({"account_id": test.account_id(), "method_name": "visit_state", "args": ""}).to_string();
    println!("{}", seconde_call_args);
    let result = root.call(test.account_id(), "cross_call_test", &json!({"account_id": mutex.account_id(), "method_name": "cross_call_mutex", "args": seconde_call_args}).to_string().into_bytes(), DEFAULT_GAS, 0);
    println!("\n\n{:?}", result.promise_results());
}
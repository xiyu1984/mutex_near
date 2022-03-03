pub use near_sdk::json_types::{Base64VecU8, ValidAccountId, U64, U128};
use near_sdk::serde_json::json;
use near_sdk::{AccountId};
use near_sdk_sim::{to_yocto, call, view, deploy, init_simulator, ContractAccount, UserAccount, ExecutionResult, STORAGE_AMOUNT, DEFAULT_GAS};
use mutex_near::Contract as MutexContract;
use test_mutex::Contract as TestContract;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    MUTEX_BYTES => "res/mutex_near.wasm",
    TEST_BYTES => "res/test_mutex.wasm",
}

fn init() -> (UserAccount, UserAccount) {
    let root = init_simulator(None);


     // Deploy the compiled Wasm bytes
    // let mutex: ContractAccount<MutexContract> = deploy!(
    //     contract: MutexContract,
    //     contract_id: "mutex".to_string(),
    //     bytes: &MUTEX_BYTES,
    //     signer_account: root
    // );
    // let mutex = root.deploy(&MUTEX_BYTES, "mutex".parse().unwrap(), to_yocto("10"));

    // Deploy the compiled Wasm bytes
    // let test: ContractAccount<TestContract> = deploy!(
    //     contract: TestContract,
    //     contract_id: "test".to_string(),
    //     bytes: &TEST_BYTES,
    //     signer_account: root
    // );
    let test = root.deploy(&TEST_BYTES, "test".parse().unwrap(), to_yocto("10"));

    (root, test)
}

#[test]
fn corss_call(){
    let (root, mutex) = init();

    // init mutex
    


}
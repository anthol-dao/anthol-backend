use candid::{decode_one, encode_one, Principal};
use pocket_ic::{PocketIc, WasmResult};
use common::{
    item::attr::{AttrKeys, AttrRequest},
    unit::Currency,
};
use std::fs;

const BACKEND_WASM: &str = "../../target/wasm32-unknown-unknown/release/anthol_backend.wasm";

fn setup() -> (PocketIc, Principal) {
    let pic = PocketIc::new();

    let backend_canister = pic.create_canister();
    pic.add_cycles(backend_canister, 2_000_000_000_000); // 2T Cycles
    let wasm = fs::read(BACKEND_WASM).expect("Wasm file not found, run 'dfx build'.");
    pic.install_canister(backend_canister, wasm, vec![], None);
    (pic, backend_canister)
}

#[test]
fn test_item_get() {
    let (pic, backend_canister) = setup();

    let balance_at_first = pic.cycle_balance(backend_canister);

    let _ = pic.update_call(
        backend_canister,
        Principal::anonymous(),
        "test_init",
        Vec::new(),
    );

    let balance_after_init = pic.cycle_balance(backend_canister);

    let arg = common::item::ItemPageRequest {
        store_id: "specimens".try_into().unwrap(),
        market_id: "specimens-authorized".try_into().unwrap(),
        request_to_store: common::item::ItemPageRequestToStoreCanister {
            item_id: "polaroid-onestep-2".try_into().unwrap(),
            attr: AttrRequest {
                keys: AttrKeys::default(),
                changed_key_index: None,
            },
            currency: Currency::USD,
        },
    };

    let Ok(WasmResult::Reply(response)) = pic.query_call(
        backend_canister,
        Principal::anonymous(),
        "get_item_page_data",
        encode_one(arg).unwrap(),
    ) else {
        panic!("Expected reply");
    };

    std::thread::sleep(std::time::Duration::from_secs(2));

    let balance_after_query = pic.cycle_balance(backend_canister);

    let result: Option<common::item::ItemPageResponse> = decode_one(&response).unwrap();
    let result = result.unwrap();
    assert_eq!(
        result.static_data.unwrap().data_from_store.item_name,
        "Polaroid OneStep 2"
    );

    println!(
        "Balance expences for init: {}",
        balance_at_first - balance_after_init
    );
    println!(
        "Balance expences for query: {}",
        balance_after_init - balance_after_query
    );
}

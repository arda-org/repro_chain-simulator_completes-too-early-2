// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                            2
// Async Callback (empty):               1
// Promise callbacks:                    1
// Total number of exported functions:   6

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    contract
    (
        init => init
        upgrade => upgrade
        remote => remote
        dummy => dummy
        remote_callback => remote_callback
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}

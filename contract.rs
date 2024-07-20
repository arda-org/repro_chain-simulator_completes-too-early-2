#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait Contract
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[proxy]
    fn self_proxy(&self, address: ManagedAddress) -> self::Proxy<Self::Api>;

    #[payable("*")]
    #[endpoint]
    fn remote(&self, remote_address: ManagedAddress) {
        let (token_id, token_amount) = self.call_value().egld_or_single_fungible_esdt();
        self.self_proxy(remote_address)
            .dummy()
            .with_gas_limit(10_000_000)
            .callback(self.callbacks().remote_callback(self.blockchain().get_caller(), token_id, token_amount))
            .register_promise();
    }

    #[promises_callback]
    fn remote_callback(
        &self,
        original_caller: ManagedAddress,
        token_id: EgldOrEsdtTokenIdentifier,
        token_amount: BigUint,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.send().direct(&original_caller, &token_id, 0, &token_amount);
            }
            ManagedAsyncCallResult::Err(_) => {}
        }
    }

    #[endpoint]
    fn dummy(&self) {}
}

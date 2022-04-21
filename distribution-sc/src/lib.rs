#![no_std]

elrond_wasm::imports!();


#[elrond_wasm::contract]
pub trait Distribution {
    #[init]
    fn init(&self, dist_token_id: TokenIdentifier, dist_token_price: BigUint) {
        self.distributable_token_id().set_if_empty(&dist_token_id);
        self.distributable_token_price().set_if_empty(&dist_token_price);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(deposit)]
    fn deposit_endpoint(&self, #[payment_token] token: TokenIdentifier) -> SCResult<()> {
        require!(token == self.distributable_token_id().get(), "invalid token");
        Ok(())
    }

    #[payable("EGLD")]
    #[endpoint(claim)]
    fn buy_endpoint(&self, #[payment_amount] paid_amount: BigUint) -> SCResult<()> {
        require!(paid_amount == 0, "zero really");

        let caller = self.blockchain().get_caller();
        let dist_token_id = self.distributable_token_id().get();
        let price_per_token = self.distributable_token_price().get();
        let _available_token_amount = self.blockchain().get_sc_balance(&dist_token_id, 0);

        let token_amount = &price_per_token;

        require!(!self.did_user_ping(&caller), "Already Claimed");

        let current_block_timestamp = self.blockchain().get_block_timestamp();
        self.user_ping_timestamp(&caller)
            .set(&current_block_timestamp);
        //require!(token_amount <= available_token_amount, "not enough tokens available");

        self.send().direct(&caller, &dist_token_id, 0, &token_amount, &[]);

        Ok(())
    }

    //view 
    #[view(didUserPing)]
    fn did_user_ping(&self, address: &ManagedAddress) -> bool {
        !self.user_ping_timestamp(address).is_empty()
    }


    #[view(getDistributableTokenId)]
    #[storage_mapper("distributableToken")]
    fn distributable_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getDistributablePrice)]
    #[storage_mapper("distributablePrice")]
    fn distributable_token_price(&self) -> SingleValueMapper<BigUint>;

    #[view(getUserPingTimestamp)]
    #[storage_mapper("userPingTimestamp")]
    fn user_ping_timestamp(&self, address: &ManagedAddress) -> SingleValueMapper<u64>;

}

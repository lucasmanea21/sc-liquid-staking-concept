elrond_wasm::derive_imports!();
elrond_wasm::imports!();

#[derive(TopEncode)]
pub struct TokenAttributes {
    pub test: u64,
}

#[derive(TopEncode)]
pub struct UndelegatedAttributes {
    undelegated_timestamp: u64,
    claim_timestamp: u64
}

#[elrond_wasm::module]
pub trait CallbacksModule: crate::storage::StorageModule {
     /*
        CALLBACKS 
     */

    //  Delegation

    #[callback]
    fn delegate_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<()>,
        caller: &ManagedAddress,
        payment_amount: BigUint
    ){

        match result {
            ManagedAsyncCallResult::Ok(()) => {
                let token_id = &self.token_identifier().get();
                
                let attributes =  TokenAttributes {
                    test: 1u64
                };


                // self.send().esdt_local_mint(token_id, 0, &payment_amount);
                // self.send().direct_esdt(&caller, token_id, 0, &payment_amount )
                self.locked_token().nft_create_and_send(&caller, payment_amount.clone(), &attributes);
                // self.locked_token().nft_add_quantity_and_send(&caller, )
            },
            ManagedAsyncCallResult::Err(err) => {
                // log the error in storage
                self.succedeed().set(&false);
            },
    }}

    
    
    #[callback]
    fn undelegate_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<()>,
        caller: ManagedAddress,
        nonce: u64,
        payment_amount: BigUint
    ){

        match result {
            ManagedAsyncCallResult::Ok(()) => {
                // let token_id = &self.token_identifier().get();
                let timestamp = self.blockchain().get_block_timestamp();

                let attributes =  UndelegatedAttributes {
                    undelegated_timestamp: timestamp,
                    claim_timestamp: timestamp
                };

                self.locked_token().nft_burn(nonce.clone(), &payment_amount);
                self.undelegated_token().nft_create_and_send(&caller, payment_amount.clone(), &attributes);
            },
            ManagedAsyncCallResult::Err(err) => {
                // log the error in storage
                self.succedeed().set(&false);
            },
    } }

    // Tokens 

    #[callback]
    fn esdt_issue_callback(
        &self,
        caller: &ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_identifier) => {
                self.issue_success_event(caller, &token_identifier, &BigUint::zero());
                self.token_identifier().set(&token_identifier);
            },
            ManagedAsyncCallResult::Err(message) => {
                let (token_identifier, returned_tokens) =
                    self.call_value().egld_or_single_fungible_esdt();
                self.issue_failure_event(caller, &message.err_msg);

                // return issue cost to the owner
                // TODO: test that it works
                if token_identifier.is_egld() && returned_tokens > 0 {
                    self.send().direct_egld(caller, &returned_tokens);
                }
            },
        }
    }    
}
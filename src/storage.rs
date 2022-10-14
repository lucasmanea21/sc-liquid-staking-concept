elrond_wasm::derive_imports!();
elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait StorageModule{
/*
        STORAGE
    */

    #[view(getSum)]
    #[storage_mapper("sum")]
    fn sum(&self) -> SingleValueMapper<BigUint>;

    #[view(getSuccedeed)]
    #[storage_mapper("succedeed")]
    fn succedeed(&self) -> SingleValueMapper<bool>;

    #[view(getToken)]
    #[storage_mapper("token_identifier")]
    fn token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;
    
    #[view(getMetaToken)]
    #[storage_mapper("locked_token")]
    fn locked_token(&self) -> NonFungibleTokenMapper<Self::Api>;
    
    #[view(getUndelegatedToken)]
    #[storage_mapper("undelegated_token")]
    fn undelegated_token(&self) -> NonFungibleTokenMapper<Self::Api>;
    


    #[only_owner]
    #[endpoint(setToken)]
    fn set_token(&self, token_id: TokenIdentifier) {
        self.token_identifier().set(token_id);
    }
}
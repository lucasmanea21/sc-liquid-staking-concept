elrond_wasm::imports!();

#[elrond_wasm::proxy]
pub trait Delegate {

    #[payable("EGLD")]
    #[endpoint(delegate)]
    fn wrap_egld(
        &self,
        #[payment_token] payment_token: EgldOrEsdtTokenIdentifier,
        #[payment_amount] payment_amount: BigUint,
    );

    
}
#![no_std]

elrond_wasm::imports!();

mod wrap;
mod delegate;
mod callbacks;
pub mod storage;
pub mod events;

// use module_namespace::ProxyTrait as _;
elrond_wasm::derive_imports!();

const EGLD_NUM_DECIMALS: usize = 18;

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[elrond_wasm::contract]
pub trait Adder: 
    elrond_wasm_modules::default_issue_callbacks::DefaultIssueCallbacksModule 
    + storage::StorageModule
    + callbacks::CallbacksModule
    + events::EventsModule {

    #[proxy]
    fn wrapper_contract(&self, sc_address: ManagedAddress) -> wrap::Proxy<Self::Api>;
    
    #[proxy]
    fn delegate_contract(&self, sc_address: ManagedAddress) -> delegate::Proxy<Self::Api>;


    #[init]
    fn init(&self, initial_value: BigUint) {
        self.sum().set(initial_value);
    }

      #[only_owner]
      #[payable("EGLD")]
      #[endpoint(issueToken)]
      fn issue_wrapped_egld(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        require!(
            self.token_identifier().is_empty(),
            "token was already issued"
        );

        let issue_cost = self.call_value().egld_value();
        let caller = self.blockchain().get_caller();
        let initial_supply = BigUint::zero();

        self.issue_started_event(&caller, &token_ticker, &initial_supply);

        self.send()
            .esdt_system_sc_proxy()
            .issue_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                &initial_supply,
                FungibleTokenProperties {
                    num_decimals: EGLD_NUM_DECIMALS,
                    can_freeze: false,
                    can_wipe: false,
                    can_pause: false,
                    can_mint: true,
                    can_burn: false,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(Adder::callbacks(self).esdt_issue_callback(&caller))
            .call_and_exit()
    }
  
   

    #[only_owner]
    #[endpoint(setLocalRoles)]
    fn set_local_roles(&self) {
        require!(
            !self.token_identifier().is_empty(),
            "Must issue token first"
        );

        let roles = [EsdtLocalRole::Mint, EsdtLocalRole::Burn];
        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(
                &self.blockchain().get_sc_address(),
                &self.token_identifier().get(),
                roles[..].iter().cloned(),
            )
            .async_call()
            .call_and_exit()
    }

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueLockedToken)]
    fn issue_locked_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        num_decimals: usize,
    ) {
        let caller = self.blockchain().get_caller();
        let payment_amount = self.call_value().egld_value();

        self.locked_token().issue_and_set_all_roles(
            EsdtTokenType::Meta,
            payment_amount,
            token_display_name,
            token_ticker,
            num_decimals,
            None,
        );
    }

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueUndelegatedToken)]
    fn issue_undelegated_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        num_decimals: usize,
    ) {
        let caller = self.blockchain().get_caller();
        let payment_amount = self.call_value().egld_value();

        self.undelegated_token().issue_and_set_all_roles(
            EsdtTokenType::Meta,
            payment_amount,
            token_display_name,
            token_ticker,
            num_decimals,
            None,
        );
    }

    #[payable("EGLD")]
    #[endpoint]
    fn stake(&self, #[payment_amount] payment_amount: BigUint, delegate_address: ManagedAddress) {
        require!(payment_amount >= BigUint::from(1000000000000000000u64), "insufficient EGLD amount sent");

        let send_amount = self.call_value().egld_value();
        let caller = self.blockchain().get_caller();

        self.delegate_contract(delegate_address)
            .delegate(EgldOrEsdtTokenIdentifier::egld(), payment_amount)
            .async_call()
            .with_callback(Adder::callbacks(self).delegate_callback(&caller, send_amount))
            .call_and_exit();
    }

    #[payable("*")]
    #[endpoint]
    fn undelegate(&self, delegate_address: ManagedAddress) {
       let (payment_token, payment_nonce, payment_amount) =
            self.call_value().single_esdt().into_tuple();

        require!(payment_token == self.token_identifier().get(), "Wrong token sent");
        require!(payment_amount >= BigUint::from(1000000000000000000u64), "Insufficient EGLD amount sent");
        let caller = self.blockchain().get_caller();

        self.delegate_contract(delegate_address)
            .unDelegate(payment_amount.clone())
            .async_call()
            .with_callback(Adder::callbacks(self).undelegate_callback(caller, payment_nonce, payment_amount))
            .call_and_exit();
    }


}

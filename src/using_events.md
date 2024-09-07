# Using Events

An event is a mechanism for emitting notifications about specific actions or state changes that occur within a blockchain runtime. Events are typically used to inform the outside world about occurrences such as token transfers, account creations, or other significant operations within the blockchain.


```rust, ignore
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    /// A user has successfully set a new value.
    SomethingStored {
        /// The new value set.
        something: u32,
        /// The account who set the new value.
        who: T::AccountId,
    },
}
```

```rust, ignores
#[pallet::call_index(0)]
#[pallet::weight(T::WeightInfo::do_something())]
pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
    // Check that the extrinsic was signed and get the signer.
    let who = ensure_signed(origin)?;

    // Update storage.
    Something::<T>::put(something);

    // Emit an event.
    Self::deposit_event(Event::SomethingStored { something, who });

    // Return a successful `DispatchResult`
    Ok(())
}
```


## Quiz
{{#quiz using_events.toml}}

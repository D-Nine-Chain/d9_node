##

`executor` chooses the appropriate binary by checking the following information for native and WebAsembly runtime:

- `spec_name`
- `spec_version`
- `authoring_version`

this information is provided by runtime version struct:

```rust
pub const VERSION: RuntimeVersion = RuntimeVersion{
   ///basically defines your network.
   /// The unique identity of the runtime. It should remain unchanged
   /// during the life cycle of a chain to identify the unique origin
   /// f the runtime.
   spec_name: create_runtime_str!("d9")
   ///differentiates implementation teams. basically keep as d9
   impl_name: create_runtime_str!("d9")
   ///The version of the authorship interface. This number needs to be
   /// updated every time there are changes made that could alter the
   /// way blocks are produced.
   authoring_version: 1
   ///This is a version number for the specification of the runtime.
   /// This version number is incremented every time there is a change
   /// in the runtime that would require a hard fork to implement,
   /// meaning all nodes need to upgrade. This could be changes to the
   /// types of data stored in the state or modifications to the block
   /// execution logic.
   spec_version:1
   ///The version of the runtime implementation. This needs to be
   /// updated every time there are significant changes in the runtime
   /// implementation that would af˝ the state format. For example,
   /// optimization in the runtime that doesn't change the function or
   /// state format.
   impl_version:1
   ///A list of supported API "features" along with their versions.
   /// refer to macro rul impl_runtime_apis! for mere details
   apis: RUNTIME_API_VERSIONS,
   /// The `transaction_version` represents the version of the interface for handling transactions.
   /// This version number is incremented every time there are changes made in the extrinsics,
   /// which include the data fields that are external to the system, such as user inputs.
   /// The parameter serves a key role in synchronizing firmware updates for hardware wallets
   /// or other signing devices, helping ensure runtime transactions remain valid.
   ///
   /// When there is a change in the index of the pallets in the `construct_runtime!` macro or
   /// if there are any changes to dispatchable functions - such as the number, order, or types of
   /// parameters - this version number must be bumped. Changes that necessitate a bump can include
   /// adding, removing, or modifying dispatchable functions in the runtime.
   ///
   /// The versioning is crucial as it allows hardware wallets and other devices to understand
   /// which transactions they can safely sign based on the specific runtime version they are compatible with.
   ///
   /// It's worth noting that an update to the `transaction_version` implies a significant change in
   /// the runtime and hence, the `spec_version` must also be updated to indicate a change in the
   /// runtime's specification.
   transaction_version:1
   /// Version of the state implementation used by this runtime.
   /// Use of an incorrect version is consensus breaking.
   state_version:1
}
```

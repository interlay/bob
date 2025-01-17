///Module containing a contract's types and functions.
/**

```solidity
library StdInvariant {
    struct FuzzInterface { address addr; string[] artifacts; }
    struct FuzzSelector { address addr; bytes4[] selectors; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod StdInvariant {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct FuzzInterface { address addr; string[] artifacts; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzInterface {
        pub addr: alloy::sol_types::private::Address,
        pub artifacts: alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<FuzzInterface> for UnderlyingRustTuple<'_> {
            fn from(value: FuzzInterface) -> Self {
                (value.addr, value.artifacts)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FuzzInterface {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    addr: tuple.0,
                    artifacts: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FuzzInterface {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FuzzInterface {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.addr,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(&self.artifacts),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for FuzzInterface {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for FuzzInterface {
            const NAME: &'static str = "FuzzInterface";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzInterface(address addr,string[] artifacts)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.addr,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.artifacts)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FuzzInterface {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.addr,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.artifacts,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.addr,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::String,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.artifacts,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**```solidity
struct FuzzSelector { address addr; bytes4[] selectors; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzSelector {
        pub addr: alloy::sol_types::private::Address,
        pub selectors: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<4>,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<4>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<4>>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<FuzzSelector> for UnderlyingRustTuple<'_> {
            fn from(value: FuzzSelector) -> Self {
                (value.addr, value.selectors)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FuzzSelector {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    addr: tuple.0,
                    selectors: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FuzzSelector {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FuzzSelector {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.addr,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::tokenize(&self.selectors),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for FuzzSelector {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for FuzzSelector {
            const NAME: &'static str = "FuzzSelector";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzSelector(address addr,bytes4[] selectors)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.addr,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.selectors)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FuzzSelector {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.addr,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.selectors,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.addr,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<4>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.selectors,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`StdInvariant`](self) contract instance.

See the [wrapper's documentation](`StdInvariantInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> StdInvariantInstance<T, P, N> {
        StdInvariantInstance::<T, P, N>::new(address, provider)
    }
    /**A [`StdInvariant`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`StdInvariant`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct StdInvariantInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for StdInvariantInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("StdInvariantInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > StdInvariantInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`StdInvariant`](self) contract instance.

See the [wrapper's documentation](`StdInvariantInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> StdInvariantInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> StdInvariantInstance<T, P, N> {
            StdInvariantInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > StdInvariantInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > StdInvariantInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library StdInvariant {
    struct FuzzInterface {
        address addr;
        string[] artifacts;
    }
    struct FuzzSelector {
        address addr;
        bytes4[] selectors;
    }
}

interface FullRelayWithVerifyTest {
    event log(string);
    event log_address(address);
    event log_array(uint256[] val);
    event log_array(int256[] val);
    event log_array(address[] val);
    event log_bytes(bytes);
    event log_bytes32(bytes32);
    event log_int(int256);
    event log_named_address(string key, address val);
    event log_named_array(string key, uint256[] val);
    event log_named_array(string key, int256[] val);
    event log_named_array(string key, address[] val);
    event log_named_bytes(string key, bytes val);
    event log_named_bytes32(string key, bytes32 val);
    event log_named_decimal_int(string key, int256 val, uint256 decimals);
    event log_named_decimal_uint(string key, uint256 val, uint256 decimals);
    event log_named_int(string key, int256 val);
    event log_named_string(string key, string val);
    event log_named_uint(string key, uint256 val);
    event log_string(string);
    event log_uint(uint256);
    event logs(bytes);

    constructor();

    function IS_TEST() external view returns (bool);
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function failed() external returns (bool);
    function getBlockHeights(string memory chainName, uint256 from, uint256 to) external view returns (uint256[] memory elements);
    function getDigestLes(string memory chainName, uint256 from, uint256 to) external view returns (bytes32[] memory elements);
    function getHeaderHexes(string memory chainName, uint256 from, uint256 to) external view returns (bytes[] memory elements);
    function getHeaders(string memory chainName, uint256 from, uint256 to) external view returns (bytes memory headers);
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
    function testGCDDoesntConfirmHeader() external;
    function testIncorrectProofSupplied() external;
    function testIncorrectTxIdSupplied() external;
    function testIncorrectTxSupplied() external;
    function testInsufficientConfirmations() external;
    function testMalformedProofSupplied() external;
    function testSuccessfullyVerify() external view;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "IS_TEST",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeArtifacts",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedArtifacts_",
        "type": "string[]",
        "internalType": "string[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeContracts",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedContracts_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeSenders",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedSenders_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "failed",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getBlockHeights",
    "inputs": [
      {
        "name": "chainName",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "from",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "to",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "elements",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getDigestLes",
    "inputs": [
      {
        "name": "chainName",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "from",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "to",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "elements",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getHeaderHexes",
    "inputs": [
      {
        "name": "chainName",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "from",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "to",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "elements",
        "type": "bytes[]",
        "internalType": "bytes[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getHeaders",
    "inputs": [
      {
        "name": "chainName",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "from",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "to",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "headers",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetArtifactSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedArtifactSelectors_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzSelector[]",
        "components": [
          {
            "name": "addr",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "selectors",
            "type": "bytes4[]",
            "internalType": "bytes4[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetArtifacts",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedArtifacts_",
        "type": "string[]",
        "internalType": "string[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetContracts",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedContracts_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetInterfaces",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedInterfaces_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzInterface[]",
        "components": [
          {
            "name": "addr",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "artifacts",
            "type": "string[]",
            "internalType": "string[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedSelectors_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzSelector[]",
        "components": [
          {
            "name": "addr",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "selectors",
            "type": "bytes4[]",
            "internalType": "bytes4[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetSenders",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedSenders_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "testGCDDoesntConfirmHeader",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testIncorrectProofSupplied",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testIncorrectTxIdSupplied",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testIncorrectTxSupplied",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testInsufficientConfirmations",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testMalformedProofSupplied",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testSuccessfullyVerify",
    "inputs": [],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "log",
    "inputs": [
      {
        "name": "",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_address",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_array",
    "inputs": [
      {
        "name": "val",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_array",
    "inputs": [
      {
        "name": "val",
        "type": "int256[]",
        "indexed": false,
        "internalType": "int256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_array",
    "inputs": [
      {
        "name": "val",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_bytes",
    "inputs": [
      {
        "name": "",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_bytes32",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_int",
    "inputs": [
      {
        "name": "",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_address",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_array",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_array",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "int256[]",
        "indexed": false,
        "internalType": "int256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_array",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_bytes",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_bytes32",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_decimal_int",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      },
      {
        "name": "decimals",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_decimal_uint",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "decimals",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_int",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_string",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_uint",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_string",
    "inputs": [
      {
        "name": "",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_uint",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "logs",
    "inputs": [
      {
        "name": "",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod FullRelayWithVerifyTest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60078054600160ff199182168117909255600b8054909116909117905561010060405260506080818152906154f060a039601e9061003d90826106c2565b5060405180610160016040528061014081526020016155406101409139601f9061006790826106c2565b507f48e5a1a0e616d8fd92b4ef228c424e0c816799a256c6a90892195ccfc53300d660205561011960215534801561009d575f5ffd5b506040518060400160405280600c81526020016b3432b0b232b939973539b7b760a11b8152506040518060400160405280600c81526020016b05ccecadccae6d2e65cd0caf60a31b8152506040518060400160405280600f81526020016e0b99d95b995cda5ccb9a195a59da1d608a1b815250604051806040016040528060128152602001712e67656e657369732e6469676573745f6c6560701b8152505f7f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c6001600160a01b031663d930a0e66040518163ffffffff1660e01b81526004015f60405180830381865afa158015610199573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526101c091908101906107f1565b90505f81866040516020016101d6929190610854565b60408051601f19818403018152908290526360f9bb1160e01b82529150737109709ecfa91a80626ff3989d68f67f5b1dd12d906360f9bb119061021d9084906004016108c6565b5f60405180830381865afa158015610237573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261025e91908101906107f1565b601d9061026b90826106c2565b5061030285601d805461027d9061063e565b80601f01602080910402602001604051908101604052809291908181526020018280546102a99061063e565b80156102f45780601f106102cb576101008083540402835291602001916102f4565b820191905f5260205f20905b8154815290600101906020018083116102d757829003601f168201915b5093949350506104e4915050565b61039885601d80546103139061063e565b80601f016020809104026020016040519081016040528092919081815260200182805461033f9061063e565b801561038a5780601f106103615761010080835404028352916020019161038a565b820191905f5260205f20905b81548152906001019060200180831161036d57829003601f168201915b509394935050610568915050565b61042e85601d80546103a99061063e565b80601f01602080910402602001604051908101604052809291908181526020018280546103d59061063e565b80156104205780601f106103f757610100808354040283529160200191610420565b820191905f5260205f20905b81548152906001019060200180831161040357829003601f168201915b5093949350506105e2915050565b60405161043a9061061d565b610446939291906108d8565b604051809103905ff08015801561045f573d5f5f3e3d5ffd5b50601c80546001600160a01b0319166001600160a01b0392909216918217905560405163f58db06f60e01b8152600160048201819052602482015290965063f58db06f955060440193506104b292505050565b5f604051808303815f87803b1580156104c9575f5ffd5b505af11580156104db573d5f5f3e3d5ffd5b50505050610937565b604051631fb2437d60e31b8152606090737109709ecfa91a80626ff3989d68f67f5b1dd12d9063fd921be89061052090869086906004016108fc565b5f60405180830381865afa15801561053a573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261056191908101906107f1565b9392505050565b6040516356eef15b60e11b81525f90737109709ecfa91a80626ff3989d68f67f5b1dd12d9063addde2b6906105a390869086906004016108fc565b602060405180830381865afa1580156105be573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105619190610920565b604051631777e59d60e01b81525f90737109709ecfa91a80626ff3989d68f67f5b1dd12d90631777e59d906105a390869086906004016108fc565b6128d780612c1983390190565b634e487b7160e01b5f52604160045260245ffd5b600181811c9082168061065257607f821691505b60208210810361067057634e487b7160e01b5f52602260045260245ffd5b50919050565b601f8211156106bd57805f5260205f20601f840160051c8101602085101561069b5750805b601f840160051c820191505b818110156106ba575f81556001016106a7565b50505b505050565b81516001600160401b038111156106db576106db61062a565b6106ef816106e9845461063e565b84610676565b6020601f821160018114610721575f831561070a5750848201515b5f19600385901b1c1916600184901b1784556106ba565b5f84815260208120601f198516915b828110156107505787850151825560209485019460019092019101610730565b508482101561076d57868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b5f806001600160401b038411156107955761079561062a565b50604051601f19601f85018116603f011681018181106001600160401b03821117156107c3576107c361062a565b6040528381529050808284018510156107da575f5ffd5b8383602083015e5f60208583010152509392505050565b5f60208284031215610801575f5ffd5b81516001600160401b03811115610816575f5ffd5b8201601f81018413610826575f5ffd5b6108358482516020840161077c565b949350505050565b5f81518060208401855e5f93019283525090919050565b5f61085f828561083d565b7f2f746573742f66756c6c52656c61792f74657374446174612f00000000000000815261088f601982018561083d565b95945050505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f6105616020830184610898565b606081525f6108ea6060830186610898565b60208301949094525060400152919050565b604081525f61090e6040830185610898565b828103602084015261088f8185610898565b5f60208284031215610930575f5ffd5b5051919050565b6122d5806109445f395ff3fe608060405234801561000f575f5ffd5b5060043610610179575f3560e01c806385226c81116100d2578063e20c9f7111610088578063f11d5cbc11610063578063f11d5cbc146102ac578063fa7626d4146102b4578063fad06b8f146102c1575f5ffd5b8063e20c9f7114610294578063e89f84191461029c578063f0c0832e146102a4575f5ffd5b8063b52b2058116100b8578063b52b20581461026c578063b5508aa914610274578063ba414fa61461027c575f5ffd5b806385226c811461024f578063916a17c614610264575f5ffd5b80633e5e3c2311610132578063641e144d1161010d578063641e144d1461022a57806366d9a9a014610232578063740dba7414610247575f5ffd5b80633e5e3c23146101fa5780633f7286f41461020257806344badbb61461020a575f5ffd5b80631ed7831c116101625780631ed7831c146101c65780632ade3880146101db57806339425f8f146101f0575f5ffd5b80630813852a1461017d5780631c0da81f146101a6575b5f5ffd5b61019061018b3660046119f4565b6102d4565b60405161019d9190611aa9565b60405180910390f35b6101b96101b43660046119f4565b61031f565b60405161019d9190611b0c565b6101ce610391565b60405161019d9190611b1e565b6101e36103fe565b60405161019d9190611bd0565b6101f8610547565b005b6101ce61066e565b6101ce6106d9565b61021d6102183660046119f4565b610744565b60405161019d9190611c54565b6101f8610787565b61023a610890565b60405161019d9190611c8b565b6101f8610993565b610257610a86565b60405161019d9190611d5c565b61023a610b51565b6101f8610c54565b610257610c9e565b610284610d69565b604051901515815260200161019d565b6101ce610ea6565b6101f8610f11565b6101f8611006565b6101f86110fb565b6007546102849060ff1681565b61021d6102cf3660046119f4565b611278565b60606103178484846040518060400160405280600381526020017f68657800000000000000000000000000000000000000000000000000000000008152506112bb565b949350505050565b60605f61032d8585856102d4565b90505f5b61033b8585611d9b565b811015610388578282828151811061035557610355611dae565b602002602001015160405160200161036e929190611df2565b60408051601f198184030181529190529250600101610331565b50509392505050565b606060148054806020026020016040519081016040528092919081815260200182805480156103f457602002820191905f5260205f20905b815473ffffffffffffffffffffffffffffffffffffffff1681526001909101906020018083116103c9575b5050505050905090565b6060601b805480602002602001604051908101604052809291908181526020015f905b8282101561053e575f848152602080822060408051808201825260028702909201805473ffffffffffffffffffffffffffffffffffffffff168352600181018054835181870281018701909452808452939591948681019491929084015b82821015610527578382905f5260205f2001805461049c90611e06565b80601f01602080910402602001604051908101604052809291908181526020018280546104c890611e06565b80156105135780601f106104ea57610100808354040283529160200191610513565b820191905f5260205f20905b8154815290600101906020018083116104f657829003601f168201915b50505050508152602001906001019061047f565b505050508152505081526020019060010190610421565b50505050905090565b604080518082018252601a81527f496e73756666696369656e7420636f6e6669726d6174696f6e73000000000000602082015290517ff28dceb3000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d9163f28dceb3916105c89190600401611b0c565b5f604051808303815f87803b1580156105df575f5ffd5b505af11580156105f1573d5f5f3e3d5ffd5b5050601c54602054602154604051625d09a760e41b815273ffffffffffffffffffffffffffffffffffffffff90931694506305d09a70935061064092601e92601f929091600990600401611f2c565b5f6040518083038186803b158015610656575f5ffd5b505afa158015610668573d5f5f3e3d5ffd5b50505050565b606060168054806020026020016040519081016040528092919081815260200182805480156103f457602002820191905f5260205f2090815473ffffffffffffffffffffffffffffffffffffffff1681526001909101906020018083116103c9575050505050905090565b606060158054806020026020016040519081016040528092919081815260200182805480156103f457602002820191905f5260205f2090815473ffffffffffffffffffffffffffffffffffffffff1681526001909101906020018083116103c9575050505050905090565b60606103178484846040518060400160405280600981526020017f6469676573745f6c65000000000000000000000000000000000000000000000081525061141c565b604080518082018252601381527f42616420696e636c7573696f6e2070726f6f6600000000000000000000000000602082015290517ff28dceb3000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d9163f28dceb3916108089190600401611b0c565b5f604051808303815f87803b15801561081f575f5ffd5b505af1158015610831573d5f5f3e3d5ffd5b5050601c5460205460215473ffffffffffffffffffffffffffffffffffffffff90921693506305d09a709250601e91601f919061086f906001611f71565b5f6040518663ffffffff1660e01b8152600401610640959493929190611f2c565b60606019805480602002602001604051908101604052809291908181526020015f905b8282101561053e575f84815260209081902060408051808201825260028602909201805473ffffffffffffffffffffffffffffffffffffffff16835260018101805483518187028101870190945280845293949193858301939283018282801561097b57602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916815260200190600401906020826003010492830192600103820291508084116109285790505b505050505081525050815260200190600101906108b3565b604080518082018252601381527f42616420696e636c7573696f6e2070726f6f6600000000000000000000000000602082015290517ff28dceb3000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d9163f28dceb391610a149190600401611b0c565b5f604051808303815f87803b158015610a2b575f5ffd5b505af1158015610a3d573d5f5f3e3d5ffd5b5050601c54602154604051625d09a760e41b815273ffffffffffffffffffffffffffffffffffffffff90921693506305d09a70925061064091601e91601f915f90600401611f84565b60606018805480602002602001604051908101604052809291908181526020015f905b8282101561053e578382905f5260205f20018054610ac690611e06565b80601f0160208091040260200160405190810160405280929190818152602001828054610af290611e06565b8015610b3d5780601f10610b1457610100808354040283529160200191610b3d565b820191905f5260205f20905b815481529060010190602001808311610b2057829003601f168201915b505050505081526020019060010190610aa9565b6060601a805480602002602001604051908101604052809291908181526020015f905b8282101561053e575f84815260209081902060408051808201825260028602909201805473ffffffffffffffffffffffffffffffffffffffff168352600181018054835181870281018701909452808452939491938583019392830182828015610c3c57602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191681526020019060040190602082600301049283019260010382029150808411610be95790505b50505050508152505081526020019060010190610b74565b601c54602054602154604051625d09a760e41b815273ffffffffffffffffffffffffffffffffffffffff909316926305d09a709261064092601e92601f9291905f90600401611f2c565b60606017805480602002602001604051908101604052809291908181526020015f905b8282101561053e578382905f5260205f20018054610cde90611e06565b80601f0160208091040260200160405190810160405280929190818152602001828054610d0a90611e06565b8015610d555780601f10610d2c57610100808354040283529160200191610d55565b820191905f5260205f20905b815481529060010190602001808311610d3857829003601f168201915b505050505081526020019060010190610cc1565b6007545f90610100900460ff1615610d8a5750600754610100900460ff1690565b5f737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ea15760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190527f6661696c6564000000000000000000000000000000000000000000000000000082840152825180830384018152606083019093525f929091610e2d917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc491608001611fc9565b60408051601f1981840301815290829052610e4791611ffc565b5f604051808303815f865af19150503d805f8114610e80576040519150601f19603f3d011682016040523d82523d5f602084013e610e85565b606091505b5091505080806020019051810190610e9d9190612007565b9150505b919050565b606060138054806020026020016040519081016040528092919081815260200182805480156103f457602002820191905f5260205f2090815473ffffffffffffffffffffffffffffffffffffffff1681526001909101906020018083116103c9575050505050905090565b604080518082018252601381527f42616420696e636c7573696f6e2070726f6f6600000000000000000000000000602082015290517ff28dceb3000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d9163f28dceb391610f929190600401611b0c565b5f604051808303815f87803b158015610fa9575f5ffd5b505af1158015610fbb573d5f5f3e3d5ffd5b5050601c54602054602154604051625d09a760e41b815273ffffffffffffffffffffffffffffffffffffffff90931694506305d09a70935061064092601e9291905f9060040161202d565b604080518082018252601681527f426164206d65726b6c652061727261792070726f6f6600000000000000000000602082015290517ff28dceb3000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d9163f28dceb3916110879190600401611b0c565b5f604051808303815f87803b15801561109e575f5ffd5b505af11580156110b0573d5f5f3e3d5ffd5b5050601c54602054602154604051625d09a760e41b815273ffffffffffffffffffffffffffffffffffffffff90931694506305d09a70935061064092601e9291905f90600401612076565b601c546040517ff58db06f0000000000000000000000000000000000000000000000000000000081525f60048201819052602482015273ffffffffffffffffffffffffffffffffffffffff9091169063f58db06f906044015f604051808303815f87803b15801561116a575f5ffd5b505af115801561117c573d5f5f3e3d5ffd5b5050604080518082018252601b81527f47434420646f6573206e6f7420636f6e6669726d206865616465720000000000602082015290517ff28dceb3000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d935063f28dceb392506112019190600401611b0c565b5f604051808303815f87803b158015611218575f5ffd5b505af115801561122a573d5f5f3e3d5ffd5b5050601c54602054602154604051625d09a760e41b815273ffffffffffffffffffffffffffffffffffffffff90931694506305d09a70935061064092601e92601f9290915f90600401611f2c565b60606103178484846040518060400160405280600681526020017f686569676874000000000000000000000000000000000000000000000000000081525061156a565b60606112c78484611d9b565b67ffffffffffffffff8111156112df576112df61196f565b60405190808252806020026020018201604052801561131257816020015b60608152602001906001900390816112fd5790505b509050835b83811015611413576113e58661132c836116b8565b8560405160200161133f939291906120bf565b604051602081830303815290604052601d805461135b90611e06565b80601f016020809104026020016040519081016040528092919081815260200182805461138790611e06565b80156113d25780601f106113a9576101008083540402835291602001916113d2565b820191905f5260205f20905b8154815290600101906020018083116113b557829003601f168201915b50505050506117e990919063ffffffff16565b826113f08784611d9b565b8151811061140057611400611dae565b6020908102919091010152600101611317565b50949350505050565b60606114288484611d9b565b67ffffffffffffffff8111156114405761144061196f565b604051908082528060200260200182016040528015611469578160200160208202803683370190505b509050835b838110156114135761153c86611483836116b8565b85604051602001611496939291906120bf565b604051602081830303815290604052601d80546114b290611e06565b80601f01602080910402602001604051908101604052809291908181526020018280546114de90611e06565b80156115295780601f1061150057610100808354040283529160200191611529565b820191905f5260205f20905b81548152906001019060200180831161150c57829003601f168201915b505050505061188890919063ffffffff16565b826115478784611d9b565b8151811061155757611557611dae565b602090810291909101015260010161146e565b60606115768484611d9b565b67ffffffffffffffff81111561158e5761158e61196f565b6040519080825280602002602001820160405280156115b7578160200160208202803683370190505b509050835b838110156114135761168a866115d1836116b8565b856040516020016115e4939291906120bf565b604051602081830303815290604052601d805461160090611e06565b80601f016020809104026020016040519081016040528092919081815260200182805461162c90611e06565b80156116775780601f1061164e57610100808354040283529160200191611677565b820191905f5260205f20905b81548152906001019060200180831161165a57829003601f168201915b505050505061191b90919063ffffffff16565b826116958784611d9b565b815181106116a5576116a5611dae565b60209081029190910101526001016115bc565b6060815f036116fa57505060408051808201909152600181527f3000000000000000000000000000000000000000000000000000000000000000602082015290565b815f5b8115611723578061170d8161215c565b915061171c9050600a836121c0565b91506116fd565b5f8167ffffffffffffffff81111561173d5761173d61196f565b6040519080825280601f01601f191660200182016040528015611767576020820181803683370190505b5090505b84156103175761177c600183611d9b565b9150611789600a866121d3565b611794906030611f71565b60f81b8183815181106117a9576117a9611dae565b60200101907effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff191690815f1a9053506117e2600a866121c0565b945061176b565b6040517ffd921be8000000000000000000000000000000000000000000000000000000008152606090737109709ecfa91a80626ff3989d68f67f5b1dd12d9063fd921be89061183e90869086906004016121e6565b5f60405180830381865afa158015611858573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261187f9190810190612213565b90505b92915050565b6040517f1777e59d0000000000000000000000000000000000000000000000000000000081525f90737109709ecfa91a80626ff3989d68f67f5b1dd12d90631777e59d906118dc90869086906004016121e6565b602060405180830381865afa1580156118f7573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061187f9190612288565b6040517faddde2b60000000000000000000000000000000000000000000000000000000081525f90737109709ecfa91a80626ff3989d68f67f5b1dd12d9063addde2b6906118dc90869086906004016121e6565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b604051601f8201601f1916810167ffffffffffffffff811182821017156119c5576119c561196f565b604052919050565b5f67ffffffffffffffff8211156119e6576119e661196f565b50601f01601f191660200190565b5f5f5f60608486031215611a06575f5ffd5b833567ffffffffffffffff811115611a1c575f5ffd5b8401601f81018613611a2c575f5ffd5b8035611a3f611a3a826119cd565b61199c565b818152876020838501011115611a53575f5ffd5b816020840160208301375f602092820183015297908601359650604090950135949350505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015611b0057603f19878603018452611aeb858351611a7b565b94506020938401939190910190600101611acf565b50929695505050505050565b602081525f61187f6020830184611a7b565b602080825282518282018190525f918401906040840190835b81811015611b6b57835173ffffffffffffffffffffffffffffffffffffffff16835260209384019390920191600101611b37565b509095945050505050565b5f82825180855260208501945060208160051b830101602085015f5b83811015611bc457601f19858403018852611bae838351611a7b565b6020988901989093509190910190600101611b92565b50909695505050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015611b0057603f19878603018452815173ffffffffffffffffffffffffffffffffffffffff81511686526020810151905060406020870152611c3e6040870182611b76565b9550506020938401939190910190600101611bf6565b602080825282518282018190525f918401906040840190835b81811015611b6b578351835260209384019390920191600101611c6d565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015611b0057868503603f190184528151805173ffffffffffffffffffffffffffffffffffffffff168652602090810151604082880181905281519088018190529101905f9060608801905b80831015611d44577fffffffff000000000000000000000000000000000000000000000000000000008451168252602082019150602084019350600183019250611cff565b50965050506020938401939190910190600101611cb1565b602081525f61187f6020830184611b76565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b8181038181111561188257611882611d6e565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f81518060208401855e5f93019283525090919050565b5f610317611e008386611ddb565b84611ddb565b600181811c90821680611e1a57607f821691505b602082108103611e51577f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b50919050565b80545f90600181811c90821680611e6f57607f821691505b602082108103611ea6577f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b81865260208601818015611ec15760018114611ef557611f21565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff008516825283151560051b82019550611f21565b5f878152602090205f5b85811015611f1b57815484820152600190910190602001611eff565b83019650505b505050505092915050565b60a081525f611f3e60a0830188611e57565b8281036020840152611f508188611e57565b60408401969096525050606081019290925260ff1660809091015292915050565b8082018082111561188257611882611d6e565b60a081525f611f9660a0830187611e57565b8281036020840152611fa88187611e57565b9150505f604083015283606083015260ff8316608083015295945050505050565b7fffffffff00000000000000000000000000000000000000000000000000000000831681525f6103176004830184611ddb565b5f61187f8284611ddb565b5f60208284031215612017575f5ffd5b81518015158114612026575f5ffd5b9392505050565b60a081525f61203f60a0830187611e57565b8281036020840152602081525f60208201526040810191505084604083015283606083015260ff8316608083015295945050505050565b60a081525f61208860a0830187611e57565b8281036020840152600181525f60208201526040810191505084604083015283606083015260ff8316608083015295945050505050565b7f2e0000000000000000000000000000000000000000000000000000000000000081525f6120f06001830186611ddb565b7f5b0000000000000000000000000000000000000000000000000000000000000081526121206001820186611ddb565b90507f5d2e00000000000000000000000000000000000000000000000000000000000081526121526002820185611ddb565b9695505050505050565b5f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff820361218c5761218c611d6e565b5060010190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f826121ce576121ce612193565b500490565b5f826121e1576121e1612193565b500690565b604081525f6121f86040830185611a7b565b828103602084015261220a8185611a7b565b95945050505050565b5f60208284031215612223575f5ffd5b815167ffffffffffffffff811115612239575f5ffd5b8201601f81018413612249575f5ffd5b8051612257611a3a826119cd565b81815285602083850101111561226b575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b5f60208284031215612298575f5ffd5b505191905056fea26469706673582212208bbb5ffd8c8f438ee4175ce418304129f214b7a1b6799bd9b080a24dbbbe514c64736f6c634300081b0033608060405234801561000f575f5ffd5b506040516128d73803806128d783398101604081905261002e9161032b565b82828282828261003f835160501490565b6100845760405162461bcd60e51b81526020600482015260116024820152704261642067656e6573697320626c6f636b60781b60448201526064015b60405180910390fd5b5f61008e84610166565b905062ffffff8216156101095760405162461bcd60e51b815260206004820152603d60248201527f506572696f64207374617274206861736820646f6573206e6f7420686176652060448201527f776f726b2e2048696e743a2077726f6e672062797465206f726465723f000000606482015260840161007b565b5f818155600182905560028290558181526004602052604090208390556101326107e0846103fe565b61013c9084610425565b5f8381526004602052604090205561015384610226565b600555506105bd98505050505050505050565b5f600280836040516101789190610438565b602060405180830381855afa158015610193573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906101b6919061044e565b6040516020016101c891815260200190565b60408051601f19818403018152908290526101e291610438565b602060405180830381855afa1580156101fd573d5f5f3e3d5ffd5b5050506040513d601f19601f82011682018060405250810190610220919061044e565b92915050565b5f61022061023383610238565b610243565b5f6102208282610253565b5f61022061ffff60d01b836102f7565b5f8061026a610263846048610465565b8590610309565b60e81c90505f8461027c85604b610465565b8151811061028c5761028c610478565b016020015160f81c90505f6102be835f60108262ffffff16901c8261ff001660108462ffffff16901b17179050919050565b62ffffff1690505f6102d160038461048c565b60ff1690506102e281610100610588565b6102ec9083610593565b979650505050505050565b5f61030282846105aa565b9392505050565b5f6103028383016020015190565b634e487b7160e01b5f52604160045260245ffd5b5f5f5f6060848603121561033d575f5ffd5b83516001600160401b03811115610352575f5ffd5b8401601f81018613610362575f5ffd5b80516001600160401b0381111561037b5761037b610317565b604051601f8201601f19908116603f011681016001600160401b03811182821017156103a9576103a9610317565b6040528181528282016020018810156103c0575f5ffd5b8160208401602083015e5f6020928201830152908601516040909601519097959650949350505050565b634e487b7160e01b5f52601260045260245ffd5b5f8261040c5761040c6103ea565b500690565b634e487b7160e01b5f52601160045260245ffd5b8181038181111561022057610220610411565b5f82518060208501845e5f920191825250919050565b5f6020828403121561045e575f5ffd5b5051919050565b8082018082111561022057610220610411565b634e487b7160e01b5f52603260045260245ffd5b60ff828116828216039081111561022057610220610411565b6001815b60018411156104e0578085048111156104c4576104c4610411565b60018416156104d257908102905b60019390931c9280026104a9565b935093915050565b5f826104f657506001610220565b8161050257505f610220565b816001811461051857600281146105225761053e565b6001915050610220565b60ff84111561053357610533610411565b50506001821b610220565b5060208310610133831016604e8410600b8410161715610561575081810a610220565b61056d5f1984846104a5565b805f190482111561058057610580610411565b029392505050565b5f61030283836104e8565b808202811582820484141761022057610220610411565b5f826105b8576105b86103ea565b500490565b61230d806105ca5f395ff3fe608060405234801561000f575f5ffd5b50600436106100fb575f3560e01c806370d53c1811610093578063b985621a11610063578063b985621a146101ed578063c58242cd14610200578063e3d8d8d814610208578063f58db06f1461020f575f5ffd5b806370d53c181461019757806374c3a3a9146101b45780637fa637fc146101c7578063b25b9b00146101da575f5ffd5b80632e4f161a116100ce5780632e4f161a1461013b57806330017b3b1461015e57806360b5c3901461017157806365da41b914610184575f5ffd5b806305d09a70146100ff578063113764be146101145780631910d9731461012b5780632b97be2414610133575b5f5ffd5b61011261010d366004611d2a565b61027b565b005b6005545b6040519081526020015b60405180910390f35b600154610118565b600654610118565b61014e610149366004611dc4565b610563565b6040519015158152602001610122565b61011861016c366004611df3565b61057b565b61011861017f366004611e13565b61058f565b61014e610192366004611e2a565b610599565b61019f600481565b60405163ffffffff9091168152602001610122565b61014e6101c2366004611e96565b610745565b61014e6101d5366004611f17565b6108ba565b6101186101e8366004611fb6565b610a99565b61014e6101fb36600461202f565b610b16565b600254610118565b5f54610118565b61011261021d36600461206c565b600780547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000169215157fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff169290921761010091151591909102179055565b6102b987878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b2c92505050565b61030a5760405162461bcd60e51b815260206004820152601060248201527f4261642068656164657220626c6f636b0000000000000000000000000000000060448201526064015b60405180910390fd5b61034885858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b3392505050565b6103945760405162461bcd60e51b815260206004820152601660248201527f426164206d65726b6c652061727261792070726f6f66000000000000000000006044820152606401610301565b610413836103d689898080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b4992505050565b87878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250889250610b55915050565b61045f5760405162461bcd60e51b815260206004820152601360248201527f42616420696e636c7573696f6e2070726f6f66000000000000000000000000006044820152606401610301565b5f61049e88888080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b8792505050565b90505f6104aa60025490565b90506104b8828260f0610c5f565b6105045760405162461bcd60e51b815260206004820152601b60248201527f47434420646f6573206e6f7420636f6e6669726d2068656164657200000000006044820152606401610301565b60ff8316600810156105585760405162461bcd60e51b815260206004820152601a60248201527f496e73756666696369656e7420636f6e6669726d6174696f6e730000000000006044820152606401610301565b505050505050505050565b5f61057085858585610c8f565b90505b949350505050565b5f6105868383610d29565b90505b92915050565b5f61058982610d9b565b5f6105d883838080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610e4992505050565b61064a5760405162461bcd60e51b815260206004820152602b60248201527f486561646572206172726179206c656e677468206d757374206265206469766960448201527f7369626c652062792038300000000000000000000000000000000000000000006064820152608401610301565b61068885858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b2c92505050565b6106d45760405162461bcd60e51b815260206004820152601760248201527f416e63686f72206d7573742062652038302062797465730000000000000000006044820152606401610301565b61057085858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525050604080516020601f890181900481028201810190925287815292508791508690819084018382808284375f9201829052509250610e58915050565b5f61078484848080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b2c92505050565b80156107c957506107c986868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b2c92505050565b61083b5760405162461bcd60e51b815260206004820152602e60248201527f42616420617267732e20436865636b2068656164657220616e6420617272617960448201527f2062797465206c656e677468732e0000000000000000000000000000000000006064820152608401610301565b6108af8787878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525050604080516020601f8b0181900481028201810190925289815292508991508890819084018382808284375f92019190915250889250611245915050565b979650505050505050565b5f6108f987878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b2c92505050565b801561093e575061093e85858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b2c92505050565b8015610983575061098383838080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610e4992505050565b6109f55760405162461bcd60e51b815260206004820152602e60248201527f42616420617267732e20436865636b2068656164657220616e6420617272617960448201527f2062797465206c656e677468732e0000000000000000000000000000000000006064820152608401610301565b6108af87878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525050604080516020601f8b0181900481028201810190925289815292508991508890819084018382808284375f9201919091525050604080516020601f8a0181900481028201810190925288815292508891508790819084018382808284375f920191909152506114e292505050565b5f610b0c8686868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525050604080516020601f8a0181900481028201810190925288815292508891508790819084018382808284375f9201919091525061177492505050565b9695505050505050565b5f610b22848484610c5f565b90505b9392505050565b5160501490565b5f60208251610b4291906120ca565b1592915050565b60448101515f90610589565b5f8385148015610b63575081155b8015610b6e57508251155b15610b7b57506001610573565b61057085848685611905565b5f60028083604051610b9991906120dd565b602060405180830381855afa158015610bb4573d5f5f3e3d5ffd5b5050506040513d601f19601f82011682018060405250810190610bd791906120f3565b604051602001610be991815260200190565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe081840301815290829052610c21916120dd565b602060405180830381855afa158015610c3c573d5f5f3e3d5ffd5b5050506040513d601f19601f8201168201806040525081019061058991906120f3565b6007545f9060ff1615610c7d5750600754610100900460ff16610b25565b610c888484846119aa565b9050610b25565b5f8385148015610c9e57508285145b15610cab57506001610573565b838381815f5b86811015610cf357898314610cd2575f838152600360205260409020549294505b898214610ceb575f828152600360205260409020549193505b600101610cb1565b50828403610d07575f945050505050610573565b808214610d1a575f945050505050610573565b50600198975050505050505050565b5f82815b83811015610d4d575f918252600360205260409091205490600101610d2d565b50806105865760405162461bcd60e51b815260206004820152601060248201527f556e6b6e6f776e20616e636573746f72000000000000000000000000000000006044820152606401610301565b5f8082815b610dac60046001612137565b63ffffffff16811015610e00575f828152600460205260408120549350839003610de5575f918252600360205260409091205490610df8565b610def8184612153565b95945050505050565b600101610da0565b5060405162461bcd60e51b815260206004820152600d60248201527f556e6b6e6f776e20626c6f636b000000000000000000000000000000000000006044820152606401610301565b5f60508251610b4291906120ca565b5f5f610e6385610b87565b90505f610e6f82610d9b565b90505f610e7b866119eb565b90508480610e90575080610e8e886119eb565b145b610f015760405162461bcd60e51b8152602060048201526024808201527f556e6578706563746564207265746172676574206f6e2065787465726e616c2060448201527f63616c6c000000000000000000000000000000000000000000000000000000006064820152608401610301565b85515f908190815b8181101561120257610f1c605082612166565b610f27906001612153565b610f319087612153565b9350610f3f8a8260506119f6565b5f818152600360205260409020549093506111155784611095845f8190506008817eff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff16901b600882901c7eff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff161790506010817dffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff16901b601082901c7dffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff161790506020817bffffffff00000000ffffffff00000000ffffffff00000000ffffffff16901b602082901c7bffffffff00000000ffffffff00000000ffffffff00000000ffffffff1617905060408177ffffffffffffffff0000000000000000ffffffffffffffff16901b604082901c77ffffffffffffffff0000000000000000ffffffffffffffff16179050608081901b608082901c179050919050565b11156110e35760405162461bcd60e51b815260206004820152601b60248201527f48656164657220776f726b20697320696e73756666696369656e7400000000006044820152606401610301565b5f8381526003602052604090208790556110fe6004856120ca565b5f03611115575f8381526004602052604090208490555b846111208b83611a1b565b1461116d5760405162461bcd60e51b815260206004820152601b60248201527f546172676574206368616e67656420756e65787065637465646c7900000000006044820152606401610301565b866111788b83611ab4565b146111eb5760405162461bcd60e51b815260206004820152602660248201527f4865616465727320646f206e6f7420666f726d206120636f6e73697374656e7460448201527f20636861696e00000000000000000000000000000000000000000000000000006064820152608401610301565b8296506050816111fb9190612153565b9050610f09565b508161120d8b610b87565b6040517ff90e4f1d9cd0dd55e339411cbc9b152482307c3a23ed64715e4a2858f641a3f5905f90a35060019998505050505050505050565b5f6107e08211156112be5760405162461bcd60e51b815260206004820152603360248201527f526571756573746564206c696d69742069732067726561746572207468616e2060448201527f3120646966666963756c747920706572696f64000000000000000000000000006064820152608401610301565b5f6112c884610b87565b90505f6112d486610b87565b905060015481146113275760405162461bcd60e51b815260206004820181905260248201527f50617373656420696e2062657374206973206e6f742062657374206b6e6f776e6044820152606401610301565b5f828152600360205260409020546113815760405162461bcd60e51b815260206004820152601360248201527f4e6577206265737420697320756e6b6e6f776e000000000000000000000000006044820152606401610301565b61138f876001548487610c8f565b6114015760405162461bcd60e51b815260206004820152602960248201527f416e636573746f72206d75737420626520686561766965737420636f6d6d6f6e60448201527f20616e636573746f7200000000000000000000000000000000000000000000006064820152608401610301565b8161140d888888611774565b146114805760405162461bcd60e51b815260206004820152603360248201527f4e65772062657374206861736820646f6573206e6f742068617665206d6f726560448201527f20776f726b207468616e2070726576696f7573000000000000000000000000006064820152608401610301565b600182905560028790555f61149486611acc565b905060055481146114a55760058190555b8783837f3cc13de64df0f0239626235c51a2da251bbc8c85664ecce39263da3ee03f606c60405160405180910390a4506001979650505050505050565b5f5f6114f56114f086610b87565b610d9b565b90505f6115046114f086610b87565b90506115126107e0826120ca565b6107df146115885760405162461bcd60e51b815260206004820152603d60248201527f4d7573742070726f7669646520746865206c61737420686561646572206f662060448201527f74686520636c6f73696e6720646966666963756c747920706572696f640000006064820152608401610301565b611594826107df612153565b81146116085760405162461bcd60e51b815260206004820152602860248201527f4d7573742070726f766964652065786163746c79203120646966666963756c7460448201527f7920706572696f640000000000000000000000000000000000000000000000006064820152608401610301565b61161185611acc565b61161a87611acc565b1461168d5760405162461bcd60e51b815260206004820152602760248201527f506572696f642068656164657220646966666963756c7469657320646f206e6f60448201527f74206d61746368000000000000000000000000000000000000000000000000006064820152608401610301565b5f611697856119eb565b90505f6116c96116a6896119eb565b6116af8a611ade565b63ffffffff166116be8a611ade565b63ffffffff16611b11565b9050818183161461171c5760405162461bcd60e51b815260206004820152601960248201527f496e76616c69642072657461726765742070726f7669646564000000000000006044820152606401610301565b5f61172689611acc565b9050806006541415801561175057506107e0611743600154610d9b565b61174d9190612179565b84115b1561175b5760068190555b61176788886001610e58565b9998505050505050505050565b5f5f61177f85610d9b565b90505f61178e6114f086610b87565b90505f61179d6114f086610b87565b90508282101580156117af5750828110155b6118215760405162461bcd60e51b815260206004820152603060248201527f412064657363656e64616e74206865696768742069732062656c6f772074686560448201527f20616e636573746f7220686569676874000000000000000000000000000000006064820152608401610301565b5f61182e6107e0856120ca565b61183a856107e0612153565b6118449190612179565b90508083108183108115826118565750805b156118715761186489610b87565b9650505050505050610b25565b81801561187c575080155b1561188a5761186488610b87565b8180156118945750805b156118b857838510156118af576118aa88610b87565b611864565b61186489610b87565b6118c188611acc565b6118cd6107e0866120ca565b6118d7919061218c565b6118e08a611acc565b6118ec6107e0886120ca565b6118f6919061218c565b10156118af5761186488610b87565b5f6020845161191491906120ca565b1561192057505f610573565b83515f0361192f57505f610573565b81855f5b865181101561199d576119476002846120ca565b60010361196b5761196461195e8883016020015190565b83611b99565b9150611984565b6119818261197c8984016020015190565b611b99565b91505b60019290921c91611996602082612153565b9050611933565b5090931495945050505050565b5f82815b838110156119e0578582036119c857600192505050610b25565b5f9182526003602052604090912054906001016119ae565b505f95945050505050565b5f610589825f611a1b565b5f60205f8385602001870160025afa5060205f60205f60025afa50505f519392505050565b5f80611a32611a2b846048612153565b8590611ba4565b60e81c90505f84611a4485604b612153565b81518110611a5457611a546121a3565b016020015160f81c90505f611a86835f60108262ffffff16901c8261ff001660108462ffffff16901b17179050919050565b62ffffff1690505f611a996003846121d0565b60ff169050611aaa816101006122cc565b6108af908361218c565b5f610586611ac3836004612153565b84016020015190565b5f610589611ad9836119eb565b611bb2565b5f610589611aeb83611bd9565b60d881901c63ff00ff001662ff00ff60e89290921c9190911617601081811b91901c1790565b5f80611b1d8385611be5565b9050611b2d621275006004611c40565b811015611b4557611b42621275006004611c40565b90505b611b53621275006004611c4b565b811115611b6b57611b68621275006004611c4b565b90505b5f611b8382611b7d8862010000611c40565b90611c4b565b9050610b0c62010000611b7d8362127500611c40565b5f6105868383611cbe565b5f6105868383016020015190565b5f6105897bffff000000000000000000000000000000000000000000000000000083611c40565b5f610589826044611ba4565b5f82821115611c365760405162461bcd60e51b815260206004820152601d60248201527f556e646572666c6f7720647572696e67207375627472616374696f6e2e0000006044820152606401610301565b6105868284612179565b5f6105868284612166565b5f825f03611c5a57505f610589565b611c64828461218c565b905081611c718483612166565b146105895760405162461bcd60e51b815260206004820152601f60248201527f4f766572666c6f7720647572696e67206d756c7469706c69636174696f6e2e006044820152606401610301565b5f825f528160205260205f60405f60025afa5060205f60205f60025afa50505f5192915050565b5f5f83601f840112611cf5575f5ffd5b50813567ffffffffffffffff811115611d0c575f5ffd5b602083019150836020828501011115611d23575f5ffd5b9250929050565b5f5f5f5f5f5f5f60a0888a031215611d40575f5ffd5b873567ffffffffffffffff811115611d56575f5ffd5b611d628a828b01611ce5565b909850965050602088013567ffffffffffffffff811115611d81575f5ffd5b611d8d8a828b01611ce5565b9096509450506040880135925060608801359150608088013560ff81168114611db4575f5ffd5b8091505092959891949750929550565b5f5f5f5f60808587031215611dd7575f5ffd5b5050823594602084013594506040840135936060013592509050565b5f5f60408385031215611e04575f5ffd5b50508035926020909101359150565b5f60208284031215611e23575f5ffd5b5035919050565b5f5f5f5f60408587031215611e3d575f5ffd5b843567ffffffffffffffff811115611e53575f5ffd5b611e5f87828801611ce5565b909550935050602085013567ffffffffffffffff811115611e7e575f5ffd5b611e8a87828801611ce5565b95989497509550505050565b5f5f5f5f5f5f60808789031215611eab575f5ffd5b86359550602087013567ffffffffffffffff811115611ec8575f5ffd5b611ed489828a01611ce5565b909650945050604087013567ffffffffffffffff811115611ef3575f5ffd5b611eff89828a01611ce5565b979a9699509497949695606090950135949350505050565b5f5f5f5f5f5f60608789031215611f2c575f5ffd5b863567ffffffffffffffff811115611f42575f5ffd5b611f4e89828a01611ce5565b909750955050602087013567ffffffffffffffff811115611f6d575f5ffd5b611f7989828a01611ce5565b909550935050604087013567ffffffffffffffff811115611f98575f5ffd5b611fa489828a01611ce5565b979a9699509497509295939492505050565b5f5f5f5f5f60608688031215611fca575f5ffd5b85359450602086013567ffffffffffffffff811115611fe7575f5ffd5b611ff388828901611ce5565b909550935050604086013567ffffffffffffffff811115612012575f5ffd5b61201e88828901611ce5565b969995985093965092949392505050565b5f5f5f60608486031215612041575f5ffd5b505081359360208301359350604090920135919050565b80358015158114612067575f5ffd5b919050565b5f5f6040838503121561207d575f5ffd5b61208683612058565b915061209460208401612058565b90509250929050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f826120d8576120d861209d565b500690565b5f82518060208501845e5f920191825250919050565b5f60208284031215612103575f5ffd5b5051919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b63ffffffff81811683821601908111156105895761058961210a565b808201808211156105895761058961210a565b5f826121745761217461209d565b500490565b818103818111156105895761058961210a565b80820281158282048414176105895761058961210a565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b60ff82811682821603908111156105895761058961210a565b6001815b6001841115612224578085048111156122085761220861210a565b600184161561221657908102905b60019390931c9280026121ed565b935093915050565b5f8261223a57506001610589565b8161224657505f610589565b816001811461225c576002811461226657612282565b6001915050610589565b60ff8411156122775761227761210a565b50506001821b610589565b5060208310610133831016604e8410600b84101617156122a5575081810a610589565b6122b15f1984846121e9565b805f19048211156122c4576122c461210a565b029392505050565b5f610586838361222c56fea2646970667358221220ae76a72a2b3e046106f2f9aab5e5345aa0093c8c04e3c2fa8c1a8f4ea03a956564736f6c634300081b00330000002073bd2184edd9c4fc76642ea6754ee40136970efc10c4190000000000000000000296ef123ea96da5cf695f22bf7d94be87d49db1ad7ac371ac43c4da4161c8c216349c5ba11928170d38782be35a0d6de94b656694589964a252957e4673a9fb1d2f8b4a92e3f0a7bb654fddb94e5a1e6d7f7f499fd1be5dd30a73bf5584bf137da5fdd77cc21aeb95b9e35788894be019284bd4fbed6dd6118ac2cb6d26bc4be4e423f55a3a48f2874d8d02a65d9c87d07de21d4dfe7b0a9f4a23cc9a58373e9e6931fefdb5afade5df54c91104048df1ee999240617984e18b6f931e2373673d0195b8c6987d7ff7650d5ce53bcec46e13ab4f2da1146a7fc621ee672f62bc22742486392d75e55e67b09960c3386a0b49e75f1723d6ab28ac9a2028a0c72866e2111d79d4817b88e17c821937847768d92837bae3832bb8e5a4ab4434b97e00a6c10182f211f592409068d6f5652400d9a3d1cc150a7fb692e874cc42d76bdafc842f2fe0f835a7c24d2d60c109b187d64571efbaa8047be85821f8e67e0e85f2f5894bc63d00c2ed9d64
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x07\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x0B\x80T\x90\x91\x16\x90\x91\x17\x90Ua\x01\0`@R`P`\x80\x81\x81R\x90aT\xF0`\xA09`\x1E\x90a\0=\x90\x82a\x06\xC2V[P`@Q\x80a\x01`\x01`@R\x80a\x01@\x81R` \x01aU@a\x01@\x919`\x1F\x90a\0g\x90\x82a\x06\xC2V[P\x7FH\xE5\xA1\xA0\xE6\x16\xD8\xFD\x92\xB4\xEF\"\x8CBN\x0C\x81g\x99\xA2V\xC6\xA9\x08\x92\x19\\\xCF\xC53\0\xD6` Ua\x01\x19`!U4\x80\x15a\0\x9DW__\xFD[P`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k42\xB0\xB22\xB99\x9759\xB7\xB7`\xA1\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x05\xCC\xEC\xAD\xCC\xAEm.e\xCD\x0C\xAF`\xA3\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n\x0B\x99\xD9[\x99\\\xDA\\\xCB\x9A\x19ZY\xDA\x1D`\x8A\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q.genesis.digest_le`p\x1B\x81RP_\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1C`\x01`\x01`\xA0\x1B\x03\x16c\xD90\xA0\xE6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x99W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x01\xC0\x91\x90\x81\x01\x90a\x07\xF1V[\x90P_\x81\x86`@Q` \x01a\x01\xD6\x92\x91\x90a\x08TV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rc`\xF9\xBB\x11`\xE0\x1B\x82R\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c`\xF9\xBB\x11\x90a\x02\x1D\x90\x84\x90`\x04\x01a\x08\xC6V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x027W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02^\x91\x90\x81\x01\x90a\x07\xF1V[`\x1D\x90a\x02k\x90\x82a\x06\xC2V[Pa\x03\x02\x85`\x1D\x80Ta\x02}\x90a\x06>V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xA9\x90a\x06>V[\x80\x15a\x02\xF4W\x80`\x1F\x10a\x02\xCBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xF4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xD7W\x82\x90\x03`\x1F\x16\x82\x01\x91[P\x93\x94\x93PPa\x04\xE4\x91PPV[a\x03\x98\x85`\x1D\x80Ta\x03\x13\x90a\x06>V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03?\x90a\x06>V[\x80\x15a\x03\x8AW\x80`\x1F\x10a\x03aWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\x8AV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03mW\x82\x90\x03`\x1F\x16\x82\x01\x91[P\x93\x94\x93PPa\x05h\x91PPV[a\x04.\x85`\x1D\x80Ta\x03\xA9\x90a\x06>V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xD5\x90a\x06>V[\x80\x15a\x04 W\x80`\x1F\x10a\x03\xF7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04 V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\x03W\x82\x90\x03`\x1F\x16\x82\x01\x91[P\x93\x94\x93PPa\x05\xE2\x91PPV[`@Qa\x04:\x90a\x06\x1DV[a\x04F\x93\x92\x91\x90a\x08\xD8V[`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x04_W=__>=_\xFD[P`\x1C\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90U`@Qc\xF5\x8D\xB0o`\xE0\x1B\x81R`\x01`\x04\x82\x01\x81\x90R`$\x82\x01R\x90\x96Pc\xF5\x8D\xB0o\x95P`D\x01\x93Pa\x04\xB2\x92PPPV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x04\xC9W__\xFD[PZ\xF1\x15\x80\x15a\x04\xDBW=__>=_\xFD[PPPPa\t7V[`@Qc\x1F\xB2C}`\xE3\x1B\x81R``\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xFD\x92\x1B\xE8\x90a\x05 \x90\x86\x90\x86\x90`\x04\x01a\x08\xFCV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05:W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05a\x91\x90\x81\x01\x90a\x07\xF1V[\x93\x92PPPV[`@QcV\xEE\xF1[`\xE1\x1B\x81R_\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xAD\xDD\xE2\xB6\x90a\x05\xA3\x90\x86\x90\x86\x90`\x04\x01a\x08\xFCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xBEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05a\x91\x90a\t V[`@Qc\x17w\xE5\x9D`\xE0\x1B\x81R_\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x17w\xE5\x9D\x90a\x05\xA3\x90\x86\x90\x86\x90`\x04\x01a\x08\xFCV[a(\xD7\x80a,\x19\x839\x01\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x06RW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x06pWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x06\xBDW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x06\x9BWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x06\xBAW_\x81U`\x01\x01a\x06\xA7V[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06\xDBWa\x06\xDBa\x06*V[a\x06\xEF\x81a\x06\xE9\x84Ta\x06>V[\x84a\x06vV[` `\x1F\x82\x11`\x01\x81\x14a\x07!W_\x83\x15a\x07\nWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x06\xBAV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x07PW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x070V[P\x84\x82\x10\x15a\x07mW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[_\x80`\x01`\x01`@\x1B\x03\x84\x11\x15a\x07\x95Wa\x07\x95a\x06*V[P`@Q`\x1F\x19`\x1F\x85\x01\x81\x16`?\x01\x16\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x07\xC3Wa\x07\xC3a\x06*V[`@R\x83\x81R\x90P\x80\x82\x84\x01\x85\x10\x15a\x07\xDAW__\xFD[\x83\x83` \x83\x01^_` \x85\x83\x01\x01RP\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x08\x01W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\x16W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x08&W__\xFD[a\x085\x84\x82Q` \x84\x01a\x07|V[\x94\x93PPPPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a\x08_\x82\x85a\x08=V[\x7F/test/fullRelay/testData/\0\0\0\0\0\0\0\x81Ra\x08\x8F`\x19\x82\x01\x85a\x08=V[\x95\x94PPPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x05a` \x83\x01\x84a\x08\x98V[``\x81R_a\x08\xEA``\x83\x01\x86a\x08\x98V[` \x83\x01\x94\x90\x94RP`@\x01R\x91\x90PV[`@\x81R_a\t\x0E`@\x83\x01\x85a\x08\x98V[\x82\x81\x03` \x84\x01Ra\x08\x8F\x81\x85a\x08\x98V[_` \x82\x84\x03\x12\x15a\t0W__\xFD[PQ\x91\x90PV[a\"\xD5\x80a\tD_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01yW_5`\xE0\x1C\x80c\x85\"l\x81\x11a\0\xD2W\x80c\xE2\x0C\x9Fq\x11a\0\x88W\x80c\xF1\x1D\\\xBC\x11a\0cW\x80c\xF1\x1D\\\xBC\x14a\x02\xACW\x80c\xFAv&\xD4\x14a\x02\xB4W\x80c\xFA\xD0k\x8F\x14a\x02\xC1W__\xFD[\x80c\xE2\x0C\x9Fq\x14a\x02\x94W\x80c\xE8\x9F\x84\x19\x14a\x02\x9CW\x80c\xF0\xC0\x83.\x14a\x02\xA4W__\xFD[\x80c\xB5+ X\x11a\0\xB8W\x80c\xB5+ X\x14a\x02lW\x80c\xB5P\x8A\xA9\x14a\x02tW\x80c\xBAAO\xA6\x14a\x02|W__\xFD[\x80c\x85\"l\x81\x14a\x02OW\x80c\x91j\x17\xC6\x14a\x02dW__\xFD[\x80c>^<#\x11a\x012W\x80cd\x1E\x14M\x11a\x01\rW\x80cd\x1E\x14M\x14a\x02*W\x80cf\xD9\xA9\xA0\x14a\x022W\x80ct\r\xBAt\x14a\x02GW__\xFD[\x80c>^<#\x14a\x01\xFAW\x80c?r\x86\xF4\x14a\x02\x02W\x80cD\xBA\xDB\xB6\x14a\x02\nW__\xFD[\x80c\x1E\xD7\x83\x1C\x11a\x01bW\x80c\x1E\xD7\x83\x1C\x14a\x01\xC6W\x80c*\xDE8\x80\x14a\x01\xDBW\x80c9B_\x8F\x14a\x01\xF0W__\xFD[\x80c\x08\x13\x85*\x14a\x01}W\x80c\x1C\r\xA8\x1F\x14a\x01\xA6W[__\xFD[a\x01\x90a\x01\x8B6`\x04a\x19\xF4V[a\x02\xD4V[`@Qa\x01\x9D\x91\x90a\x1A\xA9V[`@Q\x80\x91\x03\x90\xF3[a\x01\xB9a\x01\xB46`\x04a\x19\xF4V[a\x03\x1FV[`@Qa\x01\x9D\x91\x90a\x1B\x0CV[a\x01\xCEa\x03\x91V[`@Qa\x01\x9D\x91\x90a\x1B\x1EV[a\x01\xE3a\x03\xFEV[`@Qa\x01\x9D\x91\x90a\x1B\xD0V[a\x01\xF8a\x05GV[\0[a\x01\xCEa\x06nV[a\x01\xCEa\x06\xD9V[a\x02\x1Da\x02\x186`\x04a\x19\xF4V[a\x07DV[`@Qa\x01\x9D\x91\x90a\x1CTV[a\x01\xF8a\x07\x87V[a\x02:a\x08\x90V[`@Qa\x01\x9D\x91\x90a\x1C\x8BV[a\x01\xF8a\t\x93V[a\x02Wa\n\x86V[`@Qa\x01\x9D\x91\x90a\x1D\\V[a\x02:a\x0BQV[a\x01\xF8a\x0CTV[a\x02Wa\x0C\x9EV[a\x02\x84a\riV[`@Q\x90\x15\x15\x81R` \x01a\x01\x9DV[a\x01\xCEa\x0E\xA6V[a\x01\xF8a\x0F\x11V[a\x01\xF8a\x10\x06V[a\x01\xF8a\x10\xFBV[`\x07Ta\x02\x84\x90`\xFF\x16\x81V[a\x02\x1Da\x02\xCF6`\x04a\x19\xF4V[a\x12xV[``a\x03\x17\x84\x84\x84`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01\x7Fhex\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x12\xBBV[\x94\x93PPPPV[``_a\x03-\x85\x85\x85a\x02\xD4V[\x90P_[a\x03;\x85\x85a\x1D\x9BV[\x81\x10\x15a\x03\x88W\x82\x82\x82\x81Q\x81\x10a\x03UWa\x03Ua\x1D\xAEV[` \x02` \x01\x01Q`@Q` \x01a\x03n\x92\x91\x90a\x1D\xF2V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x92P`\x01\x01a\x031V[PP\x93\x92PPPV[```\x14\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03\xF4W` \x02\x82\x01\x91\x90_R` _ \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03\xC9W[PPPPP\x90P\x90V[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05>W_\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x05'W\x83\x82\x90_R` _ \x01\x80Ta\x04\x9C\x90a\x1E\x06V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xC8\x90a\x1E\x06V[\x80\x15a\x05\x13W\x80`\x1F\x10a\x04\xEAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\x13V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xF6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x04\x7FV[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x04!V[PPPP\x90P\x90V[`@\x80Q\x80\x82\x01\x82R`\x1A\x81R\x7FInsufficient confirmations\0\0\0\0\0\0` \x82\x01R\x90Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\x05\xC8\x91\x90`\x04\x01a\x1B\x0CV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05\xDFW__\xFD[PZ\xF1\x15\x80\x15a\x05\xF1W=__>=_\xFD[PP`\x1CT` T`!T`@Qb]\t\xA7`\xE4\x1B\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x94Pc\x05\xD0\x9Ap\x93Pa\x06@\x92`\x1E\x92`\x1F\x92\x90\x91`\t\x90`\x04\x01a\x1F,V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06VW__\xFD[PZ\xFA\x15\x80\x15a\x06hW=__>=_\xFD[PPPPV[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03\xF4W` \x02\x82\x01\x91\x90_R` _ \x90\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03\xC9WPPPPP\x90P\x90V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03\xF4W` \x02\x82\x01\x91\x90_R` _ \x90\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03\xC9WPPPPP\x90P\x90V[``a\x03\x17\x84\x84\x84`@Q\x80`@\x01`@R\x80`\t\x81R` \x01\x7Fdigest_le\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x14\x1CV[`@\x80Q\x80\x82\x01\x82R`\x13\x81R\x7FBad inclusion proof\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\x08\x08\x91\x90`\x04\x01a\x1B\x0CV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08\x1FW__\xFD[PZ\xF1\x15\x80\x15a\x081W=__>=_\xFD[PP`\x1CT` T`!Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x93Pc\x05\xD0\x9Ap\x92P`\x1E\x91`\x1F\x91\x90a\x08o\x90`\x01a\x1FqV[_`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06@\x95\x94\x93\x92\x91\x90a\x1F,V[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05>W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\t{W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\t(W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x08\xB3V[`@\x80Q\x80\x82\x01\x82R`\x13\x81R\x7FBad inclusion proof\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\n\x14\x91\x90`\x04\x01a\x1B\x0CV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\n+W__\xFD[PZ\xF1\x15\x80\x15a\n=W=__>=_\xFD[PP`\x1CT`!T`@Qb]\t\xA7`\xE4\x1B\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x93Pc\x05\xD0\x9Ap\x92Pa\x06@\x91`\x1E\x91`\x1F\x91_\x90`\x04\x01a\x1F\x84V[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05>W\x83\x82\x90_R` _ \x01\x80Ta\n\xC6\x90a\x1E\x06V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\xF2\x90a\x1E\x06V[\x80\x15a\x0B=W\x80`\x1F\x10a\x0B\x14Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B=V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\n\xA9V[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05>W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x0C<W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x0B\xE9W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x0BtV[`\x1CT` T`!T`@Qb]\t\xA7`\xE4\x1B\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x92c\x05\xD0\x9Ap\x92a\x06@\x92`\x1E\x92`\x1F\x92\x91\x90_\x90`\x04\x01a\x1F,V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05>W\x83\x82\x90_R` _ \x01\x80Ta\x0C\xDE\x90a\x1E\x06V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r\n\x90a\x1E\x06V[\x80\x15a\rUW\x80`\x1F\x10a\r,Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\rUV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x0C\xC1V[`\x07T_\x90a\x01\0\x90\x04`\xFF\x16\x15a\r\x8AWP`\x07Ta\x01\0\x90\x04`\xFF\x16\x90V[_sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0E\xA1W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R_\x92\x90\x91a\x0E-\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a\x1F\xC9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0EG\x91a\x1F\xFCV[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x0E\x80W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x0E\x85V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x0E\x9D\x91\x90a \x07V[\x91PP[\x91\x90PV[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03\xF4W` \x02\x82\x01\x91\x90_R` _ \x90\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03\xC9WPPPPP\x90P\x90V[`@\x80Q\x80\x82\x01\x82R`\x13\x81R\x7FBad inclusion proof\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\x0F\x92\x91\x90`\x04\x01a\x1B\x0CV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0F\xA9W__\xFD[PZ\xF1\x15\x80\x15a\x0F\xBBW=__>=_\xFD[PP`\x1CT` T`!T`@Qb]\t\xA7`\xE4\x1B\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x94Pc\x05\xD0\x9Ap\x93Pa\x06@\x92`\x1E\x92\x91\x90_\x90`\x04\x01a -V[`@\x80Q\x80\x82\x01\x82R`\x16\x81R\x7FBad merkle array proof\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\x10\x87\x91\x90`\x04\x01a\x1B\x0CV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10\x9EW__\xFD[PZ\xF1\x15\x80\x15a\x10\xB0W=__>=_\xFD[PP`\x1CT` T`!T`@Qb]\t\xA7`\xE4\x1B\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x94Pc\x05\xD0\x9Ap\x93Pa\x06@\x92`\x1E\x92\x91\x90_\x90`\x04\x01a vV[`\x1CT`@Q\x7F\xF5\x8D\xB0o\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_`\x04\x82\x01\x81\x90R`$\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\xF5\x8D\xB0o\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x11jW__\xFD[PZ\xF1\x15\x80\x15a\x11|W=__>=_\xFD[PP`@\x80Q\x80\x82\x01\x82R`\x1B\x81R\x7FGCD does not confirm header\0\0\0\0\0` \x82\x01R\x90Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x93Pc\xF2\x8D\xCE\xB3\x92Pa\x12\x01\x91\x90`\x04\x01a\x1B\x0CV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x12\x18W__\xFD[PZ\xF1\x15\x80\x15a\x12*W=__>=_\xFD[PP`\x1CT` T`!T`@Qb]\t\xA7`\xE4\x1B\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x94Pc\x05\xD0\x9Ap\x93Pa\x06@\x92`\x1E\x92`\x1F\x92\x90\x91_\x90`\x04\x01a\x1F,V[``a\x03\x17\x84\x84\x84`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7Fheight\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x15jV[``a\x12\xC7\x84\x84a\x1D\x9BV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xDFWa\x12\xDFa\x19oV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13\x12W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x12\xFDW\x90P[P\x90P\x83[\x83\x81\x10\x15a\x14\x13Wa\x13\xE5\x86a\x13,\x83a\x16\xB8V[\x85`@Q` \x01a\x13?\x93\x92\x91\x90a \xBFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x80Ta\x13[\x90a\x1E\x06V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\x87\x90a\x1E\x06V[\x80\x15a\x13\xD2W\x80`\x1F\x10a\x13\xA9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\xD2V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13\xB5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa\x17\xE9\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82a\x13\xF0\x87\x84a\x1D\x9BV[\x81Q\x81\x10a\x14\0Wa\x14\0a\x1D\xAEV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x13\x17V[P\x94\x93PPPPV[``a\x14(\x84\x84a\x1D\x9BV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14@Wa\x14@a\x19oV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14iW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x83[\x83\x81\x10\x15a\x14\x13Wa\x15<\x86a\x14\x83\x83a\x16\xB8V[\x85`@Q` \x01a\x14\x96\x93\x92\x91\x90a \xBFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x80Ta\x14\xB2\x90a\x1E\x06V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14\xDE\x90a\x1E\x06V[\x80\x15a\x15)W\x80`\x1F\x10a\x15\0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15)V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\x0CW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa\x18\x88\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82a\x15G\x87\x84a\x1D\x9BV[\x81Q\x81\x10a\x15WWa\x15Wa\x1D\xAEV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x14nV[``a\x15v\x84\x84a\x1D\x9BV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\x8EWa\x15\x8Ea\x19oV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x15\xB7W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x83[\x83\x81\x10\x15a\x14\x13Wa\x16\x8A\x86a\x15\xD1\x83a\x16\xB8V[\x85`@Q` \x01a\x15\xE4\x93\x92\x91\x90a \xBFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x80Ta\x16\0\x90a\x1E\x06V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x16,\x90a\x1E\x06V[\x80\x15a\x16wW\x80`\x1F\x10a\x16NWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x16wV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16ZW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa\x19\x1B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82a\x16\x95\x87\x84a\x1D\x9BV[\x81Q\x81\x10a\x16\xA5Wa\x16\xA5a\x1D\xAEV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x15\xBCV[``\x81_\x03a\x16\xFAWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81_[\x81\x15a\x17#W\x80a\x17\r\x81a!\\V[\x91Pa\x17\x1C\x90P`\n\x83a!\xC0V[\x91Pa\x16\xFDV[_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17=Wa\x17=a\x19oV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x17gW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x03\x17Wa\x17|`\x01\x83a\x1D\x9BV[\x91Pa\x17\x89`\n\x86a!\xD3V[a\x17\x94\x90`0a\x1FqV[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x17\xA9Wa\x17\xA9a\x1D\xAEV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81_\x1A\x90SPa\x17\xE2`\n\x86a!\xC0V[\x94Pa\x17kV[`@Q\x7F\xFD\x92\x1B\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R``\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xFD\x92\x1B\xE8\x90a\x18>\x90\x86\x90\x86\x90`\x04\x01a!\xE6V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18XW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x18\x7F\x91\x90\x81\x01\x90a\"\x13V[\x90P[\x92\x91PPV[`@Q\x7F\x17w\xE5\x9D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x17w\xE5\x9D\x90a\x18\xDC\x90\x86\x90\x86\x90`\x04\x01a!\xE6V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xF7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\x7F\x91\x90a\"\x88V[`@Q\x7F\xAD\xDD\xE2\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xAD\xDD\xE2\xB6\x90a\x18\xDC\x90\x86\x90\x86\x90`\x04\x01a!\xE6V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x19\xC5Wa\x19\xC5a\x19oV[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x19\xE6Wa\x19\xE6a\x19oV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[___``\x84\x86\x03\x12\x15a\x1A\x06W__\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\x1CW__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x1A,W__\xFD[\x805a\x1A?a\x1A:\x82a\x19\xCDV[a\x19\x9CV[\x81\x81R\x87` \x83\x85\x01\x01\x11\x15a\x1ASW__\xFD[\x81` \x84\x01` \x83\x017_` \x92\x82\x01\x83\x01R\x97\x90\x86\x015\x96P`@\x90\x95\x015\x94\x93PPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x1B\0W`?\x19\x87\x86\x03\x01\x84Ra\x1A\xEB\x85\x83Qa\x1A{V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x1A\xCFV[P\x92\x96\x95PPPPPPV[` \x81R_a\x18\x7F` \x83\x01\x84a\x1A{V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x1BkW\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1B7V[P\x90\x95\x94PPPPPV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15a\x1B\xC4W`\x1F\x19\x85\x84\x03\x01\x88Ra\x1B\xAE\x83\x83Qa\x1A{V[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01a\x1B\x92V[P\x90\x96\x95PPPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x1B\0W`?\x19\x87\x86\x03\x01\x84R\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x86R` \x81\x01Q\x90P`@` \x87\x01Ra\x1C>`@\x87\x01\x82a\x1BvV[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x1B\xF6V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x1BkW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1CmV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x1B\0W\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90_\x90``\x88\x01\x90[\x80\x83\x10\x15a\x1DDW\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84Q\x16\x82R` \x82\x01\x91P` \x84\x01\x93P`\x01\x83\x01\x92Pa\x1C\xFFV[P\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x1C\xB1V[` \x81R_a\x18\x7F` \x83\x01\x84a\x1BvV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x18\x82Wa\x18\x82a\x1DnV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a\x03\x17a\x1E\0\x83\x86a\x1D\xDBV[\x84a\x1D\xDBV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1E\x1AW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1EQW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[P\x91\x90PV[\x80T_\x90`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1EoW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1E\xA6W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x81\x86R` \x86\x01\x81\x80\x15a\x1E\xC1W`\x01\x81\x14a\x1E\xF5Wa\x1F!V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x85\x16\x82R\x83\x15\x15`\x05\x1B\x82\x01\x95Pa\x1F!V[_\x87\x81R` \x90 _[\x85\x81\x10\x15a\x1F\x1BW\x81T\x84\x82\x01R`\x01\x90\x91\x01\x90` \x01a\x1E\xFFV[\x83\x01\x96PP[PPPPP\x92\x91PPV[`\xA0\x81R_a\x1F>`\xA0\x83\x01\x88a\x1EWV[\x82\x81\x03` \x84\x01Ra\x1FP\x81\x88a\x1EWV[`@\x84\x01\x96\x90\x96RPP``\x81\x01\x92\x90\x92R`\xFF\x16`\x80\x90\x91\x01R\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x18\x82Wa\x18\x82a\x1DnV[`\xA0\x81R_a\x1F\x96`\xA0\x83\x01\x87a\x1EWV[\x82\x81\x03` \x84\x01Ra\x1F\xA8\x81\x87a\x1EWV[\x91PP_`@\x83\x01R\x83``\x83\x01R`\xFF\x83\x16`\x80\x83\x01R\x95\x94PPPPPV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81R_a\x03\x17`\x04\x83\x01\x84a\x1D\xDBV[_a\x18\x7F\x82\x84a\x1D\xDBV[_` \x82\x84\x03\x12\x15a \x17W__\xFD[\x81Q\x80\x15\x15\x81\x14a &W__\xFD[\x93\x92PPPV[`\xA0\x81R_a ?`\xA0\x83\x01\x87a\x1EWV[\x82\x81\x03` \x84\x01R` \x81R_` \x82\x01R`@\x81\x01\x91PP\x84`@\x83\x01R\x83``\x83\x01R`\xFF\x83\x16`\x80\x83\x01R\x95\x94PPPPPV[`\xA0\x81R_a \x88`\xA0\x83\x01\x87a\x1EWV[\x82\x81\x03` \x84\x01R`\x01\x81R_` \x82\x01R`@\x81\x01\x91PP\x84`@\x83\x01R\x83``\x83\x01R`\xFF\x83\x16`\x80\x83\x01R\x95\x94PPPPPV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_a \xF0`\x01\x83\x01\x86a\x1D\xDBV[\x7F[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra! `\x01\x82\x01\x86a\x1D\xDBV[\x90P\x7F].\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra!R`\x02\x82\x01\x85a\x1D\xDBV[\x96\x95PPPPPPV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a!\x8CWa!\x8Ca\x1DnV[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82a!\xCEWa!\xCEa!\x93V[P\x04\x90V[_\x82a!\xE1Wa!\xE1a!\x93V[P\x06\x90V[`@\x81R_a!\xF8`@\x83\x01\x85a\x1A{V[\x82\x81\x03` \x84\x01Ra\"\n\x81\x85a\x1A{V[\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\"#W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"9W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\"IW__\xFD[\x80Qa\"Wa\x1A:\x82a\x19\xCDV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\"kW__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a\"\x98W__\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 \x8B\xBB_\xFD\x8C\x8FC\x8E\xE4\x17\\\xE4\x180A)\xF2\x14\xB7\xA1\xB6y\x9B\xD9\xB0\x80\xA2M\xBB\xBEQLdsolcC\0\x08\x1B\x003`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa(\xD78\x03\x80a(\xD7\x839\x81\x01`@\x81\x90Ra\0.\x91a\x03+V[\x82\x82\x82\x82\x82\x82a\0?\x83Q`P\x14\x90V[a\0\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RpBad genesis block`x\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_a\0\x8E\x84a\x01fV[\x90Pb\xFF\xFF\xFF\x82\x16\x15a\x01\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FPeriod start hash does not have `D\x82\x01R\x7Fwork. Hint: wrong byte order?\0\0\0`d\x82\x01R`\x84\x01a\0{V[_\x81\x81U`\x01\x82\x90U`\x02\x82\x90U\x81\x81R`\x04` R`@\x90 \x83\x90Ua\x012a\x07\xE0\x84a\x03\xFEV[a\x01<\x90\x84a\x04%V[_\x83\x81R`\x04` R`@\x90 Ua\x01S\x84a\x02&V[`\x05UPa\x05\xBD\x98PPPPPPPPPV[_`\x02\x80\x83`@Qa\x01x\x91\x90a\x048V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x01\x93W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xB6\x91\x90a\x04NV[`@Q` \x01a\x01\xC8\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x01\xE2\x91a\x048V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x01\xFDW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02 \x91\x90a\x04NV[\x92\x91PPV[_a\x02 a\x023\x83a\x028V[a\x02CV[_a\x02 \x82\x82a\x02SV[_a\x02 a\xFF\xFF`\xD0\x1B\x83a\x02\xF7V[_\x80a\x02ja\x02c\x84`Ha\x04eV[\x85\x90a\x03\tV[`\xE8\x1C\x90P_\x84a\x02|\x85`Ka\x04eV[\x81Q\x81\x10a\x02\x8CWa\x02\x8Ca\x04xV[\x01` \x01Q`\xF8\x1C\x90P_a\x02\xBE\x83_`\x10\x82b\xFF\xFF\xFF\x16\x90\x1C\x82a\xFF\0\x16`\x10\x84b\xFF\xFF\xFF\x16\x90\x1B\x17\x17\x90P\x91\x90PV[b\xFF\xFF\xFF\x16\x90P_a\x02\xD1`\x03\x84a\x04\x8CV[`\xFF\x16\x90Pa\x02\xE2\x81a\x01\0a\x05\x88V[a\x02\xEC\x90\x83a\x05\x93V[\x97\x96PPPPPPPV[_a\x03\x02\x82\x84a\x05\xAAV[\x93\x92PPPV[_a\x03\x02\x83\x83\x01` \x01Q\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[___``\x84\x86\x03\x12\x15a\x03=W__\xFD[\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03RW__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x03bW__\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03{Wa\x03{a\x03\x17V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x03\xA9Wa\x03\xA9a\x03\x17V[`@R\x81\x81R\x82\x82\x01` \x01\x88\x10\x15a\x03\xC0W__\xFD[\x81` \x84\x01` \x83\x01^_` \x92\x82\x01\x83\x01R\x90\x86\x01Q`@\x90\x96\x01Q\x90\x97\x95\x96P\x94\x93PPPPV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82a\x04\x0CWa\x04\x0Ca\x03\xEAV[P\x06\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x02 Wa\x02 a\x04\x11V[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15a\x04^W__\xFD[PQ\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x02 Wa\x02 a\x04\x11V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x02 Wa\x02 a\x04\x11V[`\x01\x81[`\x01\x84\x11\x15a\x04\xE0W\x80\x85\x04\x81\x11\x15a\x04\xC4Wa\x04\xC4a\x04\x11V[`\x01\x84\x16\x15a\x04\xD2W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a\x04\xA9V[\x93P\x93\x91PPV[_\x82a\x04\xF6WP`\x01a\x02 V[\x81a\x05\x02WP_a\x02 V[\x81`\x01\x81\x14a\x05\x18W`\x02\x81\x14a\x05\"Wa\x05>V[`\x01\x91PPa\x02 V[`\xFF\x84\x11\x15a\x053Wa\x053a\x04\x11V[PP`\x01\x82\x1Ba\x02 V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x05aWP\x81\x81\na\x02 V[a\x05m_\x19\x84\x84a\x04\xA5V[\x80_\x19\x04\x82\x11\x15a\x05\x80Wa\x05\x80a\x04\x11V[\x02\x93\x92PPPV[_a\x03\x02\x83\x83a\x04\xE8V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02 Wa\x02 a\x04\x11V[_\x82a\x05\xB8Wa\x05\xB8a\x03\xEAV[P\x04\x90V[a#\r\x80a\x05\xCA_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xFBW_5`\xE0\x1C\x80cp\xD5<\x18\x11a\0\x93W\x80c\xB9\x85b\x1A\x11a\0cW\x80c\xB9\x85b\x1A\x14a\x01\xEDW\x80c\xC5\x82B\xCD\x14a\x02\0W\x80c\xE3\xD8\xD8\xD8\x14a\x02\x08W\x80c\xF5\x8D\xB0o\x14a\x02\x0FW__\xFD[\x80cp\xD5<\x18\x14a\x01\x97W\x80ct\xC3\xA3\xA9\x14a\x01\xB4W\x80c\x7F\xA67\xFC\x14a\x01\xC7W\x80c\xB2[\x9B\0\x14a\x01\xDAW__\xFD[\x80c.O\x16\x1A\x11a\0\xCEW\x80c.O\x16\x1A\x14a\x01;W\x80c0\x01{;\x14a\x01^W\x80c`\xB5\xC3\x90\x14a\x01qW\x80ce\xDAA\xB9\x14a\x01\x84W__\xFD[\x80c\x05\xD0\x9Ap\x14a\0\xFFW\x80c\x117d\xBE\x14a\x01\x14W\x80c\x19\x10\xD9s\x14a\x01+W\x80c+\x97\xBE$\x14a\x013W[__\xFD[a\x01\x12a\x01\r6`\x04a\x1D*V[a\x02{V[\0[`\x05T[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x01Ta\x01\x18V[`\x06Ta\x01\x18V[a\x01Na\x01I6`\x04a\x1D\xC4V[a\x05cV[`@Q\x90\x15\x15\x81R` \x01a\x01\"V[a\x01\x18a\x01l6`\x04a\x1D\xF3V[a\x05{V[a\x01\x18a\x01\x7F6`\x04a\x1E\x13V[a\x05\x8FV[a\x01Na\x01\x926`\x04a\x1E*V[a\x05\x99V[a\x01\x9F`\x04\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\"V[a\x01Na\x01\xC26`\x04a\x1E\x96V[a\x07EV[a\x01Na\x01\xD56`\x04a\x1F\x17V[a\x08\xBAV[a\x01\x18a\x01\xE86`\x04a\x1F\xB6V[a\n\x99V[a\x01Na\x01\xFB6`\x04a /V[a\x0B\x16V[`\x02Ta\x01\x18V[_Ta\x01\x18V[a\x01\x12a\x02\x1D6`\x04a lV[`\x07\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x16\x92\x15\x15\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x92\x90\x92\x17a\x01\0\x91\x15\x15\x91\x90\x91\x02\x17\x90UV[a\x02\xB9\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0B,\x92PPPV[a\x03\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FBad header block\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x03H\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0B3\x92PPPV[a\x03\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7FBad merkle array proof\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x01V[a\x04\x13\x83a\x03\xD6\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0BI\x92PPPV[\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x88\x92Pa\x0BU\x91PPV[a\x04_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FBad inclusion proof\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x01V[_a\x04\x9E\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0B\x87\x92PPPV[\x90P_a\x04\xAA`\x02T\x90V[\x90Pa\x04\xB8\x82\x82`\xF0a\x0C_V[a\x05\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FGCD does not confirm header\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x01V[`\xFF\x83\x16`\x08\x10\x15a\x05XW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInsufficient confirmations\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x01V[PPPPPPPPPV[_a\x05p\x85\x85\x85\x85a\x0C\x8FV[\x90P[\x94\x93PPPPV[_a\x05\x86\x83\x83a\r)V[\x90P[\x92\x91PPV[_a\x05\x89\x82a\r\x9BV[_a\x05\xD8\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0EI\x92PPPV[a\x06JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FHeader array length must be divi`D\x82\x01R\x7Fsible by 80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x01V[a\x06\x88\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0B,\x92PPPV[a\x06\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FAnchor must be 80 bytes\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x01V[a\x05p\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x89\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x87\x81R\x92P\x87\x91P\x86\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x82\x90RP\x92Pa\x0EX\x91PPV[_a\x07\x84\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0B,\x92PPPV[\x80\x15a\x07\xC9WPa\x07\xC9\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0B,\x92PPPV[a\x08;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FBad args. Check header and array`D\x82\x01R\x7F byte lengths.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x01V[a\x08\xAF\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8B\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x89\x81R\x92P\x89\x91P\x88\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x91\x90\x91RP\x88\x92Pa\x12E\x91PPV[\x97\x96PPPPPPPV[_a\x08\xF9\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0B,\x92PPPV[\x80\x15a\t>WPa\t>\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0B,\x92PPPV[\x80\x15a\t\x83WPa\t\x83\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0EI\x92PPPV[a\t\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FBad args. Check header and array`D\x82\x01R\x7F byte lengths.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x01V[a\x08\xAF\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8B\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x89\x81R\x92P\x89\x91P\x88\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8A\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x88\x81R\x92P\x88\x91P\x87\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x14\xE2\x92PPPV[_a\x0B\x0C\x86\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8A\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x88\x81R\x92P\x88\x91P\x87\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x17t\x92PPPV[\x96\x95PPPPPPV[_a\x0B\"\x84\x84\x84a\x0C_V[\x90P[\x93\x92PPPV[Q`P\x14\x90V[_` \x82Qa\x0BB\x91\x90a \xCAV[\x15\x92\x91PPV[`D\x81\x01Q_\x90a\x05\x89V[_\x83\x85\x14\x80\x15a\x0BcWP\x81\x15[\x80\x15a\x0BnWP\x82Q\x15[\x15a\x0B{WP`\x01a\x05sV[a\x05p\x85\x84\x86\x85a\x19\x05V[_`\x02\x80\x83`@Qa\x0B\x99\x91\x90a \xDDV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x0B\xB4W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xD7\x91\x90a \xF3V[`@Q` \x01a\x0B\xE9\x91\x81R` \x01\x90V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0C!\x91a \xDDV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x0C<W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x89\x91\x90a \xF3V[`\x07T_\x90`\xFF\x16\x15a\x0C}WP`\x07Ta\x01\0\x90\x04`\xFF\x16a\x0B%V[a\x0C\x88\x84\x84\x84a\x19\xAAV[\x90Pa\x0B%V[_\x83\x85\x14\x80\x15a\x0C\x9EWP\x82\x85\x14[\x15a\x0C\xABWP`\x01a\x05sV[\x83\x83\x81\x81_[\x86\x81\x10\x15a\x0C\xF3W\x89\x83\x14a\x0C\xD2W_\x83\x81R`\x03` R`@\x90 T\x92\x94P[\x89\x82\x14a\x0C\xEBW_\x82\x81R`\x03` R`@\x90 T\x91\x93P[`\x01\x01a\x0C\xB1V[P\x82\x84\x03a\r\x07W_\x94PPPPPa\x05sV[\x80\x82\x14a\r\x1AW_\x94PPPPPa\x05sV[P`\x01\x98\x97PPPPPPPPV[_\x82\x81[\x83\x81\x10\x15a\rMW_\x91\x82R`\x03` R`@\x90\x91 T\x90`\x01\x01a\r-V[P\x80a\x05\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FUnknown ancestor\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x01V[_\x80\x82\x81[a\r\xAC`\x04`\x01a!7V[c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x0E\0W_\x82\x81R`\x04` R`@\x81 T\x93P\x83\x90\x03a\r\xE5W_\x91\x82R`\x03` R`@\x90\x91 T\x90a\r\xF8V[a\r\xEF\x81\x84a!SV[\x95\x94PPPPPV[`\x01\x01a\r\xA0V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01R\x7FUnknown block\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x01V[_`P\x82Qa\x0BB\x91\x90a \xCAV[__a\x0Ec\x85a\x0B\x87V[\x90P_a\x0Eo\x82a\r\x9BV[\x90P_a\x0E{\x86a\x19\xEBV[\x90P\x84\x80a\x0E\x90WP\x80a\x0E\x8E\x88a\x19\xEBV[\x14[a\x0F\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FUnexpected retarget on external `D\x82\x01R\x7Fcall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x01V[\x85Q_\x90\x81\x90\x81[\x81\x81\x10\x15a\x12\x02Wa\x0F\x1C`P\x82a!fV[a\x0F'\x90`\x01a!SV[a\x0F1\x90\x87a!SV[\x93Pa\x0F?\x8A\x82`Pa\x19\xF6V[_\x81\x81R`\x03` R`@\x90 T\x90\x93Pa\x11\x15W\x84a\x10\x95\x84_\x81\x90P`\x08\x81~\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\x16\x90\x1B`\x08\x82\x90\x1C~\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\x16\x17\x90P`\x10\x81}\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\x16\x90\x1B`\x10\x82\x90\x1C}\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\x16\x17\x90P` \x81{\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\x16\x90\x1B` \x82\x90\x1C{\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\x16\x17\x90P`@\x81w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B`@\x82\x90\x1Cw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x90P`\x80\x81\x90\x1B`\x80\x82\x90\x1C\x17\x90P\x91\x90PV[\x11\x15a\x10\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FHeader work is insufficient\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x01V[_\x83\x81R`\x03` R`@\x90 \x87\x90Ua\x10\xFE`\x04\x85a \xCAV[_\x03a\x11\x15W_\x83\x81R`\x04` R`@\x90 \x84\x90U[\x84a\x11 \x8B\x83a\x1A\x1BV[\x14a\x11mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FTarget changed unexpectedly\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x01V[\x86a\x11x\x8B\x83a\x1A\xB4V[\x14a\x11\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FHeaders do not form a consistent`D\x82\x01R\x7F chain\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x01V[\x82\x96P`P\x81a\x11\xFB\x91\x90a!SV[\x90Pa\x0F\tV[P\x81a\x12\r\x8Ba\x0B\x87V[`@Q\x7F\xF9\x0EO\x1D\x9C\xD0\xDDU\xE39A\x1C\xBC\x9B\x15$\x820|:#\xEDdq^J(X\xF6A\xA3\xF5\x90_\x90\xA3P`\x01\x99\x98PPPPPPPPPV[_a\x07\xE0\x82\x11\x15a\x12\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FRequested limit is greater than `D\x82\x01R\x7F1 difficulty period\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x01V[_a\x12\xC8\x84a\x0B\x87V[\x90P_a\x12\xD4\x86a\x0B\x87V[\x90P`\x01T\x81\x14a\x13'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FPassed in best is not best known`D\x82\x01R`d\x01a\x03\x01V[_\x82\x81R`\x03` R`@\x90 Ta\x13\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FNew best is unknown\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x01V[a\x13\x8F\x87`\x01T\x84\x87a\x0C\x8FV[a\x14\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FAncestor must be heaviest common`D\x82\x01R\x7F ancestor\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x01V[\x81a\x14\r\x88\x88\x88a\x17tV[\x14a\x14\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FNew best hash does not have more`D\x82\x01R\x7F work than previous\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x01V[`\x01\x82\x90U`\x02\x87\x90U_a\x14\x94\x86a\x1A\xCCV[\x90P`\x05T\x81\x14a\x14\xA5W`\x05\x81\x90U[\x87\x83\x83\x7F<\xC1=\xE6M\xF0\xF0#\x96&#\\Q\xA2\xDA%\x1B\xBC\x8C\x85fN\xCC\xE3\x92c\xDA>\xE0?`l`@Q`@Q\x80\x91\x03\x90\xA4P`\x01\x97\x96PPPPPPPV[__a\x14\xF5a\x14\xF0\x86a\x0B\x87V[a\r\x9BV[\x90P_a\x15\x04a\x14\xF0\x86a\x0B\x87V[\x90Pa\x15\x12a\x07\xE0\x82a \xCAV[a\x07\xDF\x14a\x15\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FMust provide the last header of `D\x82\x01R\x7Fthe closing difficulty period\0\0\0`d\x82\x01R`\x84\x01a\x03\x01V[a\x15\x94\x82a\x07\xDFa!SV[\x81\x14a\x16\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FMust provide exactly 1 difficult`D\x82\x01R\x7Fy period\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x01V[a\x16\x11\x85a\x1A\xCCV[a\x16\x1A\x87a\x1A\xCCV[\x14a\x16\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FPeriod header difficulties do no`D\x82\x01R\x7Ft match\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x01V[_a\x16\x97\x85a\x19\xEBV[\x90P_a\x16\xC9a\x16\xA6\x89a\x19\xEBV[a\x16\xAF\x8Aa\x1A\xDEV[c\xFF\xFF\xFF\xFF\x16a\x16\xBE\x8Aa\x1A\xDEV[c\xFF\xFF\xFF\xFF\x16a\x1B\x11V[\x90P\x81\x81\x83\x16\x14a\x17\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FInvalid retarget provided\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x01V[_a\x17&\x89a\x1A\xCCV[\x90P\x80`\x06T\x14\x15\x80\x15a\x17PWPa\x07\xE0a\x17C`\x01Ta\r\x9BV[a\x17M\x91\x90a!yV[\x84\x11[\x15a\x17[W`\x06\x81\x90U[a\x17g\x88\x88`\x01a\x0EXV[\x99\x98PPPPPPPPPV[__a\x17\x7F\x85a\r\x9BV[\x90P_a\x17\x8Ea\x14\xF0\x86a\x0B\x87V[\x90P_a\x17\x9Da\x14\xF0\x86a\x0B\x87V[\x90P\x82\x82\x10\x15\x80\x15a\x17\xAFWP\x82\x81\x10\x15[a\x18!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FA descendant height is below the`D\x82\x01R\x7F ancestor height\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x01V[_a\x18.a\x07\xE0\x85a \xCAV[a\x18:\x85a\x07\xE0a!SV[a\x18D\x91\x90a!yV[\x90P\x80\x83\x10\x81\x83\x10\x81\x15\x82a\x18VWP\x80[\x15a\x18qWa\x18d\x89a\x0B\x87V[\x96PPPPPPPa\x0B%V[\x81\x80\x15a\x18|WP\x80\x15[\x15a\x18\x8AWa\x18d\x88a\x0B\x87V[\x81\x80\x15a\x18\x94WP\x80[\x15a\x18\xB8W\x83\x85\x10\x15a\x18\xAFWa\x18\xAA\x88a\x0B\x87V[a\x18dV[a\x18d\x89a\x0B\x87V[a\x18\xC1\x88a\x1A\xCCV[a\x18\xCDa\x07\xE0\x86a \xCAV[a\x18\xD7\x91\x90a!\x8CV[a\x18\xE0\x8Aa\x1A\xCCV[a\x18\xECa\x07\xE0\x88a \xCAV[a\x18\xF6\x91\x90a!\x8CV[\x10\x15a\x18\xAFWa\x18d\x88a\x0B\x87V[_` \x84Qa\x19\x14\x91\x90a \xCAV[\x15a\x19 WP_a\x05sV[\x83Q_\x03a\x19/WP_a\x05sV[\x81\x85_[\x86Q\x81\x10\x15a\x19\x9DWa\x19G`\x02\x84a \xCAV[`\x01\x03a\x19kWa\x19da\x19^\x88\x83\x01` \x01Q\x90V[\x83a\x1B\x99V[\x91Pa\x19\x84V[a\x19\x81\x82a\x19|\x89\x84\x01` \x01Q\x90V[a\x1B\x99V[\x91P[`\x01\x92\x90\x92\x1C\x91a\x19\x96` \x82a!SV[\x90Pa\x193V[P\x90\x93\x14\x95\x94PPPPPV[_\x82\x81[\x83\x81\x10\x15a\x19\xE0W\x85\x82\x03a\x19\xC8W`\x01\x92PPPa\x0B%V[_\x91\x82R`\x03` R`@\x90\x91 T\x90`\x01\x01a\x19\xAEV[P_\x95\x94PPPPPV[_a\x05\x89\x82_a\x1A\x1BV[_` _\x83\x85` \x01\x87\x01`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x93\x92PPPV[_\x80a\x1A2a\x1A+\x84`Ha!SV[\x85\x90a\x1B\xA4V[`\xE8\x1C\x90P_\x84a\x1AD\x85`Ka!SV[\x81Q\x81\x10a\x1ATWa\x1ATa!\xA3V[\x01` \x01Q`\xF8\x1C\x90P_a\x1A\x86\x83_`\x10\x82b\xFF\xFF\xFF\x16\x90\x1C\x82a\xFF\0\x16`\x10\x84b\xFF\xFF\xFF\x16\x90\x1B\x17\x17\x90P\x91\x90PV[b\xFF\xFF\xFF\x16\x90P_a\x1A\x99`\x03\x84a!\xD0V[`\xFF\x16\x90Pa\x1A\xAA\x81a\x01\0a\"\xCCV[a\x08\xAF\x90\x83a!\x8CV[_a\x05\x86a\x1A\xC3\x83`\x04a!SV[\x84\x01` \x01Q\x90V[_a\x05\x89a\x1A\xD9\x83a\x19\xEBV[a\x1B\xB2V[_a\x05\x89a\x1A\xEB\x83a\x1B\xD9V[`\xD8\x81\x90\x1Cc\xFF\0\xFF\0\x16b\xFF\0\xFF`\xE8\x92\x90\x92\x1C\x91\x90\x91\x16\x17`\x10\x81\x81\x1B\x91\x90\x1C\x17\x90V[_\x80a\x1B\x1D\x83\x85a\x1B\xE5V[\x90Pa\x1B-b\x12u\0`\x04a\x1C@V[\x81\x10\x15a\x1BEWa\x1BBb\x12u\0`\x04a\x1C@V[\x90P[a\x1BSb\x12u\0`\x04a\x1CKV[\x81\x11\x15a\x1BkWa\x1Bhb\x12u\0`\x04a\x1CKV[\x90P[_a\x1B\x83\x82a\x1B}\x88b\x01\0\0a\x1C@V[\x90a\x1CKV[\x90Pa\x0B\x0Cb\x01\0\0a\x1B}\x83b\x12u\0a\x1C@V[_a\x05\x86\x83\x83a\x1C\xBEV[_a\x05\x86\x83\x83\x01` \x01Q\x90V[_a\x05\x89{\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x1C@V[_a\x05\x89\x82`Da\x1B\xA4V[_\x82\x82\x11\x15a\x1C6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FUnderflow during subtraction.\0\0\0`D\x82\x01R`d\x01a\x03\x01V[a\x05\x86\x82\x84a!yV[_a\x05\x86\x82\x84a!fV[_\x82_\x03a\x1CZWP_a\x05\x89V[a\x1Cd\x82\x84a!\x8CV[\x90P\x81a\x1Cq\x84\x83a!fV[\x14a\x05\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FOverflow during multiplication.\0`D\x82\x01R`d\x01a\x03\x01V[_\x82_R\x81` R` _`@_`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x92\x91PPV[__\x83`\x1F\x84\x01\x12a\x1C\xF5W__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\x0CW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1D#W__\xFD[\x92P\x92\x90PV[_______`\xA0\x88\x8A\x03\x12\x15a\x1D@W__\xFD[\x875g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1DVW__\xFD[a\x1Db\x8A\x82\x8B\x01a\x1C\xE5V[\x90\x98P\x96PP` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\x81W__\xFD[a\x1D\x8D\x8A\x82\x8B\x01a\x1C\xE5V[\x90\x96P\x94PP`@\x88\x015\x92P``\x88\x015\x91P`\x80\x88\x015`\xFF\x81\x16\x81\x14a\x1D\xB4W__\xFD[\x80\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[____`\x80\x85\x87\x03\x12\x15a\x1D\xD7W__\xFD[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[__`@\x83\x85\x03\x12\x15a\x1E\x04W__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[_` \x82\x84\x03\x12\x15a\x1E#W__\xFD[P5\x91\x90PV[____`@\x85\x87\x03\x12\x15a\x1E=W__\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1ESW__\xFD[a\x1E_\x87\x82\x88\x01a\x1C\xE5V[\x90\x95P\x93PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E~W__\xFD[a\x1E\x8A\x87\x82\x88\x01a\x1C\xE5V[\x95\x98\x94\x97P\x95PPPPV[______`\x80\x87\x89\x03\x12\x15a\x1E\xABW__\xFD[\x865\x95P` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xC8W__\xFD[a\x1E\xD4\x89\x82\x8A\x01a\x1C\xE5V[\x90\x96P\x94PP`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xF3W__\xFD[a\x1E\xFF\x89\x82\x8A\x01a\x1C\xE5V[\x97\x9A\x96\x99P\x94\x97\x94\x96\x95``\x90\x95\x015\x94\x93PPPPV[______``\x87\x89\x03\x12\x15a\x1F,W__\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1FBW__\xFD[a\x1FN\x89\x82\x8A\x01a\x1C\xE5V[\x90\x97P\x95PP` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1FmW__\xFD[a\x1Fy\x89\x82\x8A\x01a\x1C\xE5V[\x90\x95P\x93PP`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F\x98W__\xFD[a\x1F\xA4\x89\x82\x8A\x01a\x1C\xE5V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[_____``\x86\x88\x03\x12\x15a\x1F\xCAW__\xFD[\x855\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F\xE7W__\xFD[a\x1F\xF3\x88\x82\x89\x01a\x1C\xE5V[\x90\x95P\x93PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \x12W__\xFD[a \x1E\x88\x82\x89\x01a\x1C\xE5V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[___``\x84\x86\x03\x12\x15a AW__\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[\x805\x80\x15\x15\x81\x14a gW__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15a }W__\xFD[a \x86\x83a XV[\x91Pa \x94` \x84\x01a XV[\x90P\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82a \xD8Wa \xD8a \x9DV[P\x06\x90V[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15a!\x03W__\xFD[PQ\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x05\x89Wa\x05\x89a!\nV[\x80\x82\x01\x80\x82\x11\x15a\x05\x89Wa\x05\x89a!\nV[_\x82a!tWa!ta \x9DV[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x05\x89Wa\x05\x89a!\nV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x05\x89Wa\x05\x89a!\nV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x05\x89Wa\x05\x89a!\nV[`\x01\x81[`\x01\x84\x11\x15a\"$W\x80\x85\x04\x81\x11\x15a\"\x08Wa\"\x08a!\nV[`\x01\x84\x16\x15a\"\x16W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a!\xEDV[\x93P\x93\x91PPV[_\x82a\":WP`\x01a\x05\x89V[\x81a\"FWP_a\x05\x89V[\x81`\x01\x81\x14a\"\\W`\x02\x81\x14a\"fWa\"\x82V[`\x01\x91PPa\x05\x89V[`\xFF\x84\x11\x15a\"wWa\"wa!\nV[PP`\x01\x82\x1Ba\x05\x89V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\"\xA5WP\x81\x81\na\x05\x89V[a\"\xB1_\x19\x84\x84a!\xE9V[\x80_\x19\x04\x82\x11\x15a\"\xC4Wa\"\xC4a!\nV[\x02\x93\x92PPPV[_a\x05\x86\x83\x83a\",V\xFE\xA2dipfsX\"\x12 \xAEv\xA7*+>\x04a\x06\xF2\xF9\xAA\xB5\xE54Z\xA0\t<\x8C\x04\xE3\xC2\xFA\x8C\x1A\x8FN\xA0:\x95edsolcC\0\x08\x1B\x003\0\0\0 s\xBD!\x84\xED\xD9\xC4\xFCvd.\xA6uN\xE4\x016\x97\x0E\xFC\x10\xC4\x19\0\0\0\0\0\0\0\0\0\x02\x96\xEF\x12>\xA9m\xA5\xCFi_\"\xBF}\x94\xBE\x87\xD4\x9D\xB1\xADz\xC3q\xACC\xC4\xDAAa\xC8\xC2\x164\x9C[\xA1\x19(\x17\r8x+\xE3Z\rm\xE9Kef\x94X\x99d\xA2R\x95~Fs\xA9\xFB\x1D/\x8BJ\x92\xE3\xF0\xA7\xBBeO\xDD\xB9NZ\x1Em\x7F\x7FI\x9F\xD1\xBE]\xD3\ns\xBFU\x84\xBF\x13}\xA5\xFD\xD7|\xC2\x1A\xEB\x95\xB9\xE3W\x88\x89K\xE0\x19(K\xD4\xFB\xEDm\xD6\x11\x8A\xC2\xCBm&\xBCK\xE4\xE4#\xF5Z:H\xF2\x87M\x8D\x02\xA6]\x9C\x87\xD0}\xE2\x1DM\xFE{\n\x9FJ#\xCC\x9AX7>\x9Ei1\xFE\xFD\xB5\xAF\xAD\xE5\xDFT\xC9\x11\x04\x04\x8D\xF1\xEE\x99\x92@ay\x84\xE1\x8Bo\x93\x1E#sg=\x01\x95\xB8\xC6\x98}\x7F\xF7e\r\\\xE5;\xCE\xC4n\x13\xABO-\xA1\x14j\x7F\xC6!\xEEg/b\xBC\"t$\x869-u\xE5^g\xB0\x99`\xC38j\x0BI\xE7_\x17#\xD6\xAB(\xAC\x9A (\xA0\xC7(f\xE2\x11\x1Dy\xD4\x81{\x88\xE1|\x82\x197\x84wh\xD9(7\xBA\xE3\x83+\xB8\xE5\xA4\xABD4\xB9~\0\xA6\xC1\x01\x82\xF2\x11\xF5\x92@\x90h\xD6\xF5e$\0\xD9\xA3\xD1\xCC\x15\n\x7F\xB6\x92\xE8t\xCCB\xD7k\xDA\xFC\x84//\xE0\xF85\xA7\xC2M-`\xC1\t\xB1\x87\xD6Eq\xEF\xBA\xA8\x04{\xE8X!\xF8\xE6~\x0E\x85\xF2\xF5\x89K\xC6=\0\xC2\xED\x9Dd",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610179575f3560e01c806385226c81116100d2578063e20c9f7111610088578063f11d5cbc11610063578063f11d5cbc146102ac578063fa7626d4146102b4578063fad06b8f146102c1575f5ffd5b8063e20c9f7114610294578063e89f84191461029c578063f0c0832e146102a4575f5ffd5b8063b52b2058116100b8578063b52b20581461026c578063b5508aa914610274578063ba414fa61461027c575f5ffd5b806385226c811461024f578063916a17c614610264575f5ffd5b80633e5e3c2311610132578063641e144d1161010d578063641e144d1461022a57806366d9a9a014610232578063740dba7414610247575f5ffd5b80633e5e3c23146101fa5780633f7286f41461020257806344badbb61461020a575f5ffd5b80631ed7831c116101625780631ed7831c146101c65780632ade3880146101db57806339425f8f146101f0575f5ffd5b80630813852a1461017d5780631c0da81f146101a6575b5f5ffd5b61019061018b3660046119f4565b6102d4565b60405161019d9190611aa9565b60405180910390f35b6101b96101b43660046119f4565b61031f565b60405161019d9190611b0c565b6101ce610391565b60405161019d9190611b1e565b6101e36103fe565b60405161019d9190611bd0565b6101f8610547565b005b6101ce61066e565b6101ce6106d9565b61021d6102183660046119f4565b610744565b60405161019d9190611c54565b6101f8610787565b61023a610890565b60405161019d9190611c8b565b6101f8610993565b610257610a86565b60405161019d9190611d5c565b61023a610b51565b6101f8610c54565b610257610c9e565b610284610d69565b604051901515815260200161019d565b6101ce610ea6565b6101f8610f11565b6101f8611006565b6101f86110fb565b6007546102849060ff1681565b61021d6102cf3660046119f4565b611278565b60606103178484846040518060400160405280600381526020017f68657800000000000000000000000000000000000000000000000000000000008152506112bb565b949350505050565b60605f61032d8585856102d4565b90505f5b61033b8585611d9b565b811015610388578282828151811061035557610355611dae565b602002602001015160405160200161036e929190611df2565b60408051601f198184030181529190529250600101610331565b50509392505050565b606060148054806020026020016040519081016040528092919081815260200182805480156103f457602002820191905f5260205f20905b815473ffffffffffffffffffffffffffffffffffffffff1681526001909101906020018083116103c9575b5050505050905090565b6060601b805480602002602001604051908101604052809291908181526020015f905b8282101561053e575f848152602080822060408051808201825260028702909201805473ffffffffffffffffffffffffffffffffffffffff168352600181018054835181870281018701909452808452939591948681019491929084015b82821015610527578382905f5260205f2001805461049c90611e06565b80601f01602080910402602001604051908101604052809291908181526020018280546104c890611e06565b80156105135780601f106104ea57610100808354040283529160200191610513565b820191905f5260205f20905b8154815290600101906020018083116104f657829003601f168201915b50505050508152602001906001019061047f565b505050508152505081526020019060010190610421565b50505050905090565b604080518082018252601a81527f496e73756666696369656e7420636f6e6669726d6174696f6e73000000000000602082015290517ff28dceb3000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d9163f28dceb3916105c89190600401611b0c565b5f604051808303815f87803b1580156105df575f5ffd5b505af11580156105f1573d5f5f3e3d5ffd5b5050601c54602054602154604051625d09a760e41b815273ffffffffffffffffffffffffffffffffffffffff90931694506305d09a70935061064092601e92601f929091600990600401611f2c565b5f6040518083038186803b158015610656575f5ffd5b505afa158015610668573d5f5f3e3d5ffd5b50505050565b606060168054806020026020016040519081016040528092919081815260200182805480156103f457602002820191905f5260205f2090815473ffffffffffffffffffffffffffffffffffffffff1681526001909101906020018083116103c9575050505050905090565b606060158054806020026020016040519081016040528092919081815260200182805480156103f457602002820191905f5260205f2090815473ffffffffffffffffffffffffffffffffffffffff1681526001909101906020018083116103c9575050505050905090565b60606103178484846040518060400160405280600981526020017f6469676573745f6c65000000000000000000000000000000000000000000000081525061141c565b604080518082018252601381527f42616420696e636c7573696f6e2070726f6f6600000000000000000000000000602082015290517ff28dceb3000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d9163f28dceb3916108089190600401611b0c565b5f604051808303815f87803b15801561081f575f5ffd5b505af1158015610831573d5f5f3e3d5ffd5b5050601c5460205460215473ffffffffffffffffffffffffffffffffffffffff90921693506305d09a709250601e91601f919061086f906001611f71565b5f6040518663ffffffff1660e01b8152600401610640959493929190611f2c565b60606019805480602002602001604051908101604052809291908181526020015f905b8282101561053e575f84815260209081902060408051808201825260028602909201805473ffffffffffffffffffffffffffffffffffffffff16835260018101805483518187028101870190945280845293949193858301939283018282801561097b57602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916815260200190600401906020826003010492830192600103820291508084116109285790505b505050505081525050815260200190600101906108b3565b604080518082018252601381527f42616420696e636c7573696f6e2070726f6f6600000000000000000000000000602082015290517ff28dceb3000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d9163f28dceb391610a149190600401611b0c565b5f604051808303815f87803b158015610a2b575f5ffd5b505af1158015610a3d573d5f5f3e3d5ffd5b5050601c54602154604051625d09a760e41b815273ffffffffffffffffffffffffffffffffffffffff90921693506305d09a70925061064091601e91601f915f90600401611f84565b60606018805480602002602001604051908101604052809291908181526020015f905b8282101561053e578382905f5260205f20018054610ac690611e06565b80601f0160208091040260200160405190810160405280929190818152602001828054610af290611e06565b8015610b3d5780601f10610b1457610100808354040283529160200191610b3d565b820191905f5260205f20905b815481529060010190602001808311610b2057829003601f168201915b505050505081526020019060010190610aa9565b6060601a805480602002602001604051908101604052809291908181526020015f905b8282101561053e575f84815260209081902060408051808201825260028602909201805473ffffffffffffffffffffffffffffffffffffffff168352600181018054835181870281018701909452808452939491938583019392830182828015610c3c57602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191681526020019060040190602082600301049283019260010382029150808411610be95790505b50505050508152505081526020019060010190610b74565b601c54602054602154604051625d09a760e41b815273ffffffffffffffffffffffffffffffffffffffff909316926305d09a709261064092601e92601f9291905f90600401611f2c565b60606017805480602002602001604051908101604052809291908181526020015f905b8282101561053e578382905f5260205f20018054610cde90611e06565b80601f0160208091040260200160405190810160405280929190818152602001828054610d0a90611e06565b8015610d555780601f10610d2c57610100808354040283529160200191610d55565b820191905f5260205f20905b815481529060010190602001808311610d3857829003601f168201915b505050505081526020019060010190610cc1565b6007545f90610100900460ff1615610d8a5750600754610100900460ff1690565b5f737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ea15760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190527f6661696c6564000000000000000000000000000000000000000000000000000082840152825180830384018152606083019093525f929091610e2d917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc491608001611fc9565b60408051601f1981840301815290829052610e4791611ffc565b5f604051808303815f865af19150503d805f8114610e80576040519150601f19603f3d011682016040523d82523d5f602084013e610e85565b606091505b5091505080806020019051810190610e9d9190612007565b9150505b919050565b606060138054806020026020016040519081016040528092919081815260200182805480156103f457602002820191905f5260205f2090815473ffffffffffffffffffffffffffffffffffffffff1681526001909101906020018083116103c9575050505050905090565b604080518082018252601381527f42616420696e636c7573696f6e2070726f6f6600000000000000000000000000602082015290517ff28dceb3000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d9163f28dceb391610f929190600401611b0c565b5f604051808303815f87803b158015610fa9575f5ffd5b505af1158015610fbb573d5f5f3e3d5ffd5b5050601c54602054602154604051625d09a760e41b815273ffffffffffffffffffffffffffffffffffffffff90931694506305d09a70935061064092601e9291905f9060040161202d565b604080518082018252601681527f426164206d65726b6c652061727261792070726f6f6600000000000000000000602082015290517ff28dceb3000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d9163f28dceb3916110879190600401611b0c565b5f604051808303815f87803b15801561109e575f5ffd5b505af11580156110b0573d5f5f3e3d5ffd5b5050601c54602054602154604051625d09a760e41b815273ffffffffffffffffffffffffffffffffffffffff90931694506305d09a70935061064092601e9291905f90600401612076565b601c546040517ff58db06f0000000000000000000000000000000000000000000000000000000081525f60048201819052602482015273ffffffffffffffffffffffffffffffffffffffff9091169063f58db06f906044015f604051808303815f87803b15801561116a575f5ffd5b505af115801561117c573d5f5f3e3d5ffd5b5050604080518082018252601b81527f47434420646f6573206e6f7420636f6e6669726d206865616465720000000000602082015290517ff28dceb3000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d935063f28dceb392506112019190600401611b0c565b5f604051808303815f87803b158015611218575f5ffd5b505af115801561122a573d5f5f3e3d5ffd5b5050601c54602054602154604051625d09a760e41b815273ffffffffffffffffffffffffffffffffffffffff90931694506305d09a70935061064092601e92601f9290915f90600401611f2c565b60606103178484846040518060400160405280600681526020017f686569676874000000000000000000000000000000000000000000000000000081525061156a565b60606112c78484611d9b565b67ffffffffffffffff8111156112df576112df61196f565b60405190808252806020026020018201604052801561131257816020015b60608152602001906001900390816112fd5790505b509050835b83811015611413576113e58661132c836116b8565b8560405160200161133f939291906120bf565b604051602081830303815290604052601d805461135b90611e06565b80601f016020809104026020016040519081016040528092919081815260200182805461138790611e06565b80156113d25780601f106113a9576101008083540402835291602001916113d2565b820191905f5260205f20905b8154815290600101906020018083116113b557829003601f168201915b50505050506117e990919063ffffffff16565b826113f08784611d9b565b8151811061140057611400611dae565b6020908102919091010152600101611317565b50949350505050565b60606114288484611d9b565b67ffffffffffffffff8111156114405761144061196f565b604051908082528060200260200182016040528015611469578160200160208202803683370190505b509050835b838110156114135761153c86611483836116b8565b85604051602001611496939291906120bf565b604051602081830303815290604052601d80546114b290611e06565b80601f01602080910402602001604051908101604052809291908181526020018280546114de90611e06565b80156115295780601f1061150057610100808354040283529160200191611529565b820191905f5260205f20905b81548152906001019060200180831161150c57829003601f168201915b505050505061188890919063ffffffff16565b826115478784611d9b565b8151811061155757611557611dae565b602090810291909101015260010161146e565b60606115768484611d9b565b67ffffffffffffffff81111561158e5761158e61196f565b6040519080825280602002602001820160405280156115b7578160200160208202803683370190505b509050835b838110156114135761168a866115d1836116b8565b856040516020016115e4939291906120bf565b604051602081830303815290604052601d805461160090611e06565b80601f016020809104026020016040519081016040528092919081815260200182805461162c90611e06565b80156116775780601f1061164e57610100808354040283529160200191611677565b820191905f5260205f20905b81548152906001019060200180831161165a57829003601f168201915b505050505061191b90919063ffffffff16565b826116958784611d9b565b815181106116a5576116a5611dae565b60209081029190910101526001016115bc565b6060815f036116fa57505060408051808201909152600181527f3000000000000000000000000000000000000000000000000000000000000000602082015290565b815f5b8115611723578061170d8161215c565b915061171c9050600a836121c0565b91506116fd565b5f8167ffffffffffffffff81111561173d5761173d61196f565b6040519080825280601f01601f191660200182016040528015611767576020820181803683370190505b5090505b84156103175761177c600183611d9b565b9150611789600a866121d3565b611794906030611f71565b60f81b8183815181106117a9576117a9611dae565b60200101907effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff191690815f1a9053506117e2600a866121c0565b945061176b565b6040517ffd921be8000000000000000000000000000000000000000000000000000000008152606090737109709ecfa91a80626ff3989d68f67f5b1dd12d9063fd921be89061183e90869086906004016121e6565b5f60405180830381865afa158015611858573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261187f9190810190612213565b90505b92915050565b6040517f1777e59d0000000000000000000000000000000000000000000000000000000081525f90737109709ecfa91a80626ff3989d68f67f5b1dd12d90631777e59d906118dc90869086906004016121e6565b602060405180830381865afa1580156118f7573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061187f9190612288565b6040517faddde2b60000000000000000000000000000000000000000000000000000000081525f90737109709ecfa91a80626ff3989d68f67f5b1dd12d9063addde2b6906118dc90869086906004016121e6565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b604051601f8201601f1916810167ffffffffffffffff811182821017156119c5576119c561196f565b604052919050565b5f67ffffffffffffffff8211156119e6576119e661196f565b50601f01601f191660200190565b5f5f5f60608486031215611a06575f5ffd5b833567ffffffffffffffff811115611a1c575f5ffd5b8401601f81018613611a2c575f5ffd5b8035611a3f611a3a826119cd565b61199c565b818152876020838501011115611a53575f5ffd5b816020840160208301375f602092820183015297908601359650604090950135949350505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015611b0057603f19878603018452611aeb858351611a7b565b94506020938401939190910190600101611acf565b50929695505050505050565b602081525f61187f6020830184611a7b565b602080825282518282018190525f918401906040840190835b81811015611b6b57835173ffffffffffffffffffffffffffffffffffffffff16835260209384019390920191600101611b37565b509095945050505050565b5f82825180855260208501945060208160051b830101602085015f5b83811015611bc457601f19858403018852611bae838351611a7b565b6020988901989093509190910190600101611b92565b50909695505050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015611b0057603f19878603018452815173ffffffffffffffffffffffffffffffffffffffff81511686526020810151905060406020870152611c3e6040870182611b76565b9550506020938401939190910190600101611bf6565b602080825282518282018190525f918401906040840190835b81811015611b6b578351835260209384019390920191600101611c6d565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015611b0057868503603f190184528151805173ffffffffffffffffffffffffffffffffffffffff168652602090810151604082880181905281519088018190529101905f9060608801905b80831015611d44577fffffffff000000000000000000000000000000000000000000000000000000008451168252602082019150602084019350600183019250611cff565b50965050506020938401939190910190600101611cb1565b602081525f61187f6020830184611b76565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b8181038181111561188257611882611d6e565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f81518060208401855e5f93019283525090919050565b5f610317611e008386611ddb565b84611ddb565b600181811c90821680611e1a57607f821691505b602082108103611e51577f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b50919050565b80545f90600181811c90821680611e6f57607f821691505b602082108103611ea6577f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b81865260208601818015611ec15760018114611ef557611f21565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff008516825283151560051b82019550611f21565b5f878152602090205f5b85811015611f1b57815484820152600190910190602001611eff565b83019650505b505050505092915050565b60a081525f611f3e60a0830188611e57565b8281036020840152611f508188611e57565b60408401969096525050606081019290925260ff1660809091015292915050565b8082018082111561188257611882611d6e565b60a081525f611f9660a0830187611e57565b8281036020840152611fa88187611e57565b9150505f604083015283606083015260ff8316608083015295945050505050565b7fffffffff00000000000000000000000000000000000000000000000000000000831681525f6103176004830184611ddb565b5f61187f8284611ddb565b5f60208284031215612017575f5ffd5b81518015158114612026575f5ffd5b9392505050565b60a081525f61203f60a0830187611e57565b8281036020840152602081525f60208201526040810191505084604083015283606083015260ff8316608083015295945050505050565b60a081525f61208860a0830187611e57565b8281036020840152600181525f60208201526040810191505084604083015283606083015260ff8316608083015295945050505050565b7f2e0000000000000000000000000000000000000000000000000000000000000081525f6120f06001830186611ddb565b7f5b0000000000000000000000000000000000000000000000000000000000000081526121206001820186611ddb565b90507f5d2e00000000000000000000000000000000000000000000000000000000000081526121526002820185611ddb565b9695505050505050565b5f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff820361218c5761218c611d6e565b5060010190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f826121ce576121ce612193565b500490565b5f826121e1576121e1612193565b500690565b604081525f6121f86040830185611a7b565b828103602084015261220a8185611a7b565b95945050505050565b5f60208284031215612223575f5ffd5b815167ffffffffffffffff811115612239575f5ffd5b8201601f81018413612249575f5ffd5b8051612257611a3a826119cd565b81815285602083850101111561226b575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b5f60208284031215612298575f5ffd5b505191905056fea26469706673582212208bbb5ffd8c8f438ee4175ce418304129f214b7a1b6799bd9b080a24dbbbe514c64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01yW_5`\xE0\x1C\x80c\x85\"l\x81\x11a\0\xD2W\x80c\xE2\x0C\x9Fq\x11a\0\x88W\x80c\xF1\x1D\\\xBC\x11a\0cW\x80c\xF1\x1D\\\xBC\x14a\x02\xACW\x80c\xFAv&\xD4\x14a\x02\xB4W\x80c\xFA\xD0k\x8F\x14a\x02\xC1W__\xFD[\x80c\xE2\x0C\x9Fq\x14a\x02\x94W\x80c\xE8\x9F\x84\x19\x14a\x02\x9CW\x80c\xF0\xC0\x83.\x14a\x02\xA4W__\xFD[\x80c\xB5+ X\x11a\0\xB8W\x80c\xB5+ X\x14a\x02lW\x80c\xB5P\x8A\xA9\x14a\x02tW\x80c\xBAAO\xA6\x14a\x02|W__\xFD[\x80c\x85\"l\x81\x14a\x02OW\x80c\x91j\x17\xC6\x14a\x02dW__\xFD[\x80c>^<#\x11a\x012W\x80cd\x1E\x14M\x11a\x01\rW\x80cd\x1E\x14M\x14a\x02*W\x80cf\xD9\xA9\xA0\x14a\x022W\x80ct\r\xBAt\x14a\x02GW__\xFD[\x80c>^<#\x14a\x01\xFAW\x80c?r\x86\xF4\x14a\x02\x02W\x80cD\xBA\xDB\xB6\x14a\x02\nW__\xFD[\x80c\x1E\xD7\x83\x1C\x11a\x01bW\x80c\x1E\xD7\x83\x1C\x14a\x01\xC6W\x80c*\xDE8\x80\x14a\x01\xDBW\x80c9B_\x8F\x14a\x01\xF0W__\xFD[\x80c\x08\x13\x85*\x14a\x01}W\x80c\x1C\r\xA8\x1F\x14a\x01\xA6W[__\xFD[a\x01\x90a\x01\x8B6`\x04a\x19\xF4V[a\x02\xD4V[`@Qa\x01\x9D\x91\x90a\x1A\xA9V[`@Q\x80\x91\x03\x90\xF3[a\x01\xB9a\x01\xB46`\x04a\x19\xF4V[a\x03\x1FV[`@Qa\x01\x9D\x91\x90a\x1B\x0CV[a\x01\xCEa\x03\x91V[`@Qa\x01\x9D\x91\x90a\x1B\x1EV[a\x01\xE3a\x03\xFEV[`@Qa\x01\x9D\x91\x90a\x1B\xD0V[a\x01\xF8a\x05GV[\0[a\x01\xCEa\x06nV[a\x01\xCEa\x06\xD9V[a\x02\x1Da\x02\x186`\x04a\x19\xF4V[a\x07DV[`@Qa\x01\x9D\x91\x90a\x1CTV[a\x01\xF8a\x07\x87V[a\x02:a\x08\x90V[`@Qa\x01\x9D\x91\x90a\x1C\x8BV[a\x01\xF8a\t\x93V[a\x02Wa\n\x86V[`@Qa\x01\x9D\x91\x90a\x1D\\V[a\x02:a\x0BQV[a\x01\xF8a\x0CTV[a\x02Wa\x0C\x9EV[a\x02\x84a\riV[`@Q\x90\x15\x15\x81R` \x01a\x01\x9DV[a\x01\xCEa\x0E\xA6V[a\x01\xF8a\x0F\x11V[a\x01\xF8a\x10\x06V[a\x01\xF8a\x10\xFBV[`\x07Ta\x02\x84\x90`\xFF\x16\x81V[a\x02\x1Da\x02\xCF6`\x04a\x19\xF4V[a\x12xV[``a\x03\x17\x84\x84\x84`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01\x7Fhex\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x12\xBBV[\x94\x93PPPPV[``_a\x03-\x85\x85\x85a\x02\xD4V[\x90P_[a\x03;\x85\x85a\x1D\x9BV[\x81\x10\x15a\x03\x88W\x82\x82\x82\x81Q\x81\x10a\x03UWa\x03Ua\x1D\xAEV[` \x02` \x01\x01Q`@Q` \x01a\x03n\x92\x91\x90a\x1D\xF2V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x92P`\x01\x01a\x031V[PP\x93\x92PPPV[```\x14\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03\xF4W` \x02\x82\x01\x91\x90_R` _ \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03\xC9W[PPPPP\x90P\x90V[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05>W_\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x05'W\x83\x82\x90_R` _ \x01\x80Ta\x04\x9C\x90a\x1E\x06V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xC8\x90a\x1E\x06V[\x80\x15a\x05\x13W\x80`\x1F\x10a\x04\xEAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\x13V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xF6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x04\x7FV[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x04!V[PPPP\x90P\x90V[`@\x80Q\x80\x82\x01\x82R`\x1A\x81R\x7FInsufficient confirmations\0\0\0\0\0\0` \x82\x01R\x90Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\x05\xC8\x91\x90`\x04\x01a\x1B\x0CV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05\xDFW__\xFD[PZ\xF1\x15\x80\x15a\x05\xF1W=__>=_\xFD[PP`\x1CT` T`!T`@Qb]\t\xA7`\xE4\x1B\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x94Pc\x05\xD0\x9Ap\x93Pa\x06@\x92`\x1E\x92`\x1F\x92\x90\x91`\t\x90`\x04\x01a\x1F,V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06VW__\xFD[PZ\xFA\x15\x80\x15a\x06hW=__>=_\xFD[PPPPV[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03\xF4W` \x02\x82\x01\x91\x90_R` _ \x90\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03\xC9WPPPPP\x90P\x90V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03\xF4W` \x02\x82\x01\x91\x90_R` _ \x90\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03\xC9WPPPPP\x90P\x90V[``a\x03\x17\x84\x84\x84`@Q\x80`@\x01`@R\x80`\t\x81R` \x01\x7Fdigest_le\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x14\x1CV[`@\x80Q\x80\x82\x01\x82R`\x13\x81R\x7FBad inclusion proof\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\x08\x08\x91\x90`\x04\x01a\x1B\x0CV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08\x1FW__\xFD[PZ\xF1\x15\x80\x15a\x081W=__>=_\xFD[PP`\x1CT` T`!Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x93Pc\x05\xD0\x9Ap\x92P`\x1E\x91`\x1F\x91\x90a\x08o\x90`\x01a\x1FqV[_`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06@\x95\x94\x93\x92\x91\x90a\x1F,V[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05>W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\t{W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\t(W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x08\xB3V[`@\x80Q\x80\x82\x01\x82R`\x13\x81R\x7FBad inclusion proof\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\n\x14\x91\x90`\x04\x01a\x1B\x0CV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\n+W__\xFD[PZ\xF1\x15\x80\x15a\n=W=__>=_\xFD[PP`\x1CT`!T`@Qb]\t\xA7`\xE4\x1B\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x93Pc\x05\xD0\x9Ap\x92Pa\x06@\x91`\x1E\x91`\x1F\x91_\x90`\x04\x01a\x1F\x84V[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05>W\x83\x82\x90_R` _ \x01\x80Ta\n\xC6\x90a\x1E\x06V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\xF2\x90a\x1E\x06V[\x80\x15a\x0B=W\x80`\x1F\x10a\x0B\x14Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B=V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\n\xA9V[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05>W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x0C<W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x0B\xE9W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x0BtV[`\x1CT` T`!T`@Qb]\t\xA7`\xE4\x1B\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x92c\x05\xD0\x9Ap\x92a\x06@\x92`\x1E\x92`\x1F\x92\x91\x90_\x90`\x04\x01a\x1F,V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05>W\x83\x82\x90_R` _ \x01\x80Ta\x0C\xDE\x90a\x1E\x06V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r\n\x90a\x1E\x06V[\x80\x15a\rUW\x80`\x1F\x10a\r,Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\rUV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x0C\xC1V[`\x07T_\x90a\x01\0\x90\x04`\xFF\x16\x15a\r\x8AWP`\x07Ta\x01\0\x90\x04`\xFF\x16\x90V[_sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0E\xA1W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R_\x92\x90\x91a\x0E-\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a\x1F\xC9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0EG\x91a\x1F\xFCV[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x0E\x80W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x0E\x85V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x0E\x9D\x91\x90a \x07V[\x91PP[\x91\x90PV[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03\xF4W` \x02\x82\x01\x91\x90_R` _ \x90\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03\xC9WPPPPP\x90P\x90V[`@\x80Q\x80\x82\x01\x82R`\x13\x81R\x7FBad inclusion proof\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\x0F\x92\x91\x90`\x04\x01a\x1B\x0CV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0F\xA9W__\xFD[PZ\xF1\x15\x80\x15a\x0F\xBBW=__>=_\xFD[PP`\x1CT` T`!T`@Qb]\t\xA7`\xE4\x1B\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x94Pc\x05\xD0\x9Ap\x93Pa\x06@\x92`\x1E\x92\x91\x90_\x90`\x04\x01a -V[`@\x80Q\x80\x82\x01\x82R`\x16\x81R\x7FBad merkle array proof\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\x10\x87\x91\x90`\x04\x01a\x1B\x0CV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10\x9EW__\xFD[PZ\xF1\x15\x80\x15a\x10\xB0W=__>=_\xFD[PP`\x1CT` T`!T`@Qb]\t\xA7`\xE4\x1B\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x94Pc\x05\xD0\x9Ap\x93Pa\x06@\x92`\x1E\x92\x91\x90_\x90`\x04\x01a vV[`\x1CT`@Q\x7F\xF5\x8D\xB0o\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_`\x04\x82\x01\x81\x90R`$\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\xF5\x8D\xB0o\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x11jW__\xFD[PZ\xF1\x15\x80\x15a\x11|W=__>=_\xFD[PP`@\x80Q\x80\x82\x01\x82R`\x1B\x81R\x7FGCD does not confirm header\0\0\0\0\0` \x82\x01R\x90Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x93Pc\xF2\x8D\xCE\xB3\x92Pa\x12\x01\x91\x90`\x04\x01a\x1B\x0CV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x12\x18W__\xFD[PZ\xF1\x15\x80\x15a\x12*W=__>=_\xFD[PP`\x1CT` T`!T`@Qb]\t\xA7`\xE4\x1B\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x94Pc\x05\xD0\x9Ap\x93Pa\x06@\x92`\x1E\x92`\x1F\x92\x90\x91_\x90`\x04\x01a\x1F,V[``a\x03\x17\x84\x84\x84`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7Fheight\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x15jV[``a\x12\xC7\x84\x84a\x1D\x9BV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xDFWa\x12\xDFa\x19oV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13\x12W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x12\xFDW\x90P[P\x90P\x83[\x83\x81\x10\x15a\x14\x13Wa\x13\xE5\x86a\x13,\x83a\x16\xB8V[\x85`@Q` \x01a\x13?\x93\x92\x91\x90a \xBFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x80Ta\x13[\x90a\x1E\x06V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\x87\x90a\x1E\x06V[\x80\x15a\x13\xD2W\x80`\x1F\x10a\x13\xA9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\xD2V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13\xB5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa\x17\xE9\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82a\x13\xF0\x87\x84a\x1D\x9BV[\x81Q\x81\x10a\x14\0Wa\x14\0a\x1D\xAEV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x13\x17V[P\x94\x93PPPPV[``a\x14(\x84\x84a\x1D\x9BV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14@Wa\x14@a\x19oV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14iW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x83[\x83\x81\x10\x15a\x14\x13Wa\x15<\x86a\x14\x83\x83a\x16\xB8V[\x85`@Q` \x01a\x14\x96\x93\x92\x91\x90a \xBFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x80Ta\x14\xB2\x90a\x1E\x06V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14\xDE\x90a\x1E\x06V[\x80\x15a\x15)W\x80`\x1F\x10a\x15\0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15)V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\x0CW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa\x18\x88\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82a\x15G\x87\x84a\x1D\x9BV[\x81Q\x81\x10a\x15WWa\x15Wa\x1D\xAEV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x14nV[``a\x15v\x84\x84a\x1D\x9BV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\x8EWa\x15\x8Ea\x19oV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x15\xB7W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x83[\x83\x81\x10\x15a\x14\x13Wa\x16\x8A\x86a\x15\xD1\x83a\x16\xB8V[\x85`@Q` \x01a\x15\xE4\x93\x92\x91\x90a \xBFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x80Ta\x16\0\x90a\x1E\x06V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x16,\x90a\x1E\x06V[\x80\x15a\x16wW\x80`\x1F\x10a\x16NWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x16wV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16ZW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa\x19\x1B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82a\x16\x95\x87\x84a\x1D\x9BV[\x81Q\x81\x10a\x16\xA5Wa\x16\xA5a\x1D\xAEV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x15\xBCV[``\x81_\x03a\x16\xFAWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81_[\x81\x15a\x17#W\x80a\x17\r\x81a!\\V[\x91Pa\x17\x1C\x90P`\n\x83a!\xC0V[\x91Pa\x16\xFDV[_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17=Wa\x17=a\x19oV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x17gW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x03\x17Wa\x17|`\x01\x83a\x1D\x9BV[\x91Pa\x17\x89`\n\x86a!\xD3V[a\x17\x94\x90`0a\x1FqV[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x17\xA9Wa\x17\xA9a\x1D\xAEV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81_\x1A\x90SPa\x17\xE2`\n\x86a!\xC0V[\x94Pa\x17kV[`@Q\x7F\xFD\x92\x1B\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R``\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xFD\x92\x1B\xE8\x90a\x18>\x90\x86\x90\x86\x90`\x04\x01a!\xE6V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18XW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x18\x7F\x91\x90\x81\x01\x90a\"\x13V[\x90P[\x92\x91PPV[`@Q\x7F\x17w\xE5\x9D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x17w\xE5\x9D\x90a\x18\xDC\x90\x86\x90\x86\x90`\x04\x01a!\xE6V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xF7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\x7F\x91\x90a\"\x88V[`@Q\x7F\xAD\xDD\xE2\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xAD\xDD\xE2\xB6\x90a\x18\xDC\x90\x86\x90\x86\x90`\x04\x01a!\xE6V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x19\xC5Wa\x19\xC5a\x19oV[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x19\xE6Wa\x19\xE6a\x19oV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[___``\x84\x86\x03\x12\x15a\x1A\x06W__\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\x1CW__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x1A,W__\xFD[\x805a\x1A?a\x1A:\x82a\x19\xCDV[a\x19\x9CV[\x81\x81R\x87` \x83\x85\x01\x01\x11\x15a\x1ASW__\xFD[\x81` \x84\x01` \x83\x017_` \x92\x82\x01\x83\x01R\x97\x90\x86\x015\x96P`@\x90\x95\x015\x94\x93PPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x1B\0W`?\x19\x87\x86\x03\x01\x84Ra\x1A\xEB\x85\x83Qa\x1A{V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x1A\xCFV[P\x92\x96\x95PPPPPPV[` \x81R_a\x18\x7F` \x83\x01\x84a\x1A{V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x1BkW\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1B7V[P\x90\x95\x94PPPPPV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15a\x1B\xC4W`\x1F\x19\x85\x84\x03\x01\x88Ra\x1B\xAE\x83\x83Qa\x1A{V[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01a\x1B\x92V[P\x90\x96\x95PPPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x1B\0W`?\x19\x87\x86\x03\x01\x84R\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x86R` \x81\x01Q\x90P`@` \x87\x01Ra\x1C>`@\x87\x01\x82a\x1BvV[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x1B\xF6V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x1BkW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1CmV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x1B\0W\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90_\x90``\x88\x01\x90[\x80\x83\x10\x15a\x1DDW\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84Q\x16\x82R` \x82\x01\x91P` \x84\x01\x93P`\x01\x83\x01\x92Pa\x1C\xFFV[P\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x1C\xB1V[` \x81R_a\x18\x7F` \x83\x01\x84a\x1BvV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x18\x82Wa\x18\x82a\x1DnV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a\x03\x17a\x1E\0\x83\x86a\x1D\xDBV[\x84a\x1D\xDBV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1E\x1AW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1EQW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[P\x91\x90PV[\x80T_\x90`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1EoW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1E\xA6W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x81\x86R` \x86\x01\x81\x80\x15a\x1E\xC1W`\x01\x81\x14a\x1E\xF5Wa\x1F!V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x85\x16\x82R\x83\x15\x15`\x05\x1B\x82\x01\x95Pa\x1F!V[_\x87\x81R` \x90 _[\x85\x81\x10\x15a\x1F\x1BW\x81T\x84\x82\x01R`\x01\x90\x91\x01\x90` \x01a\x1E\xFFV[\x83\x01\x96PP[PPPPP\x92\x91PPV[`\xA0\x81R_a\x1F>`\xA0\x83\x01\x88a\x1EWV[\x82\x81\x03` \x84\x01Ra\x1FP\x81\x88a\x1EWV[`@\x84\x01\x96\x90\x96RPP``\x81\x01\x92\x90\x92R`\xFF\x16`\x80\x90\x91\x01R\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x18\x82Wa\x18\x82a\x1DnV[`\xA0\x81R_a\x1F\x96`\xA0\x83\x01\x87a\x1EWV[\x82\x81\x03` \x84\x01Ra\x1F\xA8\x81\x87a\x1EWV[\x91PP_`@\x83\x01R\x83``\x83\x01R`\xFF\x83\x16`\x80\x83\x01R\x95\x94PPPPPV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81R_a\x03\x17`\x04\x83\x01\x84a\x1D\xDBV[_a\x18\x7F\x82\x84a\x1D\xDBV[_` \x82\x84\x03\x12\x15a \x17W__\xFD[\x81Q\x80\x15\x15\x81\x14a &W__\xFD[\x93\x92PPPV[`\xA0\x81R_a ?`\xA0\x83\x01\x87a\x1EWV[\x82\x81\x03` \x84\x01R` \x81R_` \x82\x01R`@\x81\x01\x91PP\x84`@\x83\x01R\x83``\x83\x01R`\xFF\x83\x16`\x80\x83\x01R\x95\x94PPPPPV[`\xA0\x81R_a \x88`\xA0\x83\x01\x87a\x1EWV[\x82\x81\x03` \x84\x01R`\x01\x81R_` \x82\x01R`@\x81\x01\x91PP\x84`@\x83\x01R\x83``\x83\x01R`\xFF\x83\x16`\x80\x83\x01R\x95\x94PPPPPV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_a \xF0`\x01\x83\x01\x86a\x1D\xDBV[\x7F[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra! `\x01\x82\x01\x86a\x1D\xDBV[\x90P\x7F].\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra!R`\x02\x82\x01\x85a\x1D\xDBV[\x96\x95PPPPPPV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a!\x8CWa!\x8Ca\x1DnV[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82a!\xCEWa!\xCEa!\x93V[P\x04\x90V[_\x82a!\xE1Wa!\xE1a!\x93V[P\x06\x90V[`@\x81R_a!\xF8`@\x83\x01\x85a\x1A{V[\x82\x81\x03` \x84\x01Ra\"\n\x81\x85a\x1A{V[\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\"#W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"9W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\"IW__\xFD[\x80Qa\"Wa\x1A:\x82a\x19\xCDV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\"kW__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a\"\x98W__\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 \x8B\xBB_\xFD\x8C\x8FC\x8E\xE4\x17\\\xE4\x180A)\xF2\x14\xB7\xA1\xB6y\x9B\xD9\xB0\x80\xA2M\xBB\xBEQLdsolcC\0\x08\x1B\x003",
    );
    /**Event with signature `log(string)` and selector `0x41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f50`.
```solidity
event log(string);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log(string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                65u8,
                48u8,
                79u8,
                172u8,
                217u8,
                50u8,
                61u8,
                117u8,
                177u8,
                27u8,
                205u8,
                214u8,
                9u8,
                203u8,
                56u8,
                239u8,
                255u8,
                253u8,
                176u8,
                87u8,
                16u8,
                247u8,
                202u8,
                240u8,
                233u8,
                177u8,
                108u8,
                109u8,
                157u8,
                112u8,
                159u8,
                80u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_address(address)` and selector `0x7ae74c527414ae135fd97047b12921a5ec3911b804197855d67e25c7b75ee6f3`.
```solidity
event log_address(address);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_address {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_address {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_address(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                122u8,
                231u8,
                76u8,
                82u8,
                116u8,
                20u8,
                174u8,
                19u8,
                95u8,
                217u8,
                112u8,
                71u8,
                177u8,
                41u8,
                33u8,
                165u8,
                236u8,
                57u8,
                17u8,
                184u8,
                4u8,
                25u8,
                120u8,
                85u8,
                214u8,
                126u8,
                37u8,
                199u8,
                183u8,
                94u8,
                230u8,
                243u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_address {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_address> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_address) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_array(uint256[])` and selector `0xfb102865d50addddf69da9b5aa1bced66c80cf869a5c8d0471a467e18ce9cab1`.
```solidity
event log_array(uint256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_array_0 {
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_array_0 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                251u8,
                16u8,
                40u8,
                101u8,
                213u8,
                10u8,
                221u8,
                221u8,
                246u8,
                157u8,
                169u8,
                181u8,
                170u8,
                27u8,
                206u8,
                214u8,
                108u8,
                128u8,
                207u8,
                134u8,
                154u8,
                92u8,
                141u8,
                4u8,
                113u8,
                164u8,
                103u8,
                225u8,
                140u8,
                233u8,
                202u8,
                177u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { val: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_array_0 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_array_0> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_array_0) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_array(int256[])` and selector `0x890a82679b470f2bd82816ed9b161f97d8b967f37fa3647c21d5bf39749e2dd5`.
```solidity
event log_array(int256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_array_1 {
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::I256,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_array_1 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Int<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(int256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                137u8,
                10u8,
                130u8,
                103u8,
                155u8,
                71u8,
                15u8,
                43u8,
                216u8,
                40u8,
                22u8,
                237u8,
                155u8,
                22u8,
                31u8,
                151u8,
                216u8,
                185u8,
                103u8,
                243u8,
                127u8,
                163u8,
                100u8,
                124u8,
                33u8,
                213u8,
                191u8,
                57u8,
                116u8,
                158u8,
                45u8,
                213u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { val: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Int<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_array_1 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_array_1> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_array_1) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_array(address[])` and selector `0x40e1840f5769073d61bd01372d9b75baa9842d5629a0c99ff103be1178a8e9e2`.
```solidity
event log_array(address[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_array_2 {
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_array_2 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(address[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                64u8,
                225u8,
                132u8,
                15u8,
                87u8,
                105u8,
                7u8,
                61u8,
                97u8,
                189u8,
                1u8,
                55u8,
                45u8,
                155u8,
                117u8,
                186u8,
                169u8,
                132u8,
                45u8,
                86u8,
                41u8,
                160u8,
                201u8,
                159u8,
                241u8,
                3u8,
                190u8,
                17u8,
                120u8,
                168u8,
                233u8,
                226u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { val: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_array_2 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_array_2> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_array_2) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_bytes(bytes)` and selector `0x23b62ad0584d24a75f0bf3560391ef5659ec6db1269c56e11aa241d637f19b20`.
```solidity
event log_bytes(bytes);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_bytes {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_bytes {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_bytes(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                35u8,
                182u8,
                42u8,
                208u8,
                88u8,
                77u8,
                36u8,
                167u8,
                95u8,
                11u8,
                243u8,
                86u8,
                3u8,
                145u8,
                239u8,
                86u8,
                89u8,
                236u8,
                109u8,
                177u8,
                38u8,
                156u8,
                86u8,
                225u8,
                26u8,
                162u8,
                65u8,
                214u8,
                55u8,
                241u8,
                155u8,
                32u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_bytes {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_bytes> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_bytes) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_bytes32(bytes32)` and selector `0xe81699b85113eea1c73e10588b2b035e55893369632173afd43feb192fac64e3`.
```solidity
event log_bytes32(bytes32);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_bytes32 {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_bytes32 {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_bytes32(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                232u8,
                22u8,
                153u8,
                184u8,
                81u8,
                19u8,
                238u8,
                161u8,
                199u8,
                62u8,
                16u8,
                88u8,
                139u8,
                43u8,
                3u8,
                94u8,
                85u8,
                137u8,
                51u8,
                105u8,
                99u8,
                33u8,
                115u8,
                175u8,
                212u8,
                63u8,
                235u8,
                25u8,
                47u8,
                172u8,
                100u8,
                227u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_bytes32 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_bytes32> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_bytes32) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_int(int256)` and selector `0x0eb5d52624c8d28ada9fc55a8c502ed5aa3fbe2fb6e91b71b5f376882b1d2fb8`.
```solidity
event log_int(int256);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_int {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::I256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_int {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Int<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_int(int256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                14u8,
                181u8,
                213u8,
                38u8,
                36u8,
                200u8,
                210u8,
                138u8,
                218u8,
                159u8,
                197u8,
                90u8,
                140u8,
                80u8,
                46u8,
                213u8,
                170u8,
                63u8,
                190u8,
                47u8,
                182u8,
                233u8,
                27u8,
                113u8,
                181u8,
                243u8,
                118u8,
                136u8,
                43u8,
                29u8,
                47u8,
                184u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_int {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_int> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_int) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_address(string,address)` and selector `0x9c4e8541ca8f0dc1c413f9108f66d82d3cecb1bddbce437a61caa3175c4cc96f`.
```solidity
event log_named_address(string key, address val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_address {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_address {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_address(string,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                156u8,
                78u8,
                133u8,
                65u8,
                202u8,
                143u8,
                13u8,
                193u8,
                196u8,
                19u8,
                249u8,
                16u8,
                143u8,
                102u8,
                216u8,
                45u8,
                60u8,
                236u8,
                177u8,
                189u8,
                219u8,
                206u8,
                67u8,
                122u8,
                97u8,
                202u8,
                163u8,
                23u8,
                92u8,
                76u8,
                201u8,
                111u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.val,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_named_address {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_address> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_address) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_array(string,uint256[])` and selector `0x00aaa39c9ffb5f567a4534380c737075702e1f7f14107fc95328e3b56c0325fb`.
```solidity
event log_named_array(string key, uint256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_array_0 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_array_0 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                0u8,
                170u8,
                163u8,
                156u8,
                159u8,
                251u8,
                95u8,
                86u8,
                122u8,
                69u8,
                52u8,
                56u8,
                12u8,
                115u8,
                112u8,
                117u8,
                112u8,
                46u8,
                31u8,
                127u8,
                20u8,
                16u8,
                127u8,
                201u8,
                83u8,
                40u8,
                227u8,
                181u8,
                108u8,
                3u8,
                37u8,
                251u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_named_array_0 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_array_0> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_array_0) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_array(string,int256[])` and selector `0xa73eda09662f46dde729be4611385ff34fe6c44fbbc6f7e17b042b59a3445b57`.
```solidity
event log_named_array(string key, int256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_array_1 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::I256,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_array_1 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Int<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,int256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                167u8,
                62u8,
                218u8,
                9u8,
                102u8,
                47u8,
                70u8,
                221u8,
                231u8,
                41u8,
                190u8,
                70u8,
                17u8,
                56u8,
                95u8,
                243u8,
                79u8,
                230u8,
                196u8,
                79u8,
                187u8,
                198u8,
                247u8,
                225u8,
                123u8,
                4u8,
                43u8,
                89u8,
                163u8,
                68u8,
                91u8,
                87u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Int<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_named_array_1 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_array_1> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_array_1) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_array(string,address[])` and selector `0x3bcfb2ae2e8d132dd1fce7cf278a9a19756a9fceabe470df3bdabb4bc577d1bd`.
```solidity
event log_named_array(string key, address[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_array_2 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_array_2 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,address[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                59u8,
                207u8,
                178u8,
                174u8,
                46u8,
                141u8,
                19u8,
                45u8,
                209u8,
                252u8,
                231u8,
                207u8,
                39u8,
                138u8,
                154u8,
                25u8,
                117u8,
                106u8,
                159u8,
                206u8,
                171u8,
                228u8,
                112u8,
                223u8,
                59u8,
                218u8,
                187u8,
                75u8,
                197u8,
                119u8,
                209u8,
                189u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_named_array_2 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_array_2> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_array_2) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_bytes(string,bytes)` and selector `0xd26e16cad4548705e4c9e2d94f98ee91c289085ee425594fd5635fa2964ccf18`.
```solidity
event log_named_bytes(string key, bytes val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_bytes {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_bytes {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_bytes(string,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                210u8,
                110u8,
                22u8,
                202u8,
                212u8,
                84u8,
                135u8,
                5u8,
                228u8,
                201u8,
                226u8,
                217u8,
                79u8,
                152u8,
                238u8,
                145u8,
                194u8,
                137u8,
                8u8,
                94u8,
                228u8,
                37u8,
                89u8,
                79u8,
                213u8,
                99u8,
                95u8,
                162u8,
                150u8,
                76u8,
                207u8,
                24u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.val,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_named_bytes {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_bytes> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_bytes) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_bytes32(string,bytes32)` and selector `0xafb795c9c61e4fe7468c386f925d7a5429ecad9c0495ddb8d38d690614d32f99`.
```solidity
event log_named_bytes32(string key, bytes32 val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_bytes32 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_bytes32 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_bytes32(string,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                175u8,
                183u8,
                149u8,
                201u8,
                198u8,
                30u8,
                79u8,
                231u8,
                70u8,
                140u8,
                56u8,
                111u8,
                146u8,
                93u8,
                122u8,
                84u8,
                41u8,
                236u8,
                173u8,
                156u8,
                4u8,
                149u8,
                221u8,
                184u8,
                211u8,
                141u8,
                105u8,
                6u8,
                20u8,
                211u8,
                47u8,
                153u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_named_bytes32 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_bytes32> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_bytes32) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_decimal_int(string,int256,uint256)` and selector `0x5da6ce9d51151ba10c09a559ef24d520b9dac5c5b8810ae8434e4d0d86411a95`.
```solidity
event log_named_decimal_int(string key, int256 val, uint256 decimals);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_decimal_int {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::I256,
        #[allow(missing_docs)]
        pub decimals: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_decimal_int {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Int<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_decimal_int(string,int256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                93u8,
                166u8,
                206u8,
                157u8,
                81u8,
                21u8,
                27u8,
                161u8,
                12u8,
                9u8,
                165u8,
                89u8,
                239u8,
                36u8,
                213u8,
                32u8,
                185u8,
                218u8,
                197u8,
                197u8,
                184u8,
                129u8,
                10u8,
                232u8,
                67u8,
                78u8,
                77u8,
                13u8,
                134u8,
                65u8,
                26u8,
                149u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    key: data.0,
                    val: data.1,
                    decimals: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decimals),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_named_decimal_int {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_decimal_int> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_decimal_int) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_decimal_uint(string,uint256,uint256)` and selector `0xeb8ba43ced7537421946bd43e828b8b2b8428927aa8f801c13d934bf11aca57b`.
```solidity
event log_named_decimal_uint(string key, uint256 val, uint256 decimals);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_decimal_uint {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub decimals: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_decimal_uint {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_decimal_uint(string,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                235u8,
                139u8,
                164u8,
                60u8,
                237u8,
                117u8,
                55u8,
                66u8,
                25u8,
                70u8,
                189u8,
                67u8,
                232u8,
                40u8,
                184u8,
                178u8,
                184u8,
                66u8,
                137u8,
                39u8,
                170u8,
                143u8,
                128u8,
                28u8,
                19u8,
                217u8,
                52u8,
                191u8,
                17u8,
                172u8,
                165u8,
                123u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    key: data.0,
                    val: data.1,
                    decimals: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decimals),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_named_decimal_uint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_decimal_uint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_decimal_uint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_int(string,int256)` and selector `0x2fe632779174374378442a8e978bccfbdcc1d6b2b0d81f7e8eb776ab2286f168`.
```solidity
event log_named_int(string key, int256 val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_int {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::I256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_int {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Int<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_int(string,int256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                47u8,
                230u8,
                50u8,
                119u8,
                145u8,
                116u8,
                55u8,
                67u8,
                120u8,
                68u8,
                42u8,
                142u8,
                151u8,
                139u8,
                204u8,
                251u8,
                220u8,
                193u8,
                214u8,
                178u8,
                176u8,
                216u8,
                31u8,
                126u8,
                142u8,
                183u8,
                118u8,
                171u8,
                34u8,
                134u8,
                241u8,
                104u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_named_int {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_int> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_int) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_string(string,string)` and selector `0x280f4446b28a1372417dda658d30b95b2992b12ac9c7f378535f29a97acf3583`.
```solidity
event log_named_string(string key, string val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_string {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_string {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_string(string,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                40u8,
                15u8,
                68u8,
                70u8,
                178u8,
                138u8,
                19u8,
                114u8,
                65u8,
                125u8,
                218u8,
                101u8,
                141u8,
                48u8,
                185u8,
                91u8,
                41u8,
                146u8,
                177u8,
                42u8,
                201u8,
                199u8,
                243u8,
                120u8,
                83u8,
                95u8,
                41u8,
                169u8,
                122u8,
                207u8,
                53u8,
                131u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.val,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_named_string {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_string> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_string) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_uint(string,uint256)` and selector `0xb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a8`.
```solidity
event log_named_uint(string key, uint256 val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_uint {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_uint {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_uint(string,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                178u8,
                222u8,
                47u8,
                190u8,
                128u8,
                26u8,
                13u8,
                246u8,
                192u8,
                203u8,
                221u8,
                253u8,
                68u8,
                139u8,
                163u8,
                196u8,
                29u8,
                72u8,
                160u8,
                64u8,
                202u8,
                53u8,
                197u8,
                108u8,
                129u8,
                150u8,
                239u8,
                15u8,
                202u8,
                231u8,
                33u8,
                168u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_named_uint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_uint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_uint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_string(string)` and selector `0x0b2e13ff20ac7b474198655583edf70dedd2c1dc980e329c4fbb2fc0748b796b`.
```solidity
event log_string(string);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_string {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_string {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_string(string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                11u8,
                46u8,
                19u8,
                255u8,
                32u8,
                172u8,
                123u8,
                71u8,
                65u8,
                152u8,
                101u8,
                85u8,
                131u8,
                237u8,
                247u8,
                13u8,
                237u8,
                210u8,
                193u8,
                220u8,
                152u8,
                14u8,
                50u8,
                156u8,
                79u8,
                187u8,
                47u8,
                192u8,
                116u8,
                139u8,
                121u8,
                107u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_string {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_string> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_string) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_uint(uint256)` and selector `0x2cab9790510fd8bdfbd2115288db33fec66691d476efc5427cfd4c0969301755`.
```solidity
event log_uint(uint256);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_uint {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_uint {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_uint(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                44u8,
                171u8,
                151u8,
                144u8,
                81u8,
                15u8,
                216u8,
                189u8,
                251u8,
                210u8,
                17u8,
                82u8,
                136u8,
                219u8,
                51u8,
                254u8,
                198u8,
                102u8,
                145u8,
                212u8,
                118u8,
                239u8,
                197u8,
                66u8,
                124u8,
                253u8,
                76u8,
                9u8,
                105u8,
                48u8,
                23u8,
                85u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_uint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_uint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_uint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `logs(bytes)` and selector `0xe7950ede0394b9f2ce4a5a1bf5a7e1852411f7e6661b4308c913c4bfd11027e4`.
```solidity
event logs(bytes);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct logs {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for logs {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "logs(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                231u8,
                149u8,
                14u8,
                222u8,
                3u8,
                148u8,
                185u8,
                242u8,
                206u8,
                74u8,
                90u8,
                27u8,
                245u8,
                167u8,
                225u8,
                133u8,
                36u8,
                17u8,
                247u8,
                230u8,
                102u8,
                27u8,
                67u8,
                8u8,
                201u8,
                19u8,
                196u8,
                191u8,
                209u8,
                16u8,
                39u8,
                228u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for logs {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&logs> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &logs) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {}
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Function with signature `IS_TEST()` and selector `0xfa7626d4`.
```solidity
function IS_TEST() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IS_TESTCall {}
    ///Container type for the return parameters of the [`IS_TEST()`](IS_TESTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IS_TESTReturn {
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<IS_TESTCall> for UnderlyingRustTuple<'_> {
                fn from(value: IS_TESTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for IS_TESTCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<IS_TESTReturn> for UnderlyingRustTuple<'_> {
                fn from(value: IS_TESTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for IS_TESTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for IS_TESTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = IS_TESTReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IS_TEST()";
            const SELECTOR: [u8; 4] = [250u8, 118u8, 38u8, 212u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `excludeArtifacts()` and selector `0xb5508aa9`.
```solidity
function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeArtifactsCall {}
    ///Container type for the return parameters of the [`excludeArtifacts()`](excludeArtifactsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeArtifactsReturn {
        pub excludedArtifacts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeArtifactsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeArtifactsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeArtifactsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeArtifactsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeArtifactsReturn) -> Self {
                    (value.excludedArtifacts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeArtifactsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedArtifacts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeArtifactsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeArtifactsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeArtifacts()";
            const SELECTOR: [u8; 4] = [181u8, 80u8, 138u8, 169u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `excludeContracts()` and selector `0xe20c9f71`.
```solidity
function excludeContracts() external view returns (address[] memory excludedContracts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeContractsCall {}
    ///Container type for the return parameters of the [`excludeContracts()`](excludeContractsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeContractsReturn {
        pub excludedContracts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeContractsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeContractsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeContractsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeContractsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeContractsReturn) -> Self {
                    (value.excludedContracts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeContractsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedContracts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeContractsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeContractsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeContracts()";
            const SELECTOR: [u8; 4] = [226u8, 12u8, 159u8, 113u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `excludeSenders()` and selector `0x1ed7831c`.
```solidity
function excludeSenders() external view returns (address[] memory excludedSenders_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSendersCall {}
    ///Container type for the return parameters of the [`excludeSenders()`](excludeSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSendersReturn {
        pub excludedSenders_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeSendersCall> for UnderlyingRustTuple<'_> {
                fn from(value: excludeSendersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for excludeSendersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeSendersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeSendersReturn) -> Self {
                    (value.excludedSenders_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { excludedSenders_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeSendersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeSendersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeSenders()";
            const SELECTOR: [u8; 4] = [30u8, 215u8, 131u8, 28u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `failed()` and selector `0xba414fa6`.
```solidity
function failed() external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct failedCall {}
    ///Container type for the return parameters of the [`failed()`](failedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct failedReturn {
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<failedCall> for UnderlyingRustTuple<'_> {
                fn from(value: failedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for failedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<failedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: failedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for failedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for failedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = failedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "failed()";
            const SELECTOR: [u8; 4] = [186u8, 65u8, 79u8, 166u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getBlockHeights(string,uint256,uint256)` and selector `0xfad06b8f`.
```solidity
function getBlockHeights(string memory chainName, uint256 from, uint256 to) external view returns (uint256[] memory elements);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBlockHeightsCall {
        pub chainName: alloy::sol_types::private::String,
        pub from: alloy::sol_types::private::primitives::aliases::U256,
        pub to: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getBlockHeights(string,uint256,uint256)`](getBlockHeightsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBlockHeightsReturn {
        pub elements: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::String,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getBlockHeightsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getBlockHeightsCall) -> Self {
                    (value.chainName, value.from, value.to)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBlockHeightsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        chainName: tuple.0,
                        from: tuple.1,
                        to: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getBlockHeightsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBlockHeightsReturn) -> Self {
                    (value.elements,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBlockHeightsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { elements: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBlockHeightsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBlockHeightsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBlockHeights(string,uint256,uint256)";
            const SELECTOR: [u8; 4] = [250u8, 208u8, 107u8, 143u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.chainName,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.from),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.to),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getDigestLes(string,uint256,uint256)` and selector `0x44badbb6`.
```solidity
function getDigestLes(string memory chainName, uint256 from, uint256 to) external view returns (bytes32[] memory elements);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDigestLesCall {
        pub chainName: alloy::sol_types::private::String,
        pub from: alloy::sol_types::private::primitives::aliases::U256,
        pub to: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getDigestLes(string,uint256,uint256)`](getDigestLesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDigestLesReturn {
        pub elements: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::String,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getDigestLesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getDigestLesCall) -> Self {
                    (value.chainName, value.from, value.to)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getDigestLesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        chainName: tuple.0,
                        from: tuple.1,
                        to: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getDigestLesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getDigestLesReturn) -> Self {
                    (value.elements,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getDigestLesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { elements: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getDigestLesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getDigestLesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getDigestLes(string,uint256,uint256)";
            const SELECTOR: [u8; 4] = [68u8, 186u8, 219u8, 182u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.chainName,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.from),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.to),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getHeaderHexes(string,uint256,uint256)` and selector `0x0813852a`.
```solidity
function getHeaderHexes(string memory chainName, uint256 from, uint256 to) external view returns (bytes[] memory elements);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHeaderHexesCall {
        pub chainName: alloy::sol_types::private::String,
        pub from: alloy::sol_types::private::primitives::aliases::U256,
        pub to: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getHeaderHexes(string,uint256,uint256)`](getHeaderHexesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHeaderHexesReturn {
        pub elements: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::String,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getHeaderHexesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getHeaderHexesCall) -> Self {
                    (value.chainName, value.from, value.to)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getHeaderHexesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        chainName: tuple.0,
                        from: tuple.1,
                        to: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getHeaderHexesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getHeaderHexesReturn) -> Self {
                    (value.elements,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getHeaderHexesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { elements: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getHeaderHexesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getHeaderHexesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getHeaderHexes(string,uint256,uint256)";
            const SELECTOR: [u8; 4] = [8u8, 19u8, 133u8, 42u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.chainName,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.from),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.to),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getHeaders(string,uint256,uint256)` and selector `0x1c0da81f`.
```solidity
function getHeaders(string memory chainName, uint256 from, uint256 to) external view returns (bytes memory headers);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHeadersCall {
        pub chainName: alloy::sol_types::private::String,
        pub from: alloy::sol_types::private::primitives::aliases::U256,
        pub to: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getHeaders(string,uint256,uint256)`](getHeadersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHeadersReturn {
        pub headers: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::String,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getHeadersCall> for UnderlyingRustTuple<'_> {
                fn from(value: getHeadersCall) -> Self {
                    (value.chainName, value.from, value.to)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getHeadersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        chainName: tuple.0,
                        from: tuple.1,
                        to: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getHeadersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getHeadersReturn) -> Self {
                    (value.headers,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getHeadersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { headers: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getHeadersCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getHeadersReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getHeaders(string,uint256,uint256)";
            const SELECTOR: [u8; 4] = [28u8, 13u8, 168u8, 31u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.chainName,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.from),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.to),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `targetArtifactSelectors()` and selector `0x66d9a9a0`.
```solidity
function targetArtifactSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedArtifactSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactSelectorsCall {}
    ///Container type for the return parameters of the [`targetArtifactSelectors()`](targetArtifactSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactSelectorsReturn {
        pub targetedArtifactSelectors_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetArtifactSelectorsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetArtifactSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetArtifactSelectorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactSelectorsReturn) -> Self {
                    (value.targetedArtifactSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetArtifactSelectorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedArtifactSelectors_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetArtifactSelectorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetArtifactSelectorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetArtifactSelectors()";
            const SELECTOR: [u8; 4] = [102u8, 217u8, 169u8, 160u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `targetArtifacts()` and selector `0x85226c81`.
```solidity
function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactsCall {}
    ///Container type for the return parameters of the [`targetArtifacts()`](targetArtifactsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactsReturn {
        pub targetedArtifacts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetArtifactsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetArtifactsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetArtifactsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactsReturn) -> Self {
                    (value.targetedArtifacts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetArtifactsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedArtifacts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetArtifactsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetArtifactsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetArtifacts()";
            const SELECTOR: [u8; 4] = [133u8, 34u8, 108u8, 129u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `targetContracts()` and selector `0x3f7286f4`.
```solidity
function targetContracts() external view returns (address[] memory targetedContracts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetContractsCall {}
    ///Container type for the return parameters of the [`targetContracts()`](targetContractsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetContractsReturn {
        pub targetedContracts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetContractsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetContractsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetContractsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetContractsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetContractsReturn) -> Self {
                    (value.targetedContracts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetContractsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedContracts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetContractsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetContractsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetContracts()";
            const SELECTOR: [u8; 4] = [63u8, 114u8, 134u8, 244u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `targetInterfaces()` and selector `0x2ade3880`.
```solidity
function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetInterfacesCall {}
    ///Container type for the return parameters of the [`targetInterfaces()`](targetInterfacesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetInterfacesReturn {
        pub targetedInterfaces_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzInterface as alloy::sol_types::SolType>::RustType,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetInterfacesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetInterfacesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetInterfacesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzInterface>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzInterface as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetInterfacesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetInterfacesReturn) -> Self {
                    (value.targetedInterfaces_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetInterfacesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedInterfaces_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetInterfacesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetInterfacesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzInterface>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetInterfaces()";
            const SELECTOR: [u8; 4] = [42u8, 222u8, 56u8, 128u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `targetSelectors()` and selector `0x916a17c6`.
```solidity
function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSelectorsCall {}
    ///Container type for the return parameters of the [`targetSelectors()`](targetSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSelectorsReturn {
        pub targetedSelectors_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetSelectorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetSelectorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetSelectorsReturn) -> Self {
                    (value.targetedSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetSelectorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedSelectors_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetSelectorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetSelectorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetSelectors()";
            const SELECTOR: [u8; 4] = [145u8, 106u8, 23u8, 198u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `targetSenders()` and selector `0x3e5e3c23`.
```solidity
function targetSenders() external view returns (address[] memory targetedSenders_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSendersCall {}
    ///Container type for the return parameters of the [`targetSenders()`](targetSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSendersReturn {
        pub targetedSenders_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetSendersCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetSendersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSendersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetSendersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: targetSendersReturn) -> Self {
                    (value.targetedSenders_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { targetedSenders_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetSendersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetSendersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetSenders()";
            const SELECTOR: [u8; 4] = [62u8, 94u8, 60u8, 35u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `testGCDDoesntConfirmHeader()` and selector `0xf11d5cbc`.
```solidity
function testGCDDoesntConfirmHeader() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testGCDDoesntConfirmHeaderCall {}
    ///Container type for the return parameters of the [`testGCDDoesntConfirmHeader()`](testGCDDoesntConfirmHeaderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testGCDDoesntConfirmHeaderReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<testGCDDoesntConfirmHeaderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testGCDDoesntConfirmHeaderCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testGCDDoesntConfirmHeaderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<testGCDDoesntConfirmHeaderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testGCDDoesntConfirmHeaderReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testGCDDoesntConfirmHeaderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testGCDDoesntConfirmHeaderCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testGCDDoesntConfirmHeaderReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testGCDDoesntConfirmHeader()";
            const SELECTOR: [u8; 4] = [241u8, 29u8, 92u8, 188u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `testIncorrectProofSupplied()` and selector `0xe89f8419`.
```solidity
function testIncorrectProofSupplied() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIncorrectProofSuppliedCall {}
    ///Container type for the return parameters of the [`testIncorrectProofSupplied()`](testIncorrectProofSuppliedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIncorrectProofSuppliedReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<testIncorrectProofSuppliedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIncorrectProofSuppliedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIncorrectProofSuppliedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<testIncorrectProofSuppliedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIncorrectProofSuppliedReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIncorrectProofSuppliedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIncorrectProofSuppliedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIncorrectProofSuppliedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIncorrectProofSupplied()";
            const SELECTOR: [u8; 4] = [232u8, 159u8, 132u8, 25u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `testIncorrectTxIdSupplied()` and selector `0x641e144d`.
```solidity
function testIncorrectTxIdSupplied() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIncorrectTxIdSuppliedCall {}
    ///Container type for the return parameters of the [`testIncorrectTxIdSupplied()`](testIncorrectTxIdSuppliedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIncorrectTxIdSuppliedReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<testIncorrectTxIdSuppliedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIncorrectTxIdSuppliedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIncorrectTxIdSuppliedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<testIncorrectTxIdSuppliedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIncorrectTxIdSuppliedReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIncorrectTxIdSuppliedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIncorrectTxIdSuppliedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIncorrectTxIdSuppliedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIncorrectTxIdSupplied()";
            const SELECTOR: [u8; 4] = [100u8, 30u8, 20u8, 77u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `testIncorrectTxSupplied()` and selector `0x740dba74`.
```solidity
function testIncorrectTxSupplied() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIncorrectTxSuppliedCall {}
    ///Container type for the return parameters of the [`testIncorrectTxSupplied()`](testIncorrectTxSuppliedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIncorrectTxSuppliedReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<testIncorrectTxSuppliedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIncorrectTxSuppliedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIncorrectTxSuppliedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<testIncorrectTxSuppliedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIncorrectTxSuppliedReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIncorrectTxSuppliedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIncorrectTxSuppliedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIncorrectTxSuppliedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIncorrectTxSupplied()";
            const SELECTOR: [u8; 4] = [116u8, 13u8, 186u8, 116u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `testInsufficientConfirmations()` and selector `0x39425f8f`.
```solidity
function testInsufficientConfirmations() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testInsufficientConfirmationsCall {}
    ///Container type for the return parameters of the [`testInsufficientConfirmations()`](testInsufficientConfirmationsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testInsufficientConfirmationsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<testInsufficientConfirmationsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testInsufficientConfirmationsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testInsufficientConfirmationsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<testInsufficientConfirmationsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testInsufficientConfirmationsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testInsufficientConfirmationsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testInsufficientConfirmationsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testInsufficientConfirmationsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testInsufficientConfirmations()";
            const SELECTOR: [u8; 4] = [57u8, 66u8, 95u8, 143u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `testMalformedProofSupplied()` and selector `0xf0c0832e`.
```solidity
function testMalformedProofSupplied() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testMalformedProofSuppliedCall {}
    ///Container type for the return parameters of the [`testMalformedProofSupplied()`](testMalformedProofSuppliedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testMalformedProofSuppliedReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<testMalformedProofSuppliedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testMalformedProofSuppliedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testMalformedProofSuppliedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<testMalformedProofSuppliedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testMalformedProofSuppliedReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testMalformedProofSuppliedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testMalformedProofSuppliedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testMalformedProofSuppliedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testMalformedProofSupplied()";
            const SELECTOR: [u8; 4] = [240u8, 192u8, 131u8, 46u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `testSuccessfullyVerify()` and selector `0xb52b2058`.
```solidity
function testSuccessfullyVerify() external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testSuccessfullyVerifyCall {}
    ///Container type for the return parameters of the [`testSuccessfullyVerify()`](testSuccessfullyVerifyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testSuccessfullyVerifyReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<testSuccessfullyVerifyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testSuccessfullyVerifyCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testSuccessfullyVerifyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<testSuccessfullyVerifyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testSuccessfullyVerifyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testSuccessfullyVerifyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testSuccessfullyVerifyCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testSuccessfullyVerifyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testSuccessfullyVerify()";
            const SELECTOR: [u8; 4] = [181u8, 43u8, 32u8, 88u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`FullRelayWithVerifyTest`](self) function calls.
    pub enum FullRelayWithVerifyTestCalls {
        IS_TEST(IS_TESTCall),
        excludeArtifacts(excludeArtifactsCall),
        excludeContracts(excludeContractsCall),
        excludeSenders(excludeSendersCall),
        failed(failedCall),
        getBlockHeights(getBlockHeightsCall),
        getDigestLes(getDigestLesCall),
        getHeaderHexes(getHeaderHexesCall),
        getHeaders(getHeadersCall),
        targetArtifactSelectors(targetArtifactSelectorsCall),
        targetArtifacts(targetArtifactsCall),
        targetContracts(targetContractsCall),
        targetInterfaces(targetInterfacesCall),
        targetSelectors(targetSelectorsCall),
        targetSenders(targetSendersCall),
        testGCDDoesntConfirmHeader(testGCDDoesntConfirmHeaderCall),
        testIncorrectProofSupplied(testIncorrectProofSuppliedCall),
        testIncorrectTxIdSupplied(testIncorrectTxIdSuppliedCall),
        testIncorrectTxSupplied(testIncorrectTxSuppliedCall),
        testInsufficientConfirmations(testInsufficientConfirmationsCall),
        testMalformedProofSupplied(testMalformedProofSuppliedCall),
        testSuccessfullyVerify(testSuccessfullyVerifyCall),
    }
    #[automatically_derived]
    impl FullRelayWithVerifyTestCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [8u8, 19u8, 133u8, 42u8],
            [28u8, 13u8, 168u8, 31u8],
            [30u8, 215u8, 131u8, 28u8],
            [42u8, 222u8, 56u8, 128u8],
            [57u8, 66u8, 95u8, 143u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [68u8, 186u8, 219u8, 182u8],
            [100u8, 30u8, 20u8, 77u8],
            [102u8, 217u8, 169u8, 160u8],
            [116u8, 13u8, 186u8, 116u8],
            [133u8, 34u8, 108u8, 129u8],
            [145u8, 106u8, 23u8, 198u8],
            [181u8, 43u8, 32u8, 88u8],
            [181u8, 80u8, 138u8, 169u8],
            [186u8, 65u8, 79u8, 166u8],
            [226u8, 12u8, 159u8, 113u8],
            [232u8, 159u8, 132u8, 25u8],
            [240u8, 192u8, 131u8, 46u8],
            [241u8, 29u8, 92u8, 188u8],
            [250u8, 118u8, 38u8, 212u8],
            [250u8, 208u8, 107u8, 143u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for FullRelayWithVerifyTestCalls {
        const NAME: &'static str = "FullRelayWithVerifyTestCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 22usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::excludeArtifacts(_) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeContracts(_) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeSenders(_) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::failed(_) => <failedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getBlockHeights(_) => {
                    <getBlockHeightsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getDigestLes(_) => {
                    <getDigestLesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getHeaderHexes(_) => {
                    <getHeaderHexesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getHeaders(_) => {
                    <getHeadersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetArtifactSelectors(_) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetArtifacts(_) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetContracts(_) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetInterfaces(_) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetSelectors(_) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetSenders(_) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testGCDDoesntConfirmHeader(_) => {
                    <testGCDDoesntConfirmHeaderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIncorrectProofSupplied(_) => {
                    <testIncorrectProofSuppliedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIncorrectTxIdSupplied(_) => {
                    <testIncorrectTxIdSuppliedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIncorrectTxSupplied(_) => {
                    <testIncorrectTxSuppliedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testInsufficientConfirmations(_) => {
                    <testInsufficientConfirmationsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testMalformedProofSupplied(_) => {
                    <testMalformedProofSuppliedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testSuccessfullyVerify(_) => {
                    <testSuccessfullyVerifyCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<FullRelayWithVerifyTestCalls>] = &[
                {
                    fn getHeaderHexes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<FullRelayWithVerifyTestCalls> {
                        <getHeaderHexesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(FullRelayWithVerifyTestCalls::getHeaderHexes)
                    }
                    getHeaderHexes
                },
                {
                    fn getHeaders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<FullRelayWithVerifyTestCalls> {
                        <getHeadersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(FullRelayWithVerifyTestCalls::getHeaders)
                    }
                    getHeaders
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<FullRelayWithVerifyTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(FullRelayWithVerifyTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<FullRelayWithVerifyTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(FullRelayWithVerifyTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn testInsufficientConfirmations(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<FullRelayWithVerifyTestCalls> {
                        <testInsufficientConfirmationsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyTestCalls::testInsufficientConfirmations,
                            )
                    }
                    testInsufficientConfirmations
                },
                {
                    fn targetSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<FullRelayWithVerifyTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(FullRelayWithVerifyTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<FullRelayWithVerifyTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(FullRelayWithVerifyTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn getDigestLes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<FullRelayWithVerifyTestCalls> {
                        <getDigestLesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(FullRelayWithVerifyTestCalls::getDigestLes)
                    }
                    getDigestLes
                },
                {
                    fn testIncorrectTxIdSupplied(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<FullRelayWithVerifyTestCalls> {
                        <testIncorrectTxIdSuppliedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(FullRelayWithVerifyTestCalls::testIncorrectTxIdSupplied)
                    }
                    testIncorrectTxIdSupplied
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<FullRelayWithVerifyTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(FullRelayWithVerifyTestCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn testIncorrectTxSupplied(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<FullRelayWithVerifyTestCalls> {
                        <testIncorrectTxSuppliedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(FullRelayWithVerifyTestCalls::testIncorrectTxSupplied)
                    }
                    testIncorrectTxSupplied
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<FullRelayWithVerifyTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(FullRelayWithVerifyTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<FullRelayWithVerifyTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(FullRelayWithVerifyTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn testSuccessfullyVerify(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<FullRelayWithVerifyTestCalls> {
                        <testSuccessfullyVerifyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(FullRelayWithVerifyTestCalls::testSuccessfullyVerify)
                    }
                    testSuccessfullyVerify
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<FullRelayWithVerifyTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(FullRelayWithVerifyTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<FullRelayWithVerifyTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(FullRelayWithVerifyTestCalls::failed)
                    }
                    failed
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<FullRelayWithVerifyTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(FullRelayWithVerifyTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn testIncorrectProofSupplied(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<FullRelayWithVerifyTestCalls> {
                        <testIncorrectProofSuppliedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyTestCalls::testIncorrectProofSupplied,
                            )
                    }
                    testIncorrectProofSupplied
                },
                {
                    fn testMalformedProofSupplied(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<FullRelayWithVerifyTestCalls> {
                        <testMalformedProofSuppliedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyTestCalls::testMalformedProofSupplied,
                            )
                    }
                    testMalformedProofSupplied
                },
                {
                    fn testGCDDoesntConfirmHeader(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<FullRelayWithVerifyTestCalls> {
                        <testGCDDoesntConfirmHeaderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyTestCalls::testGCDDoesntConfirmHeader,
                            )
                    }
                    testGCDDoesntConfirmHeader
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<FullRelayWithVerifyTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(FullRelayWithVerifyTestCalls::IS_TEST)
                    }
                    IS_TEST
                },
                {
                    fn getBlockHeights(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<FullRelayWithVerifyTestCalls> {
                        <getBlockHeightsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(FullRelayWithVerifyTestCalls::getBlockHeights)
                    }
                    getBlockHeights
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::excludeArtifacts(inner) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeContracts(inner) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeSenders(inner) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getBlockHeights(inner) => {
                    <getBlockHeightsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getDigestLes(inner) => {
                    <getDigestLesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getHeaderHexes(inner) => {
                    <getHeaderHexesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getHeaders(inner) => {
                    <getHeadersCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::targetArtifactSelectors(inner) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetArtifacts(inner) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetContracts(inner) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetInterfaces(inner) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetSelectors(inner) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetSenders(inner) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testGCDDoesntConfirmHeader(inner) => {
                    <testGCDDoesntConfirmHeaderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIncorrectProofSupplied(inner) => {
                    <testIncorrectProofSuppliedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIncorrectTxIdSupplied(inner) => {
                    <testIncorrectTxIdSuppliedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIncorrectTxSupplied(inner) => {
                    <testIncorrectTxSuppliedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testInsufficientConfirmations(inner) => {
                    <testInsufficientConfirmationsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testMalformedProofSupplied(inner) => {
                    <testMalformedProofSuppliedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testSuccessfullyVerify(inner) => {
                    <testSuccessfullyVerifyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::excludeArtifacts(inner) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::excludeContracts(inner) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::excludeSenders(inner) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getBlockHeights(inner) => {
                    <getBlockHeightsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getDigestLes(inner) => {
                    <getDigestLesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getHeaderHexes(inner) => {
                    <getHeaderHexesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getHeaders(inner) => {
                    <getHeadersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetArtifactSelectors(inner) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetArtifacts(inner) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetContracts(inner) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetInterfaces(inner) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetSelectors(inner) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetSenders(inner) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testGCDDoesntConfirmHeader(inner) => {
                    <testGCDDoesntConfirmHeaderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIncorrectProofSupplied(inner) => {
                    <testIncorrectProofSuppliedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIncorrectTxIdSupplied(inner) => {
                    <testIncorrectTxIdSuppliedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIncorrectTxSupplied(inner) => {
                    <testIncorrectTxSuppliedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testInsufficientConfirmations(inner) => {
                    <testInsufficientConfirmationsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testMalformedProofSupplied(inner) => {
                    <testMalformedProofSuppliedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testSuccessfullyVerify(inner) => {
                    <testSuccessfullyVerifyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`FullRelayWithVerifyTest`](self) events.
    pub enum FullRelayWithVerifyTestEvents {
        log(log),
        log_address(log_address),
        log_array_0(log_array_0),
        log_array_1(log_array_1),
        log_array_2(log_array_2),
        log_bytes(log_bytes),
        log_bytes32(log_bytes32),
        log_int(log_int),
        log_named_address(log_named_address),
        log_named_array_0(log_named_array_0),
        log_named_array_1(log_named_array_1),
        log_named_array_2(log_named_array_2),
        log_named_bytes(log_named_bytes),
        log_named_bytes32(log_named_bytes32),
        log_named_decimal_int(log_named_decimal_int),
        log_named_decimal_uint(log_named_decimal_uint),
        log_named_int(log_named_int),
        log_named_string(log_named_string),
        log_named_uint(log_named_uint),
        log_string(log_string),
        log_uint(log_uint),
        logs(logs),
    }
    #[automatically_derived]
    impl FullRelayWithVerifyTestEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                0u8,
                170u8,
                163u8,
                156u8,
                159u8,
                251u8,
                95u8,
                86u8,
                122u8,
                69u8,
                52u8,
                56u8,
                12u8,
                115u8,
                112u8,
                117u8,
                112u8,
                46u8,
                31u8,
                127u8,
                20u8,
                16u8,
                127u8,
                201u8,
                83u8,
                40u8,
                227u8,
                181u8,
                108u8,
                3u8,
                37u8,
                251u8,
            ],
            [
                11u8,
                46u8,
                19u8,
                255u8,
                32u8,
                172u8,
                123u8,
                71u8,
                65u8,
                152u8,
                101u8,
                85u8,
                131u8,
                237u8,
                247u8,
                13u8,
                237u8,
                210u8,
                193u8,
                220u8,
                152u8,
                14u8,
                50u8,
                156u8,
                79u8,
                187u8,
                47u8,
                192u8,
                116u8,
                139u8,
                121u8,
                107u8,
            ],
            [
                14u8,
                181u8,
                213u8,
                38u8,
                36u8,
                200u8,
                210u8,
                138u8,
                218u8,
                159u8,
                197u8,
                90u8,
                140u8,
                80u8,
                46u8,
                213u8,
                170u8,
                63u8,
                190u8,
                47u8,
                182u8,
                233u8,
                27u8,
                113u8,
                181u8,
                243u8,
                118u8,
                136u8,
                43u8,
                29u8,
                47u8,
                184u8,
            ],
            [
                35u8,
                182u8,
                42u8,
                208u8,
                88u8,
                77u8,
                36u8,
                167u8,
                95u8,
                11u8,
                243u8,
                86u8,
                3u8,
                145u8,
                239u8,
                86u8,
                89u8,
                236u8,
                109u8,
                177u8,
                38u8,
                156u8,
                86u8,
                225u8,
                26u8,
                162u8,
                65u8,
                214u8,
                55u8,
                241u8,
                155u8,
                32u8,
            ],
            [
                40u8,
                15u8,
                68u8,
                70u8,
                178u8,
                138u8,
                19u8,
                114u8,
                65u8,
                125u8,
                218u8,
                101u8,
                141u8,
                48u8,
                185u8,
                91u8,
                41u8,
                146u8,
                177u8,
                42u8,
                201u8,
                199u8,
                243u8,
                120u8,
                83u8,
                95u8,
                41u8,
                169u8,
                122u8,
                207u8,
                53u8,
                131u8,
            ],
            [
                44u8,
                171u8,
                151u8,
                144u8,
                81u8,
                15u8,
                216u8,
                189u8,
                251u8,
                210u8,
                17u8,
                82u8,
                136u8,
                219u8,
                51u8,
                254u8,
                198u8,
                102u8,
                145u8,
                212u8,
                118u8,
                239u8,
                197u8,
                66u8,
                124u8,
                253u8,
                76u8,
                9u8,
                105u8,
                48u8,
                23u8,
                85u8,
            ],
            [
                47u8,
                230u8,
                50u8,
                119u8,
                145u8,
                116u8,
                55u8,
                67u8,
                120u8,
                68u8,
                42u8,
                142u8,
                151u8,
                139u8,
                204u8,
                251u8,
                220u8,
                193u8,
                214u8,
                178u8,
                176u8,
                216u8,
                31u8,
                126u8,
                142u8,
                183u8,
                118u8,
                171u8,
                34u8,
                134u8,
                241u8,
                104u8,
            ],
            [
                59u8,
                207u8,
                178u8,
                174u8,
                46u8,
                141u8,
                19u8,
                45u8,
                209u8,
                252u8,
                231u8,
                207u8,
                39u8,
                138u8,
                154u8,
                25u8,
                117u8,
                106u8,
                159u8,
                206u8,
                171u8,
                228u8,
                112u8,
                223u8,
                59u8,
                218u8,
                187u8,
                75u8,
                197u8,
                119u8,
                209u8,
                189u8,
            ],
            [
                64u8,
                225u8,
                132u8,
                15u8,
                87u8,
                105u8,
                7u8,
                61u8,
                97u8,
                189u8,
                1u8,
                55u8,
                45u8,
                155u8,
                117u8,
                186u8,
                169u8,
                132u8,
                45u8,
                86u8,
                41u8,
                160u8,
                201u8,
                159u8,
                241u8,
                3u8,
                190u8,
                17u8,
                120u8,
                168u8,
                233u8,
                226u8,
            ],
            [
                65u8,
                48u8,
                79u8,
                172u8,
                217u8,
                50u8,
                61u8,
                117u8,
                177u8,
                27u8,
                205u8,
                214u8,
                9u8,
                203u8,
                56u8,
                239u8,
                255u8,
                253u8,
                176u8,
                87u8,
                16u8,
                247u8,
                202u8,
                240u8,
                233u8,
                177u8,
                108u8,
                109u8,
                157u8,
                112u8,
                159u8,
                80u8,
            ],
            [
                93u8,
                166u8,
                206u8,
                157u8,
                81u8,
                21u8,
                27u8,
                161u8,
                12u8,
                9u8,
                165u8,
                89u8,
                239u8,
                36u8,
                213u8,
                32u8,
                185u8,
                218u8,
                197u8,
                197u8,
                184u8,
                129u8,
                10u8,
                232u8,
                67u8,
                78u8,
                77u8,
                13u8,
                134u8,
                65u8,
                26u8,
                149u8,
            ],
            [
                122u8,
                231u8,
                76u8,
                82u8,
                116u8,
                20u8,
                174u8,
                19u8,
                95u8,
                217u8,
                112u8,
                71u8,
                177u8,
                41u8,
                33u8,
                165u8,
                236u8,
                57u8,
                17u8,
                184u8,
                4u8,
                25u8,
                120u8,
                85u8,
                214u8,
                126u8,
                37u8,
                199u8,
                183u8,
                94u8,
                230u8,
                243u8,
            ],
            [
                137u8,
                10u8,
                130u8,
                103u8,
                155u8,
                71u8,
                15u8,
                43u8,
                216u8,
                40u8,
                22u8,
                237u8,
                155u8,
                22u8,
                31u8,
                151u8,
                216u8,
                185u8,
                103u8,
                243u8,
                127u8,
                163u8,
                100u8,
                124u8,
                33u8,
                213u8,
                191u8,
                57u8,
                116u8,
                158u8,
                45u8,
                213u8,
            ],
            [
                156u8,
                78u8,
                133u8,
                65u8,
                202u8,
                143u8,
                13u8,
                193u8,
                196u8,
                19u8,
                249u8,
                16u8,
                143u8,
                102u8,
                216u8,
                45u8,
                60u8,
                236u8,
                177u8,
                189u8,
                219u8,
                206u8,
                67u8,
                122u8,
                97u8,
                202u8,
                163u8,
                23u8,
                92u8,
                76u8,
                201u8,
                111u8,
            ],
            [
                167u8,
                62u8,
                218u8,
                9u8,
                102u8,
                47u8,
                70u8,
                221u8,
                231u8,
                41u8,
                190u8,
                70u8,
                17u8,
                56u8,
                95u8,
                243u8,
                79u8,
                230u8,
                196u8,
                79u8,
                187u8,
                198u8,
                247u8,
                225u8,
                123u8,
                4u8,
                43u8,
                89u8,
                163u8,
                68u8,
                91u8,
                87u8,
            ],
            [
                175u8,
                183u8,
                149u8,
                201u8,
                198u8,
                30u8,
                79u8,
                231u8,
                70u8,
                140u8,
                56u8,
                111u8,
                146u8,
                93u8,
                122u8,
                84u8,
                41u8,
                236u8,
                173u8,
                156u8,
                4u8,
                149u8,
                221u8,
                184u8,
                211u8,
                141u8,
                105u8,
                6u8,
                20u8,
                211u8,
                47u8,
                153u8,
            ],
            [
                178u8,
                222u8,
                47u8,
                190u8,
                128u8,
                26u8,
                13u8,
                246u8,
                192u8,
                203u8,
                221u8,
                253u8,
                68u8,
                139u8,
                163u8,
                196u8,
                29u8,
                72u8,
                160u8,
                64u8,
                202u8,
                53u8,
                197u8,
                108u8,
                129u8,
                150u8,
                239u8,
                15u8,
                202u8,
                231u8,
                33u8,
                168u8,
            ],
            [
                210u8,
                110u8,
                22u8,
                202u8,
                212u8,
                84u8,
                135u8,
                5u8,
                228u8,
                201u8,
                226u8,
                217u8,
                79u8,
                152u8,
                238u8,
                145u8,
                194u8,
                137u8,
                8u8,
                94u8,
                228u8,
                37u8,
                89u8,
                79u8,
                213u8,
                99u8,
                95u8,
                162u8,
                150u8,
                76u8,
                207u8,
                24u8,
            ],
            [
                231u8,
                149u8,
                14u8,
                222u8,
                3u8,
                148u8,
                185u8,
                242u8,
                206u8,
                74u8,
                90u8,
                27u8,
                245u8,
                167u8,
                225u8,
                133u8,
                36u8,
                17u8,
                247u8,
                230u8,
                102u8,
                27u8,
                67u8,
                8u8,
                201u8,
                19u8,
                196u8,
                191u8,
                209u8,
                16u8,
                39u8,
                228u8,
            ],
            [
                232u8,
                22u8,
                153u8,
                184u8,
                81u8,
                19u8,
                238u8,
                161u8,
                199u8,
                62u8,
                16u8,
                88u8,
                139u8,
                43u8,
                3u8,
                94u8,
                85u8,
                137u8,
                51u8,
                105u8,
                99u8,
                33u8,
                115u8,
                175u8,
                212u8,
                63u8,
                235u8,
                25u8,
                47u8,
                172u8,
                100u8,
                227u8,
            ],
            [
                235u8,
                139u8,
                164u8,
                60u8,
                237u8,
                117u8,
                55u8,
                66u8,
                25u8,
                70u8,
                189u8,
                67u8,
                232u8,
                40u8,
                184u8,
                178u8,
                184u8,
                66u8,
                137u8,
                39u8,
                170u8,
                143u8,
                128u8,
                28u8,
                19u8,
                217u8,
                52u8,
                191u8,
                17u8,
                172u8,
                165u8,
                123u8,
            ],
            [
                251u8,
                16u8,
                40u8,
                101u8,
                213u8,
                10u8,
                221u8,
                221u8,
                246u8,
                157u8,
                169u8,
                181u8,
                170u8,
                27u8,
                206u8,
                214u8,
                108u8,
                128u8,
                207u8,
                134u8,
                154u8,
                92u8,
                141u8,
                4u8,
                113u8,
                164u8,
                103u8,
                225u8,
                140u8,
                233u8,
                202u8,
                177u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for FullRelayWithVerifyTestEvents {
        const NAME: &'static str = "FullRelayWithVerifyTestEvents";
        const COUNT: usize = 22usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<log as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log)
                }
                Some(<log_address as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_address as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_address)
                }
                Some(<log_array_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_array_0)
                }
                Some(<log_array_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_array_1)
                }
                Some(<log_array_2 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_2 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_array_2)
                }
                Some(<log_bytes as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_bytes as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_bytes)
                }
                Some(<log_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_bytes32 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_bytes32)
                }
                Some(<log_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_int as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_int)
                }
                Some(
                    <log_named_address as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_address as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_address)
                }
                Some(
                    <log_named_array_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_array_0)
                }
                Some(
                    <log_named_array_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_array_1)
                }
                Some(
                    <log_named_array_2 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_2 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_array_2)
                }
                Some(<log_named_bytes as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_bytes as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_bytes)
                }
                Some(
                    <log_named_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_bytes32 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_bytes32)
                }
                Some(
                    <log_named_decimal_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_decimal_int as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_decimal_int)
                }
                Some(
                    <log_named_decimal_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_decimal_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_decimal_uint)
                }
                Some(<log_named_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_int as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_int)
                }
                Some(<log_named_string as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_string as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_string)
                }
                Some(<log_named_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_uint)
                }
                Some(<log_string as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_string as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_string)
                }
                Some(<log_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_uint)
                }
                Some(<logs as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <logs as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::logs)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for FullRelayWithVerifyTestEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::log(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_address(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_int(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_address(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_decimal_int(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_decimal_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_int(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_string(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_string(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::logs(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::log(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_address(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_int(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_address(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_decimal_int(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_decimal_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_int(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_string(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_string(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::logs(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`FullRelayWithVerifyTest`](self) contract instance.

See the [wrapper's documentation](`FullRelayWithVerifyTestInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> FullRelayWithVerifyTestInstance<T, P, N> {
        FullRelayWithVerifyTestInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<FullRelayWithVerifyTestInstance<T, P, N>>,
    > {
        FullRelayWithVerifyTestInstance::<T, P, N>::deploy(provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
        FullRelayWithVerifyTestInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`FullRelayWithVerifyTest`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`FullRelayWithVerifyTest`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct FullRelayWithVerifyTestInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for FullRelayWithVerifyTestInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("FullRelayWithVerifyTestInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > FullRelayWithVerifyTestInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`FullRelayWithVerifyTest`](self) contract instance.

See the [wrapper's documentation](`FullRelayWithVerifyTestInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
        ) -> alloy_contract::Result<FullRelayWithVerifyTestInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> FullRelayWithVerifyTestInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> FullRelayWithVerifyTestInstance<T, P, N> {
            FullRelayWithVerifyTestInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > FullRelayWithVerifyTestInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`IS_TEST`] function.
        pub fn IS_TEST(&self) -> alloy_contract::SolCallBuilder<T, &P, IS_TESTCall, N> {
            self.call_builder(&IS_TESTCall {})
        }
        ///Creates a new call builder for the [`excludeArtifacts`] function.
        pub fn excludeArtifacts(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, excludeArtifactsCall, N> {
            self.call_builder(&excludeArtifactsCall {})
        }
        ///Creates a new call builder for the [`excludeContracts`] function.
        pub fn excludeContracts(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, excludeContractsCall, N> {
            self.call_builder(&excludeContractsCall {})
        }
        ///Creates a new call builder for the [`excludeSenders`] function.
        pub fn excludeSenders(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, excludeSendersCall, N> {
            self.call_builder(&excludeSendersCall {})
        }
        ///Creates a new call builder for the [`failed`] function.
        pub fn failed(&self) -> alloy_contract::SolCallBuilder<T, &P, failedCall, N> {
            self.call_builder(&failedCall {})
        }
        ///Creates a new call builder for the [`getBlockHeights`] function.
        pub fn getBlockHeights(
            &self,
            chainName: alloy::sol_types::private::String,
            from: alloy::sol_types::private::primitives::aliases::U256,
            to: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBlockHeightsCall, N> {
            self.call_builder(
                &getBlockHeightsCall {
                    chainName,
                    from,
                    to,
                },
            )
        }
        ///Creates a new call builder for the [`getDigestLes`] function.
        pub fn getDigestLes(
            &self,
            chainName: alloy::sol_types::private::String,
            from: alloy::sol_types::private::primitives::aliases::U256,
            to: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getDigestLesCall, N> {
            self.call_builder(
                &getDigestLesCall {
                    chainName,
                    from,
                    to,
                },
            )
        }
        ///Creates a new call builder for the [`getHeaderHexes`] function.
        pub fn getHeaderHexes(
            &self,
            chainName: alloy::sol_types::private::String,
            from: alloy::sol_types::private::primitives::aliases::U256,
            to: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getHeaderHexesCall, N> {
            self.call_builder(
                &getHeaderHexesCall {
                    chainName,
                    from,
                    to,
                },
            )
        }
        ///Creates a new call builder for the [`getHeaders`] function.
        pub fn getHeaders(
            &self,
            chainName: alloy::sol_types::private::String,
            from: alloy::sol_types::private::primitives::aliases::U256,
            to: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getHeadersCall, N> {
            self.call_builder(
                &getHeadersCall {
                    chainName,
                    from,
                    to,
                },
            )
        }
        ///Creates a new call builder for the [`targetArtifactSelectors`] function.
        pub fn targetArtifactSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetArtifactSelectorsCall, N> {
            self.call_builder(&targetArtifactSelectorsCall {})
        }
        ///Creates a new call builder for the [`targetArtifacts`] function.
        pub fn targetArtifacts(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetArtifactsCall, N> {
            self.call_builder(&targetArtifactsCall {})
        }
        ///Creates a new call builder for the [`targetContracts`] function.
        pub fn targetContracts(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetContractsCall, N> {
            self.call_builder(&targetContractsCall {})
        }
        ///Creates a new call builder for the [`targetInterfaces`] function.
        pub fn targetInterfaces(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetInterfacesCall, N> {
            self.call_builder(&targetInterfacesCall {})
        }
        ///Creates a new call builder for the [`targetSelectors`] function.
        pub fn targetSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetSelectorsCall, N> {
            self.call_builder(&targetSelectorsCall {})
        }
        ///Creates a new call builder for the [`targetSenders`] function.
        pub fn targetSenders(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetSendersCall, N> {
            self.call_builder(&targetSendersCall {})
        }
        ///Creates a new call builder for the [`testGCDDoesntConfirmHeader`] function.
        pub fn testGCDDoesntConfirmHeader(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, testGCDDoesntConfirmHeaderCall, N> {
            self.call_builder(&testGCDDoesntConfirmHeaderCall {})
        }
        ///Creates a new call builder for the [`testIncorrectProofSupplied`] function.
        pub fn testIncorrectProofSupplied(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, testIncorrectProofSuppliedCall, N> {
            self.call_builder(&testIncorrectProofSuppliedCall {})
        }
        ///Creates a new call builder for the [`testIncorrectTxIdSupplied`] function.
        pub fn testIncorrectTxIdSupplied(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, testIncorrectTxIdSuppliedCall, N> {
            self.call_builder(&testIncorrectTxIdSuppliedCall {})
        }
        ///Creates a new call builder for the [`testIncorrectTxSupplied`] function.
        pub fn testIncorrectTxSupplied(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, testIncorrectTxSuppliedCall, N> {
            self.call_builder(&testIncorrectTxSuppliedCall {})
        }
        ///Creates a new call builder for the [`testInsufficientConfirmations`] function.
        pub fn testInsufficientConfirmations(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            testInsufficientConfirmationsCall,
            N,
        > {
            self.call_builder(
                &testInsufficientConfirmationsCall {
                },
            )
        }
        ///Creates a new call builder for the [`testMalformedProofSupplied`] function.
        pub fn testMalformedProofSupplied(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, testMalformedProofSuppliedCall, N> {
            self.call_builder(&testMalformedProofSuppliedCall {})
        }
        ///Creates a new call builder for the [`testSuccessfullyVerify`] function.
        pub fn testSuccessfullyVerify(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, testSuccessfullyVerifyCall, N> {
            self.call_builder(&testSuccessfullyVerifyCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > FullRelayWithVerifyTestInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`log`] event.
        pub fn log_filter(&self) -> alloy_contract::Event<T, &P, log, N> {
            self.event_filter::<log>()
        }
        ///Creates a new event filter for the [`log_address`] event.
        pub fn log_address_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_address, N> {
            self.event_filter::<log_address>()
        }
        ///Creates a new event filter for the [`log_array_0`] event.
        pub fn log_array_0_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_array_0, N> {
            self.event_filter::<log_array_0>()
        }
        ///Creates a new event filter for the [`log_array_1`] event.
        pub fn log_array_1_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_array_1, N> {
            self.event_filter::<log_array_1>()
        }
        ///Creates a new event filter for the [`log_array_2`] event.
        pub fn log_array_2_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_array_2, N> {
            self.event_filter::<log_array_2>()
        }
        ///Creates a new event filter for the [`log_bytes`] event.
        pub fn log_bytes_filter(&self) -> alloy_contract::Event<T, &P, log_bytes, N> {
            self.event_filter::<log_bytes>()
        }
        ///Creates a new event filter for the [`log_bytes32`] event.
        pub fn log_bytes32_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_bytes32, N> {
            self.event_filter::<log_bytes32>()
        }
        ///Creates a new event filter for the [`log_int`] event.
        pub fn log_int_filter(&self) -> alloy_contract::Event<T, &P, log_int, N> {
            self.event_filter::<log_int>()
        }
        ///Creates a new event filter for the [`log_named_address`] event.
        pub fn log_named_address_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_address, N> {
            self.event_filter::<log_named_address>()
        }
        ///Creates a new event filter for the [`log_named_array_0`] event.
        pub fn log_named_array_0_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_array_0, N> {
            self.event_filter::<log_named_array_0>()
        }
        ///Creates a new event filter for the [`log_named_array_1`] event.
        pub fn log_named_array_1_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_array_1, N> {
            self.event_filter::<log_named_array_1>()
        }
        ///Creates a new event filter for the [`log_named_array_2`] event.
        pub fn log_named_array_2_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_array_2, N> {
            self.event_filter::<log_named_array_2>()
        }
        ///Creates a new event filter for the [`log_named_bytes`] event.
        pub fn log_named_bytes_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_bytes, N> {
            self.event_filter::<log_named_bytes>()
        }
        ///Creates a new event filter for the [`log_named_bytes32`] event.
        pub fn log_named_bytes32_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_bytes32, N> {
            self.event_filter::<log_named_bytes32>()
        }
        ///Creates a new event filter for the [`log_named_decimal_int`] event.
        pub fn log_named_decimal_int_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_decimal_int, N> {
            self.event_filter::<log_named_decimal_int>()
        }
        ///Creates a new event filter for the [`log_named_decimal_uint`] event.
        pub fn log_named_decimal_uint_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_decimal_uint, N> {
            self.event_filter::<log_named_decimal_uint>()
        }
        ///Creates a new event filter for the [`log_named_int`] event.
        pub fn log_named_int_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_int, N> {
            self.event_filter::<log_named_int>()
        }
        ///Creates a new event filter for the [`log_named_string`] event.
        pub fn log_named_string_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_string, N> {
            self.event_filter::<log_named_string>()
        }
        ///Creates a new event filter for the [`log_named_uint`] event.
        pub fn log_named_uint_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_uint, N> {
            self.event_filter::<log_named_uint>()
        }
        ///Creates a new event filter for the [`log_string`] event.
        pub fn log_string_filter(&self) -> alloy_contract::Event<T, &P, log_string, N> {
            self.event_filter::<log_string>()
        }
        ///Creates a new event filter for the [`log_uint`] event.
        pub fn log_uint_filter(&self) -> alloy_contract::Event<T, &P, log_uint, N> {
            self.event_filter::<log_uint>()
        }
        ///Creates a new event filter for the [`logs`] event.
        pub fn logs_filter(&self) -> alloy_contract::Event<T, &P, logs, N> {
            self.event_filter::<logs>()
        }
    }
}

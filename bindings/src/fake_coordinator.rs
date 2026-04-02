///Module containing a contract's types and functions.
/**

```solidity
library StoffelCoordinator {
    type Round is uint8;
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod StoffelCoordinator {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Round(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Round> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl Round {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl From<u8> for Round {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<Round> for u8 {
            fn from(value: Round) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Round {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Round {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`StoffelCoordinator`](self) contract instance.

See the [wrapper's documentation](`StoffelCoordinatorInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> StoffelCoordinatorInstance<P, N> {
        StoffelCoordinatorInstance::<P, N>::new(address, __provider)
    }
    /**A [`StoffelCoordinator`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`StoffelCoordinator`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct StoffelCoordinatorInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for StoffelCoordinatorInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("StoffelCoordinatorInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > StoffelCoordinatorInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`StoffelCoordinator`](self) contract instance.

See the [wrapper's documentation](`StoffelCoordinatorInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
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
    impl<P: ::core::clone::Clone, N> StoffelCoordinatorInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> StoffelCoordinatorInstance<P, N> {
            StoffelCoordinatorInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > StoffelCoordinatorInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > StoffelCoordinatorInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library StoffelCoordinator {
    type Round is uint8;
}

interface FakeCoordinator {
    error AccessControlBadConfirmation();
    error AccessControlUnauthorizedAccount(address account, bytes32 neededRole);
    error AlreadyReceivedOutputShares(address client, address sender);
    error AlreadySubmittedInputs(address client);
    error ECDSAInvalidSignature();
    error ECDSAInvalidSignatureLength(uint256 length);
    error ECDSAInvalidSignatureS(bytes32 s);
    error IndexNotReserved(address client, uint256 index);
    error IndicesAlreadyReserved(address client);
    error NoIndicesReserved(address client);
    error NotAClient(address client);
    error NotAnExistingParty(address account);
    error NotAtRound(StoffelCoordinator.Round required, StoffelCoordinator.Round current);
    error NotEnoughIndices(uint256 requested, uint256 available);
    error NotEnoughMPCParties(uint256 current, uint256 required);
    error OwnableInvalidOwner(address owner);
    error OwnableUnauthorizedAccount(address account);
    error ZeroIndices(address client);
    error ZeroMaskedInput(address client);

    event ClientAuthenticated(address indexed client, bool success);
    event CoordinatorInitialized(address coordinator, uint256 timeofInitialization, uint256 creationBlock, address designatedParty);
    event EnoughPrivateOutputShares(address indexed client, bytes[] shares);
    event ExecutionDone(address executor, uint256 timeOfExecution);
    event IndexBufferEvent(uint256 totalIndices, address designatedParty);
    event InitializeStoffelAccessControl(uint256 nParties, uint256 t, address initializer);
    event InputCollectionStarted(address executor, uint256 timeOfExecution);
    event InputMaskReservationStarted(address executor, uint256 timeOfExecution);
    event MPCStarted(address executor, uint256 timeOfExecution);
    event MaskedInputEvent(address client, uint256 maskedInput, uint256 reservedIndex);
    event OutputSendingStarted(address executor, uint256 timeOfExecution);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event PreprocessingStarted(address designatedParty, uint256 timeOfExecution);
    event ReservedInputEvent(address client, uint256[] reservedIndices);
    event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole);
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);

    constructor(bytes32 stoffelProgramHash, uint256 t, address[] initialMpcNodes, uint256 nInputs);

    function CLIENT_ROLE() external view returns (bytes32);
    function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
    function DESIGNATED_PARTY_ROLE() external view returns (bytes32);
    function PARTY_ROLE() external view returns (bytes32);
    function authenticateClient(address clientAddr, bytes memory signature) external;
    function availableInputMasks() external view returns (uint256);
    function baseNonce() external view returns (uint256);
    function collectInputs() external;
    function creationBlock() external view returns (uint256);
    function creationTime() external view returns (uint256);
    function finalize() external;
    function getRoleAdmin(bytes32 role) external view returns (bytes32);
    function getRoleMember(bytes32 role, uint256 index) external view returns (address);
    function getRoleMemberCount(bytes32 role) external view returns (uint256);
    function getRoleMembers(bytes32 role) external view returns (address[] memory);
    function grantRole(bytes32 role, address account) external;
    function hasRole(bytes32 role, address account) external view returns (bool);
    function isDesignatedParty(address account) external view returns (bool);
    function isParty(address account) external view returns (bool);
    function obtainInputMasks(uint256 nIndices) external returns (uint256[] memory);
    function owner() external view returns (address);
    function renounceOwnership() external;
    function renounceRole(bytes32 role, address account) external;
    function reserveInputMasks() external;
    function resetAccessControl(uint256 t, address[] memory initialMpcNodes) external;
    function resetCoordinator(bytes32 stoffelProgramHash, uint256 t, address[] memory initialMpcNodes, uint256 nInputs) external;
    function resetInputManager(uint256 nIndicesToReserve, uint256 t) external;
    function revokeRole(bytes32 role, address account) external;
    function round() external view returns (StoffelCoordinator.Round);
    function sendOutputs() external;
    function sendPrivateOutputShares(address client, bytes memory shares) external;
    function sendPublicOutputs(bytes memory _publicOutputs) external;
    function startMpc() external;
    function startPreprocessing() external;
    function submitMaskedInput(uint256 maskedInput, uint256 reservedIndex) external;
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
    function transferOwnership(address newOwner) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "stoffelProgramHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "t",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "initialMpcNodes",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "nInputs",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "CLIENT_ROLE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "DEFAULT_ADMIN_ROLE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "DESIGNATED_PARTY_ROLE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "PARTY_ROLE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "authenticateClient",
    "inputs": [
      {
        "name": "clientAddr",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "availableInputMasks",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "baseNonce",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "collectInputs",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "creationBlock",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "creationTime",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "finalize",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getRoleAdmin",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRoleMember",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRoleMemberCount",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRoleMembers",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "grantRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "hasRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
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
    "name": "isDesignatedParty",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
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
    "name": "isParty",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
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
    "name": "obtainInputMasks",
    "inputs": [
      {
        "name": "nIndices",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "owner",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "renounceRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "reserveInputMasks",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "resetAccessControl",
    "inputs": [
      {
        "name": "t",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "initialMpcNodes",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "resetCoordinator",
    "inputs": [
      {
        "name": "stoffelProgramHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "t",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "initialMpcNodes",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "nInputs",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "resetInputManager",
    "inputs": [
      {
        "name": "nIndicesToReserve",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "t",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "revokeRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "round",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "enum StoffelCoordinator.Round"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "sendOutputs",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "sendPrivateOutputShares",
    "inputs": [
      {
        "name": "client",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "shares",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "sendPublicOutputs",
    "inputs": [
      {
        "name": "_publicOutputs",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "startMpc",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "startPreprocessing",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "submitMaskedInput",
    "inputs": [
      {
        "name": "maskedInput",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "reservedIndex",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "supportsInterface",
    "inputs": [
      {
        "name": "interfaceId",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
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
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "ClientAuthenticated",
    "inputs": [
      {
        "name": "client",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "success",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "CoordinatorInitialized",
    "inputs": [
      {
        "name": "coordinator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "timeofInitialization",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "creationBlock",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "designatedParty",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EnoughPrivateOutputShares",
    "inputs": [
      {
        "name": "client",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "shares",
        "type": "bytes[]",
        "indexed": false,
        "internalType": "bytes[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ExecutionDone",
    "inputs": [
      {
        "name": "executor",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "timeOfExecution",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "IndexBufferEvent",
    "inputs": [
      {
        "name": "totalIndices",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "designatedParty",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "InitializeStoffelAccessControl",
    "inputs": [
      {
        "name": "nParties",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "t",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "initializer",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "InputCollectionStarted",
    "inputs": [
      {
        "name": "executor",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "timeOfExecution",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "InputMaskReservationStarted",
    "inputs": [
      {
        "name": "executor",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "timeOfExecution",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "MPCStarted",
    "inputs": [
      {
        "name": "executor",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "timeOfExecution",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "MaskedInputEvent",
    "inputs": [
      {
        "name": "client",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "maskedInput",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "reservedIndex",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OutputSendingStarted",
    "inputs": [
      {
        "name": "executor",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "timeOfExecution",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PreprocessingStarted",
    "inputs": [
      {
        "name": "designatedParty",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "timeOfExecution",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ReservedInputEvent",
    "inputs": [
      {
        "name": "client",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "reservedIndices",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleAdminChanged",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "previousAdminRole",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "newAdminRole",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleGranted",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleRevoked",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AccessControlBadConfirmation",
    "inputs": []
  },
  {
    "type": "error",
    "name": "AccessControlUnauthorizedAccount",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "neededRole",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "AlreadyReceivedOutputShares",
    "inputs": [
      {
        "name": "client",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "AlreadySubmittedInputs",
    "inputs": [
      {
        "name": "client",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignatureLength",
    "inputs": [
      {
        "name": "length",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignatureS",
    "inputs": [
      {
        "name": "s",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "IndexNotReserved",
    "inputs": [
      {
        "name": "client",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "IndicesAlreadyReserved",
    "inputs": [
      {
        "name": "client",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "NoIndicesReserved",
    "inputs": [
      {
        "name": "client",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "NotAClient",
    "inputs": [
      {
        "name": "client",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "NotAnExistingParty",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "NotAtRound",
    "inputs": [
      {
        "name": "required",
        "type": "uint8",
        "internalType": "enum StoffelCoordinator.Round"
      },
      {
        "name": "current",
        "type": "uint8",
        "internalType": "enum StoffelCoordinator.Round"
      }
    ]
  },
  {
    "type": "error",
    "name": "NotEnoughIndices",
    "inputs": [
      {
        "name": "requested",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "available",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "NotEnoughMPCParties",
    "inputs": [
      {
        "name": "current",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "required",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "OwnableInvalidOwner",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "OwnableUnauthorizedAccount",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ZeroIndices",
    "inputs": [
      {
        "name": "client",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ZeroMaskedInput",
    "inputs": [
      {
        "name": "client",
        "type": "address",
        "internalType": "address"
      }
    ]
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod FakeCoordinator {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060405161632438038061632483398181016040528101906100319190611278565b83838383338184858561004a828261011c60201b60201c565b50505f600a8190555061006382826104de60201b60201c565b50505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16036100d5575f6040517f1e4fbdf70000000000000000000000000000000000000000000000000000000081526004016100cc9190611307565b60405180910390fd5b6100e48161087260201b60201c565b5061010f84835f815181106100fc576100fb611320565b5b602002602001015161093560201b60201c565b50505050505050506115ad565b5f600183600361012c919061137a565b61013691906113bb565b90508082511015610181578151816040517f3a2362680000000000000000000000000000000000000000000000000000000081526004016101789291906113fd565b60405180910390fd5b826002819055505f6101b87f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6109b860201b60201c565b90505f6101ea7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6109e060201b60201c565b90505f5f90505b81811015610251576102437f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e8483815181106102305761022f611320565b5b6020026020010151610a0760201b60201c565b5080806001019150506101f1565b505f6102827fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696109b860201b60201c565b90505f6102b47fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696109e060201b60201c565b90505f5f90505b8181101561031b5761030d7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698483815181106102fa576102f9611320565b5b6020026020010151610a0760201b60201c565b5080806001019150506102bb565b505f61034c7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c0252326109b860201b60201c565b90505f61037e7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c0252326109e060201b60201c565b90505f5f90505b818110156103e5576103d77fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c0252328483815181106103c4576103c3611320565b5b6020026020010151610a0760201b60201c565b508080600101915050610385565b505f5f90505b885181101561044c5761043e7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698a838151811061042b5761042a611320565b5b6020026020010151610a2060201b60201c565b5080806001019150506103eb565b506104977f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e895f8151811061048457610483611320565b5b6020026020010151610a2060201b60201c565b507fa2df7830e0bedef7b1116bf547b467b16b50b3bd23146c9e099878d14e89301a878a336040516104cb93929190611424565b60405180910390a1505050505050505050565b600754600a5f8282546104f191906113bb565b92505081905550816007819055505f6008819055505f60098190555080600b819055505f6105447fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696109b860201b60201c565b90505f6105767fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696109e060201b60201c565b90505f5f90505b60075481101561083057600c5f60055f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f5f82015f9055600182015f9055505060065f60055f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f61067c9190610fb4565b5f5f90505b8281101561076a5760035f60055f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206002015f85838151811061070d5761070c611320565b5b602002602001015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81549060ff02191690558080600101915050610681565b5060035f60055f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f5f82015f6107e79190610fc0565b600182015f9055505060055f8281526020019081526020015f205f6101000a81549073ffffffffffffffffffffffffffffffffffffffff0219169055808060010191505061057d565b507ff7f0872382dff5e698b284e12084e4e7894f830216dd80cb4e909b593a58f93560075433604051610864929190611459565b60405180910390a150505050565b5f600d5f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905081600d5f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a35050565b81600e8190555042600f81905550436010819055505f60115f6101000a81548160ff0219169083600681111561096e5761096d611480565b5b02179055507fdef1f08eb655f4a75f60bd6fd7e971112004abd846a612e46dab270770d24ca530600f54601054846040516109ac94939291906114ad565b60405180910390a15050565b60606109d960015f8481526020019081526020015f20610a3960201b60201c565b9050919050565b5f610a0060015f8481526020019081526020015f20610a5e60201b60201c565b9050919050565b5f610a188383610a7760201b60201c565b905092915050565b5f610a318383610ac060201b60201c565b905092915050565b60605f610a4d835f01610b0960201b60201c565b905060608190508092505050919050565b5f610a70825f01610b6260201b60201c565b9050919050565b5f5f610a898484610b7160201b60201c565b90508015610ab657610ab48360015f8781526020019081526020015f20610c6660201b90919060201c565b505b8091505092915050565b5f5f610ad28484610c9960201b60201c565b90508015610aff57610afd8360015f8781526020019081526020015f20610d8e60201b90919060201c565b505b8091505092915050565b6060815f01805480602002602001604051908101604052809291908181526020018280548015610b5657602002820191905f5260205f20905b815481526020019060010190808311610b42575b50505050509050919050565b5f815f01805490509050919050565b5f610b828383610dc160201b60201c565b15610c5c575f5f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550610bf9610e2460201b60201c565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b60405160405180910390a460019050610c60565b5f90505b92915050565b5f610c91835f018373ffffffffffffffffffffffffffffffffffffffff165f1b610e2b60201b60201c565b905092915050565b5f610caa8383610dc160201b60201c565b610d845760015f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550610d21610e2460201b60201c565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a460019050610d88565b5f90505b92915050565b5f610db9835f018373ffffffffffffffffffffffffffffffffffffffff165f1b610f2760201b60201c565b905092915050565b5f5f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f33905090565b5f5f836001015f8481526020019081526020015f205490505f8114610f1c575f600182610e5891906114f0565b90505f6001865f0180549050610e6e91906114f0565b9050808214610ed4575f865f018281548110610e8d57610e8c611320565b5b905f5260205f200154905080875f018481548110610eae57610ead611320565b5b905f5260205f20018190555083876001015f8381526020019081526020015f2081905550505b855f01805480610ee757610ee6611523565b5b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f905560019350505050610f21565b5f9150505b92915050565b5f610f388383610f9460201b60201c565b610f8a57825f0182908060018154018082558091505060019003905f5260205f20015f9091909190915055825f0180549050836001015f8481526020019081526020015f208190555060019050610f8e565b5f90505b92915050565b5f5f836001015f8481526020019081526020015f20541415905092915050565b505f81556001015f9055565b5080545f8255905f5260205f2090610fd89190610fdb565b50565b5f5b80821115610ffb578281015f610ff39190611000565b600101610fdd565b505090565b50805461100c9061157d565b5f825580601f1061101d5750611037565b601f0160209004905f5260205f2090611036919061103a565b5b50565b5f5b80821115611052578281015f905560010161103c565b505090565b5f604051905090565b5f5ffd5b5f5ffd5b5f819050919050565b61107a81611068565b8114611084575f5ffd5b50565b5f8151905061109581611071565b92915050565b5f819050919050565b6110ad8161109b565b81146110b7575f5ffd5b50565b5f815190506110c8816110a4565b92915050565b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b611118826110d2565b810181811067ffffffffffffffff82111715611137576111366110e2565b5b80604052505050565b5f611149611057565b9050611155828261110f565b919050565b5f67ffffffffffffffff821115611174576111736110e2565b5b602082029050602081019050919050565b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6111b282611189565b9050919050565b6111c2816111a8565b81146111cc575f5ffd5b50565b5f815190506111dd816111b9565b92915050565b5f6111f56111f08461115a565b611140565b9050808382526020820190506020840283018581111561121857611217611185565b5b835b81811015611241578061122d88826111cf565b84526020840193505060208101905061121a565b5050509392505050565b5f82601f83011261125f5761125e6110ce565b5b815161126f8482602086016111e3565b91505092915050565b5f5f5f5f608085870312156112905761128f611060565b5b5f61129d87828801611087565b94505060206112ae878288016110ba565b935050604085015167ffffffffffffffff8111156112cf576112ce611064565b5b6112db8782880161124b565b92505060606112ec878288016110ba565b91505092959194509250565b611301816111a8565b82525050565b5f60208201905061131a5f8301846112f8565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6113848261109b565b915061138f8361109b565b925082820261139d8161109b565b915082820484148315176113b4576113b361134d565b5b5092915050565b5f6113c58261109b565b91506113d08361109b565b92508282019050808211156113e8576113e761134d565b5b92915050565b6113f78161109b565b82525050565b5f6040820190506114105f8301856113ee565b61141d60208301846113ee565b9392505050565b5f6060820190506114375f8301866113ee565b61144460208301856113ee565b61145160408301846112f8565b949350505050565b5f60408201905061146c5f8301856113ee565b61147960208301846112f8565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b5f6080820190506114c05f8301876112f8565b6114cd60208301866113ee565b6114da60408301856113ee565b6114e760608301846112f8565b95945050505050565b5f6114fa8261109b565b91506115058361109b565b925082820390508181111561151d5761151c61134d565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061159457607f821691505b6020821081036115a7576115a6611550565b5b50919050565b614d6a806115ba5f395ff3fe608060405234801561000f575f5ffd5b5060043610610225575f3560e01c80637f35b5601161012e578063ca15c873116100b6578063eb8575de1161007a578063eb8575de146105cb578063ebae35e7146105e7578063f2fde38b14610617578063f6603c6114610633578063fc78b2e81461064f57610225565b8063ca15c87314610539578063cb9c4cc414610569578063d547741f14610573578063d8270dce1461058f578063eae6f652146105ad57610225565b8063a217fddf116100fd578063a217fddf146104bb578063a3246ad3146104d9578063af206f2814610509578063bb51fef014610525578063c079f4951461052f57610225565b80637f35b5601461041f5780638da5cb5b1461043d5780639010d07c1461045b57806391d148541461048b57610225565b8063248a9ca3116101b157806336568abe1161018057806336568abe146103c95780633b4338d1146103e55780634b8e6488146104015780634bb278f31461040b578063715018a61461041557610225565b8063248a9ca3146103555780632f2ff15d1461038557806330104c3e146103a157806333cc9a09146103bf57610225565b8063146ca531116101f8578063146ca531146102c157806317634514146102df5780631c7453db146102fd5780631ee4ee0f1461031b5780632328bd121461033757610225565b806301ffc9a7146102295780630bda81cf146102595780630d42eb6f1461027557806313ff6dd514610291575b5f5ffd5b610243600480360381019061023e919061373b565b61067f565b6040516102509190613780565b60405180910390f35b610273600480360381019061026e91906137cc565b6106f8565b005b61028f600480360381019061028a919061386b565b61094a565b005b6102ab60048036038101906102a69190613910565b61098b565b6040516102b89190613780565b60405180910390f35b6102c9610a07565b6040516102d691906139ae565b60405180910390f35b6102e7610a19565b6040516102f491906139d6565b60405180910390f35b610305610a1f565b60405161031291906139d6565b60405180910390f35b610335600480360381019061033091906139ef565b610a25565b005b61033f610fd3565b60405161034c91906139d6565b60405180910390f35b61036f600480360381019061036a9190613a7f565b610fe9565b60405161037c9190613ab9565b60405180910390f35b61039f600480360381019061039a9190613ad2565b611005565b005b6103a961103f565b6040516103b69190613ab9565b60405180910390f35b6103c7611063565b005b6103e360048036038101906103de9190613ad2565b6110dd565b005b6103ff60048036038101906103fa91906137cc565b6111a4565b005b6104096111dd565b005b610413611257565b005b61041d6112d1565b005b6104276112e4565b6040516104349190613ab9565b60405180910390f35b610445611308565b6040516104529190613b1f565b60405180910390f35b61047560048036038101906104709190613b38565b611330565b6040516104829190613b1f565b60405180910390f35b6104a560048036038101906104a09190613ad2565b61135c565b6040516104b29190613780565b60405180910390f35b6104c36113bf565b6040516104d09190613ab9565b60405180910390f35b6104f360048036038101906104ee9190613a7f565b6113c5565b6040516105009190613c2d565b60405180910390f35b610523600480360381019061051e9190613d95565b6113e7565b005b61052d611420565b005b61053761149a565b005b610553600480360381019061054e9190613a7f565b611514565b60405161056091906139d6565b60405180910390f35b610571611535565b005b61058d60048036038101906105889190613ad2565b6115ae565b005b6105976116a1565b6040516105a491906139d6565b60405180910390f35b6105b56116a7565b6040516105c29190613ab9565b60405180910390f35b6105e560048036038101906105e091906139ef565b6116cb565b005b61060160048036038101906105fc9190613def565b611c10565b60405161060e9190613ed1565b60405180910390f35b610631600480360381019061062c9190613910565b6120dc565b005b61064d60048036038101906106489190613ef1565b612160565b005b61066960048036038101906106649190613910565b6121c9565b6040516106769190613780565b60405180910390f35b5f7f07effe0d000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191614806106f157506106f0826121fb565b5b9050919050565b7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c02523261072281612274565b3373ffffffffffffffffffffffffffffffffffffffff1660055f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16146107c45733826040517fffabbae70000000000000000000000000000000000000000000000000000000081526004016107bb929190613f71565b60405180910390fd5b5f830361080857336040517f16923cea0000000000000000000000000000000000000000000000000000000081526004016107ff9190613b1f565b60405180910390fd5b5f600c5f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20600101541461088c57336040517f4f5fbfc30000000000000000000000000000000000000000000000000000000081526004016108839190613b1f565b60405180910390fd5b604051806040016040528083815260200184815250600c5f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f820151815f0155602082015181600101559050507fb89addd937f44f902c849596641837cd7af2fcecef22d2a7866fdc1ad9c0ae2e33848460405161092693929190613f98565b60405180910390a160095f81548092919061094090613ffa565b9190505550505050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61097481612274565b828260049182610985929190614259565b50505050565b5f610995826121c9565b6109d657816040517fabdce06a0000000000000000000000000000000000000000000000000000000081526004016109cd9190613b1f565b60405180910390fd5b610a007f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e8361135c565b9050919050565b60115f9054906101000a900460ff1681565b60105481565b600a5481565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469610a4f81612274565b610a797fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c0252328561135c565b610aba57836040517fa032ac6b000000000000000000000000000000000000000000000000000000008152600401610ab19190613b1f565b60405180910390fd5b5f60075490505f5f90505b600754811015610b47578573ffffffffffffffffffffffffffffffffffffffff1660055f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1603610b3a57809150610b47565b8080600101915050610ac5565b506007548103610bbe5760055f5f81526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040517f6faf9f05000000000000000000000000000000000000000000000000000000008152600401610bb59190613b1f565b60405180910390fd5b5f81600a54610bcd9190614326565b90505f610bff82604051602001610be491906139d6565b60405160208183030381529060405280519060200120612288565b90505f610c4f8288888080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050506122bb565b90508773ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603610ced5760065f8973ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20600160028110610cd457610cd3614359565b5b015f8154610ce190613ffa565b91905081905550610d51565b60065f8973ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f60028110610d3c57610d3b614359565b5b015f8154610d4990613ffa565b919050819055505b6001600b54610d609190614326565b60065f8a73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f60028110610daf57610dae614359565b5b01541080610e1b57506001600b54610dc79190614326565b60065f8a73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20600160028110610e1757610e16614359565b5b0154105b610e5a576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610e5190614406565b60405180910390fd5b6001600b54610e699190614326565b60065f8a73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f60028110610eb857610eb7614359565b5b015410610f12578773ffffffffffffffffffffffffffffffffffffffff167f407009200520f9f10584813c0b95441ab320f6b08d97ebdaaf1e824eedd9d7c35f604051610f059190613780565b60405180910390a2610fc9565b6001600b54610f219190614326565b60065f8a73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20600160028110610f7157610f70614359565b5b015410610fc8578773ffffffffffffffffffffffffffffffffffffffff167f407009200520f9f10584813c0b95441ab320f6b08d97ebdaaf1e824eedd9d7c36001604051610fbf9190613780565b60405180910390a25b5b5050505050505050565b5f600854600754610fe49190614424565b905090565b5f5f5f8381526020019081526020015f20600101549050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61102f81612274565b61103983836122e5565b50505050565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46981565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61108d81612274565b6003611098816122f8565b7f20f55ed0c92f2bb1c8825488e1e3c98463d024b2a42dbd24838c3f75260f43e933426040516110c9929190613f71565b60405180910390a16110d9612381565b5050565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469821480156111125750611111828261135c565b5b15611196575f61112183611514565b90505f600160025460036111359190614457565b61113f9190614326565b9050808203611193576001816111559190614424565b816040517f3a23626800000000000000000000000000000000000000000000000000000000815260040161118a929190614498565b60405180910390fd5b50505b6111a082826123ea565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6111ce81612274565b6111d88383612465565b505050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61120781612274565b6004611212816122f8565b7f301f8a3701f5b260197382dd7301078542144fe8fddd18083d6f6e09e4958a593342604051611243929190613f71565b60405180910390a1611253612381565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61128181612274565b600561128c816122f8565b7f24a873651d26fb5a462cb37a91071cdd4d09abeebfe0ed144329bed1cc359d0333426040516112bd929190613f71565b60405180910390a16112cd612381565b5050565b6112d96127ed565b6112e25f612874565b565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e81565b5f600d5f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b5f6113548260015f8681526020019081526020015f2061293790919063ffffffff16565b905092915050565b5f5f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f5f1b81565b60606113e060015f8481526020019081526020015f2061294e565b9050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61141181612274565b61141b838361296d565b505050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61144a81612274565b6002611455816122f8565b7f60edf9bdc7c4ea007cae1a9bbd03e41e5bfccd7231a6ec383c2edd7800f0d20c3342604051611486929190613f71565b60405180910390a1611496612381565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6114c481612274565b60016114cf816122f8565b7f67c4489f674d03c7d19a9e36735188de7c65e8d1e99eb3a2fd258a769eb14fff3342604051611500929190613f71565b60405180910390a1611510612381565b5050565b5f61152e60015f8481526020019081526020015f20612ced565b9050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61155f81612274565b5f611569816122f8565b7fbb709dea744f06d1b26e824deec2f7140c511266ee15d7a217838b31d8b0123d334260405161159a929190613f71565b60405180910390a16115aa612381565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6115d881612274565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698314801561160d575061160c838361135c565b5b15611691575f61161c84611514565b90505f600160025460036116309190614457565b61163a9190614326565b905080820361168e576001816116509190614424565b816040517f3a236268000000000000000000000000000000000000000000000000000000008152600401611685929190614498565b60405180910390fd5b50505b61169b8383612d00565b50505050565b600f5481565b7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c02523281565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696116f581612274565b61171f7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c0252328561135c565b61176057836040517fa032ac6b0000000000000000000000000000000000000000000000000000000081526004016117579190613b1f565b60405180910390fd5b5f60035f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f2060010154905060035f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206002015f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16156118705784336040517f08e554950000000000000000000000000000000000000000000000000000000081526004016118679291906144bf565b60405180910390fd5b6001600b5460036118819190614457565b61188b9190614326565b81106118cc576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016118c390614556565b60405180910390fd5b600160035f8773ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206002015f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550838360035f8873ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f0183815481106119b2576119b1614359565b5b905f5260205f200191826119c7929190614259565b50600160035f8773ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206001015f828254611a189190614326565b92505081905550600181611a2c9190614326565b905060016002546002611a3f9190614457565b611a499190614326565b8110611c09575f8167ffffffffffffffff811115611a6a57611a69613c5d565b5b604051908082528060200260200182016040528015611a9d57816020015b6060815260200190600190039081611a885790505b5090505f5f90505b82811015611bb85760035f8873ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f018181548110611afe57611afd614359565b5b905f5260205f20018054611b1190614078565b80601f0160208091040260200160405190810160405280929190818152602001828054611b3d90614078565b8015611b885780601f10611b5f57610100808354040283529160200191611b88565b820191905f5260205f20905b815481529060010190602001808311611b6b57829003601f168201915b5050505050828281518110611ba057611b9f614359565b5b60200260200101819052508080600101915050611aa5565b508573ffffffffffffffffffffffffffffffffffffffff167f23e91dbfae03758cb88d7f6252b5710afa53a19ffe0f4b4f75d7f2de0c5eabe982604051611bff919061468f565b60405180910390a2505b5050505050565b606060018214611c55576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611c4c9061471f565b60405180910390fd5b5f8203611c9957336040517fb2fd5518000000000000000000000000000000000000000000000000000000008152600401611c909190613b1f565b60405180910390fd5b5f600854600754611caa9190614424565b905080831115611cf35782816040517fdf3d75e2000000000000000000000000000000000000000000000000000000008152600401611cea929190614498565b60405180910390fd5b5f5f90505b600754811015611daf573373ffffffffffffffffffffffffffffffffffffffff1660055f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1603611da257336040517faca92f09000000000000000000000000000000000000000000000000000000008152600401611d999190613b1f565b60405180910390fd5b8080600101915050611cf8565b505f600184600854611dc19190614326565b611dcb9190614424565b90505f8467ffffffffffffffff811115611de857611de7613c5d565b5b604051908082528060200260200182016040528015611e165781602001602082028036833780820191505090505b5090505f60085490505b8260085411611ec5573360055f60085481526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506008548282600854611e8c9190614424565b81518110611e9d57611e9c614359565b5b60200260200101818152505060085f815480929190611ebb90613ffa565b9190505550611e20565b611eef7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c025232336122e5565b505f60035f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f2090506001600b546003611f419190614457565b611f4b9190614326565b67ffffffffffffffff811115611f6457611f63613c5d565b5b604051908082528060200260200182016040528015611f9757816020015b6060815260200190600190039081611f825790505b50815f019081611fa79190614b2c565b505f81600101819055505f611fdb7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696113c5565b90505f6120077fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469611514565b90505f5f90505b81811015612093575f846002015f85848151811061202f5761202e614359565b5b602002602001015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550808060010191505061200e565b507f1e14abe5d0cdb96adde7b9eca9b14bc08df623b5805afde5a3f0acadc2bf4f5b33866040516120c5929190614b8e565b60405180910390a184975050505050505050919050565b6120e46127ed565b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603612154575f6040517f1e4fbdf700000000000000000000000000000000000000000000000000000000815260040161214b9190613b1f565b60405180910390fd5b61215d81612874565b50565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61218a81612274565b612194848461296d565b61219e8285612465565b6121c285845f815181106121b5576121b4614359565b5b6020026020010151612d13565b5050505050565b5f6121f47fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698361135c565b9050919050565b5f7f5a05180f000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916148061226d575061226c82612d96565b5b9050919050565b61228581612280612e0f565b612e16565b50565b5f7f19457468657265756d205369676e6564204d6573736167653a0a3332000000005f5281601c52603c5f209050919050565b5f5f5f5f6122c98686612e67565b9250925092506122d98282612ebc565b82935050505092915050565b5f6122f0838361301e565b905092915050565b80600681111561230b5761230a61393b565b5b60115f9054906101000a900460ff16600681111561232c5761232b61393b565b5b1461237e578060115f9054906101000a900460ff166040517fbfa217d8000000000000000000000000000000000000000000000000000000008152600401612375929190614bbc565b60405180910390fd5b50565b600160115f9054906101000a900460ff1660068111156123a4576123a361393b565b5b6123ae9190614326565b60068111156123c0576123bf61393b565b5b60115f6101000a81548160ff021916908360068111156123e3576123e261393b565b5b0217905550565b6123f2612e0f565b73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614612456576040517f6697b23200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6124608282612d00565b505050565b600754600a5f8282546124789190614326565b92505081905550816007819055505f6008819055505f60098190555080600b819055505f6124c57fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696113c5565b90505f6124f17fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469611514565b90505f5f90505b6007548110156127ab57600c5f60055f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f5f82015f9055600182015f9055505060065f60055f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6125f79190613632565b5f5f90505b828110156126e55760035f60055f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206002015f85838151811061268857612687614359565b5b602002602001015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81549060ff021916905580806001019150506125fc565b5060035f60055f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f5f82015f612762919061363e565b600182015f9055505060055f8281526020019081526020015f205f6101000a81549073ffffffffffffffffffffffffffffffffffffffff021916905580806001019150506124f8565b507ff7f0872382dff5e698b284e12084e4e7894f830216dd80cb4e909b593a58f935600754336040516127df929190614be3565b60405180910390a150505050565b6127f5612e0f565b73ffffffffffffffffffffffffffffffffffffffff16612813611308565b73ffffffffffffffffffffffffffffffffffffffff161461287257612836612e0f565b6040517f118cdaa70000000000000000000000000000000000000000000000000000000081526004016128699190613b1f565b60405180910390fd5b565b5f600d5f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905081600d5f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a35050565b5f612944835f0183613061565b5f1c905092915050565b60605f61295c835f01613088565b905060608190508092505050919050565b5f600183600361297d9190614457565b6129879190614326565b905080825110156129d2578151816040517f3a2362680000000000000000000000000000000000000000000000000000000081526004016129c9929190614498565b60405180910390fd5b826002819055505f612a037f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6113c5565b90505f612a2f7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e611514565b90505f5f90505b81811015612a9057612a827f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e848381518110612a7557612a74614359565b5b6020026020010151612d00565b508080600101915050612a36565b505f612abb7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696113c5565b90505f612ae77fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469611514565b90505f5f90505b81811015612b4857612b3a7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469848381518110612b2d57612b2c614359565b5b6020026020010151612d00565b508080600101915050612aee565b505f612b737fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c0252326113c5565b90505f612b9f7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c025232611514565b90505f5f90505b81811015612c0057612bf27fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c025232848381518110612be557612be4614359565b5b6020026020010151612d00565b508080600101915050612ba6565b505f5f90505b8851811015612c6157612c537fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698a8381518110612c4657612c45614359565b5b60200260200101516122e5565b508080600101915050612c06565b50612ca67f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e895f81518110612c9957612c98614359565b5b60200260200101516122e5565b507fa2df7830e0bedef7b1116bf547b467b16b50b3bd23146c9e099878d14e89301a878a33604051612cda93929190614c0a565b60405180910390a1505050505050505050565b5f612cf9825f016130e1565b9050919050565b5f612d0b83836130f0565b905092915050565b81600e8190555042600f81905550436010819055505f60115f6101000a81548160ff02191690836006811115612d4c57612d4b61393b565b5b02179055507fdef1f08eb655f4a75f60bd6fd7e971112004abd846a612e46dab270770d24ca530600f5460105484604051612d8a9493929190614c3f565b60405180910390a15050565b5f7f7965db0b000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19161480612e085750612e0782613133565b5b9050919050565b5f33905090565b612e20828261135c565b612e635780826040517fe2517d3f000000000000000000000000000000000000000000000000000000008152600401612e5a929190614c82565b60405180910390fd5b5050565b5f5f5f6041845103612ea7575f5f5f602087015192506040870151915060608701515f1a9050612e998882858561319c565b955095509550505050612eb5565b5f600285515f1b9250925092505b9250925092565b5f6003811115612ecf57612ece61393b565b5b826003811115612ee257612ee161393b565b5b031561301a5760016003811115612efc57612efb61393b565b5b826003811115612f0f57612f0e61393b565b5b03612f46576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60026003811115612f5a57612f5961393b565b5b826003811115612f6d57612f6c61393b565b5b03612fb157805f1c6040517ffce698f7000000000000000000000000000000000000000000000000000000008152600401612fa891906139d6565b60405180910390fd5b600380811115612fc457612fc361393b565b5b826003811115612fd757612fd661393b565b5b0361301957806040517fd78bce0c0000000000000000000000000000000000000000000000000000000081526004016130109190613ab9565b60405180910390fd5b5b5050565b5f5f61302a8484613283565b90508015613057576130558360015f8781526020019081526020015f2061336c90919063ffffffff16565b505b8091505092915050565b5f825f01828154811061307757613076614359565b5b905f5260205f200154905092915050565b6060815f018054806020026020016040519081016040528092919081815260200182805480156130d557602002820191905f5260205f20905b8154815260200190600101908083116130c1575b50505050509050919050565b5f815f01805490509050919050565b5f5f6130fc8484613399565b90508015613129576131278360015f8781526020019081526020015f2061348290919063ffffffff16565b505b8091505092915050565b5f7f01ffc9a7000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916149050919050565b5f5f5f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c11156131d8575f600385925092509250613279565b5f6001888888886040515f81526020016040526040516131fb9493929190614cc4565b6020604051602081039080840390855afa15801561321b573d5f5f3e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff160361326c575f60015f5f1b93509350935050613279565b805f5f5f1b935093509350505b9450945094915050565b5f61328e838361135c565b6133625760015f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055506132ff612e0f565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a460019050613366565b5f90505b92915050565b5f613391835f018373ffffffffffffffffffffffffffffffffffffffff165f1b6134af565b905092915050565b5f6133a4838361135c565b15613478575f5f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550613415612e0f565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b60405160405180910390a46001905061347c565b5f90505b92915050565b5f6134a7835f018373ffffffffffffffffffffffffffffffffffffffff165f1b613516565b905092915050565b5f6134ba8383613612565b61350c57825f0182908060018154018082558091505060019003905f5260205f20015f9091909190915055825f0180549050836001015f8481526020019081526020015f208190555060019050613510565b5f90505b92915050565b5f5f836001015f8481526020019081526020015f205490505f8114613607575f6001826135439190614424565b90505f6001865f01805490506135599190614424565b90508082146135bf575f865f01828154811061357857613577614359565b5b905f5260205f200154905080875f01848154811061359957613598614359565b5b905f5260205f20018190555083876001015f8381526020019081526020015f2081905550505b855f018054806135d2576135d1614d07565b5b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f90556001935050505061360c565b5f9150505b92915050565b5f5f836001015f8481526020019081526020015f20541415905092915050565b505f81556001015f9055565b5080545f8255905f5260205f20906136569190613659565b50565b5f5b80821115613679578281015f613671919061367e565b60010161365b565b505090565b50805461368a90614078565b5f825580601f1061369b57506136b5565b601f0160209004905f5260205f20906136b491906136b8565b5b50565b5f5b808211156136d0578281015f90556001016136ba565b505090565b5f604051905090565b5f5ffd5b5f5ffd5b5f7fffffffff0000000000000000000000000000000000000000000000000000000082169050919050565b61371a816136e6565b8114613724575f5ffd5b50565b5f8135905061373581613711565b92915050565b5f602082840312156137505761374f6136de565b5b5f61375d84828501613727565b91505092915050565b5f8115159050919050565b61377a81613766565b82525050565b5f6020820190506137935f830184613771565b92915050565b5f819050919050565b6137ab81613799565b81146137b5575f5ffd5b50565b5f813590506137c6816137a2565b92915050565b5f5f604083850312156137e2576137e16136de565b5b5f6137ef858286016137b8565b9250506020613800858286016137b8565b9150509250929050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f84011261382b5761382a61380a565b5b8235905067ffffffffffffffff8111156138485761384761380e565b5b60208301915083600182028301111561386457613863613812565b5b9250929050565b5f5f60208385031215613881576138806136de565b5b5f83013567ffffffffffffffff81111561389e5761389d6136e2565b5b6138aa85828601613816565b92509250509250929050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6138df826138b6565b9050919050565b6138ef816138d5565b81146138f9575f5ffd5b50565b5f8135905061390a816138e6565b92915050565b5f60208284031215613925576139246136de565b5b5f613932848285016138fc565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b600781106139795761397861393b565b5b50565b5f81905061398982613968565b919050565b5f6139988261397c565b9050919050565b6139a88161398e565b82525050565b5f6020820190506139c15f83018461399f565b92915050565b6139d081613799565b82525050565b5f6020820190506139e95f8301846139c7565b92915050565b5f5f5f60408486031215613a0657613a056136de565b5b5f613a13868287016138fc565b935050602084013567ffffffffffffffff811115613a3457613a336136e2565b5b613a4086828701613816565b92509250509250925092565b5f819050919050565b613a5e81613a4c565b8114613a68575f5ffd5b50565b5f81359050613a7981613a55565b92915050565b5f60208284031215613a9457613a936136de565b5b5f613aa184828501613a6b565b91505092915050565b613ab381613a4c565b82525050565b5f602082019050613acc5f830184613aaa565b92915050565b5f5f60408385031215613ae857613ae76136de565b5b5f613af585828601613a6b565b9250506020613b06858286016138fc565b9150509250929050565b613b19816138d5565b82525050565b5f602082019050613b325f830184613b10565b92915050565b5f5f60408385031215613b4e57613b4d6136de565b5b5f613b5b85828601613a6b565b9250506020613b6c858286016137b8565b9150509250929050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b613ba8816138d5565b82525050565b5f613bb98383613b9f565b60208301905092915050565b5f602082019050919050565b5f613bdb82613b76565b613be58185613b80565b9350613bf083613b90565b805f5b83811015613c20578151613c078882613bae565b9750613c1283613bc5565b925050600181019050613bf3565b5085935050505092915050565b5f6020820190508181035f830152613c458184613bd1565b905092915050565b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b613c9382613c4d565b810181811067ffffffffffffffff82111715613cb257613cb1613c5d565b5b80604052505050565b5f613cc46136d5565b9050613cd08282613c8a565b919050565b5f67ffffffffffffffff821115613cef57613cee613c5d565b5b602082029050602081019050919050565b5f613d12613d0d84613cd5565b613cbb565b90508083825260208201905060208402830185811115613d3557613d34613812565b5b835b81811015613d5e5780613d4a88826138fc565b845260208401935050602081019050613d37565b5050509392505050565b5f82601f830112613d7c57613d7b61380a565b5b8135613d8c848260208601613d00565b91505092915050565b5f5f60408385031215613dab57613daa6136de565b5b5f613db8858286016137b8565b925050602083013567ffffffffffffffff811115613dd957613dd86136e2565b5b613de585828601613d68565b9150509250929050565b5f60208284031215613e0457613e036136de565b5b5f613e11848285016137b8565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b613e4c81613799565b82525050565b5f613e5d8383613e43565b60208301905092915050565b5f602082019050919050565b5f613e7f82613e1a565b613e898185613e24565b9350613e9483613e34565b805f5b83811015613ec4578151613eab8882613e52565b9750613eb683613e69565b925050600181019050613e97565b5085935050505092915050565b5f6020820190508181035f830152613ee98184613e75565b905092915050565b5f5f5f5f60808587031215613f0957613f086136de565b5b5f613f1687828801613a6b565b9450506020613f27878288016137b8565b935050604085013567ffffffffffffffff811115613f4857613f476136e2565b5b613f5487828801613d68565b9250506060613f65878288016137b8565b91505092959194509250565b5f604082019050613f845f830185613b10565b613f9160208301846139c7565b9392505050565b5f606082019050613fab5f830186613b10565b613fb860208301856139c7565b613fc560408301846139c7565b949350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61400482613799565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff820361403657614035613fcd565b5b600182019050919050565b5f82905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061408f57607f821691505b6020821081036140a2576140a161404b565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026141047fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff826140c9565b61410e86836140c9565b95508019841693508086168417925050509392505050565b5f819050919050565b5f61414961414461413f84613799565b614126565b613799565b9050919050565b5f819050919050565b6141628361412f565b61417661416e82614150565b8484546140d5565b825550505050565b5f5f905090565b61418d61417e565b614198818484614159565b505050565b5f5b828110156141be576141b35f828401614185565b60018101905061419f565b505050565b601f8211156142115782821115614210576141dd816140a8565b6141e6836140ba565b6141ef856140ba565b60208610156141fc575f90505b80830161420b8284038261419d565b505050505b5b505050565b5f82821c905092915050565b5f6142315f1984600802614216565b1980831691505092915050565b5f6142498383614222565b9150826002028217905092915050565b6142638383614041565b67ffffffffffffffff81111561427c5761427b613c5d565b5b6142868254614078565b6142918282856141c3565b5f601f8311600181146142be575f84156142ac578287013590505b6142b6858261423e565b86555061431d565b601f1984166142cc866140a8565b5f5b828110156142f3578489013582556001820191506020850194506020810190506142ce565b86831015614310578489013561430c601f891682614222565b8355505b6001600288020188555050505b50505050505050565b5f61433082613799565b915061433b83613799565b925082820190508082111561435357614352613fcd565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f82825260208201905092915050565b7f4255473a207468652061757468656e7469636174696f6e20766f7465732062795f8201527f20686f6e65737420636c69656e74732061726520696e636f6e73697374656e74602082015250565b5f6143f0604083614386565b91506143fb82614396565b604082019050919050565b5f6020820190508181035f83015261441d816143e4565b9050919050565b5f61442e82613799565b915061443983613799565b925082820390508181111561445157614450613fcd565b5b92915050565b5f61446182613799565b915061446c83613799565b925082820261447a81613799565b9150828204841483151761449157614490613fcd565b5b5092915050565b5f6040820190506144ab5f8301856139c7565b6144b860208301846139c7565b9392505050565b5f6040820190506144d25f830185613b10565b6144df6020830184613b10565b9392505050565b7f4255473a20414c5245414459205245434549564544205348415245532046524f5f8201527f4d204e20504152544945532c20544f4f204d414e5920434c49454e5453000000602082015250565b5f614540603d83614386565b915061454b826144e6565b604082019050919050565b5f6020820190508181035f83015261456d81614534565b9050919050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f6145cf8261459d565b6145d981856145a7565b93506145e98185602086016145b7565b6145f281613c4d565b840191505092915050565b5f61460883836145c5565b905092915050565b5f602082019050919050565b5f61462682614574565b614630818561457e565b9350836020820285016146428561458e565b805f5b8581101561467d578484038952815161465e85826145fd565b945061466983614610565b925060208a01995050600181019050614645565b50829750879550505050505092915050565b5f6020820190508181035f8301526146a7818461461c565b905092915050565b7f43555252454e544c59204f4e4c59204f4e4520494e4445582050455220434c495f8201527f454e5420414c4c4f574544000000000000000000000000000000000000000000602082015250565b5f614709602b83614386565b9150614714826146af565b604082019050919050565b5f6020820190508181035f830152614736816146fd565b9050919050565b5f81549050919050565b5f8190506001806001038301049050919050565b5f819050815f5260205f209050919050565b61479d7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff83602003600802614216565b815481168255505050565b6147b1816140a8565b6147bc83825461423e565b8083555f825550505050565b602084105f811461482357601f8411600181146147f0576147e9868561423e565b835561481d565b6147f9836140a8565b6148116001614807886140ba565b036001830161419d565b61481b87856147a8565b505b5061487d565b61482c856140ba565b614835856140ba565b61483e846140a8565b828101601f8916801561485957614858816001840361476d565b5b8484111561486e5761486d8585038361419d565b5b60018a60020217875550505050505b5050505050565b6801000000000000000084111561489e5761489d613c5d565b5b602083105f81146148e757602085105f81146148c5576148be868561423e565b83556148e1565b8360ff19169350836148d6846140a8565b556001866002020183555b506148f1565b6001856002020182555b5050505050565b805461490381614078565b808411156149185761491784828486614884565b5b8084101561492d5761492c848284866147c8565b5b50505050565b82811015614952576149475f828401614185565b600181019050614933565b505050565b6149615f826148f8565b50565b7f4e487b71000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b5f82146149a05761499f614964565b5b6149a981614957565b5050565b5f5b828110156149ce576149c35f828401614990565b6001810190506149af565b505050565b81831015614a0a576149e482614747565b6149ed84614747565b6149f68361475b565b818101614a05838503826149ad565b505050505b505050565b68010000000000000000821115614a2957614a28613c5d565b5b614a328161473d565b828255614a408382846149d3565b505050565b5f81519050919050565b614a588261459d565b67ffffffffffffffff811115614a7157614a70613c5d565b5b614a7b8254614078565b614a868282856141c3565b5f60209050601f831160018114614ab7575f8415614aa5578287015190505b614aaf858261423e565b865550614b16565b601f198416614ac5866140a8565b5f5b82811015614aec57848901518255600182019150602085019450602081019050614ac7565b86831015614b095784890151614b05601f891682614222565b8355505b6001600288020188555050505b505050505050565b614b288282614a4f565b5050565b614b3582614574565b614b3f8183614a0f565b614b488361458e565b614b518361475b565b5f5b83811015614b8657614b6483614a45565b614b6e8184614b1e565b60208401935060018301925050600181019050614b53565b505050505050565b5f604082019050614ba15f830185613b10565b8181036020830152614bb38184613e75565b90509392505050565b5f604082019050614bcf5f83018561399f565b614bdc602083018461399f565b9392505050565b5f604082019050614bf65f8301856139c7565b614c036020830184613b10565b9392505050565b5f606082019050614c1d5f8301866139c7565b614c2a60208301856139c7565b614c376040830184613b10565b949350505050565b5f608082019050614c525f830187613b10565b614c5f60208301866139c7565b614c6c60408301856139c7565b614c796060830184613b10565b95945050505050565b5f604082019050614c955f830185613b10565b614ca26020830184613aaa565b9392505050565b5f60ff82169050919050565b614cbe81614ca9565b82525050565b5f608082019050614cd75f830187613aaa565b614ce46020830186614cb5565b614cf16040830185613aaa565b614cfe6060830184613aaa565b95945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffdfea26469706673582212203325336b06a7954ec3c2000917ebebb9f16d3d09cc2fa3648582ca1c24b171df64736f6c63430008210033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`@Qac$8\x03\x80ac$\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\x12xV[\x83\x83\x83\x833\x81\x84\x85\x85a\0J\x82\x82a\x01\x1C` \x1B` \x1CV[PP_`\n\x81\x90UPa\0c\x82\x82a\x04\xDE` \x1B` \x1CV[PP_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\0\xD5W_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\0\xCC\x91\x90a\x13\x07V[`@Q\x80\x91\x03\x90\xFD[a\0\xE4\x81a\x08r` \x1B` \x1CV[Pa\x01\x0F\x84\x83_\x81Q\x81\x10a\0\xFCWa\0\xFBa\x13 V[[` \x02` \x01\x01Qa\t5` \x1B` \x1CV[PPPPPPPPa\x15\xADV[_`\x01\x83`\x03a\x01,\x91\x90a\x13zV[a\x016\x91\x90a\x13\xBBV[\x90P\x80\x82Q\x10\x15a\x01\x81W\x81Q\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x01x\x92\x91\x90a\x13\xFDV[`@Q\x80\x91\x03\x90\xFD[\x82`\x02\x81\x90UP_a\x01\xB8\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\t\xB8` \x1B` \x1CV[\x90P_a\x01\xEA\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\t\xE0` \x1B` \x1CV[\x90P__\x90P[\x81\x81\x10\x15a\x02QWa\x02C\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x84\x83\x81Q\x81\x10a\x020Wa\x02/a\x13 V[[` \x02` \x01\x01Qa\n\x07` \x1B` \x1CV[P\x80\x80`\x01\x01\x91PPa\x01\xF1V[P_a\x02\x82\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\t\xB8` \x1B` \x1CV[\x90P_a\x02\xB4\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\t\xE0` \x1B` \x1CV[\x90P__\x90P[\x81\x81\x10\x15a\x03\x1BWa\x03\r\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x84\x83\x81Q\x81\x10a\x02\xFAWa\x02\xF9a\x13 V[[` \x02` \x01\x01Qa\n\x07` \x1B` \x1CV[P\x80\x80`\x01\x01\x91PPa\x02\xBBV[P_a\x03L\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2a\t\xB8` \x1B` \x1CV[\x90P_a\x03~\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2a\t\xE0` \x1B` \x1CV[\x90P__\x90P[\x81\x81\x10\x15a\x03\xE5Wa\x03\xD7\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2\x84\x83\x81Q\x81\x10a\x03\xC4Wa\x03\xC3a\x13 V[[` \x02` \x01\x01Qa\n\x07` \x1B` \x1CV[P\x80\x80`\x01\x01\x91PPa\x03\x85V[P__\x90P[\x88Q\x81\x10\x15a\x04LWa\x04>\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x8A\x83\x81Q\x81\x10a\x04+Wa\x04*a\x13 V[[` \x02` \x01\x01Qa\n ` \x1B` \x1CV[P\x80\x80`\x01\x01\x91PPa\x03\xEBV[Pa\x04\x97\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x89_\x81Q\x81\x10a\x04\x84Wa\x04\x83a\x13 V[[` \x02` \x01\x01Qa\n ` \x1B` \x1CV[P\x7F\xA2\xDFx0\xE0\xBE\xDE\xF7\xB1\x11k\xF5G\xB4g\xB1kP\xB3\xBD#\x14l\x9E\t\x98x\xD1N\x890\x1A\x87\x8A3`@Qa\x04\xCB\x93\x92\x91\x90a\x14$V[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPV[`\x07T`\n_\x82\x82Ta\x04\xF1\x91\x90a\x13\xBBV[\x92PP\x81\x90UP\x81`\x07\x81\x90UP_`\x08\x81\x90UP_`\t\x81\x90UP\x80`\x0B\x81\x90UP_a\x05D\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\t\xB8` \x1B` \x1CV[\x90P_a\x05v\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\t\xE0` \x1B` \x1CV[\x90P__\x90P[`\x07T\x81\x10\x15a\x080W`\x0C_`\x05_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ __\x82\x01_\x90U`\x01\x82\x01_\x90UPP`\x06_`\x05_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x06|\x91\x90a\x0F\xB4V[__\x90P[\x82\x81\x10\x15a\x07jW`\x03_`\x05_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x02\x01_\x85\x83\x81Q\x81\x10a\x07\rWa\x07\x0Ca\x13 V[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U\x80\x80`\x01\x01\x91PPa\x06\x81V[P`\x03_`\x05_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ __\x82\x01_a\x07\xE7\x91\x90a\x0F\xC0V[`\x01\x82\x01_\x90UPP`\x05_\x82\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U\x80\x80`\x01\x01\x91PPa\x05}V[P\x7F\xF7\xF0\x87#\x82\xDF\xF5\xE6\x98\xB2\x84\xE1 \x84\xE4\xE7\x89O\x83\x02\x16\xDD\x80\xCBN\x90\x9BY:X\xF95`\x07T3`@Qa\x08d\x92\x91\x90a\x14YV[`@Q\x80\x91\x03\x90\xA1PPPPV[_`\r_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\r_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[\x81`\x0E\x81\x90UPB`\x0F\x81\x90UPC`\x10\x81\x90UP_`\x11_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x06\x81\x11\x15a\tnWa\tma\x14\x80V[[\x02\x17\x90UP\x7F\xDE\xF1\xF0\x8E\xB6U\xF4\xA7_`\xBDo\xD7\xE9q\x11 \x04\xAB\xD8F\xA6\x12\xE4m\xAB'\x07p\xD2L\xA50`\x0FT`\x10T\x84`@Qa\t\xAC\x94\x93\x92\x91\x90a\x14\xADV[`@Q\x80\x91\x03\x90\xA1PPV[``a\t\xD9`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a\n9` \x1B` \x1CV[\x90P\x91\x90PV[_a\n\0`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a\n^` \x1B` \x1CV[\x90P\x91\x90PV[_a\n\x18\x83\x83a\nw` \x1B` \x1CV[\x90P\x92\x91PPV[_a\n1\x83\x83a\n\xC0` \x1B` \x1CV[\x90P\x92\x91PPV[``_a\nM\x83_\x01a\x0B\t` \x1B` \x1CV[\x90P``\x81\x90P\x80\x92PPP\x91\x90PV[_a\np\x82_\x01a\x0Bb` \x1B` \x1CV[\x90P\x91\x90PV[__a\n\x89\x84\x84a\x0Bq` \x1B` \x1CV[\x90P\x80\x15a\n\xB6Wa\n\xB4\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a\x0Cf` \x1B\x90\x91\x90` \x1CV[P[\x80\x91PP\x92\x91PPV[__a\n\xD2\x84\x84a\x0C\x99` \x1B` \x1CV[\x90P\x80\x15a\n\xFFWa\n\xFD\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a\r\x8E` \x1B\x90\x91\x90` \x1CV[P[\x80\x91PP\x92\x91PPV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0BVW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0BBW[PPPPP\x90P\x91\x90PV[_\x81_\x01\x80T\x90P\x90P\x91\x90PV[_a\x0B\x82\x83\x83a\r\xC1` \x1B` \x1CV[\x15a\x0C\\W___\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x0B\xF9a\x0E$` \x1B` \x1CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\x0C`V[_\x90P[\x92\x91PPV[_a\x0C\x91\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba\x0E+` \x1B` \x1CV[\x90P\x92\x91PPV[_a\x0C\xAA\x83\x83a\r\xC1` \x1B` \x1CV[a\r\x84W`\x01__\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\r!a\x0E$` \x1B` \x1CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\r\x88V[_\x90P[\x92\x91PPV[_a\r\xB9\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba\x0F'` \x1B` \x1CV[\x90P\x92\x91PPV[___\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[_3\x90P\x90V[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x14a\x0F\x1CW_`\x01\x82a\x0EX\x91\x90a\x14\xF0V[\x90P_`\x01\x86_\x01\x80T\x90Pa\x0En\x91\x90a\x14\xF0V[\x90P\x80\x82\x14a\x0E\xD4W_\x86_\x01\x82\x81T\x81\x10a\x0E\x8DWa\x0E\x8Ca\x13 V[[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a\x0E\xAEWa\x0E\xADa\x13 V[[\x90_R` _ \x01\x81\x90UP\x83\x87`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UPP[\x85_\x01\x80T\x80a\x0E\xE7Wa\x0E\xE6a\x15#V[[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\x0F!V[_\x91PP[\x92\x91PPV[_a\x0F8\x83\x83a\x0F\x94` \x1B` \x1CV[a\x0F\x8AW\x82_\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x82_\x01\x80T\x90P\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP`\x01\x90Pa\x0F\x8EV[_\x90P[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x14\x15\x90P\x92\x91PPV[P_\x81U`\x01\x01_\x90UV[P\x80T_\x82U\x90_R` _ \x90a\x0F\xD8\x91\x90a\x0F\xDBV[PV[_[\x80\x82\x11\x15a\x0F\xFBW\x82\x81\x01_a\x0F\xF3\x91\x90a\x10\0V[`\x01\x01a\x0F\xDDV[PP\x90V[P\x80Ta\x10\x0C\x90a\x15}V[_\x82U\x80`\x1F\x10a\x10\x1DWPa\x107V[`\x1F\x01` \x90\x04\x90_R` _ \x90a\x106\x91\x90a\x10:V[[PV[_[\x80\x82\x11\x15a\x10RW\x82\x81\x01_\x90U`\x01\x01a\x10<V[PP\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[_\x81\x90P\x91\x90PV[a\x10z\x81a\x10hV[\x81\x14a\x10\x84W__\xFD[PV[_\x81Q\x90Pa\x10\x95\x81a\x10qV[\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x10\xAD\x81a\x10\x9BV[\x81\x14a\x10\xB7W__\xFD[PV[_\x81Q\x90Pa\x10\xC8\x81a\x10\xA4V[\x92\x91PPV[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x11\x18\x82a\x10\xD2V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x117Wa\x116a\x10\xE2V[[\x80`@RPPPV[_a\x11Ia\x10WV[\x90Pa\x11U\x82\x82a\x11\x0FV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x11tWa\x11sa\x10\xE2V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x11\xB2\x82a\x11\x89V[\x90P\x91\x90PV[a\x11\xC2\x81a\x11\xA8V[\x81\x14a\x11\xCCW__\xFD[PV[_\x81Q\x90Pa\x11\xDD\x81a\x11\xB9V[\x92\x91PPV[_a\x11\xF5a\x11\xF0\x84a\x11ZV[a\x11@V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x12\x18Wa\x12\x17a\x11\x85V[[\x83[\x81\x81\x10\x15a\x12AW\x80a\x12-\x88\x82a\x11\xCFV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x12\x1AV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x12_Wa\x12^a\x10\xCEV[[\x81Qa\x12o\x84\x82` \x86\x01a\x11\xE3V[\x91PP\x92\x91PPV[____`\x80\x85\x87\x03\x12\x15a\x12\x90Wa\x12\x8Fa\x10`V[[_a\x12\x9D\x87\x82\x88\x01a\x10\x87V[\x94PP` a\x12\xAE\x87\x82\x88\x01a\x10\xBAV[\x93PP`@\x85\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xCFWa\x12\xCEa\x10dV[[a\x12\xDB\x87\x82\x88\x01a\x12KV[\x92PP``a\x12\xEC\x87\x82\x88\x01a\x10\xBAV[\x91PP\x92\x95\x91\x94P\x92PV[a\x13\x01\x81a\x11\xA8V[\x82RPPV[_` \x82\x01\x90Pa\x13\x1A_\x83\x01\x84a\x12\xF8V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x13\x84\x82a\x10\x9BV[\x91Pa\x13\x8F\x83a\x10\x9BV[\x92P\x82\x82\x02a\x13\x9D\x81a\x10\x9BV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x13\xB4Wa\x13\xB3a\x13MV[[P\x92\x91PPV[_a\x13\xC5\x82a\x10\x9BV[\x91Pa\x13\xD0\x83a\x10\x9BV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x13\xE8Wa\x13\xE7a\x13MV[[\x92\x91PPV[a\x13\xF7\x81a\x10\x9BV[\x82RPPV[_`@\x82\x01\x90Pa\x14\x10_\x83\x01\x85a\x13\xEEV[a\x14\x1D` \x83\x01\x84a\x13\xEEV[\x93\x92PPPV[_``\x82\x01\x90Pa\x147_\x83\x01\x86a\x13\xEEV[a\x14D` \x83\x01\x85a\x13\xEEV[a\x14Q`@\x83\x01\x84a\x12\xF8V[\x94\x93PPPPV[_`@\x82\x01\x90Pa\x14l_\x83\x01\x85a\x13\xEEV[a\x14y` \x83\x01\x84a\x12\xF8V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_`\x80\x82\x01\x90Pa\x14\xC0_\x83\x01\x87a\x12\xF8V[a\x14\xCD` \x83\x01\x86a\x13\xEEV[a\x14\xDA`@\x83\x01\x85a\x13\xEEV[a\x14\xE7``\x83\x01\x84a\x12\xF8V[\x95\x94PPPPPV[_a\x14\xFA\x82a\x10\x9BV[\x91Pa\x15\x05\x83a\x10\x9BV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x15\x1DWa\x15\x1Ca\x13MV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a\x15\x94W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x15\xA7Wa\x15\xA6a\x15PV[[P\x91\x90PV[aMj\x80a\x15\xBA_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02%W_5`\xE0\x1C\x80c\x7F5\xB5`\x11a\x01.W\x80c\xCA\x15\xC8s\x11a\0\xB6W\x80c\xEB\x85u\xDE\x11a\0zW\x80c\xEB\x85u\xDE\x14a\x05\xCBW\x80c\xEB\xAE5\xE7\x14a\x05\xE7W\x80c\xF2\xFD\xE3\x8B\x14a\x06\x17W\x80c\xF6`<a\x14a\x063W\x80c\xFCx\xB2\xE8\x14a\x06OWa\x02%V[\x80c\xCA\x15\xC8s\x14a\x059W\x80c\xCB\x9CL\xC4\x14a\x05iW\x80c\xD5Gt\x1F\x14a\x05sW\x80c\xD8'\r\xCE\x14a\x05\x8FW\x80c\xEA\xE6\xF6R\x14a\x05\xADWa\x02%V[\x80c\xA2\x17\xFD\xDF\x11a\0\xFDW\x80c\xA2\x17\xFD\xDF\x14a\x04\xBBW\x80c\xA3$j\xD3\x14a\x04\xD9W\x80c\xAF o(\x14a\x05\tW\x80c\xBBQ\xFE\xF0\x14a\x05%W\x80c\xC0y\xF4\x95\x14a\x05/Wa\x02%V[\x80c\x7F5\xB5`\x14a\x04\x1FW\x80c\x8D\xA5\xCB[\x14a\x04=W\x80c\x90\x10\xD0|\x14a\x04[W\x80c\x91\xD1HT\x14a\x04\x8BWa\x02%V[\x80c$\x8A\x9C\xA3\x11a\x01\xB1W\x80c6V\x8A\xBE\x11a\x01\x80W\x80c6V\x8A\xBE\x14a\x03\xC9W\x80c;C8\xD1\x14a\x03\xE5W\x80cK\x8Ed\x88\x14a\x04\x01W\x80cK\xB2x\xF3\x14a\x04\x0BW\x80cqP\x18\xA6\x14a\x04\x15Wa\x02%V[\x80c$\x8A\x9C\xA3\x14a\x03UW\x80c//\xF1]\x14a\x03\x85W\x80c0\x10L>\x14a\x03\xA1W\x80c3\xCC\x9A\t\x14a\x03\xBFWa\x02%V[\x80c\x14l\xA51\x11a\x01\xF8W\x80c\x14l\xA51\x14a\x02\xC1W\x80c\x17cE\x14\x14a\x02\xDFW\x80c\x1CtS\xDB\x14a\x02\xFDW\x80c\x1E\xE4\xEE\x0F\x14a\x03\x1BW\x80c#(\xBD\x12\x14a\x037Wa\x02%V[\x80c\x01\xFF\xC9\xA7\x14a\x02)W\x80c\x0B\xDA\x81\xCF\x14a\x02YW\x80c\rB\xEBo\x14a\x02uW\x80c\x13\xFFm\xD5\x14a\x02\x91W[__\xFD[a\x02C`\x04\x806\x03\x81\x01\x90a\x02>\x91\x90a7;V[a\x06\x7FV[`@Qa\x02P\x91\x90a7\x80V[`@Q\x80\x91\x03\x90\xF3[a\x02s`\x04\x806\x03\x81\x01\x90a\x02n\x91\x90a7\xCCV[a\x06\xF8V[\0[a\x02\x8F`\x04\x806\x03\x81\x01\x90a\x02\x8A\x91\x90a8kV[a\tJV[\0[a\x02\xAB`\x04\x806\x03\x81\x01\x90a\x02\xA6\x91\x90a9\x10V[a\t\x8BV[`@Qa\x02\xB8\x91\x90a7\x80V[`@Q\x80\x91\x03\x90\xF3[a\x02\xC9a\n\x07V[`@Qa\x02\xD6\x91\x90a9\xAEV[`@Q\x80\x91\x03\x90\xF3[a\x02\xE7a\n\x19V[`@Qa\x02\xF4\x91\x90a9\xD6V[`@Q\x80\x91\x03\x90\xF3[a\x03\x05a\n\x1FV[`@Qa\x03\x12\x91\x90a9\xD6V[`@Q\x80\x91\x03\x90\xF3[a\x035`\x04\x806\x03\x81\x01\x90a\x030\x91\x90a9\xEFV[a\n%V[\0[a\x03?a\x0F\xD3V[`@Qa\x03L\x91\x90a9\xD6V[`@Q\x80\x91\x03\x90\xF3[a\x03o`\x04\x806\x03\x81\x01\x90a\x03j\x91\x90a:\x7FV[a\x0F\xE9V[`@Qa\x03|\x91\x90a:\xB9V[`@Q\x80\x91\x03\x90\xF3[a\x03\x9F`\x04\x806\x03\x81\x01\x90a\x03\x9A\x91\x90a:\xD2V[a\x10\x05V[\0[a\x03\xA9a\x10?V[`@Qa\x03\xB6\x91\x90a:\xB9V[`@Q\x80\x91\x03\x90\xF3[a\x03\xC7a\x10cV[\0[a\x03\xE3`\x04\x806\x03\x81\x01\x90a\x03\xDE\x91\x90a:\xD2V[a\x10\xDDV[\0[a\x03\xFF`\x04\x806\x03\x81\x01\x90a\x03\xFA\x91\x90a7\xCCV[a\x11\xA4V[\0[a\x04\ta\x11\xDDV[\0[a\x04\x13a\x12WV[\0[a\x04\x1Da\x12\xD1V[\0[a\x04'a\x12\xE4V[`@Qa\x044\x91\x90a:\xB9V[`@Q\x80\x91\x03\x90\xF3[a\x04Ea\x13\x08V[`@Qa\x04R\x91\x90a;\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x04u`\x04\x806\x03\x81\x01\x90a\x04p\x91\x90a;8V[a\x130V[`@Qa\x04\x82\x91\x90a;\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x04\xA5`\x04\x806\x03\x81\x01\x90a\x04\xA0\x91\x90a:\xD2V[a\x13\\V[`@Qa\x04\xB2\x91\x90a7\x80V[`@Q\x80\x91\x03\x90\xF3[a\x04\xC3a\x13\xBFV[`@Qa\x04\xD0\x91\x90a:\xB9V[`@Q\x80\x91\x03\x90\xF3[a\x04\xF3`\x04\x806\x03\x81\x01\x90a\x04\xEE\x91\x90a:\x7FV[a\x13\xC5V[`@Qa\x05\0\x91\x90a<-V[`@Q\x80\x91\x03\x90\xF3[a\x05#`\x04\x806\x03\x81\x01\x90a\x05\x1E\x91\x90a=\x95V[a\x13\xE7V[\0[a\x05-a\x14 V[\0[a\x057a\x14\x9AV[\0[a\x05S`\x04\x806\x03\x81\x01\x90a\x05N\x91\x90a:\x7FV[a\x15\x14V[`@Qa\x05`\x91\x90a9\xD6V[`@Q\x80\x91\x03\x90\xF3[a\x05qa\x155V[\0[a\x05\x8D`\x04\x806\x03\x81\x01\x90a\x05\x88\x91\x90a:\xD2V[a\x15\xAEV[\0[a\x05\x97a\x16\xA1V[`@Qa\x05\xA4\x91\x90a9\xD6V[`@Q\x80\x91\x03\x90\xF3[a\x05\xB5a\x16\xA7V[`@Qa\x05\xC2\x91\x90a:\xB9V[`@Q\x80\x91\x03\x90\xF3[a\x05\xE5`\x04\x806\x03\x81\x01\x90a\x05\xE0\x91\x90a9\xEFV[a\x16\xCBV[\0[a\x06\x01`\x04\x806\x03\x81\x01\x90a\x05\xFC\x91\x90a=\xEFV[a\x1C\x10V[`@Qa\x06\x0E\x91\x90a>\xD1V[`@Q\x80\x91\x03\x90\xF3[a\x061`\x04\x806\x03\x81\x01\x90a\x06,\x91\x90a9\x10V[a \xDCV[\0[a\x06M`\x04\x806\x03\x81\x01\x90a\x06H\x91\x90a>\xF1V[a!`V[\0[a\x06i`\x04\x806\x03\x81\x01\x90a\x06d\x91\x90a9\x10V[a!\xC9V[`@Qa\x06v\x91\x90a7\x80V[`@Q\x80\x91\x03\x90\xF3[_\x7F\x07\xEF\xFE\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x06\xF1WPa\x06\xF0\x82a!\xFBV[[\x90P\x91\x90PV[\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2a\x07\"\x81a\"tV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x05_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07\xC4W3\x82`@Q\x7F\xFF\xAB\xBA\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\xBB\x92\x91\x90a?qV[`@Q\x80\x91\x03\x90\xFD[_\x83\x03a\x08\x08W3`@Q\x7F\x16\x92<\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\xFF\x91\x90a;\x1FV[`@Q\x80\x91\x03\x90\xFD[_`\x0C_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x14a\x08\x8CW3`@Q\x7FO_\xBF\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\x83\x91\x90a;\x1FV[`@Q\x80\x91\x03\x90\xFD[`@Q\x80`@\x01`@R\x80\x83\x81R` \x01\x84\x81RP`\x0C_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01U` \x82\x01Q\x81`\x01\x01U\x90PP\x7F\xB8\x9A\xDD\xD97\xF4O\x90,\x84\x95\x96d\x187\xCDz\xF2\xFC\xEC\xEF\"\xD2\xA7\x86o\xDC\x1A\xD9\xC0\xAE.3\x84\x84`@Qa\t&\x93\x92\x91\x90a?\x98V[`@Q\x80\x91\x03\x90\xA1`\t_\x81T\x80\x92\x91\x90a\t@\x90a?\xFAV[\x91\x90PUPPPPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\tt\x81a\"tV[\x82\x82`\x04\x91\x82a\t\x85\x92\x91\x90aBYV[PPPPV[_a\t\x95\x82a!\xC9V[a\t\xD6W\x81`@Q\x7F\xAB\xDC\xE0j\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xCD\x91\x90a;\x1FV[`@Q\x80\x91\x03\x90\xFD[a\n\0\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x83a\x13\\V[\x90P\x91\x90PV[`\x11_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\x10T\x81V[`\nT\x81V[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\nO\x81a\"tV[a\ny\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2\x85a\x13\\V[a\n\xBAW\x83`@Q\x7F\xA02\xACk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\xB1\x91\x90a;\x1FV[`@Q\x80\x91\x03\x90\xFD[_`\x07T\x90P__\x90P[`\x07T\x81\x10\x15a\x0BGW\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x05_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0B:W\x80\x91Pa\x0BGV[\x80\x80`\x01\x01\x91PPa\n\xC5V[P`\x07T\x81\x03a\x0B\xBEW`\x05__\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x7Fo\xAF\x9F\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\xB5\x91\x90a;\x1FV[`@Q\x80\x91\x03\x90\xFD[_\x81`\nTa\x0B\xCD\x91\x90aC&V[\x90P_a\x0B\xFF\x82`@Q` \x01a\x0B\xE4\x91\x90a9\xD6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\"\x88V[\x90P_a\x0CO\x82\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa\"\xBBV[\x90P\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0C\xEDW`\x06_\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01`\x02\x81\x10a\x0C\xD4Wa\x0C\xD3aCYV[[\x01_\x81Ta\x0C\xE1\x90a?\xFAV[\x91\x90P\x81\x90UPa\rQV[`\x06_\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _`\x02\x81\x10a\r<Wa\r;aCYV[[\x01_\x81Ta\rI\x90a?\xFAV[\x91\x90P\x81\x90UP[`\x01`\x0BTa\r`\x91\x90aC&V[`\x06_\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _`\x02\x81\x10a\r\xAFWa\r\xAEaCYV[[\x01T\x10\x80a\x0E\x1BWP`\x01`\x0BTa\r\xC7\x91\x90aC&V[`\x06_\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01`\x02\x81\x10a\x0E\x17Wa\x0E\x16aCYV[[\x01T\x10[a\x0EZW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0EQ\x90aD\x06V[`@Q\x80\x91\x03\x90\xFD[`\x01`\x0BTa\x0Ei\x91\x90aC&V[`\x06_\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _`\x02\x81\x10a\x0E\xB8Wa\x0E\xB7aCYV[[\x01T\x10a\x0F\x12W\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F@p\t \x05 \xF9\xF1\x05\x84\x81<\x0B\x95D\x1A\xB3 \xF6\xB0\x8D\x97\xEB\xDA\xAF\x1E\x82N\xED\xD9\xD7\xC3_`@Qa\x0F\x05\x91\x90a7\x80V[`@Q\x80\x91\x03\x90\xA2a\x0F\xC9V[`\x01`\x0BTa\x0F!\x91\x90aC&V[`\x06_\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01`\x02\x81\x10a\x0FqWa\x0FpaCYV[[\x01T\x10a\x0F\xC8W\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F@p\t \x05 \xF9\xF1\x05\x84\x81<\x0B\x95D\x1A\xB3 \xF6\xB0\x8D\x97\xEB\xDA\xAF\x1E\x82N\xED\xD9\xD7\xC3`\x01`@Qa\x0F\xBF\x91\x90a7\x80V[`@Q\x80\x91\x03\x90\xA2[[PPPPPPPPV[_`\x08T`\x07Ta\x0F\xE4\x91\x90aD$V[\x90P\x90V[___\x83\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x10/\x81a\"tV[a\x109\x83\x83a\"\xE5V[PPPPV[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x81V[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x10\x8D\x81a\"tV[`\x03a\x10\x98\x81a\"\xF8V[\x7F \xF5^\xD0\xC9/+\xB1\xC8\x82T\x88\xE1\xE3\xC9\x84c\xD0$\xB2\xA4-\xBD$\x83\x8C?u&\x0FC\xE93B`@Qa\x10\xC9\x92\x91\x90a?qV[`@Q\x80\x91\x03\x90\xA1a\x10\xD9a#\x81V[PPV[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x82\x14\x80\x15a\x11\x12WPa\x11\x11\x82\x82a\x13\\V[[\x15a\x11\x96W_a\x11!\x83a\x15\x14V[\x90P_`\x01`\x02T`\x03a\x115\x91\x90aDWV[a\x11?\x91\x90aC&V[\x90P\x80\x82\x03a\x11\x93W`\x01\x81a\x11U\x91\x90aD$V[\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\x8A\x92\x91\x90aD\x98V[`@Q\x80\x91\x03\x90\xFD[PP[a\x11\xA0\x82\x82a#\xEAV[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x11\xCE\x81a\"tV[a\x11\xD8\x83\x83a$eV[PPPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x12\x07\x81a\"tV[`\x04a\x12\x12\x81a\"\xF8V[\x7F0\x1F\x8A7\x01\xF5\xB2`\x19s\x82\xDDs\x01\x07\x85B\x14O\xE8\xFD\xDD\x18\x08=on\t\xE4\x95\x8AY3B`@Qa\x12C\x92\x91\x90a?qV[`@Q\x80\x91\x03\x90\xA1a\x12Sa#\x81V[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x12\x81\x81a\"tV[`\x05a\x12\x8C\x81a\"\xF8V[\x7F$\xA8se\x1D&\xFBZF,\xB3z\x91\x07\x1C\xDDM\t\xAB\xEE\xBF\xE0\xED\x14C)\xBE\xD1\xCC5\x9D\x033B`@Qa\x12\xBD\x92\x91\x90a?qV[`@Q\x80\x91\x03\x90\xA1a\x12\xCDa#\x81V[PPV[a\x12\xD9a'\xEDV[a\x12\xE2_a(tV[V[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x81V[_`\r_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_a\x13T\x82`\x01_\x86\x81R` \x01\x90\x81R` \x01_ a)7\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[___\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[__\x1B\x81V[``a\x13\xE0`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a)NV[\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x14\x11\x81a\"tV[a\x14\x1B\x83\x83a)mV[PPPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x14J\x81a\"tV[`\x02a\x14U\x81a\"\xF8V[\x7F`\xED\xF9\xBD\xC7\xC4\xEA\0|\xAE\x1A\x9B\xBD\x03\xE4\x1E[\xFC\xCDr1\xA6\xEC8<.\xDDx\0\xF0\xD2\x0C3B`@Qa\x14\x86\x92\x91\x90a?qV[`@Q\x80\x91\x03\x90\xA1a\x14\x96a#\x81V[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x14\xC4\x81a\"tV[`\x01a\x14\xCF\x81a\"\xF8V[\x7Fg\xC4H\x9FgM\x03\xC7\xD1\x9A\x9E6sQ\x88\xDE|e\xE8\xD1\xE9\x9E\xB3\xA2\xFD%\x8Av\x9E\xB1O\xFF3B`@Qa\x15\0\x92\x91\x90a?qV[`@Q\x80\x91\x03\x90\xA1a\x15\x10a#\x81V[PPV[_a\x15.`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a,\xEDV[\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x15_\x81a\"tV[_a\x15i\x81a\"\xF8V[\x7F\xBBp\x9D\xEAtO\x06\xD1\xB2n\x82M\xEE\xC2\xF7\x14\x0CQ\x12f\xEE\x15\xD7\xA2\x17\x83\x8B1\xD8\xB0\x12=3B`@Qa\x15\x9A\x92\x91\x90a?qV[`@Q\x80\x91\x03\x90\xA1a\x15\xAAa#\x81V[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x15\xD8\x81a\"tV[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x83\x14\x80\x15a\x16\rWPa\x16\x0C\x83\x83a\x13\\V[[\x15a\x16\x91W_a\x16\x1C\x84a\x15\x14V[\x90P_`\x01`\x02T`\x03a\x160\x91\x90aDWV[a\x16:\x91\x90aC&V[\x90P\x80\x82\x03a\x16\x8EW`\x01\x81a\x16P\x91\x90aD$V[\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16\x85\x92\x91\x90aD\x98V[`@Q\x80\x91\x03\x90\xFD[PP[a\x16\x9B\x83\x83a-\0V[PPPPV[`\x0FT\x81V[\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2\x81V[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x16\xF5\x81a\"tV[a\x17\x1F\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2\x85a\x13\\V[a\x17`W\x83`@Q\x7F\xA02\xACk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17W\x91\x90a;\x1FV[`@Q\x80\x91\x03\x90\xFD[_`\x03_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x90P`\x03_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x02\x01_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x18pW\x843`@Q\x7F\x08\xE5T\x95\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18g\x92\x91\x90aD\xBFV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x0BT`\x03a\x18\x81\x91\x90aDWV[a\x18\x8B\x91\x90aC&V[\x81\x10a\x18\xCCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\xC3\x90aEVV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x03_\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x02\x01_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x83\x83`\x03_\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x01\x83\x81T\x81\x10a\x19\xB2Wa\x19\xB1aCYV[[\x90_R` _ \x01\x91\x82a\x19\xC7\x92\x91\x90aBYV[P`\x01`\x03_\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x01_\x82\x82Ta\x1A\x18\x91\x90aC&V[\x92PP\x81\x90UP`\x01\x81a\x1A,\x91\x90aC&V[\x90P`\x01`\x02T`\x02a\x1A?\x91\x90aDWV[a\x1AI\x91\x90aC&V[\x81\x10a\x1C\tW_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1AjWa\x1Aia<]V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\x9DW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1A\x88W\x90P[P\x90P__\x90P[\x82\x81\x10\x15a\x1B\xB8W`\x03_\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x01\x81\x81T\x81\x10a\x1A\xFEWa\x1A\xFDaCYV[[\x90_R` _ \x01\x80Ta\x1B\x11\x90a@xV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1B=\x90a@xV[\x80\x15a\x1B\x88W\x80`\x1F\x10a\x1B_Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1B\x88V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1BkW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x82\x82\x81Q\x81\x10a\x1B\xA0Wa\x1B\x9FaCYV[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\x1A\xA5V[P\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F#\xE9\x1D\xBF\xAE\x03u\x8C\xB8\x8D\x7FbR\xB5q\n\xFAS\xA1\x9F\xFE\x0FKOu\xD7\xF2\xDE\x0C^\xAB\xE9\x82`@Qa\x1B\xFF\x91\x90aF\x8FV[`@Q\x80\x91\x03\x90\xA2P[PPPPPV[```\x01\x82\x14a\x1CUW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1CL\x90aG\x1FV[`@Q\x80\x91\x03\x90\xFD[_\x82\x03a\x1C\x99W3`@Q\x7F\xB2\xFDU\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C\x90\x91\x90a;\x1FV[`@Q\x80\x91\x03\x90\xFD[_`\x08T`\x07Ta\x1C\xAA\x91\x90aD$V[\x90P\x80\x83\x11\x15a\x1C\xF3W\x82\x81`@Q\x7F\xDF=u\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C\xEA\x92\x91\x90aD\x98V[`@Q\x80\x91\x03\x90\xFD[__\x90P[`\x07T\x81\x10\x15a\x1D\xAFW3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x05_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x1D\xA2W3`@Q\x7F\xAC\xA9/\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1D\x99\x91\x90a;\x1FV[`@Q\x80\x91\x03\x90\xFD[\x80\x80`\x01\x01\x91PPa\x1C\xF8V[P_`\x01\x84`\x08Ta\x1D\xC1\x91\x90aC&V[a\x1D\xCB\x91\x90aD$V[\x90P_\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\xE8Wa\x1D\xE7a<]V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\x16W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_`\x08T\x90P[\x82`\x08T\x11a\x1E\xC5W3`\x05_`\x08T\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x08T\x82\x82`\x08Ta\x1E\x8C\x91\x90aD$V[\x81Q\x81\x10a\x1E\x9DWa\x1E\x9CaCYV[[` \x02` \x01\x01\x81\x81RPP`\x08_\x81T\x80\x92\x91\x90a\x1E\xBB\x90a?\xFAV[\x91\x90PUPa\x1E V[a\x1E\xEF\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R23a\"\xE5V[P_`\x03_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x90P`\x01`\x0BT`\x03a\x1FA\x91\x90aDWV[a\x1FK\x91\x90aC&V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1FdWa\x1Fca<]V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1F\x97W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1F\x82W\x90P[P\x81_\x01\x90\x81a\x1F\xA7\x91\x90aK,V[P_\x81`\x01\x01\x81\x90UP_a\x1F\xDB\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x13\xC5V[\x90P_a \x07\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x15\x14V[\x90P__\x90P[\x81\x81\x10\x15a \x93W_\x84`\x02\x01_\x85\x84\x81Q\x81\x10a /Wa .aCYV[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80\x80`\x01\x01\x91PPa \x0EV[P\x7F\x1E\x14\xAB\xE5\xD0\xCD\xB9j\xDD\xE7\xB9\xEC\xA9\xB1K\xC0\x8D\xF6#\xB5\x80Z\xFD\xE5\xA3\xF0\xAC\xAD\xC2\xBFO[3\x86`@Qa \xC5\x92\x91\x90aK\x8EV[`@Q\x80\x91\x03\x90\xA1\x84\x97PPPPPPPP\x91\x90PV[a \xE4a'\xEDV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a!TW_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a!K\x91\x90a;\x1FV[`@Q\x80\x91\x03\x90\xFD[a!]\x81a(tV[PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa!\x8A\x81a\"tV[a!\x94\x84\x84a)mV[a!\x9E\x82\x85a$eV[a!\xC2\x85\x84_\x81Q\x81\x10a!\xB5Wa!\xB4aCYV[[` \x02` \x01\x01Qa-\x13V[PPPPPV[_a!\xF4\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x83a\x13\\V[\x90P\x91\x90PV[_\x7FZ\x05\x18\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\"mWPa\"l\x82a-\x96V[[\x90P\x91\x90PV[a\"\x85\x81a\"\x80a.\x0FV[a.\x16V[PV[_\x7F\x19Ethereum Signed Message:\n32\0\0\0\0_R\x81`\x1CR`<_ \x90P\x91\x90PV[____a\"\xC9\x86\x86a.gV[\x92P\x92P\x92Pa\"\xD9\x82\x82a.\xBCV[\x82\x93PPPP\x92\x91PPV[_a\"\xF0\x83\x83a0\x1EV[\x90P\x92\x91PPV[\x80`\x06\x81\x11\x15a#\x0BWa#\na9;V[[`\x11_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x06\x81\x11\x15a#,Wa#+a9;V[[\x14a#~W\x80`\x11_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x7F\xBF\xA2\x17\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a#u\x92\x91\x90aK\xBCV[`@Q\x80\x91\x03\x90\xFD[PV[`\x01`\x11_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x06\x81\x11\x15a#\xA4Wa#\xA3a9;V[[a#\xAE\x91\x90aC&V[`\x06\x81\x11\x15a#\xC0Wa#\xBFa9;V[[`\x11_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x06\x81\x11\x15a#\xE3Wa#\xE2a9;V[[\x02\x17\x90UPV[a#\xF2a.\x0FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a$VW`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a$`\x82\x82a-\0V[PPPV[`\x07T`\n_\x82\x82Ta$x\x91\x90aC&V[\x92PP\x81\x90UP\x81`\x07\x81\x90UP_`\x08\x81\x90UP_`\t\x81\x90UP\x80`\x0B\x81\x90UP_a$\xC5\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x13\xC5V[\x90P_a$\xF1\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x15\x14V[\x90P__\x90P[`\x07T\x81\x10\x15a'\xABW`\x0C_`\x05_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ __\x82\x01_\x90U`\x01\x82\x01_\x90UPP`\x06_`\x05_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a%\xF7\x91\x90a62V[__\x90P[\x82\x81\x10\x15a&\xE5W`\x03_`\x05_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x02\x01_\x85\x83\x81Q\x81\x10a&\x88Wa&\x87aCYV[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U\x80\x80`\x01\x01\x91PPa%\xFCV[P`\x03_`\x05_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ __\x82\x01_a'b\x91\x90a6>V[`\x01\x82\x01_\x90UPP`\x05_\x82\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U\x80\x80`\x01\x01\x91PPa$\xF8V[P\x7F\xF7\xF0\x87#\x82\xDF\xF5\xE6\x98\xB2\x84\xE1 \x84\xE4\xE7\x89O\x83\x02\x16\xDD\x80\xCBN\x90\x9BY:X\xF95`\x07T3`@Qa'\xDF\x92\x91\x90aK\xE3V[`@Q\x80\x91\x03\x90\xA1PPPPV[a'\xF5a.\x0FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a(\x13a\x13\x08V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a(rWa(6a.\x0FV[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a(i\x91\x90a;\x1FV[`@Q\x80\x91\x03\x90\xFD[V[_`\r_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\r_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[_a)D\x83_\x01\x83a0aV[_\x1C\x90P\x92\x91PPV[``_a)\\\x83_\x01a0\x88V[\x90P``\x81\x90P\x80\x92PPP\x91\x90PV[_`\x01\x83`\x03a)}\x91\x90aDWV[a)\x87\x91\x90aC&V[\x90P\x80\x82Q\x10\x15a)\xD2W\x81Q\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a)\xC9\x92\x91\x90aD\x98V[`@Q\x80\x91\x03\x90\xFD[\x82`\x02\x81\x90UP_a*\x03\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x13\xC5V[\x90P_a*/\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x15\x14V[\x90P__\x90P[\x81\x81\x10\x15a*\x90Wa*\x82\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x84\x83\x81Q\x81\x10a*uWa*taCYV[[` \x02` \x01\x01Qa-\0V[P\x80\x80`\x01\x01\x91PPa*6V[P_a*\xBB\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x13\xC5V[\x90P_a*\xE7\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x15\x14V[\x90P__\x90P[\x81\x81\x10\x15a+HWa+:\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x84\x83\x81Q\x81\x10a+-Wa+,aCYV[[` \x02` \x01\x01Qa-\0V[P\x80\x80`\x01\x01\x91PPa*\xEEV[P_a+s\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2a\x13\xC5V[\x90P_a+\x9F\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2a\x15\x14V[\x90P__\x90P[\x81\x81\x10\x15a,\0Wa+\xF2\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2\x84\x83\x81Q\x81\x10a+\xE5Wa+\xE4aCYV[[` \x02` \x01\x01Qa-\0V[P\x80\x80`\x01\x01\x91PPa+\xA6V[P__\x90P[\x88Q\x81\x10\x15a,aWa,S\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x8A\x83\x81Q\x81\x10a,FWa,EaCYV[[` \x02` \x01\x01Qa\"\xE5V[P\x80\x80`\x01\x01\x91PPa,\x06V[Pa,\xA6\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x89_\x81Q\x81\x10a,\x99Wa,\x98aCYV[[` \x02` \x01\x01Qa\"\xE5V[P\x7F\xA2\xDFx0\xE0\xBE\xDE\xF7\xB1\x11k\xF5G\xB4g\xB1kP\xB3\xBD#\x14l\x9E\t\x98x\xD1N\x890\x1A\x87\x8A3`@Qa,\xDA\x93\x92\x91\x90aL\nV[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPV[_a,\xF9\x82_\x01a0\xE1V[\x90P\x91\x90PV[_a-\x0B\x83\x83a0\xF0V[\x90P\x92\x91PPV[\x81`\x0E\x81\x90UPB`\x0F\x81\x90UPC`\x10\x81\x90UP_`\x11_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x06\x81\x11\x15a-LWa-Ka9;V[[\x02\x17\x90UP\x7F\xDE\xF1\xF0\x8E\xB6U\xF4\xA7_`\xBDo\xD7\xE9q\x11 \x04\xAB\xD8F\xA6\x12\xE4m\xAB'\x07p\xD2L\xA50`\x0FT`\x10T\x84`@Qa-\x8A\x94\x93\x92\x91\x90aL?V[`@Q\x80\x91\x03\x90\xA1PPV[_\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a.\x08WPa.\x07\x82a13V[[\x90P\x91\x90PV[_3\x90P\x90V[a. \x82\x82a\x13\\V[a.cW\x80\x82`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.Z\x92\x91\x90aL\x82V[`@Q\x80\x91\x03\x90\xFD[PPV[___`A\x84Q\x03a.\xA7W___` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90Pa.\x99\x88\x82\x85\x85a1\x9CV[\x95P\x95P\x95PPPPa.\xB5V[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15a.\xCFWa.\xCEa9;V[[\x82`\x03\x81\x11\x15a.\xE2Wa.\xE1a9;V[[\x03\x15a0\x1AW`\x01`\x03\x81\x11\x15a.\xFCWa.\xFBa9;V[[\x82`\x03\x81\x11\x15a/\x0FWa/\x0Ea9;V[[\x03a/FW`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15a/ZWa/Ya9;V[[\x82`\x03\x81\x11\x15a/mWa/la9;V[[\x03a/\xB1W\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a/\xA8\x91\x90a9\xD6V[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15a/\xC4Wa/\xC3a9;V[[\x82`\x03\x81\x11\x15a/\xD7Wa/\xD6a9;V[[\x03a0\x19W\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a0\x10\x91\x90a:\xB9V[`@Q\x80\x91\x03\x90\xFD[[PPV[__a0*\x84\x84a2\x83V[\x90P\x80\x15a0WWa0U\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a3l\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[_\x82_\x01\x82\x81T\x81\x10a0wWa0vaCYV[[\x90_R` _ \x01T\x90P\x92\x91PPV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a0\xD5W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a0\xC1W[PPPPP\x90P\x91\x90PV[_\x81_\x01\x80T\x90P\x90P\x91\x90PV[__a0\xFC\x84\x84a3\x99V[\x90P\x80\x15a1)Wa1'\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a4\x82\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[_\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[___\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15a1\xD8W_`\x03\x85\x92P\x92P\x92Pa2yV[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@Qa1\xFB\x94\x93\x92\x91\x90aL\xC4V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a2\x1BW=__>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a2lW_`\x01__\x1B\x93P\x93P\x93PPa2yV[\x80___\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[_a2\x8E\x83\x83a\x13\\V[a3bW`\x01__\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa2\xFFa.\x0FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa3fV[_\x90P[\x92\x91PPV[_a3\x91\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba4\xAFV[\x90P\x92\x91PPV[_a3\xA4\x83\x83a\x13\\V[\x15a4xW___\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa4\x15a.\x0FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa4|V[_\x90P[\x92\x91PPV[_a4\xA7\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba5\x16V[\x90P\x92\x91PPV[_a4\xBA\x83\x83a6\x12V[a5\x0CW\x82_\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x82_\x01\x80T\x90P\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP`\x01\x90Pa5\x10V[_\x90P[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x14a6\x07W_`\x01\x82a5C\x91\x90aD$V[\x90P_`\x01\x86_\x01\x80T\x90Pa5Y\x91\x90aD$V[\x90P\x80\x82\x14a5\xBFW_\x86_\x01\x82\x81T\x81\x10a5xWa5waCYV[[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a5\x99Wa5\x98aCYV[[\x90_R` _ \x01\x81\x90UP\x83\x87`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UPP[\x85_\x01\x80T\x80a5\xD2Wa5\xD1aM\x07V[[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa6\x0CV[_\x91PP[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x14\x15\x90P\x92\x91PPV[P_\x81U`\x01\x01_\x90UV[P\x80T_\x82U\x90_R` _ \x90a6V\x91\x90a6YV[PV[_[\x80\x82\x11\x15a6yW\x82\x81\x01_a6q\x91\x90a6~V[`\x01\x01a6[V[PP\x90V[P\x80Ta6\x8A\x90a@xV[_\x82U\x80`\x1F\x10a6\x9BWPa6\xB5V[`\x1F\x01` \x90\x04\x90_R` _ \x90a6\xB4\x91\x90a6\xB8V[[PV[_[\x80\x82\x11\x15a6\xD0W\x82\x81\x01_\x90U`\x01\x01a6\xBAV[PP\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a7\x1A\x81a6\xE6V[\x81\x14a7$W__\xFD[PV[_\x815\x90Pa75\x81a7\x11V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a7PWa7Oa6\xDEV[[_a7]\x84\x82\x85\x01a7'V[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a7z\x81a7fV[\x82RPPV[_` \x82\x01\x90Pa7\x93_\x83\x01\x84a7qV[\x92\x91PPV[_\x81\x90P\x91\x90PV[a7\xAB\x81a7\x99V[\x81\x14a7\xB5W__\xFD[PV[_\x815\x90Pa7\xC6\x81a7\xA2V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a7\xE2Wa7\xE1a6\xDEV[[_a7\xEF\x85\x82\x86\x01a7\xB8V[\x92PP` a8\0\x85\x82\x86\x01a7\xB8V[\x91PP\x92P\x92\x90PV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a8+Wa8*a8\nV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8HWa8Ga8\x0EV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a8dWa8ca8\x12V[[\x92P\x92\x90PV[__` \x83\x85\x03\x12\x15a8\x81Wa8\x80a6\xDEV[[_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8\x9EWa8\x9Da6\xE2V[[a8\xAA\x85\x82\x86\x01a8\x16V[\x92P\x92PP\x92P\x92\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a8\xDF\x82a8\xB6V[\x90P\x91\x90PV[a8\xEF\x81a8\xD5V[\x81\x14a8\xF9W__\xFD[PV[_\x815\x90Pa9\n\x81a8\xE6V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a9%Wa9$a6\xDEV[[_a92\x84\x82\x85\x01a8\xFCV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x07\x81\x10a9yWa9xa9;V[[PV[_\x81\x90Pa9\x89\x82a9hV[\x91\x90PV[_a9\x98\x82a9|V[\x90P\x91\x90PV[a9\xA8\x81a9\x8EV[\x82RPPV[_` \x82\x01\x90Pa9\xC1_\x83\x01\x84a9\x9FV[\x92\x91PPV[a9\xD0\x81a7\x99V[\x82RPPV[_` \x82\x01\x90Pa9\xE9_\x83\x01\x84a9\xC7V[\x92\x91PPV[___`@\x84\x86\x03\x12\x15a:\x06Wa:\x05a6\xDEV[[_a:\x13\x86\x82\x87\x01a8\xFCV[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:4Wa:3a6\xE2V[[a:@\x86\x82\x87\x01a8\x16V[\x92P\x92PP\x92P\x92P\x92V[_\x81\x90P\x91\x90PV[a:^\x81a:LV[\x81\x14a:hW__\xFD[PV[_\x815\x90Pa:y\x81a:UV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a:\x94Wa:\x93a6\xDEV[[_a:\xA1\x84\x82\x85\x01a:kV[\x91PP\x92\x91PPV[a:\xB3\x81a:LV[\x82RPPV[_` \x82\x01\x90Pa:\xCC_\x83\x01\x84a:\xAAV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a:\xE8Wa:\xE7a6\xDEV[[_a:\xF5\x85\x82\x86\x01a:kV[\x92PP` a;\x06\x85\x82\x86\x01a8\xFCV[\x91PP\x92P\x92\x90PV[a;\x19\x81a8\xD5V[\x82RPPV[_` \x82\x01\x90Pa;2_\x83\x01\x84a;\x10V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a;NWa;Ma6\xDEV[[_a;[\x85\x82\x86\x01a:kV[\x92PP` a;l\x85\x82\x86\x01a7\xB8V[\x91PP\x92P\x92\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a;\xA8\x81a8\xD5V[\x82RPPV[_a;\xB9\x83\x83a;\x9FV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a;\xDB\x82a;vV[a;\xE5\x81\x85a;\x80V[\x93Pa;\xF0\x83a;\x90V[\x80_[\x83\x81\x10\x15a< W\x81Qa<\x07\x88\x82a;\xAEV[\x97Pa<\x12\x83a;\xC5V[\x92PP`\x01\x81\x01\x90Pa;\xF3V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra<E\x81\x84a;\xD1V[\x90P\x92\x91PPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a<\x93\x82a<MV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a<\xB2Wa<\xB1a<]V[[\x80`@RPPPV[_a<\xC4a6\xD5V[\x90Pa<\xD0\x82\x82a<\x8AV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a<\xEFWa<\xEEa<]V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a=\x12a=\r\x84a<\xD5V[a<\xBBV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a=5Wa=4a8\x12V[[\x83[\x81\x81\x10\x15a=^W\x80a=J\x88\x82a8\xFCV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa=7V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a=|Wa={a8\nV[[\x815a=\x8C\x84\x82` \x86\x01a=\0V[\x91PP\x92\x91PPV[__`@\x83\x85\x03\x12\x15a=\xABWa=\xAAa6\xDEV[[_a=\xB8\x85\x82\x86\x01a7\xB8V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=\xD9Wa=\xD8a6\xE2V[[a=\xE5\x85\x82\x86\x01a=hV[\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a>\x04Wa>\x03a6\xDEV[[_a>\x11\x84\x82\x85\x01a7\xB8V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a>L\x81a7\x99V[\x82RPPV[_a>]\x83\x83a>CV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a>\x7F\x82a>\x1AV[a>\x89\x81\x85a>$V[\x93Pa>\x94\x83a>4V[\x80_[\x83\x81\x10\x15a>\xC4W\x81Qa>\xAB\x88\x82a>RV[\x97Pa>\xB6\x83a>iV[\x92PP`\x01\x81\x01\x90Pa>\x97V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra>\xE9\x81\x84a>uV[\x90P\x92\x91PPV[____`\x80\x85\x87\x03\x12\x15a?\tWa?\x08a6\xDEV[[_a?\x16\x87\x82\x88\x01a:kV[\x94PP` a?'\x87\x82\x88\x01a7\xB8V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a?HWa?Ga6\xE2V[[a?T\x87\x82\x88\x01a=hV[\x92PP``a?e\x87\x82\x88\x01a7\xB8V[\x91PP\x92\x95\x91\x94P\x92PV[_`@\x82\x01\x90Pa?\x84_\x83\x01\x85a;\x10V[a?\x91` \x83\x01\x84a9\xC7V[\x93\x92PPPV[_``\x82\x01\x90Pa?\xAB_\x83\x01\x86a;\x10V[a?\xB8` \x83\x01\x85a9\xC7V[a?\xC5`@\x83\x01\x84a9\xC7V[\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a@\x04\x82a7\x99V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a@6Wa@5a?\xCDV[[`\x01\x82\x01\x90P\x91\x90PV[_\x82\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a@\x8FW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a@\xA2Wa@\xA1a@KV[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02aA\x04\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a@\xC9V[aA\x0E\x86\x83a@\xC9V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_aAIaADaA?\x84a7\x99V[aA&V[a7\x99V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aAb\x83aA/V[aAvaAn\x82aAPV[\x84\x84Ta@\xD5V[\x82UPPPPV[__\x90P\x90V[aA\x8DaA~V[aA\x98\x81\x84\x84aAYV[PPPV[_[\x82\x81\x10\x15aA\xBEWaA\xB3_\x82\x84\x01aA\x85V[`\x01\x81\x01\x90PaA\x9FV[PPPV[`\x1F\x82\x11\x15aB\x11W\x82\x82\x11\x15aB\x10WaA\xDD\x81a@\xA8V[aA\xE6\x83a@\xBAV[aA\xEF\x85a@\xBAV[` \x86\x10\x15aA\xFCW_\x90P[\x80\x83\x01aB\x0B\x82\x84\x03\x82aA\x9DV[PPPP[[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_aB1_\x19\x84`\x08\x02aB\x16V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_aBI\x83\x83aB\"V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[aBc\x83\x83a@AV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aB|WaB{a<]V[[aB\x86\x82Ta@xV[aB\x91\x82\x82\x85aA\xC3V[_`\x1F\x83\x11`\x01\x81\x14aB\xBEW_\x84\x15aB\xACW\x82\x87\x015\x90P[aB\xB6\x85\x82aB>V[\x86UPaC\x1DV[`\x1F\x19\x84\x16aB\xCC\x86a@\xA8V[_[\x82\x81\x10\x15aB\xF3W\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaB\xCEV[\x86\x83\x10\x15aC\x10W\x84\x89\x015aC\x0C`\x1F\x89\x16\x82aB\"V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[_aC0\x82a7\x99V[\x91PaC;\x83a7\x99V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15aCSWaCRa?\xCDV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FBUG: the authentication votes by_\x82\x01R\x7F honest clients are inconsistent` \x82\x01RPV[_aC\xF0`@\x83aC\x86V[\x91PaC\xFB\x82aC\x96V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaD\x1D\x81aC\xE4V[\x90P\x91\x90PV[_aD.\x82a7\x99V[\x91PaD9\x83a7\x99V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15aDQWaDPa?\xCDV[[\x92\x91PPV[_aDa\x82a7\x99V[\x91PaDl\x83a7\x99V[\x92P\x82\x82\x02aDz\x81a7\x99V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17aD\x91WaD\x90a?\xCDV[[P\x92\x91PPV[_`@\x82\x01\x90PaD\xAB_\x83\x01\x85a9\xC7V[aD\xB8` \x83\x01\x84a9\xC7V[\x93\x92PPPV[_`@\x82\x01\x90PaD\xD2_\x83\x01\x85a;\x10V[aD\xDF` \x83\x01\x84a;\x10V[\x93\x92PPPV[\x7FBUG: ALREADY RECEIVED SHARES FRO_\x82\x01R\x7FM N PARTIES, TOO MANY CLIENTS\0\0\0` \x82\x01RPV[_aE@`=\x83aC\x86V[\x91PaEK\x82aD\xE6V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaEm\x81aE4V[\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_aE\xCF\x82aE\x9DV[aE\xD9\x81\x85aE\xA7V[\x93PaE\xE9\x81\x85` \x86\x01aE\xB7V[aE\xF2\x81a<MV[\x84\x01\x91PP\x92\x91PPV[_aF\x08\x83\x83aE\xC5V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aF&\x82aEtV[aF0\x81\x85aE~V[\x93P\x83` \x82\x02\x85\x01aFB\x85aE\x8EV[\x80_[\x85\x81\x10\x15aF}W\x84\x84\x03\x89R\x81QaF^\x85\x82aE\xFDV[\x94PaFi\x83aF\x10V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaFEV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaF\xA7\x81\x84aF\x1CV[\x90P\x92\x91PPV[\x7FCURRENTLY ONLY ONE INDEX PER CLI_\x82\x01R\x7FENT ALLOWED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_aG\t`+\x83aC\x86V[\x91PaG\x14\x82aF\xAFV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaG6\x81aF\xFDV[\x90P\x91\x90PV[_\x81T\x90P\x91\x90PV[_\x81\x90P`\x01\x80`\x01\x03\x83\x01\x04\x90P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[aG\x9D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83` \x03`\x08\x02aB\x16V[\x81T\x81\x16\x82UPPPV[aG\xB1\x81a@\xA8V[aG\xBC\x83\x82TaB>V[\x80\x83U_\x82UPPPPV[` \x84\x10_\x81\x14aH#W`\x1F\x84\x11`\x01\x81\x14aG\xF0WaG\xE9\x86\x85aB>V[\x83UaH\x1DV[aG\xF9\x83a@\xA8V[aH\x11`\x01aH\x07\x88a@\xBAV[\x03`\x01\x83\x01aA\x9DV[aH\x1B\x87\x85aG\xA8V[P[PaH}V[aH,\x85a@\xBAV[aH5\x85a@\xBAV[aH>\x84a@\xA8V[\x82\x81\x01`\x1F\x89\x16\x80\x15aHYWaHX\x81`\x01\x84\x03aGmV[[\x84\x84\x11\x15aHnWaHm\x85\x85\x03\x83aA\x9DV[[`\x01\x8A`\x02\x02\x17\x87UPPPPP[PPPPPV[h\x01\0\0\0\0\0\0\0\0\x84\x11\x15aH\x9EWaH\x9Da<]V[[` \x83\x10_\x81\x14aH\xE7W` \x85\x10_\x81\x14aH\xC5WaH\xBE\x86\x85aB>V[\x83UaH\xE1V[\x83`\xFF\x19\x16\x93P\x83aH\xD6\x84a@\xA8V[U`\x01\x86`\x02\x02\x01\x83U[PaH\xF1V[`\x01\x85`\x02\x02\x01\x82U[PPPPPV[\x80TaI\x03\x81a@xV[\x80\x84\x11\x15aI\x18WaI\x17\x84\x82\x84\x86aH\x84V[[\x80\x84\x10\x15aI-WaI,\x84\x82\x84\x86aG\xC8V[[PPPPV[\x82\x81\x10\x15aIRWaIG_\x82\x84\x01aA\x85V[`\x01\x81\x01\x90PaI3V[PPPV[aIa_\x82aH\xF8V[PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[_\x82\x14aI\xA0WaI\x9FaIdV[[aI\xA9\x81aIWV[PPV[_[\x82\x81\x10\x15aI\xCEWaI\xC3_\x82\x84\x01aI\x90V[`\x01\x81\x01\x90PaI\xAFV[PPPV[\x81\x83\x10\x15aJ\nWaI\xE4\x82aGGV[aI\xED\x84aGGV[aI\xF6\x83aG[V[\x81\x81\x01aJ\x05\x83\x85\x03\x82aI\xADV[PPPP[PPPV[h\x01\0\0\0\0\0\0\0\0\x82\x11\x15aJ)WaJ(a<]V[[aJ2\x81aG=V[\x82\x82UaJ@\x83\x82\x84aI\xD3V[PPPV[_\x81Q\x90P\x91\x90PV[aJX\x82aE\x9DV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aJqWaJpa<]V[[aJ{\x82Ta@xV[aJ\x86\x82\x82\x85aA\xC3V[_` \x90P`\x1F\x83\x11`\x01\x81\x14aJ\xB7W_\x84\x15aJ\xA5W\x82\x87\x01Q\x90P[aJ\xAF\x85\x82aB>V[\x86UPaK\x16V[`\x1F\x19\x84\x16aJ\xC5\x86a@\xA8V[_[\x82\x81\x10\x15aJ\xECW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaJ\xC7V[\x86\x83\x10\x15aK\tW\x84\x89\x01QaK\x05`\x1F\x89\x16\x82aB\"V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[aK(\x82\x82aJOV[PPV[aK5\x82aEtV[aK?\x81\x83aJ\x0FV[aKH\x83aE\x8EV[aKQ\x83aG[V[_[\x83\x81\x10\x15aK\x86WaKd\x83aJEV[aKn\x81\x84aK\x1EV[` \x84\x01\x93P`\x01\x83\x01\x92PP`\x01\x81\x01\x90PaKSV[PPPPPPV[_`@\x82\x01\x90PaK\xA1_\x83\x01\x85a;\x10V[\x81\x81\x03` \x83\x01RaK\xB3\x81\x84a>uV[\x90P\x93\x92PPPV[_`@\x82\x01\x90PaK\xCF_\x83\x01\x85a9\x9FV[aK\xDC` \x83\x01\x84a9\x9FV[\x93\x92PPPV[_`@\x82\x01\x90PaK\xF6_\x83\x01\x85a9\xC7V[aL\x03` \x83\x01\x84a;\x10V[\x93\x92PPPV[_``\x82\x01\x90PaL\x1D_\x83\x01\x86a9\xC7V[aL*` \x83\x01\x85a9\xC7V[aL7`@\x83\x01\x84a;\x10V[\x94\x93PPPPV[_`\x80\x82\x01\x90PaLR_\x83\x01\x87a;\x10V[aL_` \x83\x01\x86a9\xC7V[aLl`@\x83\x01\x85a9\xC7V[aLy``\x83\x01\x84a;\x10V[\x95\x94PPPPPV[_`@\x82\x01\x90PaL\x95_\x83\x01\x85a;\x10V[aL\xA2` \x83\x01\x84a:\xAAV[\x93\x92PPPV[_`\xFF\x82\x16\x90P\x91\x90PV[aL\xBE\x81aL\xA9V[\x82RPPV[_`\x80\x82\x01\x90PaL\xD7_\x83\x01\x87a:\xAAV[aL\xE4` \x83\x01\x86aL\xB5V[aL\xF1`@\x83\x01\x85a:\xAAV[aL\xFE``\x83\x01\x84a:\xAAV[\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 3%3k\x06\xA7\x95N\xC3\xC2\0\t\x17\xEB\xEB\xB9\xF1m=\t\xCC/\xA3d\x85\x82\xCA\x1C$\xB1q\xDFdsolcC\0\x08!\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610225575f3560e01c80637f35b5601161012e578063ca15c873116100b6578063eb8575de1161007a578063eb8575de146105cb578063ebae35e7146105e7578063f2fde38b14610617578063f6603c6114610633578063fc78b2e81461064f57610225565b8063ca15c87314610539578063cb9c4cc414610569578063d547741f14610573578063d8270dce1461058f578063eae6f652146105ad57610225565b8063a217fddf116100fd578063a217fddf146104bb578063a3246ad3146104d9578063af206f2814610509578063bb51fef014610525578063c079f4951461052f57610225565b80637f35b5601461041f5780638da5cb5b1461043d5780639010d07c1461045b57806391d148541461048b57610225565b8063248a9ca3116101b157806336568abe1161018057806336568abe146103c95780633b4338d1146103e55780634b8e6488146104015780634bb278f31461040b578063715018a61461041557610225565b8063248a9ca3146103555780632f2ff15d1461038557806330104c3e146103a157806333cc9a09146103bf57610225565b8063146ca531116101f8578063146ca531146102c157806317634514146102df5780631c7453db146102fd5780631ee4ee0f1461031b5780632328bd121461033757610225565b806301ffc9a7146102295780630bda81cf146102595780630d42eb6f1461027557806313ff6dd514610291575b5f5ffd5b610243600480360381019061023e919061373b565b61067f565b6040516102509190613780565b60405180910390f35b610273600480360381019061026e91906137cc565b6106f8565b005b61028f600480360381019061028a919061386b565b61094a565b005b6102ab60048036038101906102a69190613910565b61098b565b6040516102b89190613780565b60405180910390f35b6102c9610a07565b6040516102d691906139ae565b60405180910390f35b6102e7610a19565b6040516102f491906139d6565b60405180910390f35b610305610a1f565b60405161031291906139d6565b60405180910390f35b610335600480360381019061033091906139ef565b610a25565b005b61033f610fd3565b60405161034c91906139d6565b60405180910390f35b61036f600480360381019061036a9190613a7f565b610fe9565b60405161037c9190613ab9565b60405180910390f35b61039f600480360381019061039a9190613ad2565b611005565b005b6103a961103f565b6040516103b69190613ab9565b60405180910390f35b6103c7611063565b005b6103e360048036038101906103de9190613ad2565b6110dd565b005b6103ff60048036038101906103fa91906137cc565b6111a4565b005b6104096111dd565b005b610413611257565b005b61041d6112d1565b005b6104276112e4565b6040516104349190613ab9565b60405180910390f35b610445611308565b6040516104529190613b1f565b60405180910390f35b61047560048036038101906104709190613b38565b611330565b6040516104829190613b1f565b60405180910390f35b6104a560048036038101906104a09190613ad2565b61135c565b6040516104b29190613780565b60405180910390f35b6104c36113bf565b6040516104d09190613ab9565b60405180910390f35b6104f360048036038101906104ee9190613a7f565b6113c5565b6040516105009190613c2d565b60405180910390f35b610523600480360381019061051e9190613d95565b6113e7565b005b61052d611420565b005b61053761149a565b005b610553600480360381019061054e9190613a7f565b611514565b60405161056091906139d6565b60405180910390f35b610571611535565b005b61058d60048036038101906105889190613ad2565b6115ae565b005b6105976116a1565b6040516105a491906139d6565b60405180910390f35b6105b56116a7565b6040516105c29190613ab9565b60405180910390f35b6105e560048036038101906105e091906139ef565b6116cb565b005b61060160048036038101906105fc9190613def565b611c10565b60405161060e9190613ed1565b60405180910390f35b610631600480360381019061062c9190613910565b6120dc565b005b61064d60048036038101906106489190613ef1565b612160565b005b61066960048036038101906106649190613910565b6121c9565b6040516106769190613780565b60405180910390f35b5f7f07effe0d000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191614806106f157506106f0826121fb565b5b9050919050565b7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c02523261072281612274565b3373ffffffffffffffffffffffffffffffffffffffff1660055f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16146107c45733826040517fffabbae70000000000000000000000000000000000000000000000000000000081526004016107bb929190613f71565b60405180910390fd5b5f830361080857336040517f16923cea0000000000000000000000000000000000000000000000000000000081526004016107ff9190613b1f565b60405180910390fd5b5f600c5f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20600101541461088c57336040517f4f5fbfc30000000000000000000000000000000000000000000000000000000081526004016108839190613b1f565b60405180910390fd5b604051806040016040528083815260200184815250600c5f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f820151815f0155602082015181600101559050507fb89addd937f44f902c849596641837cd7af2fcecef22d2a7866fdc1ad9c0ae2e33848460405161092693929190613f98565b60405180910390a160095f81548092919061094090613ffa565b9190505550505050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61097481612274565b828260049182610985929190614259565b50505050565b5f610995826121c9565b6109d657816040517fabdce06a0000000000000000000000000000000000000000000000000000000081526004016109cd9190613b1f565b60405180910390fd5b610a007f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e8361135c565b9050919050565b60115f9054906101000a900460ff1681565b60105481565b600a5481565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469610a4f81612274565b610a797fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c0252328561135c565b610aba57836040517fa032ac6b000000000000000000000000000000000000000000000000000000008152600401610ab19190613b1f565b60405180910390fd5b5f60075490505f5f90505b600754811015610b47578573ffffffffffffffffffffffffffffffffffffffff1660055f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1603610b3a57809150610b47565b8080600101915050610ac5565b506007548103610bbe5760055f5f81526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040517f6faf9f05000000000000000000000000000000000000000000000000000000008152600401610bb59190613b1f565b60405180910390fd5b5f81600a54610bcd9190614326565b90505f610bff82604051602001610be491906139d6565b60405160208183030381529060405280519060200120612288565b90505f610c4f8288888080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050506122bb565b90508773ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603610ced5760065f8973ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20600160028110610cd457610cd3614359565b5b015f8154610ce190613ffa565b91905081905550610d51565b60065f8973ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f60028110610d3c57610d3b614359565b5b015f8154610d4990613ffa565b919050819055505b6001600b54610d609190614326565b60065f8a73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f60028110610daf57610dae614359565b5b01541080610e1b57506001600b54610dc79190614326565b60065f8a73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20600160028110610e1757610e16614359565b5b0154105b610e5a576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610e5190614406565b60405180910390fd5b6001600b54610e699190614326565b60065f8a73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f60028110610eb857610eb7614359565b5b015410610f12578773ffffffffffffffffffffffffffffffffffffffff167f407009200520f9f10584813c0b95441ab320f6b08d97ebdaaf1e824eedd9d7c35f604051610f059190613780565b60405180910390a2610fc9565b6001600b54610f219190614326565b60065f8a73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20600160028110610f7157610f70614359565b5b015410610fc8578773ffffffffffffffffffffffffffffffffffffffff167f407009200520f9f10584813c0b95441ab320f6b08d97ebdaaf1e824eedd9d7c36001604051610fbf9190613780565b60405180910390a25b5b5050505050505050565b5f600854600754610fe49190614424565b905090565b5f5f5f8381526020019081526020015f20600101549050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61102f81612274565b61103983836122e5565b50505050565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46981565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61108d81612274565b6003611098816122f8565b7f20f55ed0c92f2bb1c8825488e1e3c98463d024b2a42dbd24838c3f75260f43e933426040516110c9929190613f71565b60405180910390a16110d9612381565b5050565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469821480156111125750611111828261135c565b5b15611196575f61112183611514565b90505f600160025460036111359190614457565b61113f9190614326565b9050808203611193576001816111559190614424565b816040517f3a23626800000000000000000000000000000000000000000000000000000000815260040161118a929190614498565b60405180910390fd5b50505b6111a082826123ea565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6111ce81612274565b6111d88383612465565b505050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61120781612274565b6004611212816122f8565b7f301f8a3701f5b260197382dd7301078542144fe8fddd18083d6f6e09e4958a593342604051611243929190613f71565b60405180910390a1611253612381565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61128181612274565b600561128c816122f8565b7f24a873651d26fb5a462cb37a91071cdd4d09abeebfe0ed144329bed1cc359d0333426040516112bd929190613f71565b60405180910390a16112cd612381565b5050565b6112d96127ed565b6112e25f612874565b565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e81565b5f600d5f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b5f6113548260015f8681526020019081526020015f2061293790919063ffffffff16565b905092915050565b5f5f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f5f1b81565b60606113e060015f8481526020019081526020015f2061294e565b9050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61141181612274565b61141b838361296d565b505050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61144a81612274565b6002611455816122f8565b7f60edf9bdc7c4ea007cae1a9bbd03e41e5bfccd7231a6ec383c2edd7800f0d20c3342604051611486929190613f71565b60405180910390a1611496612381565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6114c481612274565b60016114cf816122f8565b7f67c4489f674d03c7d19a9e36735188de7c65e8d1e99eb3a2fd258a769eb14fff3342604051611500929190613f71565b60405180910390a1611510612381565b5050565b5f61152e60015f8481526020019081526020015f20612ced565b9050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61155f81612274565b5f611569816122f8565b7fbb709dea744f06d1b26e824deec2f7140c511266ee15d7a217838b31d8b0123d334260405161159a929190613f71565b60405180910390a16115aa612381565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6115d881612274565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698314801561160d575061160c838361135c565b5b15611691575f61161c84611514565b90505f600160025460036116309190614457565b61163a9190614326565b905080820361168e576001816116509190614424565b816040517f3a236268000000000000000000000000000000000000000000000000000000008152600401611685929190614498565b60405180910390fd5b50505b61169b8383612d00565b50505050565b600f5481565b7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c02523281565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696116f581612274565b61171f7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c0252328561135c565b61176057836040517fa032ac6b0000000000000000000000000000000000000000000000000000000081526004016117579190613b1f565b60405180910390fd5b5f60035f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f2060010154905060035f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206002015f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16156118705784336040517f08e554950000000000000000000000000000000000000000000000000000000081526004016118679291906144bf565b60405180910390fd5b6001600b5460036118819190614457565b61188b9190614326565b81106118cc576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016118c390614556565b60405180910390fd5b600160035f8773ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206002015f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550838360035f8873ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f0183815481106119b2576119b1614359565b5b905f5260205f200191826119c7929190614259565b50600160035f8773ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206001015f828254611a189190614326565b92505081905550600181611a2c9190614326565b905060016002546002611a3f9190614457565b611a499190614326565b8110611c09575f8167ffffffffffffffff811115611a6a57611a69613c5d565b5b604051908082528060200260200182016040528015611a9d57816020015b6060815260200190600190039081611a885790505b5090505f5f90505b82811015611bb85760035f8873ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f018181548110611afe57611afd614359565b5b905f5260205f20018054611b1190614078565b80601f0160208091040260200160405190810160405280929190818152602001828054611b3d90614078565b8015611b885780601f10611b5f57610100808354040283529160200191611b88565b820191905f5260205f20905b815481529060010190602001808311611b6b57829003601f168201915b5050505050828281518110611ba057611b9f614359565b5b60200260200101819052508080600101915050611aa5565b508573ffffffffffffffffffffffffffffffffffffffff167f23e91dbfae03758cb88d7f6252b5710afa53a19ffe0f4b4f75d7f2de0c5eabe982604051611bff919061468f565b60405180910390a2505b5050505050565b606060018214611c55576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611c4c9061471f565b60405180910390fd5b5f8203611c9957336040517fb2fd5518000000000000000000000000000000000000000000000000000000008152600401611c909190613b1f565b60405180910390fd5b5f600854600754611caa9190614424565b905080831115611cf35782816040517fdf3d75e2000000000000000000000000000000000000000000000000000000008152600401611cea929190614498565b60405180910390fd5b5f5f90505b600754811015611daf573373ffffffffffffffffffffffffffffffffffffffff1660055f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1603611da257336040517faca92f09000000000000000000000000000000000000000000000000000000008152600401611d999190613b1f565b60405180910390fd5b8080600101915050611cf8565b505f600184600854611dc19190614326565b611dcb9190614424565b90505f8467ffffffffffffffff811115611de857611de7613c5d565b5b604051908082528060200260200182016040528015611e165781602001602082028036833780820191505090505b5090505f60085490505b8260085411611ec5573360055f60085481526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506008548282600854611e8c9190614424565b81518110611e9d57611e9c614359565b5b60200260200101818152505060085f815480929190611ebb90613ffa565b9190505550611e20565b611eef7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c025232336122e5565b505f60035f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f2090506001600b546003611f419190614457565b611f4b9190614326565b67ffffffffffffffff811115611f6457611f63613c5d565b5b604051908082528060200260200182016040528015611f9757816020015b6060815260200190600190039081611f825790505b50815f019081611fa79190614b2c565b505f81600101819055505f611fdb7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696113c5565b90505f6120077fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469611514565b90505f5f90505b81811015612093575f846002015f85848151811061202f5761202e614359565b5b602002602001015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550808060010191505061200e565b507f1e14abe5d0cdb96adde7b9eca9b14bc08df623b5805afde5a3f0acadc2bf4f5b33866040516120c5929190614b8e565b60405180910390a184975050505050505050919050565b6120e46127ed565b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603612154575f6040517f1e4fbdf700000000000000000000000000000000000000000000000000000000815260040161214b9190613b1f565b60405180910390fd5b61215d81612874565b50565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61218a81612274565b612194848461296d565b61219e8285612465565b6121c285845f815181106121b5576121b4614359565b5b6020026020010151612d13565b5050505050565b5f6121f47fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698361135c565b9050919050565b5f7f5a05180f000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916148061226d575061226c82612d96565b5b9050919050565b61228581612280612e0f565b612e16565b50565b5f7f19457468657265756d205369676e6564204d6573736167653a0a3332000000005f5281601c52603c5f209050919050565b5f5f5f5f6122c98686612e67565b9250925092506122d98282612ebc565b82935050505092915050565b5f6122f0838361301e565b905092915050565b80600681111561230b5761230a61393b565b5b60115f9054906101000a900460ff16600681111561232c5761232b61393b565b5b1461237e578060115f9054906101000a900460ff166040517fbfa217d8000000000000000000000000000000000000000000000000000000008152600401612375929190614bbc565b60405180910390fd5b50565b600160115f9054906101000a900460ff1660068111156123a4576123a361393b565b5b6123ae9190614326565b60068111156123c0576123bf61393b565b5b60115f6101000a81548160ff021916908360068111156123e3576123e261393b565b5b0217905550565b6123f2612e0f565b73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614612456576040517f6697b23200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6124608282612d00565b505050565b600754600a5f8282546124789190614326565b92505081905550816007819055505f6008819055505f60098190555080600b819055505f6124c57fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696113c5565b90505f6124f17fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469611514565b90505f5f90505b6007548110156127ab57600c5f60055f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f5f82015f9055600182015f9055505060065f60055f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6125f79190613632565b5f5f90505b828110156126e55760035f60055f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206002015f85838151811061268857612687614359565b5b602002602001015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81549060ff021916905580806001019150506125fc565b5060035f60055f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f5f82015f612762919061363e565b600182015f9055505060055f8281526020019081526020015f205f6101000a81549073ffffffffffffffffffffffffffffffffffffffff021916905580806001019150506124f8565b507ff7f0872382dff5e698b284e12084e4e7894f830216dd80cb4e909b593a58f935600754336040516127df929190614be3565b60405180910390a150505050565b6127f5612e0f565b73ffffffffffffffffffffffffffffffffffffffff16612813611308565b73ffffffffffffffffffffffffffffffffffffffff161461287257612836612e0f565b6040517f118cdaa70000000000000000000000000000000000000000000000000000000081526004016128699190613b1f565b60405180910390fd5b565b5f600d5f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905081600d5f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a35050565b5f612944835f0183613061565b5f1c905092915050565b60605f61295c835f01613088565b905060608190508092505050919050565b5f600183600361297d9190614457565b6129879190614326565b905080825110156129d2578151816040517f3a2362680000000000000000000000000000000000000000000000000000000081526004016129c9929190614498565b60405180910390fd5b826002819055505f612a037f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6113c5565b90505f612a2f7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e611514565b90505f5f90505b81811015612a9057612a827f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e848381518110612a7557612a74614359565b5b6020026020010151612d00565b508080600101915050612a36565b505f612abb7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696113c5565b90505f612ae77fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469611514565b90505f5f90505b81811015612b4857612b3a7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469848381518110612b2d57612b2c614359565b5b6020026020010151612d00565b508080600101915050612aee565b505f612b737fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c0252326113c5565b90505f612b9f7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c025232611514565b90505f5f90505b81811015612c0057612bf27fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c025232848381518110612be557612be4614359565b5b6020026020010151612d00565b508080600101915050612ba6565b505f5f90505b8851811015612c6157612c537fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698a8381518110612c4657612c45614359565b5b60200260200101516122e5565b508080600101915050612c06565b50612ca67f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e895f81518110612c9957612c98614359565b5b60200260200101516122e5565b507fa2df7830e0bedef7b1116bf547b467b16b50b3bd23146c9e099878d14e89301a878a33604051612cda93929190614c0a565b60405180910390a1505050505050505050565b5f612cf9825f016130e1565b9050919050565b5f612d0b83836130f0565b905092915050565b81600e8190555042600f81905550436010819055505f60115f6101000a81548160ff02191690836006811115612d4c57612d4b61393b565b5b02179055507fdef1f08eb655f4a75f60bd6fd7e971112004abd846a612e46dab270770d24ca530600f5460105484604051612d8a9493929190614c3f565b60405180910390a15050565b5f7f7965db0b000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19161480612e085750612e0782613133565b5b9050919050565b5f33905090565b612e20828261135c565b612e635780826040517fe2517d3f000000000000000000000000000000000000000000000000000000008152600401612e5a929190614c82565b60405180910390fd5b5050565b5f5f5f6041845103612ea7575f5f5f602087015192506040870151915060608701515f1a9050612e998882858561319c565b955095509550505050612eb5565b5f600285515f1b9250925092505b9250925092565b5f6003811115612ecf57612ece61393b565b5b826003811115612ee257612ee161393b565b5b031561301a5760016003811115612efc57612efb61393b565b5b826003811115612f0f57612f0e61393b565b5b03612f46576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60026003811115612f5a57612f5961393b565b5b826003811115612f6d57612f6c61393b565b5b03612fb157805f1c6040517ffce698f7000000000000000000000000000000000000000000000000000000008152600401612fa891906139d6565b60405180910390fd5b600380811115612fc457612fc361393b565b5b826003811115612fd757612fd661393b565b5b0361301957806040517fd78bce0c0000000000000000000000000000000000000000000000000000000081526004016130109190613ab9565b60405180910390fd5b5b5050565b5f5f61302a8484613283565b90508015613057576130558360015f8781526020019081526020015f2061336c90919063ffffffff16565b505b8091505092915050565b5f825f01828154811061307757613076614359565b5b905f5260205f200154905092915050565b6060815f018054806020026020016040519081016040528092919081815260200182805480156130d557602002820191905f5260205f20905b8154815260200190600101908083116130c1575b50505050509050919050565b5f815f01805490509050919050565b5f5f6130fc8484613399565b90508015613129576131278360015f8781526020019081526020015f2061348290919063ffffffff16565b505b8091505092915050565b5f7f01ffc9a7000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916149050919050565b5f5f5f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c11156131d8575f600385925092509250613279565b5f6001888888886040515f81526020016040526040516131fb9493929190614cc4565b6020604051602081039080840390855afa15801561321b573d5f5f3e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff160361326c575f60015f5f1b93509350935050613279565b805f5f5f1b935093509350505b9450945094915050565b5f61328e838361135c565b6133625760015f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055506132ff612e0f565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a460019050613366565b5f90505b92915050565b5f613391835f018373ffffffffffffffffffffffffffffffffffffffff165f1b6134af565b905092915050565b5f6133a4838361135c565b15613478575f5f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550613415612e0f565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b60405160405180910390a46001905061347c565b5f90505b92915050565b5f6134a7835f018373ffffffffffffffffffffffffffffffffffffffff165f1b613516565b905092915050565b5f6134ba8383613612565b61350c57825f0182908060018154018082558091505060019003905f5260205f20015f9091909190915055825f0180549050836001015f8481526020019081526020015f208190555060019050613510565b5f90505b92915050565b5f5f836001015f8481526020019081526020015f205490505f8114613607575f6001826135439190614424565b90505f6001865f01805490506135599190614424565b90508082146135bf575f865f01828154811061357857613577614359565b5b905f5260205f200154905080875f01848154811061359957613598614359565b5b905f5260205f20018190555083876001015f8381526020019081526020015f2081905550505b855f018054806135d2576135d1614d07565b5b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f90556001935050505061360c565b5f9150505b92915050565b5f5f836001015f8481526020019081526020015f20541415905092915050565b505f81556001015f9055565b5080545f8255905f5260205f20906136569190613659565b50565b5f5b80821115613679578281015f613671919061367e565b60010161365b565b505090565b50805461368a90614078565b5f825580601f1061369b57506136b5565b601f0160209004905f5260205f20906136b491906136b8565b5b50565b5f5b808211156136d0578281015f90556001016136ba565b505090565b5f604051905090565b5f5ffd5b5f5ffd5b5f7fffffffff0000000000000000000000000000000000000000000000000000000082169050919050565b61371a816136e6565b8114613724575f5ffd5b50565b5f8135905061373581613711565b92915050565b5f602082840312156137505761374f6136de565b5b5f61375d84828501613727565b91505092915050565b5f8115159050919050565b61377a81613766565b82525050565b5f6020820190506137935f830184613771565b92915050565b5f819050919050565b6137ab81613799565b81146137b5575f5ffd5b50565b5f813590506137c6816137a2565b92915050565b5f5f604083850312156137e2576137e16136de565b5b5f6137ef858286016137b8565b9250506020613800858286016137b8565b9150509250929050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f84011261382b5761382a61380a565b5b8235905067ffffffffffffffff8111156138485761384761380e565b5b60208301915083600182028301111561386457613863613812565b5b9250929050565b5f5f60208385031215613881576138806136de565b5b5f83013567ffffffffffffffff81111561389e5761389d6136e2565b5b6138aa85828601613816565b92509250509250929050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6138df826138b6565b9050919050565b6138ef816138d5565b81146138f9575f5ffd5b50565b5f8135905061390a816138e6565b92915050565b5f60208284031215613925576139246136de565b5b5f613932848285016138fc565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b600781106139795761397861393b565b5b50565b5f81905061398982613968565b919050565b5f6139988261397c565b9050919050565b6139a88161398e565b82525050565b5f6020820190506139c15f83018461399f565b92915050565b6139d081613799565b82525050565b5f6020820190506139e95f8301846139c7565b92915050565b5f5f5f60408486031215613a0657613a056136de565b5b5f613a13868287016138fc565b935050602084013567ffffffffffffffff811115613a3457613a336136e2565b5b613a4086828701613816565b92509250509250925092565b5f819050919050565b613a5e81613a4c565b8114613a68575f5ffd5b50565b5f81359050613a7981613a55565b92915050565b5f60208284031215613a9457613a936136de565b5b5f613aa184828501613a6b565b91505092915050565b613ab381613a4c565b82525050565b5f602082019050613acc5f830184613aaa565b92915050565b5f5f60408385031215613ae857613ae76136de565b5b5f613af585828601613a6b565b9250506020613b06858286016138fc565b9150509250929050565b613b19816138d5565b82525050565b5f602082019050613b325f830184613b10565b92915050565b5f5f60408385031215613b4e57613b4d6136de565b5b5f613b5b85828601613a6b565b9250506020613b6c858286016137b8565b9150509250929050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b613ba8816138d5565b82525050565b5f613bb98383613b9f565b60208301905092915050565b5f602082019050919050565b5f613bdb82613b76565b613be58185613b80565b9350613bf083613b90565b805f5b83811015613c20578151613c078882613bae565b9750613c1283613bc5565b925050600181019050613bf3565b5085935050505092915050565b5f6020820190508181035f830152613c458184613bd1565b905092915050565b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b613c9382613c4d565b810181811067ffffffffffffffff82111715613cb257613cb1613c5d565b5b80604052505050565b5f613cc46136d5565b9050613cd08282613c8a565b919050565b5f67ffffffffffffffff821115613cef57613cee613c5d565b5b602082029050602081019050919050565b5f613d12613d0d84613cd5565b613cbb565b90508083825260208201905060208402830185811115613d3557613d34613812565b5b835b81811015613d5e5780613d4a88826138fc565b845260208401935050602081019050613d37565b5050509392505050565b5f82601f830112613d7c57613d7b61380a565b5b8135613d8c848260208601613d00565b91505092915050565b5f5f60408385031215613dab57613daa6136de565b5b5f613db8858286016137b8565b925050602083013567ffffffffffffffff811115613dd957613dd86136e2565b5b613de585828601613d68565b9150509250929050565b5f60208284031215613e0457613e036136de565b5b5f613e11848285016137b8565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b613e4c81613799565b82525050565b5f613e5d8383613e43565b60208301905092915050565b5f602082019050919050565b5f613e7f82613e1a565b613e898185613e24565b9350613e9483613e34565b805f5b83811015613ec4578151613eab8882613e52565b9750613eb683613e69565b925050600181019050613e97565b5085935050505092915050565b5f6020820190508181035f830152613ee98184613e75565b905092915050565b5f5f5f5f60808587031215613f0957613f086136de565b5b5f613f1687828801613a6b565b9450506020613f27878288016137b8565b935050604085013567ffffffffffffffff811115613f4857613f476136e2565b5b613f5487828801613d68565b9250506060613f65878288016137b8565b91505092959194509250565b5f604082019050613f845f830185613b10565b613f9160208301846139c7565b9392505050565b5f606082019050613fab5f830186613b10565b613fb860208301856139c7565b613fc560408301846139c7565b949350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61400482613799565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff820361403657614035613fcd565b5b600182019050919050565b5f82905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061408f57607f821691505b6020821081036140a2576140a161404b565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026141047fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff826140c9565b61410e86836140c9565b95508019841693508086168417925050509392505050565b5f819050919050565b5f61414961414461413f84613799565b614126565b613799565b9050919050565b5f819050919050565b6141628361412f565b61417661416e82614150565b8484546140d5565b825550505050565b5f5f905090565b61418d61417e565b614198818484614159565b505050565b5f5b828110156141be576141b35f828401614185565b60018101905061419f565b505050565b601f8211156142115782821115614210576141dd816140a8565b6141e6836140ba565b6141ef856140ba565b60208610156141fc575f90505b80830161420b8284038261419d565b505050505b5b505050565b5f82821c905092915050565b5f6142315f1984600802614216565b1980831691505092915050565b5f6142498383614222565b9150826002028217905092915050565b6142638383614041565b67ffffffffffffffff81111561427c5761427b613c5d565b5b6142868254614078565b6142918282856141c3565b5f601f8311600181146142be575f84156142ac578287013590505b6142b6858261423e565b86555061431d565b601f1984166142cc866140a8565b5f5b828110156142f3578489013582556001820191506020850194506020810190506142ce565b86831015614310578489013561430c601f891682614222565b8355505b6001600288020188555050505b50505050505050565b5f61433082613799565b915061433b83613799565b925082820190508082111561435357614352613fcd565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f82825260208201905092915050565b7f4255473a207468652061757468656e7469636174696f6e20766f7465732062795f8201527f20686f6e65737420636c69656e74732061726520696e636f6e73697374656e74602082015250565b5f6143f0604083614386565b91506143fb82614396565b604082019050919050565b5f6020820190508181035f83015261441d816143e4565b9050919050565b5f61442e82613799565b915061443983613799565b925082820390508181111561445157614450613fcd565b5b92915050565b5f61446182613799565b915061446c83613799565b925082820261447a81613799565b9150828204841483151761449157614490613fcd565b5b5092915050565b5f6040820190506144ab5f8301856139c7565b6144b860208301846139c7565b9392505050565b5f6040820190506144d25f830185613b10565b6144df6020830184613b10565b9392505050565b7f4255473a20414c5245414459205245434549564544205348415245532046524f5f8201527f4d204e20504152544945532c20544f4f204d414e5920434c49454e5453000000602082015250565b5f614540603d83614386565b915061454b826144e6565b604082019050919050565b5f6020820190508181035f83015261456d81614534565b9050919050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f6145cf8261459d565b6145d981856145a7565b93506145e98185602086016145b7565b6145f281613c4d565b840191505092915050565b5f61460883836145c5565b905092915050565b5f602082019050919050565b5f61462682614574565b614630818561457e565b9350836020820285016146428561458e565b805f5b8581101561467d578484038952815161465e85826145fd565b945061466983614610565b925060208a01995050600181019050614645565b50829750879550505050505092915050565b5f6020820190508181035f8301526146a7818461461c565b905092915050565b7f43555252454e544c59204f4e4c59204f4e4520494e4445582050455220434c495f8201527f454e5420414c4c4f574544000000000000000000000000000000000000000000602082015250565b5f614709602b83614386565b9150614714826146af565b604082019050919050565b5f6020820190508181035f830152614736816146fd565b9050919050565b5f81549050919050565b5f8190506001806001038301049050919050565b5f819050815f5260205f209050919050565b61479d7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff83602003600802614216565b815481168255505050565b6147b1816140a8565b6147bc83825461423e565b8083555f825550505050565b602084105f811461482357601f8411600181146147f0576147e9868561423e565b835561481d565b6147f9836140a8565b6148116001614807886140ba565b036001830161419d565b61481b87856147a8565b505b5061487d565b61482c856140ba565b614835856140ba565b61483e846140a8565b828101601f8916801561485957614858816001840361476d565b5b8484111561486e5761486d8585038361419d565b5b60018a60020217875550505050505b5050505050565b6801000000000000000084111561489e5761489d613c5d565b5b602083105f81146148e757602085105f81146148c5576148be868561423e565b83556148e1565b8360ff19169350836148d6846140a8565b556001866002020183555b506148f1565b6001856002020182555b5050505050565b805461490381614078565b808411156149185761491784828486614884565b5b8084101561492d5761492c848284866147c8565b5b50505050565b82811015614952576149475f828401614185565b600181019050614933565b505050565b6149615f826148f8565b50565b7f4e487b71000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b5f82146149a05761499f614964565b5b6149a981614957565b5050565b5f5b828110156149ce576149c35f828401614990565b6001810190506149af565b505050565b81831015614a0a576149e482614747565b6149ed84614747565b6149f68361475b565b818101614a05838503826149ad565b505050505b505050565b68010000000000000000821115614a2957614a28613c5d565b5b614a328161473d565b828255614a408382846149d3565b505050565b5f81519050919050565b614a588261459d565b67ffffffffffffffff811115614a7157614a70613c5d565b5b614a7b8254614078565b614a868282856141c3565b5f60209050601f831160018114614ab7575f8415614aa5578287015190505b614aaf858261423e565b865550614b16565b601f198416614ac5866140a8565b5f5b82811015614aec57848901518255600182019150602085019450602081019050614ac7565b86831015614b095784890151614b05601f891682614222565b8355505b6001600288020188555050505b505050505050565b614b288282614a4f565b5050565b614b3582614574565b614b3f8183614a0f565b614b488361458e565b614b518361475b565b5f5b83811015614b8657614b6483614a45565b614b6e8184614b1e565b60208401935060018301925050600181019050614b53565b505050505050565b5f604082019050614ba15f830185613b10565b8181036020830152614bb38184613e75565b90509392505050565b5f604082019050614bcf5f83018561399f565b614bdc602083018461399f565b9392505050565b5f604082019050614bf65f8301856139c7565b614c036020830184613b10565b9392505050565b5f606082019050614c1d5f8301866139c7565b614c2a60208301856139c7565b614c376040830184613b10565b949350505050565b5f608082019050614c525f830187613b10565b614c5f60208301866139c7565b614c6c60408301856139c7565b614c796060830184613b10565b95945050505050565b5f604082019050614c955f830185613b10565b614ca26020830184613aaa565b9392505050565b5f60ff82169050919050565b614cbe81614ca9565b82525050565b5f608082019050614cd75f830187613aaa565b614ce46020830186614cb5565b614cf16040830185613aaa565b614cfe6060830184613aaa565b95945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffdfea26469706673582212203325336b06a7954ec3c2000917ebebb9f16d3d09cc2fa3648582ca1c24b171df64736f6c63430008210033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02%W_5`\xE0\x1C\x80c\x7F5\xB5`\x11a\x01.W\x80c\xCA\x15\xC8s\x11a\0\xB6W\x80c\xEB\x85u\xDE\x11a\0zW\x80c\xEB\x85u\xDE\x14a\x05\xCBW\x80c\xEB\xAE5\xE7\x14a\x05\xE7W\x80c\xF2\xFD\xE3\x8B\x14a\x06\x17W\x80c\xF6`<a\x14a\x063W\x80c\xFCx\xB2\xE8\x14a\x06OWa\x02%V[\x80c\xCA\x15\xC8s\x14a\x059W\x80c\xCB\x9CL\xC4\x14a\x05iW\x80c\xD5Gt\x1F\x14a\x05sW\x80c\xD8'\r\xCE\x14a\x05\x8FW\x80c\xEA\xE6\xF6R\x14a\x05\xADWa\x02%V[\x80c\xA2\x17\xFD\xDF\x11a\0\xFDW\x80c\xA2\x17\xFD\xDF\x14a\x04\xBBW\x80c\xA3$j\xD3\x14a\x04\xD9W\x80c\xAF o(\x14a\x05\tW\x80c\xBBQ\xFE\xF0\x14a\x05%W\x80c\xC0y\xF4\x95\x14a\x05/Wa\x02%V[\x80c\x7F5\xB5`\x14a\x04\x1FW\x80c\x8D\xA5\xCB[\x14a\x04=W\x80c\x90\x10\xD0|\x14a\x04[W\x80c\x91\xD1HT\x14a\x04\x8BWa\x02%V[\x80c$\x8A\x9C\xA3\x11a\x01\xB1W\x80c6V\x8A\xBE\x11a\x01\x80W\x80c6V\x8A\xBE\x14a\x03\xC9W\x80c;C8\xD1\x14a\x03\xE5W\x80cK\x8Ed\x88\x14a\x04\x01W\x80cK\xB2x\xF3\x14a\x04\x0BW\x80cqP\x18\xA6\x14a\x04\x15Wa\x02%V[\x80c$\x8A\x9C\xA3\x14a\x03UW\x80c//\xF1]\x14a\x03\x85W\x80c0\x10L>\x14a\x03\xA1W\x80c3\xCC\x9A\t\x14a\x03\xBFWa\x02%V[\x80c\x14l\xA51\x11a\x01\xF8W\x80c\x14l\xA51\x14a\x02\xC1W\x80c\x17cE\x14\x14a\x02\xDFW\x80c\x1CtS\xDB\x14a\x02\xFDW\x80c\x1E\xE4\xEE\x0F\x14a\x03\x1BW\x80c#(\xBD\x12\x14a\x037Wa\x02%V[\x80c\x01\xFF\xC9\xA7\x14a\x02)W\x80c\x0B\xDA\x81\xCF\x14a\x02YW\x80c\rB\xEBo\x14a\x02uW\x80c\x13\xFFm\xD5\x14a\x02\x91W[__\xFD[a\x02C`\x04\x806\x03\x81\x01\x90a\x02>\x91\x90a7;V[a\x06\x7FV[`@Qa\x02P\x91\x90a7\x80V[`@Q\x80\x91\x03\x90\xF3[a\x02s`\x04\x806\x03\x81\x01\x90a\x02n\x91\x90a7\xCCV[a\x06\xF8V[\0[a\x02\x8F`\x04\x806\x03\x81\x01\x90a\x02\x8A\x91\x90a8kV[a\tJV[\0[a\x02\xAB`\x04\x806\x03\x81\x01\x90a\x02\xA6\x91\x90a9\x10V[a\t\x8BV[`@Qa\x02\xB8\x91\x90a7\x80V[`@Q\x80\x91\x03\x90\xF3[a\x02\xC9a\n\x07V[`@Qa\x02\xD6\x91\x90a9\xAEV[`@Q\x80\x91\x03\x90\xF3[a\x02\xE7a\n\x19V[`@Qa\x02\xF4\x91\x90a9\xD6V[`@Q\x80\x91\x03\x90\xF3[a\x03\x05a\n\x1FV[`@Qa\x03\x12\x91\x90a9\xD6V[`@Q\x80\x91\x03\x90\xF3[a\x035`\x04\x806\x03\x81\x01\x90a\x030\x91\x90a9\xEFV[a\n%V[\0[a\x03?a\x0F\xD3V[`@Qa\x03L\x91\x90a9\xD6V[`@Q\x80\x91\x03\x90\xF3[a\x03o`\x04\x806\x03\x81\x01\x90a\x03j\x91\x90a:\x7FV[a\x0F\xE9V[`@Qa\x03|\x91\x90a:\xB9V[`@Q\x80\x91\x03\x90\xF3[a\x03\x9F`\x04\x806\x03\x81\x01\x90a\x03\x9A\x91\x90a:\xD2V[a\x10\x05V[\0[a\x03\xA9a\x10?V[`@Qa\x03\xB6\x91\x90a:\xB9V[`@Q\x80\x91\x03\x90\xF3[a\x03\xC7a\x10cV[\0[a\x03\xE3`\x04\x806\x03\x81\x01\x90a\x03\xDE\x91\x90a:\xD2V[a\x10\xDDV[\0[a\x03\xFF`\x04\x806\x03\x81\x01\x90a\x03\xFA\x91\x90a7\xCCV[a\x11\xA4V[\0[a\x04\ta\x11\xDDV[\0[a\x04\x13a\x12WV[\0[a\x04\x1Da\x12\xD1V[\0[a\x04'a\x12\xE4V[`@Qa\x044\x91\x90a:\xB9V[`@Q\x80\x91\x03\x90\xF3[a\x04Ea\x13\x08V[`@Qa\x04R\x91\x90a;\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x04u`\x04\x806\x03\x81\x01\x90a\x04p\x91\x90a;8V[a\x130V[`@Qa\x04\x82\x91\x90a;\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x04\xA5`\x04\x806\x03\x81\x01\x90a\x04\xA0\x91\x90a:\xD2V[a\x13\\V[`@Qa\x04\xB2\x91\x90a7\x80V[`@Q\x80\x91\x03\x90\xF3[a\x04\xC3a\x13\xBFV[`@Qa\x04\xD0\x91\x90a:\xB9V[`@Q\x80\x91\x03\x90\xF3[a\x04\xF3`\x04\x806\x03\x81\x01\x90a\x04\xEE\x91\x90a:\x7FV[a\x13\xC5V[`@Qa\x05\0\x91\x90a<-V[`@Q\x80\x91\x03\x90\xF3[a\x05#`\x04\x806\x03\x81\x01\x90a\x05\x1E\x91\x90a=\x95V[a\x13\xE7V[\0[a\x05-a\x14 V[\0[a\x057a\x14\x9AV[\0[a\x05S`\x04\x806\x03\x81\x01\x90a\x05N\x91\x90a:\x7FV[a\x15\x14V[`@Qa\x05`\x91\x90a9\xD6V[`@Q\x80\x91\x03\x90\xF3[a\x05qa\x155V[\0[a\x05\x8D`\x04\x806\x03\x81\x01\x90a\x05\x88\x91\x90a:\xD2V[a\x15\xAEV[\0[a\x05\x97a\x16\xA1V[`@Qa\x05\xA4\x91\x90a9\xD6V[`@Q\x80\x91\x03\x90\xF3[a\x05\xB5a\x16\xA7V[`@Qa\x05\xC2\x91\x90a:\xB9V[`@Q\x80\x91\x03\x90\xF3[a\x05\xE5`\x04\x806\x03\x81\x01\x90a\x05\xE0\x91\x90a9\xEFV[a\x16\xCBV[\0[a\x06\x01`\x04\x806\x03\x81\x01\x90a\x05\xFC\x91\x90a=\xEFV[a\x1C\x10V[`@Qa\x06\x0E\x91\x90a>\xD1V[`@Q\x80\x91\x03\x90\xF3[a\x061`\x04\x806\x03\x81\x01\x90a\x06,\x91\x90a9\x10V[a \xDCV[\0[a\x06M`\x04\x806\x03\x81\x01\x90a\x06H\x91\x90a>\xF1V[a!`V[\0[a\x06i`\x04\x806\x03\x81\x01\x90a\x06d\x91\x90a9\x10V[a!\xC9V[`@Qa\x06v\x91\x90a7\x80V[`@Q\x80\x91\x03\x90\xF3[_\x7F\x07\xEF\xFE\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x06\xF1WPa\x06\xF0\x82a!\xFBV[[\x90P\x91\x90PV[\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2a\x07\"\x81a\"tV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x05_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07\xC4W3\x82`@Q\x7F\xFF\xAB\xBA\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\xBB\x92\x91\x90a?qV[`@Q\x80\x91\x03\x90\xFD[_\x83\x03a\x08\x08W3`@Q\x7F\x16\x92<\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\xFF\x91\x90a;\x1FV[`@Q\x80\x91\x03\x90\xFD[_`\x0C_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x14a\x08\x8CW3`@Q\x7FO_\xBF\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\x83\x91\x90a;\x1FV[`@Q\x80\x91\x03\x90\xFD[`@Q\x80`@\x01`@R\x80\x83\x81R` \x01\x84\x81RP`\x0C_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01U` \x82\x01Q\x81`\x01\x01U\x90PP\x7F\xB8\x9A\xDD\xD97\xF4O\x90,\x84\x95\x96d\x187\xCDz\xF2\xFC\xEC\xEF\"\xD2\xA7\x86o\xDC\x1A\xD9\xC0\xAE.3\x84\x84`@Qa\t&\x93\x92\x91\x90a?\x98V[`@Q\x80\x91\x03\x90\xA1`\t_\x81T\x80\x92\x91\x90a\t@\x90a?\xFAV[\x91\x90PUPPPPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\tt\x81a\"tV[\x82\x82`\x04\x91\x82a\t\x85\x92\x91\x90aBYV[PPPPV[_a\t\x95\x82a!\xC9V[a\t\xD6W\x81`@Q\x7F\xAB\xDC\xE0j\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xCD\x91\x90a;\x1FV[`@Q\x80\x91\x03\x90\xFD[a\n\0\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x83a\x13\\V[\x90P\x91\x90PV[`\x11_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\x10T\x81V[`\nT\x81V[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\nO\x81a\"tV[a\ny\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2\x85a\x13\\V[a\n\xBAW\x83`@Q\x7F\xA02\xACk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\xB1\x91\x90a;\x1FV[`@Q\x80\x91\x03\x90\xFD[_`\x07T\x90P__\x90P[`\x07T\x81\x10\x15a\x0BGW\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x05_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0B:W\x80\x91Pa\x0BGV[\x80\x80`\x01\x01\x91PPa\n\xC5V[P`\x07T\x81\x03a\x0B\xBEW`\x05__\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x7Fo\xAF\x9F\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\xB5\x91\x90a;\x1FV[`@Q\x80\x91\x03\x90\xFD[_\x81`\nTa\x0B\xCD\x91\x90aC&V[\x90P_a\x0B\xFF\x82`@Q` \x01a\x0B\xE4\x91\x90a9\xD6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\"\x88V[\x90P_a\x0CO\x82\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa\"\xBBV[\x90P\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0C\xEDW`\x06_\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01`\x02\x81\x10a\x0C\xD4Wa\x0C\xD3aCYV[[\x01_\x81Ta\x0C\xE1\x90a?\xFAV[\x91\x90P\x81\x90UPa\rQV[`\x06_\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _`\x02\x81\x10a\r<Wa\r;aCYV[[\x01_\x81Ta\rI\x90a?\xFAV[\x91\x90P\x81\x90UP[`\x01`\x0BTa\r`\x91\x90aC&V[`\x06_\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _`\x02\x81\x10a\r\xAFWa\r\xAEaCYV[[\x01T\x10\x80a\x0E\x1BWP`\x01`\x0BTa\r\xC7\x91\x90aC&V[`\x06_\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01`\x02\x81\x10a\x0E\x17Wa\x0E\x16aCYV[[\x01T\x10[a\x0EZW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0EQ\x90aD\x06V[`@Q\x80\x91\x03\x90\xFD[`\x01`\x0BTa\x0Ei\x91\x90aC&V[`\x06_\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _`\x02\x81\x10a\x0E\xB8Wa\x0E\xB7aCYV[[\x01T\x10a\x0F\x12W\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F@p\t \x05 \xF9\xF1\x05\x84\x81<\x0B\x95D\x1A\xB3 \xF6\xB0\x8D\x97\xEB\xDA\xAF\x1E\x82N\xED\xD9\xD7\xC3_`@Qa\x0F\x05\x91\x90a7\x80V[`@Q\x80\x91\x03\x90\xA2a\x0F\xC9V[`\x01`\x0BTa\x0F!\x91\x90aC&V[`\x06_\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01`\x02\x81\x10a\x0FqWa\x0FpaCYV[[\x01T\x10a\x0F\xC8W\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F@p\t \x05 \xF9\xF1\x05\x84\x81<\x0B\x95D\x1A\xB3 \xF6\xB0\x8D\x97\xEB\xDA\xAF\x1E\x82N\xED\xD9\xD7\xC3`\x01`@Qa\x0F\xBF\x91\x90a7\x80V[`@Q\x80\x91\x03\x90\xA2[[PPPPPPPPV[_`\x08T`\x07Ta\x0F\xE4\x91\x90aD$V[\x90P\x90V[___\x83\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x10/\x81a\"tV[a\x109\x83\x83a\"\xE5V[PPPPV[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x81V[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x10\x8D\x81a\"tV[`\x03a\x10\x98\x81a\"\xF8V[\x7F \xF5^\xD0\xC9/+\xB1\xC8\x82T\x88\xE1\xE3\xC9\x84c\xD0$\xB2\xA4-\xBD$\x83\x8C?u&\x0FC\xE93B`@Qa\x10\xC9\x92\x91\x90a?qV[`@Q\x80\x91\x03\x90\xA1a\x10\xD9a#\x81V[PPV[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x82\x14\x80\x15a\x11\x12WPa\x11\x11\x82\x82a\x13\\V[[\x15a\x11\x96W_a\x11!\x83a\x15\x14V[\x90P_`\x01`\x02T`\x03a\x115\x91\x90aDWV[a\x11?\x91\x90aC&V[\x90P\x80\x82\x03a\x11\x93W`\x01\x81a\x11U\x91\x90aD$V[\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\x8A\x92\x91\x90aD\x98V[`@Q\x80\x91\x03\x90\xFD[PP[a\x11\xA0\x82\x82a#\xEAV[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x11\xCE\x81a\"tV[a\x11\xD8\x83\x83a$eV[PPPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x12\x07\x81a\"tV[`\x04a\x12\x12\x81a\"\xF8V[\x7F0\x1F\x8A7\x01\xF5\xB2`\x19s\x82\xDDs\x01\x07\x85B\x14O\xE8\xFD\xDD\x18\x08=on\t\xE4\x95\x8AY3B`@Qa\x12C\x92\x91\x90a?qV[`@Q\x80\x91\x03\x90\xA1a\x12Sa#\x81V[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x12\x81\x81a\"tV[`\x05a\x12\x8C\x81a\"\xF8V[\x7F$\xA8se\x1D&\xFBZF,\xB3z\x91\x07\x1C\xDDM\t\xAB\xEE\xBF\xE0\xED\x14C)\xBE\xD1\xCC5\x9D\x033B`@Qa\x12\xBD\x92\x91\x90a?qV[`@Q\x80\x91\x03\x90\xA1a\x12\xCDa#\x81V[PPV[a\x12\xD9a'\xEDV[a\x12\xE2_a(tV[V[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x81V[_`\r_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_a\x13T\x82`\x01_\x86\x81R` \x01\x90\x81R` \x01_ a)7\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[___\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[__\x1B\x81V[``a\x13\xE0`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a)NV[\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x14\x11\x81a\"tV[a\x14\x1B\x83\x83a)mV[PPPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x14J\x81a\"tV[`\x02a\x14U\x81a\"\xF8V[\x7F`\xED\xF9\xBD\xC7\xC4\xEA\0|\xAE\x1A\x9B\xBD\x03\xE4\x1E[\xFC\xCDr1\xA6\xEC8<.\xDDx\0\xF0\xD2\x0C3B`@Qa\x14\x86\x92\x91\x90a?qV[`@Q\x80\x91\x03\x90\xA1a\x14\x96a#\x81V[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x14\xC4\x81a\"tV[`\x01a\x14\xCF\x81a\"\xF8V[\x7Fg\xC4H\x9FgM\x03\xC7\xD1\x9A\x9E6sQ\x88\xDE|e\xE8\xD1\xE9\x9E\xB3\xA2\xFD%\x8Av\x9E\xB1O\xFF3B`@Qa\x15\0\x92\x91\x90a?qV[`@Q\x80\x91\x03\x90\xA1a\x15\x10a#\x81V[PPV[_a\x15.`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a,\xEDV[\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x15_\x81a\"tV[_a\x15i\x81a\"\xF8V[\x7F\xBBp\x9D\xEAtO\x06\xD1\xB2n\x82M\xEE\xC2\xF7\x14\x0CQ\x12f\xEE\x15\xD7\xA2\x17\x83\x8B1\xD8\xB0\x12=3B`@Qa\x15\x9A\x92\x91\x90a?qV[`@Q\x80\x91\x03\x90\xA1a\x15\xAAa#\x81V[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x15\xD8\x81a\"tV[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x83\x14\x80\x15a\x16\rWPa\x16\x0C\x83\x83a\x13\\V[[\x15a\x16\x91W_a\x16\x1C\x84a\x15\x14V[\x90P_`\x01`\x02T`\x03a\x160\x91\x90aDWV[a\x16:\x91\x90aC&V[\x90P\x80\x82\x03a\x16\x8EW`\x01\x81a\x16P\x91\x90aD$V[\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16\x85\x92\x91\x90aD\x98V[`@Q\x80\x91\x03\x90\xFD[PP[a\x16\x9B\x83\x83a-\0V[PPPPV[`\x0FT\x81V[\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2\x81V[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x16\xF5\x81a\"tV[a\x17\x1F\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2\x85a\x13\\V[a\x17`W\x83`@Q\x7F\xA02\xACk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17W\x91\x90a;\x1FV[`@Q\x80\x91\x03\x90\xFD[_`\x03_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x90P`\x03_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x02\x01_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x18pW\x843`@Q\x7F\x08\xE5T\x95\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18g\x92\x91\x90aD\xBFV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x0BT`\x03a\x18\x81\x91\x90aDWV[a\x18\x8B\x91\x90aC&V[\x81\x10a\x18\xCCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\xC3\x90aEVV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x03_\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x02\x01_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x83\x83`\x03_\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x01\x83\x81T\x81\x10a\x19\xB2Wa\x19\xB1aCYV[[\x90_R` _ \x01\x91\x82a\x19\xC7\x92\x91\x90aBYV[P`\x01`\x03_\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x01_\x82\x82Ta\x1A\x18\x91\x90aC&V[\x92PP\x81\x90UP`\x01\x81a\x1A,\x91\x90aC&V[\x90P`\x01`\x02T`\x02a\x1A?\x91\x90aDWV[a\x1AI\x91\x90aC&V[\x81\x10a\x1C\tW_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1AjWa\x1Aia<]V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\x9DW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1A\x88W\x90P[P\x90P__\x90P[\x82\x81\x10\x15a\x1B\xB8W`\x03_\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x01\x81\x81T\x81\x10a\x1A\xFEWa\x1A\xFDaCYV[[\x90_R` _ \x01\x80Ta\x1B\x11\x90a@xV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1B=\x90a@xV[\x80\x15a\x1B\x88W\x80`\x1F\x10a\x1B_Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1B\x88V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1BkW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x82\x82\x81Q\x81\x10a\x1B\xA0Wa\x1B\x9FaCYV[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\x1A\xA5V[P\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F#\xE9\x1D\xBF\xAE\x03u\x8C\xB8\x8D\x7FbR\xB5q\n\xFAS\xA1\x9F\xFE\x0FKOu\xD7\xF2\xDE\x0C^\xAB\xE9\x82`@Qa\x1B\xFF\x91\x90aF\x8FV[`@Q\x80\x91\x03\x90\xA2P[PPPPPV[```\x01\x82\x14a\x1CUW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1CL\x90aG\x1FV[`@Q\x80\x91\x03\x90\xFD[_\x82\x03a\x1C\x99W3`@Q\x7F\xB2\xFDU\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C\x90\x91\x90a;\x1FV[`@Q\x80\x91\x03\x90\xFD[_`\x08T`\x07Ta\x1C\xAA\x91\x90aD$V[\x90P\x80\x83\x11\x15a\x1C\xF3W\x82\x81`@Q\x7F\xDF=u\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C\xEA\x92\x91\x90aD\x98V[`@Q\x80\x91\x03\x90\xFD[__\x90P[`\x07T\x81\x10\x15a\x1D\xAFW3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x05_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x1D\xA2W3`@Q\x7F\xAC\xA9/\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1D\x99\x91\x90a;\x1FV[`@Q\x80\x91\x03\x90\xFD[\x80\x80`\x01\x01\x91PPa\x1C\xF8V[P_`\x01\x84`\x08Ta\x1D\xC1\x91\x90aC&V[a\x1D\xCB\x91\x90aD$V[\x90P_\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\xE8Wa\x1D\xE7a<]V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\x16W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_`\x08T\x90P[\x82`\x08T\x11a\x1E\xC5W3`\x05_`\x08T\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x08T\x82\x82`\x08Ta\x1E\x8C\x91\x90aD$V[\x81Q\x81\x10a\x1E\x9DWa\x1E\x9CaCYV[[` \x02` \x01\x01\x81\x81RPP`\x08_\x81T\x80\x92\x91\x90a\x1E\xBB\x90a?\xFAV[\x91\x90PUPa\x1E V[a\x1E\xEF\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R23a\"\xE5V[P_`\x03_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x90P`\x01`\x0BT`\x03a\x1FA\x91\x90aDWV[a\x1FK\x91\x90aC&V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1FdWa\x1Fca<]V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1F\x97W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1F\x82W\x90P[P\x81_\x01\x90\x81a\x1F\xA7\x91\x90aK,V[P_\x81`\x01\x01\x81\x90UP_a\x1F\xDB\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x13\xC5V[\x90P_a \x07\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x15\x14V[\x90P__\x90P[\x81\x81\x10\x15a \x93W_\x84`\x02\x01_\x85\x84\x81Q\x81\x10a /Wa .aCYV[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80\x80`\x01\x01\x91PPa \x0EV[P\x7F\x1E\x14\xAB\xE5\xD0\xCD\xB9j\xDD\xE7\xB9\xEC\xA9\xB1K\xC0\x8D\xF6#\xB5\x80Z\xFD\xE5\xA3\xF0\xAC\xAD\xC2\xBFO[3\x86`@Qa \xC5\x92\x91\x90aK\x8EV[`@Q\x80\x91\x03\x90\xA1\x84\x97PPPPPPPP\x91\x90PV[a \xE4a'\xEDV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a!TW_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a!K\x91\x90a;\x1FV[`@Q\x80\x91\x03\x90\xFD[a!]\x81a(tV[PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa!\x8A\x81a\"tV[a!\x94\x84\x84a)mV[a!\x9E\x82\x85a$eV[a!\xC2\x85\x84_\x81Q\x81\x10a!\xB5Wa!\xB4aCYV[[` \x02` \x01\x01Qa-\x13V[PPPPPV[_a!\xF4\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x83a\x13\\V[\x90P\x91\x90PV[_\x7FZ\x05\x18\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\"mWPa\"l\x82a-\x96V[[\x90P\x91\x90PV[a\"\x85\x81a\"\x80a.\x0FV[a.\x16V[PV[_\x7F\x19Ethereum Signed Message:\n32\0\0\0\0_R\x81`\x1CR`<_ \x90P\x91\x90PV[____a\"\xC9\x86\x86a.gV[\x92P\x92P\x92Pa\"\xD9\x82\x82a.\xBCV[\x82\x93PPPP\x92\x91PPV[_a\"\xF0\x83\x83a0\x1EV[\x90P\x92\x91PPV[\x80`\x06\x81\x11\x15a#\x0BWa#\na9;V[[`\x11_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x06\x81\x11\x15a#,Wa#+a9;V[[\x14a#~W\x80`\x11_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x7F\xBF\xA2\x17\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a#u\x92\x91\x90aK\xBCV[`@Q\x80\x91\x03\x90\xFD[PV[`\x01`\x11_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x06\x81\x11\x15a#\xA4Wa#\xA3a9;V[[a#\xAE\x91\x90aC&V[`\x06\x81\x11\x15a#\xC0Wa#\xBFa9;V[[`\x11_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x06\x81\x11\x15a#\xE3Wa#\xE2a9;V[[\x02\x17\x90UPV[a#\xF2a.\x0FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a$VW`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a$`\x82\x82a-\0V[PPPV[`\x07T`\n_\x82\x82Ta$x\x91\x90aC&V[\x92PP\x81\x90UP\x81`\x07\x81\x90UP_`\x08\x81\x90UP_`\t\x81\x90UP\x80`\x0B\x81\x90UP_a$\xC5\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x13\xC5V[\x90P_a$\xF1\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x15\x14V[\x90P__\x90P[`\x07T\x81\x10\x15a'\xABW`\x0C_`\x05_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ __\x82\x01_\x90U`\x01\x82\x01_\x90UPP`\x06_`\x05_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a%\xF7\x91\x90a62V[__\x90P[\x82\x81\x10\x15a&\xE5W`\x03_`\x05_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x02\x01_\x85\x83\x81Q\x81\x10a&\x88Wa&\x87aCYV[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U\x80\x80`\x01\x01\x91PPa%\xFCV[P`\x03_`\x05_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ __\x82\x01_a'b\x91\x90a6>V[`\x01\x82\x01_\x90UPP`\x05_\x82\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U\x80\x80`\x01\x01\x91PPa$\xF8V[P\x7F\xF7\xF0\x87#\x82\xDF\xF5\xE6\x98\xB2\x84\xE1 \x84\xE4\xE7\x89O\x83\x02\x16\xDD\x80\xCBN\x90\x9BY:X\xF95`\x07T3`@Qa'\xDF\x92\x91\x90aK\xE3V[`@Q\x80\x91\x03\x90\xA1PPPPV[a'\xF5a.\x0FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a(\x13a\x13\x08V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a(rWa(6a.\x0FV[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a(i\x91\x90a;\x1FV[`@Q\x80\x91\x03\x90\xFD[V[_`\r_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\r_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[_a)D\x83_\x01\x83a0aV[_\x1C\x90P\x92\x91PPV[``_a)\\\x83_\x01a0\x88V[\x90P``\x81\x90P\x80\x92PPP\x91\x90PV[_`\x01\x83`\x03a)}\x91\x90aDWV[a)\x87\x91\x90aC&V[\x90P\x80\x82Q\x10\x15a)\xD2W\x81Q\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a)\xC9\x92\x91\x90aD\x98V[`@Q\x80\x91\x03\x90\xFD[\x82`\x02\x81\x90UP_a*\x03\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x13\xC5V[\x90P_a*/\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x15\x14V[\x90P__\x90P[\x81\x81\x10\x15a*\x90Wa*\x82\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x84\x83\x81Q\x81\x10a*uWa*taCYV[[` \x02` \x01\x01Qa-\0V[P\x80\x80`\x01\x01\x91PPa*6V[P_a*\xBB\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x13\xC5V[\x90P_a*\xE7\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x15\x14V[\x90P__\x90P[\x81\x81\x10\x15a+HWa+:\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x84\x83\x81Q\x81\x10a+-Wa+,aCYV[[` \x02` \x01\x01Qa-\0V[P\x80\x80`\x01\x01\x91PPa*\xEEV[P_a+s\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2a\x13\xC5V[\x90P_a+\x9F\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2a\x15\x14V[\x90P__\x90P[\x81\x81\x10\x15a,\0Wa+\xF2\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2\x84\x83\x81Q\x81\x10a+\xE5Wa+\xE4aCYV[[` \x02` \x01\x01Qa-\0V[P\x80\x80`\x01\x01\x91PPa+\xA6V[P__\x90P[\x88Q\x81\x10\x15a,aWa,S\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x8A\x83\x81Q\x81\x10a,FWa,EaCYV[[` \x02` \x01\x01Qa\"\xE5V[P\x80\x80`\x01\x01\x91PPa,\x06V[Pa,\xA6\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x89_\x81Q\x81\x10a,\x99Wa,\x98aCYV[[` \x02` \x01\x01Qa\"\xE5V[P\x7F\xA2\xDFx0\xE0\xBE\xDE\xF7\xB1\x11k\xF5G\xB4g\xB1kP\xB3\xBD#\x14l\x9E\t\x98x\xD1N\x890\x1A\x87\x8A3`@Qa,\xDA\x93\x92\x91\x90aL\nV[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPV[_a,\xF9\x82_\x01a0\xE1V[\x90P\x91\x90PV[_a-\x0B\x83\x83a0\xF0V[\x90P\x92\x91PPV[\x81`\x0E\x81\x90UPB`\x0F\x81\x90UPC`\x10\x81\x90UP_`\x11_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x06\x81\x11\x15a-LWa-Ka9;V[[\x02\x17\x90UP\x7F\xDE\xF1\xF0\x8E\xB6U\xF4\xA7_`\xBDo\xD7\xE9q\x11 \x04\xAB\xD8F\xA6\x12\xE4m\xAB'\x07p\xD2L\xA50`\x0FT`\x10T\x84`@Qa-\x8A\x94\x93\x92\x91\x90aL?V[`@Q\x80\x91\x03\x90\xA1PPV[_\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a.\x08WPa.\x07\x82a13V[[\x90P\x91\x90PV[_3\x90P\x90V[a. \x82\x82a\x13\\V[a.cW\x80\x82`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.Z\x92\x91\x90aL\x82V[`@Q\x80\x91\x03\x90\xFD[PPV[___`A\x84Q\x03a.\xA7W___` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90Pa.\x99\x88\x82\x85\x85a1\x9CV[\x95P\x95P\x95PPPPa.\xB5V[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15a.\xCFWa.\xCEa9;V[[\x82`\x03\x81\x11\x15a.\xE2Wa.\xE1a9;V[[\x03\x15a0\x1AW`\x01`\x03\x81\x11\x15a.\xFCWa.\xFBa9;V[[\x82`\x03\x81\x11\x15a/\x0FWa/\x0Ea9;V[[\x03a/FW`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15a/ZWa/Ya9;V[[\x82`\x03\x81\x11\x15a/mWa/la9;V[[\x03a/\xB1W\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a/\xA8\x91\x90a9\xD6V[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15a/\xC4Wa/\xC3a9;V[[\x82`\x03\x81\x11\x15a/\xD7Wa/\xD6a9;V[[\x03a0\x19W\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a0\x10\x91\x90a:\xB9V[`@Q\x80\x91\x03\x90\xFD[[PPV[__a0*\x84\x84a2\x83V[\x90P\x80\x15a0WWa0U\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a3l\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[_\x82_\x01\x82\x81T\x81\x10a0wWa0vaCYV[[\x90_R` _ \x01T\x90P\x92\x91PPV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a0\xD5W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a0\xC1W[PPPPP\x90P\x91\x90PV[_\x81_\x01\x80T\x90P\x90P\x91\x90PV[__a0\xFC\x84\x84a3\x99V[\x90P\x80\x15a1)Wa1'\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a4\x82\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[_\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[___\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15a1\xD8W_`\x03\x85\x92P\x92P\x92Pa2yV[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@Qa1\xFB\x94\x93\x92\x91\x90aL\xC4V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a2\x1BW=__>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a2lW_`\x01__\x1B\x93P\x93P\x93PPa2yV[\x80___\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[_a2\x8E\x83\x83a\x13\\V[a3bW`\x01__\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa2\xFFa.\x0FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa3fV[_\x90P[\x92\x91PPV[_a3\x91\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba4\xAFV[\x90P\x92\x91PPV[_a3\xA4\x83\x83a\x13\\V[\x15a4xW___\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa4\x15a.\x0FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa4|V[_\x90P[\x92\x91PPV[_a4\xA7\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba5\x16V[\x90P\x92\x91PPV[_a4\xBA\x83\x83a6\x12V[a5\x0CW\x82_\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x82_\x01\x80T\x90P\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP`\x01\x90Pa5\x10V[_\x90P[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x14a6\x07W_`\x01\x82a5C\x91\x90aD$V[\x90P_`\x01\x86_\x01\x80T\x90Pa5Y\x91\x90aD$V[\x90P\x80\x82\x14a5\xBFW_\x86_\x01\x82\x81T\x81\x10a5xWa5waCYV[[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a5\x99Wa5\x98aCYV[[\x90_R` _ \x01\x81\x90UP\x83\x87`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UPP[\x85_\x01\x80T\x80a5\xD2Wa5\xD1aM\x07V[[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa6\x0CV[_\x91PP[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x14\x15\x90P\x92\x91PPV[P_\x81U`\x01\x01_\x90UV[P\x80T_\x82U\x90_R` _ \x90a6V\x91\x90a6YV[PV[_[\x80\x82\x11\x15a6yW\x82\x81\x01_a6q\x91\x90a6~V[`\x01\x01a6[V[PP\x90V[P\x80Ta6\x8A\x90a@xV[_\x82U\x80`\x1F\x10a6\x9BWPa6\xB5V[`\x1F\x01` \x90\x04\x90_R` _ \x90a6\xB4\x91\x90a6\xB8V[[PV[_[\x80\x82\x11\x15a6\xD0W\x82\x81\x01_\x90U`\x01\x01a6\xBAV[PP\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a7\x1A\x81a6\xE6V[\x81\x14a7$W__\xFD[PV[_\x815\x90Pa75\x81a7\x11V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a7PWa7Oa6\xDEV[[_a7]\x84\x82\x85\x01a7'V[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a7z\x81a7fV[\x82RPPV[_` \x82\x01\x90Pa7\x93_\x83\x01\x84a7qV[\x92\x91PPV[_\x81\x90P\x91\x90PV[a7\xAB\x81a7\x99V[\x81\x14a7\xB5W__\xFD[PV[_\x815\x90Pa7\xC6\x81a7\xA2V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a7\xE2Wa7\xE1a6\xDEV[[_a7\xEF\x85\x82\x86\x01a7\xB8V[\x92PP` a8\0\x85\x82\x86\x01a7\xB8V[\x91PP\x92P\x92\x90PV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a8+Wa8*a8\nV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8HWa8Ga8\x0EV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a8dWa8ca8\x12V[[\x92P\x92\x90PV[__` \x83\x85\x03\x12\x15a8\x81Wa8\x80a6\xDEV[[_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8\x9EWa8\x9Da6\xE2V[[a8\xAA\x85\x82\x86\x01a8\x16V[\x92P\x92PP\x92P\x92\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a8\xDF\x82a8\xB6V[\x90P\x91\x90PV[a8\xEF\x81a8\xD5V[\x81\x14a8\xF9W__\xFD[PV[_\x815\x90Pa9\n\x81a8\xE6V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a9%Wa9$a6\xDEV[[_a92\x84\x82\x85\x01a8\xFCV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x07\x81\x10a9yWa9xa9;V[[PV[_\x81\x90Pa9\x89\x82a9hV[\x91\x90PV[_a9\x98\x82a9|V[\x90P\x91\x90PV[a9\xA8\x81a9\x8EV[\x82RPPV[_` \x82\x01\x90Pa9\xC1_\x83\x01\x84a9\x9FV[\x92\x91PPV[a9\xD0\x81a7\x99V[\x82RPPV[_` \x82\x01\x90Pa9\xE9_\x83\x01\x84a9\xC7V[\x92\x91PPV[___`@\x84\x86\x03\x12\x15a:\x06Wa:\x05a6\xDEV[[_a:\x13\x86\x82\x87\x01a8\xFCV[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:4Wa:3a6\xE2V[[a:@\x86\x82\x87\x01a8\x16V[\x92P\x92PP\x92P\x92P\x92V[_\x81\x90P\x91\x90PV[a:^\x81a:LV[\x81\x14a:hW__\xFD[PV[_\x815\x90Pa:y\x81a:UV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a:\x94Wa:\x93a6\xDEV[[_a:\xA1\x84\x82\x85\x01a:kV[\x91PP\x92\x91PPV[a:\xB3\x81a:LV[\x82RPPV[_` \x82\x01\x90Pa:\xCC_\x83\x01\x84a:\xAAV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a:\xE8Wa:\xE7a6\xDEV[[_a:\xF5\x85\x82\x86\x01a:kV[\x92PP` a;\x06\x85\x82\x86\x01a8\xFCV[\x91PP\x92P\x92\x90PV[a;\x19\x81a8\xD5V[\x82RPPV[_` \x82\x01\x90Pa;2_\x83\x01\x84a;\x10V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a;NWa;Ma6\xDEV[[_a;[\x85\x82\x86\x01a:kV[\x92PP` a;l\x85\x82\x86\x01a7\xB8V[\x91PP\x92P\x92\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a;\xA8\x81a8\xD5V[\x82RPPV[_a;\xB9\x83\x83a;\x9FV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a;\xDB\x82a;vV[a;\xE5\x81\x85a;\x80V[\x93Pa;\xF0\x83a;\x90V[\x80_[\x83\x81\x10\x15a< W\x81Qa<\x07\x88\x82a;\xAEV[\x97Pa<\x12\x83a;\xC5V[\x92PP`\x01\x81\x01\x90Pa;\xF3V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra<E\x81\x84a;\xD1V[\x90P\x92\x91PPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a<\x93\x82a<MV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a<\xB2Wa<\xB1a<]V[[\x80`@RPPPV[_a<\xC4a6\xD5V[\x90Pa<\xD0\x82\x82a<\x8AV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a<\xEFWa<\xEEa<]V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a=\x12a=\r\x84a<\xD5V[a<\xBBV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a=5Wa=4a8\x12V[[\x83[\x81\x81\x10\x15a=^W\x80a=J\x88\x82a8\xFCV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa=7V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a=|Wa={a8\nV[[\x815a=\x8C\x84\x82` \x86\x01a=\0V[\x91PP\x92\x91PPV[__`@\x83\x85\x03\x12\x15a=\xABWa=\xAAa6\xDEV[[_a=\xB8\x85\x82\x86\x01a7\xB8V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=\xD9Wa=\xD8a6\xE2V[[a=\xE5\x85\x82\x86\x01a=hV[\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a>\x04Wa>\x03a6\xDEV[[_a>\x11\x84\x82\x85\x01a7\xB8V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a>L\x81a7\x99V[\x82RPPV[_a>]\x83\x83a>CV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a>\x7F\x82a>\x1AV[a>\x89\x81\x85a>$V[\x93Pa>\x94\x83a>4V[\x80_[\x83\x81\x10\x15a>\xC4W\x81Qa>\xAB\x88\x82a>RV[\x97Pa>\xB6\x83a>iV[\x92PP`\x01\x81\x01\x90Pa>\x97V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra>\xE9\x81\x84a>uV[\x90P\x92\x91PPV[____`\x80\x85\x87\x03\x12\x15a?\tWa?\x08a6\xDEV[[_a?\x16\x87\x82\x88\x01a:kV[\x94PP` a?'\x87\x82\x88\x01a7\xB8V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a?HWa?Ga6\xE2V[[a?T\x87\x82\x88\x01a=hV[\x92PP``a?e\x87\x82\x88\x01a7\xB8V[\x91PP\x92\x95\x91\x94P\x92PV[_`@\x82\x01\x90Pa?\x84_\x83\x01\x85a;\x10V[a?\x91` \x83\x01\x84a9\xC7V[\x93\x92PPPV[_``\x82\x01\x90Pa?\xAB_\x83\x01\x86a;\x10V[a?\xB8` \x83\x01\x85a9\xC7V[a?\xC5`@\x83\x01\x84a9\xC7V[\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a@\x04\x82a7\x99V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a@6Wa@5a?\xCDV[[`\x01\x82\x01\x90P\x91\x90PV[_\x82\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a@\x8FW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a@\xA2Wa@\xA1a@KV[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02aA\x04\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a@\xC9V[aA\x0E\x86\x83a@\xC9V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_aAIaADaA?\x84a7\x99V[aA&V[a7\x99V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aAb\x83aA/V[aAvaAn\x82aAPV[\x84\x84Ta@\xD5V[\x82UPPPPV[__\x90P\x90V[aA\x8DaA~V[aA\x98\x81\x84\x84aAYV[PPPV[_[\x82\x81\x10\x15aA\xBEWaA\xB3_\x82\x84\x01aA\x85V[`\x01\x81\x01\x90PaA\x9FV[PPPV[`\x1F\x82\x11\x15aB\x11W\x82\x82\x11\x15aB\x10WaA\xDD\x81a@\xA8V[aA\xE6\x83a@\xBAV[aA\xEF\x85a@\xBAV[` \x86\x10\x15aA\xFCW_\x90P[\x80\x83\x01aB\x0B\x82\x84\x03\x82aA\x9DV[PPPP[[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_aB1_\x19\x84`\x08\x02aB\x16V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_aBI\x83\x83aB\"V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[aBc\x83\x83a@AV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aB|WaB{a<]V[[aB\x86\x82Ta@xV[aB\x91\x82\x82\x85aA\xC3V[_`\x1F\x83\x11`\x01\x81\x14aB\xBEW_\x84\x15aB\xACW\x82\x87\x015\x90P[aB\xB6\x85\x82aB>V[\x86UPaC\x1DV[`\x1F\x19\x84\x16aB\xCC\x86a@\xA8V[_[\x82\x81\x10\x15aB\xF3W\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaB\xCEV[\x86\x83\x10\x15aC\x10W\x84\x89\x015aC\x0C`\x1F\x89\x16\x82aB\"V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[_aC0\x82a7\x99V[\x91PaC;\x83a7\x99V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15aCSWaCRa?\xCDV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FBUG: the authentication votes by_\x82\x01R\x7F honest clients are inconsistent` \x82\x01RPV[_aC\xF0`@\x83aC\x86V[\x91PaC\xFB\x82aC\x96V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaD\x1D\x81aC\xE4V[\x90P\x91\x90PV[_aD.\x82a7\x99V[\x91PaD9\x83a7\x99V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15aDQWaDPa?\xCDV[[\x92\x91PPV[_aDa\x82a7\x99V[\x91PaDl\x83a7\x99V[\x92P\x82\x82\x02aDz\x81a7\x99V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17aD\x91WaD\x90a?\xCDV[[P\x92\x91PPV[_`@\x82\x01\x90PaD\xAB_\x83\x01\x85a9\xC7V[aD\xB8` \x83\x01\x84a9\xC7V[\x93\x92PPPV[_`@\x82\x01\x90PaD\xD2_\x83\x01\x85a;\x10V[aD\xDF` \x83\x01\x84a;\x10V[\x93\x92PPPV[\x7FBUG: ALREADY RECEIVED SHARES FRO_\x82\x01R\x7FM N PARTIES, TOO MANY CLIENTS\0\0\0` \x82\x01RPV[_aE@`=\x83aC\x86V[\x91PaEK\x82aD\xE6V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaEm\x81aE4V[\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_aE\xCF\x82aE\x9DV[aE\xD9\x81\x85aE\xA7V[\x93PaE\xE9\x81\x85` \x86\x01aE\xB7V[aE\xF2\x81a<MV[\x84\x01\x91PP\x92\x91PPV[_aF\x08\x83\x83aE\xC5V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aF&\x82aEtV[aF0\x81\x85aE~V[\x93P\x83` \x82\x02\x85\x01aFB\x85aE\x8EV[\x80_[\x85\x81\x10\x15aF}W\x84\x84\x03\x89R\x81QaF^\x85\x82aE\xFDV[\x94PaFi\x83aF\x10V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaFEV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaF\xA7\x81\x84aF\x1CV[\x90P\x92\x91PPV[\x7FCURRENTLY ONLY ONE INDEX PER CLI_\x82\x01R\x7FENT ALLOWED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_aG\t`+\x83aC\x86V[\x91PaG\x14\x82aF\xAFV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaG6\x81aF\xFDV[\x90P\x91\x90PV[_\x81T\x90P\x91\x90PV[_\x81\x90P`\x01\x80`\x01\x03\x83\x01\x04\x90P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[aG\x9D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83` \x03`\x08\x02aB\x16V[\x81T\x81\x16\x82UPPPV[aG\xB1\x81a@\xA8V[aG\xBC\x83\x82TaB>V[\x80\x83U_\x82UPPPPV[` \x84\x10_\x81\x14aH#W`\x1F\x84\x11`\x01\x81\x14aG\xF0WaG\xE9\x86\x85aB>V[\x83UaH\x1DV[aG\xF9\x83a@\xA8V[aH\x11`\x01aH\x07\x88a@\xBAV[\x03`\x01\x83\x01aA\x9DV[aH\x1B\x87\x85aG\xA8V[P[PaH}V[aH,\x85a@\xBAV[aH5\x85a@\xBAV[aH>\x84a@\xA8V[\x82\x81\x01`\x1F\x89\x16\x80\x15aHYWaHX\x81`\x01\x84\x03aGmV[[\x84\x84\x11\x15aHnWaHm\x85\x85\x03\x83aA\x9DV[[`\x01\x8A`\x02\x02\x17\x87UPPPPP[PPPPPV[h\x01\0\0\0\0\0\0\0\0\x84\x11\x15aH\x9EWaH\x9Da<]V[[` \x83\x10_\x81\x14aH\xE7W` \x85\x10_\x81\x14aH\xC5WaH\xBE\x86\x85aB>V[\x83UaH\xE1V[\x83`\xFF\x19\x16\x93P\x83aH\xD6\x84a@\xA8V[U`\x01\x86`\x02\x02\x01\x83U[PaH\xF1V[`\x01\x85`\x02\x02\x01\x82U[PPPPPV[\x80TaI\x03\x81a@xV[\x80\x84\x11\x15aI\x18WaI\x17\x84\x82\x84\x86aH\x84V[[\x80\x84\x10\x15aI-WaI,\x84\x82\x84\x86aG\xC8V[[PPPPV[\x82\x81\x10\x15aIRWaIG_\x82\x84\x01aA\x85V[`\x01\x81\x01\x90PaI3V[PPPV[aIa_\x82aH\xF8V[PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[_\x82\x14aI\xA0WaI\x9FaIdV[[aI\xA9\x81aIWV[PPV[_[\x82\x81\x10\x15aI\xCEWaI\xC3_\x82\x84\x01aI\x90V[`\x01\x81\x01\x90PaI\xAFV[PPPV[\x81\x83\x10\x15aJ\nWaI\xE4\x82aGGV[aI\xED\x84aGGV[aI\xF6\x83aG[V[\x81\x81\x01aJ\x05\x83\x85\x03\x82aI\xADV[PPPP[PPPV[h\x01\0\0\0\0\0\0\0\0\x82\x11\x15aJ)WaJ(a<]V[[aJ2\x81aG=V[\x82\x82UaJ@\x83\x82\x84aI\xD3V[PPPV[_\x81Q\x90P\x91\x90PV[aJX\x82aE\x9DV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aJqWaJpa<]V[[aJ{\x82Ta@xV[aJ\x86\x82\x82\x85aA\xC3V[_` \x90P`\x1F\x83\x11`\x01\x81\x14aJ\xB7W_\x84\x15aJ\xA5W\x82\x87\x01Q\x90P[aJ\xAF\x85\x82aB>V[\x86UPaK\x16V[`\x1F\x19\x84\x16aJ\xC5\x86a@\xA8V[_[\x82\x81\x10\x15aJ\xECW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaJ\xC7V[\x86\x83\x10\x15aK\tW\x84\x89\x01QaK\x05`\x1F\x89\x16\x82aB\"V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[aK(\x82\x82aJOV[PPV[aK5\x82aEtV[aK?\x81\x83aJ\x0FV[aKH\x83aE\x8EV[aKQ\x83aG[V[_[\x83\x81\x10\x15aK\x86WaKd\x83aJEV[aKn\x81\x84aK\x1EV[` \x84\x01\x93P`\x01\x83\x01\x92PP`\x01\x81\x01\x90PaKSV[PPPPPPV[_`@\x82\x01\x90PaK\xA1_\x83\x01\x85a;\x10V[\x81\x81\x03` \x83\x01RaK\xB3\x81\x84a>uV[\x90P\x93\x92PPPV[_`@\x82\x01\x90PaK\xCF_\x83\x01\x85a9\x9FV[aK\xDC` \x83\x01\x84a9\x9FV[\x93\x92PPPV[_`@\x82\x01\x90PaK\xF6_\x83\x01\x85a9\xC7V[aL\x03` \x83\x01\x84a;\x10V[\x93\x92PPPV[_``\x82\x01\x90PaL\x1D_\x83\x01\x86a9\xC7V[aL*` \x83\x01\x85a9\xC7V[aL7`@\x83\x01\x84a;\x10V[\x94\x93PPPPV[_`\x80\x82\x01\x90PaLR_\x83\x01\x87a;\x10V[aL_` \x83\x01\x86a9\xC7V[aLl`@\x83\x01\x85a9\xC7V[aLy``\x83\x01\x84a;\x10V[\x95\x94PPPPPV[_`@\x82\x01\x90PaL\x95_\x83\x01\x85a;\x10V[aL\xA2` \x83\x01\x84a:\xAAV[\x93\x92PPPV[_`\xFF\x82\x16\x90P\x91\x90PV[aL\xBE\x81aL\xA9V[\x82RPPV[_`\x80\x82\x01\x90PaL\xD7_\x83\x01\x87a:\xAAV[aL\xE4` \x83\x01\x86aL\xB5V[aL\xF1`@\x83\x01\x85a:\xAAV[aL\xFE``\x83\x01\x84a:\xAAV[\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 3%3k\x06\xA7\x95N\xC3\xC2\0\t\x17\xEB\xEB\xB9\xF1m=\t\xCC/\xA3d\x85\x82\xCA\x1C$\xB1q\xDFdsolcC\0\x08!\x003",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `AccessControlBadConfirmation()` and selector `0x6697b232`.
```solidity
error AccessControlBadConfirmation();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AccessControlBadConfirmation;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
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
        impl ::core::convert::From<AccessControlBadConfirmation>
        for UnderlyingRustTuple<'_> {
            fn from(value: AccessControlBadConfirmation) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for AccessControlBadConfirmation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AccessControlBadConfirmation {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AccessControlBadConfirmation()";
            const SELECTOR: [u8; 4] = [102u8, 151u8, 178u8, 50u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `AccessControlUnauthorizedAccount(address,bytes32)` and selector `0xe2517d3f`.
```solidity
error AccessControlUnauthorizedAccount(address account, bytes32 neededRole);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AccessControlUnauthorizedAccount {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub neededRole: alloy::sol_types::private::FixedBytes<32>,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<AccessControlUnauthorizedAccount>
        for UnderlyingRustTuple<'_> {
            fn from(value: AccessControlUnauthorizedAccount) -> Self {
                (value.account, value.neededRole)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for AccessControlUnauthorizedAccount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    account: tuple.0,
                    neededRole: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AccessControlUnauthorizedAccount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AccessControlUnauthorizedAccount(address,bytes32)";
            const SELECTOR: [u8; 4] = [226u8, 81u8, 125u8, 63u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.neededRole),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `AlreadyReceivedOutputShares(address,address)` and selector `0x08e55495`.
```solidity
error AlreadyReceivedOutputShares(address client, address sender);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AlreadyReceivedOutputShares {
        #[allow(missing_docs)]
        pub client: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<AlreadyReceivedOutputShares>
        for UnderlyingRustTuple<'_> {
            fn from(value: AlreadyReceivedOutputShares) -> Self {
                (value.client, value.sender)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for AlreadyReceivedOutputShares {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    client: tuple.0,
                    sender: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AlreadyReceivedOutputShares {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AlreadyReceivedOutputShares(address,address)";
            const SELECTOR: [u8; 4] = [8u8, 229u8, 84u8, 149u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.client,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sender,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `AlreadySubmittedInputs(address)` and selector `0x4f5fbfc3`.
```solidity
error AlreadySubmittedInputs(address client);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AlreadySubmittedInputs {
        #[allow(missing_docs)]
        pub client: alloy::sol_types::private::Address,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
        impl ::core::convert::From<AlreadySubmittedInputs> for UnderlyingRustTuple<'_> {
            fn from(value: AlreadySubmittedInputs) -> Self {
                (value.client,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AlreadySubmittedInputs {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { client: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AlreadySubmittedInputs {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AlreadySubmittedInputs(address)";
            const SELECTOR: [u8; 4] = [79u8, 95u8, 191u8, 195u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.client,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ECDSAInvalidSignature()` and selector `0xf645eedf`.
```solidity
error ECDSAInvalidSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignature;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
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
        impl ::core::convert::From<ECDSAInvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignature()";
            const SELECTOR: [u8; 4] = [246u8, 69u8, 238u8, 223u8];
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
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ECDSAInvalidSignatureLength(uint256)` and selector `0xfce698f7`.
```solidity
error ECDSAInvalidSignatureLength(uint256 length);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignatureLength {
        #[allow(missing_docs)]
        pub length: alloy::sol_types::private::primitives::aliases::U256,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<ECDSAInvalidSignatureLength>
        for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignatureLength) -> Self {
                (value.length,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ECDSAInvalidSignatureLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { length: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignatureLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignatureLength(uint256)";
            const SELECTOR: [u8; 4] = [252u8, 230u8, 152u8, 247u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.length),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ECDSAInvalidSignatureS(bytes32)` and selector `0xd78bce0c`.
```solidity
error ECDSAInvalidSignatureS(bytes32 s);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignatureS {
        #[allow(missing_docs)]
        pub s: alloy::sol_types::private::FixedBytes<32>,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
        impl ::core::convert::From<ECDSAInvalidSignatureS> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignatureS) -> Self {
                (value.s,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignatureS {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { s: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignatureS {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignatureS(bytes32)";
            const SELECTOR: [u8; 4] = [215u8, 139u8, 206u8, 12u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.s),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `IndexNotReserved(address,uint256)` and selector `0xffabbae7`.
```solidity
error IndexNotReserved(address client, uint256 index);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IndexNotReserved {
        #[allow(missing_docs)]
        pub client: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<IndexNotReserved> for UnderlyingRustTuple<'_> {
            fn from(value: IndexNotReserved) -> Self {
                (value.client, value.index)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for IndexNotReserved {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    client: tuple.0,
                    index: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for IndexNotReserved {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IndexNotReserved(address,uint256)";
            const SELECTOR: [u8; 4] = [255u8, 171u8, 186u8, 231u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.client,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `IndicesAlreadyReserved(address)` and selector `0xaca92f09`.
```solidity
error IndicesAlreadyReserved(address client);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IndicesAlreadyReserved {
        #[allow(missing_docs)]
        pub client: alloy::sol_types::private::Address,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
        impl ::core::convert::From<IndicesAlreadyReserved> for UnderlyingRustTuple<'_> {
            fn from(value: IndicesAlreadyReserved) -> Self {
                (value.client,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for IndicesAlreadyReserved {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { client: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for IndicesAlreadyReserved {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IndicesAlreadyReserved(address)";
            const SELECTOR: [u8; 4] = [172u8, 169u8, 47u8, 9u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.client,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NoIndicesReserved(address)` and selector `0x6faf9f05`.
```solidity
error NoIndicesReserved(address client);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NoIndicesReserved {
        #[allow(missing_docs)]
        pub client: alloy::sol_types::private::Address,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
        impl ::core::convert::From<NoIndicesReserved> for UnderlyingRustTuple<'_> {
            fn from(value: NoIndicesReserved) -> Self {
                (value.client,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoIndicesReserved {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { client: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NoIndicesReserved {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NoIndicesReserved(address)";
            const SELECTOR: [u8; 4] = [111u8, 175u8, 159u8, 5u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.client,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NotAClient(address)` and selector `0xa032ac6b`.
```solidity
error NotAClient(address client);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotAClient {
        #[allow(missing_docs)]
        pub client: alloy::sol_types::private::Address,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
        impl ::core::convert::From<NotAClient> for UnderlyingRustTuple<'_> {
            fn from(value: NotAClient) -> Self {
                (value.client,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotAClient {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { client: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotAClient {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotAClient(address)";
            const SELECTOR: [u8; 4] = [160u8, 50u8, 172u8, 107u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.client,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NotAnExistingParty(address)` and selector `0xabdce06a`.
```solidity
error NotAnExistingParty(address account);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotAnExistingParty {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
        impl ::core::convert::From<NotAnExistingParty> for UnderlyingRustTuple<'_> {
            fn from(value: NotAnExistingParty) -> Self {
                (value.account,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotAnExistingParty {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { account: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotAnExistingParty {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotAnExistingParty(address)";
            const SELECTOR: [u8; 4] = [171u8, 220u8, 224u8, 106u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NotAtRound(uint8,uint8)` and selector `0xbfa217d8`.
```solidity
error NotAtRound(StoffelCoordinator.Round required, StoffelCoordinator.Round current);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotAtRound {
        #[allow(missing_docs)]
        pub required: <StoffelCoordinator::Round as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub current: <StoffelCoordinator::Round as alloy::sol_types::SolType>::RustType,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            StoffelCoordinator::Round,
            StoffelCoordinator::Round,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <StoffelCoordinator::Round as alloy::sol_types::SolType>::RustType,
            <StoffelCoordinator::Round as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<NotAtRound> for UnderlyingRustTuple<'_> {
            fn from(value: NotAtRound) -> Self {
                (value.required, value.current)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotAtRound {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    required: tuple.0,
                    current: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotAtRound {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotAtRound(uint8,uint8)";
            const SELECTOR: [u8; 4] = [191u8, 162u8, 23u8, 216u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <StoffelCoordinator::Round as alloy_sol_types::SolType>::tokenize(
                        &self.required,
                    ),
                    <StoffelCoordinator::Round as alloy_sol_types::SolType>::tokenize(
                        &self.current,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NotEnoughIndices(uint256,uint256)` and selector `0xdf3d75e2`.
```solidity
error NotEnoughIndices(uint256 requested, uint256 available);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotEnoughIndices {
        #[allow(missing_docs)]
        pub requested: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub available: alloy::sol_types::private::primitives::aliases::U256,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<NotEnoughIndices> for UnderlyingRustTuple<'_> {
            fn from(value: NotEnoughIndices) -> Self {
                (value.requested, value.available)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotEnoughIndices {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    requested: tuple.0,
                    available: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotEnoughIndices {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotEnoughIndices(uint256,uint256)";
            const SELECTOR: [u8; 4] = [223u8, 61u8, 117u8, 226u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.requested),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.available),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NotEnoughMPCParties(uint256,uint256)` and selector `0x3a236268`.
```solidity
error NotEnoughMPCParties(uint256 current, uint256 required);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotEnoughMPCParties {
        #[allow(missing_docs)]
        pub current: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub required: alloy::sol_types::private::primitives::aliases::U256,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<NotEnoughMPCParties> for UnderlyingRustTuple<'_> {
            fn from(value: NotEnoughMPCParties) -> Self {
                (value.current, value.required)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotEnoughMPCParties {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    current: tuple.0,
                    required: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotEnoughMPCParties {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotEnoughMPCParties(uint256,uint256)";
            const SELECTOR: [u8; 4] = [58u8, 35u8, 98u8, 104u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.current),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.required),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OwnableInvalidOwner(address)` and selector `0x1e4fbdf7`.
```solidity
error OwnableInvalidOwner(address owner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OwnableInvalidOwner {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
        impl ::core::convert::From<OwnableInvalidOwner> for UnderlyingRustTuple<'_> {
            fn from(value: OwnableInvalidOwner) -> Self {
                (value.owner,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OwnableInvalidOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { owner: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OwnableInvalidOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OwnableInvalidOwner(address)";
            const SELECTOR: [u8; 4] = [30u8, 79u8, 189u8, 247u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OwnableUnauthorizedAccount(address)` and selector `0x118cdaa7`.
```solidity
error OwnableUnauthorizedAccount(address account);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OwnableUnauthorizedAccount {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
        impl ::core::convert::From<OwnableUnauthorizedAccount>
        for UnderlyingRustTuple<'_> {
            fn from(value: OwnableUnauthorizedAccount) -> Self {
                (value.account,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OwnableUnauthorizedAccount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { account: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OwnableUnauthorizedAccount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OwnableUnauthorizedAccount(address)";
            const SELECTOR: [u8; 4] = [17u8, 140u8, 218u8, 167u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ZeroIndices(address)` and selector `0xb2fd5518`.
```solidity
error ZeroIndices(address client);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroIndices {
        #[allow(missing_docs)]
        pub client: alloy::sol_types::private::Address,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
        impl ::core::convert::From<ZeroIndices> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroIndices) -> Self {
                (value.client,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroIndices {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { client: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroIndices {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroIndices(address)";
            const SELECTOR: [u8; 4] = [178u8, 253u8, 85u8, 24u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.client,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ZeroMaskedInput(address)` and selector `0x16923cea`.
```solidity
error ZeroMaskedInput(address client);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroMaskedInput {
        #[allow(missing_docs)]
        pub client: alloy::sol_types::private::Address,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
        impl ::core::convert::From<ZeroMaskedInput> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroMaskedInput) -> Self {
                (value.client,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroMaskedInput {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { client: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroMaskedInput {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroMaskedInput(address)";
            const SELECTOR: [u8; 4] = [22u8, 146u8, 60u8, 234u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.client,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ClientAuthenticated(address,bool)` and selector `0x407009200520f9f10584813c0b95441ab320f6b08d97ebdaaf1e824eedd9d7c3`.
```solidity
event ClientAuthenticated(address indexed client, bool success);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ClientAuthenticated {
        #[allow(missing_docs)]
        pub client: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub success: bool,
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
        impl alloy_sol_types::SolEvent for ClientAuthenticated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ClientAuthenticated(address,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                64u8, 112u8, 9u8, 32u8, 5u8, 32u8, 249u8, 241u8, 5u8, 132u8, 129u8, 60u8,
                11u8, 149u8, 68u8, 26u8, 179u8, 32u8, 246u8, 176u8, 141u8, 151u8, 235u8,
                218u8, 175u8, 30u8, 130u8, 78u8, 237u8, 217u8, 215u8, 195u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    client: topics.1,
                    success: data.0,
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
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.success,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.client.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.client,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ClientAuthenticated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ClientAuthenticated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ClientAuthenticated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `CoordinatorInitialized(address,uint256,uint256,address)` and selector `0xdef1f08eb655f4a75f60bd6fd7e971112004abd846a612e46dab270770d24ca5`.
```solidity
event CoordinatorInitialized(address coordinator, uint256 timeofInitialization, uint256 creationBlock, address designatedParty);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CoordinatorInitialized {
        #[allow(missing_docs)]
        pub coordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub timeofInitialization: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub creationBlock: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub designatedParty: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for CoordinatorInitialized {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "CoordinatorInitialized(address,uint256,uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                222u8, 241u8, 240u8, 142u8, 182u8, 85u8, 244u8, 167u8, 95u8, 96u8, 189u8,
                111u8, 215u8, 233u8, 113u8, 17u8, 32u8, 4u8, 171u8, 216u8, 70u8, 166u8,
                18u8, 228u8, 109u8, 171u8, 39u8, 7u8, 112u8, 210u8, 76u8, 165u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    coordinator: data.0,
                    timeofInitialization: data.1,
                    creationBlock: data.2,
                    designatedParty: data.3,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.coordinator,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.timeofInitialization),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.creationBlock),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.designatedParty,
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
        impl alloy_sol_types::private::IntoLogData for CoordinatorInitialized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CoordinatorInitialized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &CoordinatorInitialized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `EnoughPrivateOutputShares(address,bytes[])` and selector `0x23e91dbfae03758cb88d7f6252b5710afa53a19ffe0f4b4f75d7f2de0c5eabe9`.
```solidity
event EnoughPrivateOutputShares(address indexed client, bytes[] shares);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EnoughPrivateOutputShares {
        #[allow(missing_docs)]
        pub client: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub shares: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
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
        impl alloy_sol_types::SolEvent for EnoughPrivateOutputShares {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "EnoughPrivateOutputShares(address,bytes[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                35u8, 233u8, 29u8, 191u8, 174u8, 3u8, 117u8, 140u8, 184u8, 141u8, 127u8,
                98u8, 82u8, 181u8, 113u8, 10u8, 250u8, 83u8, 161u8, 159u8, 254u8, 15u8,
                75u8, 79u8, 117u8, 215u8, 242u8, 222u8, 12u8, 94u8, 171u8, 233u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    client: topics.1,
                    shares: data.0,
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.shares),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.client.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.client,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EnoughPrivateOutputShares {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EnoughPrivateOutputShares> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &EnoughPrivateOutputShares,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ExecutionDone(address,uint256)` and selector `0x24a873651d26fb5a462cb37a91071cdd4d09abeebfe0ed144329bed1cc359d03`.
```solidity
event ExecutionDone(address executor, uint256 timeOfExecution);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ExecutionDone {
        #[allow(missing_docs)]
        pub executor: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub timeOfExecution: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for ExecutionDone {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ExecutionDone(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                36u8, 168u8, 115u8, 101u8, 29u8, 38u8, 251u8, 90u8, 70u8, 44u8, 179u8,
                122u8, 145u8, 7u8, 28u8, 221u8, 77u8, 9u8, 171u8, 238u8, 191u8, 224u8,
                237u8, 20u8, 67u8, 41u8, 190u8, 209u8, 204u8, 53u8, 157u8, 3u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    executor: data.0,
                    timeOfExecution: data.1,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.executor,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.timeOfExecution),
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
        impl alloy_sol_types::private::IntoLogData for ExecutionDone {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ExecutionDone> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ExecutionDone) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `IndexBufferEvent(uint256,address)` and selector `0xf7f0872382dff5e698b284e12084e4e7894f830216dd80cb4e909b593a58f935`.
```solidity
event IndexBufferEvent(uint256 totalIndices, address designatedParty);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct IndexBufferEvent {
        #[allow(missing_docs)]
        pub totalIndices: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub designatedParty: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for IndexBufferEvent {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "IndexBufferEvent(uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                247u8, 240u8, 135u8, 35u8, 130u8, 223u8, 245u8, 230u8, 152u8, 178u8,
                132u8, 225u8, 32u8, 132u8, 228u8, 231u8, 137u8, 79u8, 131u8, 2u8, 22u8,
                221u8, 128u8, 203u8, 78u8, 144u8, 155u8, 89u8, 58u8, 88u8, 249u8, 53u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    totalIndices: data.0,
                    designatedParty: data.1,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalIndices),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.designatedParty,
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
        impl alloy_sol_types::private::IntoLogData for IndexBufferEvent {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&IndexBufferEvent> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &IndexBufferEvent) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `InitializeStoffelAccessControl(uint256,uint256,address)` and selector `0xa2df7830e0bedef7b1116bf547b467b16b50b3bd23146c9e099878d14e89301a`.
```solidity
event InitializeStoffelAccessControl(uint256 nParties, uint256 t, address initializer);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct InitializeStoffelAccessControl {
        #[allow(missing_docs)]
        pub nParties: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub t: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub initializer: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for InitializeStoffelAccessControl {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "InitializeStoffelAccessControl(uint256,uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                162u8, 223u8, 120u8, 48u8, 224u8, 190u8, 222u8, 247u8, 177u8, 17u8,
                107u8, 245u8, 71u8, 180u8, 103u8, 177u8, 107u8, 80u8, 179u8, 189u8, 35u8,
                20u8, 108u8, 158u8, 9u8, 152u8, 120u8, 209u8, 78u8, 137u8, 48u8, 26u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    nParties: data.0,
                    t: data.1,
                    initializer: data.2,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nParties),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.t),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.initializer,
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
        impl alloy_sol_types::private::IntoLogData for InitializeStoffelAccessControl {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&InitializeStoffelAccessControl>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &InitializeStoffelAccessControl,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `InputCollectionStarted(address,uint256)` and selector `0x60edf9bdc7c4ea007cae1a9bbd03e41e5bfccd7231a6ec383c2edd7800f0d20c`.
```solidity
event InputCollectionStarted(address executor, uint256 timeOfExecution);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct InputCollectionStarted {
        #[allow(missing_docs)]
        pub executor: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub timeOfExecution: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for InputCollectionStarted {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "InputCollectionStarted(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                96u8, 237u8, 249u8, 189u8, 199u8, 196u8, 234u8, 0u8, 124u8, 174u8, 26u8,
                155u8, 189u8, 3u8, 228u8, 30u8, 91u8, 252u8, 205u8, 114u8, 49u8, 166u8,
                236u8, 56u8, 60u8, 46u8, 221u8, 120u8, 0u8, 240u8, 210u8, 12u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    executor: data.0,
                    timeOfExecution: data.1,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.executor,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.timeOfExecution),
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
        impl alloy_sol_types::private::IntoLogData for InputCollectionStarted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&InputCollectionStarted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &InputCollectionStarted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `InputMaskReservationStarted(address,uint256)` and selector `0x67c4489f674d03c7d19a9e36735188de7c65e8d1e99eb3a2fd258a769eb14fff`.
```solidity
event InputMaskReservationStarted(address executor, uint256 timeOfExecution);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct InputMaskReservationStarted {
        #[allow(missing_docs)]
        pub executor: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub timeOfExecution: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for InputMaskReservationStarted {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "InputMaskReservationStarted(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                103u8, 196u8, 72u8, 159u8, 103u8, 77u8, 3u8, 199u8, 209u8, 154u8, 158u8,
                54u8, 115u8, 81u8, 136u8, 222u8, 124u8, 101u8, 232u8, 209u8, 233u8,
                158u8, 179u8, 162u8, 253u8, 37u8, 138u8, 118u8, 158u8, 177u8, 79u8, 255u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    executor: data.0,
                    timeOfExecution: data.1,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.executor,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.timeOfExecution),
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
        impl alloy_sol_types::private::IntoLogData for InputMaskReservationStarted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&InputMaskReservationStarted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &InputMaskReservationStarted,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `MPCStarted(address,uint256)` and selector `0x20f55ed0c92f2bb1c8825488e1e3c98463d024b2a42dbd24838c3f75260f43e9`.
```solidity
event MPCStarted(address executor, uint256 timeOfExecution);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MPCStarted {
        #[allow(missing_docs)]
        pub executor: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub timeOfExecution: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for MPCStarted {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "MPCStarted(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                32u8, 245u8, 94u8, 208u8, 201u8, 47u8, 43u8, 177u8, 200u8, 130u8, 84u8,
                136u8, 225u8, 227u8, 201u8, 132u8, 99u8, 208u8, 36u8, 178u8, 164u8, 45u8,
                189u8, 36u8, 131u8, 140u8, 63u8, 117u8, 38u8, 15u8, 67u8, 233u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    executor: data.0,
                    timeOfExecution: data.1,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.executor,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.timeOfExecution),
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
        impl alloy_sol_types::private::IntoLogData for MPCStarted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MPCStarted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MPCStarted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `MaskedInputEvent(address,uint256,uint256)` and selector `0xb89addd937f44f902c849596641837cd7af2fcecef22d2a7866fdc1ad9c0ae2e`.
```solidity
event MaskedInputEvent(address client, uint256 maskedInput, uint256 reservedIndex);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MaskedInputEvent {
        #[allow(missing_docs)]
        pub client: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub maskedInput: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub reservedIndex: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for MaskedInputEvent {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "MaskedInputEvent(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                184u8, 154u8, 221u8, 217u8, 55u8, 244u8, 79u8, 144u8, 44u8, 132u8, 149u8,
                150u8, 100u8, 24u8, 55u8, 205u8, 122u8, 242u8, 252u8, 236u8, 239u8, 34u8,
                210u8, 167u8, 134u8, 111u8, 220u8, 26u8, 217u8, 192u8, 174u8, 46u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    client: data.0,
                    maskedInput: data.1,
                    reservedIndex: data.2,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.client,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maskedInput),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.reservedIndex),
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
        impl alloy_sol_types::private::IntoLogData for MaskedInputEvent {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MaskedInputEvent> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MaskedInputEvent) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OutputSendingStarted(address,uint256)` and selector `0x301f8a3701f5b260197382dd7301078542144fe8fddd18083d6f6e09e4958a59`.
```solidity
event OutputSendingStarted(address executor, uint256 timeOfExecution);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OutputSendingStarted {
        #[allow(missing_docs)]
        pub executor: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub timeOfExecution: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for OutputSendingStarted {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "OutputSendingStarted(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                48u8, 31u8, 138u8, 55u8, 1u8, 245u8, 178u8, 96u8, 25u8, 115u8, 130u8,
                221u8, 115u8, 1u8, 7u8, 133u8, 66u8, 20u8, 79u8, 232u8, 253u8, 221u8,
                24u8, 8u8, 61u8, 111u8, 110u8, 9u8, 228u8, 149u8, 138u8, 89u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    executor: data.0,
                    timeOfExecution: data.1,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.executor,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.timeOfExecution),
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
        impl alloy_sol_types::private::IntoLogData for OutputSendingStarted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OutputSendingStarted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OutputSendingStarted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
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
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
                )
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PreprocessingStarted(address,uint256)` and selector `0xbb709dea744f06d1b26e824deec2f7140c511266ee15d7a217838b31d8b0123d`.
```solidity
event PreprocessingStarted(address designatedParty, uint256 timeOfExecution);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PreprocessingStarted {
        #[allow(missing_docs)]
        pub designatedParty: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub timeOfExecution: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for PreprocessingStarted {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "PreprocessingStarted(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                187u8, 112u8, 157u8, 234u8, 116u8, 79u8, 6u8, 209u8, 178u8, 110u8, 130u8,
                77u8, 238u8, 194u8, 247u8, 20u8, 12u8, 81u8, 18u8, 102u8, 238u8, 21u8,
                215u8, 162u8, 23u8, 131u8, 139u8, 49u8, 216u8, 176u8, 18u8, 61u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    designatedParty: data.0,
                    timeOfExecution: data.1,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.designatedParty,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.timeOfExecution),
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
        impl alloy_sol_types::private::IntoLogData for PreprocessingStarted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PreprocessingStarted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PreprocessingStarted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ReservedInputEvent(address,uint256[])` and selector `0x1e14abe5d0cdb96adde7b9eca9b14bc08df623b5805afde5a3f0acadc2bf4f5b`.
```solidity
event ReservedInputEvent(address client, uint256[] reservedIndices);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ReservedInputEvent {
        #[allow(missing_docs)]
        pub client: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub reservedIndices: alloy::sol_types::private::Vec<
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
        impl alloy_sol_types::SolEvent for ReservedInputEvent {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ReservedInputEvent(address,uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                30u8, 20u8, 171u8, 229u8, 208u8, 205u8, 185u8, 106u8, 221u8, 231u8,
                185u8, 236u8, 169u8, 177u8, 75u8, 192u8, 141u8, 246u8, 35u8, 181u8,
                128u8, 90u8, 253u8, 229u8, 163u8, 240u8, 172u8, 173u8, 194u8, 191u8,
                79u8, 91u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    client: data.0,
                    reservedIndices: data.1,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.client,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.reservedIndices),
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
        impl alloy_sol_types::private::IntoLogData for ReservedInputEvent {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ReservedInputEvent> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ReservedInputEvent) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RoleAdminChanged(bytes32,bytes32,bytes32)` and selector `0xbd79b86ffe0ab8e8776151514217cd7cacd52c909f66475c3af44e129f0b00ff`.
```solidity
event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RoleAdminChanged {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub previousAdminRole: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub newAdminRole: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for RoleAdminChanged {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "RoleAdminChanged(bytes32,bytes32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                189u8, 121u8, 184u8, 111u8, 254u8, 10u8, 184u8, 232u8, 119u8, 97u8, 81u8,
                81u8, 66u8, 23u8, 205u8, 124u8, 172u8, 213u8, 44u8, 144u8, 159u8, 102u8,
                71u8, 92u8, 58u8, 244u8, 78u8, 18u8, 159u8, 11u8, 0u8, 255u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    role: topics.1,
                    previousAdminRole: topics.2,
                    newAdminRole: topics.3,
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
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.previousAdminRole.clone(),
                    self.newAdminRole.clone(),
                )
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
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.previousAdminRole);
                out[3usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.newAdminRole);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleAdminChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleAdminChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleAdminChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RoleGranted(bytes32,address,address)` and selector `0x2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d`.
```solidity
event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RoleGranted {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for RoleGranted {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RoleGranted(bytes32,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                47u8, 135u8, 136u8, 17u8, 126u8, 126u8, 255u8, 29u8, 130u8, 233u8, 38u8,
                236u8, 121u8, 73u8, 1u8, 209u8, 124u8, 120u8, 2u8, 74u8, 80u8, 39u8, 9u8,
                64u8, 48u8, 69u8, 64u8, 167u8, 51u8, 101u8, 111u8, 13u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    role: topics.1,
                    account: topics.2,
                    sender: topics.3,
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
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.account.clone(),
                    self.sender.clone(),
                )
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
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleGranted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleGranted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleGranted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RoleRevoked(bytes32,address,address)` and selector `0xf6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b`.
```solidity
event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RoleRevoked {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for RoleRevoked {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RoleRevoked(bytes32,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                246u8, 57u8, 31u8, 92u8, 50u8, 217u8, 198u8, 157u8, 42u8, 71u8, 234u8,
                103u8, 11u8, 68u8, 41u8, 116u8, 181u8, 57u8, 53u8, 209u8, 237u8, 199u8,
                253u8, 100u8, 235u8, 33u8, 224u8, 71u8, 168u8, 57u8, 23u8, 27u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    role: topics.1,
                    account: topics.2,
                    sender: topics.3,
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
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.account.clone(),
                    self.sender.clone(),
                )
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
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleRevoked {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleRevoked> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleRevoked) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(bytes32 stoffelProgramHash, uint256 t, address[] initialMpcNodes, uint256 nInputs);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub stoffelProgramHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub t: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub initialMpcNodes: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub nInputs: alloy::sol_types::private::primitives::aliases::U256,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (
                        value.stoffelProgramHash,
                        value.t,
                        value.initialMpcNodes,
                        value.nInputs,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        stoffelProgramHash: tuple.0,
                        t: tuple.1,
                        initialMpcNodes: tuple.2,
                        nInputs: tuple.3,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<256>,
            );
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
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.stoffelProgramHash),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.t),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.initialMpcNodes),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nInputs),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `CLIENT_ROLE()` and selector `0xeae6f652`.
```solidity
function CLIENT_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CLIENT_ROLECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`CLIENT_ROLE()`](CLIENT_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CLIENT_ROLEReturn {
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
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<CLIENT_ROLECall> for UnderlyingRustTuple<'_> {
                fn from(value: CLIENT_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CLIENT_ROLECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<CLIENT_ROLEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: CLIENT_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CLIENT_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for CLIENT_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CLIENT_ROLE()";
            const SELECTOR: [u8; 4] = [234u8, 230u8, 246u8, 82u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: CLIENT_ROLEReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: CLIENT_ROLEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`.
```solidity
function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_ADMIN_ROLECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`DEFAULT_ADMIN_ROLE()`](DEFAULT_ADMIN_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_ADMIN_ROLEReturn {
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
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<DEFAULT_ADMIN_ROLECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_ADMIN_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEFAULT_ADMIN_ROLECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<DEFAULT_ADMIN_ROLEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_ADMIN_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEFAULT_ADMIN_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DEFAULT_ADMIN_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DEFAULT_ADMIN_ROLE()";
            const SELECTOR: [u8; 4] = [162u8, 23u8, 253u8, 223u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: DEFAULT_ADMIN_ROLEReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: DEFAULT_ADMIN_ROLEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `DESIGNATED_PARTY_ROLE()` and selector `0x7f35b560`.
```solidity
function DESIGNATED_PARTY_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DESIGNATED_PARTY_ROLECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`DESIGNATED_PARTY_ROLE()`](DESIGNATED_PARTY_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DESIGNATED_PARTY_ROLEReturn {
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
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<DESIGNATED_PARTY_ROLECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DESIGNATED_PARTY_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DESIGNATED_PARTY_ROLECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<DESIGNATED_PARTY_ROLEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DESIGNATED_PARTY_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DESIGNATED_PARTY_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DESIGNATED_PARTY_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DESIGNATED_PARTY_ROLE()";
            const SELECTOR: [u8; 4] = [127u8, 53u8, 181u8, 96u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: DESIGNATED_PARTY_ROLEReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: DESIGNATED_PARTY_ROLEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `PARTY_ROLE()` and selector `0x30104c3e`.
```solidity
function PARTY_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PARTY_ROLECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`PARTY_ROLE()`](PARTY_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PARTY_ROLEReturn {
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
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<PARTY_ROLECall> for UnderlyingRustTuple<'_> {
                fn from(value: PARTY_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for PARTY_ROLECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<PARTY_ROLEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: PARTY_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for PARTY_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for PARTY_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PARTY_ROLE()";
            const SELECTOR: [u8; 4] = [48u8, 16u8, 76u8, 62u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: PARTY_ROLEReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: PARTY_ROLEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `authenticateClient(address,bytes)` and selector `0x1ee4ee0f`.
```solidity
function authenticateClient(address clientAddr, bytes memory signature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct authenticateClientCall {
        #[allow(missing_docs)]
        pub clientAddr: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`authenticateClient(address,bytes)`](authenticateClientCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct authenticateClientReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<authenticateClientCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: authenticateClientCall) -> Self {
                    (value.clientAddr, value.signature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for authenticateClientCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        clientAddr: tuple.0,
                        signature: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<authenticateClientReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: authenticateClientReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for authenticateClientReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl authenticateClientReturn {
            fn _tokenize(
                &self,
            ) -> <authenticateClientCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for authenticateClientCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = authenticateClientReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "authenticateClient(address,bytes)";
            const SELECTOR: [u8; 4] = [30u8, 228u8, 238u8, 15u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.clientAddr,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                authenticateClientReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `availableInputMasks()` and selector `0x2328bd12`.
```solidity
function availableInputMasks() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct availableInputMasksCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`availableInputMasks()`](availableInputMasksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct availableInputMasksReturn {
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
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<availableInputMasksCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: availableInputMasksCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for availableInputMasksCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<availableInputMasksReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: availableInputMasksReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for availableInputMasksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for availableInputMasksCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "availableInputMasks()";
            const SELECTOR: [u8; 4] = [35u8, 40u8, 189u8, 18u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: availableInputMasksReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: availableInputMasksReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `baseNonce()` and selector `0x1c7453db`.
```solidity
function baseNonce() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct baseNonceCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`baseNonce()`](baseNonceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct baseNonceReturn {
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
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<baseNonceCall> for UnderlyingRustTuple<'_> {
                fn from(value: baseNonceCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for baseNonceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<baseNonceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: baseNonceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for baseNonceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for baseNonceCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "baseNonce()";
            const SELECTOR: [u8; 4] = [28u8, 116u8, 83u8, 219u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: baseNonceReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: baseNonceReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `collectInputs()` and selector `0xbb51fef0`.
```solidity
function collectInputs() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct collectInputsCall;
    ///Container type for the return parameters of the [`collectInputs()`](collectInputsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct collectInputsReturn {}
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<collectInputsCall> for UnderlyingRustTuple<'_> {
                fn from(value: collectInputsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for collectInputsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<collectInputsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: collectInputsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for collectInputsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl collectInputsReturn {
            fn _tokenize(
                &self,
            ) -> <collectInputsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for collectInputsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = collectInputsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "collectInputs()";
            const SELECTOR: [u8; 4] = [187u8, 81u8, 254u8, 240u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                collectInputsReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `creationBlock()` and selector `0x17634514`.
```solidity
function creationBlock() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct creationBlockCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`creationBlock()`](creationBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct creationBlockReturn {
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
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<creationBlockCall> for UnderlyingRustTuple<'_> {
                fn from(value: creationBlockCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for creationBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<creationBlockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: creationBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for creationBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for creationBlockCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "creationBlock()";
            const SELECTOR: [u8; 4] = [23u8, 99u8, 69u8, 20u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: creationBlockReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: creationBlockReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `creationTime()` and selector `0xd8270dce`.
```solidity
function creationTime() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct creationTimeCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`creationTime()`](creationTimeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct creationTimeReturn {
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
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<creationTimeCall> for UnderlyingRustTuple<'_> {
                fn from(value: creationTimeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for creationTimeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<creationTimeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: creationTimeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for creationTimeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for creationTimeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "creationTime()";
            const SELECTOR: [u8; 4] = [216u8, 39u8, 13u8, 206u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: creationTimeReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: creationTimeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `finalize()` and selector `0x4bb278f3`.
```solidity
function finalize() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct finalizeCall;
    ///Container type for the return parameters of the [`finalize()`](finalizeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct finalizeReturn {}
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<finalizeCall> for UnderlyingRustTuple<'_> {
                fn from(value: finalizeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for finalizeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<finalizeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: finalizeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for finalizeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl finalizeReturn {
            fn _tokenize(
                &self,
            ) -> <finalizeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for finalizeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = finalizeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "finalize()";
            const SELECTOR: [u8; 4] = [75u8, 178u8, 120u8, 243u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                finalizeReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`.
```solidity
function getRoleAdmin(bytes32 role) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleAdminCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getRoleAdmin(bytes32)`](getRoleAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleAdminReturn {
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
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<getRoleAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRoleAdminCall) -> Self {
                    (value.role,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoleAdminCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { role: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<getRoleAdminReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getRoleAdminReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoleAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRoleAdminCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRoleAdmin(bytes32)";
            const SELECTOR: [u8; 4] = [36u8, 138u8, 156u8, 163u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getRoleAdminReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getRoleAdminReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getRoleMember(bytes32,uint256)` and selector `0x9010d07c`.
```solidity
function getRoleMember(bytes32 role, uint256 index) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleMemberCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getRoleMember(bytes32,uint256)`](getRoleMemberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleMemberReturn {
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
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getRoleMemberCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRoleMemberCall) -> Self {
                    (value.role, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoleMemberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        index: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<getRoleMemberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getRoleMemberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoleMemberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRoleMemberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRoleMember(bytes32,uint256)";
            const SELECTOR: [u8; 4] = [144u8, 16u8, 208u8, 124u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getRoleMemberReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getRoleMemberReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getRoleMemberCount(bytes32)` and selector `0xca15c873`.
```solidity
function getRoleMemberCount(bytes32 role) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleMemberCountCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getRoleMemberCount(bytes32)`](getRoleMemberCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleMemberCountReturn {
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
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<getRoleMemberCountCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRoleMemberCountCall) -> Self {
                    (value.role,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRoleMemberCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { role: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getRoleMemberCountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRoleMemberCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRoleMemberCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRoleMemberCountCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRoleMemberCount(bytes32)";
            const SELECTOR: [u8; 4] = [202u8, 21u8, 200u8, 115u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getRoleMemberCountReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getRoleMemberCountReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getRoleMembers(bytes32)` and selector `0xa3246ad3`.
```solidity
function getRoleMembers(bytes32 role) external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleMembersCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getRoleMembers(bytes32)`](getRoleMembersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleMembersReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<getRoleMembersCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRoleMembersCall) -> Self {
                    (value.role,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoleMembersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { role: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<getRoleMembersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRoleMembersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRoleMembersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRoleMembersCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRoleMembers(bytes32)";
            const SELECTOR: [u8; 4] = [163u8, 36u8, 106u8, 211u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getRoleMembersReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getRoleMembersReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`.
```solidity
function grantRole(bytes32 role, address account) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct grantRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`grantRole(bytes32,address)`](grantRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct grantRoleReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<grantRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: grantRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for grantRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<grantRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: grantRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for grantRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl grantRoleReturn {
            fn _tokenize(
                &self,
            ) -> <grantRoleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for grantRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = grantRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "grantRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [47u8, 47u8, 241u8, 93u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                grantRoleReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `hasRole(bytes32,address)` and selector `0x91d14854`.
```solidity
function hasRole(bytes32 role, address account) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`hasRole(bytes32,address)`](hasRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasRoleReturn {
        #[allow(missing_docs)]
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<hasRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: hasRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<hasRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: hasRoleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hasRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hasRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [145u8, 209u8, 72u8, 84u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: hasRoleReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: hasRoleReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isDesignatedParty(address)` and selector `0x13ff6dd5`.
```solidity
function isDesignatedParty(address account) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isDesignatedPartyCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isDesignatedParty(address)`](isDesignatedPartyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isDesignatedPartyReturn {
        #[allow(missing_docs)]
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<isDesignatedPartyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isDesignatedPartyCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isDesignatedPartyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<isDesignatedPartyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isDesignatedPartyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isDesignatedPartyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isDesignatedPartyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isDesignatedParty(address)";
            const SELECTOR: [u8; 4] = [19u8, 255u8, 109u8, 213u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: isDesignatedPartyReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: isDesignatedPartyReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isParty(address)` and selector `0xfc78b2e8`.
```solidity
function isParty(address account) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isPartyCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isParty(address)`](isPartyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isPartyReturn {
        #[allow(missing_docs)]
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<isPartyCall> for UnderlyingRustTuple<'_> {
                fn from(value: isPartyCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isPartyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<isPartyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isPartyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isPartyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isPartyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isParty(address)";
            const SELECTOR: [u8; 4] = [252u8, 120u8, 178u8, 232u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: isPartyReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: isPartyReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `obtainInputMasks(uint256)` and selector `0xebae35e7`.
```solidity
function obtainInputMasks(uint256 nIndices) external returns (uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct obtainInputMasksCall {
        #[allow(missing_docs)]
        pub nIndices: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`obtainInputMasks(uint256)`](obtainInputMasksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct obtainInputMasksReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<obtainInputMasksCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: obtainInputMasksCall) -> Self {
                    (value.nIndices,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for obtainInputMasksCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { nIndices: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<obtainInputMasksReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: obtainInputMasksReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for obtainInputMasksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for obtainInputMasksCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "obtainInputMasks(uint256)";
            const SELECTOR: [u8; 4] = [235u8, 174u8, 53u8, 231u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nIndices),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: obtainInputMasksReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: obtainInputMasksReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
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
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: ownerReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: ownerReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
```solidity
function renounceOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall;
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<renounceOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<renounceOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl renounceOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <renounceOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                renounceOwnershipReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`.
```solidity
function renounceRole(bytes32 role, address account) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`renounceRole(bytes32,address)`](renounceRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceRoleReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<renounceRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: renounceRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<renounceRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: renounceRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl renounceRoleReturn {
            fn _tokenize(
                &self,
            ) -> <renounceRoleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [54u8, 86u8, 138u8, 190u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                renounceRoleReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `reserveInputMasks()` and selector `0xc079f495`.
```solidity
function reserveInputMasks() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reserveInputMasksCall;
    ///Container type for the return parameters of the [`reserveInputMasks()`](reserveInputMasksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reserveInputMasksReturn {}
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<reserveInputMasksCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: reserveInputMasksCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for reserveInputMasksCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<reserveInputMasksReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: reserveInputMasksReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for reserveInputMasksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl reserveInputMasksReturn {
            fn _tokenize(
                &self,
            ) -> <reserveInputMasksCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for reserveInputMasksCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = reserveInputMasksReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "reserveInputMasks()";
            const SELECTOR: [u8; 4] = [192u8, 121u8, 244u8, 149u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                reserveInputMasksReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `resetAccessControl(uint256,address[])` and selector `0xaf206f28`.
```solidity
function resetAccessControl(uint256 t, address[] memory initialMpcNodes) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resetAccessControlCall {
        #[allow(missing_docs)]
        pub t: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub initialMpcNodes: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`resetAccessControl(uint256,address[])`](resetAccessControlCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resetAccessControlReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<resetAccessControlCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: resetAccessControlCall) -> Self {
                    (value.t, value.initialMpcNodes)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for resetAccessControlCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        t: tuple.0,
                        initialMpcNodes: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<resetAccessControlReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: resetAccessControlReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for resetAccessControlReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl resetAccessControlReturn {
            fn _tokenize(
                &self,
            ) -> <resetAccessControlCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for resetAccessControlCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = resetAccessControlReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "resetAccessControl(uint256,address[])";
            const SELECTOR: [u8; 4] = [175u8, 32u8, 111u8, 40u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.t),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.initialMpcNodes),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                resetAccessControlReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `resetCoordinator(bytes32,uint256,address[],uint256)` and selector `0xf6603c61`.
```solidity
function resetCoordinator(bytes32 stoffelProgramHash, uint256 t, address[] memory initialMpcNodes, uint256 nInputs) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resetCoordinatorCall {
        #[allow(missing_docs)]
        pub stoffelProgramHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub t: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub initialMpcNodes: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub nInputs: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`resetCoordinator(bytes32,uint256,address[],uint256)`](resetCoordinatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resetCoordinatorReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<resetCoordinatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: resetCoordinatorCall) -> Self {
                    (
                        value.stoffelProgramHash,
                        value.t,
                        value.initialMpcNodes,
                        value.nInputs,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for resetCoordinatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        stoffelProgramHash: tuple.0,
                        t: tuple.1,
                        initialMpcNodes: tuple.2,
                        nInputs: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<resetCoordinatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: resetCoordinatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for resetCoordinatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl resetCoordinatorReturn {
            fn _tokenize(
                &self,
            ) -> <resetCoordinatorCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for resetCoordinatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = resetCoordinatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "resetCoordinator(bytes32,uint256,address[],uint256)";
            const SELECTOR: [u8; 4] = [246u8, 96u8, 60u8, 97u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.stoffelProgramHash),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.t),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.initialMpcNodes),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nInputs),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                resetCoordinatorReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `resetInputManager(uint256,uint256)` and selector `0x3b4338d1`.
```solidity
function resetInputManager(uint256 nIndicesToReserve, uint256 t) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resetInputManagerCall {
        #[allow(missing_docs)]
        pub nIndicesToReserve: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub t: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`resetInputManager(uint256,uint256)`](resetInputManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resetInputManagerReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<resetInputManagerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: resetInputManagerCall) -> Self {
                    (value.nIndicesToReserve, value.t)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for resetInputManagerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        nIndicesToReserve: tuple.0,
                        t: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<resetInputManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: resetInputManagerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for resetInputManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl resetInputManagerReturn {
            fn _tokenize(
                &self,
            ) -> <resetInputManagerCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for resetInputManagerCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = resetInputManagerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "resetInputManager(uint256,uint256)";
            const SELECTOR: [u8; 4] = [59u8, 67u8, 56u8, 209u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nIndicesToReserve),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.t),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                resetInputManagerReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`.
```solidity
function revokeRole(bytes32 role, address account) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revokeRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`revokeRole(bytes32,address)`](revokeRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revokeRoleReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<revokeRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: revokeRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<revokeRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: revokeRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl revokeRoleReturn {
            fn _tokenize(
                &self,
            ) -> <revokeRoleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for revokeRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = revokeRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "revokeRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [213u8, 71u8, 116u8, 31u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                revokeRoleReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `round()` and selector `0x146ca531`.
```solidity
function round() external view returns (StoffelCoordinator.Round);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct roundCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`round()`](roundCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct roundReturn {
        #[allow(missing_docs)]
        pub _0: <StoffelCoordinator::Round as alloy::sol_types::SolType>::RustType,
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<roundCall> for UnderlyingRustTuple<'_> {
                fn from(value: roundCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for roundCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (StoffelCoordinator::Round,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <StoffelCoordinator::Round as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<roundReturn> for UnderlyingRustTuple<'_> {
                fn from(value: roundReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for roundReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for roundCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <StoffelCoordinator::Round as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (StoffelCoordinator::Round,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "round()";
            const SELECTOR: [u8; 4] = [20u8, 108u8, 165u8, 49u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<StoffelCoordinator::Round as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: roundReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: roundReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `sendOutputs()` and selector `0x4b8e6488`.
```solidity
function sendOutputs() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct sendOutputsCall;
    ///Container type for the return parameters of the [`sendOutputs()`](sendOutputsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct sendOutputsReturn {}
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<sendOutputsCall> for UnderlyingRustTuple<'_> {
                fn from(value: sendOutputsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for sendOutputsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<sendOutputsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: sendOutputsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for sendOutputsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl sendOutputsReturn {
            fn _tokenize(
                &self,
            ) -> <sendOutputsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for sendOutputsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = sendOutputsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "sendOutputs()";
            const SELECTOR: [u8; 4] = [75u8, 142u8, 100u8, 136u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                sendOutputsReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `sendPrivateOutputShares(address,bytes)` and selector `0xeb8575de`.
```solidity
function sendPrivateOutputShares(address client, bytes memory shares) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct sendPrivateOutputSharesCall {
        #[allow(missing_docs)]
        pub client: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub shares: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`sendPrivateOutputShares(address,bytes)`](sendPrivateOutputSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct sendPrivateOutputSharesReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<sendPrivateOutputSharesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: sendPrivateOutputSharesCall) -> Self {
                    (value.client, value.shares)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for sendPrivateOutputSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        client: tuple.0,
                        shares: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<sendPrivateOutputSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: sendPrivateOutputSharesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for sendPrivateOutputSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl sendPrivateOutputSharesReturn {
            fn _tokenize(
                &self,
            ) -> <sendPrivateOutputSharesCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for sendPrivateOutputSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = sendPrivateOutputSharesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "sendPrivateOutputShares(address,bytes)";
            const SELECTOR: [u8; 4] = [235u8, 133u8, 117u8, 222u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.client,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.shares,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                sendPrivateOutputSharesReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `sendPublicOutputs(bytes)` and selector `0x0d42eb6f`.
```solidity
function sendPublicOutputs(bytes memory _publicOutputs) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct sendPublicOutputsCall {
        #[allow(missing_docs)]
        pub _publicOutputs: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`sendPublicOutputs(bytes)`](sendPublicOutputsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct sendPublicOutputsReturn {}
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<sendPublicOutputsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: sendPublicOutputsCall) -> Self {
                    (value._publicOutputs,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for sendPublicOutputsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _publicOutputs: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<sendPublicOutputsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: sendPublicOutputsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for sendPublicOutputsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl sendPublicOutputsReturn {
            fn _tokenize(
                &self,
            ) -> <sendPublicOutputsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for sendPublicOutputsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = sendPublicOutputsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "sendPublicOutputs(bytes)";
            const SELECTOR: [u8; 4] = [13u8, 66u8, 235u8, 111u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._publicOutputs,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                sendPublicOutputsReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `startMpc()` and selector `0x33cc9a09`.
```solidity
function startMpc() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startMpcCall;
    ///Container type for the return parameters of the [`startMpc()`](startMpcCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startMpcReturn {}
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<startMpcCall> for UnderlyingRustTuple<'_> {
                fn from(value: startMpcCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for startMpcCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<startMpcReturn> for UnderlyingRustTuple<'_> {
                fn from(value: startMpcReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for startMpcReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl startMpcReturn {
            fn _tokenize(
                &self,
            ) -> <startMpcCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for startMpcCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = startMpcReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "startMpc()";
            const SELECTOR: [u8; 4] = [51u8, 204u8, 154u8, 9u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                startMpcReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `startPreprocessing()` and selector `0xcb9c4cc4`.
```solidity
function startPreprocessing() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startPreprocessingCall;
    ///Container type for the return parameters of the [`startPreprocessing()`](startPreprocessingCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startPreprocessingReturn {}
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<startPreprocessingCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: startPreprocessingCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for startPreprocessingCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<startPreprocessingReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: startPreprocessingReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for startPreprocessingReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl startPreprocessingReturn {
            fn _tokenize(
                &self,
            ) -> <startPreprocessingCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for startPreprocessingCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = startPreprocessingReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "startPreprocessing()";
            const SELECTOR: [u8; 4] = [203u8, 156u8, 76u8, 196u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                startPreprocessingReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `submitMaskedInput(uint256,uint256)` and selector `0x0bda81cf`.
```solidity
function submitMaskedInput(uint256 maskedInput, uint256 reservedIndex) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submitMaskedInputCall {
        #[allow(missing_docs)]
        pub maskedInput: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub reservedIndex: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`submitMaskedInput(uint256,uint256)`](submitMaskedInputCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submitMaskedInputReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<submitMaskedInputCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: submitMaskedInputCall) -> Self {
                    (value.maskedInput, value.reservedIndex)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for submitMaskedInputCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        maskedInput: tuple.0,
                        reservedIndex: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<submitMaskedInputReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: submitMaskedInputReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for submitMaskedInputReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl submitMaskedInputReturn {
            fn _tokenize(
                &self,
            ) -> <submitMaskedInputCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for submitMaskedInputCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = submitMaskedInputReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "submitMaskedInput(uint256,uint256)";
            const SELECTOR: [u8; 4] = [11u8, 218u8, 129u8, 207u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maskedInput),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.reservedIndex),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                submitMaskedInputReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`.
```solidity
function supportsInterface(bytes4 interfaceId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsInterfaceCall {
        #[allow(missing_docs)]
        pub interfaceId: alloy::sol_types::private::FixedBytes<4>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`supportsInterface(bytes4)`](supportsInterfaceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsInterfaceReturn {
        #[allow(missing_docs)]
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
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
            impl ::core::convert::From<supportsInterfaceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceCall) -> Self {
                    (value.interfaceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportsInterfaceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { interfaceId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<supportsInterfaceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportsInterfaceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for supportsInterfaceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "supportsInterface(bytes4)";
            const SELECTOR: [u8; 4] = [1u8, 255u8, 201u8, 167u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.interfaceId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: supportsInterfaceReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: supportsInterfaceReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl transferOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <transferOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newOwner,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                transferOwnershipReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`FakeCoordinator`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum FakeCoordinatorCalls {
        #[allow(missing_docs)]
        CLIENT_ROLE(CLIENT_ROLECall),
        #[allow(missing_docs)]
        DEFAULT_ADMIN_ROLE(DEFAULT_ADMIN_ROLECall),
        #[allow(missing_docs)]
        DESIGNATED_PARTY_ROLE(DESIGNATED_PARTY_ROLECall),
        #[allow(missing_docs)]
        PARTY_ROLE(PARTY_ROLECall),
        #[allow(missing_docs)]
        authenticateClient(authenticateClientCall),
        #[allow(missing_docs)]
        availableInputMasks(availableInputMasksCall),
        #[allow(missing_docs)]
        baseNonce(baseNonceCall),
        #[allow(missing_docs)]
        collectInputs(collectInputsCall),
        #[allow(missing_docs)]
        creationBlock(creationBlockCall),
        #[allow(missing_docs)]
        creationTime(creationTimeCall),
        #[allow(missing_docs)]
        finalize(finalizeCall),
        #[allow(missing_docs)]
        getRoleAdmin(getRoleAdminCall),
        #[allow(missing_docs)]
        getRoleMember(getRoleMemberCall),
        #[allow(missing_docs)]
        getRoleMemberCount(getRoleMemberCountCall),
        #[allow(missing_docs)]
        getRoleMembers(getRoleMembersCall),
        #[allow(missing_docs)]
        grantRole(grantRoleCall),
        #[allow(missing_docs)]
        hasRole(hasRoleCall),
        #[allow(missing_docs)]
        isDesignatedParty(isDesignatedPartyCall),
        #[allow(missing_docs)]
        isParty(isPartyCall),
        #[allow(missing_docs)]
        obtainInputMasks(obtainInputMasksCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        renounceRole(renounceRoleCall),
        #[allow(missing_docs)]
        reserveInputMasks(reserveInputMasksCall),
        #[allow(missing_docs)]
        resetAccessControl(resetAccessControlCall),
        #[allow(missing_docs)]
        resetCoordinator(resetCoordinatorCall),
        #[allow(missing_docs)]
        resetInputManager(resetInputManagerCall),
        #[allow(missing_docs)]
        revokeRole(revokeRoleCall),
        #[allow(missing_docs)]
        round(roundCall),
        #[allow(missing_docs)]
        sendOutputs(sendOutputsCall),
        #[allow(missing_docs)]
        sendPrivateOutputShares(sendPrivateOutputSharesCall),
        #[allow(missing_docs)]
        sendPublicOutputs(sendPublicOutputsCall),
        #[allow(missing_docs)]
        startMpc(startMpcCall),
        #[allow(missing_docs)]
        startPreprocessing(startPreprocessingCall),
        #[allow(missing_docs)]
        submitMaskedInput(submitMaskedInputCall),
        #[allow(missing_docs)]
        supportsInterface(supportsInterfaceCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
    }
    impl FakeCoordinatorCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 255u8, 201u8, 167u8],
            [11u8, 218u8, 129u8, 207u8],
            [13u8, 66u8, 235u8, 111u8],
            [19u8, 255u8, 109u8, 213u8],
            [20u8, 108u8, 165u8, 49u8],
            [23u8, 99u8, 69u8, 20u8],
            [28u8, 116u8, 83u8, 219u8],
            [30u8, 228u8, 238u8, 15u8],
            [35u8, 40u8, 189u8, 18u8],
            [36u8, 138u8, 156u8, 163u8],
            [47u8, 47u8, 241u8, 93u8],
            [48u8, 16u8, 76u8, 62u8],
            [51u8, 204u8, 154u8, 9u8],
            [54u8, 86u8, 138u8, 190u8],
            [59u8, 67u8, 56u8, 209u8],
            [75u8, 142u8, 100u8, 136u8],
            [75u8, 178u8, 120u8, 243u8],
            [113u8, 80u8, 24u8, 166u8],
            [127u8, 53u8, 181u8, 96u8],
            [141u8, 165u8, 203u8, 91u8],
            [144u8, 16u8, 208u8, 124u8],
            [145u8, 209u8, 72u8, 84u8],
            [162u8, 23u8, 253u8, 223u8],
            [163u8, 36u8, 106u8, 211u8],
            [175u8, 32u8, 111u8, 40u8],
            [187u8, 81u8, 254u8, 240u8],
            [192u8, 121u8, 244u8, 149u8],
            [202u8, 21u8, 200u8, 115u8],
            [203u8, 156u8, 76u8, 196u8],
            [213u8, 71u8, 116u8, 31u8],
            [216u8, 39u8, 13u8, 206u8],
            [234u8, 230u8, 246u8, 82u8],
            [235u8, 133u8, 117u8, 222u8],
            [235u8, 174u8, 53u8, 231u8],
            [242u8, 253u8, 227u8, 139u8],
            [246u8, 96u8, 60u8, 97u8],
            [252u8, 120u8, 178u8, 232u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(supportsInterface),
            ::core::stringify!(submitMaskedInput),
            ::core::stringify!(sendPublicOutputs),
            ::core::stringify!(isDesignatedParty),
            ::core::stringify!(round),
            ::core::stringify!(creationBlock),
            ::core::stringify!(baseNonce),
            ::core::stringify!(authenticateClient),
            ::core::stringify!(availableInputMasks),
            ::core::stringify!(getRoleAdmin),
            ::core::stringify!(grantRole),
            ::core::stringify!(PARTY_ROLE),
            ::core::stringify!(startMpc),
            ::core::stringify!(renounceRole),
            ::core::stringify!(resetInputManager),
            ::core::stringify!(sendOutputs),
            ::core::stringify!(finalize),
            ::core::stringify!(renounceOwnership),
            ::core::stringify!(DESIGNATED_PARTY_ROLE),
            ::core::stringify!(owner),
            ::core::stringify!(getRoleMember),
            ::core::stringify!(hasRole),
            ::core::stringify!(DEFAULT_ADMIN_ROLE),
            ::core::stringify!(getRoleMembers),
            ::core::stringify!(resetAccessControl),
            ::core::stringify!(collectInputs),
            ::core::stringify!(reserveInputMasks),
            ::core::stringify!(getRoleMemberCount),
            ::core::stringify!(startPreprocessing),
            ::core::stringify!(revokeRole),
            ::core::stringify!(creationTime),
            ::core::stringify!(CLIENT_ROLE),
            ::core::stringify!(sendPrivateOutputShares),
            ::core::stringify!(obtainInputMasks),
            ::core::stringify!(transferOwnership),
            ::core::stringify!(resetCoordinator),
            ::core::stringify!(isParty),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <supportsInterfaceCall as alloy_sol_types::SolCall>::SIGNATURE,
            <submitMaskedInputCall as alloy_sol_types::SolCall>::SIGNATURE,
            <sendPublicOutputsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isDesignatedPartyCall as alloy_sol_types::SolCall>::SIGNATURE,
            <roundCall as alloy_sol_types::SolCall>::SIGNATURE,
            <creationBlockCall as alloy_sol_types::SolCall>::SIGNATURE,
            <baseNonceCall as alloy_sol_types::SolCall>::SIGNATURE,
            <authenticateClientCall as alloy_sol_types::SolCall>::SIGNATURE,
            <availableInputMasksCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRoleAdminCall as alloy_sol_types::SolCall>::SIGNATURE,
            <grantRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <PARTY_ROLECall as alloy_sol_types::SolCall>::SIGNATURE,
            <startMpcCall as alloy_sol_types::SolCall>::SIGNATURE,
            <renounceRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <resetInputManagerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <sendOutputsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <finalizeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <renounceOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <DESIGNATED_PARTY_ROLECall as alloy_sol_types::SolCall>::SIGNATURE,
            <ownerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRoleMemberCall as alloy_sol_types::SolCall>::SIGNATURE,
            <hasRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRoleMembersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <resetAccessControlCall as alloy_sol_types::SolCall>::SIGNATURE,
            <collectInputsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <reserveInputMasksCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRoleMemberCountCall as alloy_sol_types::SolCall>::SIGNATURE,
            <startPreprocessingCall as alloy_sol_types::SolCall>::SIGNATURE,
            <revokeRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <creationTimeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <CLIENT_ROLECall as alloy_sol_types::SolCall>::SIGNATURE,
            <sendPrivateOutputSharesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <obtainInputMasksCall as alloy_sol_types::SolCall>::SIGNATURE,
            <transferOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <resetCoordinatorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isPartyCall as alloy_sol_types::SolCall>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for FakeCoordinatorCalls {
        const NAME: &'static str = "FakeCoordinatorCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 37usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::CLIENT_ROLE(_) => {
                    <CLIENT_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::DEFAULT_ADMIN_ROLE(_) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::DESIGNATED_PARTY_ROLE(_) => {
                    <DESIGNATED_PARTY_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::PARTY_ROLE(_) => {
                    <PARTY_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::authenticateClient(_) => {
                    <authenticateClientCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::availableInputMasks(_) => {
                    <availableInputMasksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::baseNonce(_) => {
                    <baseNonceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::collectInputs(_) => {
                    <collectInputsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::creationBlock(_) => {
                    <creationBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::creationTime(_) => {
                    <creationTimeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::finalize(_) => <finalizeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getRoleAdmin(_) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRoleMember(_) => {
                    <getRoleMemberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRoleMemberCount(_) => {
                    <getRoleMemberCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRoleMembers(_) => {
                    <getRoleMembersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::grantRole(_) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::hasRole(_) => <hasRoleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::isDesignatedParty(_) => {
                    <isDesignatedPartyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isParty(_) => <isPartyCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::obtainInputMasks(_) => {
                    <obtainInputMasksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceRole(_) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::reserveInputMasks(_) => {
                    <reserveInputMasksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::resetAccessControl(_) => {
                    <resetAccessControlCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::resetCoordinator(_) => {
                    <resetCoordinatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::resetInputManager(_) => {
                    <resetInputManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::revokeRole(_) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::round(_) => <roundCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::sendOutputs(_) => {
                    <sendOutputsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::sendPrivateOutputShares(_) => {
                    <sendPrivateOutputSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::sendPublicOutputs(_) => {
                    <sendPublicOutputsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::startMpc(_) => <startMpcCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::startPreprocessing(_) => {
                    <startPreprocessingCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::submitMaskedInput(_) => {
                    <submitMaskedInputCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::supportsInterface(_) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
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
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<FakeCoordinatorCalls>] = &[
                {
                    fn supportsInterface(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn submitMaskedInput(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <submitMaskedInputCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::submitMaskedInput)
                    }
                    submitMaskedInput
                },
                {
                    fn sendPublicOutputs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <sendPublicOutputsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::sendPublicOutputs)
                    }
                    sendPublicOutputs
                },
                {
                    fn isDesignatedParty(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <isDesignatedPartyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::isDesignatedParty)
                    }
                    isDesignatedParty
                },
                {
                    fn round(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <roundCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FakeCoordinatorCalls::round)
                    }
                    round
                },
                {
                    fn creationBlock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <creationBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::creationBlock)
                    }
                    creationBlock
                },
                {
                    fn baseNonce(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <baseNonceCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FakeCoordinatorCalls::baseNonce)
                    }
                    baseNonce
                },
                {
                    fn authenticateClient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <authenticateClientCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::authenticateClient)
                    }
                    authenticateClient
                },
                {
                    fn availableInputMasks(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <availableInputMasksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::availableInputMasks)
                    }
                    availableInputMasks
                },
                {
                    fn getRoleAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <getRoleAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::getRoleAdmin)
                    }
                    getRoleAdmin
                },
                {
                    fn grantRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <grantRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FakeCoordinatorCalls::grantRole)
                    }
                    grantRole
                },
                {
                    fn PARTY_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <PARTY_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::PARTY_ROLE)
                    }
                    PARTY_ROLE
                },
                {
                    fn startMpc(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <startMpcCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FakeCoordinatorCalls::startMpc)
                    }
                    startMpc
                },
                {
                    fn renounceRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <renounceRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::renounceRole)
                    }
                    renounceRole
                },
                {
                    fn resetInputManager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <resetInputManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::resetInputManager)
                    }
                    resetInputManager
                },
                {
                    fn sendOutputs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <sendOutputsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::sendOutputs)
                    }
                    sendOutputs
                },
                {
                    fn finalize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <finalizeCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FakeCoordinatorCalls::finalize)
                    }
                    finalize
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn DESIGNATED_PARTY_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <DESIGNATED_PARTY_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::DESIGNATED_PARTY_ROLE)
                    }
                    DESIGNATED_PARTY_ROLE
                },
                {
                    fn owner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FakeCoordinatorCalls::owner)
                    }
                    owner
                },
                {
                    fn getRoleMember(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <getRoleMemberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::getRoleMember)
                    }
                    getRoleMember
                },
                {
                    fn hasRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <hasRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FakeCoordinatorCalls::hasRole)
                    }
                    hasRole
                },
                {
                    fn DEFAULT_ADMIN_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::DEFAULT_ADMIN_ROLE)
                    }
                    DEFAULT_ADMIN_ROLE
                },
                {
                    fn getRoleMembers(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <getRoleMembersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::getRoleMembers)
                    }
                    getRoleMembers
                },
                {
                    fn resetAccessControl(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <resetAccessControlCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::resetAccessControl)
                    }
                    resetAccessControl
                },
                {
                    fn collectInputs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <collectInputsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::collectInputs)
                    }
                    collectInputs
                },
                {
                    fn reserveInputMasks(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <reserveInputMasksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::reserveInputMasks)
                    }
                    reserveInputMasks
                },
                {
                    fn getRoleMemberCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <getRoleMemberCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::getRoleMemberCount)
                    }
                    getRoleMemberCount
                },
                {
                    fn startPreprocessing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <startPreprocessingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::startPreprocessing)
                    }
                    startPreprocessing
                },
                {
                    fn revokeRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <revokeRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::revokeRole)
                    }
                    revokeRole
                },
                {
                    fn creationTime(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <creationTimeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::creationTime)
                    }
                    creationTime
                },
                {
                    fn CLIENT_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <CLIENT_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::CLIENT_ROLE)
                    }
                    CLIENT_ROLE
                },
                {
                    fn sendPrivateOutputShares(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <sendPrivateOutputSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::sendPrivateOutputShares)
                    }
                    sendPrivateOutputShares
                },
                {
                    fn obtainInputMasks(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <obtainInputMasksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::obtainInputMasks)
                    }
                    obtainInputMasks
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn resetCoordinator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <resetCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::resetCoordinator)
                    }
                    resetCoordinator
                },
                {
                    fn isParty(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <isPartyCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FakeCoordinatorCalls::isParty)
                    }
                    isParty
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
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<FakeCoordinatorCalls>] = &[
                {
                    fn supportsInterface(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn submitMaskedInput(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <submitMaskedInputCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::submitMaskedInput)
                    }
                    submitMaskedInput
                },
                {
                    fn sendPublicOutputs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <sendPublicOutputsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::sendPublicOutputs)
                    }
                    sendPublicOutputs
                },
                {
                    fn isDesignatedParty(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <isDesignatedPartyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::isDesignatedParty)
                    }
                    isDesignatedParty
                },
                {
                    fn round(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <roundCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::round)
                    }
                    round
                },
                {
                    fn creationBlock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <creationBlockCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::creationBlock)
                    }
                    creationBlock
                },
                {
                    fn baseNonce(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <baseNonceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::baseNonce)
                    }
                    baseNonce
                },
                {
                    fn authenticateClient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <authenticateClientCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::authenticateClient)
                    }
                    authenticateClient
                },
                {
                    fn availableInputMasks(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <availableInputMasksCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::availableInputMasks)
                    }
                    availableInputMasks
                },
                {
                    fn getRoleAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <getRoleAdminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::getRoleAdmin)
                    }
                    getRoleAdmin
                },
                {
                    fn grantRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <grantRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::grantRole)
                    }
                    grantRole
                },
                {
                    fn PARTY_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <PARTY_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::PARTY_ROLE)
                    }
                    PARTY_ROLE
                },
                {
                    fn startMpc(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <startMpcCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::startMpc)
                    }
                    startMpc
                },
                {
                    fn renounceRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <renounceRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::renounceRole)
                    }
                    renounceRole
                },
                {
                    fn resetInputManager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <resetInputManagerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::resetInputManager)
                    }
                    resetInputManager
                },
                {
                    fn sendOutputs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <sendOutputsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::sendOutputs)
                    }
                    sendOutputs
                },
                {
                    fn finalize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <finalizeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::finalize)
                    }
                    finalize
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn DESIGNATED_PARTY_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <DESIGNATED_PARTY_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::DESIGNATED_PARTY_ROLE)
                    }
                    DESIGNATED_PARTY_ROLE
                },
                {
                    fn owner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::owner)
                    }
                    owner
                },
                {
                    fn getRoleMember(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <getRoleMemberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::getRoleMember)
                    }
                    getRoleMember
                },
                {
                    fn hasRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <hasRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::hasRole)
                    }
                    hasRole
                },
                {
                    fn DEFAULT_ADMIN_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::DEFAULT_ADMIN_ROLE)
                    }
                    DEFAULT_ADMIN_ROLE
                },
                {
                    fn getRoleMembers(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <getRoleMembersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::getRoleMembers)
                    }
                    getRoleMembers
                },
                {
                    fn resetAccessControl(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <resetAccessControlCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::resetAccessControl)
                    }
                    resetAccessControl
                },
                {
                    fn collectInputs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <collectInputsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::collectInputs)
                    }
                    collectInputs
                },
                {
                    fn reserveInputMasks(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <reserveInputMasksCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::reserveInputMasks)
                    }
                    reserveInputMasks
                },
                {
                    fn getRoleMemberCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <getRoleMemberCountCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::getRoleMemberCount)
                    }
                    getRoleMemberCount
                },
                {
                    fn startPreprocessing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <startPreprocessingCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::startPreprocessing)
                    }
                    startPreprocessing
                },
                {
                    fn revokeRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <revokeRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::revokeRole)
                    }
                    revokeRole
                },
                {
                    fn creationTime(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <creationTimeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::creationTime)
                    }
                    creationTime
                },
                {
                    fn CLIENT_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <CLIENT_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::CLIENT_ROLE)
                    }
                    CLIENT_ROLE
                },
                {
                    fn sendPrivateOutputShares(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <sendPrivateOutputSharesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::sendPrivateOutputShares)
                    }
                    sendPrivateOutputShares
                },
                {
                    fn obtainInputMasks(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <obtainInputMasksCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::obtainInputMasks)
                    }
                    obtainInputMasks
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn resetCoordinator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <resetCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::resetCoordinator)
                    }
                    resetCoordinator
                },
                {
                    fn isParty(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <isPartyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::isParty)
                    }
                    isParty
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
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::CLIENT_ROLE(inner) => {
                    <CLIENT_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DEFAULT_ADMIN_ROLE(inner) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DESIGNATED_PARTY_ROLE(inner) => {
                    <DESIGNATED_PARTY_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PARTY_ROLE(inner) => {
                    <PARTY_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::authenticateClient(inner) => {
                    <authenticateClientCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::availableInputMasks(inner) => {
                    <availableInputMasksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::baseNonce(inner) => {
                    <baseNonceCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::collectInputs(inner) => {
                    <collectInputsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::creationBlock(inner) => {
                    <creationBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::creationTime(inner) => {
                    <creationTimeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::finalize(inner) => {
                    <finalizeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getRoleAdmin(inner) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRoleMember(inner) => {
                    <getRoleMemberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRoleMemberCount(inner) => {
                    <getRoleMemberCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRoleMembers(inner) => {
                    <getRoleMembersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::grantRole(inner) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::hasRole(inner) => {
                    <hasRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isDesignatedParty(inner) => {
                    <isDesignatedPartyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isParty(inner) => {
                    <isPartyCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::obtainInputMasks(inner) => {
                    <obtainInputMasksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceRole(inner) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::reserveInputMasks(inner) => {
                    <reserveInputMasksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::resetAccessControl(inner) => {
                    <resetAccessControlCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::resetCoordinator(inner) => {
                    <resetCoordinatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::resetInputManager(inner) => {
                    <resetInputManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::revokeRole(inner) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::round(inner) => {
                    <roundCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::sendOutputs(inner) => {
                    <sendOutputsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::sendPrivateOutputShares(inner) => {
                    <sendPrivateOutputSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::sendPublicOutputs(inner) => {
                    <sendPublicOutputsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::startMpc(inner) => {
                    <startMpcCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::startPreprocessing(inner) => {
                    <startPreprocessingCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::submitMaskedInput(inner) => {
                    <submitMaskedInputCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::CLIENT_ROLE(inner) => {
                    <CLIENT_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DEFAULT_ADMIN_ROLE(inner) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DESIGNATED_PARTY_ROLE(inner) => {
                    <DESIGNATED_PARTY_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PARTY_ROLE(inner) => {
                    <PARTY_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::authenticateClient(inner) => {
                    <authenticateClientCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::availableInputMasks(inner) => {
                    <availableInputMasksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::baseNonce(inner) => {
                    <baseNonceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::collectInputs(inner) => {
                    <collectInputsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::creationBlock(inner) => {
                    <creationBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::creationTime(inner) => {
                    <creationTimeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::finalize(inner) => {
                    <finalizeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRoleAdmin(inner) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRoleMember(inner) => {
                    <getRoleMemberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRoleMemberCount(inner) => {
                    <getRoleMemberCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRoleMembers(inner) => {
                    <getRoleMembersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::grantRole(inner) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::hasRole(inner) => {
                    <hasRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::isDesignatedParty(inner) => {
                    <isDesignatedPartyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isParty(inner) => {
                    <isPartyCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::obtainInputMasks(inner) => {
                    <obtainInputMasksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceRole(inner) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::reserveInputMasks(inner) => {
                    <reserveInputMasksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::resetAccessControl(inner) => {
                    <resetAccessControlCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::resetCoordinator(inner) => {
                    <resetCoordinatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::resetInputManager(inner) => {
                    <resetInputManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::revokeRole(inner) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::round(inner) => {
                    <roundCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::sendOutputs(inner) => {
                    <sendOutputsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::sendPrivateOutputShares(inner) => {
                    <sendPrivateOutputSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::sendPublicOutputs(inner) => {
                    <sendPublicOutputsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::startMpc(inner) => {
                    <startMpcCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::startPreprocessing(inner) => {
                    <startPreprocessingCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::submitMaskedInput(inner) => {
                    <submitMaskedInputCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`FakeCoordinator`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum FakeCoordinatorErrors {
        #[allow(missing_docs)]
        AccessControlBadConfirmation(AccessControlBadConfirmation),
        #[allow(missing_docs)]
        AccessControlUnauthorizedAccount(AccessControlUnauthorizedAccount),
        #[allow(missing_docs)]
        AlreadyReceivedOutputShares(AlreadyReceivedOutputShares),
        #[allow(missing_docs)]
        AlreadySubmittedInputs(AlreadySubmittedInputs),
        #[allow(missing_docs)]
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        #[allow(missing_docs)]
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        #[allow(missing_docs)]
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
        #[allow(missing_docs)]
        IndexNotReserved(IndexNotReserved),
        #[allow(missing_docs)]
        IndicesAlreadyReserved(IndicesAlreadyReserved),
        #[allow(missing_docs)]
        NoIndicesReserved(NoIndicesReserved),
        #[allow(missing_docs)]
        NotAClient(NotAClient),
        #[allow(missing_docs)]
        NotAnExistingParty(NotAnExistingParty),
        #[allow(missing_docs)]
        NotAtRound(NotAtRound),
        #[allow(missing_docs)]
        NotEnoughIndices(NotEnoughIndices),
        #[allow(missing_docs)]
        NotEnoughMPCParties(NotEnoughMPCParties),
        #[allow(missing_docs)]
        OwnableInvalidOwner(OwnableInvalidOwner),
        #[allow(missing_docs)]
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        #[allow(missing_docs)]
        ZeroIndices(ZeroIndices),
        #[allow(missing_docs)]
        ZeroMaskedInput(ZeroMaskedInput),
    }
    impl FakeCoordinatorErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [8u8, 229u8, 84u8, 149u8],
            [17u8, 140u8, 218u8, 167u8],
            [22u8, 146u8, 60u8, 234u8],
            [30u8, 79u8, 189u8, 247u8],
            [58u8, 35u8, 98u8, 104u8],
            [79u8, 95u8, 191u8, 195u8],
            [102u8, 151u8, 178u8, 50u8],
            [111u8, 175u8, 159u8, 5u8],
            [160u8, 50u8, 172u8, 107u8],
            [171u8, 220u8, 224u8, 106u8],
            [172u8, 169u8, 47u8, 9u8],
            [178u8, 253u8, 85u8, 24u8],
            [191u8, 162u8, 23u8, 216u8],
            [215u8, 139u8, 206u8, 12u8],
            [223u8, 61u8, 117u8, 226u8],
            [226u8, 81u8, 125u8, 63u8],
            [246u8, 69u8, 238u8, 223u8],
            [252u8, 230u8, 152u8, 247u8],
            [255u8, 171u8, 186u8, 231u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(AlreadyReceivedOutputShares),
            ::core::stringify!(OwnableUnauthorizedAccount),
            ::core::stringify!(ZeroMaskedInput),
            ::core::stringify!(OwnableInvalidOwner),
            ::core::stringify!(NotEnoughMPCParties),
            ::core::stringify!(AlreadySubmittedInputs),
            ::core::stringify!(AccessControlBadConfirmation),
            ::core::stringify!(NoIndicesReserved),
            ::core::stringify!(NotAClient),
            ::core::stringify!(NotAnExistingParty),
            ::core::stringify!(IndicesAlreadyReserved),
            ::core::stringify!(ZeroIndices),
            ::core::stringify!(NotAtRound),
            ::core::stringify!(ECDSAInvalidSignatureS),
            ::core::stringify!(NotEnoughIndices),
            ::core::stringify!(AccessControlUnauthorizedAccount),
            ::core::stringify!(ECDSAInvalidSignature),
            ::core::stringify!(ECDSAInvalidSignatureLength),
            ::core::stringify!(IndexNotReserved),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <AlreadyReceivedOutputShares as alloy_sol_types::SolError>::SIGNATURE,
            <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroMaskedInput as alloy_sol_types::SolError>::SIGNATURE,
            <OwnableInvalidOwner as alloy_sol_types::SolError>::SIGNATURE,
            <NotEnoughMPCParties as alloy_sol_types::SolError>::SIGNATURE,
            <AlreadySubmittedInputs as alloy_sol_types::SolError>::SIGNATURE,
            <AccessControlBadConfirmation as alloy_sol_types::SolError>::SIGNATURE,
            <NoIndicesReserved as alloy_sol_types::SolError>::SIGNATURE,
            <NotAClient as alloy_sol_types::SolError>::SIGNATURE,
            <NotAnExistingParty as alloy_sol_types::SolError>::SIGNATURE,
            <IndicesAlreadyReserved as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroIndices as alloy_sol_types::SolError>::SIGNATURE,
            <NotAtRound as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SIGNATURE,
            <NotEnoughIndices as alloy_sol_types::SolError>::SIGNATURE,
            <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignature as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::SIGNATURE,
            <IndexNotReserved as alloy_sol_types::SolError>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for FakeCoordinatorErrors {
        const NAME: &'static str = "FakeCoordinatorErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 19usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AccessControlBadConfirmation(_) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AccessControlUnauthorizedAccount(_) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AlreadyReceivedOutputShares(_) => {
                    <AlreadyReceivedOutputShares as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AlreadySubmittedInputs(_) => {
                    <AlreadySubmittedInputs as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignature(_) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignatureLength(_) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignatureS(_) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SELECTOR
                }
                Self::IndexNotReserved(_) => {
                    <IndexNotReserved as alloy_sol_types::SolError>::SELECTOR
                }
                Self::IndicesAlreadyReserved(_) => {
                    <IndicesAlreadyReserved as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoIndicesReserved(_) => {
                    <NoIndicesReserved as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotAClient(_) => {
                    <NotAClient as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotAnExistingParty(_) => {
                    <NotAnExistingParty as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotAtRound(_) => {
                    <NotAtRound as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotEnoughIndices(_) => {
                    <NotEnoughIndices as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotEnoughMPCParties(_) => {
                    <NotEnoughMPCParties as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OwnableInvalidOwner(_) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OwnableUnauthorizedAccount(_) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroIndices(_) => {
                    <ZeroIndices as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroMaskedInput(_) => {
                    <ZeroMaskedInput as alloy_sol_types::SolError>::SELECTOR
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
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<FakeCoordinatorErrors>] = &[
                {
                    fn AlreadyReceivedOutputShares(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <AlreadyReceivedOutputShares as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorErrors::AlreadyReceivedOutputShares)
                    }
                    AlreadyReceivedOutputShares
                },
                {
                    fn OwnableUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorErrors::OwnableUnauthorizedAccount)
                    }
                    OwnableUnauthorizedAccount
                },
                {
                    fn ZeroMaskedInput(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <ZeroMaskedInput as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorErrors::ZeroMaskedInput)
                    }
                    ZeroMaskedInput
                },
                {
                    fn OwnableInvalidOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorErrors::OwnableInvalidOwner)
                    }
                    OwnableInvalidOwner
                },
                {
                    fn NotEnoughMPCParties(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <NotEnoughMPCParties as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorErrors::NotEnoughMPCParties)
                    }
                    NotEnoughMPCParties
                },
                {
                    fn AlreadySubmittedInputs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <AlreadySubmittedInputs as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorErrors::AlreadySubmittedInputs)
                    }
                    AlreadySubmittedInputs
                },
                {
                    fn AccessControlBadConfirmation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorErrors::AccessControlBadConfirmation)
                    }
                    AccessControlBadConfirmation
                },
                {
                    fn NoIndicesReserved(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <NoIndicesReserved as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorErrors::NoIndicesReserved)
                    }
                    NoIndicesReserved
                },
                {
                    fn NotAClient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <NotAClient as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(FakeCoordinatorErrors::NotAClient)
                    }
                    NotAClient
                },
                {
                    fn NotAnExistingParty(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <NotAnExistingParty as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorErrors::NotAnExistingParty)
                    }
                    NotAnExistingParty
                },
                {
                    fn IndicesAlreadyReserved(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <IndicesAlreadyReserved as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorErrors::IndicesAlreadyReserved)
                    }
                    IndicesAlreadyReserved
                },
                {
                    fn ZeroIndices(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <ZeroIndices as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(FakeCoordinatorErrors::ZeroIndices)
                    }
                    ZeroIndices
                },
                {
                    fn NotAtRound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <NotAtRound as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(FakeCoordinatorErrors::NotAtRound)
                    }
                    NotAtRound
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn NotEnoughIndices(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <NotEnoughIndices as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorErrors::NotEnoughIndices)
                    }
                    NotEnoughIndices
                },
                {
                    fn AccessControlUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorErrors::AccessControlUnauthorizedAccount)
                    }
                    AccessControlUnauthorizedAccount
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorErrors::ECDSAInvalidSignatureLength)
                    }
                    ECDSAInvalidSignatureLength
                },
                {
                    fn IndexNotReserved(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <IndexNotReserved as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorErrors::IndexNotReserved)
                    }
                    IndexNotReserved
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
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<FakeCoordinatorErrors>] = &[
                {
                    fn AlreadyReceivedOutputShares(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <AlreadyReceivedOutputShares as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::AlreadyReceivedOutputShares)
                    }
                    AlreadyReceivedOutputShares
                },
                {
                    fn OwnableUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::OwnableUnauthorizedAccount)
                    }
                    OwnableUnauthorizedAccount
                },
                {
                    fn ZeroMaskedInput(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <ZeroMaskedInput as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::ZeroMaskedInput)
                    }
                    ZeroMaskedInput
                },
                {
                    fn OwnableInvalidOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::OwnableInvalidOwner)
                    }
                    OwnableInvalidOwner
                },
                {
                    fn NotEnoughMPCParties(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <NotEnoughMPCParties as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::NotEnoughMPCParties)
                    }
                    NotEnoughMPCParties
                },
                {
                    fn AlreadySubmittedInputs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <AlreadySubmittedInputs as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::AlreadySubmittedInputs)
                    }
                    AlreadySubmittedInputs
                },
                {
                    fn AccessControlBadConfirmation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::AccessControlBadConfirmation)
                    }
                    AccessControlBadConfirmation
                },
                {
                    fn NoIndicesReserved(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <NoIndicesReserved as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::NoIndicesReserved)
                    }
                    NoIndicesReserved
                },
                {
                    fn NotAClient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <NotAClient as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::NotAClient)
                    }
                    NotAClient
                },
                {
                    fn NotAnExistingParty(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <NotAnExistingParty as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::NotAnExistingParty)
                    }
                    NotAnExistingParty
                },
                {
                    fn IndicesAlreadyReserved(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <IndicesAlreadyReserved as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::IndicesAlreadyReserved)
                    }
                    IndicesAlreadyReserved
                },
                {
                    fn ZeroIndices(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <ZeroIndices as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::ZeroIndices)
                    }
                    ZeroIndices
                },
                {
                    fn NotAtRound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <NotAtRound as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::NotAtRound)
                    }
                    NotAtRound
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn NotEnoughIndices(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <NotEnoughIndices as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::NotEnoughIndices)
                    }
                    NotEnoughIndices
                },
                {
                    fn AccessControlUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::AccessControlUnauthorizedAccount)
                    }
                    AccessControlUnauthorizedAccount
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::ECDSAInvalidSignatureLength)
                    }
                    ECDSAInvalidSignatureLength
                },
                {
                    fn IndexNotReserved(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <IndexNotReserved as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::IndexNotReserved)
                    }
                    IndexNotReserved
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
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::AccessControlBadConfirmation(inner) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AccessControlUnauthorizedAccount(inner) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AlreadyReceivedOutputShares(inner) => {
                    <AlreadyReceivedOutputShares as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AlreadySubmittedInputs(inner) => {
                    <AlreadySubmittedInputs as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignature(inner) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignatureLength(inner) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignatureS(inner) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::IndexNotReserved(inner) => {
                    <IndexNotReserved as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::IndicesAlreadyReserved(inner) => {
                    <IndicesAlreadyReserved as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NoIndicesReserved(inner) => {
                    <NoIndicesReserved as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotAClient(inner) => {
                    <NotAClient as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotAnExistingParty(inner) => {
                    <NotAnExistingParty as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotAtRound(inner) => {
                    <NotAtRound as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotEnoughIndices(inner) => {
                    <NotEnoughIndices as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotEnoughMPCParties(inner) => {
                    <NotEnoughMPCParties as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OwnableInvalidOwner(inner) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OwnableUnauthorizedAccount(inner) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ZeroIndices(inner) => {
                    <ZeroIndices as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ZeroMaskedInput(inner) => {
                    <ZeroMaskedInput as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AccessControlBadConfirmation(inner) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AccessControlUnauthorizedAccount(inner) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AlreadyReceivedOutputShares(inner) => {
                    <AlreadyReceivedOutputShares as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AlreadySubmittedInputs(inner) => {
                    <AlreadySubmittedInputs as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignature(inner) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignatureLength(inner) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignatureS(inner) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::IndexNotReserved(inner) => {
                    <IndexNotReserved as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::IndicesAlreadyReserved(inner) => {
                    <IndicesAlreadyReserved as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NoIndicesReserved(inner) => {
                    <NoIndicesReserved as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotAClient(inner) => {
                    <NotAClient as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::NotAnExistingParty(inner) => {
                    <NotAnExistingParty as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotAtRound(inner) => {
                    <NotAtRound as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::NotEnoughIndices(inner) => {
                    <NotEnoughIndices as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotEnoughMPCParties(inner) => {
                    <NotEnoughMPCParties as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OwnableInvalidOwner(inner) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OwnableUnauthorizedAccount(inner) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ZeroIndices(inner) => {
                    <ZeroIndices as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ZeroMaskedInput(inner) => {
                    <ZeroMaskedInput as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`FakeCoordinator`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum FakeCoordinatorEvents {
        #[allow(missing_docs)]
        ClientAuthenticated(ClientAuthenticated),
        #[allow(missing_docs)]
        CoordinatorInitialized(CoordinatorInitialized),
        #[allow(missing_docs)]
        EnoughPrivateOutputShares(EnoughPrivateOutputShares),
        #[allow(missing_docs)]
        ExecutionDone(ExecutionDone),
        #[allow(missing_docs)]
        IndexBufferEvent(IndexBufferEvent),
        #[allow(missing_docs)]
        InitializeStoffelAccessControl(InitializeStoffelAccessControl),
        #[allow(missing_docs)]
        InputCollectionStarted(InputCollectionStarted),
        #[allow(missing_docs)]
        InputMaskReservationStarted(InputMaskReservationStarted),
        #[allow(missing_docs)]
        MPCStarted(MPCStarted),
        #[allow(missing_docs)]
        MaskedInputEvent(MaskedInputEvent),
        #[allow(missing_docs)]
        OutputSendingStarted(OutputSendingStarted),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        PreprocessingStarted(PreprocessingStarted),
        #[allow(missing_docs)]
        ReservedInputEvent(ReservedInputEvent),
        #[allow(missing_docs)]
        RoleAdminChanged(RoleAdminChanged),
        #[allow(missing_docs)]
        RoleGranted(RoleGranted),
        #[allow(missing_docs)]
        RoleRevoked(RoleRevoked),
    }
    impl FakeCoordinatorEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                30u8, 20u8, 171u8, 229u8, 208u8, 205u8, 185u8, 106u8, 221u8, 231u8,
                185u8, 236u8, 169u8, 177u8, 75u8, 192u8, 141u8, 246u8, 35u8, 181u8,
                128u8, 90u8, 253u8, 229u8, 163u8, 240u8, 172u8, 173u8, 194u8, 191u8,
                79u8, 91u8,
            ],
            [
                32u8, 245u8, 94u8, 208u8, 201u8, 47u8, 43u8, 177u8, 200u8, 130u8, 84u8,
                136u8, 225u8, 227u8, 201u8, 132u8, 99u8, 208u8, 36u8, 178u8, 164u8, 45u8,
                189u8, 36u8, 131u8, 140u8, 63u8, 117u8, 38u8, 15u8, 67u8, 233u8,
            ],
            [
                35u8, 233u8, 29u8, 191u8, 174u8, 3u8, 117u8, 140u8, 184u8, 141u8, 127u8,
                98u8, 82u8, 181u8, 113u8, 10u8, 250u8, 83u8, 161u8, 159u8, 254u8, 15u8,
                75u8, 79u8, 117u8, 215u8, 242u8, 222u8, 12u8, 94u8, 171u8, 233u8,
            ],
            [
                36u8, 168u8, 115u8, 101u8, 29u8, 38u8, 251u8, 90u8, 70u8, 44u8, 179u8,
                122u8, 145u8, 7u8, 28u8, 221u8, 77u8, 9u8, 171u8, 238u8, 191u8, 224u8,
                237u8, 20u8, 67u8, 41u8, 190u8, 209u8, 204u8, 53u8, 157u8, 3u8,
            ],
            [
                47u8, 135u8, 136u8, 17u8, 126u8, 126u8, 255u8, 29u8, 130u8, 233u8, 38u8,
                236u8, 121u8, 73u8, 1u8, 209u8, 124u8, 120u8, 2u8, 74u8, 80u8, 39u8, 9u8,
                64u8, 48u8, 69u8, 64u8, 167u8, 51u8, 101u8, 111u8, 13u8,
            ],
            [
                48u8, 31u8, 138u8, 55u8, 1u8, 245u8, 178u8, 96u8, 25u8, 115u8, 130u8,
                221u8, 115u8, 1u8, 7u8, 133u8, 66u8, 20u8, 79u8, 232u8, 253u8, 221u8,
                24u8, 8u8, 61u8, 111u8, 110u8, 9u8, 228u8, 149u8, 138u8, 89u8,
            ],
            [
                64u8, 112u8, 9u8, 32u8, 5u8, 32u8, 249u8, 241u8, 5u8, 132u8, 129u8, 60u8,
                11u8, 149u8, 68u8, 26u8, 179u8, 32u8, 246u8, 176u8, 141u8, 151u8, 235u8,
                218u8, 175u8, 30u8, 130u8, 78u8, 237u8, 217u8, 215u8, 195u8,
            ],
            [
                96u8, 237u8, 249u8, 189u8, 199u8, 196u8, 234u8, 0u8, 124u8, 174u8, 26u8,
                155u8, 189u8, 3u8, 228u8, 30u8, 91u8, 252u8, 205u8, 114u8, 49u8, 166u8,
                236u8, 56u8, 60u8, 46u8, 221u8, 120u8, 0u8, 240u8, 210u8, 12u8,
            ],
            [
                103u8, 196u8, 72u8, 159u8, 103u8, 77u8, 3u8, 199u8, 209u8, 154u8, 158u8,
                54u8, 115u8, 81u8, 136u8, 222u8, 124u8, 101u8, 232u8, 209u8, 233u8,
                158u8, 179u8, 162u8, 253u8, 37u8, 138u8, 118u8, 158u8, 177u8, 79u8, 255u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                162u8, 223u8, 120u8, 48u8, 224u8, 190u8, 222u8, 247u8, 177u8, 17u8,
                107u8, 245u8, 71u8, 180u8, 103u8, 177u8, 107u8, 80u8, 179u8, 189u8, 35u8,
                20u8, 108u8, 158u8, 9u8, 152u8, 120u8, 209u8, 78u8, 137u8, 48u8, 26u8,
            ],
            [
                184u8, 154u8, 221u8, 217u8, 55u8, 244u8, 79u8, 144u8, 44u8, 132u8, 149u8,
                150u8, 100u8, 24u8, 55u8, 205u8, 122u8, 242u8, 252u8, 236u8, 239u8, 34u8,
                210u8, 167u8, 134u8, 111u8, 220u8, 26u8, 217u8, 192u8, 174u8, 46u8,
            ],
            [
                187u8, 112u8, 157u8, 234u8, 116u8, 79u8, 6u8, 209u8, 178u8, 110u8, 130u8,
                77u8, 238u8, 194u8, 247u8, 20u8, 12u8, 81u8, 18u8, 102u8, 238u8, 21u8,
                215u8, 162u8, 23u8, 131u8, 139u8, 49u8, 216u8, 176u8, 18u8, 61u8,
            ],
            [
                189u8, 121u8, 184u8, 111u8, 254u8, 10u8, 184u8, 232u8, 119u8, 97u8, 81u8,
                81u8, 66u8, 23u8, 205u8, 124u8, 172u8, 213u8, 44u8, 144u8, 159u8, 102u8,
                71u8, 92u8, 58u8, 244u8, 78u8, 18u8, 159u8, 11u8, 0u8, 255u8,
            ],
            [
                222u8, 241u8, 240u8, 142u8, 182u8, 85u8, 244u8, 167u8, 95u8, 96u8, 189u8,
                111u8, 215u8, 233u8, 113u8, 17u8, 32u8, 4u8, 171u8, 216u8, 70u8, 166u8,
                18u8, 228u8, 109u8, 171u8, 39u8, 7u8, 112u8, 210u8, 76u8, 165u8,
            ],
            [
                246u8, 57u8, 31u8, 92u8, 50u8, 217u8, 198u8, 157u8, 42u8, 71u8, 234u8,
                103u8, 11u8, 68u8, 41u8, 116u8, 181u8, 57u8, 53u8, 209u8, 237u8, 199u8,
                253u8, 100u8, 235u8, 33u8, 224u8, 71u8, 168u8, 57u8, 23u8, 27u8,
            ],
            [
                247u8, 240u8, 135u8, 35u8, 130u8, 223u8, 245u8, 230u8, 152u8, 178u8,
                132u8, 225u8, 32u8, 132u8, 228u8, 231u8, 137u8, 79u8, 131u8, 2u8, 22u8,
                221u8, 128u8, 203u8, 78u8, 144u8, 155u8, 89u8, 58u8, 88u8, 249u8, 53u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(ReservedInputEvent),
            ::core::stringify!(MPCStarted),
            ::core::stringify!(EnoughPrivateOutputShares),
            ::core::stringify!(ExecutionDone),
            ::core::stringify!(RoleGranted),
            ::core::stringify!(OutputSendingStarted),
            ::core::stringify!(ClientAuthenticated),
            ::core::stringify!(InputCollectionStarted),
            ::core::stringify!(InputMaskReservationStarted),
            ::core::stringify!(OwnershipTransferred),
            ::core::stringify!(InitializeStoffelAccessControl),
            ::core::stringify!(MaskedInputEvent),
            ::core::stringify!(PreprocessingStarted),
            ::core::stringify!(RoleAdminChanged),
            ::core::stringify!(CoordinatorInitialized),
            ::core::stringify!(RoleRevoked),
            ::core::stringify!(IndexBufferEvent),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <ReservedInputEvent as alloy_sol_types::SolEvent>::SIGNATURE,
            <MPCStarted as alloy_sol_types::SolEvent>::SIGNATURE,
            <EnoughPrivateOutputShares as alloy_sol_types::SolEvent>::SIGNATURE,
            <ExecutionDone as alloy_sol_types::SolEvent>::SIGNATURE,
            <RoleGranted as alloy_sol_types::SolEvent>::SIGNATURE,
            <OutputSendingStarted as alloy_sol_types::SolEvent>::SIGNATURE,
            <ClientAuthenticated as alloy_sol_types::SolEvent>::SIGNATURE,
            <InputCollectionStarted as alloy_sol_types::SolEvent>::SIGNATURE,
            <InputMaskReservationStarted as alloy_sol_types::SolEvent>::SIGNATURE,
            <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE,
            <InitializeStoffelAccessControl as alloy_sol_types::SolEvent>::SIGNATURE,
            <MaskedInputEvent as alloy_sol_types::SolEvent>::SIGNATURE,
            <PreprocessingStarted as alloy_sol_types::SolEvent>::SIGNATURE,
            <RoleAdminChanged as alloy_sol_types::SolEvent>::SIGNATURE,
            <CoordinatorInitialized as alloy_sol_types::SolEvent>::SIGNATURE,
            <RoleRevoked as alloy_sol_types::SolEvent>::SIGNATURE,
            <IndexBufferEvent as alloy_sol_types::SolEvent>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for FakeCoordinatorEvents {
        const NAME: &'static str = "FakeCoordinatorEvents";
        const COUNT: usize = 17usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <ClientAuthenticated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ClientAuthenticated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ClientAuthenticated)
                }
                Some(
                    <CoordinatorInitialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <CoordinatorInitialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::CoordinatorInitialized)
                }
                Some(
                    <EnoughPrivateOutputShares as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <EnoughPrivateOutputShares as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::EnoughPrivateOutputShares)
                }
                Some(<ExecutionDone as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ExecutionDone as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ExecutionDone)
                }
                Some(<IndexBufferEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <IndexBufferEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::IndexBufferEvent)
                }
                Some(
                    <InitializeStoffelAccessControl as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <InitializeStoffelAccessControl as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::InitializeStoffelAccessControl)
                }
                Some(
                    <InputCollectionStarted as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <InputCollectionStarted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::InputCollectionStarted)
                }
                Some(
                    <InputMaskReservationStarted as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <InputMaskReservationStarted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::InputMaskReservationStarted)
                }
                Some(<MPCStarted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <MPCStarted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::MPCStarted)
                }
                Some(<MaskedInputEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <MaskedInputEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::MaskedInputEvent)
                }
                Some(
                    <OutputSendingStarted as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OutputSendingStarted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OutputSendingStarted)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(
                    <PreprocessingStarted as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PreprocessingStarted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PreprocessingStarted)
                }
                Some(
                    <ReservedInputEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ReservedInputEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ReservedInputEvent)
                }
                Some(<RoleAdminChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleAdminChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RoleAdminChanged)
                }
                Some(<RoleGranted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleGranted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RoleGranted)
                }
                Some(<RoleRevoked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleRevoked as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RoleRevoked)
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
    impl alloy_sol_types::private::IntoLogData for FakeCoordinatorEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ClientAuthenticated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::CoordinatorInitialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EnoughPrivateOutputShares(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ExecutionDone(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::IndexBufferEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::InitializeStoffelAccessControl(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::InputCollectionStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::InputMaskReservationStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MPCStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MaskedInputEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OutputSendingStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PreprocessingStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ReservedInputEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleAdminChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleGranted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleRevoked(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ClientAuthenticated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::CoordinatorInitialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EnoughPrivateOutputShares(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ExecutionDone(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::IndexBufferEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::InitializeStoffelAccessControl(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::InputCollectionStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::InputMaskReservationStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MPCStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MaskedInputEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OutputSendingStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PreprocessingStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ReservedInputEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleAdminChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleGranted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleRevoked(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`FakeCoordinator`](self) contract instance.

See the [wrapper's documentation](`FakeCoordinatorInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> FakeCoordinatorInstance<P, N> {
        FakeCoordinatorInstance::<P, N>::new(address, __provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
        stoffelProgramHash: alloy::sol_types::private::FixedBytes<32>,
        t: alloy::sol_types::private::primitives::aliases::U256,
        initialMpcNodes: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        nInputs: alloy::sol_types::private::primitives::aliases::U256,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<FakeCoordinatorInstance<P, N>>,
    > {
        FakeCoordinatorInstance::<
            P,
            N,
        >::deploy(__provider, stoffelProgramHash, t, initialMpcNodes, nInputs)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
        stoffelProgramHash: alloy::sol_types::private::FixedBytes<32>,
        t: alloy::sol_types::private::primitives::aliases::U256,
        initialMpcNodes: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        nInputs: alloy::sol_types::private::primitives::aliases::U256,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        FakeCoordinatorInstance::<
            P,
            N,
        >::deploy_builder(__provider, stoffelProgramHash, t, initialMpcNodes, nInputs)
    }
    /**A [`FakeCoordinator`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`FakeCoordinator`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct FakeCoordinatorInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for FakeCoordinatorInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("FakeCoordinatorInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > FakeCoordinatorInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`FakeCoordinator`](self) contract instance.

See the [wrapper's documentation](`FakeCoordinatorInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            __provider: P,
            stoffelProgramHash: alloy::sol_types::private::FixedBytes<32>,
            t: alloy::sol_types::private::primitives::aliases::U256,
            initialMpcNodes: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            nInputs: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::Result<FakeCoordinatorInstance<P, N>> {
            let call_builder = Self::deploy_builder(
                __provider,
                stoffelProgramHash,
                t,
                initialMpcNodes,
                nInputs,
            );
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            __provider: P,
            stoffelProgramHash: alloy::sol_types::private::FixedBytes<32>,
            t: alloy::sol_types::private::primitives::aliases::U256,
            initialMpcNodes: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            nInputs: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            stoffelProgramHash,
                            t,
                            initialMpcNodes,
                            nInputs,
                        },
                    )[..],
                ]
                    .concat()
                    .into(),
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
    impl<P: ::core::clone::Clone, N> FakeCoordinatorInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> FakeCoordinatorInstance<P, N> {
            FakeCoordinatorInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > FakeCoordinatorInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`CLIENT_ROLE`] function.
        pub fn CLIENT_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, CLIENT_ROLECall, N> {
            self.call_builder(&CLIENT_ROLECall)
        }
        ///Creates a new call builder for the [`DEFAULT_ADMIN_ROLE`] function.
        pub fn DEFAULT_ADMIN_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, DEFAULT_ADMIN_ROLECall, N> {
            self.call_builder(&DEFAULT_ADMIN_ROLECall)
        }
        ///Creates a new call builder for the [`DESIGNATED_PARTY_ROLE`] function.
        pub fn DESIGNATED_PARTY_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, DESIGNATED_PARTY_ROLECall, N> {
            self.call_builder(&DESIGNATED_PARTY_ROLECall)
        }
        ///Creates a new call builder for the [`PARTY_ROLE`] function.
        pub fn PARTY_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, PARTY_ROLECall, N> {
            self.call_builder(&PARTY_ROLECall)
        }
        ///Creates a new call builder for the [`authenticateClient`] function.
        pub fn authenticateClient(
            &self,
            clientAddr: alloy::sol_types::private::Address,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, authenticateClientCall, N> {
            self.call_builder(
                &authenticateClientCall {
                    clientAddr,
                    signature,
                },
            )
        }
        ///Creates a new call builder for the [`availableInputMasks`] function.
        pub fn availableInputMasks(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, availableInputMasksCall, N> {
            self.call_builder(&availableInputMasksCall)
        }
        ///Creates a new call builder for the [`baseNonce`] function.
        pub fn baseNonce(&self) -> alloy_contract::SolCallBuilder<&P, baseNonceCall, N> {
            self.call_builder(&baseNonceCall)
        }
        ///Creates a new call builder for the [`collectInputs`] function.
        pub fn collectInputs(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, collectInputsCall, N> {
            self.call_builder(&collectInputsCall)
        }
        ///Creates a new call builder for the [`creationBlock`] function.
        pub fn creationBlock(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, creationBlockCall, N> {
            self.call_builder(&creationBlockCall)
        }
        ///Creates a new call builder for the [`creationTime`] function.
        pub fn creationTime(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, creationTimeCall, N> {
            self.call_builder(&creationTimeCall)
        }
        ///Creates a new call builder for the [`finalize`] function.
        pub fn finalize(&self) -> alloy_contract::SolCallBuilder<&P, finalizeCall, N> {
            self.call_builder(&finalizeCall)
        }
        ///Creates a new call builder for the [`getRoleAdmin`] function.
        pub fn getRoleAdmin(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, getRoleAdminCall, N> {
            self.call_builder(&getRoleAdminCall { role })
        }
        ///Creates a new call builder for the [`getRoleMember`] function.
        pub fn getRoleMember(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getRoleMemberCall, N> {
            self.call_builder(&getRoleMemberCall { role, index })
        }
        ///Creates a new call builder for the [`getRoleMemberCount`] function.
        pub fn getRoleMemberCount(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, getRoleMemberCountCall, N> {
            self.call_builder(&getRoleMemberCountCall { role })
        }
        ///Creates a new call builder for the [`getRoleMembers`] function.
        pub fn getRoleMembers(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, getRoleMembersCall, N> {
            self.call_builder(&getRoleMembersCall { role })
        }
        ///Creates a new call builder for the [`grantRole`] function.
        pub fn grantRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, grantRoleCall, N> {
            self.call_builder(&grantRoleCall { role, account })
        }
        ///Creates a new call builder for the [`hasRole`] function.
        pub fn hasRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, hasRoleCall, N> {
            self.call_builder(&hasRoleCall { role, account })
        }
        ///Creates a new call builder for the [`isDesignatedParty`] function.
        pub fn isDesignatedParty(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, isDesignatedPartyCall, N> {
            self.call_builder(&isDesignatedPartyCall { account })
        }
        ///Creates a new call builder for the [`isParty`] function.
        pub fn isParty(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, isPartyCall, N> {
            self.call_builder(&isPartyCall { account })
        }
        ///Creates a new call builder for the [`obtainInputMasks`] function.
        pub fn obtainInputMasks(
            &self,
            nIndices: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, obtainInputMasksCall, N> {
            self.call_builder(&obtainInputMasksCall { nIndices })
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<&P, ownerCall, N> {
            self.call_builder(&ownerCall)
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall)
        }
        ///Creates a new call builder for the [`renounceRole`] function.
        pub fn renounceRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, renounceRoleCall, N> {
            self.call_builder(&renounceRoleCall { role, account })
        }
        ///Creates a new call builder for the [`reserveInputMasks`] function.
        pub fn reserveInputMasks(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, reserveInputMasksCall, N> {
            self.call_builder(&reserveInputMasksCall)
        }
        ///Creates a new call builder for the [`resetAccessControl`] function.
        pub fn resetAccessControl(
            &self,
            t: alloy::sol_types::private::primitives::aliases::U256,
            initialMpcNodes: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, resetAccessControlCall, N> {
            self.call_builder(
                &resetAccessControlCall {
                    t,
                    initialMpcNodes,
                },
            )
        }
        ///Creates a new call builder for the [`resetCoordinator`] function.
        pub fn resetCoordinator(
            &self,
            stoffelProgramHash: alloy::sol_types::private::FixedBytes<32>,
            t: alloy::sol_types::private::primitives::aliases::U256,
            initialMpcNodes: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            nInputs: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, resetCoordinatorCall, N> {
            self.call_builder(
                &resetCoordinatorCall {
                    stoffelProgramHash,
                    t,
                    initialMpcNodes,
                    nInputs,
                },
            )
        }
        ///Creates a new call builder for the [`resetInputManager`] function.
        pub fn resetInputManager(
            &self,
            nIndicesToReserve: alloy::sol_types::private::primitives::aliases::U256,
            t: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, resetInputManagerCall, N> {
            self.call_builder(
                &resetInputManagerCall {
                    nIndicesToReserve,
                    t,
                },
            )
        }
        ///Creates a new call builder for the [`revokeRole`] function.
        pub fn revokeRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, revokeRoleCall, N> {
            self.call_builder(&revokeRoleCall { role, account })
        }
        ///Creates a new call builder for the [`round`] function.
        pub fn round(&self) -> alloy_contract::SolCallBuilder<&P, roundCall, N> {
            self.call_builder(&roundCall)
        }
        ///Creates a new call builder for the [`sendOutputs`] function.
        pub fn sendOutputs(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, sendOutputsCall, N> {
            self.call_builder(&sendOutputsCall)
        }
        ///Creates a new call builder for the [`sendPrivateOutputShares`] function.
        pub fn sendPrivateOutputShares(
            &self,
            client: alloy::sol_types::private::Address,
            shares: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, sendPrivateOutputSharesCall, N> {
            self.call_builder(
                &sendPrivateOutputSharesCall {
                    client,
                    shares,
                },
            )
        }
        ///Creates a new call builder for the [`sendPublicOutputs`] function.
        pub fn sendPublicOutputs(
            &self,
            _publicOutputs: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, sendPublicOutputsCall, N> {
            self.call_builder(
                &sendPublicOutputsCall {
                    _publicOutputs,
                },
            )
        }
        ///Creates a new call builder for the [`startMpc`] function.
        pub fn startMpc(&self) -> alloy_contract::SolCallBuilder<&P, startMpcCall, N> {
            self.call_builder(&startMpcCall)
        }
        ///Creates a new call builder for the [`startPreprocessing`] function.
        pub fn startPreprocessing(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, startPreprocessingCall, N> {
            self.call_builder(&startPreprocessingCall)
        }
        ///Creates a new call builder for the [`submitMaskedInput`] function.
        pub fn submitMaskedInput(
            &self,
            maskedInput: alloy::sol_types::private::primitives::aliases::U256,
            reservedIndex: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, submitMaskedInputCall, N> {
            self.call_builder(
                &submitMaskedInputCall {
                    maskedInput,
                    reservedIndex,
                },
            )
        }
        ///Creates a new call builder for the [`supportsInterface`] function.
        pub fn supportsInterface(
            &self,
            interfaceId: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::SolCallBuilder<&P, supportsInterfaceCall, N> {
            self.call_builder(
                &supportsInterfaceCall {
                    interfaceId,
                },
            )
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > FakeCoordinatorInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`ClientAuthenticated`] event.
        pub fn ClientAuthenticated_filter(
            &self,
        ) -> alloy_contract::Event<&P, ClientAuthenticated, N> {
            self.event_filter::<ClientAuthenticated>()
        }
        ///Creates a new event filter for the [`CoordinatorInitialized`] event.
        pub fn CoordinatorInitialized_filter(
            &self,
        ) -> alloy_contract::Event<&P, CoordinatorInitialized, N> {
            self.event_filter::<CoordinatorInitialized>()
        }
        ///Creates a new event filter for the [`EnoughPrivateOutputShares`] event.
        pub fn EnoughPrivateOutputShares_filter(
            &self,
        ) -> alloy_contract::Event<&P, EnoughPrivateOutputShares, N> {
            self.event_filter::<EnoughPrivateOutputShares>()
        }
        ///Creates a new event filter for the [`ExecutionDone`] event.
        pub fn ExecutionDone_filter(
            &self,
        ) -> alloy_contract::Event<&P, ExecutionDone, N> {
            self.event_filter::<ExecutionDone>()
        }
        ///Creates a new event filter for the [`IndexBufferEvent`] event.
        pub fn IndexBufferEvent_filter(
            &self,
        ) -> alloy_contract::Event<&P, IndexBufferEvent, N> {
            self.event_filter::<IndexBufferEvent>()
        }
        ///Creates a new event filter for the [`InitializeStoffelAccessControl`] event.
        pub fn InitializeStoffelAccessControl_filter(
            &self,
        ) -> alloy_contract::Event<&P, InitializeStoffelAccessControl, N> {
            self.event_filter::<InitializeStoffelAccessControl>()
        }
        ///Creates a new event filter for the [`InputCollectionStarted`] event.
        pub fn InputCollectionStarted_filter(
            &self,
        ) -> alloy_contract::Event<&P, InputCollectionStarted, N> {
            self.event_filter::<InputCollectionStarted>()
        }
        ///Creates a new event filter for the [`InputMaskReservationStarted`] event.
        pub fn InputMaskReservationStarted_filter(
            &self,
        ) -> alloy_contract::Event<&P, InputMaskReservationStarted, N> {
            self.event_filter::<InputMaskReservationStarted>()
        }
        ///Creates a new event filter for the [`MPCStarted`] event.
        pub fn MPCStarted_filter(&self) -> alloy_contract::Event<&P, MPCStarted, N> {
            self.event_filter::<MPCStarted>()
        }
        ///Creates a new event filter for the [`MaskedInputEvent`] event.
        pub fn MaskedInputEvent_filter(
            &self,
        ) -> alloy_contract::Event<&P, MaskedInputEvent, N> {
            self.event_filter::<MaskedInputEvent>()
        }
        ///Creates a new event filter for the [`OutputSendingStarted`] event.
        pub fn OutputSendingStarted_filter(
            &self,
        ) -> alloy_contract::Event<&P, OutputSendingStarted, N> {
            self.event_filter::<OutputSendingStarted>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<&P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`PreprocessingStarted`] event.
        pub fn PreprocessingStarted_filter(
            &self,
        ) -> alloy_contract::Event<&P, PreprocessingStarted, N> {
            self.event_filter::<PreprocessingStarted>()
        }
        ///Creates a new event filter for the [`ReservedInputEvent`] event.
        pub fn ReservedInputEvent_filter(
            &self,
        ) -> alloy_contract::Event<&P, ReservedInputEvent, N> {
            self.event_filter::<ReservedInputEvent>()
        }
        ///Creates a new event filter for the [`RoleAdminChanged`] event.
        pub fn RoleAdminChanged_filter(
            &self,
        ) -> alloy_contract::Event<&P, RoleAdminChanged, N> {
            self.event_filter::<RoleAdminChanged>()
        }
        ///Creates a new event filter for the [`RoleGranted`] event.
        pub fn RoleGranted_filter(&self) -> alloy_contract::Event<&P, RoleGranted, N> {
            self.event_filter::<RoleGranted>()
        }
        ///Creates a new event filter for the [`RoleRevoked`] event.
        pub fn RoleRevoked_filter(&self) -> alloy_contract::Event<&P, RoleRevoked, N> {
            self.event_filter::<RoleRevoked>()
        }
    }
}

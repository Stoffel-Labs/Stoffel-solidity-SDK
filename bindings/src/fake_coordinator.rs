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
    error ClientAlreadyReservedIndex(address client, uint256 i);
    error ECDSAInvalidSignature();
    error ECDSAInvalidSignatureLength(uint256 length);
    error ECDSAInvalidSignatureS(bytes32 s);
    error IndexAlreadyReserved(uint256 i, address reqClient, address resClient);
    error IndexNotReserved(address client, uint256 index);
    error NoIndicesReserved(address client);
    error NotAClient(address client);
    error NotAnExistingParty(address account);
    error NotAtRound(StoffelCoordinator.Round required, StoffelCoordinator.Round current);
    error NotEnoughMPCParties(uint256 current, uint256 required);
    error OutOfIndices(address client);
    error OwnableInvalidOwner(address owner);
    error OwnableUnauthorizedAccount(address account);
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
    event ReservedInputEvent(address client, uint256 reservedIndex);
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
    function owner() external view returns (address);
    function renounceOwnership() external;
    function renounceRole(bytes32 role, address account) external;
    function reserveInputMasks() external;
    function reserveMaskIndex(uint256 i) external;
    function resetAccessControl(uint256 t, address[] memory initialMpcNodes) external;
    function resetCoordinator(bytes32 stoffelProgramHash, uint256 t, address[] memory initialMpcNodes, uint256 nInputs) external;
    function resetInputManager(uint256 nIndicesToReserve, uint256 _t) external;
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
    "name": "reserveMaskIndex",
    "inputs": [
      {
        "name": "i",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
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
        "name": "_t",
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
    "name": "ClientAlreadyReservedIndex",
    "inputs": [
      {
        "name": "client",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "i",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "IndexAlreadyReserved",
    "inputs": [
      {
        "name": "i",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "reqClient",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "resClient",
        "type": "address",
        "internalType": "address"
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
    "name": "OutOfIndices",
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
    ///0x608060405234801561000f575f5ffd5b50604051615fd7380380615fd7833981810160405281019061003191906111ed565b83838383338184858561004a828261011c60201b60201c565b50505f600a8190555061006382826104de60201b60201c565b50505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16036100d5575f6040517f1e4fbdf70000000000000000000000000000000000000000000000000000000081526004016100cc919061127c565b60405180910390fd5b6100e4816107e760201b60201c565b5061010f84835f815181106100fc576100fb611295565b5b60200260200101516108aa60201b60201c565b5050505050505050611522565b5f600183600361012c91906112ef565b6101369190611330565b90508082511015610181578151816040517f3a236268000000000000000000000000000000000000000000000000000000008152600401610178929190611372565b60405180910390fd5b826002819055505f6101b87f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61092d60201b60201c565b90505f6101ea7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61095560201b60201c565b90505f5f90505b81811015610251576102437f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e8483815181106102305761022f611295565b5b602002602001015161097c60201b60201c565b5080806001019150506101f1565b505f6102827fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46961092d60201b60201c565b90505f6102b47fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46961095560201b60201c565b90505f5f90505b8181101561031b5761030d7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698483815181106102fa576102f9611295565b5b602002602001015161097c60201b60201c565b5080806001019150506102bb565b505f61034c7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c02523261092d60201b60201c565b90505f61037e7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c02523261095560201b60201c565b90505f5f90505b818110156103e5576103d77fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c0252328483815181106103c4576103c3611295565b5b602002602001015161097c60201b60201c565b508080600101915050610385565b505f5f90505b885181101561044c5761043e7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698a838151811061042b5761042a611295565b5b602002602001015161099560201b60201c565b5080806001019150506103eb565b506104977f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e895f8151811061048457610483611295565b5b602002602001015161099560201b60201c565b507fa2df7830e0bedef7b1116bf547b467b16b50b3bd23146c9e099878d14e89301a878a336040516104cb93929190611399565b60405180910390a1505050505050505050565b600754600a5f8282546104f19190611330565b92505081905550816007819055505f6008819055505f60098190555080600b819055505f6105447fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46961092d60201b60201c565b90505f6105767fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46961095560201b60201c565b90505f5f90505b6007548110156107a5575f60055f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff169050600c5f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f5f82015f9055600182015f9055505060065f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6106509190610f29565b5f5f90505b8381101561070e5760035f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206002015f8683815181106106b1576106b0611295565b5b602002602001015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81549060ff02191690558080600101915050610655565b5060035f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f5f82015f61075b9190610f35565b600182015f9055505060055f8381526020019081526020015f205f6101000a81549073ffffffffffffffffffffffffffffffffffffffff021916905550808060010191505061057d565b507ff7f0872382dff5e698b284e12084e4e7894f830216dd80cb4e909b593a58f935600754336040516107d99291906113ce565b60405180910390a150505050565b5f600d5f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905081600d5f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a35050565b81600e8190555042600f81905550436010819055505f60115f6101000a81548160ff021916908360068111156108e3576108e26113f5565b5b02179055507fdef1f08eb655f4a75f60bd6fd7e971112004abd846a612e46dab270770d24ca530600f54601054846040516109219493929190611422565b60405180910390a15050565b606061094e60015f8481526020019081526020015f206109ae60201b60201c565b9050919050565b5f61097560015f8481526020019081526020015f206109d360201b60201c565b9050919050565b5f61098d83836109ec60201b60201c565b905092915050565b5f6109a68383610a3560201b60201c565b905092915050565b60605f6109c2835f01610a7e60201b60201c565b905060608190508092505050919050565b5f6109e5825f01610ad760201b60201c565b9050919050565b5f5f6109fe8484610ae660201b60201c565b90508015610a2b57610a298360015f8781526020019081526020015f20610bdb60201b90919060201c565b505b8091505092915050565b5f5f610a478484610c0e60201b60201c565b90508015610a7457610a728360015f8781526020019081526020015f20610d0360201b90919060201c565b505b8091505092915050565b6060815f01805480602002602001604051908101604052809291908181526020018280548015610acb57602002820191905f5260205f20905b815481526020019060010190808311610ab7575b50505050509050919050565b5f815f01805490509050919050565b5f610af78383610d3660201b60201c565b15610bd1575f5f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550610b6e610d9960201b60201c565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b60405160405180910390a460019050610bd5565b5f90505b92915050565b5f610c06835f018373ffffffffffffffffffffffffffffffffffffffff165f1b610da060201b60201c565b905092915050565b5f610c1f8383610d3660201b60201c565b610cf95760015f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550610c96610d9960201b60201c565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a460019050610cfd565b5f90505b92915050565b5f610d2e835f018373ffffffffffffffffffffffffffffffffffffffff165f1b610e9c60201b60201c565b905092915050565b5f5f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f33905090565b5f5f836001015f8481526020019081526020015f205490505f8114610e91575f600182610dcd9190611465565b90505f6001865f0180549050610de39190611465565b9050808214610e49575f865f018281548110610e0257610e01611295565b5b905f5260205f200154905080875f018481548110610e2357610e22611295565b5b905f5260205f20018190555083876001015f8381526020019081526020015f2081905550505b855f01805480610e5c57610e5b611498565b5b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f905560019350505050610e96565b5f9150505b92915050565b5f610ead8383610f0960201b60201c565b610eff57825f0182908060018154018082558091505060019003905f5260205f20015f9091909190915055825f0180549050836001015f8481526020019081526020015f208190555060019050610f03565b5f90505b92915050565b5f5f836001015f8481526020019081526020015f20541415905092915050565b505f81556001015f9055565b5080545f8255905f5260205f2090610f4d9190610f50565b50565b5f5b80821115610f70578281015f610f689190610f75565b600101610f52565b505090565b508054610f81906114f2565b5f825580601f10610f925750610fac565b601f0160209004905f5260205f2090610fab9190610faf565b5b50565b5f5b80821115610fc7578281015f9055600101610fb1565b505090565b5f604051905090565b5f5ffd5b5f5ffd5b5f819050919050565b610fef81610fdd565b8114610ff9575f5ffd5b50565b5f8151905061100a81610fe6565b92915050565b5f819050919050565b61102281611010565b811461102c575f5ffd5b50565b5f8151905061103d81611019565b92915050565b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61108d82611047565b810181811067ffffffffffffffff821117156110ac576110ab611057565b5b80604052505050565b5f6110be610fcc565b90506110ca8282611084565b919050565b5f67ffffffffffffffff8211156110e9576110e8611057565b5b602082029050602081019050919050565b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f611127826110fe565b9050919050565b6111378161111d565b8114611141575f5ffd5b50565b5f815190506111528161112e565b92915050565b5f61116a611165846110cf565b6110b5565b9050808382526020820190506020840283018581111561118d5761118c6110fa565b5b835b818110156111b657806111a28882611144565b84526020840193505060208101905061118f565b5050509392505050565b5f82601f8301126111d4576111d3611043565b5b81516111e4848260208601611158565b91505092915050565b5f5f5f5f6080858703121561120557611204610fd5565b5b5f61121287828801610ffc565b94505060206112238782880161102f565b935050604085015167ffffffffffffffff81111561124457611243610fd9565b5b611250878288016111c0565b92505060606112618782880161102f565b91505092959194509250565b6112768161111d565b82525050565b5f60208201905061128f5f83018461126d565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6112f982611010565b915061130483611010565b925082820261131281611010565b91508282048414831517611329576113286112c2565b5b5092915050565b5f61133a82611010565b915061134583611010565b925082820190508082111561135d5761135c6112c2565b5b92915050565b61136c81611010565b82525050565b5f6040820190506113855f830185611363565b6113926020830184611363565b9392505050565b5f6060820190506113ac5f830186611363565b6113b96020830185611363565b6113c6604083018461126d565b949350505050565b5f6040820190506113e15f830185611363565b6113ee602083018461126d565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b5f6080820190506114355f83018761126d565b6114426020830186611363565b61144f6040830185611363565b61145c606083018461126d565b95945050505050565b5f61146f82611010565b915061147a83611010565b9250828203905081811115611492576114916112c2565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061150957607f821691505b60208210810361151c5761151b6114c5565b5b50919050565b614aa88061152f5f395ff3fe608060405234801561000f575f5ffd5b5060043610610225575f3560e01c8063715018a61161012e578063c079f495116100b6578063eae6f6521161007a578063eae6f652146105c9578063eb8575de146105e7578063f2fde38b14610603578063f6603c611461061f578063fc78b2e81461063b57610225565b8063c079f4951461054b578063ca15c87314610555578063cb9c4cc414610585578063d547741f1461058f578063d8270dce146105ab57610225565b806391d14854116100fd57806391d14854146104a7578063a217fddf146104d7578063a3246ad3146104f5578063af206f2814610525578063bb51fef01461054157610225565b8063715018a6146104315780637f35b5601461043b5780638da5cb5b146104595780639010d07c1461047757610225565b80632328bd12116101b157806333cc9a091161018057806333cc9a09146103db57806336568abe146103e55780633b4338d1146104015780634b8e64881461041d5780634bb278f31461042757610225565b80632328bd1214610353578063248a9ca3146103715780632f2ff15d146103a157806330104c3e146103bd57610225565b8063146ca531116101f8578063146ca531146102c157806317634514146102df5780631c7453db146102fd5780631ee4ee0f1461031b57806321dc7b9b1461033757610225565b806301ffc9a7146102295780630bda81cf146102595780630d42eb6f1461027557806313ff6dd514610291575b5f5ffd5b610243600480360381019061023e91906135d7565b61066b565b604051610250919061361c565b60405180910390f35b610273600480360381019061026e9190613668565b6106e4565b005b61028f600480360381019061028a9190613707565b610936565b005b6102ab60048036038101906102a691906137ac565b610977565b6040516102b8919061361c565b60405180910390f35b6102c96109f3565b6040516102d6919061384a565b60405180910390f35b6102e7610a05565b6040516102f49190613872565b60405180910390f35b610305610a0b565b6040516103129190613872565b60405180910390f35b6103356004803603810190610330919061388b565b610a11565b005b610351600480360381019061034c91906138e8565b610fbf565b005b61035b6113c6565b6040516103689190613872565b60405180910390f35b61038b60048036038101906103869190613946565b6113dc565b6040516103989190613980565b60405180910390f35b6103bb60048036038101906103b69190613999565b6113f8565b005b6103c5611432565b6040516103d29190613980565b60405180910390f35b6103e3611456565b005b6103ff60048036038101906103fa9190613999565b6114d0565b005b61041b60048036038101906104169190613668565b611597565b005b6104256115d0565b005b61042f61164a565b005b6104396116c4565b005b6104436116d7565b6040516104509190613980565b60405180910390f35b6104616116fb565b60405161046e91906139e6565b60405180910390f35b610491600480360381019061048c91906139ff565b611723565b60405161049e91906139e6565b60405180910390f35b6104c160048036038101906104bc9190613999565b61174f565b6040516104ce919061361c565b60405180910390f35b6104df6117b2565b6040516104ec9190613980565b60405180910390f35b61050f600480360381019061050a9190613946565b6117b8565b60405161051c9190613af4565b60405180910390f35b61053f600480360381019061053a9190613c5c565b6117da565b005b610549611813565b005b61055361188d565b005b61056f600480360381019061056a9190613946565b611907565b60405161057c9190613872565b60405180910390f35b61058d611928565b005b6105a960048036038101906105a49190613999565b6119a1565b005b6105b3611a94565b6040516105c09190613872565b60405180910390f35b6105d1611a9a565b6040516105de9190613980565b60405180910390f35b61060160048036038101906105fc919061388b565b611abe565b005b61061d600480360381019061061891906137ac565b612003565b005b61063960048036038101906106349190613cb6565b612087565b005b610655600480360381019061065091906137ac565b6120f0565b604051610662919061361c565b60405180910390f35b5f7f07effe0d000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191614806106dd57506106dc82612122565b5b9050919050565b7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c02523261070e8161219b565b3373ffffffffffffffffffffffffffffffffffffffff1660055f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16146107b05733826040517fffabbae70000000000000000000000000000000000000000000000000000000081526004016107a7929190613d36565b60405180910390fd5b5f83036107f457336040517f16923cea0000000000000000000000000000000000000000000000000000000081526004016107eb91906139e6565b60405180910390fd5b5f600c5f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20600101541461087857336040517f4f5fbfc300000000000000000000000000000000000000000000000000000000815260040161086f91906139e6565b60405180910390fd5b604051806040016040528083815260200184815250600c5f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f820151815f0155602082015181600101559050507fb89addd937f44f902c849596641837cd7af2fcecef22d2a7866fdc1ad9c0ae2e33848460405161091293929190613d5d565b60405180910390a160095f81548092919061092c90613dbf565b9190505550505050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6109608161219b565b82826004918261097192919061401e565b50505050565b5f610981826120f0565b6109c257816040517fabdce06a0000000000000000000000000000000000000000000000000000000081526004016109b991906139e6565b60405180910390fd5b6109ec7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e8361174f565b9050919050565b60115f9054906101000a900460ff1681565b60105481565b600a5481565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469610a3b8161219b565b610a657fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c0252328561174f565b610aa657836040517fa032ac6b000000000000000000000000000000000000000000000000000000008152600401610a9d91906139e6565b60405180910390fd5b5f60075490505f5f90505b600754811015610b33578573ffffffffffffffffffffffffffffffffffffffff1660055f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1603610b2657809150610b33565b8080600101915050610ab1565b506007548103610baa5760055f5f81526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040517f6faf9f05000000000000000000000000000000000000000000000000000000008152600401610ba191906139e6565b60405180910390fd5b5f81600a54610bb991906140eb565b90505f610beb82604051602001610bd09190613872565b604051602081830303815290604052805190602001206121af565b90505f610c3b8288888080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050506121e2565b90508773ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603610cd95760065f8973ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20600160028110610cc057610cbf61411e565b5b015f8154610ccd90613dbf565b91905081905550610d3d565b60065f8973ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f60028110610d2857610d2761411e565b5b015f8154610d3590613dbf565b919050819055505b6001600b54610d4c91906140eb565b60065f8a73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f60028110610d9b57610d9a61411e565b5b01541080610e0757506001600b54610db391906140eb565b60065f8a73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20600160028110610e0357610e0261411e565b5b0154105b610e46576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610e3d906141cb565b60405180910390fd5b6001600b54610e5591906140eb565b60065f8a73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f60028110610ea457610ea361411e565b5b015410610efe578773ffffffffffffffffffffffffffffffffffffffff167f407009200520f9f10584813c0b95441ab320f6b08d97ebdaaf1e824eedd9d7c35f604051610ef1919061361c565b60405180910390a2610fb5565b6001600b54610f0d91906140eb565b60065f8a73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20600160028110610f5d57610f5c61411e565b5b015410610fb4578773ffffffffffffffffffffffffffffffffffffffff167f407009200520f9f10584813c0b95441ab320f6b08d97ebdaaf1e824eedd9d7c36001604051610fab919061361c565b60405180910390a25b5b5050505050505050565b5f5f90505b60075481101561107d573373ffffffffffffffffffffffffffffffffffffffff1660055f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16036110705733816040517fc315a0f5000000000000000000000000000000000000000000000000000000008152600401611067929190613d36565b60405180910390fd5b8080600101915050610fc4565b505f73ffffffffffffffffffffffffffffffffffffffff1660055f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff161461115257803360055f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040517fa0b8c708000000000000000000000000000000000000000000000000000000008152600401611149939291906141e9565b60405180910390fd5b3360055f8381526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506111cb7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c0252323361220c565b505f60035f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f2090506001600b54600361121d919061421e565b61122791906140eb565b67ffffffffffffffff8111156112405761123f613b24565b5b60405190808252806020026020018201604052801561127357816020015b606081526020019060019003908161125e5790505b50815f0190816112839190614671565b505f81600101819055505f6112b77fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696117b8565b90505f6112e37fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469611907565b90505f5f90505b8181101561136f575f846002015f85848151811061130b5761130a61411e565b5b602002602001015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555080806001019150506112ea565b5060085f81548092919061138290613dbf565b91905055507fabde16b7a9192c31c6231b1539bad6fed77635de4c008718dbdcafb7b8363afe33856040516113b8929190613d36565b60405180910390a150505050565b5f6008546007546113d791906146d3565b905090565b5f5f5f8381526020019081526020015f20600101549050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6114228161219b565b61142c838361220c565b50505050565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46981565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6114808161219b565b600361148b8161221f565b7f20f55ed0c92f2bb1c8825488e1e3c98463d024b2a42dbd24838c3f75260f43e933426040516114bc929190613d36565b60405180910390a16114cc6122a8565b5050565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469821480156115055750611504828261174f565b5b15611589575f61151483611907565b90505f60016002546003611528919061421e565b61153291906140eb565b90508082036115865760018161154891906146d3565b816040517f3a23626800000000000000000000000000000000000000000000000000000000815260040161157d929190614706565b60405180910390fd5b50505b6115938282612311565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6115c18161219b565b6115cb838361238c565b505050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6115fa8161219b565b60046116058161221f565b7f301f8a3701f5b260197382dd7301078542144fe8fddd18083d6f6e09e4958a593342604051611636929190613d36565b60405180910390a16116466122a8565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6116748161219b565b600561167f8161221f565b7f24a873651d26fb5a462cb37a91071cdd4d09abeebfe0ed144329bed1cc359d0333426040516116b0929190613d36565b60405180910390a16116c06122a8565b5050565b6116cc612689565b6116d55f612710565b565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e81565b5f600d5f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b5f6117478260015f8681526020019081526020015f206127d390919063ffffffff16565b905092915050565b5f5f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f5f1b81565b60606117d360015f8481526020019081526020015f206127ea565b9050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6118048161219b565b61180e8383612809565b505050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61183d8161219b565b60026118488161221f565b7f60edf9bdc7c4ea007cae1a9bbd03e41e5bfccd7231a6ec383c2edd7800f0d20c3342604051611879929190613d36565b60405180910390a16118896122a8565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6118b78161219b565b60016118c28161221f565b7f67c4489f674d03c7d19a9e36735188de7c65e8d1e99eb3a2fd258a769eb14fff33426040516118f3929190613d36565b60405180910390a16119036122a8565b5050565b5f61192160015f8481526020019081526020015f20612b89565b9050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6119528161219b565b5f61195c8161221f565b7fbb709dea744f06d1b26e824deec2f7140c511266ee15d7a217838b31d8b0123d334260405161198d929190613d36565b60405180910390a161199d6122a8565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6119cb8161219b565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46983148015611a0057506119ff838361174f565b5b15611a84575f611a0f84611907565b90505f60016002546003611a23919061421e565b611a2d91906140eb565b9050808203611a8157600181611a4391906146d3565b816040517f3a236268000000000000000000000000000000000000000000000000000000008152600401611a78929190614706565b60405180910390fd5b50505b611a8e8383612b9c565b50505050565b600f5481565b7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c02523281565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469611ae88161219b565b611b127fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c0252328561174f565b611b5357836040517fa032ac6b000000000000000000000000000000000000000000000000000000008152600401611b4a91906139e6565b60405180910390fd5b5f60035f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f2060010154905060035f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206002015f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615611c635784336040517f08e55495000000000000000000000000000000000000000000000000000000008152600401611c5a92919061472d565b60405180910390fd5b6001600b546003611c74919061421e565b611c7e91906140eb565b8110611cbf576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611cb6906147c4565b60405180910390fd5b600160035f8773ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206002015f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550838360035f8873ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f018381548110611da557611da461411e565b5b905f5260205f20019182611dba92919061401e565b50600160035f8773ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206001015f828254611e0b91906140eb565b92505081905550600181611e1f91906140eb565b905060016002546002611e32919061421e565b611e3c91906140eb565b8110611ffc575f8167ffffffffffffffff811115611e5d57611e5c613b24565b5b604051908082528060200260200182016040528015611e9057816020015b6060815260200190600190039081611e7b5790505b5090505f5f90505b82811015611fab5760035f8873ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f018181548110611ef157611ef061411e565b5b905f5260205f20018054611f0490613e3d565b80601f0160208091040260200160405190810160405280929190818152602001828054611f3090613e3d565b8015611f7b5780601f10611f5257610100808354040283529160200191611f7b565b820191905f5260205f20905b815481529060010190602001808311611f5e57829003601f168201915b5050505050828281518110611f9357611f9261411e565b5b60200260200101819052508080600101915050611e98565b508573ffffffffffffffffffffffffffffffffffffffff167f23e91dbfae03758cb88d7f6252b5710afa53a19ffe0f4b4f75d7f2de0c5eabe982604051611ff291906148da565b60405180910390a2505b5050505050565b61200b612689565b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff160361207b575f6040517f1e4fbdf700000000000000000000000000000000000000000000000000000000815260040161207291906139e6565b60405180910390fd5b61208481612710565b50565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6120b18161219b565b6120bb8484612809565b6120c5828561238c565b6120e985845f815181106120dc576120db61411e565b5b6020026020010151612baf565b5050505050565b5f61211b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698361174f565b9050919050565b5f7f5a05180f000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19161480612194575061219382612c32565b5b9050919050565b6121ac816121a7612cab565b612cb2565b50565b5f7f19457468657265756d205369676e6564204d6573736167653a0a3332000000005f5281601c52603c5f209050919050565b5f5f5f5f6121f08686612d03565b9250925092506122008282612d58565b82935050505092915050565b5f6122178383612eba565b905092915050565b806006811115612232576122316137d7565b5b60115f9054906101000a900460ff166006811115612253576122526137d7565b5b146122a5578060115f9054906101000a900460ff166040517fbfa217d800000000000000000000000000000000000000000000000000000000815260040161229c9291906148fa565b60405180910390fd5b50565b600160115f9054906101000a900460ff1660068111156122cb576122ca6137d7565b5b6122d591906140eb565b60068111156122e7576122e66137d7565b5b60115f6101000a81548160ff0219169083600681111561230a576123096137d7565b5b0217905550565b612319612cab565b73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff161461237d576040517f6697b23200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6123878282612b9c565b505050565b600754600a5f82825461239f91906140eb565b92505081905550816007819055505f6008819055505f60098190555080600b819055505f6123ec7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696117b8565b90505f6124187fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469611907565b90505f5f90505b600754811015612647575f60055f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff169050600c5f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f5f82015f9055600182015f9055505060065f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6124f291906134ce565b5f5f90505b838110156125b05760035f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206002015f8683815181106125535761255261411e565b5b602002602001015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81549060ff021916905580806001019150506124f7565b5060035f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f5f82015f6125fd91906134da565b600182015f9055505060055f8381526020019081526020015f205f6101000a81549073ffffffffffffffffffffffffffffffffffffffff021916905550808060010191505061241f565b507ff7f0872382dff5e698b284e12084e4e7894f830216dd80cb4e909b593a58f9356007543360405161267b929190614921565b60405180910390a150505050565b612691612cab565b73ffffffffffffffffffffffffffffffffffffffff166126af6116fb565b73ffffffffffffffffffffffffffffffffffffffff161461270e576126d2612cab565b6040517f118cdaa700000000000000000000000000000000000000000000000000000000815260040161270591906139e6565b60405180910390fd5b565b5f600d5f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905081600d5f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a35050565b5f6127e0835f0183612efd565b5f1c905092915050565b60605f6127f8835f01612f24565b905060608190508092505050919050565b5f6001836003612819919061421e565b61282391906140eb565b9050808251101561286e578151816040517f3a236268000000000000000000000000000000000000000000000000000000008152600401612865929190614706565b60405180910390fd5b826002819055505f61289f7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6117b8565b90505f6128cb7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e611907565b90505f5f90505b8181101561292c5761291e7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e8483815181106129115761291061411e565b5b6020026020010151612b9c565b5080806001019150506128d2565b505f6129577fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696117b8565b90505f6129837fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469611907565b90505f5f90505b818110156129e4576129d67fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698483815181106129c9576129c861411e565b5b6020026020010151612b9c565b50808060010191505061298a565b505f612a0f7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c0252326117b8565b90505f612a3b7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c025232611907565b90505f5f90505b81811015612a9c57612a8e7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c025232848381518110612a8157612a8061411e565b5b6020026020010151612b9c565b508080600101915050612a42565b505f5f90505b8851811015612afd57612aef7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698a8381518110612ae257612ae161411e565b5b602002602001015161220c565b508080600101915050612aa2565b50612b427f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e895f81518110612b3557612b3461411e565b5b602002602001015161220c565b507fa2df7830e0bedef7b1116bf547b467b16b50b3bd23146c9e099878d14e89301a878a33604051612b7693929190614948565b60405180910390a1505050505050505050565b5f612b95825f01612f7d565b9050919050565b5f612ba78383612f8c565b905092915050565b81600e8190555042600f81905550436010819055505f60115f6101000a81548160ff02191690836006811115612be857612be76137d7565b5b02179055507fdef1f08eb655f4a75f60bd6fd7e971112004abd846a612e46dab270770d24ca530600f5460105484604051612c26949392919061497d565b60405180910390a15050565b5f7f7965db0b000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19161480612ca45750612ca382612fcf565b5b9050919050565b5f33905090565b612cbc828261174f565b612cff5780826040517fe2517d3f000000000000000000000000000000000000000000000000000000008152600401612cf69291906149c0565b60405180910390fd5b5050565b5f5f5f6041845103612d43575f5f5f602087015192506040870151915060608701515f1a9050612d3588828585613038565b955095509550505050612d51565b5f600285515f1b9250925092505b9250925092565b5f6003811115612d6b57612d6a6137d7565b5b826003811115612d7e57612d7d6137d7565b5b0315612eb65760016003811115612d9857612d976137d7565b5b826003811115612dab57612daa6137d7565b5b03612de2576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60026003811115612df657612df56137d7565b5b826003811115612e0957612e086137d7565b5b03612e4d57805f1c6040517ffce698f7000000000000000000000000000000000000000000000000000000008152600401612e449190613872565b60405180910390fd5b600380811115612e6057612e5f6137d7565b5b826003811115612e7357612e726137d7565b5b03612eb557806040517fd78bce0c000000000000000000000000000000000000000000000000000000008152600401612eac9190613980565b60405180910390fd5b5b5050565b5f5f612ec6848461311f565b90508015612ef357612ef18360015f8781526020019081526020015f2061320890919063ffffffff16565b505b8091505092915050565b5f825f018281548110612f1357612f1261411e565b5b905f5260205f200154905092915050565b6060815f01805480602002602001604051908101604052809291908181526020018280548015612f7157602002820191905f5260205f20905b815481526020019060010190808311612f5d575b50505050509050919050565b5f815f01805490509050919050565b5f5f612f988484613235565b90508015612fc557612fc38360015f8781526020019081526020015f2061331e90919063ffffffff16565b505b8091505092915050565b5f7f01ffc9a7000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916149050919050565b5f5f5f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c1115613074575f600385925092509250613115565b5f6001888888886040515f81526020016040526040516130979493929190614a02565b6020604051602081039080840390855afa1580156130b7573d5f5f3e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603613108575f60015f5f1b93509350935050613115565b805f5f5f1b935093509350505b9450945094915050565b5f61312a838361174f565b6131fe5760015f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555061319b612cab565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a460019050613202565b5f90505b92915050565b5f61322d835f018373ffffffffffffffffffffffffffffffffffffffff165f1b61334b565b905092915050565b5f613240838361174f565b15613314575f5f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055506132b1612cab565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b60405160405180910390a460019050613318565b5f90505b92915050565b5f613343835f018373ffffffffffffffffffffffffffffffffffffffff165f1b6133b2565b905092915050565b5f61335683836134ae565b6133a857825f0182908060018154018082558091505060019003905f5260205f20015f9091909190915055825f0180549050836001015f8481526020019081526020015f2081905550600190506133ac565b5f90505b92915050565b5f5f836001015f8481526020019081526020015f205490505f81146134a3575f6001826133df91906146d3565b90505f6001865f01805490506133f591906146d3565b905080821461345b575f865f0182815481106134145761341361411e565b5b905f5260205f200154905080875f0184815481106134355761343461411e565b5b905f5260205f20018190555083876001015f8381526020019081526020015f2081905550505b855f0180548061346e5761346d614a45565b5b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f9055600193505050506134a8565b5f9150505b92915050565b5f5f836001015f8481526020019081526020015f20541415905092915050565b505f81556001015f9055565b5080545f8255905f5260205f20906134f291906134f5565b50565b5f5b80821115613515578281015f61350d919061351a565b6001016134f7565b505090565b50805461352690613e3d565b5f825580601f106135375750613551565b601f0160209004905f5260205f20906135509190613554565b5b50565b5f5b8082111561356c578281015f9055600101613556565b505090565b5f604051905090565b5f5ffd5b5f5ffd5b5f7fffffffff0000000000000000000000000000000000000000000000000000000082169050919050565b6135b681613582565b81146135c0575f5ffd5b50565b5f813590506135d1816135ad565b92915050565b5f602082840312156135ec576135eb61357a565b5b5f6135f9848285016135c3565b91505092915050565b5f8115159050919050565b61361681613602565b82525050565b5f60208201905061362f5f83018461360d565b92915050565b5f819050919050565b61364781613635565b8114613651575f5ffd5b50565b5f813590506136628161363e565b92915050565b5f5f6040838503121561367e5761367d61357a565b5b5f61368b85828601613654565b925050602061369c85828601613654565b9150509250929050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f8401126136c7576136c66136a6565b5b8235905067ffffffffffffffff8111156136e4576136e36136aa565b5b602083019150836001820283011115613700576136ff6136ae565b5b9250929050565b5f5f6020838503121561371d5761371c61357a565b5b5f83013567ffffffffffffffff81111561373a5761373961357e565b5b613746858286016136b2565b92509250509250929050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61377b82613752565b9050919050565b61378b81613771565b8114613795575f5ffd5b50565b5f813590506137a681613782565b92915050565b5f602082840312156137c1576137c061357a565b5b5f6137ce84828501613798565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b60078110613815576138146137d7565b5b50565b5f81905061382582613804565b919050565b5f61383482613818565b9050919050565b6138448161382a565b82525050565b5f60208201905061385d5f83018461383b565b92915050565b61386c81613635565b82525050565b5f6020820190506138855f830184613863565b92915050565b5f5f5f604084860312156138a2576138a161357a565b5b5f6138af86828701613798565b935050602084013567ffffffffffffffff8111156138d0576138cf61357e565b5b6138dc868287016136b2565b92509250509250925092565b5f602082840312156138fd576138fc61357a565b5b5f61390a84828501613654565b91505092915050565b5f819050919050565b61392581613913565b811461392f575f5ffd5b50565b5f813590506139408161391c565b92915050565b5f6020828403121561395b5761395a61357a565b5b5f61396884828501613932565b91505092915050565b61397a81613913565b82525050565b5f6020820190506139935f830184613971565b92915050565b5f5f604083850312156139af576139ae61357a565b5b5f6139bc85828601613932565b92505060206139cd85828601613798565b9150509250929050565b6139e081613771565b82525050565b5f6020820190506139f95f8301846139d7565b92915050565b5f5f60408385031215613a1557613a1461357a565b5b5f613a2285828601613932565b9250506020613a3385828601613654565b9150509250929050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b613a6f81613771565b82525050565b5f613a808383613a66565b60208301905092915050565b5f602082019050919050565b5f613aa282613a3d565b613aac8185613a47565b9350613ab783613a57565b805f5b83811015613ae7578151613ace8882613a75565b9750613ad983613a8c565b925050600181019050613aba565b5085935050505092915050565b5f6020820190508181035f830152613b0c8184613a98565b905092915050565b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b613b5a82613b14565b810181811067ffffffffffffffff82111715613b7957613b78613b24565b5b80604052505050565b5f613b8b613571565b9050613b978282613b51565b919050565b5f67ffffffffffffffff821115613bb657613bb5613b24565b5b602082029050602081019050919050565b5f613bd9613bd484613b9c565b613b82565b90508083825260208201905060208402830185811115613bfc57613bfb6136ae565b5b835b81811015613c255780613c118882613798565b845260208401935050602081019050613bfe565b5050509392505050565b5f82601f830112613c4357613c426136a6565b5b8135613c53848260208601613bc7565b91505092915050565b5f5f60408385031215613c7257613c7161357a565b5b5f613c7f85828601613654565b925050602083013567ffffffffffffffff811115613ca057613c9f61357e565b5b613cac85828601613c2f565b9150509250929050565b5f5f5f5f60808587031215613cce57613ccd61357a565b5b5f613cdb87828801613932565b9450506020613cec87828801613654565b935050604085013567ffffffffffffffff811115613d0d57613d0c61357e565b5b613d1987828801613c2f565b9250506060613d2a87828801613654565b91505092959194509250565b5f604082019050613d495f8301856139d7565b613d566020830184613863565b9392505050565b5f606082019050613d705f8301866139d7565b613d7d6020830185613863565b613d8a6040830184613863565b949350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f613dc982613635565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203613dfb57613dfa613d92565b5b600182019050919050565b5f82905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f6002820490506001821680613e5457607f821691505b602082108103613e6757613e66613e10565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f60088302613ec97fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82613e8e565b613ed38683613e8e565b95508019841693508086168417925050509392505050565b5f819050919050565b5f613f0e613f09613f0484613635565b613eeb565b613635565b9050919050565b5f819050919050565b613f2783613ef4565b613f3b613f3382613f15565b848454613e9a565b825550505050565b5f5f905090565b613f52613f43565b613f5d818484613f1e565b505050565b5f5b82811015613f8357613f785f828401613f4a565b600181019050613f64565b505050565b601f821115613fd65782821115613fd557613fa281613e6d565b613fab83613e7f565b613fb485613e7f565b6020861015613fc1575f90505b808301613fd082840382613f62565b505050505b5b505050565b5f82821c905092915050565b5f613ff65f1984600802613fdb565b1980831691505092915050565b5f61400e8383613fe7565b9150826002028217905092915050565b6140288383613e06565b67ffffffffffffffff81111561404157614040613b24565b5b61404b8254613e3d565b614056828285613f88565b5f601f831160018114614083575f8415614071578287013590505b61407b8582614003565b8655506140e2565b601f19841661409186613e6d565b5f5b828110156140b857848901358255600182019150602085019450602081019050614093565b868310156140d557848901356140d1601f891682613fe7565b8355505b6001600288020188555050505b50505050505050565b5f6140f582613635565b915061410083613635565b925082820190508082111561411857614117613d92565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f82825260208201905092915050565b7f4255473a207468652061757468656e7469636174696f6e20766f7465732062795f8201527f20686f6e65737420636c69656e74732061726520696e636f6e73697374656e74602082015250565b5f6141b560408361414b565b91506141c08261415b565b604082019050919050565b5f6020820190508181035f8301526141e2816141a9565b9050919050565b5f6060820190506141fc5f830186613863565b61420960208301856139d7565b61421660408301846139d7565b949350505050565b5f61422882613635565b915061423383613635565b925082820261424181613635565b9150828204841483151761425857614257613d92565b5b5092915050565b5f819050602082019050919050565b5f81549050919050565b5f8190506001806001038301049050919050565b5f819050815f5260205f209050919050565b6142ce7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff83602003600802613fdb565b815481168255505050565b6142e281613e6d565b6142ed838254614003565b8083555f825550505050565b602084105f811461435457601f8411600181146143215761431a8685614003565b835561434e565b61432a83613e6d565b614342600161433888613e7f565b0360018301613f62565b61434c87856142d9565b505b506143ae565b61435d85613e7f565b61436685613e7f565b61436f84613e6d565b828101601f8916801561438a57614389816001840361429e565b5b8484111561439f5761439e85850383613f62565b5b60018a60020217875550505050505b5050505050565b680100000000000000008411156143cf576143ce613b24565b5b602083105f811461441857602085105f81146143f6576143ef8685614003565b8355614412565b8360ff191693508361440784613e6d565b556001866002020183555b50614422565b6001856002020182555b5050505050565b805461443481613e3d565b8084111561444957614448848284866143b5565b5b8084101561445e5761445d848284866142f9565b5b50505050565b82811015614483576144785f828401613f4a565b600181019050614464565b505050565b6144925f82614429565b50565b7f4e487b71000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b5f82146144d1576144d0614495565b5b6144da81614488565b5050565b5f5b828110156144ff576144f45f8284016144c1565b6001810190506144e0565b505050565b8183101561453b5761451582614278565b61451e84614278565b6145278361428c565b818101614536838503826144de565b505050505b505050565b6801000000000000000082111561455a57614559613b24565b5b6145638161426e565b828255614571838284614504565b505050565b5f81519050919050565b5f81519050919050565b5f81519050919050565b61459d8261458a565b67ffffffffffffffff8111156145b6576145b5613b24565b5b6145c08254613e3d565b6145cb828285613f88565b5f60209050601f8311600181146145fc575f84156145ea578287015190505b6145f48582614003565b86555061465b565b601f19841661460a86613e6d565b5f5b828110156146315784890151825560018201915060208501945060208101905061460c565b8683101561464e578489015161464a601f891682613fe7565b8355505b6001600288020188555050505b505050505050565b61466d8282614594565b5050565b61467a82614576565b6146848183614540565b61468d8361425f565b6146968361428c565b5f5b838110156146cb576146a983614580565b6146b38184614663565b60208401935060018301925050600181019050614698565b505050505050565b5f6146dd82613635565b91506146e883613635565b9250828203905081811115614700576146ff613d92565b5b92915050565b5f6040820190506147195f830185613863565b6147266020830184613863565b9392505050565b5f6040820190506147405f8301856139d7565b61474d60208301846139d7565b9392505050565b7f4255473a20414c5245414459205245434549564544205348415245532046524f5f8201527f4d204e20504152544945532c20544f4f204d414e5920434c49454e5453000000602082015250565b5f6147ae603d8361414b565b91506147b982614754565b604082019050919050565b5f6020820190508181035f8301526147db816147a2565b9050919050565b5f82825260208201905092915050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f61481a8261458a565b61482481856147f2565b9350614834818560208601614802565b61483d81613b14565b840191505092915050565b5f6148538383614810565b905092915050565b5f602082019050919050565b5f61487182614576565b61487b81856147e2565b93508360208202850161488d8561425f565b805f5b858110156148c857848403895281516148a98582614848565b94506148b48361485b565b925060208a01995050600181019050614890565b50829750879550505050505092915050565b5f6020820190508181035f8301526148f28184614867565b905092915050565b5f60408201905061490d5f83018561383b565b61491a602083018461383b565b9392505050565b5f6040820190506149345f830185613863565b61494160208301846139d7565b9392505050565b5f60608201905061495b5f830186613863565b6149686020830185613863565b61497560408301846139d7565b949350505050565b5f6080820190506149905f8301876139d7565b61499d6020830186613863565b6149aa6040830185613863565b6149b760608301846139d7565b95945050505050565b5f6040820190506149d35f8301856139d7565b6149e06020830184613971565b9392505050565b5f60ff82169050919050565b6149fc816149e7565b82525050565b5f608082019050614a155f830187613971565b614a2260208301866149f3565b614a2f6040830185613971565b614a3c6060830184613971565b95945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffdfea26469706673582212206a6f38d4605ef37f7117223bc9cfd00d4a200ab7971e775e818a102edbc2b14364736f6c63430008210033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa_\xD78\x03\x80a_\xD7\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\x11\xEDV[\x83\x83\x83\x833\x81\x84\x85\x85a\0J\x82\x82a\x01\x1C` \x1B` \x1CV[PP_`\n\x81\x90UPa\0c\x82\x82a\x04\xDE` \x1B` \x1CV[PP_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\0\xD5W_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\0\xCC\x91\x90a\x12|V[`@Q\x80\x91\x03\x90\xFD[a\0\xE4\x81a\x07\xE7` \x1B` \x1CV[Pa\x01\x0F\x84\x83_\x81Q\x81\x10a\0\xFCWa\0\xFBa\x12\x95V[[` \x02` \x01\x01Qa\x08\xAA` \x1B` \x1CV[PPPPPPPPa\x15\"V[_`\x01\x83`\x03a\x01,\x91\x90a\x12\xEFV[a\x016\x91\x90a\x130V[\x90P\x80\x82Q\x10\x15a\x01\x81W\x81Q\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x01x\x92\x91\x90a\x13rV[`@Q\x80\x91\x03\x90\xFD[\x82`\x02\x81\x90UP_a\x01\xB8\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\t-` \x1B` \x1CV[\x90P_a\x01\xEA\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\tU` \x1B` \x1CV[\x90P__\x90P[\x81\x81\x10\x15a\x02QWa\x02C\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x84\x83\x81Q\x81\x10a\x020Wa\x02/a\x12\x95V[[` \x02` \x01\x01Qa\t|` \x1B` \x1CV[P\x80\x80`\x01\x01\x91PPa\x01\xF1V[P_a\x02\x82\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\t-` \x1B` \x1CV[\x90P_a\x02\xB4\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\tU` \x1B` \x1CV[\x90P__\x90P[\x81\x81\x10\x15a\x03\x1BWa\x03\r\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x84\x83\x81Q\x81\x10a\x02\xFAWa\x02\xF9a\x12\x95V[[` \x02` \x01\x01Qa\t|` \x1B` \x1CV[P\x80\x80`\x01\x01\x91PPa\x02\xBBV[P_a\x03L\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2a\t-` \x1B` \x1CV[\x90P_a\x03~\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2a\tU` \x1B` \x1CV[\x90P__\x90P[\x81\x81\x10\x15a\x03\xE5Wa\x03\xD7\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2\x84\x83\x81Q\x81\x10a\x03\xC4Wa\x03\xC3a\x12\x95V[[` \x02` \x01\x01Qa\t|` \x1B` \x1CV[P\x80\x80`\x01\x01\x91PPa\x03\x85V[P__\x90P[\x88Q\x81\x10\x15a\x04LWa\x04>\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x8A\x83\x81Q\x81\x10a\x04+Wa\x04*a\x12\x95V[[` \x02` \x01\x01Qa\t\x95` \x1B` \x1CV[P\x80\x80`\x01\x01\x91PPa\x03\xEBV[Pa\x04\x97\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x89_\x81Q\x81\x10a\x04\x84Wa\x04\x83a\x12\x95V[[` \x02` \x01\x01Qa\t\x95` \x1B` \x1CV[P\x7F\xA2\xDFx0\xE0\xBE\xDE\xF7\xB1\x11k\xF5G\xB4g\xB1kP\xB3\xBD#\x14l\x9E\t\x98x\xD1N\x890\x1A\x87\x8A3`@Qa\x04\xCB\x93\x92\x91\x90a\x13\x99V[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPV[`\x07T`\n_\x82\x82Ta\x04\xF1\x91\x90a\x130V[\x92PP\x81\x90UP\x81`\x07\x81\x90UP_`\x08\x81\x90UP_`\t\x81\x90UP\x80`\x0B\x81\x90UP_a\x05D\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\t-` \x1B` \x1CV[\x90P_a\x05v\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\tU` \x1B` \x1CV[\x90P__\x90P[`\x07T\x81\x10\x15a\x07\xA5W_`\x05_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\x0C_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ __\x82\x01_\x90U`\x01\x82\x01_\x90UPP`\x06_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x06P\x91\x90a\x0F)V[__\x90P[\x83\x81\x10\x15a\x07\x0EW`\x03_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x02\x01_\x86\x83\x81Q\x81\x10a\x06\xB1Wa\x06\xB0a\x12\x95V[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U\x80\x80`\x01\x01\x91PPa\x06UV[P`\x03_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ __\x82\x01_a\x07[\x91\x90a\x0F5V[`\x01\x82\x01_\x90UPP`\x05_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90UP\x80\x80`\x01\x01\x91PPa\x05}V[P\x7F\xF7\xF0\x87#\x82\xDF\xF5\xE6\x98\xB2\x84\xE1 \x84\xE4\xE7\x89O\x83\x02\x16\xDD\x80\xCBN\x90\x9BY:X\xF95`\x07T3`@Qa\x07\xD9\x92\x91\x90a\x13\xCEV[`@Q\x80\x91\x03\x90\xA1PPPPV[_`\r_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\r_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[\x81`\x0E\x81\x90UPB`\x0F\x81\x90UPC`\x10\x81\x90UP_`\x11_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x06\x81\x11\x15a\x08\xE3Wa\x08\xE2a\x13\xF5V[[\x02\x17\x90UP\x7F\xDE\xF1\xF0\x8E\xB6U\xF4\xA7_`\xBDo\xD7\xE9q\x11 \x04\xAB\xD8F\xA6\x12\xE4m\xAB'\x07p\xD2L\xA50`\x0FT`\x10T\x84`@Qa\t!\x94\x93\x92\x91\x90a\x14\"V[`@Q\x80\x91\x03\x90\xA1PPV[``a\tN`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a\t\xAE` \x1B` \x1CV[\x90P\x91\x90PV[_a\tu`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a\t\xD3` \x1B` \x1CV[\x90P\x91\x90PV[_a\t\x8D\x83\x83a\t\xEC` \x1B` \x1CV[\x90P\x92\x91PPV[_a\t\xA6\x83\x83a\n5` \x1B` \x1CV[\x90P\x92\x91PPV[``_a\t\xC2\x83_\x01a\n~` \x1B` \x1CV[\x90P``\x81\x90P\x80\x92PPP\x91\x90PV[_a\t\xE5\x82_\x01a\n\xD7` \x1B` \x1CV[\x90P\x91\x90PV[__a\t\xFE\x84\x84a\n\xE6` \x1B` \x1CV[\x90P\x80\x15a\n+Wa\n)\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a\x0B\xDB` \x1B\x90\x91\x90` \x1CV[P[\x80\x91PP\x92\x91PPV[__a\nG\x84\x84a\x0C\x0E` \x1B` \x1CV[\x90P\x80\x15a\ntWa\nr\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a\r\x03` \x1B\x90\x91\x90` \x1CV[P[\x80\x91PP\x92\x91PPV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\n\xCBW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\n\xB7W[PPPPP\x90P\x91\x90PV[_\x81_\x01\x80T\x90P\x90P\x91\x90PV[_a\n\xF7\x83\x83a\r6` \x1B` \x1CV[\x15a\x0B\xD1W___\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x0Bna\r\x99` \x1B` \x1CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\x0B\xD5V[_\x90P[\x92\x91PPV[_a\x0C\x06\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba\r\xA0` \x1B` \x1CV[\x90P\x92\x91PPV[_a\x0C\x1F\x83\x83a\r6` \x1B` \x1CV[a\x0C\xF9W`\x01__\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x0C\x96a\r\x99` \x1B` \x1CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\x0C\xFDV[_\x90P[\x92\x91PPV[_a\r.\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba\x0E\x9C` \x1B` \x1CV[\x90P\x92\x91PPV[___\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[_3\x90P\x90V[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x14a\x0E\x91W_`\x01\x82a\r\xCD\x91\x90a\x14eV[\x90P_`\x01\x86_\x01\x80T\x90Pa\r\xE3\x91\x90a\x14eV[\x90P\x80\x82\x14a\x0EIW_\x86_\x01\x82\x81T\x81\x10a\x0E\x02Wa\x0E\x01a\x12\x95V[[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a\x0E#Wa\x0E\"a\x12\x95V[[\x90_R` _ \x01\x81\x90UP\x83\x87`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UPP[\x85_\x01\x80T\x80a\x0E\\Wa\x0E[a\x14\x98V[[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\x0E\x96V[_\x91PP[\x92\x91PPV[_a\x0E\xAD\x83\x83a\x0F\t` \x1B` \x1CV[a\x0E\xFFW\x82_\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x82_\x01\x80T\x90P\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP`\x01\x90Pa\x0F\x03V[_\x90P[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x14\x15\x90P\x92\x91PPV[P_\x81U`\x01\x01_\x90UV[P\x80T_\x82U\x90_R` _ \x90a\x0FM\x91\x90a\x0FPV[PV[_[\x80\x82\x11\x15a\x0FpW\x82\x81\x01_a\x0Fh\x91\x90a\x0FuV[`\x01\x01a\x0FRV[PP\x90V[P\x80Ta\x0F\x81\x90a\x14\xF2V[_\x82U\x80`\x1F\x10a\x0F\x92WPa\x0F\xACV[`\x1F\x01` \x90\x04\x90_R` _ \x90a\x0F\xAB\x91\x90a\x0F\xAFV[[PV[_[\x80\x82\x11\x15a\x0F\xC7W\x82\x81\x01_\x90U`\x01\x01a\x0F\xB1V[PP\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[_\x81\x90P\x91\x90PV[a\x0F\xEF\x81a\x0F\xDDV[\x81\x14a\x0F\xF9W__\xFD[PV[_\x81Q\x90Pa\x10\n\x81a\x0F\xE6V[\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x10\"\x81a\x10\x10V[\x81\x14a\x10,W__\xFD[PV[_\x81Q\x90Pa\x10=\x81a\x10\x19V[\x92\x91PPV[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x10\x8D\x82a\x10GV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x10\xACWa\x10\xABa\x10WV[[\x80`@RPPPV[_a\x10\xBEa\x0F\xCCV[\x90Pa\x10\xCA\x82\x82a\x10\x84V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x10\xE9Wa\x10\xE8a\x10WV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x11'\x82a\x10\xFEV[\x90P\x91\x90PV[a\x117\x81a\x11\x1DV[\x81\x14a\x11AW__\xFD[PV[_\x81Q\x90Pa\x11R\x81a\x11.V[\x92\x91PPV[_a\x11ja\x11e\x84a\x10\xCFV[a\x10\xB5V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x11\x8DWa\x11\x8Ca\x10\xFAV[[\x83[\x81\x81\x10\x15a\x11\xB6W\x80a\x11\xA2\x88\x82a\x11DV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x11\x8FV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x11\xD4Wa\x11\xD3a\x10CV[[\x81Qa\x11\xE4\x84\x82` \x86\x01a\x11XV[\x91PP\x92\x91PPV[____`\x80\x85\x87\x03\x12\x15a\x12\x05Wa\x12\x04a\x0F\xD5V[[_a\x12\x12\x87\x82\x88\x01a\x0F\xFCV[\x94PP` a\x12#\x87\x82\x88\x01a\x10/V[\x93PP`@\x85\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12DWa\x12Ca\x0F\xD9V[[a\x12P\x87\x82\x88\x01a\x11\xC0V[\x92PP``a\x12a\x87\x82\x88\x01a\x10/V[\x91PP\x92\x95\x91\x94P\x92PV[a\x12v\x81a\x11\x1DV[\x82RPPV[_` \x82\x01\x90Pa\x12\x8F_\x83\x01\x84a\x12mV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x12\xF9\x82a\x10\x10V[\x91Pa\x13\x04\x83a\x10\x10V[\x92P\x82\x82\x02a\x13\x12\x81a\x10\x10V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x13)Wa\x13(a\x12\xC2V[[P\x92\x91PPV[_a\x13:\x82a\x10\x10V[\x91Pa\x13E\x83a\x10\x10V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x13]Wa\x13\\a\x12\xC2V[[\x92\x91PPV[a\x13l\x81a\x10\x10V[\x82RPPV[_`@\x82\x01\x90Pa\x13\x85_\x83\x01\x85a\x13cV[a\x13\x92` \x83\x01\x84a\x13cV[\x93\x92PPPV[_``\x82\x01\x90Pa\x13\xAC_\x83\x01\x86a\x13cV[a\x13\xB9` \x83\x01\x85a\x13cV[a\x13\xC6`@\x83\x01\x84a\x12mV[\x94\x93PPPPV[_`@\x82\x01\x90Pa\x13\xE1_\x83\x01\x85a\x13cV[a\x13\xEE` \x83\x01\x84a\x12mV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_`\x80\x82\x01\x90Pa\x145_\x83\x01\x87a\x12mV[a\x14B` \x83\x01\x86a\x13cV[a\x14O`@\x83\x01\x85a\x13cV[a\x14\\``\x83\x01\x84a\x12mV[\x95\x94PPPPPV[_a\x14o\x82a\x10\x10V[\x91Pa\x14z\x83a\x10\x10V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x14\x92Wa\x14\x91a\x12\xC2V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a\x15\tW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x15\x1CWa\x15\x1Ba\x14\xC5V[[P\x91\x90PV[aJ\xA8\x80a\x15/_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02%W_5`\xE0\x1C\x80cqP\x18\xA6\x11a\x01.W\x80c\xC0y\xF4\x95\x11a\0\xB6W\x80c\xEA\xE6\xF6R\x11a\0zW\x80c\xEA\xE6\xF6R\x14a\x05\xC9W\x80c\xEB\x85u\xDE\x14a\x05\xE7W\x80c\xF2\xFD\xE3\x8B\x14a\x06\x03W\x80c\xF6`<a\x14a\x06\x1FW\x80c\xFCx\xB2\xE8\x14a\x06;Wa\x02%V[\x80c\xC0y\xF4\x95\x14a\x05KW\x80c\xCA\x15\xC8s\x14a\x05UW\x80c\xCB\x9CL\xC4\x14a\x05\x85W\x80c\xD5Gt\x1F\x14a\x05\x8FW\x80c\xD8'\r\xCE\x14a\x05\xABWa\x02%V[\x80c\x91\xD1HT\x11a\0\xFDW\x80c\x91\xD1HT\x14a\x04\xA7W\x80c\xA2\x17\xFD\xDF\x14a\x04\xD7W\x80c\xA3$j\xD3\x14a\x04\xF5W\x80c\xAF o(\x14a\x05%W\x80c\xBBQ\xFE\xF0\x14a\x05AWa\x02%V[\x80cqP\x18\xA6\x14a\x041W\x80c\x7F5\xB5`\x14a\x04;W\x80c\x8D\xA5\xCB[\x14a\x04YW\x80c\x90\x10\xD0|\x14a\x04wWa\x02%V[\x80c#(\xBD\x12\x11a\x01\xB1W\x80c3\xCC\x9A\t\x11a\x01\x80W\x80c3\xCC\x9A\t\x14a\x03\xDBW\x80c6V\x8A\xBE\x14a\x03\xE5W\x80c;C8\xD1\x14a\x04\x01W\x80cK\x8Ed\x88\x14a\x04\x1DW\x80cK\xB2x\xF3\x14a\x04'Wa\x02%V[\x80c#(\xBD\x12\x14a\x03SW\x80c$\x8A\x9C\xA3\x14a\x03qW\x80c//\xF1]\x14a\x03\xA1W\x80c0\x10L>\x14a\x03\xBDWa\x02%V[\x80c\x14l\xA51\x11a\x01\xF8W\x80c\x14l\xA51\x14a\x02\xC1W\x80c\x17cE\x14\x14a\x02\xDFW\x80c\x1CtS\xDB\x14a\x02\xFDW\x80c\x1E\xE4\xEE\x0F\x14a\x03\x1BW\x80c!\xDC{\x9B\x14a\x037Wa\x02%V[\x80c\x01\xFF\xC9\xA7\x14a\x02)W\x80c\x0B\xDA\x81\xCF\x14a\x02YW\x80c\rB\xEBo\x14a\x02uW\x80c\x13\xFFm\xD5\x14a\x02\x91W[__\xFD[a\x02C`\x04\x806\x03\x81\x01\x90a\x02>\x91\x90a5\xD7V[a\x06kV[`@Qa\x02P\x91\x90a6\x1CV[`@Q\x80\x91\x03\x90\xF3[a\x02s`\x04\x806\x03\x81\x01\x90a\x02n\x91\x90a6hV[a\x06\xE4V[\0[a\x02\x8F`\x04\x806\x03\x81\x01\x90a\x02\x8A\x91\x90a7\x07V[a\t6V[\0[a\x02\xAB`\x04\x806\x03\x81\x01\x90a\x02\xA6\x91\x90a7\xACV[a\twV[`@Qa\x02\xB8\x91\x90a6\x1CV[`@Q\x80\x91\x03\x90\xF3[a\x02\xC9a\t\xF3V[`@Qa\x02\xD6\x91\x90a8JV[`@Q\x80\x91\x03\x90\xF3[a\x02\xE7a\n\x05V[`@Qa\x02\xF4\x91\x90a8rV[`@Q\x80\x91\x03\x90\xF3[a\x03\x05a\n\x0BV[`@Qa\x03\x12\x91\x90a8rV[`@Q\x80\x91\x03\x90\xF3[a\x035`\x04\x806\x03\x81\x01\x90a\x030\x91\x90a8\x8BV[a\n\x11V[\0[a\x03Q`\x04\x806\x03\x81\x01\x90a\x03L\x91\x90a8\xE8V[a\x0F\xBFV[\0[a\x03[a\x13\xC6V[`@Qa\x03h\x91\x90a8rV[`@Q\x80\x91\x03\x90\xF3[a\x03\x8B`\x04\x806\x03\x81\x01\x90a\x03\x86\x91\x90a9FV[a\x13\xDCV[`@Qa\x03\x98\x91\x90a9\x80V[`@Q\x80\x91\x03\x90\xF3[a\x03\xBB`\x04\x806\x03\x81\x01\x90a\x03\xB6\x91\x90a9\x99V[a\x13\xF8V[\0[a\x03\xC5a\x142V[`@Qa\x03\xD2\x91\x90a9\x80V[`@Q\x80\x91\x03\x90\xF3[a\x03\xE3a\x14VV[\0[a\x03\xFF`\x04\x806\x03\x81\x01\x90a\x03\xFA\x91\x90a9\x99V[a\x14\xD0V[\0[a\x04\x1B`\x04\x806\x03\x81\x01\x90a\x04\x16\x91\x90a6hV[a\x15\x97V[\0[a\x04%a\x15\xD0V[\0[a\x04/a\x16JV[\0[a\x049a\x16\xC4V[\0[a\x04Ca\x16\xD7V[`@Qa\x04P\x91\x90a9\x80V[`@Q\x80\x91\x03\x90\xF3[a\x04aa\x16\xFBV[`@Qa\x04n\x91\x90a9\xE6V[`@Q\x80\x91\x03\x90\xF3[a\x04\x91`\x04\x806\x03\x81\x01\x90a\x04\x8C\x91\x90a9\xFFV[a\x17#V[`@Qa\x04\x9E\x91\x90a9\xE6V[`@Q\x80\x91\x03\x90\xF3[a\x04\xC1`\x04\x806\x03\x81\x01\x90a\x04\xBC\x91\x90a9\x99V[a\x17OV[`@Qa\x04\xCE\x91\x90a6\x1CV[`@Q\x80\x91\x03\x90\xF3[a\x04\xDFa\x17\xB2V[`@Qa\x04\xEC\x91\x90a9\x80V[`@Q\x80\x91\x03\x90\xF3[a\x05\x0F`\x04\x806\x03\x81\x01\x90a\x05\n\x91\x90a9FV[a\x17\xB8V[`@Qa\x05\x1C\x91\x90a:\xF4V[`@Q\x80\x91\x03\x90\xF3[a\x05?`\x04\x806\x03\x81\x01\x90a\x05:\x91\x90a<\\V[a\x17\xDAV[\0[a\x05Ia\x18\x13V[\0[a\x05Sa\x18\x8DV[\0[a\x05o`\x04\x806\x03\x81\x01\x90a\x05j\x91\x90a9FV[a\x19\x07V[`@Qa\x05|\x91\x90a8rV[`@Q\x80\x91\x03\x90\xF3[a\x05\x8Da\x19(V[\0[a\x05\xA9`\x04\x806\x03\x81\x01\x90a\x05\xA4\x91\x90a9\x99V[a\x19\xA1V[\0[a\x05\xB3a\x1A\x94V[`@Qa\x05\xC0\x91\x90a8rV[`@Q\x80\x91\x03\x90\xF3[a\x05\xD1a\x1A\x9AV[`@Qa\x05\xDE\x91\x90a9\x80V[`@Q\x80\x91\x03\x90\xF3[a\x06\x01`\x04\x806\x03\x81\x01\x90a\x05\xFC\x91\x90a8\x8BV[a\x1A\xBEV[\0[a\x06\x1D`\x04\x806\x03\x81\x01\x90a\x06\x18\x91\x90a7\xACV[a \x03V[\0[a\x069`\x04\x806\x03\x81\x01\x90a\x064\x91\x90a<\xB6V[a \x87V[\0[a\x06U`\x04\x806\x03\x81\x01\x90a\x06P\x91\x90a7\xACV[a \xF0V[`@Qa\x06b\x91\x90a6\x1CV[`@Q\x80\x91\x03\x90\xF3[_\x7F\x07\xEF\xFE\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x06\xDDWPa\x06\xDC\x82a!\"V[[\x90P\x91\x90PV[\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2a\x07\x0E\x81a!\x9BV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x05_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07\xB0W3\x82`@Q\x7F\xFF\xAB\xBA\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\xA7\x92\x91\x90a=6V[`@Q\x80\x91\x03\x90\xFD[_\x83\x03a\x07\xF4W3`@Q\x7F\x16\x92<\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\xEB\x91\x90a9\xE6V[`@Q\x80\x91\x03\x90\xFD[_`\x0C_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x14a\x08xW3`@Q\x7FO_\xBF\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08o\x91\x90a9\xE6V[`@Q\x80\x91\x03\x90\xFD[`@Q\x80`@\x01`@R\x80\x83\x81R` \x01\x84\x81RP`\x0C_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01U` \x82\x01Q\x81`\x01\x01U\x90PP\x7F\xB8\x9A\xDD\xD97\xF4O\x90,\x84\x95\x96d\x187\xCDz\xF2\xFC\xEC\xEF\"\xD2\xA7\x86o\xDC\x1A\xD9\xC0\xAE.3\x84\x84`@Qa\t\x12\x93\x92\x91\x90a=]V[`@Q\x80\x91\x03\x90\xA1`\t_\x81T\x80\x92\x91\x90a\t,\x90a=\xBFV[\x91\x90PUPPPPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\t`\x81a!\x9BV[\x82\x82`\x04\x91\x82a\tq\x92\x91\x90a@\x1EV[PPPPV[_a\t\x81\x82a \xF0V[a\t\xC2W\x81`@Q\x7F\xAB\xDC\xE0j\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xB9\x91\x90a9\xE6V[`@Q\x80\x91\x03\x90\xFD[a\t\xEC\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x83a\x17OV[\x90P\x91\x90PV[`\x11_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\x10T\x81V[`\nT\x81V[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\n;\x81a!\x9BV[a\ne\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2\x85a\x17OV[a\n\xA6W\x83`@Q\x7F\xA02\xACk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\x9D\x91\x90a9\xE6V[`@Q\x80\x91\x03\x90\xFD[_`\x07T\x90P__\x90P[`\x07T\x81\x10\x15a\x0B3W\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x05_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0B&W\x80\x91Pa\x0B3V[\x80\x80`\x01\x01\x91PPa\n\xB1V[P`\x07T\x81\x03a\x0B\xAAW`\x05__\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x7Fo\xAF\x9F\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\xA1\x91\x90a9\xE6V[`@Q\x80\x91\x03\x90\xFD[_\x81`\nTa\x0B\xB9\x91\x90a@\xEBV[\x90P_a\x0B\xEB\x82`@Q` \x01a\x0B\xD0\x91\x90a8rV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a!\xAFV[\x90P_a\x0C;\x82\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa!\xE2V[\x90P\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0C\xD9W`\x06_\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01`\x02\x81\x10a\x0C\xC0Wa\x0C\xBFaA\x1EV[[\x01_\x81Ta\x0C\xCD\x90a=\xBFV[\x91\x90P\x81\x90UPa\r=V[`\x06_\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _`\x02\x81\x10a\r(Wa\r'aA\x1EV[[\x01_\x81Ta\r5\x90a=\xBFV[\x91\x90P\x81\x90UP[`\x01`\x0BTa\rL\x91\x90a@\xEBV[`\x06_\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _`\x02\x81\x10a\r\x9BWa\r\x9AaA\x1EV[[\x01T\x10\x80a\x0E\x07WP`\x01`\x0BTa\r\xB3\x91\x90a@\xEBV[`\x06_\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01`\x02\x81\x10a\x0E\x03Wa\x0E\x02aA\x1EV[[\x01T\x10[a\x0EFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E=\x90aA\xCBV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x0BTa\x0EU\x91\x90a@\xEBV[`\x06_\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _`\x02\x81\x10a\x0E\xA4Wa\x0E\xA3aA\x1EV[[\x01T\x10a\x0E\xFEW\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F@p\t \x05 \xF9\xF1\x05\x84\x81<\x0B\x95D\x1A\xB3 \xF6\xB0\x8D\x97\xEB\xDA\xAF\x1E\x82N\xED\xD9\xD7\xC3_`@Qa\x0E\xF1\x91\x90a6\x1CV[`@Q\x80\x91\x03\x90\xA2a\x0F\xB5V[`\x01`\x0BTa\x0F\r\x91\x90a@\xEBV[`\x06_\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01`\x02\x81\x10a\x0F]Wa\x0F\\aA\x1EV[[\x01T\x10a\x0F\xB4W\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F@p\t \x05 \xF9\xF1\x05\x84\x81<\x0B\x95D\x1A\xB3 \xF6\xB0\x8D\x97\xEB\xDA\xAF\x1E\x82N\xED\xD9\xD7\xC3`\x01`@Qa\x0F\xAB\x91\x90a6\x1CV[`@Q\x80\x91\x03\x90\xA2[[PPPPPPPPV[__\x90P[`\x07T\x81\x10\x15a\x10}W3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x05_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x10pW3\x81`@Q\x7F\xC3\x15\xA0\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10g\x92\x91\x90a=6V[`@Q\x80\x91\x03\x90\xFD[\x80\x80`\x01\x01\x91PPa\x0F\xC4V[P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x05_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x11RW\x803`\x05_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x7F\xA0\xB8\xC7\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11I\x93\x92\x91\x90aA\xE9V[`@Q\x80\x91\x03\x90\xFD[3`\x05_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPa\x11\xCB\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R23a\"\x0CV[P_`\x03_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x90P`\x01`\x0BT`\x03a\x12\x1D\x91\x90aB\x1EV[a\x12'\x91\x90a@\xEBV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12@Wa\x12?a;$V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12sW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x12^W\x90P[P\x81_\x01\x90\x81a\x12\x83\x91\x90aFqV[P_\x81`\x01\x01\x81\x90UP_a\x12\xB7\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x17\xB8V[\x90P_a\x12\xE3\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x19\x07V[\x90P__\x90P[\x81\x81\x10\x15a\x13oW_\x84`\x02\x01_\x85\x84\x81Q\x81\x10a\x13\x0BWa\x13\naA\x1EV[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80\x80`\x01\x01\x91PPa\x12\xEAV[P`\x08_\x81T\x80\x92\x91\x90a\x13\x82\x90a=\xBFV[\x91\x90PUP\x7F\xAB\xDE\x16\xB7\xA9\x19,1\xC6#\x1B\x159\xBA\xD6\xFE\xD7v5\xDEL\0\x87\x18\xDB\xDC\xAF\xB7\xB86:\xFE3\x85`@Qa\x13\xB8\x92\x91\x90a=6V[`@Q\x80\x91\x03\x90\xA1PPPPV[_`\x08T`\x07Ta\x13\xD7\x91\x90aF\xD3V[\x90P\x90V[___\x83\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x14\"\x81a!\x9BV[a\x14,\x83\x83a\"\x0CV[PPPPV[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x81V[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x14\x80\x81a!\x9BV[`\x03a\x14\x8B\x81a\"\x1FV[\x7F \xF5^\xD0\xC9/+\xB1\xC8\x82T\x88\xE1\xE3\xC9\x84c\xD0$\xB2\xA4-\xBD$\x83\x8C?u&\x0FC\xE93B`@Qa\x14\xBC\x92\x91\x90a=6V[`@Q\x80\x91\x03\x90\xA1a\x14\xCCa\"\xA8V[PPV[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x82\x14\x80\x15a\x15\x05WPa\x15\x04\x82\x82a\x17OV[[\x15a\x15\x89W_a\x15\x14\x83a\x19\x07V[\x90P_`\x01`\x02T`\x03a\x15(\x91\x90aB\x1EV[a\x152\x91\x90a@\xEBV[\x90P\x80\x82\x03a\x15\x86W`\x01\x81a\x15H\x91\x90aF\xD3V[\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15}\x92\x91\x90aG\x06V[`@Q\x80\x91\x03\x90\xFD[PP[a\x15\x93\x82\x82a#\x11V[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x15\xC1\x81a!\x9BV[a\x15\xCB\x83\x83a#\x8CV[PPPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x15\xFA\x81a!\x9BV[`\x04a\x16\x05\x81a\"\x1FV[\x7F0\x1F\x8A7\x01\xF5\xB2`\x19s\x82\xDDs\x01\x07\x85B\x14O\xE8\xFD\xDD\x18\x08=on\t\xE4\x95\x8AY3B`@Qa\x166\x92\x91\x90a=6V[`@Q\x80\x91\x03\x90\xA1a\x16Fa\"\xA8V[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x16t\x81a!\x9BV[`\x05a\x16\x7F\x81a\"\x1FV[\x7F$\xA8se\x1D&\xFBZF,\xB3z\x91\x07\x1C\xDDM\t\xAB\xEE\xBF\xE0\xED\x14C)\xBE\xD1\xCC5\x9D\x033B`@Qa\x16\xB0\x92\x91\x90a=6V[`@Q\x80\x91\x03\x90\xA1a\x16\xC0a\"\xA8V[PPV[a\x16\xCCa&\x89V[a\x16\xD5_a'\x10V[V[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x81V[_`\r_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_a\x17G\x82`\x01_\x86\x81R` \x01\x90\x81R` \x01_ a'\xD3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[___\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[__\x1B\x81V[``a\x17\xD3`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a'\xEAV[\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x18\x04\x81a!\x9BV[a\x18\x0E\x83\x83a(\tV[PPPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x18=\x81a!\x9BV[`\x02a\x18H\x81a\"\x1FV[\x7F`\xED\xF9\xBD\xC7\xC4\xEA\0|\xAE\x1A\x9B\xBD\x03\xE4\x1E[\xFC\xCDr1\xA6\xEC8<.\xDDx\0\xF0\xD2\x0C3B`@Qa\x18y\x92\x91\x90a=6V[`@Q\x80\x91\x03\x90\xA1a\x18\x89a\"\xA8V[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x18\xB7\x81a!\x9BV[`\x01a\x18\xC2\x81a\"\x1FV[\x7Fg\xC4H\x9FgM\x03\xC7\xD1\x9A\x9E6sQ\x88\xDE|e\xE8\xD1\xE9\x9E\xB3\xA2\xFD%\x8Av\x9E\xB1O\xFF3B`@Qa\x18\xF3\x92\x91\x90a=6V[`@Q\x80\x91\x03\x90\xA1a\x19\x03a\"\xA8V[PPV[_a\x19!`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a+\x89V[\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x19R\x81a!\x9BV[_a\x19\\\x81a\"\x1FV[\x7F\xBBp\x9D\xEAtO\x06\xD1\xB2n\x82M\xEE\xC2\xF7\x14\x0CQ\x12f\xEE\x15\xD7\xA2\x17\x83\x8B1\xD8\xB0\x12=3B`@Qa\x19\x8D\x92\x91\x90a=6V[`@Q\x80\x91\x03\x90\xA1a\x19\x9Da\"\xA8V[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x19\xCB\x81a!\x9BV[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x83\x14\x80\x15a\x1A\0WPa\x19\xFF\x83\x83a\x17OV[[\x15a\x1A\x84W_a\x1A\x0F\x84a\x19\x07V[\x90P_`\x01`\x02T`\x03a\x1A#\x91\x90aB\x1EV[a\x1A-\x91\x90a@\xEBV[\x90P\x80\x82\x03a\x1A\x81W`\x01\x81a\x1AC\x91\x90aF\xD3V[\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1Ax\x92\x91\x90aG\x06V[`@Q\x80\x91\x03\x90\xFD[PP[a\x1A\x8E\x83\x83a+\x9CV[PPPPV[`\x0FT\x81V[\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2\x81V[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x1A\xE8\x81a!\x9BV[a\x1B\x12\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2\x85a\x17OV[a\x1BSW\x83`@Q\x7F\xA02\xACk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1BJ\x91\x90a9\xE6V[`@Q\x80\x91\x03\x90\xFD[_`\x03_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x90P`\x03_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x02\x01_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x1CcW\x843`@Q\x7F\x08\xE5T\x95\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1CZ\x92\x91\x90aG-V[`@Q\x80\x91\x03\x90\xFD[`\x01`\x0BT`\x03a\x1Ct\x91\x90aB\x1EV[a\x1C~\x91\x90a@\xEBV[\x81\x10a\x1C\xBFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C\xB6\x90aG\xC4V[`@Q\x80\x91\x03\x90\xFD[`\x01`\x03_\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x02\x01_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x83\x83`\x03_\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x01\x83\x81T\x81\x10a\x1D\xA5Wa\x1D\xA4aA\x1EV[[\x90_R` _ \x01\x91\x82a\x1D\xBA\x92\x91\x90a@\x1EV[P`\x01`\x03_\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x01_\x82\x82Ta\x1E\x0B\x91\x90a@\xEBV[\x92PP\x81\x90UP`\x01\x81a\x1E\x1F\x91\x90a@\xEBV[\x90P`\x01`\x02T`\x02a\x1E2\x91\x90aB\x1EV[a\x1E<\x91\x90a@\xEBV[\x81\x10a\x1F\xFCW_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E]Wa\x1E\\a;$V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\x90W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1E{W\x90P[P\x90P__\x90P[\x82\x81\x10\x15a\x1F\xABW`\x03_\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x01\x81\x81T\x81\x10a\x1E\xF1Wa\x1E\xF0aA\x1EV[[\x90_R` _ \x01\x80Ta\x1F\x04\x90a>=V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1F0\x90a>=V[\x80\x15a\x1F{W\x80`\x1F\x10a\x1FRWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1F{V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1F^W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x82\x82\x81Q\x81\x10a\x1F\x93Wa\x1F\x92aA\x1EV[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\x1E\x98V[P\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F#\xE9\x1D\xBF\xAE\x03u\x8C\xB8\x8D\x7FbR\xB5q\n\xFAS\xA1\x9F\xFE\x0FKOu\xD7\xF2\xDE\x0C^\xAB\xE9\x82`@Qa\x1F\xF2\x91\x90aH\xDAV[`@Q\x80\x91\x03\x90\xA2P[PPPPPV[a \x0Ba&\x89V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a {W_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a r\x91\x90a9\xE6V[`@Q\x80\x91\x03\x90\xFD[a \x84\x81a'\x10V[PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa \xB1\x81a!\x9BV[a \xBB\x84\x84a(\tV[a \xC5\x82\x85a#\x8CV[a \xE9\x85\x84_\x81Q\x81\x10a \xDCWa \xDBaA\x1EV[[` \x02` \x01\x01Qa+\xAFV[PPPPPV[_a!\x1B\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x83a\x17OV[\x90P\x91\x90PV[_\x7FZ\x05\x18\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a!\x94WPa!\x93\x82a,2V[[\x90P\x91\x90PV[a!\xAC\x81a!\xA7a,\xABV[a,\xB2V[PV[_\x7F\x19Ethereum Signed Message:\n32\0\0\0\0_R\x81`\x1CR`<_ \x90P\x91\x90PV[____a!\xF0\x86\x86a-\x03V[\x92P\x92P\x92Pa\"\0\x82\x82a-XV[\x82\x93PPPP\x92\x91PPV[_a\"\x17\x83\x83a.\xBAV[\x90P\x92\x91PPV[\x80`\x06\x81\x11\x15a\"2Wa\"1a7\xD7V[[`\x11_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x06\x81\x11\x15a\"SWa\"Ra7\xD7V[[\x14a\"\xA5W\x80`\x11_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x7F\xBF\xA2\x17\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\"\x9C\x92\x91\x90aH\xFAV[`@Q\x80\x91\x03\x90\xFD[PV[`\x01`\x11_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x06\x81\x11\x15a\"\xCBWa\"\xCAa7\xD7V[[a\"\xD5\x91\x90a@\xEBV[`\x06\x81\x11\x15a\"\xE7Wa\"\xE6a7\xD7V[[`\x11_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x06\x81\x11\x15a#\nWa#\ta7\xD7V[[\x02\x17\x90UPV[a#\x19a,\xABV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a#}W`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a#\x87\x82\x82a+\x9CV[PPPV[`\x07T`\n_\x82\x82Ta#\x9F\x91\x90a@\xEBV[\x92PP\x81\x90UP\x81`\x07\x81\x90UP_`\x08\x81\x90UP_`\t\x81\x90UP\x80`\x0B\x81\x90UP_a#\xEC\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x17\xB8V[\x90P_a$\x18\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x19\x07V[\x90P__\x90P[`\x07T\x81\x10\x15a&GW_`\x05_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\x0C_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ __\x82\x01_\x90U`\x01\x82\x01_\x90UPP`\x06_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a$\xF2\x91\x90a4\xCEV[__\x90P[\x83\x81\x10\x15a%\xB0W`\x03_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x02\x01_\x86\x83\x81Q\x81\x10a%SWa%RaA\x1EV[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U\x80\x80`\x01\x01\x91PPa$\xF7V[P`\x03_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ __\x82\x01_a%\xFD\x91\x90a4\xDAV[`\x01\x82\x01_\x90UPP`\x05_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90UP\x80\x80`\x01\x01\x91PPa$\x1FV[P\x7F\xF7\xF0\x87#\x82\xDF\xF5\xE6\x98\xB2\x84\xE1 \x84\xE4\xE7\x89O\x83\x02\x16\xDD\x80\xCBN\x90\x9BY:X\xF95`\x07T3`@Qa&{\x92\x91\x90aI!V[`@Q\x80\x91\x03\x90\xA1PPPPV[a&\x91a,\xABV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a&\xAFa\x16\xFBV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a'\x0EWa&\xD2a,\xABV[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a'\x05\x91\x90a9\xE6V[`@Q\x80\x91\x03\x90\xFD[V[_`\r_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\r_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[_a'\xE0\x83_\x01\x83a.\xFDV[_\x1C\x90P\x92\x91PPV[``_a'\xF8\x83_\x01a/$V[\x90P``\x81\x90P\x80\x92PPP\x91\x90PV[_`\x01\x83`\x03a(\x19\x91\x90aB\x1EV[a(#\x91\x90a@\xEBV[\x90P\x80\x82Q\x10\x15a(nW\x81Q\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a(e\x92\x91\x90aG\x06V[`@Q\x80\x91\x03\x90\xFD[\x82`\x02\x81\x90UP_a(\x9F\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x17\xB8V[\x90P_a(\xCB\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x19\x07V[\x90P__\x90P[\x81\x81\x10\x15a),Wa)\x1E\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x84\x83\x81Q\x81\x10a)\x11Wa)\x10aA\x1EV[[` \x02` \x01\x01Qa+\x9CV[P\x80\x80`\x01\x01\x91PPa(\xD2V[P_a)W\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x17\xB8V[\x90P_a)\x83\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x19\x07V[\x90P__\x90P[\x81\x81\x10\x15a)\xE4Wa)\xD6\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x84\x83\x81Q\x81\x10a)\xC9Wa)\xC8aA\x1EV[[` \x02` \x01\x01Qa+\x9CV[P\x80\x80`\x01\x01\x91PPa)\x8AV[P_a*\x0F\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2a\x17\xB8V[\x90P_a*;\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2a\x19\x07V[\x90P__\x90P[\x81\x81\x10\x15a*\x9CWa*\x8E\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2\x84\x83\x81Q\x81\x10a*\x81Wa*\x80aA\x1EV[[` \x02` \x01\x01Qa+\x9CV[P\x80\x80`\x01\x01\x91PPa*BV[P__\x90P[\x88Q\x81\x10\x15a*\xFDWa*\xEF\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x8A\x83\x81Q\x81\x10a*\xE2Wa*\xE1aA\x1EV[[` \x02` \x01\x01Qa\"\x0CV[P\x80\x80`\x01\x01\x91PPa*\xA2V[Pa+B\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x89_\x81Q\x81\x10a+5Wa+4aA\x1EV[[` \x02` \x01\x01Qa\"\x0CV[P\x7F\xA2\xDFx0\xE0\xBE\xDE\xF7\xB1\x11k\xF5G\xB4g\xB1kP\xB3\xBD#\x14l\x9E\t\x98x\xD1N\x890\x1A\x87\x8A3`@Qa+v\x93\x92\x91\x90aIHV[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPV[_a+\x95\x82_\x01a/}V[\x90P\x91\x90PV[_a+\xA7\x83\x83a/\x8CV[\x90P\x92\x91PPV[\x81`\x0E\x81\x90UPB`\x0F\x81\x90UPC`\x10\x81\x90UP_`\x11_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x06\x81\x11\x15a+\xE8Wa+\xE7a7\xD7V[[\x02\x17\x90UP\x7F\xDE\xF1\xF0\x8E\xB6U\xF4\xA7_`\xBDo\xD7\xE9q\x11 \x04\xAB\xD8F\xA6\x12\xE4m\xAB'\x07p\xD2L\xA50`\x0FT`\x10T\x84`@Qa,&\x94\x93\x92\x91\x90aI}V[`@Q\x80\x91\x03\x90\xA1PPV[_\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a,\xA4WPa,\xA3\x82a/\xCFV[[\x90P\x91\x90PV[_3\x90P\x90V[a,\xBC\x82\x82a\x17OV[a,\xFFW\x80\x82`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a,\xF6\x92\x91\x90aI\xC0V[`@Q\x80\x91\x03\x90\xFD[PPV[___`A\x84Q\x03a-CW___` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90Pa-5\x88\x82\x85\x85a08V[\x95P\x95P\x95PPPPa-QV[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15a-kWa-ja7\xD7V[[\x82`\x03\x81\x11\x15a-~Wa-}a7\xD7V[[\x03\x15a.\xB6W`\x01`\x03\x81\x11\x15a-\x98Wa-\x97a7\xD7V[[\x82`\x03\x81\x11\x15a-\xABWa-\xAAa7\xD7V[[\x03a-\xE2W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15a-\xF6Wa-\xF5a7\xD7V[[\x82`\x03\x81\x11\x15a.\tWa.\x08a7\xD7V[[\x03a.MW\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.D\x91\x90a8rV[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15a.`Wa._a7\xD7V[[\x82`\x03\x81\x11\x15a.sWa.ra7\xD7V[[\x03a.\xB5W\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.\xAC\x91\x90a9\x80V[`@Q\x80\x91\x03\x90\xFD[[PPV[__a.\xC6\x84\x84a1\x1FV[\x90P\x80\x15a.\xF3Wa.\xF1\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a2\x08\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[_\x82_\x01\x82\x81T\x81\x10a/\x13Wa/\x12aA\x1EV[[\x90_R` _ \x01T\x90P\x92\x91PPV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a/qW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a/]W[PPPPP\x90P\x91\x90PV[_\x81_\x01\x80T\x90P\x90P\x91\x90PV[__a/\x98\x84\x84a25V[\x90P\x80\x15a/\xC5Wa/\xC3\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a3\x1E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[_\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[___\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15a0tW_`\x03\x85\x92P\x92P\x92Pa1\x15V[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@Qa0\x97\x94\x93\x92\x91\x90aJ\x02V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a0\xB7W=__>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a1\x08W_`\x01__\x1B\x93P\x93P\x93PPa1\x15V[\x80___\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[_a1*\x83\x83a\x17OV[a1\xFEW`\x01__\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa1\x9Ba,\xABV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa2\x02V[_\x90P[\x92\x91PPV[_a2-\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba3KV[\x90P\x92\x91PPV[_a2@\x83\x83a\x17OV[\x15a3\x14W___\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa2\xB1a,\xABV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa3\x18V[_\x90P[\x92\x91PPV[_a3C\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba3\xB2V[\x90P\x92\x91PPV[_a3V\x83\x83a4\xAEV[a3\xA8W\x82_\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x82_\x01\x80T\x90P\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP`\x01\x90Pa3\xACV[_\x90P[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x14a4\xA3W_`\x01\x82a3\xDF\x91\x90aF\xD3V[\x90P_`\x01\x86_\x01\x80T\x90Pa3\xF5\x91\x90aF\xD3V[\x90P\x80\x82\x14a4[W_\x86_\x01\x82\x81T\x81\x10a4\x14Wa4\x13aA\x1EV[[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a45Wa44aA\x1EV[[\x90_R` _ \x01\x81\x90UP\x83\x87`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UPP[\x85_\x01\x80T\x80a4nWa4maJEV[[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa4\xA8V[_\x91PP[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x14\x15\x90P\x92\x91PPV[P_\x81U`\x01\x01_\x90UV[P\x80T_\x82U\x90_R` _ \x90a4\xF2\x91\x90a4\xF5V[PV[_[\x80\x82\x11\x15a5\x15W\x82\x81\x01_a5\r\x91\x90a5\x1AV[`\x01\x01a4\xF7V[PP\x90V[P\x80Ta5&\x90a>=V[_\x82U\x80`\x1F\x10a57WPa5QV[`\x1F\x01` \x90\x04\x90_R` _ \x90a5P\x91\x90a5TV[[PV[_[\x80\x82\x11\x15a5lW\x82\x81\x01_\x90U`\x01\x01a5VV[PP\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a5\xB6\x81a5\x82V[\x81\x14a5\xC0W__\xFD[PV[_\x815\x90Pa5\xD1\x81a5\xADV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a5\xECWa5\xEBa5zV[[_a5\xF9\x84\x82\x85\x01a5\xC3V[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a6\x16\x81a6\x02V[\x82RPPV[_` \x82\x01\x90Pa6/_\x83\x01\x84a6\rV[\x92\x91PPV[_\x81\x90P\x91\x90PV[a6G\x81a65V[\x81\x14a6QW__\xFD[PV[_\x815\x90Pa6b\x81a6>V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a6~Wa6}a5zV[[_a6\x8B\x85\x82\x86\x01a6TV[\x92PP` a6\x9C\x85\x82\x86\x01a6TV[\x91PP\x92P\x92\x90PV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a6\xC7Wa6\xC6a6\xA6V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\xE4Wa6\xE3a6\xAAV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a7\0Wa6\xFFa6\xAEV[[\x92P\x92\x90PV[__` \x83\x85\x03\x12\x15a7\x1DWa7\x1Ca5zV[[_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7:Wa79a5~V[[a7F\x85\x82\x86\x01a6\xB2V[\x92P\x92PP\x92P\x92\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a7{\x82a7RV[\x90P\x91\x90PV[a7\x8B\x81a7qV[\x81\x14a7\x95W__\xFD[PV[_\x815\x90Pa7\xA6\x81a7\x82V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a7\xC1Wa7\xC0a5zV[[_a7\xCE\x84\x82\x85\x01a7\x98V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x07\x81\x10a8\x15Wa8\x14a7\xD7V[[PV[_\x81\x90Pa8%\x82a8\x04V[\x91\x90PV[_a84\x82a8\x18V[\x90P\x91\x90PV[a8D\x81a8*V[\x82RPPV[_` \x82\x01\x90Pa8]_\x83\x01\x84a8;V[\x92\x91PPV[a8l\x81a65V[\x82RPPV[_` \x82\x01\x90Pa8\x85_\x83\x01\x84a8cV[\x92\x91PPV[___`@\x84\x86\x03\x12\x15a8\xA2Wa8\xA1a5zV[[_a8\xAF\x86\x82\x87\x01a7\x98V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8\xD0Wa8\xCFa5~V[[a8\xDC\x86\x82\x87\x01a6\xB2V[\x92P\x92PP\x92P\x92P\x92V[_` \x82\x84\x03\x12\x15a8\xFDWa8\xFCa5zV[[_a9\n\x84\x82\x85\x01a6TV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a9%\x81a9\x13V[\x81\x14a9/W__\xFD[PV[_\x815\x90Pa9@\x81a9\x1CV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a9[Wa9Za5zV[[_a9h\x84\x82\x85\x01a92V[\x91PP\x92\x91PPV[a9z\x81a9\x13V[\x82RPPV[_` \x82\x01\x90Pa9\x93_\x83\x01\x84a9qV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a9\xAFWa9\xAEa5zV[[_a9\xBC\x85\x82\x86\x01a92V[\x92PP` a9\xCD\x85\x82\x86\x01a7\x98V[\x91PP\x92P\x92\x90PV[a9\xE0\x81a7qV[\x82RPPV[_` \x82\x01\x90Pa9\xF9_\x83\x01\x84a9\xD7V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a:\x15Wa:\x14a5zV[[_a:\"\x85\x82\x86\x01a92V[\x92PP` a:3\x85\x82\x86\x01a6TV[\x91PP\x92P\x92\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a:o\x81a7qV[\x82RPPV[_a:\x80\x83\x83a:fV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a:\xA2\x82a:=V[a:\xAC\x81\x85a:GV[\x93Pa:\xB7\x83a:WV[\x80_[\x83\x81\x10\x15a:\xE7W\x81Qa:\xCE\x88\x82a:uV[\x97Pa:\xD9\x83a:\x8CV[\x92PP`\x01\x81\x01\x90Pa:\xBAV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra;\x0C\x81\x84a:\x98V[\x90P\x92\x91PPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a;Z\x82a;\x14V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a;yWa;xa;$V[[\x80`@RPPPV[_a;\x8Ba5qV[\x90Pa;\x97\x82\x82a;QV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a;\xB6Wa;\xB5a;$V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a;\xD9a;\xD4\x84a;\x9CV[a;\x82V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a;\xFCWa;\xFBa6\xAEV[[\x83[\x81\x81\x10\x15a<%W\x80a<\x11\x88\x82a7\x98V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa;\xFEV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a<CWa<Ba6\xA6V[[\x815a<S\x84\x82` \x86\x01a;\xC7V[\x91PP\x92\x91PPV[__`@\x83\x85\x03\x12\x15a<rWa<qa5zV[[_a<\x7F\x85\x82\x86\x01a6TV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<\xA0Wa<\x9Fa5~V[[a<\xAC\x85\x82\x86\x01a</V[\x91PP\x92P\x92\x90PV[____`\x80\x85\x87\x03\x12\x15a<\xCEWa<\xCDa5zV[[_a<\xDB\x87\x82\x88\x01a92V[\x94PP` a<\xEC\x87\x82\x88\x01a6TV[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=\rWa=\x0Ca5~V[[a=\x19\x87\x82\x88\x01a</V[\x92PP``a=*\x87\x82\x88\x01a6TV[\x91PP\x92\x95\x91\x94P\x92PV[_`@\x82\x01\x90Pa=I_\x83\x01\x85a9\xD7V[a=V` \x83\x01\x84a8cV[\x93\x92PPPV[_``\x82\x01\x90Pa=p_\x83\x01\x86a9\xD7V[a=}` \x83\x01\x85a8cV[a=\x8A`@\x83\x01\x84a8cV[\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a=\xC9\x82a65V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a=\xFBWa=\xFAa=\x92V[[`\x01\x82\x01\x90P\x91\x90PV[_\x82\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a>TW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a>gWa>fa>\x10V[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a>\xC9\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a>\x8EV[a>\xD3\x86\x83a>\x8EV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a?\x0Ea?\ta?\x04\x84a65V[a>\xEBV[a65V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a?'\x83a>\xF4V[a?;a?3\x82a?\x15V[\x84\x84Ta>\x9AV[\x82UPPPPV[__\x90P\x90V[a?Ra?CV[a?]\x81\x84\x84a?\x1EV[PPPV[_[\x82\x81\x10\x15a?\x83Wa?x_\x82\x84\x01a?JV[`\x01\x81\x01\x90Pa?dV[PPPV[`\x1F\x82\x11\x15a?\xD6W\x82\x82\x11\x15a?\xD5Wa?\xA2\x81a>mV[a?\xAB\x83a>\x7FV[a?\xB4\x85a>\x7FV[` \x86\x10\x15a?\xC1W_\x90P[\x80\x83\x01a?\xD0\x82\x84\x03\x82a?bV[PPPP[[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_a?\xF6_\x19\x84`\x08\x02a?\xDBV[\x19\x80\x83\x16\x91PP\x92\x91PPV[_a@\x0E\x83\x83a?\xE7V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a@(\x83\x83a>\x06V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a@AWa@@a;$V[[a@K\x82Ta>=V[a@V\x82\x82\x85a?\x88V[_`\x1F\x83\x11`\x01\x81\x14a@\x83W_\x84\x15a@qW\x82\x87\x015\x90P[a@{\x85\x82a@\x03V[\x86UPa@\xE2V[`\x1F\x19\x84\x16a@\x91\x86a>mV[_[\x82\x81\x10\x15a@\xB8W\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa@\x93V[\x86\x83\x10\x15a@\xD5W\x84\x89\x015a@\xD1`\x1F\x89\x16\x82a?\xE7V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[_a@\xF5\x82a65V[\x91PaA\0\x83a65V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15aA\x18WaA\x17a=\x92V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FBUG: the authentication votes by_\x82\x01R\x7F honest clients are inconsistent` \x82\x01RPV[_aA\xB5`@\x83aAKV[\x91PaA\xC0\x82aA[V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaA\xE2\x81aA\xA9V[\x90P\x91\x90PV[_``\x82\x01\x90PaA\xFC_\x83\x01\x86a8cV[aB\t` \x83\x01\x85a9\xD7V[aB\x16`@\x83\x01\x84a9\xD7V[\x94\x93PPPPV[_aB(\x82a65V[\x91PaB3\x83a65V[\x92P\x82\x82\x02aBA\x81a65V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17aBXWaBWa=\x92V[[P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x81T\x90P\x91\x90PV[_\x81\x90P`\x01\x80`\x01\x03\x83\x01\x04\x90P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[aB\xCE\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83` \x03`\x08\x02a?\xDBV[\x81T\x81\x16\x82UPPPV[aB\xE2\x81a>mV[aB\xED\x83\x82Ta@\x03V[\x80\x83U_\x82UPPPPV[` \x84\x10_\x81\x14aCTW`\x1F\x84\x11`\x01\x81\x14aC!WaC\x1A\x86\x85a@\x03V[\x83UaCNV[aC*\x83a>mV[aCB`\x01aC8\x88a>\x7FV[\x03`\x01\x83\x01a?bV[aCL\x87\x85aB\xD9V[P[PaC\xAEV[aC]\x85a>\x7FV[aCf\x85a>\x7FV[aCo\x84a>mV[\x82\x81\x01`\x1F\x89\x16\x80\x15aC\x8AWaC\x89\x81`\x01\x84\x03aB\x9EV[[\x84\x84\x11\x15aC\x9FWaC\x9E\x85\x85\x03\x83a?bV[[`\x01\x8A`\x02\x02\x17\x87UPPPPP[PPPPPV[h\x01\0\0\0\0\0\0\0\0\x84\x11\x15aC\xCFWaC\xCEa;$V[[` \x83\x10_\x81\x14aD\x18W` \x85\x10_\x81\x14aC\xF6WaC\xEF\x86\x85a@\x03V[\x83UaD\x12V[\x83`\xFF\x19\x16\x93P\x83aD\x07\x84a>mV[U`\x01\x86`\x02\x02\x01\x83U[PaD\"V[`\x01\x85`\x02\x02\x01\x82U[PPPPPV[\x80TaD4\x81a>=V[\x80\x84\x11\x15aDIWaDH\x84\x82\x84\x86aC\xB5V[[\x80\x84\x10\x15aD^WaD]\x84\x82\x84\x86aB\xF9V[[PPPPV[\x82\x81\x10\x15aD\x83WaDx_\x82\x84\x01a?JV[`\x01\x81\x01\x90PaDdV[PPPV[aD\x92_\x82aD)V[PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[_\x82\x14aD\xD1WaD\xD0aD\x95V[[aD\xDA\x81aD\x88V[PPV[_[\x82\x81\x10\x15aD\xFFWaD\xF4_\x82\x84\x01aD\xC1V[`\x01\x81\x01\x90PaD\xE0V[PPPV[\x81\x83\x10\x15aE;WaE\x15\x82aBxV[aE\x1E\x84aBxV[aE'\x83aB\x8CV[\x81\x81\x01aE6\x83\x85\x03\x82aD\xDEV[PPPP[PPPV[h\x01\0\0\0\0\0\0\0\0\x82\x11\x15aEZWaEYa;$V[[aEc\x81aBnV[\x82\x82UaEq\x83\x82\x84aE\x04V[PPPV[_\x81Q\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[aE\x9D\x82aE\x8AV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aE\xB6WaE\xB5a;$V[[aE\xC0\x82Ta>=V[aE\xCB\x82\x82\x85a?\x88V[_` \x90P`\x1F\x83\x11`\x01\x81\x14aE\xFCW_\x84\x15aE\xEAW\x82\x87\x01Q\x90P[aE\xF4\x85\x82a@\x03V[\x86UPaF[V[`\x1F\x19\x84\x16aF\n\x86a>mV[_[\x82\x81\x10\x15aF1W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaF\x0CV[\x86\x83\x10\x15aFNW\x84\x89\x01QaFJ`\x1F\x89\x16\x82a?\xE7V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[aFm\x82\x82aE\x94V[PPV[aFz\x82aEvV[aF\x84\x81\x83aE@V[aF\x8D\x83aB_V[aF\x96\x83aB\x8CV[_[\x83\x81\x10\x15aF\xCBWaF\xA9\x83aE\x80V[aF\xB3\x81\x84aFcV[` \x84\x01\x93P`\x01\x83\x01\x92PP`\x01\x81\x01\x90PaF\x98V[PPPPPPV[_aF\xDD\x82a65V[\x91PaF\xE8\x83a65V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15aG\0WaF\xFFa=\x92V[[\x92\x91PPV[_`@\x82\x01\x90PaG\x19_\x83\x01\x85a8cV[aG&` \x83\x01\x84a8cV[\x93\x92PPPV[_`@\x82\x01\x90PaG@_\x83\x01\x85a9\xD7V[aGM` \x83\x01\x84a9\xD7V[\x93\x92PPPV[\x7FBUG: ALREADY RECEIVED SHARES FRO_\x82\x01R\x7FM N PARTIES, TOO MANY CLIENTS\0\0\0` \x82\x01RPV[_aG\xAE`=\x83aAKV[\x91PaG\xB9\x82aGTV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaG\xDB\x81aG\xA2V[\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_aH\x1A\x82aE\x8AV[aH$\x81\x85aG\xF2V[\x93PaH4\x81\x85` \x86\x01aH\x02V[aH=\x81a;\x14V[\x84\x01\x91PP\x92\x91PPV[_aHS\x83\x83aH\x10V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aHq\x82aEvV[aH{\x81\x85aG\xE2V[\x93P\x83` \x82\x02\x85\x01aH\x8D\x85aB_V[\x80_[\x85\x81\x10\x15aH\xC8W\x84\x84\x03\x89R\x81QaH\xA9\x85\x82aHHV[\x94PaH\xB4\x83aH[V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaH\x90V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaH\xF2\x81\x84aHgV[\x90P\x92\x91PPV[_`@\x82\x01\x90PaI\r_\x83\x01\x85a8;V[aI\x1A` \x83\x01\x84a8;V[\x93\x92PPPV[_`@\x82\x01\x90PaI4_\x83\x01\x85a8cV[aIA` \x83\x01\x84a9\xD7V[\x93\x92PPPV[_``\x82\x01\x90PaI[_\x83\x01\x86a8cV[aIh` \x83\x01\x85a8cV[aIu`@\x83\x01\x84a9\xD7V[\x94\x93PPPPV[_`\x80\x82\x01\x90PaI\x90_\x83\x01\x87a9\xD7V[aI\x9D` \x83\x01\x86a8cV[aI\xAA`@\x83\x01\x85a8cV[aI\xB7``\x83\x01\x84a9\xD7V[\x95\x94PPPPPV[_`@\x82\x01\x90PaI\xD3_\x83\x01\x85a9\xD7V[aI\xE0` \x83\x01\x84a9qV[\x93\x92PPPV[_`\xFF\x82\x16\x90P\x91\x90PV[aI\xFC\x81aI\xE7V[\x82RPPV[_`\x80\x82\x01\x90PaJ\x15_\x83\x01\x87a9qV[aJ\"` \x83\x01\x86aI\xF3V[aJ/`@\x83\x01\x85a9qV[aJ<``\x83\x01\x84a9qV[\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 jo8\xD4`^\xF3\x7Fq\x17\";\xC9\xCF\xD0\rJ \n\xB7\x97\x1Ew^\x81\x8A\x10.\xDB\xC2\xB1CdsolcC\0\x08!\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610225575f3560e01c8063715018a61161012e578063c079f495116100b6578063eae6f6521161007a578063eae6f652146105c9578063eb8575de146105e7578063f2fde38b14610603578063f6603c611461061f578063fc78b2e81461063b57610225565b8063c079f4951461054b578063ca15c87314610555578063cb9c4cc414610585578063d547741f1461058f578063d8270dce146105ab57610225565b806391d14854116100fd57806391d14854146104a7578063a217fddf146104d7578063a3246ad3146104f5578063af206f2814610525578063bb51fef01461054157610225565b8063715018a6146104315780637f35b5601461043b5780638da5cb5b146104595780639010d07c1461047757610225565b80632328bd12116101b157806333cc9a091161018057806333cc9a09146103db57806336568abe146103e55780633b4338d1146104015780634b8e64881461041d5780634bb278f31461042757610225565b80632328bd1214610353578063248a9ca3146103715780632f2ff15d146103a157806330104c3e146103bd57610225565b8063146ca531116101f8578063146ca531146102c157806317634514146102df5780631c7453db146102fd5780631ee4ee0f1461031b57806321dc7b9b1461033757610225565b806301ffc9a7146102295780630bda81cf146102595780630d42eb6f1461027557806313ff6dd514610291575b5f5ffd5b610243600480360381019061023e91906135d7565b61066b565b604051610250919061361c565b60405180910390f35b610273600480360381019061026e9190613668565b6106e4565b005b61028f600480360381019061028a9190613707565b610936565b005b6102ab60048036038101906102a691906137ac565b610977565b6040516102b8919061361c565b60405180910390f35b6102c96109f3565b6040516102d6919061384a565b60405180910390f35b6102e7610a05565b6040516102f49190613872565b60405180910390f35b610305610a0b565b6040516103129190613872565b60405180910390f35b6103356004803603810190610330919061388b565b610a11565b005b610351600480360381019061034c91906138e8565b610fbf565b005b61035b6113c6565b6040516103689190613872565b60405180910390f35b61038b60048036038101906103869190613946565b6113dc565b6040516103989190613980565b60405180910390f35b6103bb60048036038101906103b69190613999565b6113f8565b005b6103c5611432565b6040516103d29190613980565b60405180910390f35b6103e3611456565b005b6103ff60048036038101906103fa9190613999565b6114d0565b005b61041b60048036038101906104169190613668565b611597565b005b6104256115d0565b005b61042f61164a565b005b6104396116c4565b005b6104436116d7565b6040516104509190613980565b60405180910390f35b6104616116fb565b60405161046e91906139e6565b60405180910390f35b610491600480360381019061048c91906139ff565b611723565b60405161049e91906139e6565b60405180910390f35b6104c160048036038101906104bc9190613999565b61174f565b6040516104ce919061361c565b60405180910390f35b6104df6117b2565b6040516104ec9190613980565b60405180910390f35b61050f600480360381019061050a9190613946565b6117b8565b60405161051c9190613af4565b60405180910390f35b61053f600480360381019061053a9190613c5c565b6117da565b005b610549611813565b005b61055361188d565b005b61056f600480360381019061056a9190613946565b611907565b60405161057c9190613872565b60405180910390f35b61058d611928565b005b6105a960048036038101906105a49190613999565b6119a1565b005b6105b3611a94565b6040516105c09190613872565b60405180910390f35b6105d1611a9a565b6040516105de9190613980565b60405180910390f35b61060160048036038101906105fc919061388b565b611abe565b005b61061d600480360381019061061891906137ac565b612003565b005b61063960048036038101906106349190613cb6565b612087565b005b610655600480360381019061065091906137ac565b6120f0565b604051610662919061361c565b60405180910390f35b5f7f07effe0d000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191614806106dd57506106dc82612122565b5b9050919050565b7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c02523261070e8161219b565b3373ffffffffffffffffffffffffffffffffffffffff1660055f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16146107b05733826040517fffabbae70000000000000000000000000000000000000000000000000000000081526004016107a7929190613d36565b60405180910390fd5b5f83036107f457336040517f16923cea0000000000000000000000000000000000000000000000000000000081526004016107eb91906139e6565b60405180910390fd5b5f600c5f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20600101541461087857336040517f4f5fbfc300000000000000000000000000000000000000000000000000000000815260040161086f91906139e6565b60405180910390fd5b604051806040016040528083815260200184815250600c5f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f820151815f0155602082015181600101559050507fb89addd937f44f902c849596641837cd7af2fcecef22d2a7866fdc1ad9c0ae2e33848460405161091293929190613d5d565b60405180910390a160095f81548092919061092c90613dbf565b9190505550505050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6109608161219b565b82826004918261097192919061401e565b50505050565b5f610981826120f0565b6109c257816040517fabdce06a0000000000000000000000000000000000000000000000000000000081526004016109b991906139e6565b60405180910390fd5b6109ec7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e8361174f565b9050919050565b60115f9054906101000a900460ff1681565b60105481565b600a5481565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469610a3b8161219b565b610a657fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c0252328561174f565b610aa657836040517fa032ac6b000000000000000000000000000000000000000000000000000000008152600401610a9d91906139e6565b60405180910390fd5b5f60075490505f5f90505b600754811015610b33578573ffffffffffffffffffffffffffffffffffffffff1660055f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1603610b2657809150610b33565b8080600101915050610ab1565b506007548103610baa5760055f5f81526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040517f6faf9f05000000000000000000000000000000000000000000000000000000008152600401610ba191906139e6565b60405180910390fd5b5f81600a54610bb991906140eb565b90505f610beb82604051602001610bd09190613872565b604051602081830303815290604052805190602001206121af565b90505f610c3b8288888080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050506121e2565b90508773ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603610cd95760065f8973ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20600160028110610cc057610cbf61411e565b5b015f8154610ccd90613dbf565b91905081905550610d3d565b60065f8973ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f60028110610d2857610d2761411e565b5b015f8154610d3590613dbf565b919050819055505b6001600b54610d4c91906140eb565b60065f8a73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f60028110610d9b57610d9a61411e565b5b01541080610e0757506001600b54610db391906140eb565b60065f8a73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20600160028110610e0357610e0261411e565b5b0154105b610e46576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610e3d906141cb565b60405180910390fd5b6001600b54610e5591906140eb565b60065f8a73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f60028110610ea457610ea361411e565b5b015410610efe578773ffffffffffffffffffffffffffffffffffffffff167f407009200520f9f10584813c0b95441ab320f6b08d97ebdaaf1e824eedd9d7c35f604051610ef1919061361c565b60405180910390a2610fb5565b6001600b54610f0d91906140eb565b60065f8a73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20600160028110610f5d57610f5c61411e565b5b015410610fb4578773ffffffffffffffffffffffffffffffffffffffff167f407009200520f9f10584813c0b95441ab320f6b08d97ebdaaf1e824eedd9d7c36001604051610fab919061361c565b60405180910390a25b5b5050505050505050565b5f5f90505b60075481101561107d573373ffffffffffffffffffffffffffffffffffffffff1660055f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16036110705733816040517fc315a0f5000000000000000000000000000000000000000000000000000000008152600401611067929190613d36565b60405180910390fd5b8080600101915050610fc4565b505f73ffffffffffffffffffffffffffffffffffffffff1660055f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff161461115257803360055f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040517fa0b8c708000000000000000000000000000000000000000000000000000000008152600401611149939291906141e9565b60405180910390fd5b3360055f8381526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506111cb7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c0252323361220c565b505f60035f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f2090506001600b54600361121d919061421e565b61122791906140eb565b67ffffffffffffffff8111156112405761123f613b24565b5b60405190808252806020026020018201604052801561127357816020015b606081526020019060019003908161125e5790505b50815f0190816112839190614671565b505f81600101819055505f6112b77fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696117b8565b90505f6112e37fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469611907565b90505f5f90505b8181101561136f575f846002015f85848151811061130b5761130a61411e565b5b602002602001015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555080806001019150506112ea565b5060085f81548092919061138290613dbf565b91905055507fabde16b7a9192c31c6231b1539bad6fed77635de4c008718dbdcafb7b8363afe33856040516113b8929190613d36565b60405180910390a150505050565b5f6008546007546113d791906146d3565b905090565b5f5f5f8381526020019081526020015f20600101549050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6114228161219b565b61142c838361220c565b50505050565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46981565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6114808161219b565b600361148b8161221f565b7f20f55ed0c92f2bb1c8825488e1e3c98463d024b2a42dbd24838c3f75260f43e933426040516114bc929190613d36565b60405180910390a16114cc6122a8565b5050565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469821480156115055750611504828261174f565b5b15611589575f61151483611907565b90505f60016002546003611528919061421e565b61153291906140eb565b90508082036115865760018161154891906146d3565b816040517f3a23626800000000000000000000000000000000000000000000000000000000815260040161157d929190614706565b60405180910390fd5b50505b6115938282612311565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6115c18161219b565b6115cb838361238c565b505050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6115fa8161219b565b60046116058161221f565b7f301f8a3701f5b260197382dd7301078542144fe8fddd18083d6f6e09e4958a593342604051611636929190613d36565b60405180910390a16116466122a8565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6116748161219b565b600561167f8161221f565b7f24a873651d26fb5a462cb37a91071cdd4d09abeebfe0ed144329bed1cc359d0333426040516116b0929190613d36565b60405180910390a16116c06122a8565b5050565b6116cc612689565b6116d55f612710565b565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e81565b5f600d5f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b5f6117478260015f8681526020019081526020015f206127d390919063ffffffff16565b905092915050565b5f5f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f5f1b81565b60606117d360015f8481526020019081526020015f206127ea565b9050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6118048161219b565b61180e8383612809565b505050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61183d8161219b565b60026118488161221f565b7f60edf9bdc7c4ea007cae1a9bbd03e41e5bfccd7231a6ec383c2edd7800f0d20c3342604051611879929190613d36565b60405180910390a16118896122a8565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6118b78161219b565b60016118c28161221f565b7f67c4489f674d03c7d19a9e36735188de7c65e8d1e99eb3a2fd258a769eb14fff33426040516118f3929190613d36565b60405180910390a16119036122a8565b5050565b5f61192160015f8481526020019081526020015f20612b89565b9050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6119528161219b565b5f61195c8161221f565b7fbb709dea744f06d1b26e824deec2f7140c511266ee15d7a217838b31d8b0123d334260405161198d929190613d36565b60405180910390a161199d6122a8565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6119cb8161219b565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46983148015611a0057506119ff838361174f565b5b15611a84575f611a0f84611907565b90505f60016002546003611a23919061421e565b611a2d91906140eb565b9050808203611a8157600181611a4391906146d3565b816040517f3a236268000000000000000000000000000000000000000000000000000000008152600401611a78929190614706565b60405180910390fd5b50505b611a8e8383612b9c565b50505050565b600f5481565b7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c02523281565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469611ae88161219b565b611b127fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c0252328561174f565b611b5357836040517fa032ac6b000000000000000000000000000000000000000000000000000000008152600401611b4a91906139e6565b60405180910390fd5b5f60035f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f2060010154905060035f8673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206002015f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615611c635784336040517f08e55495000000000000000000000000000000000000000000000000000000008152600401611c5a92919061472d565b60405180910390fd5b6001600b546003611c74919061421e565b611c7e91906140eb565b8110611cbf576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611cb6906147c4565b60405180910390fd5b600160035f8773ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206002015f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550838360035f8873ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f018381548110611da557611da461411e565b5b905f5260205f20019182611dba92919061401e565b50600160035f8773ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206001015f828254611e0b91906140eb565b92505081905550600181611e1f91906140eb565b905060016002546002611e32919061421e565b611e3c91906140eb565b8110611ffc575f8167ffffffffffffffff811115611e5d57611e5c613b24565b5b604051908082528060200260200182016040528015611e9057816020015b6060815260200190600190039081611e7b5790505b5090505f5f90505b82811015611fab5760035f8873ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f018181548110611ef157611ef061411e565b5b905f5260205f20018054611f0490613e3d565b80601f0160208091040260200160405190810160405280929190818152602001828054611f3090613e3d565b8015611f7b5780601f10611f5257610100808354040283529160200191611f7b565b820191905f5260205f20905b815481529060010190602001808311611f5e57829003601f168201915b5050505050828281518110611f9357611f9261411e565b5b60200260200101819052508080600101915050611e98565b508573ffffffffffffffffffffffffffffffffffffffff167f23e91dbfae03758cb88d7f6252b5710afa53a19ffe0f4b4f75d7f2de0c5eabe982604051611ff291906148da565b60405180910390a2505b5050505050565b61200b612689565b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff160361207b575f6040517f1e4fbdf700000000000000000000000000000000000000000000000000000000815260040161207291906139e6565b60405180910390fd5b61208481612710565b50565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6120b18161219b565b6120bb8484612809565b6120c5828561238c565b6120e985845f815181106120dc576120db61411e565b5b6020026020010151612baf565b5050505050565b5f61211b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698361174f565b9050919050565b5f7f5a05180f000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19161480612194575061219382612c32565b5b9050919050565b6121ac816121a7612cab565b612cb2565b50565b5f7f19457468657265756d205369676e6564204d6573736167653a0a3332000000005f5281601c52603c5f209050919050565b5f5f5f5f6121f08686612d03565b9250925092506122008282612d58565b82935050505092915050565b5f6122178383612eba565b905092915050565b806006811115612232576122316137d7565b5b60115f9054906101000a900460ff166006811115612253576122526137d7565b5b146122a5578060115f9054906101000a900460ff166040517fbfa217d800000000000000000000000000000000000000000000000000000000815260040161229c9291906148fa565b60405180910390fd5b50565b600160115f9054906101000a900460ff1660068111156122cb576122ca6137d7565b5b6122d591906140eb565b60068111156122e7576122e66137d7565b5b60115f6101000a81548160ff0219169083600681111561230a576123096137d7565b5b0217905550565b612319612cab565b73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff161461237d576040517f6697b23200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6123878282612b9c565b505050565b600754600a5f82825461239f91906140eb565b92505081905550816007819055505f6008819055505f60098190555080600b819055505f6123ec7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696117b8565b90505f6124187fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469611907565b90505f5f90505b600754811015612647575f60055f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff169050600c5f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f5f82015f9055600182015f9055505060065f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6124f291906134ce565b5f5f90505b838110156125b05760035f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206002015f8683815181106125535761255261411e565b5b602002602001015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81549060ff021916905580806001019150506124f7565b5060035f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f5f82015f6125fd91906134da565b600182015f9055505060055f8381526020019081526020015f205f6101000a81549073ffffffffffffffffffffffffffffffffffffffff021916905550808060010191505061241f565b507ff7f0872382dff5e698b284e12084e4e7894f830216dd80cb4e909b593a58f9356007543360405161267b929190614921565b60405180910390a150505050565b612691612cab565b73ffffffffffffffffffffffffffffffffffffffff166126af6116fb565b73ffffffffffffffffffffffffffffffffffffffff161461270e576126d2612cab565b6040517f118cdaa700000000000000000000000000000000000000000000000000000000815260040161270591906139e6565b60405180910390fd5b565b5f600d5f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905081600d5f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a35050565b5f6127e0835f0183612efd565b5f1c905092915050565b60605f6127f8835f01612f24565b905060608190508092505050919050565b5f6001836003612819919061421e565b61282391906140eb565b9050808251101561286e578151816040517f3a236268000000000000000000000000000000000000000000000000000000008152600401612865929190614706565b60405180910390fd5b826002819055505f61289f7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6117b8565b90505f6128cb7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e611907565b90505f5f90505b8181101561292c5761291e7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e8483815181106129115761291061411e565b5b6020026020010151612b9c565b5080806001019150506128d2565b505f6129577fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696117b8565b90505f6129837fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469611907565b90505f5f90505b818110156129e4576129d67fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698483815181106129c9576129c861411e565b5b6020026020010151612b9c565b50808060010191505061298a565b505f612a0f7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c0252326117b8565b90505f612a3b7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c025232611907565b90505f5f90505b81811015612a9c57612a8e7fa5ff3ec7a96cdbba4d2d5172d66bbc73c6db3885f29b21be5da9fa7a7c025232848381518110612a8157612a8061411e565b5b6020026020010151612b9c565b508080600101915050612a42565b505f5f90505b8851811015612afd57612aef7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698a8381518110612ae257612ae161411e565b5b602002602001015161220c565b508080600101915050612aa2565b50612b427f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e895f81518110612b3557612b3461411e565b5b602002602001015161220c565b507fa2df7830e0bedef7b1116bf547b467b16b50b3bd23146c9e099878d14e89301a878a33604051612b7693929190614948565b60405180910390a1505050505050505050565b5f612b95825f01612f7d565b9050919050565b5f612ba78383612f8c565b905092915050565b81600e8190555042600f81905550436010819055505f60115f6101000a81548160ff02191690836006811115612be857612be76137d7565b5b02179055507fdef1f08eb655f4a75f60bd6fd7e971112004abd846a612e46dab270770d24ca530600f5460105484604051612c26949392919061497d565b60405180910390a15050565b5f7f7965db0b000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19161480612ca45750612ca382612fcf565b5b9050919050565b5f33905090565b612cbc828261174f565b612cff5780826040517fe2517d3f000000000000000000000000000000000000000000000000000000008152600401612cf69291906149c0565b60405180910390fd5b5050565b5f5f5f6041845103612d43575f5f5f602087015192506040870151915060608701515f1a9050612d3588828585613038565b955095509550505050612d51565b5f600285515f1b9250925092505b9250925092565b5f6003811115612d6b57612d6a6137d7565b5b826003811115612d7e57612d7d6137d7565b5b0315612eb65760016003811115612d9857612d976137d7565b5b826003811115612dab57612daa6137d7565b5b03612de2576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60026003811115612df657612df56137d7565b5b826003811115612e0957612e086137d7565b5b03612e4d57805f1c6040517ffce698f7000000000000000000000000000000000000000000000000000000008152600401612e449190613872565b60405180910390fd5b600380811115612e6057612e5f6137d7565b5b826003811115612e7357612e726137d7565b5b03612eb557806040517fd78bce0c000000000000000000000000000000000000000000000000000000008152600401612eac9190613980565b60405180910390fd5b5b5050565b5f5f612ec6848461311f565b90508015612ef357612ef18360015f8781526020019081526020015f2061320890919063ffffffff16565b505b8091505092915050565b5f825f018281548110612f1357612f1261411e565b5b905f5260205f200154905092915050565b6060815f01805480602002602001604051908101604052809291908181526020018280548015612f7157602002820191905f5260205f20905b815481526020019060010190808311612f5d575b50505050509050919050565b5f815f01805490509050919050565b5f5f612f988484613235565b90508015612fc557612fc38360015f8781526020019081526020015f2061331e90919063ffffffff16565b505b8091505092915050565b5f7f01ffc9a7000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916149050919050565b5f5f5f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c1115613074575f600385925092509250613115565b5f6001888888886040515f81526020016040526040516130979493929190614a02565b6020604051602081039080840390855afa1580156130b7573d5f5f3e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603613108575f60015f5f1b93509350935050613115565b805f5f5f1b935093509350505b9450945094915050565b5f61312a838361174f565b6131fe5760015f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555061319b612cab565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a460019050613202565b5f90505b92915050565b5f61322d835f018373ffffffffffffffffffffffffffffffffffffffff165f1b61334b565b905092915050565b5f613240838361174f565b15613314575f5f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055506132b1612cab565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b60405160405180910390a460019050613318565b5f90505b92915050565b5f613343835f018373ffffffffffffffffffffffffffffffffffffffff165f1b6133b2565b905092915050565b5f61335683836134ae565b6133a857825f0182908060018154018082558091505060019003905f5260205f20015f9091909190915055825f0180549050836001015f8481526020019081526020015f2081905550600190506133ac565b5f90505b92915050565b5f5f836001015f8481526020019081526020015f205490505f81146134a3575f6001826133df91906146d3565b90505f6001865f01805490506133f591906146d3565b905080821461345b575f865f0182815481106134145761341361411e565b5b905f5260205f200154905080875f0184815481106134355761343461411e565b5b905f5260205f20018190555083876001015f8381526020019081526020015f2081905550505b855f0180548061346e5761346d614a45565b5b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f9055600193505050506134a8565b5f9150505b92915050565b5f5f836001015f8481526020019081526020015f20541415905092915050565b505f81556001015f9055565b5080545f8255905f5260205f20906134f291906134f5565b50565b5f5b80821115613515578281015f61350d919061351a565b6001016134f7565b505090565b50805461352690613e3d565b5f825580601f106135375750613551565b601f0160209004905f5260205f20906135509190613554565b5b50565b5f5b8082111561356c578281015f9055600101613556565b505090565b5f604051905090565b5f5ffd5b5f5ffd5b5f7fffffffff0000000000000000000000000000000000000000000000000000000082169050919050565b6135b681613582565b81146135c0575f5ffd5b50565b5f813590506135d1816135ad565b92915050565b5f602082840312156135ec576135eb61357a565b5b5f6135f9848285016135c3565b91505092915050565b5f8115159050919050565b61361681613602565b82525050565b5f60208201905061362f5f83018461360d565b92915050565b5f819050919050565b61364781613635565b8114613651575f5ffd5b50565b5f813590506136628161363e565b92915050565b5f5f6040838503121561367e5761367d61357a565b5b5f61368b85828601613654565b925050602061369c85828601613654565b9150509250929050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f8401126136c7576136c66136a6565b5b8235905067ffffffffffffffff8111156136e4576136e36136aa565b5b602083019150836001820283011115613700576136ff6136ae565b5b9250929050565b5f5f6020838503121561371d5761371c61357a565b5b5f83013567ffffffffffffffff81111561373a5761373961357e565b5b613746858286016136b2565b92509250509250929050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61377b82613752565b9050919050565b61378b81613771565b8114613795575f5ffd5b50565b5f813590506137a681613782565b92915050565b5f602082840312156137c1576137c061357a565b5b5f6137ce84828501613798565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b60078110613815576138146137d7565b5b50565b5f81905061382582613804565b919050565b5f61383482613818565b9050919050565b6138448161382a565b82525050565b5f60208201905061385d5f83018461383b565b92915050565b61386c81613635565b82525050565b5f6020820190506138855f830184613863565b92915050565b5f5f5f604084860312156138a2576138a161357a565b5b5f6138af86828701613798565b935050602084013567ffffffffffffffff8111156138d0576138cf61357e565b5b6138dc868287016136b2565b92509250509250925092565b5f602082840312156138fd576138fc61357a565b5b5f61390a84828501613654565b91505092915050565b5f819050919050565b61392581613913565b811461392f575f5ffd5b50565b5f813590506139408161391c565b92915050565b5f6020828403121561395b5761395a61357a565b5b5f61396884828501613932565b91505092915050565b61397a81613913565b82525050565b5f6020820190506139935f830184613971565b92915050565b5f5f604083850312156139af576139ae61357a565b5b5f6139bc85828601613932565b92505060206139cd85828601613798565b9150509250929050565b6139e081613771565b82525050565b5f6020820190506139f95f8301846139d7565b92915050565b5f5f60408385031215613a1557613a1461357a565b5b5f613a2285828601613932565b9250506020613a3385828601613654565b9150509250929050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b613a6f81613771565b82525050565b5f613a808383613a66565b60208301905092915050565b5f602082019050919050565b5f613aa282613a3d565b613aac8185613a47565b9350613ab783613a57565b805f5b83811015613ae7578151613ace8882613a75565b9750613ad983613a8c565b925050600181019050613aba565b5085935050505092915050565b5f6020820190508181035f830152613b0c8184613a98565b905092915050565b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b613b5a82613b14565b810181811067ffffffffffffffff82111715613b7957613b78613b24565b5b80604052505050565b5f613b8b613571565b9050613b978282613b51565b919050565b5f67ffffffffffffffff821115613bb657613bb5613b24565b5b602082029050602081019050919050565b5f613bd9613bd484613b9c565b613b82565b90508083825260208201905060208402830185811115613bfc57613bfb6136ae565b5b835b81811015613c255780613c118882613798565b845260208401935050602081019050613bfe565b5050509392505050565b5f82601f830112613c4357613c426136a6565b5b8135613c53848260208601613bc7565b91505092915050565b5f5f60408385031215613c7257613c7161357a565b5b5f613c7f85828601613654565b925050602083013567ffffffffffffffff811115613ca057613c9f61357e565b5b613cac85828601613c2f565b9150509250929050565b5f5f5f5f60808587031215613cce57613ccd61357a565b5b5f613cdb87828801613932565b9450506020613cec87828801613654565b935050604085013567ffffffffffffffff811115613d0d57613d0c61357e565b5b613d1987828801613c2f565b9250506060613d2a87828801613654565b91505092959194509250565b5f604082019050613d495f8301856139d7565b613d566020830184613863565b9392505050565b5f606082019050613d705f8301866139d7565b613d7d6020830185613863565b613d8a6040830184613863565b949350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f613dc982613635565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203613dfb57613dfa613d92565b5b600182019050919050565b5f82905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f6002820490506001821680613e5457607f821691505b602082108103613e6757613e66613e10565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f60088302613ec97fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82613e8e565b613ed38683613e8e565b95508019841693508086168417925050509392505050565b5f819050919050565b5f613f0e613f09613f0484613635565b613eeb565b613635565b9050919050565b5f819050919050565b613f2783613ef4565b613f3b613f3382613f15565b848454613e9a565b825550505050565b5f5f905090565b613f52613f43565b613f5d818484613f1e565b505050565b5f5b82811015613f8357613f785f828401613f4a565b600181019050613f64565b505050565b601f821115613fd65782821115613fd557613fa281613e6d565b613fab83613e7f565b613fb485613e7f565b6020861015613fc1575f90505b808301613fd082840382613f62565b505050505b5b505050565b5f82821c905092915050565b5f613ff65f1984600802613fdb565b1980831691505092915050565b5f61400e8383613fe7565b9150826002028217905092915050565b6140288383613e06565b67ffffffffffffffff81111561404157614040613b24565b5b61404b8254613e3d565b614056828285613f88565b5f601f831160018114614083575f8415614071578287013590505b61407b8582614003565b8655506140e2565b601f19841661409186613e6d565b5f5b828110156140b857848901358255600182019150602085019450602081019050614093565b868310156140d557848901356140d1601f891682613fe7565b8355505b6001600288020188555050505b50505050505050565b5f6140f582613635565b915061410083613635565b925082820190508082111561411857614117613d92565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f82825260208201905092915050565b7f4255473a207468652061757468656e7469636174696f6e20766f7465732062795f8201527f20686f6e65737420636c69656e74732061726520696e636f6e73697374656e74602082015250565b5f6141b560408361414b565b91506141c08261415b565b604082019050919050565b5f6020820190508181035f8301526141e2816141a9565b9050919050565b5f6060820190506141fc5f830186613863565b61420960208301856139d7565b61421660408301846139d7565b949350505050565b5f61422882613635565b915061423383613635565b925082820261424181613635565b9150828204841483151761425857614257613d92565b5b5092915050565b5f819050602082019050919050565b5f81549050919050565b5f8190506001806001038301049050919050565b5f819050815f5260205f209050919050565b6142ce7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff83602003600802613fdb565b815481168255505050565b6142e281613e6d565b6142ed838254614003565b8083555f825550505050565b602084105f811461435457601f8411600181146143215761431a8685614003565b835561434e565b61432a83613e6d565b614342600161433888613e7f565b0360018301613f62565b61434c87856142d9565b505b506143ae565b61435d85613e7f565b61436685613e7f565b61436f84613e6d565b828101601f8916801561438a57614389816001840361429e565b5b8484111561439f5761439e85850383613f62565b5b60018a60020217875550505050505b5050505050565b680100000000000000008411156143cf576143ce613b24565b5b602083105f811461441857602085105f81146143f6576143ef8685614003565b8355614412565b8360ff191693508361440784613e6d565b556001866002020183555b50614422565b6001856002020182555b5050505050565b805461443481613e3d565b8084111561444957614448848284866143b5565b5b8084101561445e5761445d848284866142f9565b5b50505050565b82811015614483576144785f828401613f4a565b600181019050614464565b505050565b6144925f82614429565b50565b7f4e487b71000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b5f82146144d1576144d0614495565b5b6144da81614488565b5050565b5f5b828110156144ff576144f45f8284016144c1565b6001810190506144e0565b505050565b8183101561453b5761451582614278565b61451e84614278565b6145278361428c565b818101614536838503826144de565b505050505b505050565b6801000000000000000082111561455a57614559613b24565b5b6145638161426e565b828255614571838284614504565b505050565b5f81519050919050565b5f81519050919050565b5f81519050919050565b61459d8261458a565b67ffffffffffffffff8111156145b6576145b5613b24565b5b6145c08254613e3d565b6145cb828285613f88565b5f60209050601f8311600181146145fc575f84156145ea578287015190505b6145f48582614003565b86555061465b565b601f19841661460a86613e6d565b5f5b828110156146315784890151825560018201915060208501945060208101905061460c565b8683101561464e578489015161464a601f891682613fe7565b8355505b6001600288020188555050505b505050505050565b61466d8282614594565b5050565b61467a82614576565b6146848183614540565b61468d8361425f565b6146968361428c565b5f5b838110156146cb576146a983614580565b6146b38184614663565b60208401935060018301925050600181019050614698565b505050505050565b5f6146dd82613635565b91506146e883613635565b9250828203905081811115614700576146ff613d92565b5b92915050565b5f6040820190506147195f830185613863565b6147266020830184613863565b9392505050565b5f6040820190506147405f8301856139d7565b61474d60208301846139d7565b9392505050565b7f4255473a20414c5245414459205245434549564544205348415245532046524f5f8201527f4d204e20504152544945532c20544f4f204d414e5920434c49454e5453000000602082015250565b5f6147ae603d8361414b565b91506147b982614754565b604082019050919050565b5f6020820190508181035f8301526147db816147a2565b9050919050565b5f82825260208201905092915050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f61481a8261458a565b61482481856147f2565b9350614834818560208601614802565b61483d81613b14565b840191505092915050565b5f6148538383614810565b905092915050565b5f602082019050919050565b5f61487182614576565b61487b81856147e2565b93508360208202850161488d8561425f565b805f5b858110156148c857848403895281516148a98582614848565b94506148b48361485b565b925060208a01995050600181019050614890565b50829750879550505050505092915050565b5f6020820190508181035f8301526148f28184614867565b905092915050565b5f60408201905061490d5f83018561383b565b61491a602083018461383b565b9392505050565b5f6040820190506149345f830185613863565b61494160208301846139d7565b9392505050565b5f60608201905061495b5f830186613863565b6149686020830185613863565b61497560408301846139d7565b949350505050565b5f6080820190506149905f8301876139d7565b61499d6020830186613863565b6149aa6040830185613863565b6149b760608301846139d7565b95945050505050565b5f6040820190506149d35f8301856139d7565b6149e06020830184613971565b9392505050565b5f60ff82169050919050565b6149fc816149e7565b82525050565b5f608082019050614a155f830187613971565b614a2260208301866149f3565b614a2f6040830185613971565b614a3c6060830184613971565b95945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffdfea26469706673582212206a6f38d4605ef37f7117223bc9cfd00d4a200ab7971e775e818a102edbc2b14364736f6c63430008210033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02%W_5`\xE0\x1C\x80cqP\x18\xA6\x11a\x01.W\x80c\xC0y\xF4\x95\x11a\0\xB6W\x80c\xEA\xE6\xF6R\x11a\0zW\x80c\xEA\xE6\xF6R\x14a\x05\xC9W\x80c\xEB\x85u\xDE\x14a\x05\xE7W\x80c\xF2\xFD\xE3\x8B\x14a\x06\x03W\x80c\xF6`<a\x14a\x06\x1FW\x80c\xFCx\xB2\xE8\x14a\x06;Wa\x02%V[\x80c\xC0y\xF4\x95\x14a\x05KW\x80c\xCA\x15\xC8s\x14a\x05UW\x80c\xCB\x9CL\xC4\x14a\x05\x85W\x80c\xD5Gt\x1F\x14a\x05\x8FW\x80c\xD8'\r\xCE\x14a\x05\xABWa\x02%V[\x80c\x91\xD1HT\x11a\0\xFDW\x80c\x91\xD1HT\x14a\x04\xA7W\x80c\xA2\x17\xFD\xDF\x14a\x04\xD7W\x80c\xA3$j\xD3\x14a\x04\xF5W\x80c\xAF o(\x14a\x05%W\x80c\xBBQ\xFE\xF0\x14a\x05AWa\x02%V[\x80cqP\x18\xA6\x14a\x041W\x80c\x7F5\xB5`\x14a\x04;W\x80c\x8D\xA5\xCB[\x14a\x04YW\x80c\x90\x10\xD0|\x14a\x04wWa\x02%V[\x80c#(\xBD\x12\x11a\x01\xB1W\x80c3\xCC\x9A\t\x11a\x01\x80W\x80c3\xCC\x9A\t\x14a\x03\xDBW\x80c6V\x8A\xBE\x14a\x03\xE5W\x80c;C8\xD1\x14a\x04\x01W\x80cK\x8Ed\x88\x14a\x04\x1DW\x80cK\xB2x\xF3\x14a\x04'Wa\x02%V[\x80c#(\xBD\x12\x14a\x03SW\x80c$\x8A\x9C\xA3\x14a\x03qW\x80c//\xF1]\x14a\x03\xA1W\x80c0\x10L>\x14a\x03\xBDWa\x02%V[\x80c\x14l\xA51\x11a\x01\xF8W\x80c\x14l\xA51\x14a\x02\xC1W\x80c\x17cE\x14\x14a\x02\xDFW\x80c\x1CtS\xDB\x14a\x02\xFDW\x80c\x1E\xE4\xEE\x0F\x14a\x03\x1BW\x80c!\xDC{\x9B\x14a\x037Wa\x02%V[\x80c\x01\xFF\xC9\xA7\x14a\x02)W\x80c\x0B\xDA\x81\xCF\x14a\x02YW\x80c\rB\xEBo\x14a\x02uW\x80c\x13\xFFm\xD5\x14a\x02\x91W[__\xFD[a\x02C`\x04\x806\x03\x81\x01\x90a\x02>\x91\x90a5\xD7V[a\x06kV[`@Qa\x02P\x91\x90a6\x1CV[`@Q\x80\x91\x03\x90\xF3[a\x02s`\x04\x806\x03\x81\x01\x90a\x02n\x91\x90a6hV[a\x06\xE4V[\0[a\x02\x8F`\x04\x806\x03\x81\x01\x90a\x02\x8A\x91\x90a7\x07V[a\t6V[\0[a\x02\xAB`\x04\x806\x03\x81\x01\x90a\x02\xA6\x91\x90a7\xACV[a\twV[`@Qa\x02\xB8\x91\x90a6\x1CV[`@Q\x80\x91\x03\x90\xF3[a\x02\xC9a\t\xF3V[`@Qa\x02\xD6\x91\x90a8JV[`@Q\x80\x91\x03\x90\xF3[a\x02\xE7a\n\x05V[`@Qa\x02\xF4\x91\x90a8rV[`@Q\x80\x91\x03\x90\xF3[a\x03\x05a\n\x0BV[`@Qa\x03\x12\x91\x90a8rV[`@Q\x80\x91\x03\x90\xF3[a\x035`\x04\x806\x03\x81\x01\x90a\x030\x91\x90a8\x8BV[a\n\x11V[\0[a\x03Q`\x04\x806\x03\x81\x01\x90a\x03L\x91\x90a8\xE8V[a\x0F\xBFV[\0[a\x03[a\x13\xC6V[`@Qa\x03h\x91\x90a8rV[`@Q\x80\x91\x03\x90\xF3[a\x03\x8B`\x04\x806\x03\x81\x01\x90a\x03\x86\x91\x90a9FV[a\x13\xDCV[`@Qa\x03\x98\x91\x90a9\x80V[`@Q\x80\x91\x03\x90\xF3[a\x03\xBB`\x04\x806\x03\x81\x01\x90a\x03\xB6\x91\x90a9\x99V[a\x13\xF8V[\0[a\x03\xC5a\x142V[`@Qa\x03\xD2\x91\x90a9\x80V[`@Q\x80\x91\x03\x90\xF3[a\x03\xE3a\x14VV[\0[a\x03\xFF`\x04\x806\x03\x81\x01\x90a\x03\xFA\x91\x90a9\x99V[a\x14\xD0V[\0[a\x04\x1B`\x04\x806\x03\x81\x01\x90a\x04\x16\x91\x90a6hV[a\x15\x97V[\0[a\x04%a\x15\xD0V[\0[a\x04/a\x16JV[\0[a\x049a\x16\xC4V[\0[a\x04Ca\x16\xD7V[`@Qa\x04P\x91\x90a9\x80V[`@Q\x80\x91\x03\x90\xF3[a\x04aa\x16\xFBV[`@Qa\x04n\x91\x90a9\xE6V[`@Q\x80\x91\x03\x90\xF3[a\x04\x91`\x04\x806\x03\x81\x01\x90a\x04\x8C\x91\x90a9\xFFV[a\x17#V[`@Qa\x04\x9E\x91\x90a9\xE6V[`@Q\x80\x91\x03\x90\xF3[a\x04\xC1`\x04\x806\x03\x81\x01\x90a\x04\xBC\x91\x90a9\x99V[a\x17OV[`@Qa\x04\xCE\x91\x90a6\x1CV[`@Q\x80\x91\x03\x90\xF3[a\x04\xDFa\x17\xB2V[`@Qa\x04\xEC\x91\x90a9\x80V[`@Q\x80\x91\x03\x90\xF3[a\x05\x0F`\x04\x806\x03\x81\x01\x90a\x05\n\x91\x90a9FV[a\x17\xB8V[`@Qa\x05\x1C\x91\x90a:\xF4V[`@Q\x80\x91\x03\x90\xF3[a\x05?`\x04\x806\x03\x81\x01\x90a\x05:\x91\x90a<\\V[a\x17\xDAV[\0[a\x05Ia\x18\x13V[\0[a\x05Sa\x18\x8DV[\0[a\x05o`\x04\x806\x03\x81\x01\x90a\x05j\x91\x90a9FV[a\x19\x07V[`@Qa\x05|\x91\x90a8rV[`@Q\x80\x91\x03\x90\xF3[a\x05\x8Da\x19(V[\0[a\x05\xA9`\x04\x806\x03\x81\x01\x90a\x05\xA4\x91\x90a9\x99V[a\x19\xA1V[\0[a\x05\xB3a\x1A\x94V[`@Qa\x05\xC0\x91\x90a8rV[`@Q\x80\x91\x03\x90\xF3[a\x05\xD1a\x1A\x9AV[`@Qa\x05\xDE\x91\x90a9\x80V[`@Q\x80\x91\x03\x90\xF3[a\x06\x01`\x04\x806\x03\x81\x01\x90a\x05\xFC\x91\x90a8\x8BV[a\x1A\xBEV[\0[a\x06\x1D`\x04\x806\x03\x81\x01\x90a\x06\x18\x91\x90a7\xACV[a \x03V[\0[a\x069`\x04\x806\x03\x81\x01\x90a\x064\x91\x90a<\xB6V[a \x87V[\0[a\x06U`\x04\x806\x03\x81\x01\x90a\x06P\x91\x90a7\xACV[a \xF0V[`@Qa\x06b\x91\x90a6\x1CV[`@Q\x80\x91\x03\x90\xF3[_\x7F\x07\xEF\xFE\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x06\xDDWPa\x06\xDC\x82a!\"V[[\x90P\x91\x90PV[\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2a\x07\x0E\x81a!\x9BV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x05_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07\xB0W3\x82`@Q\x7F\xFF\xAB\xBA\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\xA7\x92\x91\x90a=6V[`@Q\x80\x91\x03\x90\xFD[_\x83\x03a\x07\xF4W3`@Q\x7F\x16\x92<\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\xEB\x91\x90a9\xE6V[`@Q\x80\x91\x03\x90\xFD[_`\x0C_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x14a\x08xW3`@Q\x7FO_\xBF\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08o\x91\x90a9\xE6V[`@Q\x80\x91\x03\x90\xFD[`@Q\x80`@\x01`@R\x80\x83\x81R` \x01\x84\x81RP`\x0C_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01U` \x82\x01Q\x81`\x01\x01U\x90PP\x7F\xB8\x9A\xDD\xD97\xF4O\x90,\x84\x95\x96d\x187\xCDz\xF2\xFC\xEC\xEF\"\xD2\xA7\x86o\xDC\x1A\xD9\xC0\xAE.3\x84\x84`@Qa\t\x12\x93\x92\x91\x90a=]V[`@Q\x80\x91\x03\x90\xA1`\t_\x81T\x80\x92\x91\x90a\t,\x90a=\xBFV[\x91\x90PUPPPPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\t`\x81a!\x9BV[\x82\x82`\x04\x91\x82a\tq\x92\x91\x90a@\x1EV[PPPPV[_a\t\x81\x82a \xF0V[a\t\xC2W\x81`@Q\x7F\xAB\xDC\xE0j\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xB9\x91\x90a9\xE6V[`@Q\x80\x91\x03\x90\xFD[a\t\xEC\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x83a\x17OV[\x90P\x91\x90PV[`\x11_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\x10T\x81V[`\nT\x81V[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\n;\x81a!\x9BV[a\ne\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2\x85a\x17OV[a\n\xA6W\x83`@Q\x7F\xA02\xACk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\x9D\x91\x90a9\xE6V[`@Q\x80\x91\x03\x90\xFD[_`\x07T\x90P__\x90P[`\x07T\x81\x10\x15a\x0B3W\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x05_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0B&W\x80\x91Pa\x0B3V[\x80\x80`\x01\x01\x91PPa\n\xB1V[P`\x07T\x81\x03a\x0B\xAAW`\x05__\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x7Fo\xAF\x9F\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\xA1\x91\x90a9\xE6V[`@Q\x80\x91\x03\x90\xFD[_\x81`\nTa\x0B\xB9\x91\x90a@\xEBV[\x90P_a\x0B\xEB\x82`@Q` \x01a\x0B\xD0\x91\x90a8rV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a!\xAFV[\x90P_a\x0C;\x82\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa!\xE2V[\x90P\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0C\xD9W`\x06_\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01`\x02\x81\x10a\x0C\xC0Wa\x0C\xBFaA\x1EV[[\x01_\x81Ta\x0C\xCD\x90a=\xBFV[\x91\x90P\x81\x90UPa\r=V[`\x06_\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _`\x02\x81\x10a\r(Wa\r'aA\x1EV[[\x01_\x81Ta\r5\x90a=\xBFV[\x91\x90P\x81\x90UP[`\x01`\x0BTa\rL\x91\x90a@\xEBV[`\x06_\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _`\x02\x81\x10a\r\x9BWa\r\x9AaA\x1EV[[\x01T\x10\x80a\x0E\x07WP`\x01`\x0BTa\r\xB3\x91\x90a@\xEBV[`\x06_\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01`\x02\x81\x10a\x0E\x03Wa\x0E\x02aA\x1EV[[\x01T\x10[a\x0EFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E=\x90aA\xCBV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x0BTa\x0EU\x91\x90a@\xEBV[`\x06_\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _`\x02\x81\x10a\x0E\xA4Wa\x0E\xA3aA\x1EV[[\x01T\x10a\x0E\xFEW\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F@p\t \x05 \xF9\xF1\x05\x84\x81<\x0B\x95D\x1A\xB3 \xF6\xB0\x8D\x97\xEB\xDA\xAF\x1E\x82N\xED\xD9\xD7\xC3_`@Qa\x0E\xF1\x91\x90a6\x1CV[`@Q\x80\x91\x03\x90\xA2a\x0F\xB5V[`\x01`\x0BTa\x0F\r\x91\x90a@\xEBV[`\x06_\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01`\x02\x81\x10a\x0F]Wa\x0F\\aA\x1EV[[\x01T\x10a\x0F\xB4W\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F@p\t \x05 \xF9\xF1\x05\x84\x81<\x0B\x95D\x1A\xB3 \xF6\xB0\x8D\x97\xEB\xDA\xAF\x1E\x82N\xED\xD9\xD7\xC3`\x01`@Qa\x0F\xAB\x91\x90a6\x1CV[`@Q\x80\x91\x03\x90\xA2[[PPPPPPPPV[__\x90P[`\x07T\x81\x10\x15a\x10}W3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x05_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x10pW3\x81`@Q\x7F\xC3\x15\xA0\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10g\x92\x91\x90a=6V[`@Q\x80\x91\x03\x90\xFD[\x80\x80`\x01\x01\x91PPa\x0F\xC4V[P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x05_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x11RW\x803`\x05_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x7F\xA0\xB8\xC7\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11I\x93\x92\x91\x90aA\xE9V[`@Q\x80\x91\x03\x90\xFD[3`\x05_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPa\x11\xCB\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R23a\"\x0CV[P_`\x03_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x90P`\x01`\x0BT`\x03a\x12\x1D\x91\x90aB\x1EV[a\x12'\x91\x90a@\xEBV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12@Wa\x12?a;$V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12sW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x12^W\x90P[P\x81_\x01\x90\x81a\x12\x83\x91\x90aFqV[P_\x81`\x01\x01\x81\x90UP_a\x12\xB7\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x17\xB8V[\x90P_a\x12\xE3\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x19\x07V[\x90P__\x90P[\x81\x81\x10\x15a\x13oW_\x84`\x02\x01_\x85\x84\x81Q\x81\x10a\x13\x0BWa\x13\naA\x1EV[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80\x80`\x01\x01\x91PPa\x12\xEAV[P`\x08_\x81T\x80\x92\x91\x90a\x13\x82\x90a=\xBFV[\x91\x90PUP\x7F\xAB\xDE\x16\xB7\xA9\x19,1\xC6#\x1B\x159\xBA\xD6\xFE\xD7v5\xDEL\0\x87\x18\xDB\xDC\xAF\xB7\xB86:\xFE3\x85`@Qa\x13\xB8\x92\x91\x90a=6V[`@Q\x80\x91\x03\x90\xA1PPPPV[_`\x08T`\x07Ta\x13\xD7\x91\x90aF\xD3V[\x90P\x90V[___\x83\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x14\"\x81a!\x9BV[a\x14,\x83\x83a\"\x0CV[PPPPV[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x81V[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x14\x80\x81a!\x9BV[`\x03a\x14\x8B\x81a\"\x1FV[\x7F \xF5^\xD0\xC9/+\xB1\xC8\x82T\x88\xE1\xE3\xC9\x84c\xD0$\xB2\xA4-\xBD$\x83\x8C?u&\x0FC\xE93B`@Qa\x14\xBC\x92\x91\x90a=6V[`@Q\x80\x91\x03\x90\xA1a\x14\xCCa\"\xA8V[PPV[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x82\x14\x80\x15a\x15\x05WPa\x15\x04\x82\x82a\x17OV[[\x15a\x15\x89W_a\x15\x14\x83a\x19\x07V[\x90P_`\x01`\x02T`\x03a\x15(\x91\x90aB\x1EV[a\x152\x91\x90a@\xEBV[\x90P\x80\x82\x03a\x15\x86W`\x01\x81a\x15H\x91\x90aF\xD3V[\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15}\x92\x91\x90aG\x06V[`@Q\x80\x91\x03\x90\xFD[PP[a\x15\x93\x82\x82a#\x11V[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x15\xC1\x81a!\x9BV[a\x15\xCB\x83\x83a#\x8CV[PPPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x15\xFA\x81a!\x9BV[`\x04a\x16\x05\x81a\"\x1FV[\x7F0\x1F\x8A7\x01\xF5\xB2`\x19s\x82\xDDs\x01\x07\x85B\x14O\xE8\xFD\xDD\x18\x08=on\t\xE4\x95\x8AY3B`@Qa\x166\x92\x91\x90a=6V[`@Q\x80\x91\x03\x90\xA1a\x16Fa\"\xA8V[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x16t\x81a!\x9BV[`\x05a\x16\x7F\x81a\"\x1FV[\x7F$\xA8se\x1D&\xFBZF,\xB3z\x91\x07\x1C\xDDM\t\xAB\xEE\xBF\xE0\xED\x14C)\xBE\xD1\xCC5\x9D\x033B`@Qa\x16\xB0\x92\x91\x90a=6V[`@Q\x80\x91\x03\x90\xA1a\x16\xC0a\"\xA8V[PPV[a\x16\xCCa&\x89V[a\x16\xD5_a'\x10V[V[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x81V[_`\r_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_a\x17G\x82`\x01_\x86\x81R` \x01\x90\x81R` \x01_ a'\xD3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[___\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[__\x1B\x81V[``a\x17\xD3`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a'\xEAV[\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x18\x04\x81a!\x9BV[a\x18\x0E\x83\x83a(\tV[PPPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x18=\x81a!\x9BV[`\x02a\x18H\x81a\"\x1FV[\x7F`\xED\xF9\xBD\xC7\xC4\xEA\0|\xAE\x1A\x9B\xBD\x03\xE4\x1E[\xFC\xCDr1\xA6\xEC8<.\xDDx\0\xF0\xD2\x0C3B`@Qa\x18y\x92\x91\x90a=6V[`@Q\x80\x91\x03\x90\xA1a\x18\x89a\"\xA8V[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x18\xB7\x81a!\x9BV[`\x01a\x18\xC2\x81a\"\x1FV[\x7Fg\xC4H\x9FgM\x03\xC7\xD1\x9A\x9E6sQ\x88\xDE|e\xE8\xD1\xE9\x9E\xB3\xA2\xFD%\x8Av\x9E\xB1O\xFF3B`@Qa\x18\xF3\x92\x91\x90a=6V[`@Q\x80\x91\x03\x90\xA1a\x19\x03a\"\xA8V[PPV[_a\x19!`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a+\x89V[\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x19R\x81a!\x9BV[_a\x19\\\x81a\"\x1FV[\x7F\xBBp\x9D\xEAtO\x06\xD1\xB2n\x82M\xEE\xC2\xF7\x14\x0CQ\x12f\xEE\x15\xD7\xA2\x17\x83\x8B1\xD8\xB0\x12=3B`@Qa\x19\x8D\x92\x91\x90a=6V[`@Q\x80\x91\x03\x90\xA1a\x19\x9Da\"\xA8V[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x19\xCB\x81a!\x9BV[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x83\x14\x80\x15a\x1A\0WPa\x19\xFF\x83\x83a\x17OV[[\x15a\x1A\x84W_a\x1A\x0F\x84a\x19\x07V[\x90P_`\x01`\x02T`\x03a\x1A#\x91\x90aB\x1EV[a\x1A-\x91\x90a@\xEBV[\x90P\x80\x82\x03a\x1A\x81W`\x01\x81a\x1AC\x91\x90aF\xD3V[\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1Ax\x92\x91\x90aG\x06V[`@Q\x80\x91\x03\x90\xFD[PP[a\x1A\x8E\x83\x83a+\x9CV[PPPPV[`\x0FT\x81V[\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2\x81V[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x1A\xE8\x81a!\x9BV[a\x1B\x12\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2\x85a\x17OV[a\x1BSW\x83`@Q\x7F\xA02\xACk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1BJ\x91\x90a9\xE6V[`@Q\x80\x91\x03\x90\xFD[_`\x03_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x90P`\x03_\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x02\x01_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x1CcW\x843`@Q\x7F\x08\xE5T\x95\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1CZ\x92\x91\x90aG-V[`@Q\x80\x91\x03\x90\xFD[`\x01`\x0BT`\x03a\x1Ct\x91\x90aB\x1EV[a\x1C~\x91\x90a@\xEBV[\x81\x10a\x1C\xBFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C\xB6\x90aG\xC4V[`@Q\x80\x91\x03\x90\xFD[`\x01`\x03_\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x02\x01_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x83\x83`\x03_\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x01\x83\x81T\x81\x10a\x1D\xA5Wa\x1D\xA4aA\x1EV[[\x90_R` _ \x01\x91\x82a\x1D\xBA\x92\x91\x90a@\x1EV[P`\x01`\x03_\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x01_\x82\x82Ta\x1E\x0B\x91\x90a@\xEBV[\x92PP\x81\x90UP`\x01\x81a\x1E\x1F\x91\x90a@\xEBV[\x90P`\x01`\x02T`\x02a\x1E2\x91\x90aB\x1EV[a\x1E<\x91\x90a@\xEBV[\x81\x10a\x1F\xFCW_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E]Wa\x1E\\a;$V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\x90W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1E{W\x90P[P\x90P__\x90P[\x82\x81\x10\x15a\x1F\xABW`\x03_\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x01\x81\x81T\x81\x10a\x1E\xF1Wa\x1E\xF0aA\x1EV[[\x90_R` _ \x01\x80Ta\x1F\x04\x90a>=V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1F0\x90a>=V[\x80\x15a\x1F{W\x80`\x1F\x10a\x1FRWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1F{V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1F^W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x82\x82\x81Q\x81\x10a\x1F\x93Wa\x1F\x92aA\x1EV[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\x1E\x98V[P\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F#\xE9\x1D\xBF\xAE\x03u\x8C\xB8\x8D\x7FbR\xB5q\n\xFAS\xA1\x9F\xFE\x0FKOu\xD7\xF2\xDE\x0C^\xAB\xE9\x82`@Qa\x1F\xF2\x91\x90aH\xDAV[`@Q\x80\x91\x03\x90\xA2P[PPPPPV[a \x0Ba&\x89V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a {W_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a r\x91\x90a9\xE6V[`@Q\x80\x91\x03\x90\xFD[a \x84\x81a'\x10V[PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa \xB1\x81a!\x9BV[a \xBB\x84\x84a(\tV[a \xC5\x82\x85a#\x8CV[a \xE9\x85\x84_\x81Q\x81\x10a \xDCWa \xDBaA\x1EV[[` \x02` \x01\x01Qa+\xAFV[PPPPPV[_a!\x1B\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x83a\x17OV[\x90P\x91\x90PV[_\x7FZ\x05\x18\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a!\x94WPa!\x93\x82a,2V[[\x90P\x91\x90PV[a!\xAC\x81a!\xA7a,\xABV[a,\xB2V[PV[_\x7F\x19Ethereum Signed Message:\n32\0\0\0\0_R\x81`\x1CR`<_ \x90P\x91\x90PV[____a!\xF0\x86\x86a-\x03V[\x92P\x92P\x92Pa\"\0\x82\x82a-XV[\x82\x93PPPP\x92\x91PPV[_a\"\x17\x83\x83a.\xBAV[\x90P\x92\x91PPV[\x80`\x06\x81\x11\x15a\"2Wa\"1a7\xD7V[[`\x11_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x06\x81\x11\x15a\"SWa\"Ra7\xD7V[[\x14a\"\xA5W\x80`\x11_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x7F\xBF\xA2\x17\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\"\x9C\x92\x91\x90aH\xFAV[`@Q\x80\x91\x03\x90\xFD[PV[`\x01`\x11_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x06\x81\x11\x15a\"\xCBWa\"\xCAa7\xD7V[[a\"\xD5\x91\x90a@\xEBV[`\x06\x81\x11\x15a\"\xE7Wa\"\xE6a7\xD7V[[`\x11_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x06\x81\x11\x15a#\nWa#\ta7\xD7V[[\x02\x17\x90UPV[a#\x19a,\xABV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a#}W`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a#\x87\x82\x82a+\x9CV[PPPV[`\x07T`\n_\x82\x82Ta#\x9F\x91\x90a@\xEBV[\x92PP\x81\x90UP\x81`\x07\x81\x90UP_`\x08\x81\x90UP_`\t\x81\x90UP\x80`\x0B\x81\x90UP_a#\xEC\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x17\xB8V[\x90P_a$\x18\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x19\x07V[\x90P__\x90P[`\x07T\x81\x10\x15a&GW_`\x05_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\x0C_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ __\x82\x01_\x90U`\x01\x82\x01_\x90UPP`\x06_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a$\xF2\x91\x90a4\xCEV[__\x90P[\x83\x81\x10\x15a%\xB0W`\x03_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x02\x01_\x86\x83\x81Q\x81\x10a%SWa%RaA\x1EV[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U\x80\x80`\x01\x01\x91PPa$\xF7V[P`\x03_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ __\x82\x01_a%\xFD\x91\x90a4\xDAV[`\x01\x82\x01_\x90UPP`\x05_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90UP\x80\x80`\x01\x01\x91PPa$\x1FV[P\x7F\xF7\xF0\x87#\x82\xDF\xF5\xE6\x98\xB2\x84\xE1 \x84\xE4\xE7\x89O\x83\x02\x16\xDD\x80\xCBN\x90\x9BY:X\xF95`\x07T3`@Qa&{\x92\x91\x90aI!V[`@Q\x80\x91\x03\x90\xA1PPPPV[a&\x91a,\xABV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a&\xAFa\x16\xFBV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a'\x0EWa&\xD2a,\xABV[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a'\x05\x91\x90a9\xE6V[`@Q\x80\x91\x03\x90\xFD[V[_`\r_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\r_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[_a'\xE0\x83_\x01\x83a.\xFDV[_\x1C\x90P\x92\x91PPV[``_a'\xF8\x83_\x01a/$V[\x90P``\x81\x90P\x80\x92PPP\x91\x90PV[_`\x01\x83`\x03a(\x19\x91\x90aB\x1EV[a(#\x91\x90a@\xEBV[\x90P\x80\x82Q\x10\x15a(nW\x81Q\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a(e\x92\x91\x90aG\x06V[`@Q\x80\x91\x03\x90\xFD[\x82`\x02\x81\x90UP_a(\x9F\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x17\xB8V[\x90P_a(\xCB\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x19\x07V[\x90P__\x90P[\x81\x81\x10\x15a),Wa)\x1E\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x84\x83\x81Q\x81\x10a)\x11Wa)\x10aA\x1EV[[` \x02` \x01\x01Qa+\x9CV[P\x80\x80`\x01\x01\x91PPa(\xD2V[P_a)W\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x17\xB8V[\x90P_a)\x83\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x19\x07V[\x90P__\x90P[\x81\x81\x10\x15a)\xE4Wa)\xD6\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x84\x83\x81Q\x81\x10a)\xC9Wa)\xC8aA\x1EV[[` \x02` \x01\x01Qa+\x9CV[P\x80\x80`\x01\x01\x91PPa)\x8AV[P_a*\x0F\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2a\x17\xB8V[\x90P_a*;\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2a\x19\x07V[\x90P__\x90P[\x81\x81\x10\x15a*\x9CWa*\x8E\x7F\xA5\xFF>\xC7\xA9l\xDB\xBAM-Qr\xD6k\xBCs\xC6\xDB8\x85\xF2\x9B!\xBE]\xA9\xFAz|\x02R2\x84\x83\x81Q\x81\x10a*\x81Wa*\x80aA\x1EV[[` \x02` \x01\x01Qa+\x9CV[P\x80\x80`\x01\x01\x91PPa*BV[P__\x90P[\x88Q\x81\x10\x15a*\xFDWa*\xEF\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x8A\x83\x81Q\x81\x10a*\xE2Wa*\xE1aA\x1EV[[` \x02` \x01\x01Qa\"\x0CV[P\x80\x80`\x01\x01\x91PPa*\xA2V[Pa+B\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x89_\x81Q\x81\x10a+5Wa+4aA\x1EV[[` \x02` \x01\x01Qa\"\x0CV[P\x7F\xA2\xDFx0\xE0\xBE\xDE\xF7\xB1\x11k\xF5G\xB4g\xB1kP\xB3\xBD#\x14l\x9E\t\x98x\xD1N\x890\x1A\x87\x8A3`@Qa+v\x93\x92\x91\x90aIHV[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPV[_a+\x95\x82_\x01a/}V[\x90P\x91\x90PV[_a+\xA7\x83\x83a/\x8CV[\x90P\x92\x91PPV[\x81`\x0E\x81\x90UPB`\x0F\x81\x90UPC`\x10\x81\x90UP_`\x11_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x06\x81\x11\x15a+\xE8Wa+\xE7a7\xD7V[[\x02\x17\x90UP\x7F\xDE\xF1\xF0\x8E\xB6U\xF4\xA7_`\xBDo\xD7\xE9q\x11 \x04\xAB\xD8F\xA6\x12\xE4m\xAB'\x07p\xD2L\xA50`\x0FT`\x10T\x84`@Qa,&\x94\x93\x92\x91\x90aI}V[`@Q\x80\x91\x03\x90\xA1PPV[_\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a,\xA4WPa,\xA3\x82a/\xCFV[[\x90P\x91\x90PV[_3\x90P\x90V[a,\xBC\x82\x82a\x17OV[a,\xFFW\x80\x82`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a,\xF6\x92\x91\x90aI\xC0V[`@Q\x80\x91\x03\x90\xFD[PPV[___`A\x84Q\x03a-CW___` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90Pa-5\x88\x82\x85\x85a08V[\x95P\x95P\x95PPPPa-QV[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15a-kWa-ja7\xD7V[[\x82`\x03\x81\x11\x15a-~Wa-}a7\xD7V[[\x03\x15a.\xB6W`\x01`\x03\x81\x11\x15a-\x98Wa-\x97a7\xD7V[[\x82`\x03\x81\x11\x15a-\xABWa-\xAAa7\xD7V[[\x03a-\xE2W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15a-\xF6Wa-\xF5a7\xD7V[[\x82`\x03\x81\x11\x15a.\tWa.\x08a7\xD7V[[\x03a.MW\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.D\x91\x90a8rV[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15a.`Wa._a7\xD7V[[\x82`\x03\x81\x11\x15a.sWa.ra7\xD7V[[\x03a.\xB5W\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.\xAC\x91\x90a9\x80V[`@Q\x80\x91\x03\x90\xFD[[PPV[__a.\xC6\x84\x84a1\x1FV[\x90P\x80\x15a.\xF3Wa.\xF1\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a2\x08\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[_\x82_\x01\x82\x81T\x81\x10a/\x13Wa/\x12aA\x1EV[[\x90_R` _ \x01T\x90P\x92\x91PPV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a/qW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a/]W[PPPPP\x90P\x91\x90PV[_\x81_\x01\x80T\x90P\x90P\x91\x90PV[__a/\x98\x84\x84a25V[\x90P\x80\x15a/\xC5Wa/\xC3\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a3\x1E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[_\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[___\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15a0tW_`\x03\x85\x92P\x92P\x92Pa1\x15V[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@Qa0\x97\x94\x93\x92\x91\x90aJ\x02V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a0\xB7W=__>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a1\x08W_`\x01__\x1B\x93P\x93P\x93PPa1\x15V[\x80___\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[_a1*\x83\x83a\x17OV[a1\xFEW`\x01__\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa1\x9Ba,\xABV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa2\x02V[_\x90P[\x92\x91PPV[_a2-\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba3KV[\x90P\x92\x91PPV[_a2@\x83\x83a\x17OV[\x15a3\x14W___\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa2\xB1a,\xABV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa3\x18V[_\x90P[\x92\x91PPV[_a3C\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba3\xB2V[\x90P\x92\x91PPV[_a3V\x83\x83a4\xAEV[a3\xA8W\x82_\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x82_\x01\x80T\x90P\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP`\x01\x90Pa3\xACV[_\x90P[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x14a4\xA3W_`\x01\x82a3\xDF\x91\x90aF\xD3V[\x90P_`\x01\x86_\x01\x80T\x90Pa3\xF5\x91\x90aF\xD3V[\x90P\x80\x82\x14a4[W_\x86_\x01\x82\x81T\x81\x10a4\x14Wa4\x13aA\x1EV[[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a45Wa44aA\x1EV[[\x90_R` _ \x01\x81\x90UP\x83\x87`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UPP[\x85_\x01\x80T\x80a4nWa4maJEV[[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa4\xA8V[_\x91PP[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x14\x15\x90P\x92\x91PPV[P_\x81U`\x01\x01_\x90UV[P\x80T_\x82U\x90_R` _ \x90a4\xF2\x91\x90a4\xF5V[PV[_[\x80\x82\x11\x15a5\x15W\x82\x81\x01_a5\r\x91\x90a5\x1AV[`\x01\x01a4\xF7V[PP\x90V[P\x80Ta5&\x90a>=V[_\x82U\x80`\x1F\x10a57WPa5QV[`\x1F\x01` \x90\x04\x90_R` _ \x90a5P\x91\x90a5TV[[PV[_[\x80\x82\x11\x15a5lW\x82\x81\x01_\x90U`\x01\x01a5VV[PP\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a5\xB6\x81a5\x82V[\x81\x14a5\xC0W__\xFD[PV[_\x815\x90Pa5\xD1\x81a5\xADV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a5\xECWa5\xEBa5zV[[_a5\xF9\x84\x82\x85\x01a5\xC3V[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a6\x16\x81a6\x02V[\x82RPPV[_` \x82\x01\x90Pa6/_\x83\x01\x84a6\rV[\x92\x91PPV[_\x81\x90P\x91\x90PV[a6G\x81a65V[\x81\x14a6QW__\xFD[PV[_\x815\x90Pa6b\x81a6>V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a6~Wa6}a5zV[[_a6\x8B\x85\x82\x86\x01a6TV[\x92PP` a6\x9C\x85\x82\x86\x01a6TV[\x91PP\x92P\x92\x90PV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a6\xC7Wa6\xC6a6\xA6V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\xE4Wa6\xE3a6\xAAV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a7\0Wa6\xFFa6\xAEV[[\x92P\x92\x90PV[__` \x83\x85\x03\x12\x15a7\x1DWa7\x1Ca5zV[[_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7:Wa79a5~V[[a7F\x85\x82\x86\x01a6\xB2V[\x92P\x92PP\x92P\x92\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a7{\x82a7RV[\x90P\x91\x90PV[a7\x8B\x81a7qV[\x81\x14a7\x95W__\xFD[PV[_\x815\x90Pa7\xA6\x81a7\x82V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a7\xC1Wa7\xC0a5zV[[_a7\xCE\x84\x82\x85\x01a7\x98V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x07\x81\x10a8\x15Wa8\x14a7\xD7V[[PV[_\x81\x90Pa8%\x82a8\x04V[\x91\x90PV[_a84\x82a8\x18V[\x90P\x91\x90PV[a8D\x81a8*V[\x82RPPV[_` \x82\x01\x90Pa8]_\x83\x01\x84a8;V[\x92\x91PPV[a8l\x81a65V[\x82RPPV[_` \x82\x01\x90Pa8\x85_\x83\x01\x84a8cV[\x92\x91PPV[___`@\x84\x86\x03\x12\x15a8\xA2Wa8\xA1a5zV[[_a8\xAF\x86\x82\x87\x01a7\x98V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8\xD0Wa8\xCFa5~V[[a8\xDC\x86\x82\x87\x01a6\xB2V[\x92P\x92PP\x92P\x92P\x92V[_` \x82\x84\x03\x12\x15a8\xFDWa8\xFCa5zV[[_a9\n\x84\x82\x85\x01a6TV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a9%\x81a9\x13V[\x81\x14a9/W__\xFD[PV[_\x815\x90Pa9@\x81a9\x1CV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a9[Wa9Za5zV[[_a9h\x84\x82\x85\x01a92V[\x91PP\x92\x91PPV[a9z\x81a9\x13V[\x82RPPV[_` \x82\x01\x90Pa9\x93_\x83\x01\x84a9qV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a9\xAFWa9\xAEa5zV[[_a9\xBC\x85\x82\x86\x01a92V[\x92PP` a9\xCD\x85\x82\x86\x01a7\x98V[\x91PP\x92P\x92\x90PV[a9\xE0\x81a7qV[\x82RPPV[_` \x82\x01\x90Pa9\xF9_\x83\x01\x84a9\xD7V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a:\x15Wa:\x14a5zV[[_a:\"\x85\x82\x86\x01a92V[\x92PP` a:3\x85\x82\x86\x01a6TV[\x91PP\x92P\x92\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a:o\x81a7qV[\x82RPPV[_a:\x80\x83\x83a:fV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a:\xA2\x82a:=V[a:\xAC\x81\x85a:GV[\x93Pa:\xB7\x83a:WV[\x80_[\x83\x81\x10\x15a:\xE7W\x81Qa:\xCE\x88\x82a:uV[\x97Pa:\xD9\x83a:\x8CV[\x92PP`\x01\x81\x01\x90Pa:\xBAV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra;\x0C\x81\x84a:\x98V[\x90P\x92\x91PPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a;Z\x82a;\x14V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a;yWa;xa;$V[[\x80`@RPPPV[_a;\x8Ba5qV[\x90Pa;\x97\x82\x82a;QV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a;\xB6Wa;\xB5a;$V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a;\xD9a;\xD4\x84a;\x9CV[a;\x82V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a;\xFCWa;\xFBa6\xAEV[[\x83[\x81\x81\x10\x15a<%W\x80a<\x11\x88\x82a7\x98V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa;\xFEV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a<CWa<Ba6\xA6V[[\x815a<S\x84\x82` \x86\x01a;\xC7V[\x91PP\x92\x91PPV[__`@\x83\x85\x03\x12\x15a<rWa<qa5zV[[_a<\x7F\x85\x82\x86\x01a6TV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<\xA0Wa<\x9Fa5~V[[a<\xAC\x85\x82\x86\x01a</V[\x91PP\x92P\x92\x90PV[____`\x80\x85\x87\x03\x12\x15a<\xCEWa<\xCDa5zV[[_a<\xDB\x87\x82\x88\x01a92V[\x94PP` a<\xEC\x87\x82\x88\x01a6TV[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=\rWa=\x0Ca5~V[[a=\x19\x87\x82\x88\x01a</V[\x92PP``a=*\x87\x82\x88\x01a6TV[\x91PP\x92\x95\x91\x94P\x92PV[_`@\x82\x01\x90Pa=I_\x83\x01\x85a9\xD7V[a=V` \x83\x01\x84a8cV[\x93\x92PPPV[_``\x82\x01\x90Pa=p_\x83\x01\x86a9\xD7V[a=}` \x83\x01\x85a8cV[a=\x8A`@\x83\x01\x84a8cV[\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a=\xC9\x82a65V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a=\xFBWa=\xFAa=\x92V[[`\x01\x82\x01\x90P\x91\x90PV[_\x82\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a>TW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a>gWa>fa>\x10V[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a>\xC9\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a>\x8EV[a>\xD3\x86\x83a>\x8EV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a?\x0Ea?\ta?\x04\x84a65V[a>\xEBV[a65V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a?'\x83a>\xF4V[a?;a?3\x82a?\x15V[\x84\x84Ta>\x9AV[\x82UPPPPV[__\x90P\x90V[a?Ra?CV[a?]\x81\x84\x84a?\x1EV[PPPV[_[\x82\x81\x10\x15a?\x83Wa?x_\x82\x84\x01a?JV[`\x01\x81\x01\x90Pa?dV[PPPV[`\x1F\x82\x11\x15a?\xD6W\x82\x82\x11\x15a?\xD5Wa?\xA2\x81a>mV[a?\xAB\x83a>\x7FV[a?\xB4\x85a>\x7FV[` \x86\x10\x15a?\xC1W_\x90P[\x80\x83\x01a?\xD0\x82\x84\x03\x82a?bV[PPPP[[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_a?\xF6_\x19\x84`\x08\x02a?\xDBV[\x19\x80\x83\x16\x91PP\x92\x91PPV[_a@\x0E\x83\x83a?\xE7V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a@(\x83\x83a>\x06V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a@AWa@@a;$V[[a@K\x82Ta>=V[a@V\x82\x82\x85a?\x88V[_`\x1F\x83\x11`\x01\x81\x14a@\x83W_\x84\x15a@qW\x82\x87\x015\x90P[a@{\x85\x82a@\x03V[\x86UPa@\xE2V[`\x1F\x19\x84\x16a@\x91\x86a>mV[_[\x82\x81\x10\x15a@\xB8W\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa@\x93V[\x86\x83\x10\x15a@\xD5W\x84\x89\x015a@\xD1`\x1F\x89\x16\x82a?\xE7V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[_a@\xF5\x82a65V[\x91PaA\0\x83a65V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15aA\x18WaA\x17a=\x92V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FBUG: the authentication votes by_\x82\x01R\x7F honest clients are inconsistent` \x82\x01RPV[_aA\xB5`@\x83aAKV[\x91PaA\xC0\x82aA[V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaA\xE2\x81aA\xA9V[\x90P\x91\x90PV[_``\x82\x01\x90PaA\xFC_\x83\x01\x86a8cV[aB\t` \x83\x01\x85a9\xD7V[aB\x16`@\x83\x01\x84a9\xD7V[\x94\x93PPPPV[_aB(\x82a65V[\x91PaB3\x83a65V[\x92P\x82\x82\x02aBA\x81a65V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17aBXWaBWa=\x92V[[P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x81T\x90P\x91\x90PV[_\x81\x90P`\x01\x80`\x01\x03\x83\x01\x04\x90P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[aB\xCE\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83` \x03`\x08\x02a?\xDBV[\x81T\x81\x16\x82UPPPV[aB\xE2\x81a>mV[aB\xED\x83\x82Ta@\x03V[\x80\x83U_\x82UPPPPV[` \x84\x10_\x81\x14aCTW`\x1F\x84\x11`\x01\x81\x14aC!WaC\x1A\x86\x85a@\x03V[\x83UaCNV[aC*\x83a>mV[aCB`\x01aC8\x88a>\x7FV[\x03`\x01\x83\x01a?bV[aCL\x87\x85aB\xD9V[P[PaC\xAEV[aC]\x85a>\x7FV[aCf\x85a>\x7FV[aCo\x84a>mV[\x82\x81\x01`\x1F\x89\x16\x80\x15aC\x8AWaC\x89\x81`\x01\x84\x03aB\x9EV[[\x84\x84\x11\x15aC\x9FWaC\x9E\x85\x85\x03\x83a?bV[[`\x01\x8A`\x02\x02\x17\x87UPPPPP[PPPPPV[h\x01\0\0\0\0\0\0\0\0\x84\x11\x15aC\xCFWaC\xCEa;$V[[` \x83\x10_\x81\x14aD\x18W` \x85\x10_\x81\x14aC\xF6WaC\xEF\x86\x85a@\x03V[\x83UaD\x12V[\x83`\xFF\x19\x16\x93P\x83aD\x07\x84a>mV[U`\x01\x86`\x02\x02\x01\x83U[PaD\"V[`\x01\x85`\x02\x02\x01\x82U[PPPPPV[\x80TaD4\x81a>=V[\x80\x84\x11\x15aDIWaDH\x84\x82\x84\x86aC\xB5V[[\x80\x84\x10\x15aD^WaD]\x84\x82\x84\x86aB\xF9V[[PPPPV[\x82\x81\x10\x15aD\x83WaDx_\x82\x84\x01a?JV[`\x01\x81\x01\x90PaDdV[PPPV[aD\x92_\x82aD)V[PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[_\x82\x14aD\xD1WaD\xD0aD\x95V[[aD\xDA\x81aD\x88V[PPV[_[\x82\x81\x10\x15aD\xFFWaD\xF4_\x82\x84\x01aD\xC1V[`\x01\x81\x01\x90PaD\xE0V[PPPV[\x81\x83\x10\x15aE;WaE\x15\x82aBxV[aE\x1E\x84aBxV[aE'\x83aB\x8CV[\x81\x81\x01aE6\x83\x85\x03\x82aD\xDEV[PPPP[PPPV[h\x01\0\0\0\0\0\0\0\0\x82\x11\x15aEZWaEYa;$V[[aEc\x81aBnV[\x82\x82UaEq\x83\x82\x84aE\x04V[PPPV[_\x81Q\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[aE\x9D\x82aE\x8AV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aE\xB6WaE\xB5a;$V[[aE\xC0\x82Ta>=V[aE\xCB\x82\x82\x85a?\x88V[_` \x90P`\x1F\x83\x11`\x01\x81\x14aE\xFCW_\x84\x15aE\xEAW\x82\x87\x01Q\x90P[aE\xF4\x85\x82a@\x03V[\x86UPaF[V[`\x1F\x19\x84\x16aF\n\x86a>mV[_[\x82\x81\x10\x15aF1W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaF\x0CV[\x86\x83\x10\x15aFNW\x84\x89\x01QaFJ`\x1F\x89\x16\x82a?\xE7V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[aFm\x82\x82aE\x94V[PPV[aFz\x82aEvV[aF\x84\x81\x83aE@V[aF\x8D\x83aB_V[aF\x96\x83aB\x8CV[_[\x83\x81\x10\x15aF\xCBWaF\xA9\x83aE\x80V[aF\xB3\x81\x84aFcV[` \x84\x01\x93P`\x01\x83\x01\x92PP`\x01\x81\x01\x90PaF\x98V[PPPPPPV[_aF\xDD\x82a65V[\x91PaF\xE8\x83a65V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15aG\0WaF\xFFa=\x92V[[\x92\x91PPV[_`@\x82\x01\x90PaG\x19_\x83\x01\x85a8cV[aG&` \x83\x01\x84a8cV[\x93\x92PPPV[_`@\x82\x01\x90PaG@_\x83\x01\x85a9\xD7V[aGM` \x83\x01\x84a9\xD7V[\x93\x92PPPV[\x7FBUG: ALREADY RECEIVED SHARES FRO_\x82\x01R\x7FM N PARTIES, TOO MANY CLIENTS\0\0\0` \x82\x01RPV[_aG\xAE`=\x83aAKV[\x91PaG\xB9\x82aGTV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaG\xDB\x81aG\xA2V[\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_aH\x1A\x82aE\x8AV[aH$\x81\x85aG\xF2V[\x93PaH4\x81\x85` \x86\x01aH\x02V[aH=\x81a;\x14V[\x84\x01\x91PP\x92\x91PPV[_aHS\x83\x83aH\x10V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aHq\x82aEvV[aH{\x81\x85aG\xE2V[\x93P\x83` \x82\x02\x85\x01aH\x8D\x85aB_V[\x80_[\x85\x81\x10\x15aH\xC8W\x84\x84\x03\x89R\x81QaH\xA9\x85\x82aHHV[\x94PaH\xB4\x83aH[V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaH\x90V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaH\xF2\x81\x84aHgV[\x90P\x92\x91PPV[_`@\x82\x01\x90PaI\r_\x83\x01\x85a8;V[aI\x1A` \x83\x01\x84a8;V[\x93\x92PPPV[_`@\x82\x01\x90PaI4_\x83\x01\x85a8cV[aIA` \x83\x01\x84a9\xD7V[\x93\x92PPPV[_``\x82\x01\x90PaI[_\x83\x01\x86a8cV[aIh` \x83\x01\x85a8cV[aIu`@\x83\x01\x84a9\xD7V[\x94\x93PPPPV[_`\x80\x82\x01\x90PaI\x90_\x83\x01\x87a9\xD7V[aI\x9D` \x83\x01\x86a8cV[aI\xAA`@\x83\x01\x85a8cV[aI\xB7``\x83\x01\x84a9\xD7V[\x95\x94PPPPPV[_`@\x82\x01\x90PaI\xD3_\x83\x01\x85a9\xD7V[aI\xE0` \x83\x01\x84a9qV[\x93\x92PPPV[_`\xFF\x82\x16\x90P\x91\x90PV[aI\xFC\x81aI\xE7V[\x82RPPV[_`\x80\x82\x01\x90PaJ\x15_\x83\x01\x87a9qV[aJ\"` \x83\x01\x86aI\xF3V[aJ/`@\x83\x01\x85a9qV[aJ<``\x83\x01\x84a9qV[\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 jo8\xD4`^\xF3\x7Fq\x17\";\xC9\xCF\xD0\rJ \n\xB7\x97\x1Ew^\x81\x8A\x10.\xDB\xC2\xB1CdsolcC\0\x08!\x003",
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
    /**Custom error with signature `ClientAlreadyReservedIndex(address,uint256)` and selector `0xc315a0f5`.
```solidity
error ClientAlreadyReservedIndex(address client, uint256 i);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ClientAlreadyReservedIndex {
        #[allow(missing_docs)]
        pub client: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub i: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<ClientAlreadyReservedIndex>
        for UnderlyingRustTuple<'_> {
            fn from(value: ClientAlreadyReservedIndex) -> Self {
                (value.client, value.i)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ClientAlreadyReservedIndex {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    client: tuple.0,
                    i: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ClientAlreadyReservedIndex {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ClientAlreadyReservedIndex(address,uint256)";
            const SELECTOR: [u8; 4] = [195u8, 21u8, 160u8, 245u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.i),
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
    /**Custom error with signature `IndexAlreadyReserved(uint256,address,address)` and selector `0xa0b8c708`.
```solidity
error IndexAlreadyReserved(uint256 i, address reqClient, address resClient);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IndexAlreadyReserved {
        #[allow(missing_docs)]
        pub i: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub reqClient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub resClient: alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<IndexAlreadyReserved> for UnderlyingRustTuple<'_> {
            fn from(value: IndexAlreadyReserved) -> Self {
                (value.i, value.reqClient, value.resClient)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for IndexAlreadyReserved {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    i: tuple.0,
                    reqClient: tuple.1,
                    resClient: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for IndexAlreadyReserved {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IndexAlreadyReserved(uint256,address,address)";
            const SELECTOR: [u8; 4] = [160u8, 184u8, 199u8, 8u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.i),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.reqClient,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.resClient,
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
    /**Custom error with signature `OutOfIndices(address)` and selector `0x4ae235f9`.
```solidity
error OutOfIndices(address client);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OutOfIndices {
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
        impl ::core::convert::From<OutOfIndices> for UnderlyingRustTuple<'_> {
            fn from(value: OutOfIndices) -> Self {
                (value.client,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OutOfIndices {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { client: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OutOfIndices {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OutOfIndices(address)";
            const SELECTOR: [u8; 4] = [74u8, 226u8, 53u8, 249u8];
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
    /**Event with signature `ReservedInputEvent(address,uint256)` and selector `0xabde16b7a9192c31c6231b1539bad6fed77635de4c008718dbdcafb7b8363afe`.
```solidity
event ReservedInputEvent(address client, uint256 reservedIndex);
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
        impl alloy_sol_types::SolEvent for ReservedInputEvent {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ReservedInputEvent(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                171u8, 222u8, 22u8, 183u8, 169u8, 25u8, 44u8, 49u8, 198u8, 35u8, 27u8,
                21u8, 57u8, 186u8, 214u8, 254u8, 215u8, 118u8, 53u8, 222u8, 76u8, 0u8,
                135u8, 24u8, 219u8, 220u8, 175u8, 183u8, 184u8, 54u8, 58u8, 254u8,
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
                    reservedIndex: data.1,
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
    /**Function with signature `reserveMaskIndex(uint256)` and selector `0x21dc7b9b`.
```solidity
function reserveMaskIndex(uint256 i) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reserveMaskIndexCall {
        #[allow(missing_docs)]
        pub i: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`reserveMaskIndex(uint256)`](reserveMaskIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reserveMaskIndexReturn {}
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
            impl ::core::convert::From<reserveMaskIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: reserveMaskIndexCall) -> Self {
                    (value.i,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for reserveMaskIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { i: tuple.0 }
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
            impl ::core::convert::From<reserveMaskIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: reserveMaskIndexReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for reserveMaskIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl reserveMaskIndexReturn {
            fn _tokenize(
                &self,
            ) -> <reserveMaskIndexCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for reserveMaskIndexCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = reserveMaskIndexReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "reserveMaskIndex(uint256)";
            const SELECTOR: [u8; 4] = [33u8, 220u8, 123u8, 155u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.i),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                reserveMaskIndexReturn::_tokenize(ret)
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
function resetInputManager(uint256 nIndicesToReserve, uint256 _t) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resetInputManagerCall {
        #[allow(missing_docs)]
        pub nIndicesToReserve: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _t: alloy::sol_types::private::primitives::aliases::U256,
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
                    (value.nIndicesToReserve, value._t)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for resetInputManagerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        nIndicesToReserve: tuple.0,
                        _t: tuple.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self._t),
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
        owner(ownerCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        renounceRole(renounceRoleCall),
        #[allow(missing_docs)]
        reserveInputMasks(reserveInputMasksCall),
        #[allow(missing_docs)]
        reserveMaskIndex(reserveMaskIndexCall),
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
            [33u8, 220u8, 123u8, 155u8],
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
            ::core::stringify!(reserveMaskIndex),
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
            <reserveMaskIndexCall as alloy_sol_types::SolCall>::SIGNATURE,
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
                Self::reserveMaskIndex(_) => {
                    <reserveMaskIndexCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn reserveMaskIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <reserveMaskIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::reserveMaskIndex)
                    }
                    reserveMaskIndex
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
                    fn reserveMaskIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <reserveMaskIndexCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::reserveMaskIndex)
                    }
                    reserveMaskIndex
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
                Self::reserveMaskIndex(inner) => {
                    <reserveMaskIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::reserveMaskIndex(inner) => {
                    <reserveMaskIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        ClientAlreadyReservedIndex(ClientAlreadyReservedIndex),
        #[allow(missing_docs)]
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        #[allow(missing_docs)]
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        #[allow(missing_docs)]
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
        #[allow(missing_docs)]
        IndexAlreadyReserved(IndexAlreadyReserved),
        #[allow(missing_docs)]
        IndexNotReserved(IndexNotReserved),
        #[allow(missing_docs)]
        NoIndicesReserved(NoIndicesReserved),
        #[allow(missing_docs)]
        NotAClient(NotAClient),
        #[allow(missing_docs)]
        NotAnExistingParty(NotAnExistingParty),
        #[allow(missing_docs)]
        NotAtRound(NotAtRound),
        #[allow(missing_docs)]
        NotEnoughMPCParties(NotEnoughMPCParties),
        #[allow(missing_docs)]
        OutOfIndices(OutOfIndices),
        #[allow(missing_docs)]
        OwnableInvalidOwner(OwnableInvalidOwner),
        #[allow(missing_docs)]
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
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
            [74u8, 226u8, 53u8, 249u8],
            [79u8, 95u8, 191u8, 195u8],
            [102u8, 151u8, 178u8, 50u8],
            [111u8, 175u8, 159u8, 5u8],
            [160u8, 50u8, 172u8, 107u8],
            [160u8, 184u8, 199u8, 8u8],
            [171u8, 220u8, 224u8, 106u8],
            [191u8, 162u8, 23u8, 216u8],
            [195u8, 21u8, 160u8, 245u8],
            [215u8, 139u8, 206u8, 12u8],
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
            ::core::stringify!(OutOfIndices),
            ::core::stringify!(AlreadySubmittedInputs),
            ::core::stringify!(AccessControlBadConfirmation),
            ::core::stringify!(NoIndicesReserved),
            ::core::stringify!(NotAClient),
            ::core::stringify!(IndexAlreadyReserved),
            ::core::stringify!(NotAnExistingParty),
            ::core::stringify!(NotAtRound),
            ::core::stringify!(ClientAlreadyReservedIndex),
            ::core::stringify!(ECDSAInvalidSignatureS),
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
            <OutOfIndices as alloy_sol_types::SolError>::SIGNATURE,
            <AlreadySubmittedInputs as alloy_sol_types::SolError>::SIGNATURE,
            <AccessControlBadConfirmation as alloy_sol_types::SolError>::SIGNATURE,
            <NoIndicesReserved as alloy_sol_types::SolError>::SIGNATURE,
            <NotAClient as alloy_sol_types::SolError>::SIGNATURE,
            <IndexAlreadyReserved as alloy_sol_types::SolError>::SIGNATURE,
            <NotAnExistingParty as alloy_sol_types::SolError>::SIGNATURE,
            <NotAtRound as alloy_sol_types::SolError>::SIGNATURE,
            <ClientAlreadyReservedIndex as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SIGNATURE,
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
                Self::ClientAlreadyReservedIndex(_) => {
                    <ClientAlreadyReservedIndex as alloy_sol_types::SolError>::SELECTOR
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
                Self::IndexAlreadyReserved(_) => {
                    <IndexAlreadyReserved as alloy_sol_types::SolError>::SELECTOR
                }
                Self::IndexNotReserved(_) => {
                    <IndexNotReserved as alloy_sol_types::SolError>::SELECTOR
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
                Self::NotEnoughMPCParties(_) => {
                    <NotEnoughMPCParties as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OutOfIndices(_) => {
                    <OutOfIndices as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OwnableInvalidOwner(_) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OwnableUnauthorizedAccount(_) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::SELECTOR
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
                    fn OutOfIndices(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <OutOfIndices as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(FakeCoordinatorErrors::OutOfIndices)
                    }
                    OutOfIndices
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
                    fn IndexAlreadyReserved(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <IndexAlreadyReserved as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorErrors::IndexAlreadyReserved)
                    }
                    IndexAlreadyReserved
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
                    fn NotAtRound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <NotAtRound as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(FakeCoordinatorErrors::NotAtRound)
                    }
                    NotAtRound
                },
                {
                    fn ClientAlreadyReservedIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <ClientAlreadyReservedIndex as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorErrors::ClientAlreadyReservedIndex)
                    }
                    ClientAlreadyReservedIndex
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
                    fn OutOfIndices(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <OutOfIndices as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::OutOfIndices)
                    }
                    OutOfIndices
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
                    fn IndexAlreadyReserved(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <IndexAlreadyReserved as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::IndexAlreadyReserved)
                    }
                    IndexAlreadyReserved
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
                    fn ClientAlreadyReservedIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <ClientAlreadyReservedIndex as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::ClientAlreadyReservedIndex)
                    }
                    ClientAlreadyReservedIndex
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
                Self::ClientAlreadyReservedIndex(inner) => {
                    <ClientAlreadyReservedIndex as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::IndexAlreadyReserved(inner) => {
                    <IndexAlreadyReserved as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::IndexNotReserved(inner) => {
                    <IndexNotReserved as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::NotEnoughMPCParties(inner) => {
                    <NotEnoughMPCParties as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OutOfIndices(inner) => {
                    <OutOfIndices as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::ClientAlreadyReservedIndex(inner) => {
                    <ClientAlreadyReservedIndex as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::IndexAlreadyReserved(inner) => {
                    <IndexAlreadyReserved as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::NotEnoughMPCParties(inner) => {
                    <NotEnoughMPCParties as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OutOfIndices(inner) => {
                    <OutOfIndices as alloy_sol_types::SolError>::abi_encode_raw(
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
                171u8, 222u8, 22u8, 183u8, 169u8, 25u8, 44u8, 49u8, 198u8, 35u8, 27u8,
                21u8, 57u8, 186u8, 214u8, 254u8, 215u8, 118u8, 53u8, 222u8, 76u8, 0u8,
                135u8, 24u8, 219u8, 220u8, 175u8, 183u8, 184u8, 54u8, 58u8, 254u8,
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
            ::core::stringify!(ReservedInputEvent),
            ::core::stringify!(MaskedInputEvent),
            ::core::stringify!(PreprocessingStarted),
            ::core::stringify!(RoleAdminChanged),
            ::core::stringify!(CoordinatorInitialized),
            ::core::stringify!(RoleRevoked),
            ::core::stringify!(IndexBufferEvent),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
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
            <ReservedInputEvent as alloy_sol_types::SolEvent>::SIGNATURE,
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
        ///Creates a new call builder for the [`reserveMaskIndex`] function.
        pub fn reserveMaskIndex(
            &self,
            i: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, reserveMaskIndexCall, N> {
            self.call_builder(&reserveMaskIndexCall { i })
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
            _t: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, resetInputManagerCall, N> {
            self.call_builder(
                &resetInputManagerCall {
                    nIndicesToReserve,
                    _t,
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

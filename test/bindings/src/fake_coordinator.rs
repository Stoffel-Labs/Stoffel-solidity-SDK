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
    error IndexAlreadyReserved(uint256 i, address reqClient, address resClient);
    error IndexNotReserved(address client, uint256 index);
    error IndexOutOfBounds(address client, uint256 index);
    error NoIndicesReserved(address client);
    error NotAnExistingParty(address account);
    error NotAtRound(StoffelCoordinator.Round required, StoffelCoordinator.Round current);
    error NotEnoughMPCParties(uint256 current, uint256 required);
    error OutputClientNotRegistered(address client);
    error OwnableInvalidOwner(address owner);
    error OwnableUnauthorizedAccount(address account);
    error RoleChangeNotAllowed(StoffelCoordinator.Round current);
    error ZeroMaskedInput(address client);

    event CoordinatorInitialized(address coordinator, uint256 timeofInitialization, uint256 creationBlock, address designatedParty);
    event CoordinatorReset(address coordinator, uint256 lastResetBlock);
    event EnoughOutputShares(address indexed client, bytes[] shares);
    event ExecutionDone(address executor, uint256 timeOfExecution);
    event IndexBufferEvent(uint256 totalIndices, address designatedParty);
    event InitializeStoffelAccessControl(uint256 nParties, uint256 t, address initializer);
    event InputCollectionStarted(address executor, uint256 timeOfExecution);
    event InputMaskReservationStarted(address executor, uint256 timeOfExecution);
    event MPCStarted(address executor, uint256 timeOfExecution);
    event MaskedInputEvent(address client, bytes maskedInput, uint256 reservedIndex);
    event OutputSendingStarted(address executor, uint256 timeOfExecution);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event PreprocessingStarted(address designatedParty, uint256 timeOfExecution);
    event ReservedInputEvent(address client, uint256 reservedIndex);
    event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole);
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);

    constructor(bytes32 stoffelProgramHash, uint256 t, address[] initialMpcNodes, uint256 nInputs, address[] outputClients, uint256 reconstructionThreshold);

    function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
    function DESIGNATED_PARTY_ROLE() external view returns (bytes32);
    function INPUT_CLIENT_ROLE() external view returns (bytes32);
    function OUTPUT_CLIENT_ROLE() external view returns (bytes32);
    function PARTY_ROLE() external view returns (bytes32);
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
    function lastResetBlock() external view returns (uint256);
    function owner() external view returns (address);
    function renounceOwnership() external;
    function renounceRole(bytes32 role, address account) external;
    function reserveInputMasks() external;
    function reserveMaskIndex(uint256 i) external;
    function resetCoordinator() external;
    function revokeRole(bytes32 role, address account) external;
    function round() external view returns (StoffelCoordinator.Round);
    function sendOutputShares(address client, bytes memory shares) external;
    function sendOutputs() external;
    function startMpc() external;
    function startPreprocessing() external;
    function submitMaskedInput(bytes memory maskedInput, uint256 reservedIndex) external;
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
      },
      {
        "name": "outputClients",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "reconstructionThreshold",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
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
    "name": "INPUT_CLIENT_ROLE",
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
    "name": "OUTPUT_CLIENT_ROLE",
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
    "name": "lastResetBlock",
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
    "name": "resetCoordinator",
    "inputs": [],
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
    "name": "sendOutputShares",
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
    "name": "sendOutputs",
    "inputs": [],
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
        "type": "bytes",
        "internalType": "bytes"
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
    "name": "CoordinatorReset",
    "inputs": [
      {
        "name": "coordinator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "lastResetBlock",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EnoughOutputShares",
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
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
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
    "name": "IndexOutOfBounds",
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
    "name": "OutputClientNotRegistered",
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
    "name": "RoleChangeNotAllowed",
    "inputs": [
      {
        "name": "current",
        "type": "uint8",
        "internalType": "enum StoffelCoordinator.Round"
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
    ///0x60e060405234801561000f575f5ffd5b50604051614b5f380380614b5f833981810160405281019061003191906109b0565b8585858585853383838388888160808181525050600160805160036100569190610a9e565b6100609190610adf565b60a0818152505060a05181511015815160a05190916100b6576040517f3a2362680000000000000000000000000000000000000000000000000000000081526004016100ad929190610b21565b60405180910390fd5b50505f5f90505b815181101561011e576101107fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698383815181106100fd576100fc610b48565b5b60200260200101516103de60201b60201c565b5080806001019150506100bd565b506101697f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e825f8151811061015657610155610b48565b5b60200260200101516103de60201b60201c565b507fa2df7830e0bedef7b1116bf547b467b16b50b3bd23146c9e099878d14e89301a60a051608051336040516101a193929190610b84565b60405180910390a150508060c081815250505f600781905550826004819055505f6005819055505f6006819055505f5f90505b82518110156102eb576102277f601a28e6abeea52b0ba15b54f6d15d789fd3e8c0080c042a009a9964f0e18f1c84838151811061021457610213610b48565b5b60200260200101516103de60201b60201c565b5060a05167ffffffffffffffff8111156102445761024361081a565b5b60405190808252806020026020018201604052801561027757816020015b60608152602001906001900390816102625790505b5060025f85848151811061028e5761028d610b48565b5b602002602001015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f0190816102dd91906111d9565b5080806001019150506101d4565b507ff7f0872382dff5e698b284e12084e4e7894f830216dd80cb4e909b593a58f9356004543360405161031f92919061123b565b60405180910390a15050505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff160361039a575f6040517f1e4fbdf70000000000000000000000000000000000000000000000000000000081526004016103919190611262565b60405180910390fd5b6103a9816103f760201b60201c565b5085600a8190555042600b8190555043600c819055506103cd6104ba60201b60201c565b5050505050505050505050506112cf565b5f6103ef838361052760201b60201c565b905092915050565b5f60095f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690508160095f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a35050565b43600d819055505f600e5f6101000a81548160ff021916908360068111156104e5576104e461127b565b5b02179055507f51fb20da0aafaceb18d92ff1a476059a0a8bbf16a0bf7c38b94a98b356ace45730600d5460405161051d9291906112a8565b60405180910390a1565b5f5f610539848461057060201b60201c565b90508015610566576105648360015f8781526020019081526020015f2061066560201b90919060201c565b505b8091505092915050565b5f610581838361069860201b60201c565b61065b5760015f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055506105f86106fb60201b60201c565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a46001905061065f565b5f90505b92915050565b5f610690835f018373ffffffffffffffffffffffffffffffffffffffff165f1b61070260201b60201c565b905092915050565b5f5f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f33905090565b5f610713838361076f60201b60201c565b61076557825f0182908060018154018082558091505060019003905f5260205f20015f9091909190915055825f0180549050836001015f8481526020019081526020015f208190555060019050610769565b5f90505b92915050565b5f5f836001015f8481526020019081526020015f20541415905092915050565b5f604051905090565b5f5ffd5b5f5ffd5b5f819050919050565b6107b2816107a0565b81146107bc575f5ffd5b50565b5f815190506107cd816107a9565b92915050565b5f819050919050565b6107e5816107d3565b81146107ef575f5ffd5b50565b5f81519050610800816107dc565b92915050565b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6108508261080a565b810181811067ffffffffffffffff8211171561086f5761086e61081a565b5b80604052505050565b5f61088161078f565b905061088d8282610847565b919050565b5f67ffffffffffffffff8211156108ac576108ab61081a565b5b602082029050602081019050919050565b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6108ea826108c1565b9050919050565b6108fa816108e0565b8114610904575f5ffd5b50565b5f81519050610915816108f1565b92915050565b5f61092d61092884610892565b610878565b905080838252602082019050602084028301858111156109505761094f6108bd565b5b835b8181101561097957806109658882610907565b845260208401935050602081019050610952565b5050509392505050565b5f82601f83011261099757610996610806565b5b81516109a784826020860161091b565b91505092915050565b5f5f5f5f5f5f60c087890312156109ca576109c9610798565b5b5f6109d789828a016107bf565b96505060206109e889828a016107f2565b955050604087015167ffffffffffffffff811115610a0957610a0861079c565b5b610a1589828a01610983565b9450506060610a2689828a016107f2565b935050608087015167ffffffffffffffff811115610a4757610a4661079c565b5b610a5389828a01610983565b92505060a0610a6489828a016107f2565b9150509295509295509295565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610aa8826107d3565b9150610ab3836107d3565b9250828202610ac1816107d3565b91508282048414831517610ad857610ad7610a71565b5b5092915050565b5f610ae9826107d3565b9150610af4836107d3565b9250828201905080821115610b0c57610b0b610a71565b5b92915050565b610b1b816107d3565b82525050565b5f604082019050610b345f830185610b12565b610b416020830184610b12565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b610b7e816108e0565b82525050565b5f606082019050610b975f830186610b12565b610ba46020830185610b12565b610bb16040830184610b75565b949350505050565b5f819050602082019050919050565b5f81549050919050565b5f8190506001806001038301049050919050565b5f819050815f5260205f209050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f6002820490506001821680610c3c57607f821691505b602082108103610c4f57610c4e610bf8565b5b50919050565b5f819050815f5260205f209050919050565b5f82821c905092915050565b610ca37fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff83602003600802610c67565b815481168255505050565b5f82821b905092915050565b5f60088302610ce97fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82610cae565b610cf38683610cae565b95508019841693508086168417925050509392505050565b5f819050919050565b5f610d2e610d29610d24846107d3565b610d0b565b6107d3565b9050919050565b5f819050919050565b610d4783610d14565b610d5b610d5382610d35565b848454610cba565b825550505050565b5f5f905090565b610d72610d63565b610d7d818484610d3e565b505050565b5f5b82811015610da357610d985f828401610d6a565b600181019050610d84565b505050565b5f610db75f1984600802610c67565b1980831691505092915050565b5f610dcf8383610da8565b9150826002028217905092915050565b610de881610c55565b610df3838254610dc4565b8083555f825550505050565b5f6020601f8301049050919050565b602084105f8114610e6957601f841160018114610e3657610e2f8685610dc4565b8355610e63565b610e3f83610c55565b610e576001610e4d88610dff565b0360018301610d82565b610e618785610ddf565b505b50610ec3565b610e7285610dff565b610e7b85610dff565b610e8484610c55565b828101601f89168015610e9f57610e9e8160018403610c73565b5b84841115610eb457610eb385850383610d82565b5b60018a60020217875550505050505b5050505050565b68010000000000000000841115610ee457610ee361081a565b5b602083105f8114610f2d57602085105f8114610f0b57610f048685610dc4565b8355610f27565b8360ff1916935083610f1c84610c55565b556001866002020183555b50610f37565b6001856002020182555b5050505050565b8054610f4981610c25565b80841115610f5e57610f5d84828486610eca565b5b80841015610f7357610f7284828486610e0e565b5b50505050565b82811015610f9857610f8d5f828401610d6a565b600181019050610f79565b505050565b610fa75f82610f3e565b50565b7f4e487b71000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b5f8214610fe657610fe5610faa565b5b610fef81610f9d565b5050565b5f5b82811015611014576110095f828401610fd6565b600181019050610ff5565b505050565b818310156110505761102a82610bd2565b61103384610bd2565b61103c83610be6565b81810161104b83850382610ff3565b505050505b505050565b6801000000000000000082111561106f5761106e61081a565b5b61107881610bc8565b828255611086838284611019565b505050565b5f81519050919050565b5f81519050919050565b5f81519050919050565b601f8211156110f757828211156110f6576110c381610c55565b6110cc83610dff565b6110d585610dff565b60208610156110e2575f90505b8083016110f182840382610d82565b505050505b5b505050565b6111058261109f565b67ffffffffffffffff81111561111e5761111d61081a565b5b6111288254610c25565b6111338282856110a9565b5f60209050601f831160018114611164575f8415611152578287015190505b61115c8582610dc4565b8655506111c3565b601f19841661117286610c55565b5f5b8281101561119957848901518255600182019150602085019450602081019050611174565b868310156111b657848901516111b2601f891682610da8565b8355505b6001600288020188555050505b505050505050565b6111d582826110fc565b5050565b6111e28261108b565b6111ec8183611055565b6111f583610bb9565b6111fe83610be6565b5f5b838110156112335761121183611095565b61121b81846111cb565b60208401935060018301925050600181019050611200565b505050505050565b5f60408201905061124e5f830185610b12565b61125b6020830184610b75565b9392505050565b5f6020820190506112755f830184610b75565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b5f6040820190506112bb5f830185610b75565b6112c86020830184610b12565b9392505050565b60805160a05160c05161385461130b5f395f611e6601525f818161185801528181611b7b01528181611ba00152611d5d01525f50506138545ff3fe608060405234801561000f575f5ffd5b506004361061020f575f3560e01c80635cb86b7411610123578063bb51fef0116100ab578063d547741f1161007a578063d547741f1461056d578063d8270dce14610589578063ede69216146105a7578063f2fde38b146105c3578063fc78b2e8146105df5761020f565b8063bb51fef01461051f578063c079f49514610529578063ca15c87314610533578063cb9c4cc4146105635761020f565b80638da5cb5b116100f25780638da5cb5b146104535780639010d07c1461047157806391d14854146104a1578063a217fddf146104d1578063a3246ad3146104ef5761020f565b80635cb86b74146104035780636b5e12ca1461040d578063715018a61461042b5780637f35b560146104355761020f565b80632f2ff15d116101a657806349f2ada01161017557806349f2ada0146103975780634b8e6488146103b55780634bb278f3146103bf5780635648526c146103c957806358df0d01146103e55761020f565b80632f2ff15d1461033757806330104c3e1461035357806333cc9a091461037157806336568abe1461037b5761020f565b80631c7453db116101e25780631c7453db146102af57806321dc7b9b146102cd5780632328bd12146102e9578063248a9ca3146103075761020f565b806301ffc9a71461021357806313ff6dd514610243578063146ca531146102735780631763451414610291575b5f5ffd5b61022d6004803603810190610228919061270d565b61060f565b60405161023a9190612752565b60405180910390f35b61025d600480360381019061025891906127c5565b610688565b60405161026a9190612752565b60405180910390f35b61027b6106cb565b6040516102889190612863565b60405180910390f35b6102996106dd565b6040516102a69190612894565b60405180910390f35b6102b76106e3565b6040516102c49190612894565b60405180910390f35b6102e760048036038101906102e291906128d7565b6106e9565b005b6102f16109a0565b6040516102fe9190612894565b60405180910390f35b610321600480360381019061031c9190612935565b6109b6565b60405161032e919061296f565b60405180910390f35b610351600480360381019061034c9190612988565b6109d2565b005b61035b610a14565b604051610368919061296f565b60405180910390f35b610379610a38565b005b61039560048036038101906103909190612988565b610ab2565b005b61039f610ac8565b6040516103ac919061296f565b60405180910390f35b6103bd610aec565b005b6103c7610b66565b005b6103e360048036038101906103de9190612a27565b610be0565b005b6103ed610e9c565b6040516103fa919061296f565b60405180910390f35b61040b610ec0565b005b610415610efd565b6040516104229190612894565b60405180910390f35b610433610f03565b005b61043d610f16565b60405161044a919061296f565b60405180910390f35b61045b610f3a565b6040516104689190612a93565b60405180910390f35b61048b60048036038101906104869190612aac565b610f62565b6040516104989190612a93565b60405180910390f35b6104bb60048036038101906104b69190612988565b610f8e565b6040516104c89190612752565b60405180910390f35b6104d9610ff1565b6040516104e6919061296f565b60405180910390f35b61050960048036038101906105049190612935565b610ff7565b6040516105169190612ba1565b60405180910390f35b610527611019565b005b610531611093565b005b61054d60048036038101906105489190612935565b61110d565b60405161055a9190612894565b60405180910390f35b61056b61112e565b005b61058760048036038101906105829190612988565b6111af565b005b6105916111f1565b60405161059e9190612894565b60405180910390f35b6105c160048036038101906105bc9190612bc1565b6111f7565b005b6105dd60048036038101906105d891906127c5565b61123e565b005b6105f960048036038101906105f491906127c5565b6112c2565b6040516106069190612752565b60405180910390f35b5f7f07effe0d000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191614806106815750610680826112f4565b5b9050919050565b5f610692826112c2565b80156106c457506106c37f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e83610f8e565b5b9050919050565b600e5f9054906101000a900460ff1681565b600c5481565b60075481565b600454811033829091610733576040517f6867a17000000000000000000000000000000000000000000000000000000000815260040161072a929190612c1e565b60405180910390fd5b50505f5f90505b6004548110156107f8573373ffffffffffffffffffffffffffffffffffffffff1660035f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff161415338290916107e9576040517fc315a0f50000000000000000000000000000000000000000000000000000000081526004016107e0929190612c1e565b60405180910390fd5b5050808060010191505061073a565b505f73ffffffffffffffffffffffffffffffffffffffff1660035f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1614813360035f8581526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff169091926108d0576040517fa0b8c7080000000000000000000000000000000000000000000000000000000081526004016108c793929190612c45565b60405180910390fd5b5050503360035f8381526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555061094c7fd68e3e5e367fee47ce11a5dab404596980e158eb9069330a8f775de7dc6bb64d3361136d565b5060055f81548092919061095f90612ca7565b91905055507fabde16b7a9192c31c6231b1539bad6fed77635de4c008718dbdcafb7b8363afe3382604051610995929190612c1e565b60405180910390a150565b5f6005546004546109b19190612cee565b905090565b5f5f5f8381526020019081526020015f20600101549050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6109fc81611380565b610a04611394565b610a0e838361136d565b50505050565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46981565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610a6281611380565b6003610a6d8161141c565b7f20f55ed0c92f2bb1c8825488e1e3c98463d024b2a42dbd24838c3f75260f43e93342604051610a9e929190612c1e565b60405180910390a1610aae6114a9565b5050565b610aba611394565b610ac48282611512565b5050565b7fd68e3e5e367fee47ce11a5dab404596980e158eb9069330a8f775de7dc6bb64d81565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610b1681611380565b6004610b218161141c565b7f301f8a3701f5b260197382dd7301078542144fe8fddd18083d6f6e09e4958a593342604051610b52929190612c1e565b60405180910390a1610b626114a9565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610b9081611380565b6005610b9b8161141c565b7f24a873651d26fb5a462cb37a91071cdd4d09abeebfe0ed144329bed1cc359d033342604051610bcc929190612c1e565b60405180910390a1610bdc6114a9565b5050565b7fd68e3e5e367fee47ce11a5dab404596980e158eb9069330a8f775de7dc6bb64d610c0a81611380565b3373ffffffffffffffffffffffffffffffffffffffff1660035f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff161433839091610cae576040517fffabbae7000000000000000000000000000000000000000000000000000000008152600401610ca5929190612c1e565b60405180910390fd5b50505f8484905014153390610cf9576040517f16923cea000000000000000000000000000000000000000000000000000000008152600401610cf09190612a93565b60405180910390fd5b505f60085f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206001018054610d4690612d4e565b9050143390610d8b576040517f4f5fbfc3000000000000000000000000000000000000000000000000000000008152600401610d829190612a93565b60405180910390fd5b50604051806040016040528083815260200185858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081525060085f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f820151815f01556020820151816001019081610e3e9190612f66565b509050507f56d03e5f1ebec3d4b4f9ded07e82c6bb6897c142cfbaf8dff8f9ef897ce4f75f33858585604051610e77949392919061308f565b60405180910390a160065f815480929190610e9190612ca7565b919050555050505050565b7f601a28e6abeea52b0ba15b54f6d15d789fd3e8c0080c042a009a9964f0e18f1c81565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610eea81611380565b610ef261158d565b610efa61194d565b50565b600d5481565b610f0b6119ba565b610f145f611a41565b565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e81565b5f60095f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b5f610f868260015f8681526020019081526020015f20611b0490919063ffffffff16565b905092915050565b5f5f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f5f1b81565b606061101260015f8481526020019081526020015f20611b1b565b9050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61104381611380565b600261104e8161141c565b7f60edf9bdc7c4ea007cae1a9bbd03e41e5bfccd7231a6ec383c2edd7800f0d20c334260405161107f929190612c1e565b60405180910390a161108f6114a9565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6110bd81611380565b60016110c88161141c565b7f67c4489f674d03c7d19a9e36735188de7c65e8d1e99eb3a2fd258a769eb14fff33426040516110f9929190612c1e565b60405180910390a16111096114a9565b5050565b5f61112760015f8481526020019081526020015f20611b3a565b9050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61115881611380565b5f6111628161141c565b61116a611b4d565b7fbb709dea744f06d1b26e824deec2f7140c511266ee15d7a217838b31d8b0123d334260405161119b929190612c1e565b60405180910390a16111ab6114a9565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6111d981611380565b6111e1611394565b6111eb8383611c07565b50505050565b600b5481565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46961122181611380565b600561122c8161141c565b611237858585611c1a565b5050505050565b6112466119ba565b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16036112b6575f6040517f1e4fbdf70000000000000000000000000000000000000000000000000000000081526004016112ad9190612a93565b60405180910390fd5b6112bf81611a41565b50565b5f6112ed7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46983610f8e565b9050919050565b5f7f5a05180f000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916148061136657506113658261201b565b5b9050919050565b5f6113788383612094565b905092915050565b6113918161138c6120d7565b6120de565b50565b6006808111156113a7576113a66127f0565b5b600e5f9054906101000a900460ff1660068111156113c8576113c76127f0565b5b14600e5f9054906101000a900460ff1690611419576040517f630180540000000000000000000000000000000000000000000000000000000081526004016114109190612863565b60405180910390fd5b50565b80600681111561142f5761142e6127f0565b5b600e5f9054906101000a900460ff1660068111156114505761144f6127f0565b5b1481600e5f9054906101000a900460ff1690916114a4576040517fbfa217d800000000000000000000000000000000000000000000000000000000815260040161149b9291906130cd565b60405180910390fd5b505050565b6001600e5f9054906101000a900460ff1660068111156114cc576114cb6127f0565b5b6114d691906130f4565b60068111156114e8576114e76127f0565b5b600e5f6101000a81548160ff0219169083600681111561150b5761150a6127f0565b5b0217905550565b61151a6120d7565b73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff161461157e576040517f6697b23200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6115888282611c07565b505050565b5f6115b77fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469610ff7565b90505f6115e37fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46961110d565b90505f61160f7f601a28e6abeea52b0ba15b54f6d15d789fd3e8c0080c042a009a9964f0e18f1c610ff7565b90505f61163b7f601a28e6abeea52b0ba15b54f6d15d789fd3e8c0080c042a009a9964f0e18f1c61110d565b90505f5f90505b600454811015611716575f60035f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905060085f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f5f82015f9055600182015f6116d39190612619565b505060035f8381526020019081526020015f205f6101000a81549073ffffffffffffffffffffffffffffffffffffffff0219169055508080600101915050611642565b505f5f90505b8181101561191e575f83828151811061173857611737613127565b5b602002602001015190505f5f90505b858110156118005760025f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206002015f8883815181106117a3576117a2613127565b5b602002602001015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81549060ff02191690558080600101915050611747565b5060025f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f5f82015f61184d9190612653565b600182015f905550507f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff8111156118905761188f612d88565b5b6040519080825280602002602001820160405280156118c357816020015b60608152602001906001900390816118ae5790505b5060025f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f01908161190f919061348d565b5050808060010191505061171c565b505f6005819055505f60068190555060045460075f82825461194091906130f4565b9250508190555050505050565b43600d819055505f600e5f6101000a81548160ff02191690836006811115611978576119776127f0565b5b02179055507f51fb20da0aafaceb18d92ff1a476059a0a8bbf16a0bf7c38b94a98b356ace45730600d546040516119b0929190612c1e565b60405180910390a1565b6119c26120d7565b73ffffffffffffffffffffffffffffffffffffffff166119e0610f3a565b73ffffffffffffffffffffffffffffffffffffffff1614611a3f57611a036120d7565b6040517f118cdaa7000000000000000000000000000000000000000000000000000000008152600401611a369190612a93565b60405180910390fd5b565b5f60095f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690508160095f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a35050565b5f611b11835f018361212f565b5f1c905092915050565b60605f611b29835f01612156565b905060608190508092505050919050565b5f611b46825f016121af565b9050919050565b5f611b777fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46961110d565b90507f0000000000000000000000000000000000000000000000000000000000000000811015817f00000000000000000000000000000000000000000000000000000000000000009091611c02576040517f3a236268000000000000000000000000000000000000000000000000000000008152600401611bf99291906134ef565b60405180910390fd5b505050565b5f611c1283836121be565b905092915050565b611c447f601a28e6abeea52b0ba15b54f6d15d789fd3e8c0080c042a009a9964f0e18f1c84610f8e565b8390611c86576040517f5c9f71ac000000000000000000000000000000000000000000000000000000008152600401611c7d9190612a93565b60405180910390fd5b505f60025f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f209050806002015f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff161584339091611d59576040517f08e55495000000000000000000000000000000000000000000000000000000008152600401611d50929190613516565b60405180910390fd5b50507f0000000000000000000000000000000000000000000000000000000000000000816001015410611dc1576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611db8906135bd565b60405180910390fd5b6001816002015f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055508282825f01836001015481548110611e3357611e32613127565b5b905f5260205f20019182611e489291906135e5565b506001816001015f828254611e5d91906130f4565b925050819055507f0000000000000000000000000000000000000000000000000000000000000000816001015410612015575f816001015467ffffffffffffffff811115611eae57611ead612d88565b5b604051908082528060200260200182016040528015611ee157816020015b6060815260200190600190039081611ecc5790505b5090505f5f90505b8260010154811015611fc457825f018181548110611f0a57611f09613127565b5b905f5260205f20018054611f1d90612d4e565b80601f0160208091040260200160405190810160405280929190818152602001828054611f4990612d4e565b8015611f945780601f10611f6b57610100808354040283529160200191611f94565b820191905f5260205f20905b815481529060010190602001808311611f7757829003601f168201915b5050505050828281518110611fac57611fab613127565b5b60200260200101819052508080600101915050611ee9565b508473ffffffffffffffffffffffffffffffffffffffff167fd2395e16bbe28e696809e1f9b1520776c9ec5998fc726c54e89d67dd041f9ff18260405161200b91906137aa565b60405180910390a2505b50505050565b5f7f7965db0b000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916148061208d575061208c82612201565b5b9050919050565b5f5f6120a0848461226a565b905080156120cd576120cb8360015f8781526020019081526020015f2061235390919063ffffffff16565b505b8091505092915050565b5f33905090565b6120e88282610f8e565b61212b5780826040517fe2517d3f0000000000000000000000000000000000000000000000000000000081526004016121229291906137ca565b60405180910390fd5b5050565b5f825f01828154811061214557612144613127565b5b905f5260205f200154905092915050565b6060815f018054806020026020016040519081016040528092919081815260200182805480156121a357602002820191905f5260205f20905b81548152602001906001019080831161218f575b50505050509050919050565b5f815f01805490509050919050565b5f5f6121ca8484612380565b905080156121f7576121f58360015f8781526020019081526020015f2061246990919063ffffffff16565b505b8091505092915050565b5f7f01ffc9a7000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916149050919050565b5f6122758383610f8e565b6123495760015f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055506122e66120d7565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a46001905061234d565b5f90505b92915050565b5f612378835f018373ffffffffffffffffffffffffffffffffffffffff165f1b612496565b905092915050565b5f61238b8383610f8e565b1561245f575f5f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055506123fc6120d7565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b60405160405180910390a460019050612463565b5f90505b92915050565b5f61248e835f018373ffffffffffffffffffffffffffffffffffffffff165f1b6124fd565b905092915050565b5f6124a183836125f9565b6124f357825f0182908060018154018082558091505060019003905f5260205f20015f9091909190915055825f0180549050836001015f8481526020019081526020015f2081905550600190506124f7565b5f90505b92915050565b5f5f836001015f8481526020019081526020015f205490505f81146125ee575f60018261252a9190612cee565b90505f6001865f01805490506125409190612cee565b90508082146125a6575f865f01828154811061255f5761255e613127565b5b905f5260205f200154905080875f0184815481106125805761257f613127565b5b905f5260205f20018190555083876001015f8381526020019081526020015f2081905550505b855f018054806125b9576125b86137f1565b5b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f9055600193505050506125f3565b5f9150505b92915050565b5f5f836001015f8481526020019081526020015f20541415905092915050565b50805461262590612d4e565b5f825580601f106126365750612650565b601f0160209004905f5260205f209061264f919061266e565b5b50565b5080545f8255905f5260205f209061266b919061268b565b50565b5f5b80821115612686578281015f9055600101612670565b505090565b5f5b808211156126ab578281015f6126a39190612619565b60010161268d565b505090565b5f5ffd5b5f5ffd5b5f7fffffffff0000000000000000000000000000000000000000000000000000000082169050919050565b6126ec816126b8565b81146126f6575f5ffd5b50565b5f81359050612707816126e3565b92915050565b5f60208284031215612722576127216126b0565b5b5f61272f848285016126f9565b91505092915050565b5f8115159050919050565b61274c81612738565b82525050565b5f6020820190506127655f830184612743565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6127948261276b565b9050919050565b6127a48161278a565b81146127ae575f5ffd5b50565b5f813590506127bf8161279b565b92915050565b5f602082840312156127da576127d96126b0565b5b5f6127e7848285016127b1565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6007811061282e5761282d6127f0565b5b50565b5f81905061283e8261281d565b919050565b5f61284d82612831565b9050919050565b61285d81612843565b82525050565b5f6020820190506128765f830184612854565b92915050565b5f819050919050565b61288e8161287c565b82525050565b5f6020820190506128a75f830184612885565b92915050565b6128b68161287c565b81146128c0575f5ffd5b50565b5f813590506128d1816128ad565b92915050565b5f602082840312156128ec576128eb6126b0565b5b5f6128f9848285016128c3565b91505092915050565b5f819050919050565b61291481612902565b811461291e575f5ffd5b50565b5f8135905061292f8161290b565b92915050565b5f6020828403121561294a576129496126b0565b5b5f61295784828501612921565b91505092915050565b61296981612902565b82525050565b5f6020820190506129825f830184612960565b92915050565b5f5f6040838503121561299e5761299d6126b0565b5b5f6129ab85828601612921565b92505060206129bc858286016127b1565b9150509250929050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f8401126129e7576129e66129c6565b5b8235905067ffffffffffffffff811115612a0457612a036129ca565b5b602083019150836001820283011115612a2057612a1f6129ce565b5b9250929050565b5f5f5f60408486031215612a3e57612a3d6126b0565b5b5f84013567ffffffffffffffff811115612a5b57612a5a6126b4565b5b612a67868287016129d2565b93509350506020612a7a868287016128c3565b9150509250925092565b612a8d8161278a565b82525050565b5f602082019050612aa65f830184612a84565b92915050565b5f5f60408385031215612ac257612ac16126b0565b5b5f612acf85828601612921565b9250506020612ae0858286016128c3565b9150509250929050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b612b1c8161278a565b82525050565b5f612b2d8383612b13565b60208301905092915050565b5f602082019050919050565b5f612b4f82612aea565b612b598185612af4565b9350612b6483612b04565b805f5b83811015612b94578151612b7b8882612b22565b9750612b8683612b39565b925050600181019050612b67565b5085935050505092915050565b5f6020820190508181035f830152612bb98184612b45565b905092915050565b5f5f5f60408486031215612bd857612bd76126b0565b5b5f612be5868287016127b1565b935050602084013567ffffffffffffffff811115612c0657612c056126b4565b5b612c12868287016129d2565b92509250509250925092565b5f604082019050612c315f830185612a84565b612c3e6020830184612885565b9392505050565b5f606082019050612c585f830186612885565b612c656020830185612a84565b612c726040830184612a84565b949350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f612cb18261287c565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203612ce357612ce2612c7a565b5b600182019050919050565b5f612cf88261287c565b9150612d038361287c565b9250828203905081811115612d1b57612d1a612c7a565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f6002820490506001821680612d6557607f821691505b602082108103612d7857612d77612d21565b5b50919050565b5f81519050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f60088302612e117fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82612dd6565b612e1b8683612dd6565b95508019841693508086168417925050509392505050565b5f819050919050565b5f612e56612e51612e4c8461287c565b612e33565b61287c565b9050919050565b5f819050919050565b612e6f83612e3c565b612e83612e7b82612e5d565b848454612de2565b825550505050565b5f5f905090565b612e9a612e8b565b612ea5818484612e66565b505050565b5f5b82811015612ecb57612ec05f828401612e92565b600181019050612eac565b505050565b601f821115612f1e5782821115612f1d57612eea81612db5565b612ef383612dc7565b612efc85612dc7565b6020861015612f09575f90505b808301612f1882840382612eaa565b505050505b5b505050565b5f82821c905092915050565b5f612f3e5f1984600802612f23565b1980831691505092915050565b5f612f568383612f2f565b9150826002028217905092915050565b612f6f82612d7e565b67ffffffffffffffff811115612f8857612f87612d88565b5b612f928254612d4e565b612f9d828285612ed0565b5f60209050601f831160018114612fce575f8415612fbc578287015190505b612fc68582612f4b565b86555061302d565b601f198416612fdc86612db5565b5f5b8281101561300357848901518255600182019150602085019450602081019050612fde565b86831015613020578489015161301c601f891682612f2f565b8355505b6001600288020188555050505b505050505050565b5f82825260208201905092915050565b828183375f83830152505050565b5f601f19601f8301169050919050565b5f61306e8385613035565b935061307b838584613045565b61308483613053565b840190509392505050565b5f6060820190506130a25f830187612a84565b81810360208301526130b5818587613063565b90506130c46040830184612885565b95945050505050565b5f6040820190506130e05f830185612854565b6130ed6020830184612854565b9392505050565b5f6130fe8261287c565b91506131098361287c565b925082820190508082111561312157613120612c7a565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f819050602082019050919050565b5f81549050919050565b5f8190506001806001038301049050919050565b5f819050815f5260205f209050919050565b6131c37fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff83602003600802612f23565b815481168255505050565b6131d781612db5565b6131e2838254612f4b565b8083555f825550505050565b602084105f811461324957601f8411600181146132165761320f8685612f4b565b8355613243565b61321f83612db5565b613237600161322d88612dc7565b0360018301612eaa565b61324187856131ce565b505b506132a3565b61325285612dc7565b61325b85612dc7565b61326484612db5565b828101601f8916801561327f5761327e8160018403613193565b5b848411156132945761329385850383612eaa565b5b60018a60020217875550505050505b5050505050565b680100000000000000008411156132c4576132c3612d88565b5b602083105f811461330d57602085105f81146132eb576132e48685612f4b565b8355613307565b8360ff19169350836132fc84612db5565b556001866002020183555b50613317565b6001856002020182555b5050505050565b805461332981612d4e565b8084111561333e5761333d848284866132aa565b5b8084101561335357613352848284866131ee565b5b50505050565b828110156133785761336d5f828401612e92565b600181019050613359565b505050565b6133875f8261331e565b50565b7f4e487b71000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b5f82146133c6576133c561338a565b5b6133cf8161337d565b5050565b5f5b828110156133f4576133e95f8284016133b6565b6001810190506133d5565b505050565b818310156134305761340a8261316d565b6134138461316d565b61341c83613181565b81810161342b838503826133d3565b505050505b505050565b6801000000000000000082111561344f5761344e612d88565b5b61345881613163565b8282556134668382846133f9565b505050565b5f81519050919050565b5f81519050919050565b6134898282612f66565b5050565b6134968261346b565b6134a08183613435565b6134a983613154565b6134b283613181565b5f5b838110156134e7576134c583613475565b6134cf818461347f565b602084019350600183019250506001810190506134b4565b505050505050565b5f6040820190506135025f830185612885565b61350f6020830184612885565b9392505050565b5f6040820190506135295f830185612a84565b6135366020830184612a84565b9392505050565b5f82825260208201905092915050565b7f4255473a20414c5245414459205245434549564544205348415245532046524f5f8201527f4d204e20504152544945532c20544f4f204d414e5920434c49454e5453000000602082015250565b5f6135a7603d8361353d565b91506135b28261354d565b604082019050919050565b5f6020820190508181035f8301526135d48161359b565b9050919050565b5f82905092915050565b6135ef83836135db565b67ffffffffffffffff81111561360857613607612d88565b5b6136128254612d4e565b61361d828285612ed0565b5f601f83116001811461364a575f8415613638578287013590505b6136428582612f4b565b8655506136a9565b601f19841661365886612db5565b5f5b8281101561367f5784890135825560018201915060208501945060208101905061365a565b8683101561369c5784890135613698601f891682612f2f565b8355505b6001600288020188555050505b50505050505050565b5f82825260208201905092915050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f6136ea82612d7e565b6136f481856136c2565b93506137048185602086016136d2565b61370d81613053565b840191505092915050565b5f61372383836136e0565b905092915050565b5f602082019050919050565b5f6137418261346b565b61374b81856136b2565b93508360208202850161375d85613154565b805f5b8581101561379857848403895281516137798582613718565b94506137848361372b565b925060208a01995050600181019050613760565b50829750879550505050505092915050565b5f6020820190508181035f8301526137c28184613737565b905092915050565b5f6040820190506137dd5f830185612a84565b6137ea6020830184612960565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffdfea2646970667358221220fda3a7d1bf92f7c8aba897cbea15cbc5eb28b5817690c6bc423f33af2b9e209064736f6c63430008210033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xE0`@R4\x80\x15a\0\x0FW__\xFD[P`@QaK_8\x03\x80aK_\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\t\xB0V[\x85\x85\x85\x85\x85\x853\x83\x83\x83\x88\x88\x81`\x80\x81\x81RPP`\x01`\x80Q`\x03a\0V\x91\x90a\n\x9EV[a\0`\x91\x90a\n\xDFV[`\xA0\x81\x81RPP`\xA0Q\x81Q\x10\x15\x81Q`\xA0Q\x90\x91a\0\xB6W`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\0\xAD\x92\x91\x90a\x0B!V[`@Q\x80\x91\x03\x90\xFD[PP__\x90P[\x81Q\x81\x10\x15a\x01\x1EWa\x01\x10\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x83\x83\x81Q\x81\x10a\0\xFDWa\0\xFCa\x0BHV[[` \x02` \x01\x01Qa\x03\xDE` \x1B` \x1CV[P\x80\x80`\x01\x01\x91PPa\0\xBDV[Pa\x01i\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x82_\x81Q\x81\x10a\x01VWa\x01Ua\x0BHV[[` \x02` \x01\x01Qa\x03\xDE` \x1B` \x1CV[P\x7F\xA2\xDFx0\xE0\xBE\xDE\xF7\xB1\x11k\xF5G\xB4g\xB1kP\xB3\xBD#\x14l\x9E\t\x98x\xD1N\x890\x1A`\xA0Q`\x80Q3`@Qa\x01\xA1\x93\x92\x91\x90a\x0B\x84V[`@Q\x80\x91\x03\x90\xA1PP\x80`\xC0\x81\x81RPP_`\x07\x81\x90UP\x82`\x04\x81\x90UP_`\x05\x81\x90UP_`\x06\x81\x90UP__\x90P[\x82Q\x81\x10\x15a\x02\xEBWa\x02'\x7F`\x1A(\xE6\xAB\xEE\xA5+\x0B\xA1[T\xF6\xD1]x\x9F\xD3\xE8\xC0\x08\x0C\x04*\0\x9A\x99d\xF0\xE1\x8F\x1C\x84\x83\x81Q\x81\x10a\x02\x14Wa\x02\x13a\x0BHV[[` \x02` \x01\x01Qa\x03\xDE` \x1B` \x1CV[P`\xA0Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02DWa\x02Ca\x08\x1AV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02wW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x02bW\x90P[P`\x02_\x85\x84\x81Q\x81\x10a\x02\x8EWa\x02\x8Da\x0BHV[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x01\x90\x81a\x02\xDD\x91\x90a\x11\xD9V[P\x80\x80`\x01\x01\x91PPa\x01\xD4V[P\x7F\xF7\xF0\x87#\x82\xDF\xF5\xE6\x98\xB2\x84\xE1 \x84\xE4\xE7\x89O\x83\x02\x16\xDD\x80\xCBN\x90\x9BY:X\xF95`\x04T3`@Qa\x03\x1F\x92\x91\x90a\x12;V[`@Q\x80\x91\x03\x90\xA1PPP_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x03\x9AW_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03\x91\x91\x90a\x12bV[`@Q\x80\x91\x03\x90\xFD[a\x03\xA9\x81a\x03\xF7` \x1B` \x1CV[P\x85`\n\x81\x90UPB`\x0B\x81\x90UPC`\x0C\x81\x90UPa\x03\xCDa\x04\xBA` \x1B` \x1CV[PPPPPPPPPPPPa\x12\xCFV[_a\x03\xEF\x83\x83a\x05'` \x1B` \x1CV[\x90P\x92\x91PPV[_`\t_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\t_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[C`\r\x81\x90UP_`\x0E_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x06\x81\x11\x15a\x04\xE5Wa\x04\xE4a\x12{V[[\x02\x17\x90UP\x7FQ\xFB \xDA\n\xAF\xAC\xEB\x18\xD9/\xF1\xA4v\x05\x9A\n\x8B\xBF\x16\xA0\xBF|8\xB9J\x98\xB3V\xAC\xE4W0`\rT`@Qa\x05\x1D\x92\x91\x90a\x12\xA8V[`@Q\x80\x91\x03\x90\xA1V[__a\x059\x84\x84a\x05p` \x1B` \x1CV[\x90P\x80\x15a\x05fWa\x05d\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a\x06e` \x1B\x90\x91\x90` \x1CV[P[\x80\x91PP\x92\x91PPV[_a\x05\x81\x83\x83a\x06\x98` \x1B` \x1CV[a\x06[W`\x01__\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x05\xF8a\x06\xFB` \x1B` \x1CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\x06_V[_\x90P[\x92\x91PPV[_a\x06\x90\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba\x07\x02` \x1B` \x1CV[\x90P\x92\x91PPV[___\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[_3\x90P\x90V[_a\x07\x13\x83\x83a\x07o` \x1B` \x1CV[a\x07eW\x82_\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x82_\x01\x80T\x90P\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP`\x01\x90Pa\x07iV[_\x90P[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x14\x15\x90P\x92\x91PPV[_`@Q\x90P\x90V[__\xFD[__\xFD[_\x81\x90P\x91\x90PV[a\x07\xB2\x81a\x07\xA0V[\x81\x14a\x07\xBCW__\xFD[PV[_\x81Q\x90Pa\x07\xCD\x81a\x07\xA9V[\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x07\xE5\x81a\x07\xD3V[\x81\x14a\x07\xEFW__\xFD[PV[_\x81Q\x90Pa\x08\0\x81a\x07\xDCV[\x92\x91PPV[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x08P\x82a\x08\nV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x08oWa\x08na\x08\x1AV[[\x80`@RPPPV[_a\x08\x81a\x07\x8FV[\x90Pa\x08\x8D\x82\x82a\x08GV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x08\xACWa\x08\xABa\x08\x1AV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x08\xEA\x82a\x08\xC1V[\x90P\x91\x90PV[a\x08\xFA\x81a\x08\xE0V[\x81\x14a\t\x04W__\xFD[PV[_\x81Q\x90Pa\t\x15\x81a\x08\xF1V[\x92\x91PPV[_a\t-a\t(\x84a\x08\x92V[a\x08xV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\tPWa\tOa\x08\xBDV[[\x83[\x81\x81\x10\x15a\tyW\x80a\te\x88\x82a\t\x07V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\tRV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\t\x97Wa\t\x96a\x08\x06V[[\x81Qa\t\xA7\x84\x82` \x86\x01a\t\x1BV[\x91PP\x92\x91PPV[______`\xC0\x87\x89\x03\x12\x15a\t\xCAWa\t\xC9a\x07\x98V[[_a\t\xD7\x89\x82\x8A\x01a\x07\xBFV[\x96PP` a\t\xE8\x89\x82\x8A\x01a\x07\xF2V[\x95PP`@\x87\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\tWa\n\x08a\x07\x9CV[[a\n\x15\x89\x82\x8A\x01a\t\x83V[\x94PP``a\n&\x89\x82\x8A\x01a\x07\xF2V[\x93PP`\x80\x87\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\nGWa\nFa\x07\x9CV[[a\nS\x89\x82\x8A\x01a\t\x83V[\x92PP`\xA0a\nd\x89\x82\x8A\x01a\x07\xF2V[\x91PP\x92\x95P\x92\x95P\x92\x95V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\n\xA8\x82a\x07\xD3V[\x91Pa\n\xB3\x83a\x07\xD3V[\x92P\x82\x82\x02a\n\xC1\x81a\x07\xD3V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\n\xD8Wa\n\xD7a\nqV[[P\x92\x91PPV[_a\n\xE9\x82a\x07\xD3V[\x91Pa\n\xF4\x83a\x07\xD3V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x0B\x0CWa\x0B\x0Ba\nqV[[\x92\x91PPV[a\x0B\x1B\x81a\x07\xD3V[\x82RPPV[_`@\x82\x01\x90Pa\x0B4_\x83\x01\x85a\x0B\x12V[a\x0BA` \x83\x01\x84a\x0B\x12V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[a\x0B~\x81a\x08\xE0V[\x82RPPV[_``\x82\x01\x90Pa\x0B\x97_\x83\x01\x86a\x0B\x12V[a\x0B\xA4` \x83\x01\x85a\x0B\x12V[a\x0B\xB1`@\x83\x01\x84a\x0BuV[\x94\x93PPPPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x81T\x90P\x91\x90PV[_\x81\x90P`\x01\x80`\x01\x03\x83\x01\x04\x90P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a\x0C<W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0COWa\x0CNa\x0B\xF8V[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_\x82\x82\x1C\x90P\x92\x91PPV[a\x0C\xA3\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83` \x03`\x08\x02a\x0CgV[\x81T\x81\x16\x82UPPPV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a\x0C\xE9\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a\x0C\xAEV[a\x0C\xF3\x86\x83a\x0C\xAEV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a\r.a\r)a\r$\x84a\x07\xD3V[a\r\x0BV[a\x07\xD3V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\rG\x83a\r\x14V[a\r[a\rS\x82a\r5V[\x84\x84Ta\x0C\xBAV[\x82UPPPPV[__\x90P\x90V[a\rra\rcV[a\r}\x81\x84\x84a\r>V[PPPV[_[\x82\x81\x10\x15a\r\xA3Wa\r\x98_\x82\x84\x01a\rjV[`\x01\x81\x01\x90Pa\r\x84V[PPPV[_a\r\xB7_\x19\x84`\x08\x02a\x0CgV[\x19\x80\x83\x16\x91PP\x92\x91PPV[_a\r\xCF\x83\x83a\r\xA8V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a\r\xE8\x81a\x0CUV[a\r\xF3\x83\x82Ta\r\xC4V[\x80\x83U_\x82UPPPPV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[` \x84\x10_\x81\x14a\x0EiW`\x1F\x84\x11`\x01\x81\x14a\x0E6Wa\x0E/\x86\x85a\r\xC4V[\x83Ua\x0EcV[a\x0E?\x83a\x0CUV[a\x0EW`\x01a\x0EM\x88a\r\xFFV[\x03`\x01\x83\x01a\r\x82V[a\x0Ea\x87\x85a\r\xDFV[P[Pa\x0E\xC3V[a\x0Er\x85a\r\xFFV[a\x0E{\x85a\r\xFFV[a\x0E\x84\x84a\x0CUV[\x82\x81\x01`\x1F\x89\x16\x80\x15a\x0E\x9FWa\x0E\x9E\x81`\x01\x84\x03a\x0CsV[[\x84\x84\x11\x15a\x0E\xB4Wa\x0E\xB3\x85\x85\x03\x83a\r\x82V[[`\x01\x8A`\x02\x02\x17\x87UPPPPP[PPPPPV[h\x01\0\0\0\0\0\0\0\0\x84\x11\x15a\x0E\xE4Wa\x0E\xE3a\x08\x1AV[[` \x83\x10_\x81\x14a\x0F-W` \x85\x10_\x81\x14a\x0F\x0BWa\x0F\x04\x86\x85a\r\xC4V[\x83Ua\x0F'V[\x83`\xFF\x19\x16\x93P\x83a\x0F\x1C\x84a\x0CUV[U`\x01\x86`\x02\x02\x01\x83U[Pa\x0F7V[`\x01\x85`\x02\x02\x01\x82U[PPPPPV[\x80Ta\x0FI\x81a\x0C%V[\x80\x84\x11\x15a\x0F^Wa\x0F]\x84\x82\x84\x86a\x0E\xCAV[[\x80\x84\x10\x15a\x0FsWa\x0Fr\x84\x82\x84\x86a\x0E\x0EV[[PPPPV[\x82\x81\x10\x15a\x0F\x98Wa\x0F\x8D_\x82\x84\x01a\rjV[`\x01\x81\x01\x90Pa\x0FyV[PPPV[a\x0F\xA7_\x82a\x0F>V[PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[_\x82\x14a\x0F\xE6Wa\x0F\xE5a\x0F\xAAV[[a\x0F\xEF\x81a\x0F\x9DV[PPV[_[\x82\x81\x10\x15a\x10\x14Wa\x10\t_\x82\x84\x01a\x0F\xD6V[`\x01\x81\x01\x90Pa\x0F\xF5V[PPPV[\x81\x83\x10\x15a\x10PWa\x10*\x82a\x0B\xD2V[a\x103\x84a\x0B\xD2V[a\x10<\x83a\x0B\xE6V[\x81\x81\x01a\x10K\x83\x85\x03\x82a\x0F\xF3V[PPPP[PPPV[h\x01\0\0\0\0\0\0\0\0\x82\x11\x15a\x10oWa\x10na\x08\x1AV[[a\x10x\x81a\x0B\xC8V[\x82\x82Ua\x10\x86\x83\x82\x84a\x10\x19V[PPPV[_\x81Q\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[`\x1F\x82\x11\x15a\x10\xF7W\x82\x82\x11\x15a\x10\xF6Wa\x10\xC3\x81a\x0CUV[a\x10\xCC\x83a\r\xFFV[a\x10\xD5\x85a\r\xFFV[` \x86\x10\x15a\x10\xE2W_\x90P[\x80\x83\x01a\x10\xF1\x82\x84\x03\x82a\r\x82V[PPPP[[PPPV[a\x11\x05\x82a\x10\x9FV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\x1EWa\x11\x1Da\x08\x1AV[[a\x11(\x82Ta\x0C%V[a\x113\x82\x82\x85a\x10\xA9V[_` \x90P`\x1F\x83\x11`\x01\x81\x14a\x11dW_\x84\x15a\x11RW\x82\x87\x01Q\x90P[a\x11\\\x85\x82a\r\xC4V[\x86UPa\x11\xC3V[`\x1F\x19\x84\x16a\x11r\x86a\x0CUV[_[\x82\x81\x10\x15a\x11\x99W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa\x11tV[\x86\x83\x10\x15a\x11\xB6W\x84\x89\x01Qa\x11\xB2`\x1F\x89\x16\x82a\r\xA8V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[a\x11\xD5\x82\x82a\x10\xFCV[PPV[a\x11\xE2\x82a\x10\x8BV[a\x11\xEC\x81\x83a\x10UV[a\x11\xF5\x83a\x0B\xB9V[a\x11\xFE\x83a\x0B\xE6V[_[\x83\x81\x10\x15a\x123Wa\x12\x11\x83a\x10\x95V[a\x12\x1B\x81\x84a\x11\xCBV[` \x84\x01\x93P`\x01\x83\x01\x92PP`\x01\x81\x01\x90Pa\x12\0V[PPPPPPV[_`@\x82\x01\x90Pa\x12N_\x83\x01\x85a\x0B\x12V[a\x12[` \x83\x01\x84a\x0BuV[\x93\x92PPPV[_` \x82\x01\x90Pa\x12u_\x83\x01\x84a\x0BuV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_`@\x82\x01\x90Pa\x12\xBB_\x83\x01\x85a\x0BuV[a\x12\xC8` \x83\x01\x84a\x0B\x12V[\x93\x92PPPV[`\x80Q`\xA0Q`\xC0Qa8Ta\x13\x0B_9_a\x1Ef\x01R_\x81\x81a\x18X\x01R\x81\x81a\x1B{\x01R\x81\x81a\x1B\xA0\x01Ra\x1D]\x01R_PPa8T_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02\x0FW_5`\xE0\x1C\x80c\\\xB8kt\x11a\x01#W\x80c\xBBQ\xFE\xF0\x11a\0\xABW\x80c\xD5Gt\x1F\x11a\0zW\x80c\xD5Gt\x1F\x14a\x05mW\x80c\xD8'\r\xCE\x14a\x05\x89W\x80c\xED\xE6\x92\x16\x14a\x05\xA7W\x80c\xF2\xFD\xE3\x8B\x14a\x05\xC3W\x80c\xFCx\xB2\xE8\x14a\x05\xDFWa\x02\x0FV[\x80c\xBBQ\xFE\xF0\x14a\x05\x1FW\x80c\xC0y\xF4\x95\x14a\x05)W\x80c\xCA\x15\xC8s\x14a\x053W\x80c\xCB\x9CL\xC4\x14a\x05cWa\x02\x0FV[\x80c\x8D\xA5\xCB[\x11a\0\xF2W\x80c\x8D\xA5\xCB[\x14a\x04SW\x80c\x90\x10\xD0|\x14a\x04qW\x80c\x91\xD1HT\x14a\x04\xA1W\x80c\xA2\x17\xFD\xDF\x14a\x04\xD1W\x80c\xA3$j\xD3\x14a\x04\xEFWa\x02\x0FV[\x80c\\\xB8kt\x14a\x04\x03W\x80ck^\x12\xCA\x14a\x04\rW\x80cqP\x18\xA6\x14a\x04+W\x80c\x7F5\xB5`\x14a\x045Wa\x02\x0FV[\x80c//\xF1]\x11a\x01\xA6W\x80cI\xF2\xAD\xA0\x11a\x01uW\x80cI\xF2\xAD\xA0\x14a\x03\x97W\x80cK\x8Ed\x88\x14a\x03\xB5W\x80cK\xB2x\xF3\x14a\x03\xBFW\x80cVHRl\x14a\x03\xC9W\x80cX\xDF\r\x01\x14a\x03\xE5Wa\x02\x0FV[\x80c//\xF1]\x14a\x037W\x80c0\x10L>\x14a\x03SW\x80c3\xCC\x9A\t\x14a\x03qW\x80c6V\x8A\xBE\x14a\x03{Wa\x02\x0FV[\x80c\x1CtS\xDB\x11a\x01\xE2W\x80c\x1CtS\xDB\x14a\x02\xAFW\x80c!\xDC{\x9B\x14a\x02\xCDW\x80c#(\xBD\x12\x14a\x02\xE9W\x80c$\x8A\x9C\xA3\x14a\x03\x07Wa\x02\x0FV[\x80c\x01\xFF\xC9\xA7\x14a\x02\x13W\x80c\x13\xFFm\xD5\x14a\x02CW\x80c\x14l\xA51\x14a\x02sW\x80c\x17cE\x14\x14a\x02\x91W[__\xFD[a\x02-`\x04\x806\x03\x81\x01\x90a\x02(\x91\x90a'\rV[a\x06\x0FV[`@Qa\x02:\x91\x90a'RV[`@Q\x80\x91\x03\x90\xF3[a\x02]`\x04\x806\x03\x81\x01\x90a\x02X\x91\x90a'\xC5V[a\x06\x88V[`@Qa\x02j\x91\x90a'RV[`@Q\x80\x91\x03\x90\xF3[a\x02{a\x06\xCBV[`@Qa\x02\x88\x91\x90a(cV[`@Q\x80\x91\x03\x90\xF3[a\x02\x99a\x06\xDDV[`@Qa\x02\xA6\x91\x90a(\x94V[`@Q\x80\x91\x03\x90\xF3[a\x02\xB7a\x06\xE3V[`@Qa\x02\xC4\x91\x90a(\x94V[`@Q\x80\x91\x03\x90\xF3[a\x02\xE7`\x04\x806\x03\x81\x01\x90a\x02\xE2\x91\x90a(\xD7V[a\x06\xE9V[\0[a\x02\xF1a\t\xA0V[`@Qa\x02\xFE\x91\x90a(\x94V[`@Q\x80\x91\x03\x90\xF3[a\x03!`\x04\x806\x03\x81\x01\x90a\x03\x1C\x91\x90a)5V[a\t\xB6V[`@Qa\x03.\x91\x90a)oV[`@Q\x80\x91\x03\x90\xF3[a\x03Q`\x04\x806\x03\x81\x01\x90a\x03L\x91\x90a)\x88V[a\t\xD2V[\0[a\x03[a\n\x14V[`@Qa\x03h\x91\x90a)oV[`@Q\x80\x91\x03\x90\xF3[a\x03ya\n8V[\0[a\x03\x95`\x04\x806\x03\x81\x01\x90a\x03\x90\x91\x90a)\x88V[a\n\xB2V[\0[a\x03\x9Fa\n\xC8V[`@Qa\x03\xAC\x91\x90a)oV[`@Q\x80\x91\x03\x90\xF3[a\x03\xBDa\n\xECV[\0[a\x03\xC7a\x0BfV[\0[a\x03\xE3`\x04\x806\x03\x81\x01\x90a\x03\xDE\x91\x90a*'V[a\x0B\xE0V[\0[a\x03\xEDa\x0E\x9CV[`@Qa\x03\xFA\x91\x90a)oV[`@Q\x80\x91\x03\x90\xF3[a\x04\x0Ba\x0E\xC0V[\0[a\x04\x15a\x0E\xFDV[`@Qa\x04\"\x91\x90a(\x94V[`@Q\x80\x91\x03\x90\xF3[a\x043a\x0F\x03V[\0[a\x04=a\x0F\x16V[`@Qa\x04J\x91\x90a)oV[`@Q\x80\x91\x03\x90\xF3[a\x04[a\x0F:V[`@Qa\x04h\x91\x90a*\x93V[`@Q\x80\x91\x03\x90\xF3[a\x04\x8B`\x04\x806\x03\x81\x01\x90a\x04\x86\x91\x90a*\xACV[a\x0FbV[`@Qa\x04\x98\x91\x90a*\x93V[`@Q\x80\x91\x03\x90\xF3[a\x04\xBB`\x04\x806\x03\x81\x01\x90a\x04\xB6\x91\x90a)\x88V[a\x0F\x8EV[`@Qa\x04\xC8\x91\x90a'RV[`@Q\x80\x91\x03\x90\xF3[a\x04\xD9a\x0F\xF1V[`@Qa\x04\xE6\x91\x90a)oV[`@Q\x80\x91\x03\x90\xF3[a\x05\t`\x04\x806\x03\x81\x01\x90a\x05\x04\x91\x90a)5V[a\x0F\xF7V[`@Qa\x05\x16\x91\x90a+\xA1V[`@Q\x80\x91\x03\x90\xF3[a\x05'a\x10\x19V[\0[a\x051a\x10\x93V[\0[a\x05M`\x04\x806\x03\x81\x01\x90a\x05H\x91\x90a)5V[a\x11\rV[`@Qa\x05Z\x91\x90a(\x94V[`@Q\x80\x91\x03\x90\xF3[a\x05ka\x11.V[\0[a\x05\x87`\x04\x806\x03\x81\x01\x90a\x05\x82\x91\x90a)\x88V[a\x11\xAFV[\0[a\x05\x91a\x11\xF1V[`@Qa\x05\x9E\x91\x90a(\x94V[`@Q\x80\x91\x03\x90\xF3[a\x05\xC1`\x04\x806\x03\x81\x01\x90a\x05\xBC\x91\x90a+\xC1V[a\x11\xF7V[\0[a\x05\xDD`\x04\x806\x03\x81\x01\x90a\x05\xD8\x91\x90a'\xC5V[a\x12>V[\0[a\x05\xF9`\x04\x806\x03\x81\x01\x90a\x05\xF4\x91\x90a'\xC5V[a\x12\xC2V[`@Qa\x06\x06\x91\x90a'RV[`@Q\x80\x91\x03\x90\xF3[_\x7F\x07\xEF\xFE\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x06\x81WPa\x06\x80\x82a\x12\xF4V[[\x90P\x91\x90PV[_a\x06\x92\x82a\x12\xC2V[\x80\x15a\x06\xC4WPa\x06\xC3\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x83a\x0F\x8EV[[\x90P\x91\x90PV[`\x0E_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\x0CT\x81V[`\x07T\x81V[`\x04T\x81\x103\x82\x90\x91a\x073W`@Q\x7Fhg\xA1p\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07*\x92\x91\x90a,\x1EV[`@Q\x80\x91\x03\x90\xFD[PP__\x90P[`\x04T\x81\x10\x15a\x07\xF8W3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x153\x82\x90\x91a\x07\xE9W`@Q\x7F\xC3\x15\xA0\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\xE0\x92\x91\x90a,\x1EV[`@Q\x80\x91\x03\x90\xFD[PP\x80\x80`\x01\x01\x91PPa\x07:V[P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x813`\x03_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x91\x92a\x08\xD0W`@Q\x7F\xA0\xB8\xC7\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xC7\x93\x92\x91\x90a,EV[`@Q\x80\x91\x03\x90\xFD[PPP3`\x03_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPa\tL\x7F\xD6\x8E>^6\x7F\xEEG\xCE\x11\xA5\xDA\xB4\x04Yi\x80\xE1X\xEB\x90i3\n\x8Fw]\xE7\xDCk\xB6M3a\x13mV[P`\x05_\x81T\x80\x92\x91\x90a\t_\x90a,\xA7V[\x91\x90PUP\x7F\xAB\xDE\x16\xB7\xA9\x19,1\xC6#\x1B\x159\xBA\xD6\xFE\xD7v5\xDEL\0\x87\x18\xDB\xDC\xAF\xB7\xB86:\xFE3\x82`@Qa\t\x95\x92\x91\x90a,\x1EV[`@Q\x80\x91\x03\x90\xA1PV[_`\x05T`\x04Ta\t\xB1\x91\x90a,\xEEV[\x90P\x90V[___\x83\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\t\xFC\x81a\x13\x80V[a\n\x04a\x13\x94V[a\n\x0E\x83\x83a\x13mV[PPPPV[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x81V[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\nb\x81a\x13\x80V[`\x03a\nm\x81a\x14\x1CV[\x7F \xF5^\xD0\xC9/+\xB1\xC8\x82T\x88\xE1\xE3\xC9\x84c\xD0$\xB2\xA4-\xBD$\x83\x8C?u&\x0FC\xE93B`@Qa\n\x9E\x92\x91\x90a,\x1EV[`@Q\x80\x91\x03\x90\xA1a\n\xAEa\x14\xA9V[PPV[a\n\xBAa\x13\x94V[a\n\xC4\x82\x82a\x15\x12V[PPV[\x7F\xD6\x8E>^6\x7F\xEEG\xCE\x11\xA5\xDA\xB4\x04Yi\x80\xE1X\xEB\x90i3\n\x8Fw]\xE7\xDCk\xB6M\x81V[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0B\x16\x81a\x13\x80V[`\x04a\x0B!\x81a\x14\x1CV[\x7F0\x1F\x8A7\x01\xF5\xB2`\x19s\x82\xDDs\x01\x07\x85B\x14O\xE8\xFD\xDD\x18\x08=on\t\xE4\x95\x8AY3B`@Qa\x0BR\x92\x91\x90a,\x1EV[`@Q\x80\x91\x03\x90\xA1a\x0Bba\x14\xA9V[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0B\x90\x81a\x13\x80V[`\x05a\x0B\x9B\x81a\x14\x1CV[\x7F$\xA8se\x1D&\xFBZF,\xB3z\x91\x07\x1C\xDDM\t\xAB\xEE\xBF\xE0\xED\x14C)\xBE\xD1\xCC5\x9D\x033B`@Qa\x0B\xCC\x92\x91\x90a,\x1EV[`@Q\x80\x91\x03\x90\xA1a\x0B\xDCa\x14\xA9V[PPV[\x7F\xD6\x8E>^6\x7F\xEEG\xCE\x11\xA5\xDA\xB4\x04Yi\x80\xE1X\xEB\x90i3\n\x8Fw]\xE7\xDCk\xB6Ma\x0C\n\x81a\x13\x80V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x143\x83\x90\x91a\x0C\xAEW`@Q\x7F\xFF\xAB\xBA\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\xA5\x92\x91\x90a,\x1EV[`@Q\x80\x91\x03\x90\xFD[PP_\x84\x84\x90P\x14\x153\x90a\x0C\xF9W`@Q\x7F\x16\x92<\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\xF0\x91\x90a*\x93V[`@Q\x80\x91\x03\x90\xFD[P_`\x08_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x01\x80Ta\rF\x90a-NV[\x90P\x143\x90a\r\x8BW`@Q\x7FO_\xBF\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\x82\x91\x90a*\x93V[`@Q\x80\x91\x03\x90\xFD[P`@Q\x80`@\x01`@R\x80\x83\x81R` \x01\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81RP`\x08_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01U` \x82\x01Q\x81`\x01\x01\x90\x81a\x0E>\x91\x90a/fV[P\x90PP\x7FV\xD0>_\x1E\xBE\xC3\xD4\xB4\xF9\xDE\xD0~\x82\xC6\xBBh\x97\xC1B\xCF\xBA\xF8\xDF\xF8\xF9\xEF\x89|\xE4\xF7_3\x85\x85\x85`@Qa\x0Ew\x94\x93\x92\x91\x90a0\x8FV[`@Q\x80\x91\x03\x90\xA1`\x06_\x81T\x80\x92\x91\x90a\x0E\x91\x90a,\xA7V[\x91\x90PUPPPPPV[\x7F`\x1A(\xE6\xAB\xEE\xA5+\x0B\xA1[T\xF6\xD1]x\x9F\xD3\xE8\xC0\x08\x0C\x04*\0\x9A\x99d\xF0\xE1\x8F\x1C\x81V[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0E\xEA\x81a\x13\x80V[a\x0E\xF2a\x15\x8DV[a\x0E\xFAa\x19MV[PV[`\rT\x81V[a\x0F\x0Ba\x19\xBAV[a\x0F\x14_a\x1AAV[V[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x81V[_`\t_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_a\x0F\x86\x82`\x01_\x86\x81R` \x01\x90\x81R` \x01_ a\x1B\x04\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[___\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[__\x1B\x81V[``a\x10\x12`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a\x1B\x1BV[\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x10C\x81a\x13\x80V[`\x02a\x10N\x81a\x14\x1CV[\x7F`\xED\xF9\xBD\xC7\xC4\xEA\0|\xAE\x1A\x9B\xBD\x03\xE4\x1E[\xFC\xCDr1\xA6\xEC8<.\xDDx\0\xF0\xD2\x0C3B`@Qa\x10\x7F\x92\x91\x90a,\x1EV[`@Q\x80\x91\x03\x90\xA1a\x10\x8Fa\x14\xA9V[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x10\xBD\x81a\x13\x80V[`\x01a\x10\xC8\x81a\x14\x1CV[\x7Fg\xC4H\x9FgM\x03\xC7\xD1\x9A\x9E6sQ\x88\xDE|e\xE8\xD1\xE9\x9E\xB3\xA2\xFD%\x8Av\x9E\xB1O\xFF3B`@Qa\x10\xF9\x92\x91\x90a,\x1EV[`@Q\x80\x91\x03\x90\xA1a\x11\ta\x14\xA9V[PPV[_a\x11'`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a\x1B:V[\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x11X\x81a\x13\x80V[_a\x11b\x81a\x14\x1CV[a\x11ja\x1BMV[\x7F\xBBp\x9D\xEAtO\x06\xD1\xB2n\x82M\xEE\xC2\xF7\x14\x0CQ\x12f\xEE\x15\xD7\xA2\x17\x83\x8B1\xD8\xB0\x12=3B`@Qa\x11\x9B\x92\x91\x90a,\x1EV[`@Q\x80\x91\x03\x90\xA1a\x11\xABa\x14\xA9V[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x11\xD9\x81a\x13\x80V[a\x11\xE1a\x13\x94V[a\x11\xEB\x83\x83a\x1C\x07V[PPPPV[`\x0BT\x81V[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x12!\x81a\x13\x80V[`\x05a\x12,\x81a\x14\x1CV[a\x127\x85\x85\x85a\x1C\x1AV[PPPPPV[a\x12Fa\x19\xBAV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x12\xB6W_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12\xAD\x91\x90a*\x93V[`@Q\x80\x91\x03\x90\xFD[a\x12\xBF\x81a\x1AAV[PV[_a\x12\xED\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x83a\x0F\x8EV[\x90P\x91\x90PV[_\x7FZ\x05\x18\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x13fWPa\x13e\x82a \x1BV[[\x90P\x91\x90PV[_a\x13x\x83\x83a \x94V[\x90P\x92\x91PPV[a\x13\x91\x81a\x13\x8Ca \xD7V[a \xDEV[PV[`\x06\x80\x81\x11\x15a\x13\xA7Wa\x13\xA6a'\xF0V[[`\x0E_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x06\x81\x11\x15a\x13\xC8Wa\x13\xC7a'\xF0V[[\x14`\x0E_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90a\x14\x19W`@Q\x7Fc\x01\x80T\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14\x10\x91\x90a(cV[`@Q\x80\x91\x03\x90\xFD[PV[\x80`\x06\x81\x11\x15a\x14/Wa\x14.a'\xF0V[[`\x0E_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x06\x81\x11\x15a\x14PWa\x14Oa'\xF0V[[\x14\x81`\x0E_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90\x91a\x14\xA4W`@Q\x7F\xBF\xA2\x17\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14\x9B\x92\x91\x90a0\xCDV[`@Q\x80\x91\x03\x90\xFD[PPPV[`\x01`\x0E_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x06\x81\x11\x15a\x14\xCCWa\x14\xCBa'\xF0V[[a\x14\xD6\x91\x90a0\xF4V[`\x06\x81\x11\x15a\x14\xE8Wa\x14\xE7a'\xF0V[[`\x0E_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x06\x81\x11\x15a\x15\x0BWa\x15\na'\xF0V[[\x02\x17\x90UPV[a\x15\x1Aa \xD7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x15~W`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x15\x88\x82\x82a\x1C\x07V[PPPV[_a\x15\xB7\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x0F\xF7V[\x90P_a\x15\xE3\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x11\rV[\x90P_a\x16\x0F\x7F`\x1A(\xE6\xAB\xEE\xA5+\x0B\xA1[T\xF6\xD1]x\x9F\xD3\xE8\xC0\x08\x0C\x04*\0\x9A\x99d\xF0\xE1\x8F\x1Ca\x0F\xF7V[\x90P_a\x16;\x7F`\x1A(\xE6\xAB\xEE\xA5+\x0B\xA1[T\xF6\xD1]x\x9F\xD3\xE8\xC0\x08\x0C\x04*\0\x9A\x99d\xF0\xE1\x8F\x1Ca\x11\rV[\x90P__\x90P[`\x04T\x81\x10\x15a\x17\x16W_`\x03_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\x08_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ __\x82\x01_\x90U`\x01\x82\x01_a\x16\xD3\x91\x90a&\x19V[PP`\x03_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90UP\x80\x80`\x01\x01\x91PPa\x16BV[P__\x90P[\x81\x81\x10\x15a\x19\x1EW_\x83\x82\x81Q\x81\x10a\x178Wa\x177a1'V[[` \x02` \x01\x01Q\x90P__\x90P[\x85\x81\x10\x15a\x18\0W`\x02_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x02\x01_\x88\x83\x81Q\x81\x10a\x17\xA3Wa\x17\xA2a1'V[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U\x80\x80`\x01\x01\x91PPa\x17GV[P`\x02_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ __\x82\x01_a\x18M\x91\x90a&SV[`\x01\x82\x01_\x90UPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\x90Wa\x18\x8Fa-\x88V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18\xC3W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x18\xAEW\x90P[P`\x02_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x01\x90\x81a\x19\x0F\x91\x90a4\x8DV[PP\x80\x80`\x01\x01\x91PPa\x17\x1CV[P_`\x05\x81\x90UP_`\x06\x81\x90UP`\x04T`\x07_\x82\x82Ta\x19@\x91\x90a0\xF4V[\x92PP\x81\x90UPPPPPV[C`\r\x81\x90UP_`\x0E_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x06\x81\x11\x15a\x19xWa\x19wa'\xF0V[[\x02\x17\x90UP\x7FQ\xFB \xDA\n\xAF\xAC\xEB\x18\xD9/\xF1\xA4v\x05\x9A\n\x8B\xBF\x16\xA0\xBF|8\xB9J\x98\xB3V\xAC\xE4W0`\rT`@Qa\x19\xB0\x92\x91\x90a,\x1EV[`@Q\x80\x91\x03\x90\xA1V[a\x19\xC2a \xD7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x19\xE0a\x0F:V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1A?Wa\x1A\x03a \xD7V[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1A6\x91\x90a*\x93V[`@Q\x80\x91\x03\x90\xFD[V[_`\t_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\t_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[_a\x1B\x11\x83_\x01\x83a!/V[_\x1C\x90P\x92\x91PPV[``_a\x1B)\x83_\x01a!VV[\x90P``\x81\x90P\x80\x92PPP\x91\x90PV[_a\x1BF\x82_\x01a!\xAFV[\x90P\x91\x90PV[_a\x1Bw\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x11\rV[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x10\x15\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91a\x1C\x02W`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1B\xF9\x92\x91\x90a4\xEFV[`@Q\x80\x91\x03\x90\xFD[PPPV[_a\x1C\x12\x83\x83a!\xBEV[\x90P\x92\x91PPV[a\x1CD\x7F`\x1A(\xE6\xAB\xEE\xA5+\x0B\xA1[T\xF6\xD1]x\x9F\xD3\xE8\xC0\x08\x0C\x04*\0\x9A\x99d\xF0\xE1\x8F\x1C\x84a\x0F\x8EV[\x83\x90a\x1C\x86W`@Q\x7F\\\x9Fq\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C}\x91\x90a*\x93V[`@Q\x80\x91\x03\x90\xFD[P_`\x02_\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x90P\x80`\x02\x01_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x843\x90\x91a\x1DYW`@Q\x7F\x08\xE5T\x95\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1DP\x92\x91\x90a5\x16V[`@Q\x80\x91\x03\x90\xFD[PP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x01T\x10a\x1D\xC1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1D\xB8\x90a5\xBDV[`@Q\x80\x91\x03\x90\xFD[`\x01\x81`\x02\x01_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x82\x82\x82_\x01\x83`\x01\x01T\x81T\x81\x10a\x1E3Wa\x1E2a1'V[[\x90_R` _ \x01\x91\x82a\x1EH\x92\x91\x90a5\xE5V[P`\x01\x81`\x01\x01_\x82\x82Ta\x1E]\x91\x90a0\xF4V[\x92PP\x81\x90UP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x01T\x10a \x15W_\x81`\x01\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xAEWa\x1E\xADa-\x88V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\xE1W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1E\xCCW\x90P[P\x90P__\x90P[\x82`\x01\x01T\x81\x10\x15a\x1F\xC4W\x82_\x01\x81\x81T\x81\x10a\x1F\nWa\x1F\ta1'V[[\x90_R` _ \x01\x80Ta\x1F\x1D\x90a-NV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1FI\x90a-NV[\x80\x15a\x1F\x94W\x80`\x1F\x10a\x1FkWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1F\x94V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1FwW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x82\x82\x81Q\x81\x10a\x1F\xACWa\x1F\xABa1'V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\x1E\xE9V[P\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xD29^\x16\xBB\xE2\x8Eih\t\xE1\xF9\xB1R\x07v\xC9\xECY\x98\xFCrlT\xE8\x9Dg\xDD\x04\x1F\x9F\xF1\x82`@Qa \x0B\x91\x90a7\xAAV[`@Q\x80\x91\x03\x90\xA2P[PPPPV[_\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a \x8DWPa \x8C\x82a\"\x01V[[\x90P\x91\x90PV[__a \xA0\x84\x84a\"jV[\x90P\x80\x15a \xCDWa \xCB\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a#S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[_3\x90P\x90V[a \xE8\x82\x82a\x0F\x8EV[a!+W\x80\x82`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a!\"\x92\x91\x90a7\xCAV[`@Q\x80\x91\x03\x90\xFD[PPV[_\x82_\x01\x82\x81T\x81\x10a!EWa!Da1'V[[\x90_R` _ \x01T\x90P\x92\x91PPV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a!\xA3W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a!\x8FW[PPPPP\x90P\x91\x90PV[_\x81_\x01\x80T\x90P\x90P\x91\x90PV[__a!\xCA\x84\x84a#\x80V[\x90P\x80\x15a!\xF7Wa!\xF5\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a$i\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[_\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[_a\"u\x83\x83a\x0F\x8EV[a#IW`\x01__\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\"\xE6a \xD7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa#MV[_\x90P[\x92\x91PPV[_a#x\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba$\x96V[\x90P\x92\x91PPV[_a#\x8B\x83\x83a\x0F\x8EV[\x15a$_W___\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa#\xFCa \xD7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa$cV[_\x90P[\x92\x91PPV[_a$\x8E\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba$\xFDV[\x90P\x92\x91PPV[_a$\xA1\x83\x83a%\xF9V[a$\xF3W\x82_\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x82_\x01\x80T\x90P\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP`\x01\x90Pa$\xF7V[_\x90P[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x14a%\xEEW_`\x01\x82a%*\x91\x90a,\xEEV[\x90P_`\x01\x86_\x01\x80T\x90Pa%@\x91\x90a,\xEEV[\x90P\x80\x82\x14a%\xA6W_\x86_\x01\x82\x81T\x81\x10a%_Wa%^a1'V[[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a%\x80Wa%\x7Fa1'V[[\x90_R` _ \x01\x81\x90UP\x83\x87`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UPP[\x85_\x01\x80T\x80a%\xB9Wa%\xB8a7\xF1V[[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa%\xF3V[_\x91PP[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x14\x15\x90P\x92\x91PPV[P\x80Ta&%\x90a-NV[_\x82U\x80`\x1F\x10a&6WPa&PV[`\x1F\x01` \x90\x04\x90_R` _ \x90a&O\x91\x90a&nV[[PV[P\x80T_\x82U\x90_R` _ \x90a&k\x91\x90a&\x8BV[PV[_[\x80\x82\x11\x15a&\x86W\x82\x81\x01_\x90U`\x01\x01a&pV[PP\x90V[_[\x80\x82\x11\x15a&\xABW\x82\x81\x01_a&\xA3\x91\x90a&\x19V[`\x01\x01a&\x8DV[PP\x90V[__\xFD[__\xFD[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a&\xEC\x81a&\xB8V[\x81\x14a&\xF6W__\xFD[PV[_\x815\x90Pa'\x07\x81a&\xE3V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a'\"Wa'!a&\xB0V[[_a'/\x84\x82\x85\x01a&\xF9V[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a'L\x81a'8V[\x82RPPV[_` \x82\x01\x90Pa'e_\x83\x01\x84a'CV[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a'\x94\x82a'kV[\x90P\x91\x90PV[a'\xA4\x81a'\x8AV[\x81\x14a'\xAEW__\xFD[PV[_\x815\x90Pa'\xBF\x81a'\x9BV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a'\xDAWa'\xD9a&\xB0V[[_a'\xE7\x84\x82\x85\x01a'\xB1V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x07\x81\x10a(.Wa(-a'\xF0V[[PV[_\x81\x90Pa(>\x82a(\x1DV[\x91\x90PV[_a(M\x82a(1V[\x90P\x91\x90PV[a(]\x81a(CV[\x82RPPV[_` \x82\x01\x90Pa(v_\x83\x01\x84a(TV[\x92\x91PPV[_\x81\x90P\x91\x90PV[a(\x8E\x81a(|V[\x82RPPV[_` \x82\x01\x90Pa(\xA7_\x83\x01\x84a(\x85V[\x92\x91PPV[a(\xB6\x81a(|V[\x81\x14a(\xC0W__\xFD[PV[_\x815\x90Pa(\xD1\x81a(\xADV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a(\xECWa(\xEBa&\xB0V[[_a(\xF9\x84\x82\x85\x01a(\xC3V[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a)\x14\x81a)\x02V[\x81\x14a)\x1EW__\xFD[PV[_\x815\x90Pa)/\x81a)\x0BV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a)JWa)Ia&\xB0V[[_a)W\x84\x82\x85\x01a)!V[\x91PP\x92\x91PPV[a)i\x81a)\x02V[\x82RPPV[_` \x82\x01\x90Pa)\x82_\x83\x01\x84a)`V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a)\x9EWa)\x9Da&\xB0V[[_a)\xAB\x85\x82\x86\x01a)!V[\x92PP` a)\xBC\x85\x82\x86\x01a'\xB1V[\x91PP\x92P\x92\x90PV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a)\xE7Wa)\xE6a)\xC6V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\x04Wa*\x03a)\xCAV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a* Wa*\x1Fa)\xCEV[[\x92P\x92\x90PV[___`@\x84\x86\x03\x12\x15a*>Wa*=a&\xB0V[[_\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*[Wa*Za&\xB4V[[a*g\x86\x82\x87\x01a)\xD2V[\x93P\x93PP` a*z\x86\x82\x87\x01a(\xC3V[\x91PP\x92P\x92P\x92V[a*\x8D\x81a'\x8AV[\x82RPPV[_` \x82\x01\x90Pa*\xA6_\x83\x01\x84a*\x84V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a*\xC2Wa*\xC1a&\xB0V[[_a*\xCF\x85\x82\x86\x01a)!V[\x92PP` a*\xE0\x85\x82\x86\x01a(\xC3V[\x91PP\x92P\x92\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a+\x1C\x81a'\x8AV[\x82RPPV[_a+-\x83\x83a+\x13V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a+O\x82a*\xEAV[a+Y\x81\x85a*\xF4V[\x93Pa+d\x83a+\x04V[\x80_[\x83\x81\x10\x15a+\x94W\x81Qa+{\x88\x82a+\"V[\x97Pa+\x86\x83a+9V[\x92PP`\x01\x81\x01\x90Pa+gV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra+\xB9\x81\x84a+EV[\x90P\x92\x91PPV[___`@\x84\x86\x03\x12\x15a+\xD8Wa+\xD7a&\xB0V[[_a+\xE5\x86\x82\x87\x01a'\xB1V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\x06Wa,\x05a&\xB4V[[a,\x12\x86\x82\x87\x01a)\xD2V[\x92P\x92PP\x92P\x92P\x92V[_`@\x82\x01\x90Pa,1_\x83\x01\x85a*\x84V[a,>` \x83\x01\x84a(\x85V[\x93\x92PPPV[_``\x82\x01\x90Pa,X_\x83\x01\x86a(\x85V[a,e` \x83\x01\x85a*\x84V[a,r`@\x83\x01\x84a*\x84V[\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a,\xB1\x82a(|V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a,\xE3Wa,\xE2a,zV[[`\x01\x82\x01\x90P\x91\x90PV[_a,\xF8\x82a(|V[\x91Pa-\x03\x83a(|V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a-\x1BWa-\x1Aa,zV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a-eW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a-xWa-wa-!V[[P\x91\x90PV[_\x81Q\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a.\x11\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a-\xD6V[a.\x1B\x86\x83a-\xD6V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a.Va.Qa.L\x84a(|V[a.3V[a(|V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a.o\x83a.<V[a.\x83a.{\x82a.]V[\x84\x84Ta-\xE2V[\x82UPPPPV[__\x90P\x90V[a.\x9Aa.\x8BV[a.\xA5\x81\x84\x84a.fV[PPPV[_[\x82\x81\x10\x15a.\xCBWa.\xC0_\x82\x84\x01a.\x92V[`\x01\x81\x01\x90Pa.\xACV[PPPV[`\x1F\x82\x11\x15a/\x1EW\x82\x82\x11\x15a/\x1DWa.\xEA\x81a-\xB5V[a.\xF3\x83a-\xC7V[a.\xFC\x85a-\xC7V[` \x86\x10\x15a/\tW_\x90P[\x80\x83\x01a/\x18\x82\x84\x03\x82a.\xAAV[PPPP[[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_a/>_\x19\x84`\x08\x02a/#V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_a/V\x83\x83a//V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a/o\x82a-~V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/\x88Wa/\x87a-\x88V[[a/\x92\x82Ta-NV[a/\x9D\x82\x82\x85a.\xD0V[_` \x90P`\x1F\x83\x11`\x01\x81\x14a/\xCEW_\x84\x15a/\xBCW\x82\x87\x01Q\x90P[a/\xC6\x85\x82a/KV[\x86UPa0-V[`\x1F\x19\x84\x16a/\xDC\x86a-\xB5V[_[\x82\x81\x10\x15a0\x03W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa/\xDEV[\x86\x83\x10\x15a0 W\x84\x89\x01Qa0\x1C`\x1F\x89\x16\x82a//V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x837_\x83\x83\x01RPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a0n\x83\x85a05V[\x93Pa0{\x83\x85\x84a0EV[a0\x84\x83a0SV[\x84\x01\x90P\x93\x92PPPV[_``\x82\x01\x90Pa0\xA2_\x83\x01\x87a*\x84V[\x81\x81\x03` \x83\x01Ra0\xB5\x81\x85\x87a0cV[\x90Pa0\xC4`@\x83\x01\x84a(\x85V[\x95\x94PPPPPV[_`@\x82\x01\x90Pa0\xE0_\x83\x01\x85a(TV[a0\xED` \x83\x01\x84a(TV[\x93\x92PPPV[_a0\xFE\x82a(|V[\x91Pa1\t\x83a(|V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a1!Wa1 a,zV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x81T\x90P\x91\x90PV[_\x81\x90P`\x01\x80`\x01\x03\x83\x01\x04\x90P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[a1\xC3\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83` \x03`\x08\x02a/#V[\x81T\x81\x16\x82UPPPV[a1\xD7\x81a-\xB5V[a1\xE2\x83\x82Ta/KV[\x80\x83U_\x82UPPPPV[` \x84\x10_\x81\x14a2IW`\x1F\x84\x11`\x01\x81\x14a2\x16Wa2\x0F\x86\x85a/KV[\x83Ua2CV[a2\x1F\x83a-\xB5V[a27`\x01a2-\x88a-\xC7V[\x03`\x01\x83\x01a.\xAAV[a2A\x87\x85a1\xCEV[P[Pa2\xA3V[a2R\x85a-\xC7V[a2[\x85a-\xC7V[a2d\x84a-\xB5V[\x82\x81\x01`\x1F\x89\x16\x80\x15a2\x7FWa2~\x81`\x01\x84\x03a1\x93V[[\x84\x84\x11\x15a2\x94Wa2\x93\x85\x85\x03\x83a.\xAAV[[`\x01\x8A`\x02\x02\x17\x87UPPPPP[PPPPPV[h\x01\0\0\0\0\0\0\0\0\x84\x11\x15a2\xC4Wa2\xC3a-\x88V[[` \x83\x10_\x81\x14a3\rW` \x85\x10_\x81\x14a2\xEBWa2\xE4\x86\x85a/KV[\x83Ua3\x07V[\x83`\xFF\x19\x16\x93P\x83a2\xFC\x84a-\xB5V[U`\x01\x86`\x02\x02\x01\x83U[Pa3\x17V[`\x01\x85`\x02\x02\x01\x82U[PPPPPV[\x80Ta3)\x81a-NV[\x80\x84\x11\x15a3>Wa3=\x84\x82\x84\x86a2\xAAV[[\x80\x84\x10\x15a3SWa3R\x84\x82\x84\x86a1\xEEV[[PPPPV[\x82\x81\x10\x15a3xWa3m_\x82\x84\x01a.\x92V[`\x01\x81\x01\x90Pa3YV[PPPV[a3\x87_\x82a3\x1EV[PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[_\x82\x14a3\xC6Wa3\xC5a3\x8AV[[a3\xCF\x81a3}V[PPV[_[\x82\x81\x10\x15a3\xF4Wa3\xE9_\x82\x84\x01a3\xB6V[`\x01\x81\x01\x90Pa3\xD5V[PPPV[\x81\x83\x10\x15a40Wa4\n\x82a1mV[a4\x13\x84a1mV[a4\x1C\x83a1\x81V[\x81\x81\x01a4+\x83\x85\x03\x82a3\xD3V[PPPP[PPPV[h\x01\0\0\0\0\0\0\0\0\x82\x11\x15a4OWa4Na-\x88V[[a4X\x81a1cV[\x82\x82Ua4f\x83\x82\x84a3\xF9V[PPPV[_\x81Q\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[a4\x89\x82\x82a/fV[PPV[a4\x96\x82a4kV[a4\xA0\x81\x83a45V[a4\xA9\x83a1TV[a4\xB2\x83a1\x81V[_[\x83\x81\x10\x15a4\xE7Wa4\xC5\x83a4uV[a4\xCF\x81\x84a4\x7FV[` \x84\x01\x93P`\x01\x83\x01\x92PP`\x01\x81\x01\x90Pa4\xB4V[PPPPPPV[_`@\x82\x01\x90Pa5\x02_\x83\x01\x85a(\x85V[a5\x0F` \x83\x01\x84a(\x85V[\x93\x92PPPV[_`@\x82\x01\x90Pa5)_\x83\x01\x85a*\x84V[a56` \x83\x01\x84a*\x84V[\x93\x92PPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FBUG: ALREADY RECEIVED SHARES FRO_\x82\x01R\x7FM N PARTIES, TOO MANY CLIENTS\0\0\0` \x82\x01RPV[_a5\xA7`=\x83a5=V[\x91Pa5\xB2\x82a5MV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra5\xD4\x81a5\x9BV[\x90P\x91\x90PV[_\x82\x90P\x92\x91PPV[a5\xEF\x83\x83a5\xDBV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\x08Wa6\x07a-\x88V[[a6\x12\x82Ta-NV[a6\x1D\x82\x82\x85a.\xD0V[_`\x1F\x83\x11`\x01\x81\x14a6JW_\x84\x15a68W\x82\x87\x015\x90P[a6B\x85\x82a/KV[\x86UPa6\xA9V[`\x1F\x19\x84\x16a6X\x86a-\xB5V[_[\x82\x81\x10\x15a6\x7FW\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa6ZV[\x86\x83\x10\x15a6\x9CW\x84\x89\x015a6\x98`\x1F\x89\x16\x82a//V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a6\xEA\x82a-~V[a6\xF4\x81\x85a6\xC2V[\x93Pa7\x04\x81\x85` \x86\x01a6\xD2V[a7\r\x81a0SV[\x84\x01\x91PP\x92\x91PPV[_a7#\x83\x83a6\xE0V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a7A\x82a4kV[a7K\x81\x85a6\xB2V[\x93P\x83` \x82\x02\x85\x01a7]\x85a1TV[\x80_[\x85\x81\x10\x15a7\x98W\x84\x84\x03\x89R\x81Qa7y\x85\x82a7\x18V[\x94Pa7\x84\x83a7+V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa7`V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra7\xC2\x81\x84a77V[\x90P\x92\x91PPV[_`@\x82\x01\x90Pa7\xDD_\x83\x01\x85a*\x84V[a7\xEA` \x83\x01\x84a)`V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \xFD\xA3\xA7\xD1\xBF\x92\xF7\xC8\xAB\xA8\x97\xCB\xEA\x15\xCB\xC5\xEB(\xB5\x81v\x90\xC6\xBCB?3\xAF+\x9E \x90dsolcC\0\x08!\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b506004361061020f575f3560e01c80635cb86b7411610123578063bb51fef0116100ab578063d547741f1161007a578063d547741f1461056d578063d8270dce14610589578063ede69216146105a7578063f2fde38b146105c3578063fc78b2e8146105df5761020f565b8063bb51fef01461051f578063c079f49514610529578063ca15c87314610533578063cb9c4cc4146105635761020f565b80638da5cb5b116100f25780638da5cb5b146104535780639010d07c1461047157806391d14854146104a1578063a217fddf146104d1578063a3246ad3146104ef5761020f565b80635cb86b74146104035780636b5e12ca1461040d578063715018a61461042b5780637f35b560146104355761020f565b80632f2ff15d116101a657806349f2ada01161017557806349f2ada0146103975780634b8e6488146103b55780634bb278f3146103bf5780635648526c146103c957806358df0d01146103e55761020f565b80632f2ff15d1461033757806330104c3e1461035357806333cc9a091461037157806336568abe1461037b5761020f565b80631c7453db116101e25780631c7453db146102af57806321dc7b9b146102cd5780632328bd12146102e9578063248a9ca3146103075761020f565b806301ffc9a71461021357806313ff6dd514610243578063146ca531146102735780631763451414610291575b5f5ffd5b61022d6004803603810190610228919061270d565b61060f565b60405161023a9190612752565b60405180910390f35b61025d600480360381019061025891906127c5565b610688565b60405161026a9190612752565b60405180910390f35b61027b6106cb565b6040516102889190612863565b60405180910390f35b6102996106dd565b6040516102a69190612894565b60405180910390f35b6102b76106e3565b6040516102c49190612894565b60405180910390f35b6102e760048036038101906102e291906128d7565b6106e9565b005b6102f16109a0565b6040516102fe9190612894565b60405180910390f35b610321600480360381019061031c9190612935565b6109b6565b60405161032e919061296f565b60405180910390f35b610351600480360381019061034c9190612988565b6109d2565b005b61035b610a14565b604051610368919061296f565b60405180910390f35b610379610a38565b005b61039560048036038101906103909190612988565b610ab2565b005b61039f610ac8565b6040516103ac919061296f565b60405180910390f35b6103bd610aec565b005b6103c7610b66565b005b6103e360048036038101906103de9190612a27565b610be0565b005b6103ed610e9c565b6040516103fa919061296f565b60405180910390f35b61040b610ec0565b005b610415610efd565b6040516104229190612894565b60405180910390f35b610433610f03565b005b61043d610f16565b60405161044a919061296f565b60405180910390f35b61045b610f3a565b6040516104689190612a93565b60405180910390f35b61048b60048036038101906104869190612aac565b610f62565b6040516104989190612a93565b60405180910390f35b6104bb60048036038101906104b69190612988565b610f8e565b6040516104c89190612752565b60405180910390f35b6104d9610ff1565b6040516104e6919061296f565b60405180910390f35b61050960048036038101906105049190612935565b610ff7565b6040516105169190612ba1565b60405180910390f35b610527611019565b005b610531611093565b005b61054d60048036038101906105489190612935565b61110d565b60405161055a9190612894565b60405180910390f35b61056b61112e565b005b61058760048036038101906105829190612988565b6111af565b005b6105916111f1565b60405161059e9190612894565b60405180910390f35b6105c160048036038101906105bc9190612bc1565b6111f7565b005b6105dd60048036038101906105d891906127c5565b61123e565b005b6105f960048036038101906105f491906127c5565b6112c2565b6040516106069190612752565b60405180910390f35b5f7f07effe0d000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191614806106815750610680826112f4565b5b9050919050565b5f610692826112c2565b80156106c457506106c37f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e83610f8e565b5b9050919050565b600e5f9054906101000a900460ff1681565b600c5481565b60075481565b600454811033829091610733576040517f6867a17000000000000000000000000000000000000000000000000000000000815260040161072a929190612c1e565b60405180910390fd5b50505f5f90505b6004548110156107f8573373ffffffffffffffffffffffffffffffffffffffff1660035f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff161415338290916107e9576040517fc315a0f50000000000000000000000000000000000000000000000000000000081526004016107e0929190612c1e565b60405180910390fd5b5050808060010191505061073a565b505f73ffffffffffffffffffffffffffffffffffffffff1660035f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1614813360035f8581526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff169091926108d0576040517fa0b8c7080000000000000000000000000000000000000000000000000000000081526004016108c793929190612c45565b60405180910390fd5b5050503360035f8381526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555061094c7fd68e3e5e367fee47ce11a5dab404596980e158eb9069330a8f775de7dc6bb64d3361136d565b5060055f81548092919061095f90612ca7565b91905055507fabde16b7a9192c31c6231b1539bad6fed77635de4c008718dbdcafb7b8363afe3382604051610995929190612c1e565b60405180910390a150565b5f6005546004546109b19190612cee565b905090565b5f5f5f8381526020019081526020015f20600101549050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6109fc81611380565b610a04611394565b610a0e838361136d565b50505050565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46981565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610a6281611380565b6003610a6d8161141c565b7f20f55ed0c92f2bb1c8825488e1e3c98463d024b2a42dbd24838c3f75260f43e93342604051610a9e929190612c1e565b60405180910390a1610aae6114a9565b5050565b610aba611394565b610ac48282611512565b5050565b7fd68e3e5e367fee47ce11a5dab404596980e158eb9069330a8f775de7dc6bb64d81565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610b1681611380565b6004610b218161141c565b7f301f8a3701f5b260197382dd7301078542144fe8fddd18083d6f6e09e4958a593342604051610b52929190612c1e565b60405180910390a1610b626114a9565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610b9081611380565b6005610b9b8161141c565b7f24a873651d26fb5a462cb37a91071cdd4d09abeebfe0ed144329bed1cc359d033342604051610bcc929190612c1e565b60405180910390a1610bdc6114a9565b5050565b7fd68e3e5e367fee47ce11a5dab404596980e158eb9069330a8f775de7dc6bb64d610c0a81611380565b3373ffffffffffffffffffffffffffffffffffffffff1660035f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff161433839091610cae576040517fffabbae7000000000000000000000000000000000000000000000000000000008152600401610ca5929190612c1e565b60405180910390fd5b50505f8484905014153390610cf9576040517f16923cea000000000000000000000000000000000000000000000000000000008152600401610cf09190612a93565b60405180910390fd5b505f60085f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206001018054610d4690612d4e565b9050143390610d8b576040517f4f5fbfc3000000000000000000000000000000000000000000000000000000008152600401610d829190612a93565b60405180910390fd5b50604051806040016040528083815260200185858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081525060085f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f820151815f01556020820151816001019081610e3e9190612f66565b509050507f56d03e5f1ebec3d4b4f9ded07e82c6bb6897c142cfbaf8dff8f9ef897ce4f75f33858585604051610e77949392919061308f565b60405180910390a160065f815480929190610e9190612ca7565b919050555050505050565b7f601a28e6abeea52b0ba15b54f6d15d789fd3e8c0080c042a009a9964f0e18f1c81565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610eea81611380565b610ef261158d565b610efa61194d565b50565b600d5481565b610f0b6119ba565b610f145f611a41565b565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e81565b5f60095f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b5f610f868260015f8681526020019081526020015f20611b0490919063ffffffff16565b905092915050565b5f5f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f5f1b81565b606061101260015f8481526020019081526020015f20611b1b565b9050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61104381611380565b600261104e8161141c565b7f60edf9bdc7c4ea007cae1a9bbd03e41e5bfccd7231a6ec383c2edd7800f0d20c334260405161107f929190612c1e565b60405180910390a161108f6114a9565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6110bd81611380565b60016110c88161141c565b7f67c4489f674d03c7d19a9e36735188de7c65e8d1e99eb3a2fd258a769eb14fff33426040516110f9929190612c1e565b60405180910390a16111096114a9565b5050565b5f61112760015f8481526020019081526020015f20611b3a565b9050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61115881611380565b5f6111628161141c565b61116a611b4d565b7fbb709dea744f06d1b26e824deec2f7140c511266ee15d7a217838b31d8b0123d334260405161119b929190612c1e565b60405180910390a16111ab6114a9565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6111d981611380565b6111e1611394565b6111eb8383611c07565b50505050565b600b5481565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46961122181611380565b600561122c8161141c565b611237858585611c1a565b5050505050565b6112466119ba565b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16036112b6575f6040517f1e4fbdf70000000000000000000000000000000000000000000000000000000081526004016112ad9190612a93565b60405180910390fd5b6112bf81611a41565b50565b5f6112ed7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46983610f8e565b9050919050565b5f7f5a05180f000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916148061136657506113658261201b565b5b9050919050565b5f6113788383612094565b905092915050565b6113918161138c6120d7565b6120de565b50565b6006808111156113a7576113a66127f0565b5b600e5f9054906101000a900460ff1660068111156113c8576113c76127f0565b5b14600e5f9054906101000a900460ff1690611419576040517f630180540000000000000000000000000000000000000000000000000000000081526004016114109190612863565b60405180910390fd5b50565b80600681111561142f5761142e6127f0565b5b600e5f9054906101000a900460ff1660068111156114505761144f6127f0565b5b1481600e5f9054906101000a900460ff1690916114a4576040517fbfa217d800000000000000000000000000000000000000000000000000000000815260040161149b9291906130cd565b60405180910390fd5b505050565b6001600e5f9054906101000a900460ff1660068111156114cc576114cb6127f0565b5b6114d691906130f4565b60068111156114e8576114e76127f0565b5b600e5f6101000a81548160ff0219169083600681111561150b5761150a6127f0565b5b0217905550565b61151a6120d7565b73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff161461157e576040517f6697b23200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6115888282611c07565b505050565b5f6115b77fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469610ff7565b90505f6115e37fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46961110d565b90505f61160f7f601a28e6abeea52b0ba15b54f6d15d789fd3e8c0080c042a009a9964f0e18f1c610ff7565b90505f61163b7f601a28e6abeea52b0ba15b54f6d15d789fd3e8c0080c042a009a9964f0e18f1c61110d565b90505f5f90505b600454811015611716575f60035f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905060085f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f5f82015f9055600182015f6116d39190612619565b505060035f8381526020019081526020015f205f6101000a81549073ffffffffffffffffffffffffffffffffffffffff0219169055508080600101915050611642565b505f5f90505b8181101561191e575f83828151811061173857611737613127565b5b602002602001015190505f5f90505b858110156118005760025f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206002015f8883815181106117a3576117a2613127565b5b602002602001015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81549060ff02191690558080600101915050611747565b5060025f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f5f82015f61184d9190612653565b600182015f905550507f000000000000000000000000000000000000000000000000000000000000000067ffffffffffffffff8111156118905761188f612d88565b5b6040519080825280602002602001820160405280156118c357816020015b60608152602001906001900390816118ae5790505b5060025f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f01908161190f919061348d565b5050808060010191505061171c565b505f6005819055505f60068190555060045460075f82825461194091906130f4565b9250508190555050505050565b43600d819055505f600e5f6101000a81548160ff02191690836006811115611978576119776127f0565b5b02179055507f51fb20da0aafaceb18d92ff1a476059a0a8bbf16a0bf7c38b94a98b356ace45730600d546040516119b0929190612c1e565b60405180910390a1565b6119c26120d7565b73ffffffffffffffffffffffffffffffffffffffff166119e0610f3a565b73ffffffffffffffffffffffffffffffffffffffff1614611a3f57611a036120d7565b6040517f118cdaa7000000000000000000000000000000000000000000000000000000008152600401611a369190612a93565b60405180910390fd5b565b5f60095f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690508160095f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a35050565b5f611b11835f018361212f565b5f1c905092915050565b60605f611b29835f01612156565b905060608190508092505050919050565b5f611b46825f016121af565b9050919050565b5f611b777fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46961110d565b90507f0000000000000000000000000000000000000000000000000000000000000000811015817f00000000000000000000000000000000000000000000000000000000000000009091611c02576040517f3a236268000000000000000000000000000000000000000000000000000000008152600401611bf99291906134ef565b60405180910390fd5b505050565b5f611c1283836121be565b905092915050565b611c447f601a28e6abeea52b0ba15b54f6d15d789fd3e8c0080c042a009a9964f0e18f1c84610f8e565b8390611c86576040517f5c9f71ac000000000000000000000000000000000000000000000000000000008152600401611c7d9190612a93565b60405180910390fd5b505f60025f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f209050806002015f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff161584339091611d59576040517f08e55495000000000000000000000000000000000000000000000000000000008152600401611d50929190613516565b60405180910390fd5b50507f0000000000000000000000000000000000000000000000000000000000000000816001015410611dc1576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611db8906135bd565b60405180910390fd5b6001816002015f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055508282825f01836001015481548110611e3357611e32613127565b5b905f5260205f20019182611e489291906135e5565b506001816001015f828254611e5d91906130f4565b925050819055507f0000000000000000000000000000000000000000000000000000000000000000816001015410612015575f816001015467ffffffffffffffff811115611eae57611ead612d88565b5b604051908082528060200260200182016040528015611ee157816020015b6060815260200190600190039081611ecc5790505b5090505f5f90505b8260010154811015611fc457825f018181548110611f0a57611f09613127565b5b905f5260205f20018054611f1d90612d4e565b80601f0160208091040260200160405190810160405280929190818152602001828054611f4990612d4e565b8015611f945780601f10611f6b57610100808354040283529160200191611f94565b820191905f5260205f20905b815481529060010190602001808311611f7757829003601f168201915b5050505050828281518110611fac57611fab613127565b5b60200260200101819052508080600101915050611ee9565b508473ffffffffffffffffffffffffffffffffffffffff167fd2395e16bbe28e696809e1f9b1520776c9ec5998fc726c54e89d67dd041f9ff18260405161200b91906137aa565b60405180910390a2505b50505050565b5f7f7965db0b000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916148061208d575061208c82612201565b5b9050919050565b5f5f6120a0848461226a565b905080156120cd576120cb8360015f8781526020019081526020015f2061235390919063ffffffff16565b505b8091505092915050565b5f33905090565b6120e88282610f8e565b61212b5780826040517fe2517d3f0000000000000000000000000000000000000000000000000000000081526004016121229291906137ca565b60405180910390fd5b5050565b5f825f01828154811061214557612144613127565b5b905f5260205f200154905092915050565b6060815f018054806020026020016040519081016040528092919081815260200182805480156121a357602002820191905f5260205f20905b81548152602001906001019080831161218f575b50505050509050919050565b5f815f01805490509050919050565b5f5f6121ca8484612380565b905080156121f7576121f58360015f8781526020019081526020015f2061246990919063ffffffff16565b505b8091505092915050565b5f7f01ffc9a7000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916149050919050565b5f6122758383610f8e565b6123495760015f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055506122e66120d7565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a46001905061234d565b5f90505b92915050565b5f612378835f018373ffffffffffffffffffffffffffffffffffffffff165f1b612496565b905092915050565b5f61238b8383610f8e565b1561245f575f5f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055506123fc6120d7565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b60405160405180910390a460019050612463565b5f90505b92915050565b5f61248e835f018373ffffffffffffffffffffffffffffffffffffffff165f1b6124fd565b905092915050565b5f6124a183836125f9565b6124f357825f0182908060018154018082558091505060019003905f5260205f20015f9091909190915055825f0180549050836001015f8481526020019081526020015f2081905550600190506124f7565b5f90505b92915050565b5f5f836001015f8481526020019081526020015f205490505f81146125ee575f60018261252a9190612cee565b90505f6001865f01805490506125409190612cee565b90508082146125a6575f865f01828154811061255f5761255e613127565b5b905f5260205f200154905080875f0184815481106125805761257f613127565b5b905f5260205f20018190555083876001015f8381526020019081526020015f2081905550505b855f018054806125b9576125b86137f1565b5b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f9055600193505050506125f3565b5f9150505b92915050565b5f5f836001015f8481526020019081526020015f20541415905092915050565b50805461262590612d4e565b5f825580601f106126365750612650565b601f0160209004905f5260205f209061264f919061266e565b5b50565b5080545f8255905f5260205f209061266b919061268b565b50565b5f5b80821115612686578281015f9055600101612670565b505090565b5f5b808211156126ab578281015f6126a39190612619565b60010161268d565b505090565b5f5ffd5b5f5ffd5b5f7fffffffff0000000000000000000000000000000000000000000000000000000082169050919050565b6126ec816126b8565b81146126f6575f5ffd5b50565b5f81359050612707816126e3565b92915050565b5f60208284031215612722576127216126b0565b5b5f61272f848285016126f9565b91505092915050565b5f8115159050919050565b61274c81612738565b82525050565b5f6020820190506127655f830184612743565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6127948261276b565b9050919050565b6127a48161278a565b81146127ae575f5ffd5b50565b5f813590506127bf8161279b565b92915050565b5f602082840312156127da576127d96126b0565b5b5f6127e7848285016127b1565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6007811061282e5761282d6127f0565b5b50565b5f81905061283e8261281d565b919050565b5f61284d82612831565b9050919050565b61285d81612843565b82525050565b5f6020820190506128765f830184612854565b92915050565b5f819050919050565b61288e8161287c565b82525050565b5f6020820190506128a75f830184612885565b92915050565b6128b68161287c565b81146128c0575f5ffd5b50565b5f813590506128d1816128ad565b92915050565b5f602082840312156128ec576128eb6126b0565b5b5f6128f9848285016128c3565b91505092915050565b5f819050919050565b61291481612902565b811461291e575f5ffd5b50565b5f8135905061292f8161290b565b92915050565b5f6020828403121561294a576129496126b0565b5b5f61295784828501612921565b91505092915050565b61296981612902565b82525050565b5f6020820190506129825f830184612960565b92915050565b5f5f6040838503121561299e5761299d6126b0565b5b5f6129ab85828601612921565b92505060206129bc858286016127b1565b9150509250929050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f8401126129e7576129e66129c6565b5b8235905067ffffffffffffffff811115612a0457612a036129ca565b5b602083019150836001820283011115612a2057612a1f6129ce565b5b9250929050565b5f5f5f60408486031215612a3e57612a3d6126b0565b5b5f84013567ffffffffffffffff811115612a5b57612a5a6126b4565b5b612a67868287016129d2565b93509350506020612a7a868287016128c3565b9150509250925092565b612a8d8161278a565b82525050565b5f602082019050612aa65f830184612a84565b92915050565b5f5f60408385031215612ac257612ac16126b0565b5b5f612acf85828601612921565b9250506020612ae0858286016128c3565b9150509250929050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b612b1c8161278a565b82525050565b5f612b2d8383612b13565b60208301905092915050565b5f602082019050919050565b5f612b4f82612aea565b612b598185612af4565b9350612b6483612b04565b805f5b83811015612b94578151612b7b8882612b22565b9750612b8683612b39565b925050600181019050612b67565b5085935050505092915050565b5f6020820190508181035f830152612bb98184612b45565b905092915050565b5f5f5f60408486031215612bd857612bd76126b0565b5b5f612be5868287016127b1565b935050602084013567ffffffffffffffff811115612c0657612c056126b4565b5b612c12868287016129d2565b92509250509250925092565b5f604082019050612c315f830185612a84565b612c3e6020830184612885565b9392505050565b5f606082019050612c585f830186612885565b612c656020830185612a84565b612c726040830184612a84565b949350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f612cb18261287c565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203612ce357612ce2612c7a565b5b600182019050919050565b5f612cf88261287c565b9150612d038361287c565b9250828203905081811115612d1b57612d1a612c7a565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f6002820490506001821680612d6557607f821691505b602082108103612d7857612d77612d21565b5b50919050565b5f81519050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f60088302612e117fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82612dd6565b612e1b8683612dd6565b95508019841693508086168417925050509392505050565b5f819050919050565b5f612e56612e51612e4c8461287c565b612e33565b61287c565b9050919050565b5f819050919050565b612e6f83612e3c565b612e83612e7b82612e5d565b848454612de2565b825550505050565b5f5f905090565b612e9a612e8b565b612ea5818484612e66565b505050565b5f5b82811015612ecb57612ec05f828401612e92565b600181019050612eac565b505050565b601f821115612f1e5782821115612f1d57612eea81612db5565b612ef383612dc7565b612efc85612dc7565b6020861015612f09575f90505b808301612f1882840382612eaa565b505050505b5b505050565b5f82821c905092915050565b5f612f3e5f1984600802612f23565b1980831691505092915050565b5f612f568383612f2f565b9150826002028217905092915050565b612f6f82612d7e565b67ffffffffffffffff811115612f8857612f87612d88565b5b612f928254612d4e565b612f9d828285612ed0565b5f60209050601f831160018114612fce575f8415612fbc578287015190505b612fc68582612f4b565b86555061302d565b601f198416612fdc86612db5565b5f5b8281101561300357848901518255600182019150602085019450602081019050612fde565b86831015613020578489015161301c601f891682612f2f565b8355505b6001600288020188555050505b505050505050565b5f82825260208201905092915050565b828183375f83830152505050565b5f601f19601f8301169050919050565b5f61306e8385613035565b935061307b838584613045565b61308483613053565b840190509392505050565b5f6060820190506130a25f830187612a84565b81810360208301526130b5818587613063565b90506130c46040830184612885565b95945050505050565b5f6040820190506130e05f830185612854565b6130ed6020830184612854565b9392505050565b5f6130fe8261287c565b91506131098361287c565b925082820190508082111561312157613120612c7a565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f819050602082019050919050565b5f81549050919050565b5f8190506001806001038301049050919050565b5f819050815f5260205f209050919050565b6131c37fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff83602003600802612f23565b815481168255505050565b6131d781612db5565b6131e2838254612f4b565b8083555f825550505050565b602084105f811461324957601f8411600181146132165761320f8685612f4b565b8355613243565b61321f83612db5565b613237600161322d88612dc7565b0360018301612eaa565b61324187856131ce565b505b506132a3565b61325285612dc7565b61325b85612dc7565b61326484612db5565b828101601f8916801561327f5761327e8160018403613193565b5b848411156132945761329385850383612eaa565b5b60018a60020217875550505050505b5050505050565b680100000000000000008411156132c4576132c3612d88565b5b602083105f811461330d57602085105f81146132eb576132e48685612f4b565b8355613307565b8360ff19169350836132fc84612db5565b556001866002020183555b50613317565b6001856002020182555b5050505050565b805461332981612d4e565b8084111561333e5761333d848284866132aa565b5b8084101561335357613352848284866131ee565b5b50505050565b828110156133785761336d5f828401612e92565b600181019050613359565b505050565b6133875f8261331e565b50565b7f4e487b71000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b5f82146133c6576133c561338a565b5b6133cf8161337d565b5050565b5f5b828110156133f4576133e95f8284016133b6565b6001810190506133d5565b505050565b818310156134305761340a8261316d565b6134138461316d565b61341c83613181565b81810161342b838503826133d3565b505050505b505050565b6801000000000000000082111561344f5761344e612d88565b5b61345881613163565b8282556134668382846133f9565b505050565b5f81519050919050565b5f81519050919050565b6134898282612f66565b5050565b6134968261346b565b6134a08183613435565b6134a983613154565b6134b283613181565b5f5b838110156134e7576134c583613475565b6134cf818461347f565b602084019350600183019250506001810190506134b4565b505050505050565b5f6040820190506135025f830185612885565b61350f6020830184612885565b9392505050565b5f6040820190506135295f830185612a84565b6135366020830184612a84565b9392505050565b5f82825260208201905092915050565b7f4255473a20414c5245414459205245434549564544205348415245532046524f5f8201527f4d204e20504152544945532c20544f4f204d414e5920434c49454e5453000000602082015250565b5f6135a7603d8361353d565b91506135b28261354d565b604082019050919050565b5f6020820190508181035f8301526135d48161359b565b9050919050565b5f82905092915050565b6135ef83836135db565b67ffffffffffffffff81111561360857613607612d88565b5b6136128254612d4e565b61361d828285612ed0565b5f601f83116001811461364a575f8415613638578287013590505b6136428582612f4b565b8655506136a9565b601f19841661365886612db5565b5f5b8281101561367f5784890135825560018201915060208501945060208101905061365a565b8683101561369c5784890135613698601f891682612f2f565b8355505b6001600288020188555050505b50505050505050565b5f82825260208201905092915050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f6136ea82612d7e565b6136f481856136c2565b93506137048185602086016136d2565b61370d81613053565b840191505092915050565b5f61372383836136e0565b905092915050565b5f602082019050919050565b5f6137418261346b565b61374b81856136b2565b93508360208202850161375d85613154565b805f5b8581101561379857848403895281516137798582613718565b94506137848361372b565b925060208a01995050600181019050613760565b50829750879550505050505092915050565b5f6020820190508181035f8301526137c28184613737565b905092915050565b5f6040820190506137dd5f830185612a84565b6137ea6020830184612960565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffdfea2646970667358221220fda3a7d1bf92f7c8aba897cbea15cbc5eb28b5817690c6bc423f33af2b9e209064736f6c63430008210033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02\x0FW_5`\xE0\x1C\x80c\\\xB8kt\x11a\x01#W\x80c\xBBQ\xFE\xF0\x11a\0\xABW\x80c\xD5Gt\x1F\x11a\0zW\x80c\xD5Gt\x1F\x14a\x05mW\x80c\xD8'\r\xCE\x14a\x05\x89W\x80c\xED\xE6\x92\x16\x14a\x05\xA7W\x80c\xF2\xFD\xE3\x8B\x14a\x05\xC3W\x80c\xFCx\xB2\xE8\x14a\x05\xDFWa\x02\x0FV[\x80c\xBBQ\xFE\xF0\x14a\x05\x1FW\x80c\xC0y\xF4\x95\x14a\x05)W\x80c\xCA\x15\xC8s\x14a\x053W\x80c\xCB\x9CL\xC4\x14a\x05cWa\x02\x0FV[\x80c\x8D\xA5\xCB[\x11a\0\xF2W\x80c\x8D\xA5\xCB[\x14a\x04SW\x80c\x90\x10\xD0|\x14a\x04qW\x80c\x91\xD1HT\x14a\x04\xA1W\x80c\xA2\x17\xFD\xDF\x14a\x04\xD1W\x80c\xA3$j\xD3\x14a\x04\xEFWa\x02\x0FV[\x80c\\\xB8kt\x14a\x04\x03W\x80ck^\x12\xCA\x14a\x04\rW\x80cqP\x18\xA6\x14a\x04+W\x80c\x7F5\xB5`\x14a\x045Wa\x02\x0FV[\x80c//\xF1]\x11a\x01\xA6W\x80cI\xF2\xAD\xA0\x11a\x01uW\x80cI\xF2\xAD\xA0\x14a\x03\x97W\x80cK\x8Ed\x88\x14a\x03\xB5W\x80cK\xB2x\xF3\x14a\x03\xBFW\x80cVHRl\x14a\x03\xC9W\x80cX\xDF\r\x01\x14a\x03\xE5Wa\x02\x0FV[\x80c//\xF1]\x14a\x037W\x80c0\x10L>\x14a\x03SW\x80c3\xCC\x9A\t\x14a\x03qW\x80c6V\x8A\xBE\x14a\x03{Wa\x02\x0FV[\x80c\x1CtS\xDB\x11a\x01\xE2W\x80c\x1CtS\xDB\x14a\x02\xAFW\x80c!\xDC{\x9B\x14a\x02\xCDW\x80c#(\xBD\x12\x14a\x02\xE9W\x80c$\x8A\x9C\xA3\x14a\x03\x07Wa\x02\x0FV[\x80c\x01\xFF\xC9\xA7\x14a\x02\x13W\x80c\x13\xFFm\xD5\x14a\x02CW\x80c\x14l\xA51\x14a\x02sW\x80c\x17cE\x14\x14a\x02\x91W[__\xFD[a\x02-`\x04\x806\x03\x81\x01\x90a\x02(\x91\x90a'\rV[a\x06\x0FV[`@Qa\x02:\x91\x90a'RV[`@Q\x80\x91\x03\x90\xF3[a\x02]`\x04\x806\x03\x81\x01\x90a\x02X\x91\x90a'\xC5V[a\x06\x88V[`@Qa\x02j\x91\x90a'RV[`@Q\x80\x91\x03\x90\xF3[a\x02{a\x06\xCBV[`@Qa\x02\x88\x91\x90a(cV[`@Q\x80\x91\x03\x90\xF3[a\x02\x99a\x06\xDDV[`@Qa\x02\xA6\x91\x90a(\x94V[`@Q\x80\x91\x03\x90\xF3[a\x02\xB7a\x06\xE3V[`@Qa\x02\xC4\x91\x90a(\x94V[`@Q\x80\x91\x03\x90\xF3[a\x02\xE7`\x04\x806\x03\x81\x01\x90a\x02\xE2\x91\x90a(\xD7V[a\x06\xE9V[\0[a\x02\xF1a\t\xA0V[`@Qa\x02\xFE\x91\x90a(\x94V[`@Q\x80\x91\x03\x90\xF3[a\x03!`\x04\x806\x03\x81\x01\x90a\x03\x1C\x91\x90a)5V[a\t\xB6V[`@Qa\x03.\x91\x90a)oV[`@Q\x80\x91\x03\x90\xF3[a\x03Q`\x04\x806\x03\x81\x01\x90a\x03L\x91\x90a)\x88V[a\t\xD2V[\0[a\x03[a\n\x14V[`@Qa\x03h\x91\x90a)oV[`@Q\x80\x91\x03\x90\xF3[a\x03ya\n8V[\0[a\x03\x95`\x04\x806\x03\x81\x01\x90a\x03\x90\x91\x90a)\x88V[a\n\xB2V[\0[a\x03\x9Fa\n\xC8V[`@Qa\x03\xAC\x91\x90a)oV[`@Q\x80\x91\x03\x90\xF3[a\x03\xBDa\n\xECV[\0[a\x03\xC7a\x0BfV[\0[a\x03\xE3`\x04\x806\x03\x81\x01\x90a\x03\xDE\x91\x90a*'V[a\x0B\xE0V[\0[a\x03\xEDa\x0E\x9CV[`@Qa\x03\xFA\x91\x90a)oV[`@Q\x80\x91\x03\x90\xF3[a\x04\x0Ba\x0E\xC0V[\0[a\x04\x15a\x0E\xFDV[`@Qa\x04\"\x91\x90a(\x94V[`@Q\x80\x91\x03\x90\xF3[a\x043a\x0F\x03V[\0[a\x04=a\x0F\x16V[`@Qa\x04J\x91\x90a)oV[`@Q\x80\x91\x03\x90\xF3[a\x04[a\x0F:V[`@Qa\x04h\x91\x90a*\x93V[`@Q\x80\x91\x03\x90\xF3[a\x04\x8B`\x04\x806\x03\x81\x01\x90a\x04\x86\x91\x90a*\xACV[a\x0FbV[`@Qa\x04\x98\x91\x90a*\x93V[`@Q\x80\x91\x03\x90\xF3[a\x04\xBB`\x04\x806\x03\x81\x01\x90a\x04\xB6\x91\x90a)\x88V[a\x0F\x8EV[`@Qa\x04\xC8\x91\x90a'RV[`@Q\x80\x91\x03\x90\xF3[a\x04\xD9a\x0F\xF1V[`@Qa\x04\xE6\x91\x90a)oV[`@Q\x80\x91\x03\x90\xF3[a\x05\t`\x04\x806\x03\x81\x01\x90a\x05\x04\x91\x90a)5V[a\x0F\xF7V[`@Qa\x05\x16\x91\x90a+\xA1V[`@Q\x80\x91\x03\x90\xF3[a\x05'a\x10\x19V[\0[a\x051a\x10\x93V[\0[a\x05M`\x04\x806\x03\x81\x01\x90a\x05H\x91\x90a)5V[a\x11\rV[`@Qa\x05Z\x91\x90a(\x94V[`@Q\x80\x91\x03\x90\xF3[a\x05ka\x11.V[\0[a\x05\x87`\x04\x806\x03\x81\x01\x90a\x05\x82\x91\x90a)\x88V[a\x11\xAFV[\0[a\x05\x91a\x11\xF1V[`@Qa\x05\x9E\x91\x90a(\x94V[`@Q\x80\x91\x03\x90\xF3[a\x05\xC1`\x04\x806\x03\x81\x01\x90a\x05\xBC\x91\x90a+\xC1V[a\x11\xF7V[\0[a\x05\xDD`\x04\x806\x03\x81\x01\x90a\x05\xD8\x91\x90a'\xC5V[a\x12>V[\0[a\x05\xF9`\x04\x806\x03\x81\x01\x90a\x05\xF4\x91\x90a'\xC5V[a\x12\xC2V[`@Qa\x06\x06\x91\x90a'RV[`@Q\x80\x91\x03\x90\xF3[_\x7F\x07\xEF\xFE\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x06\x81WPa\x06\x80\x82a\x12\xF4V[[\x90P\x91\x90PV[_a\x06\x92\x82a\x12\xC2V[\x80\x15a\x06\xC4WPa\x06\xC3\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x83a\x0F\x8EV[[\x90P\x91\x90PV[`\x0E_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\x0CT\x81V[`\x07T\x81V[`\x04T\x81\x103\x82\x90\x91a\x073W`@Q\x7Fhg\xA1p\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07*\x92\x91\x90a,\x1EV[`@Q\x80\x91\x03\x90\xFD[PP__\x90P[`\x04T\x81\x10\x15a\x07\xF8W3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x153\x82\x90\x91a\x07\xE9W`@Q\x7F\xC3\x15\xA0\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\xE0\x92\x91\x90a,\x1EV[`@Q\x80\x91\x03\x90\xFD[PP\x80\x80`\x01\x01\x91PPa\x07:V[P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x813`\x03_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x91\x92a\x08\xD0W`@Q\x7F\xA0\xB8\xC7\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xC7\x93\x92\x91\x90a,EV[`@Q\x80\x91\x03\x90\xFD[PPP3`\x03_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPa\tL\x7F\xD6\x8E>^6\x7F\xEEG\xCE\x11\xA5\xDA\xB4\x04Yi\x80\xE1X\xEB\x90i3\n\x8Fw]\xE7\xDCk\xB6M3a\x13mV[P`\x05_\x81T\x80\x92\x91\x90a\t_\x90a,\xA7V[\x91\x90PUP\x7F\xAB\xDE\x16\xB7\xA9\x19,1\xC6#\x1B\x159\xBA\xD6\xFE\xD7v5\xDEL\0\x87\x18\xDB\xDC\xAF\xB7\xB86:\xFE3\x82`@Qa\t\x95\x92\x91\x90a,\x1EV[`@Q\x80\x91\x03\x90\xA1PV[_`\x05T`\x04Ta\t\xB1\x91\x90a,\xEEV[\x90P\x90V[___\x83\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\t\xFC\x81a\x13\x80V[a\n\x04a\x13\x94V[a\n\x0E\x83\x83a\x13mV[PPPPV[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x81V[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\nb\x81a\x13\x80V[`\x03a\nm\x81a\x14\x1CV[\x7F \xF5^\xD0\xC9/+\xB1\xC8\x82T\x88\xE1\xE3\xC9\x84c\xD0$\xB2\xA4-\xBD$\x83\x8C?u&\x0FC\xE93B`@Qa\n\x9E\x92\x91\x90a,\x1EV[`@Q\x80\x91\x03\x90\xA1a\n\xAEa\x14\xA9V[PPV[a\n\xBAa\x13\x94V[a\n\xC4\x82\x82a\x15\x12V[PPV[\x7F\xD6\x8E>^6\x7F\xEEG\xCE\x11\xA5\xDA\xB4\x04Yi\x80\xE1X\xEB\x90i3\n\x8Fw]\xE7\xDCk\xB6M\x81V[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0B\x16\x81a\x13\x80V[`\x04a\x0B!\x81a\x14\x1CV[\x7F0\x1F\x8A7\x01\xF5\xB2`\x19s\x82\xDDs\x01\x07\x85B\x14O\xE8\xFD\xDD\x18\x08=on\t\xE4\x95\x8AY3B`@Qa\x0BR\x92\x91\x90a,\x1EV[`@Q\x80\x91\x03\x90\xA1a\x0Bba\x14\xA9V[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0B\x90\x81a\x13\x80V[`\x05a\x0B\x9B\x81a\x14\x1CV[\x7F$\xA8se\x1D&\xFBZF,\xB3z\x91\x07\x1C\xDDM\t\xAB\xEE\xBF\xE0\xED\x14C)\xBE\xD1\xCC5\x9D\x033B`@Qa\x0B\xCC\x92\x91\x90a,\x1EV[`@Q\x80\x91\x03\x90\xA1a\x0B\xDCa\x14\xA9V[PPV[\x7F\xD6\x8E>^6\x7F\xEEG\xCE\x11\xA5\xDA\xB4\x04Yi\x80\xE1X\xEB\x90i3\n\x8Fw]\xE7\xDCk\xB6Ma\x0C\n\x81a\x13\x80V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x143\x83\x90\x91a\x0C\xAEW`@Q\x7F\xFF\xAB\xBA\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\xA5\x92\x91\x90a,\x1EV[`@Q\x80\x91\x03\x90\xFD[PP_\x84\x84\x90P\x14\x153\x90a\x0C\xF9W`@Q\x7F\x16\x92<\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\xF0\x91\x90a*\x93V[`@Q\x80\x91\x03\x90\xFD[P_`\x08_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x01\x80Ta\rF\x90a-NV[\x90P\x143\x90a\r\x8BW`@Q\x7FO_\xBF\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\x82\x91\x90a*\x93V[`@Q\x80\x91\x03\x90\xFD[P`@Q\x80`@\x01`@R\x80\x83\x81R` \x01\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81RP`\x08_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01U` \x82\x01Q\x81`\x01\x01\x90\x81a\x0E>\x91\x90a/fV[P\x90PP\x7FV\xD0>_\x1E\xBE\xC3\xD4\xB4\xF9\xDE\xD0~\x82\xC6\xBBh\x97\xC1B\xCF\xBA\xF8\xDF\xF8\xF9\xEF\x89|\xE4\xF7_3\x85\x85\x85`@Qa\x0Ew\x94\x93\x92\x91\x90a0\x8FV[`@Q\x80\x91\x03\x90\xA1`\x06_\x81T\x80\x92\x91\x90a\x0E\x91\x90a,\xA7V[\x91\x90PUPPPPPV[\x7F`\x1A(\xE6\xAB\xEE\xA5+\x0B\xA1[T\xF6\xD1]x\x9F\xD3\xE8\xC0\x08\x0C\x04*\0\x9A\x99d\xF0\xE1\x8F\x1C\x81V[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0E\xEA\x81a\x13\x80V[a\x0E\xF2a\x15\x8DV[a\x0E\xFAa\x19MV[PV[`\rT\x81V[a\x0F\x0Ba\x19\xBAV[a\x0F\x14_a\x1AAV[V[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x81V[_`\t_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_a\x0F\x86\x82`\x01_\x86\x81R` \x01\x90\x81R` \x01_ a\x1B\x04\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[___\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[__\x1B\x81V[``a\x10\x12`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a\x1B\x1BV[\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x10C\x81a\x13\x80V[`\x02a\x10N\x81a\x14\x1CV[\x7F`\xED\xF9\xBD\xC7\xC4\xEA\0|\xAE\x1A\x9B\xBD\x03\xE4\x1E[\xFC\xCDr1\xA6\xEC8<.\xDDx\0\xF0\xD2\x0C3B`@Qa\x10\x7F\x92\x91\x90a,\x1EV[`@Q\x80\x91\x03\x90\xA1a\x10\x8Fa\x14\xA9V[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x10\xBD\x81a\x13\x80V[`\x01a\x10\xC8\x81a\x14\x1CV[\x7Fg\xC4H\x9FgM\x03\xC7\xD1\x9A\x9E6sQ\x88\xDE|e\xE8\xD1\xE9\x9E\xB3\xA2\xFD%\x8Av\x9E\xB1O\xFF3B`@Qa\x10\xF9\x92\x91\x90a,\x1EV[`@Q\x80\x91\x03\x90\xA1a\x11\ta\x14\xA9V[PPV[_a\x11'`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a\x1B:V[\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x11X\x81a\x13\x80V[_a\x11b\x81a\x14\x1CV[a\x11ja\x1BMV[\x7F\xBBp\x9D\xEAtO\x06\xD1\xB2n\x82M\xEE\xC2\xF7\x14\x0CQ\x12f\xEE\x15\xD7\xA2\x17\x83\x8B1\xD8\xB0\x12=3B`@Qa\x11\x9B\x92\x91\x90a,\x1EV[`@Q\x80\x91\x03\x90\xA1a\x11\xABa\x14\xA9V[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x11\xD9\x81a\x13\x80V[a\x11\xE1a\x13\x94V[a\x11\xEB\x83\x83a\x1C\x07V[PPPPV[`\x0BT\x81V[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x12!\x81a\x13\x80V[`\x05a\x12,\x81a\x14\x1CV[a\x127\x85\x85\x85a\x1C\x1AV[PPPPPV[a\x12Fa\x19\xBAV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x12\xB6W_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12\xAD\x91\x90a*\x93V[`@Q\x80\x91\x03\x90\xFD[a\x12\xBF\x81a\x1AAV[PV[_a\x12\xED\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x83a\x0F\x8EV[\x90P\x91\x90PV[_\x7FZ\x05\x18\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x13fWPa\x13e\x82a \x1BV[[\x90P\x91\x90PV[_a\x13x\x83\x83a \x94V[\x90P\x92\x91PPV[a\x13\x91\x81a\x13\x8Ca \xD7V[a \xDEV[PV[`\x06\x80\x81\x11\x15a\x13\xA7Wa\x13\xA6a'\xF0V[[`\x0E_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x06\x81\x11\x15a\x13\xC8Wa\x13\xC7a'\xF0V[[\x14`\x0E_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90a\x14\x19W`@Q\x7Fc\x01\x80T\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14\x10\x91\x90a(cV[`@Q\x80\x91\x03\x90\xFD[PV[\x80`\x06\x81\x11\x15a\x14/Wa\x14.a'\xF0V[[`\x0E_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x06\x81\x11\x15a\x14PWa\x14Oa'\xF0V[[\x14\x81`\x0E_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90\x91a\x14\xA4W`@Q\x7F\xBF\xA2\x17\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14\x9B\x92\x91\x90a0\xCDV[`@Q\x80\x91\x03\x90\xFD[PPPV[`\x01`\x0E_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x06\x81\x11\x15a\x14\xCCWa\x14\xCBa'\xF0V[[a\x14\xD6\x91\x90a0\xF4V[`\x06\x81\x11\x15a\x14\xE8Wa\x14\xE7a'\xF0V[[`\x0E_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x06\x81\x11\x15a\x15\x0BWa\x15\na'\xF0V[[\x02\x17\x90UPV[a\x15\x1Aa \xD7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x15~W`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x15\x88\x82\x82a\x1C\x07V[PPPV[_a\x15\xB7\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x0F\xF7V[\x90P_a\x15\xE3\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x11\rV[\x90P_a\x16\x0F\x7F`\x1A(\xE6\xAB\xEE\xA5+\x0B\xA1[T\xF6\xD1]x\x9F\xD3\xE8\xC0\x08\x0C\x04*\0\x9A\x99d\xF0\xE1\x8F\x1Ca\x0F\xF7V[\x90P_a\x16;\x7F`\x1A(\xE6\xAB\xEE\xA5+\x0B\xA1[T\xF6\xD1]x\x9F\xD3\xE8\xC0\x08\x0C\x04*\0\x9A\x99d\xF0\xE1\x8F\x1Ca\x11\rV[\x90P__\x90P[`\x04T\x81\x10\x15a\x17\x16W_`\x03_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\x08_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ __\x82\x01_\x90U`\x01\x82\x01_a\x16\xD3\x91\x90a&\x19V[PP`\x03_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90UP\x80\x80`\x01\x01\x91PPa\x16BV[P__\x90P[\x81\x81\x10\x15a\x19\x1EW_\x83\x82\x81Q\x81\x10a\x178Wa\x177a1'V[[` \x02` \x01\x01Q\x90P__\x90P[\x85\x81\x10\x15a\x18\0W`\x02_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x02\x01_\x88\x83\x81Q\x81\x10a\x17\xA3Wa\x17\xA2a1'V[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U\x80\x80`\x01\x01\x91PPa\x17GV[P`\x02_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ __\x82\x01_a\x18M\x91\x90a&SV[`\x01\x82\x01_\x90UPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\x90Wa\x18\x8Fa-\x88V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18\xC3W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x18\xAEW\x90P[P`\x02_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x01\x90\x81a\x19\x0F\x91\x90a4\x8DV[PP\x80\x80`\x01\x01\x91PPa\x17\x1CV[P_`\x05\x81\x90UP_`\x06\x81\x90UP`\x04T`\x07_\x82\x82Ta\x19@\x91\x90a0\xF4V[\x92PP\x81\x90UPPPPPV[C`\r\x81\x90UP_`\x0E_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x06\x81\x11\x15a\x19xWa\x19wa'\xF0V[[\x02\x17\x90UP\x7FQ\xFB \xDA\n\xAF\xAC\xEB\x18\xD9/\xF1\xA4v\x05\x9A\n\x8B\xBF\x16\xA0\xBF|8\xB9J\x98\xB3V\xAC\xE4W0`\rT`@Qa\x19\xB0\x92\x91\x90a,\x1EV[`@Q\x80\x91\x03\x90\xA1V[a\x19\xC2a \xD7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x19\xE0a\x0F:V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1A?Wa\x1A\x03a \xD7V[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1A6\x91\x90a*\x93V[`@Q\x80\x91\x03\x90\xFD[V[_`\t_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\t_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[_a\x1B\x11\x83_\x01\x83a!/V[_\x1C\x90P\x92\x91PPV[``_a\x1B)\x83_\x01a!VV[\x90P``\x81\x90P\x80\x92PPP\x91\x90PV[_a\x1BF\x82_\x01a!\xAFV[\x90P\x91\x90PV[_a\x1Bw\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x11\rV[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x10\x15\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91a\x1C\x02W`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1B\xF9\x92\x91\x90a4\xEFV[`@Q\x80\x91\x03\x90\xFD[PPPV[_a\x1C\x12\x83\x83a!\xBEV[\x90P\x92\x91PPV[a\x1CD\x7F`\x1A(\xE6\xAB\xEE\xA5+\x0B\xA1[T\xF6\xD1]x\x9F\xD3\xE8\xC0\x08\x0C\x04*\0\x9A\x99d\xF0\xE1\x8F\x1C\x84a\x0F\x8EV[\x83\x90a\x1C\x86W`@Q\x7F\\\x9Fq\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C}\x91\x90a*\x93V[`@Q\x80\x91\x03\x90\xFD[P_`\x02_\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x90P\x80`\x02\x01_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x843\x90\x91a\x1DYW`@Q\x7F\x08\xE5T\x95\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1DP\x92\x91\x90a5\x16V[`@Q\x80\x91\x03\x90\xFD[PP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x01T\x10a\x1D\xC1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1D\xB8\x90a5\xBDV[`@Q\x80\x91\x03\x90\xFD[`\x01\x81`\x02\x01_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x82\x82\x82_\x01\x83`\x01\x01T\x81T\x81\x10a\x1E3Wa\x1E2a1'V[[\x90_R` _ \x01\x91\x82a\x1EH\x92\x91\x90a5\xE5V[P`\x01\x81`\x01\x01_\x82\x82Ta\x1E]\x91\x90a0\xF4V[\x92PP\x81\x90UP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x01T\x10a \x15W_\x81`\x01\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xAEWa\x1E\xADa-\x88V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\xE1W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1E\xCCW\x90P[P\x90P__\x90P[\x82`\x01\x01T\x81\x10\x15a\x1F\xC4W\x82_\x01\x81\x81T\x81\x10a\x1F\nWa\x1F\ta1'V[[\x90_R` _ \x01\x80Ta\x1F\x1D\x90a-NV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1FI\x90a-NV[\x80\x15a\x1F\x94W\x80`\x1F\x10a\x1FkWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1F\x94V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1FwW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x82\x82\x81Q\x81\x10a\x1F\xACWa\x1F\xABa1'V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa\x1E\xE9V[P\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xD29^\x16\xBB\xE2\x8Eih\t\xE1\xF9\xB1R\x07v\xC9\xECY\x98\xFCrlT\xE8\x9Dg\xDD\x04\x1F\x9F\xF1\x82`@Qa \x0B\x91\x90a7\xAAV[`@Q\x80\x91\x03\x90\xA2P[PPPPV[_\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a \x8DWPa \x8C\x82a\"\x01V[[\x90P\x91\x90PV[__a \xA0\x84\x84a\"jV[\x90P\x80\x15a \xCDWa \xCB\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a#S\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[_3\x90P\x90V[a \xE8\x82\x82a\x0F\x8EV[a!+W\x80\x82`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a!\"\x92\x91\x90a7\xCAV[`@Q\x80\x91\x03\x90\xFD[PPV[_\x82_\x01\x82\x81T\x81\x10a!EWa!Da1'V[[\x90_R` _ \x01T\x90P\x92\x91PPV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a!\xA3W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a!\x8FW[PPPPP\x90P\x91\x90PV[_\x81_\x01\x80T\x90P\x90P\x91\x90PV[__a!\xCA\x84\x84a#\x80V[\x90P\x80\x15a!\xF7Wa!\xF5\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a$i\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[_\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[_a\"u\x83\x83a\x0F\x8EV[a#IW`\x01__\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\"\xE6a \xD7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa#MV[_\x90P[\x92\x91PPV[_a#x\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba$\x96V[\x90P\x92\x91PPV[_a#\x8B\x83\x83a\x0F\x8EV[\x15a$_W___\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa#\xFCa \xD7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa$cV[_\x90P[\x92\x91PPV[_a$\x8E\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba$\xFDV[\x90P\x92\x91PPV[_a$\xA1\x83\x83a%\xF9V[a$\xF3W\x82_\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x82_\x01\x80T\x90P\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP`\x01\x90Pa$\xF7V[_\x90P[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x14a%\xEEW_`\x01\x82a%*\x91\x90a,\xEEV[\x90P_`\x01\x86_\x01\x80T\x90Pa%@\x91\x90a,\xEEV[\x90P\x80\x82\x14a%\xA6W_\x86_\x01\x82\x81T\x81\x10a%_Wa%^a1'V[[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a%\x80Wa%\x7Fa1'V[[\x90_R` _ \x01\x81\x90UP\x83\x87`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UPP[\x85_\x01\x80T\x80a%\xB9Wa%\xB8a7\xF1V[[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa%\xF3V[_\x91PP[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x14\x15\x90P\x92\x91PPV[P\x80Ta&%\x90a-NV[_\x82U\x80`\x1F\x10a&6WPa&PV[`\x1F\x01` \x90\x04\x90_R` _ \x90a&O\x91\x90a&nV[[PV[P\x80T_\x82U\x90_R` _ \x90a&k\x91\x90a&\x8BV[PV[_[\x80\x82\x11\x15a&\x86W\x82\x81\x01_\x90U`\x01\x01a&pV[PP\x90V[_[\x80\x82\x11\x15a&\xABW\x82\x81\x01_a&\xA3\x91\x90a&\x19V[`\x01\x01a&\x8DV[PP\x90V[__\xFD[__\xFD[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a&\xEC\x81a&\xB8V[\x81\x14a&\xF6W__\xFD[PV[_\x815\x90Pa'\x07\x81a&\xE3V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a'\"Wa'!a&\xB0V[[_a'/\x84\x82\x85\x01a&\xF9V[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a'L\x81a'8V[\x82RPPV[_` \x82\x01\x90Pa'e_\x83\x01\x84a'CV[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a'\x94\x82a'kV[\x90P\x91\x90PV[a'\xA4\x81a'\x8AV[\x81\x14a'\xAEW__\xFD[PV[_\x815\x90Pa'\xBF\x81a'\x9BV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a'\xDAWa'\xD9a&\xB0V[[_a'\xE7\x84\x82\x85\x01a'\xB1V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x07\x81\x10a(.Wa(-a'\xF0V[[PV[_\x81\x90Pa(>\x82a(\x1DV[\x91\x90PV[_a(M\x82a(1V[\x90P\x91\x90PV[a(]\x81a(CV[\x82RPPV[_` \x82\x01\x90Pa(v_\x83\x01\x84a(TV[\x92\x91PPV[_\x81\x90P\x91\x90PV[a(\x8E\x81a(|V[\x82RPPV[_` \x82\x01\x90Pa(\xA7_\x83\x01\x84a(\x85V[\x92\x91PPV[a(\xB6\x81a(|V[\x81\x14a(\xC0W__\xFD[PV[_\x815\x90Pa(\xD1\x81a(\xADV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a(\xECWa(\xEBa&\xB0V[[_a(\xF9\x84\x82\x85\x01a(\xC3V[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a)\x14\x81a)\x02V[\x81\x14a)\x1EW__\xFD[PV[_\x815\x90Pa)/\x81a)\x0BV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a)JWa)Ia&\xB0V[[_a)W\x84\x82\x85\x01a)!V[\x91PP\x92\x91PPV[a)i\x81a)\x02V[\x82RPPV[_` \x82\x01\x90Pa)\x82_\x83\x01\x84a)`V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a)\x9EWa)\x9Da&\xB0V[[_a)\xAB\x85\x82\x86\x01a)!V[\x92PP` a)\xBC\x85\x82\x86\x01a'\xB1V[\x91PP\x92P\x92\x90PV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a)\xE7Wa)\xE6a)\xC6V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\x04Wa*\x03a)\xCAV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a* Wa*\x1Fa)\xCEV[[\x92P\x92\x90PV[___`@\x84\x86\x03\x12\x15a*>Wa*=a&\xB0V[[_\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*[Wa*Za&\xB4V[[a*g\x86\x82\x87\x01a)\xD2V[\x93P\x93PP` a*z\x86\x82\x87\x01a(\xC3V[\x91PP\x92P\x92P\x92V[a*\x8D\x81a'\x8AV[\x82RPPV[_` \x82\x01\x90Pa*\xA6_\x83\x01\x84a*\x84V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a*\xC2Wa*\xC1a&\xB0V[[_a*\xCF\x85\x82\x86\x01a)!V[\x92PP` a*\xE0\x85\x82\x86\x01a(\xC3V[\x91PP\x92P\x92\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a+\x1C\x81a'\x8AV[\x82RPPV[_a+-\x83\x83a+\x13V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a+O\x82a*\xEAV[a+Y\x81\x85a*\xF4V[\x93Pa+d\x83a+\x04V[\x80_[\x83\x81\x10\x15a+\x94W\x81Qa+{\x88\x82a+\"V[\x97Pa+\x86\x83a+9V[\x92PP`\x01\x81\x01\x90Pa+gV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra+\xB9\x81\x84a+EV[\x90P\x92\x91PPV[___`@\x84\x86\x03\x12\x15a+\xD8Wa+\xD7a&\xB0V[[_a+\xE5\x86\x82\x87\x01a'\xB1V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\x06Wa,\x05a&\xB4V[[a,\x12\x86\x82\x87\x01a)\xD2V[\x92P\x92PP\x92P\x92P\x92V[_`@\x82\x01\x90Pa,1_\x83\x01\x85a*\x84V[a,>` \x83\x01\x84a(\x85V[\x93\x92PPPV[_``\x82\x01\x90Pa,X_\x83\x01\x86a(\x85V[a,e` \x83\x01\x85a*\x84V[a,r`@\x83\x01\x84a*\x84V[\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a,\xB1\x82a(|V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a,\xE3Wa,\xE2a,zV[[`\x01\x82\x01\x90P\x91\x90PV[_a,\xF8\x82a(|V[\x91Pa-\x03\x83a(|V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a-\x1BWa-\x1Aa,zV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a-eW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a-xWa-wa-!V[[P\x91\x90PV[_\x81Q\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a.\x11\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a-\xD6V[a.\x1B\x86\x83a-\xD6V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a.Va.Qa.L\x84a(|V[a.3V[a(|V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a.o\x83a.<V[a.\x83a.{\x82a.]V[\x84\x84Ta-\xE2V[\x82UPPPPV[__\x90P\x90V[a.\x9Aa.\x8BV[a.\xA5\x81\x84\x84a.fV[PPPV[_[\x82\x81\x10\x15a.\xCBWa.\xC0_\x82\x84\x01a.\x92V[`\x01\x81\x01\x90Pa.\xACV[PPPV[`\x1F\x82\x11\x15a/\x1EW\x82\x82\x11\x15a/\x1DWa.\xEA\x81a-\xB5V[a.\xF3\x83a-\xC7V[a.\xFC\x85a-\xC7V[` \x86\x10\x15a/\tW_\x90P[\x80\x83\x01a/\x18\x82\x84\x03\x82a.\xAAV[PPPP[[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_a/>_\x19\x84`\x08\x02a/#V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_a/V\x83\x83a//V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a/o\x82a-~V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/\x88Wa/\x87a-\x88V[[a/\x92\x82Ta-NV[a/\x9D\x82\x82\x85a.\xD0V[_` \x90P`\x1F\x83\x11`\x01\x81\x14a/\xCEW_\x84\x15a/\xBCW\x82\x87\x01Q\x90P[a/\xC6\x85\x82a/KV[\x86UPa0-V[`\x1F\x19\x84\x16a/\xDC\x86a-\xB5V[_[\x82\x81\x10\x15a0\x03W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa/\xDEV[\x86\x83\x10\x15a0 W\x84\x89\x01Qa0\x1C`\x1F\x89\x16\x82a//V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x837_\x83\x83\x01RPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a0n\x83\x85a05V[\x93Pa0{\x83\x85\x84a0EV[a0\x84\x83a0SV[\x84\x01\x90P\x93\x92PPPV[_``\x82\x01\x90Pa0\xA2_\x83\x01\x87a*\x84V[\x81\x81\x03` \x83\x01Ra0\xB5\x81\x85\x87a0cV[\x90Pa0\xC4`@\x83\x01\x84a(\x85V[\x95\x94PPPPPV[_`@\x82\x01\x90Pa0\xE0_\x83\x01\x85a(TV[a0\xED` \x83\x01\x84a(TV[\x93\x92PPPV[_a0\xFE\x82a(|V[\x91Pa1\t\x83a(|V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a1!Wa1 a,zV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x81T\x90P\x91\x90PV[_\x81\x90P`\x01\x80`\x01\x03\x83\x01\x04\x90P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[a1\xC3\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83` \x03`\x08\x02a/#V[\x81T\x81\x16\x82UPPPV[a1\xD7\x81a-\xB5V[a1\xE2\x83\x82Ta/KV[\x80\x83U_\x82UPPPPV[` \x84\x10_\x81\x14a2IW`\x1F\x84\x11`\x01\x81\x14a2\x16Wa2\x0F\x86\x85a/KV[\x83Ua2CV[a2\x1F\x83a-\xB5V[a27`\x01a2-\x88a-\xC7V[\x03`\x01\x83\x01a.\xAAV[a2A\x87\x85a1\xCEV[P[Pa2\xA3V[a2R\x85a-\xC7V[a2[\x85a-\xC7V[a2d\x84a-\xB5V[\x82\x81\x01`\x1F\x89\x16\x80\x15a2\x7FWa2~\x81`\x01\x84\x03a1\x93V[[\x84\x84\x11\x15a2\x94Wa2\x93\x85\x85\x03\x83a.\xAAV[[`\x01\x8A`\x02\x02\x17\x87UPPPPP[PPPPPV[h\x01\0\0\0\0\0\0\0\0\x84\x11\x15a2\xC4Wa2\xC3a-\x88V[[` \x83\x10_\x81\x14a3\rW` \x85\x10_\x81\x14a2\xEBWa2\xE4\x86\x85a/KV[\x83Ua3\x07V[\x83`\xFF\x19\x16\x93P\x83a2\xFC\x84a-\xB5V[U`\x01\x86`\x02\x02\x01\x83U[Pa3\x17V[`\x01\x85`\x02\x02\x01\x82U[PPPPPV[\x80Ta3)\x81a-NV[\x80\x84\x11\x15a3>Wa3=\x84\x82\x84\x86a2\xAAV[[\x80\x84\x10\x15a3SWa3R\x84\x82\x84\x86a1\xEEV[[PPPPV[\x82\x81\x10\x15a3xWa3m_\x82\x84\x01a.\x92V[`\x01\x81\x01\x90Pa3YV[PPPV[a3\x87_\x82a3\x1EV[PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[_\x82\x14a3\xC6Wa3\xC5a3\x8AV[[a3\xCF\x81a3}V[PPV[_[\x82\x81\x10\x15a3\xF4Wa3\xE9_\x82\x84\x01a3\xB6V[`\x01\x81\x01\x90Pa3\xD5V[PPPV[\x81\x83\x10\x15a40Wa4\n\x82a1mV[a4\x13\x84a1mV[a4\x1C\x83a1\x81V[\x81\x81\x01a4+\x83\x85\x03\x82a3\xD3V[PPPP[PPPV[h\x01\0\0\0\0\0\0\0\0\x82\x11\x15a4OWa4Na-\x88V[[a4X\x81a1cV[\x82\x82Ua4f\x83\x82\x84a3\xF9V[PPPV[_\x81Q\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[a4\x89\x82\x82a/fV[PPV[a4\x96\x82a4kV[a4\xA0\x81\x83a45V[a4\xA9\x83a1TV[a4\xB2\x83a1\x81V[_[\x83\x81\x10\x15a4\xE7Wa4\xC5\x83a4uV[a4\xCF\x81\x84a4\x7FV[` \x84\x01\x93P`\x01\x83\x01\x92PP`\x01\x81\x01\x90Pa4\xB4V[PPPPPPV[_`@\x82\x01\x90Pa5\x02_\x83\x01\x85a(\x85V[a5\x0F` \x83\x01\x84a(\x85V[\x93\x92PPPV[_`@\x82\x01\x90Pa5)_\x83\x01\x85a*\x84V[a56` \x83\x01\x84a*\x84V[\x93\x92PPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FBUG: ALREADY RECEIVED SHARES FRO_\x82\x01R\x7FM N PARTIES, TOO MANY CLIENTS\0\0\0` \x82\x01RPV[_a5\xA7`=\x83a5=V[\x91Pa5\xB2\x82a5MV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra5\xD4\x81a5\x9BV[\x90P\x91\x90PV[_\x82\x90P\x92\x91PPV[a5\xEF\x83\x83a5\xDBV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\x08Wa6\x07a-\x88V[[a6\x12\x82Ta-NV[a6\x1D\x82\x82\x85a.\xD0V[_`\x1F\x83\x11`\x01\x81\x14a6JW_\x84\x15a68W\x82\x87\x015\x90P[a6B\x85\x82a/KV[\x86UPa6\xA9V[`\x1F\x19\x84\x16a6X\x86a-\xB5V[_[\x82\x81\x10\x15a6\x7FW\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa6ZV[\x86\x83\x10\x15a6\x9CW\x84\x89\x015a6\x98`\x1F\x89\x16\x82a//V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a6\xEA\x82a-~V[a6\xF4\x81\x85a6\xC2V[\x93Pa7\x04\x81\x85` \x86\x01a6\xD2V[a7\r\x81a0SV[\x84\x01\x91PP\x92\x91PPV[_a7#\x83\x83a6\xE0V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a7A\x82a4kV[a7K\x81\x85a6\xB2V[\x93P\x83` \x82\x02\x85\x01a7]\x85a1TV[\x80_[\x85\x81\x10\x15a7\x98W\x84\x84\x03\x89R\x81Qa7y\x85\x82a7\x18V[\x94Pa7\x84\x83a7+V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa7`V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra7\xC2\x81\x84a77V[\x90P\x92\x91PPV[_`@\x82\x01\x90Pa7\xDD_\x83\x01\x85a*\x84V[a7\xEA` \x83\x01\x84a)`V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \xFD\xA3\xA7\xD1\xBF\x92\xF7\xC8\xAB\xA8\x97\xCB\xEA\x15\xCB\xC5\xEB(\xB5\x81v\x90\xC6\xBCB?3\xAF+\x9E \x90dsolcC\0\x08!\x003",
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
    /**Custom error with signature `IndexOutOfBounds(address,uint256)` and selector `0x6867a170`.
```solidity
error IndexOutOfBounds(address client, uint256 index);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IndexOutOfBounds {
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
        impl ::core::convert::From<IndexOutOfBounds> for UnderlyingRustTuple<'_> {
            fn from(value: IndexOutOfBounds) -> Self {
                (value.client, value.index)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for IndexOutOfBounds {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    client: tuple.0,
                    index: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for IndexOutOfBounds {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IndexOutOfBounds(address,uint256)";
            const SELECTOR: [u8; 4] = [104u8, 103u8, 161u8, 112u8];
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
    /**Custom error with signature `OutputClientNotRegistered(address)` and selector `0x5c9f71ac`.
```solidity
error OutputClientNotRegistered(address client);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OutputClientNotRegistered {
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
        impl ::core::convert::From<OutputClientNotRegistered>
        for UnderlyingRustTuple<'_> {
            fn from(value: OutputClientNotRegistered) -> Self {
                (value.client,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OutputClientNotRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { client: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OutputClientNotRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OutputClientNotRegistered(address)";
            const SELECTOR: [u8; 4] = [92u8, 159u8, 113u8, 172u8];
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
    /**Custom error with signature `RoleChangeNotAllowed(uint8)` and selector `0x63018054`.
```solidity
error RoleChangeNotAllowed(StoffelCoordinator.Round current);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RoleChangeNotAllowed {
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
        impl ::core::convert::From<RoleChangeNotAllowed> for UnderlyingRustTuple<'_> {
            fn from(value: RoleChangeNotAllowed) -> Self {
                (value.current,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RoleChangeNotAllowed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { current: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for RoleChangeNotAllowed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "RoleChangeNotAllowed(uint8)";
            const SELECTOR: [u8; 4] = [99u8, 1u8, 128u8, 84u8];
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
    /**Event with signature `CoordinatorReset(address,uint256)` and selector `0x51fb20da0aafaceb18d92ff1a476059a0a8bbf16a0bf7c38b94a98b356ace457`.
```solidity
event CoordinatorReset(address coordinator, uint256 lastResetBlock);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CoordinatorReset {
        #[allow(missing_docs)]
        pub coordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub lastResetBlock: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for CoordinatorReset {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "CoordinatorReset(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                81u8, 251u8, 32u8, 218u8, 10u8, 175u8, 172u8, 235u8, 24u8, 217u8, 47u8,
                241u8, 164u8, 118u8, 5u8, 154u8, 10u8, 139u8, 191u8, 22u8, 160u8, 191u8,
                124u8, 56u8, 185u8, 74u8, 152u8, 179u8, 86u8, 172u8, 228u8, 87u8,
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
                    lastResetBlock: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.lastResetBlock),
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
        impl alloy_sol_types::private::IntoLogData for CoordinatorReset {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CoordinatorReset> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &CoordinatorReset) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `EnoughOutputShares(address,bytes[])` and selector `0xd2395e16bbe28e696809e1f9b1520776c9ec5998fc726c54e89d67dd041f9ff1`.
```solidity
event EnoughOutputShares(address indexed client, bytes[] shares);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EnoughOutputShares {
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
        impl alloy_sol_types::SolEvent for EnoughOutputShares {
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
            const SIGNATURE: &'static str = "EnoughOutputShares(address,bytes[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                210u8, 57u8, 94u8, 22u8, 187u8, 226u8, 142u8, 105u8, 104u8, 9u8, 225u8,
                249u8, 177u8, 82u8, 7u8, 118u8, 201u8, 236u8, 89u8, 152u8, 252u8, 114u8,
                108u8, 84u8, 232u8, 157u8, 103u8, 221u8, 4u8, 31u8, 159u8, 241u8,
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
        impl alloy_sol_types::private::IntoLogData for EnoughOutputShares {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EnoughOutputShares> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EnoughOutputShares) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `MaskedInputEvent(address,bytes,uint256)` and selector `0x56d03e5f1ebec3d4b4f9ded07e82c6bb6897c142cfbaf8dff8f9ef897ce4f75f`.
```solidity
event MaskedInputEvent(address client, bytes maskedInput, uint256 reservedIndex);
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
        pub maskedInput: alloy::sol_types::private::Bytes,
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
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "MaskedInputEvent(address,bytes,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                86u8, 208u8, 62u8, 95u8, 30u8, 190u8, 195u8, 212u8, 180u8, 249u8, 222u8,
                208u8, 126u8, 130u8, 198u8, 187u8, 104u8, 151u8, 193u8, 66u8, 207u8,
                186u8, 248u8, 223u8, 248u8, 249u8, 239u8, 137u8, 124u8, 228u8, 247u8,
                95u8,
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.maskedInput,
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
constructor(bytes32 stoffelProgramHash, uint256 t, address[] initialMpcNodes, uint256 nInputs, address[] outputClients, uint256 reconstructionThreshold);
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
        #[allow(missing_docs)]
        pub outputClients: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub reconstructionThreshold: alloy::sol_types::private::primitives::aliases::U256,
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
                        value.outputClients,
                        value.reconstructionThreshold,
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
                        outputClients: tuple.4,
                        reconstructionThreshold: tuple.5,
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.outputClients),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.reconstructionThreshold,
                    ),
                )
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
    /**Function with signature `INPUT_CLIENT_ROLE()` and selector `0x49f2ada0`.
```solidity
function INPUT_CLIENT_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct INPUT_CLIENT_ROLECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`INPUT_CLIENT_ROLE()`](INPUT_CLIENT_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct INPUT_CLIENT_ROLEReturn {
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
            impl ::core::convert::From<INPUT_CLIENT_ROLECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: INPUT_CLIENT_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for INPUT_CLIENT_ROLECall {
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
            impl ::core::convert::From<INPUT_CLIENT_ROLEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: INPUT_CLIENT_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for INPUT_CLIENT_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for INPUT_CLIENT_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "INPUT_CLIENT_ROLE()";
            const SELECTOR: [u8; 4] = [73u8, 242u8, 173u8, 160u8];
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
                        let r: INPUT_CLIENT_ROLEReturn = r.into();
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
                        let r: INPUT_CLIENT_ROLEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `OUTPUT_CLIENT_ROLE()` and selector `0x58df0d01`.
```solidity
function OUTPUT_CLIENT_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OUTPUT_CLIENT_ROLECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`OUTPUT_CLIENT_ROLE()`](OUTPUT_CLIENT_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OUTPUT_CLIENT_ROLEReturn {
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
            impl ::core::convert::From<OUTPUT_CLIENT_ROLECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: OUTPUT_CLIENT_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for OUTPUT_CLIENT_ROLECall {
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
            impl ::core::convert::From<OUTPUT_CLIENT_ROLEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: OUTPUT_CLIENT_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for OUTPUT_CLIENT_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for OUTPUT_CLIENT_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OUTPUT_CLIENT_ROLE()";
            const SELECTOR: [u8; 4] = [88u8, 223u8, 13u8, 1u8];
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
                        let r: OUTPUT_CLIENT_ROLEReturn = r.into();
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
                        let r: OUTPUT_CLIENT_ROLEReturn = r.into();
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
    /**Function with signature `lastResetBlock()` and selector `0x6b5e12ca`.
```solidity
function lastResetBlock() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastResetBlockCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`lastResetBlock()`](lastResetBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastResetBlockReturn {
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
            impl ::core::convert::From<lastResetBlockCall> for UnderlyingRustTuple<'_> {
                fn from(value: lastResetBlockCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lastResetBlockCall {
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
            impl ::core::convert::From<lastResetBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastResetBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastResetBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastResetBlockCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lastResetBlock()";
            const SELECTOR: [u8; 4] = [107u8, 94u8, 18u8, 202u8];
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
                        let r: lastResetBlockReturn = r.into();
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
                        let r: lastResetBlockReturn = r.into();
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
    /**Function with signature `resetCoordinator()` and selector `0x5cb86b74`.
```solidity
function resetCoordinator() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resetCoordinatorCall;
    ///Container type for the return parameters of the [`resetCoordinator()`](resetCoordinatorCall) function.
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
            impl ::core::convert::From<resetCoordinatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: resetCoordinatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for resetCoordinatorCall {
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
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = resetCoordinatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "resetCoordinator()";
            const SELECTOR: [u8; 4] = [92u8, 184u8, 107u8, 116u8];
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
    /**Function with signature `sendOutputShares(address,bytes)` and selector `0xede69216`.
```solidity
function sendOutputShares(address client, bytes memory shares) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct sendOutputSharesCall {
        #[allow(missing_docs)]
        pub client: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub shares: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`sendOutputShares(address,bytes)`](sendOutputSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct sendOutputSharesReturn {}
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
            impl ::core::convert::From<sendOutputSharesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: sendOutputSharesCall) -> Self {
                    (value.client, value.shares)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for sendOutputSharesCall {
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
            impl ::core::convert::From<sendOutputSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: sendOutputSharesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for sendOutputSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl sendOutputSharesReturn {
            fn _tokenize(
                &self,
            ) -> <sendOutputSharesCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for sendOutputSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = sendOutputSharesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "sendOutputShares(address,bytes)";
            const SELECTOR: [u8; 4] = [237u8, 230u8, 146u8, 22u8];
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
                sendOutputSharesReturn::_tokenize(ret)
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
    /**Function with signature `submitMaskedInput(bytes,uint256)` and selector `0x5648526c`.
```solidity
function submitMaskedInput(bytes memory maskedInput, uint256 reservedIndex) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submitMaskedInputCall {
        #[allow(missing_docs)]
        pub maskedInput: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub reservedIndex: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`submitMaskedInput(bytes,uint256)`](submitMaskedInputCall) function.
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
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Bytes,
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
                alloy::sol_types::sol_data::Bytes,
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
            const SIGNATURE: &'static str = "submitMaskedInput(bytes,uint256)";
            const SELECTOR: [u8; 4] = [86u8, 72u8, 82u8, 108u8];
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
                        &self.maskedInput,
                    ),
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
        DEFAULT_ADMIN_ROLE(DEFAULT_ADMIN_ROLECall),
        #[allow(missing_docs)]
        DESIGNATED_PARTY_ROLE(DESIGNATED_PARTY_ROLECall),
        #[allow(missing_docs)]
        INPUT_CLIENT_ROLE(INPUT_CLIENT_ROLECall),
        #[allow(missing_docs)]
        OUTPUT_CLIENT_ROLE(OUTPUT_CLIENT_ROLECall),
        #[allow(missing_docs)]
        PARTY_ROLE(PARTY_ROLECall),
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
        lastResetBlock(lastResetBlockCall),
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
        resetCoordinator(resetCoordinatorCall),
        #[allow(missing_docs)]
        revokeRole(revokeRoleCall),
        #[allow(missing_docs)]
        round(roundCall),
        #[allow(missing_docs)]
        sendOutputShares(sendOutputSharesCall),
        #[allow(missing_docs)]
        sendOutputs(sendOutputsCall),
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
            [19u8, 255u8, 109u8, 213u8],
            [20u8, 108u8, 165u8, 49u8],
            [23u8, 99u8, 69u8, 20u8],
            [28u8, 116u8, 83u8, 219u8],
            [33u8, 220u8, 123u8, 155u8],
            [35u8, 40u8, 189u8, 18u8],
            [36u8, 138u8, 156u8, 163u8],
            [47u8, 47u8, 241u8, 93u8],
            [48u8, 16u8, 76u8, 62u8],
            [51u8, 204u8, 154u8, 9u8],
            [54u8, 86u8, 138u8, 190u8],
            [73u8, 242u8, 173u8, 160u8],
            [75u8, 142u8, 100u8, 136u8],
            [75u8, 178u8, 120u8, 243u8],
            [86u8, 72u8, 82u8, 108u8],
            [88u8, 223u8, 13u8, 1u8],
            [92u8, 184u8, 107u8, 116u8],
            [107u8, 94u8, 18u8, 202u8],
            [113u8, 80u8, 24u8, 166u8],
            [127u8, 53u8, 181u8, 96u8],
            [141u8, 165u8, 203u8, 91u8],
            [144u8, 16u8, 208u8, 124u8],
            [145u8, 209u8, 72u8, 84u8],
            [162u8, 23u8, 253u8, 223u8],
            [163u8, 36u8, 106u8, 211u8],
            [187u8, 81u8, 254u8, 240u8],
            [192u8, 121u8, 244u8, 149u8],
            [202u8, 21u8, 200u8, 115u8],
            [203u8, 156u8, 76u8, 196u8],
            [213u8, 71u8, 116u8, 31u8],
            [216u8, 39u8, 13u8, 206u8],
            [237u8, 230u8, 146u8, 22u8],
            [242u8, 253u8, 227u8, 139u8],
            [252u8, 120u8, 178u8, 232u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(supportsInterface),
            ::core::stringify!(isDesignatedParty),
            ::core::stringify!(round),
            ::core::stringify!(creationBlock),
            ::core::stringify!(baseNonce),
            ::core::stringify!(reserveMaskIndex),
            ::core::stringify!(availableInputMasks),
            ::core::stringify!(getRoleAdmin),
            ::core::stringify!(grantRole),
            ::core::stringify!(PARTY_ROLE),
            ::core::stringify!(startMpc),
            ::core::stringify!(renounceRole),
            ::core::stringify!(INPUT_CLIENT_ROLE),
            ::core::stringify!(sendOutputs),
            ::core::stringify!(finalize),
            ::core::stringify!(submitMaskedInput),
            ::core::stringify!(OUTPUT_CLIENT_ROLE),
            ::core::stringify!(resetCoordinator),
            ::core::stringify!(lastResetBlock),
            ::core::stringify!(renounceOwnership),
            ::core::stringify!(DESIGNATED_PARTY_ROLE),
            ::core::stringify!(owner),
            ::core::stringify!(getRoleMember),
            ::core::stringify!(hasRole),
            ::core::stringify!(DEFAULT_ADMIN_ROLE),
            ::core::stringify!(getRoleMembers),
            ::core::stringify!(collectInputs),
            ::core::stringify!(reserveInputMasks),
            ::core::stringify!(getRoleMemberCount),
            ::core::stringify!(startPreprocessing),
            ::core::stringify!(revokeRole),
            ::core::stringify!(creationTime),
            ::core::stringify!(sendOutputShares),
            ::core::stringify!(transferOwnership),
            ::core::stringify!(isParty),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <supportsInterfaceCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isDesignatedPartyCall as alloy_sol_types::SolCall>::SIGNATURE,
            <roundCall as alloy_sol_types::SolCall>::SIGNATURE,
            <creationBlockCall as alloy_sol_types::SolCall>::SIGNATURE,
            <baseNonceCall as alloy_sol_types::SolCall>::SIGNATURE,
            <reserveMaskIndexCall as alloy_sol_types::SolCall>::SIGNATURE,
            <availableInputMasksCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRoleAdminCall as alloy_sol_types::SolCall>::SIGNATURE,
            <grantRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <PARTY_ROLECall as alloy_sol_types::SolCall>::SIGNATURE,
            <startMpcCall as alloy_sol_types::SolCall>::SIGNATURE,
            <renounceRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <INPUT_CLIENT_ROLECall as alloy_sol_types::SolCall>::SIGNATURE,
            <sendOutputsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <finalizeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <submitMaskedInputCall as alloy_sol_types::SolCall>::SIGNATURE,
            <OUTPUT_CLIENT_ROLECall as alloy_sol_types::SolCall>::SIGNATURE,
            <resetCoordinatorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <lastResetBlockCall as alloy_sol_types::SolCall>::SIGNATURE,
            <renounceOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <DESIGNATED_PARTY_ROLECall as alloy_sol_types::SolCall>::SIGNATURE,
            <ownerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRoleMemberCall as alloy_sol_types::SolCall>::SIGNATURE,
            <hasRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRoleMembersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <collectInputsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <reserveInputMasksCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRoleMemberCountCall as alloy_sol_types::SolCall>::SIGNATURE,
            <startPreprocessingCall as alloy_sol_types::SolCall>::SIGNATURE,
            <revokeRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <creationTimeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <sendOutputSharesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <transferOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
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
        const COUNT: usize = 35usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::DEFAULT_ADMIN_ROLE(_) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::DESIGNATED_PARTY_ROLE(_) => {
                    <DESIGNATED_PARTY_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::INPUT_CLIENT_ROLE(_) => {
                    <INPUT_CLIENT_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::OUTPUT_CLIENT_ROLE(_) => {
                    <OUTPUT_CLIENT_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::PARTY_ROLE(_) => {
                    <PARTY_ROLECall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::lastResetBlock(_) => {
                    <lastResetBlockCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::reserveMaskIndex(_) => {
                    <reserveMaskIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::resetCoordinator(_) => {
                    <resetCoordinatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::revokeRole(_) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::round(_) => <roundCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::sendOutputShares(_) => {
                    <sendOutputSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::sendOutputs(_) => {
                    <sendOutputsCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn INPUT_CLIENT_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <INPUT_CLIENT_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::INPUT_CLIENT_ROLE)
                    }
                    INPUT_CLIENT_ROLE
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
                    fn OUTPUT_CLIENT_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <OUTPUT_CLIENT_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::OUTPUT_CLIENT_ROLE)
                    }
                    OUTPUT_CLIENT_ROLE
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
                    fn lastResetBlock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <lastResetBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::lastResetBlock)
                    }
                    lastResetBlock
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
                    fn sendOutputShares(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <sendOutputSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::sendOutputShares)
                    }
                    sendOutputShares
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
                    fn INPUT_CLIENT_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <INPUT_CLIENT_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::INPUT_CLIENT_ROLE)
                    }
                    INPUT_CLIENT_ROLE
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
                    fn OUTPUT_CLIENT_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <OUTPUT_CLIENT_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::OUTPUT_CLIENT_ROLE)
                    }
                    OUTPUT_CLIENT_ROLE
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
                    fn lastResetBlock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <lastResetBlockCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::lastResetBlock)
                    }
                    lastResetBlock
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
                    fn sendOutputShares(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <sendOutputSharesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::sendOutputShares)
                    }
                    sendOutputShares
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
                Self::INPUT_CLIENT_ROLE(inner) => {
                    <INPUT_CLIENT_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OUTPUT_CLIENT_ROLE(inner) => {
                    <OUTPUT_CLIENT_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PARTY_ROLE(inner) => {
                    <PARTY_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::lastResetBlock(inner) => {
                    <lastResetBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::reserveMaskIndex(inner) => {
                    <reserveMaskIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::resetCoordinator(inner) => {
                    <resetCoordinatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::revokeRole(inner) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::round(inner) => {
                    <roundCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::sendOutputShares(inner) => {
                    <sendOutputSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::sendOutputs(inner) => {
                    <sendOutputsCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::INPUT_CLIENT_ROLE(inner) => {
                    <INPUT_CLIENT_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OUTPUT_CLIENT_ROLE(inner) => {
                    <OUTPUT_CLIENT_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::lastResetBlock(inner) => {
                    <lastResetBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::reserveMaskIndex(inner) => {
                    <reserveMaskIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::revokeRole(inner) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::round(inner) => {
                    <roundCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::sendOutputShares(inner) => {
                    <sendOutputSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::sendOutputs(inner) => {
                    <sendOutputsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        IndexAlreadyReserved(IndexAlreadyReserved),
        #[allow(missing_docs)]
        IndexNotReserved(IndexNotReserved),
        #[allow(missing_docs)]
        IndexOutOfBounds(IndexOutOfBounds),
        #[allow(missing_docs)]
        NoIndicesReserved(NoIndicesReserved),
        #[allow(missing_docs)]
        NotAnExistingParty(NotAnExistingParty),
        #[allow(missing_docs)]
        NotAtRound(NotAtRound),
        #[allow(missing_docs)]
        NotEnoughMPCParties(NotEnoughMPCParties),
        #[allow(missing_docs)]
        OutputClientNotRegistered(OutputClientNotRegistered),
        #[allow(missing_docs)]
        OwnableInvalidOwner(OwnableInvalidOwner),
        #[allow(missing_docs)]
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        #[allow(missing_docs)]
        RoleChangeNotAllowed(RoleChangeNotAllowed),
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
            [92u8, 159u8, 113u8, 172u8],
            [99u8, 1u8, 128u8, 84u8],
            [102u8, 151u8, 178u8, 50u8],
            [104u8, 103u8, 161u8, 112u8],
            [111u8, 175u8, 159u8, 5u8],
            [160u8, 184u8, 199u8, 8u8],
            [171u8, 220u8, 224u8, 106u8],
            [191u8, 162u8, 23u8, 216u8],
            [195u8, 21u8, 160u8, 245u8],
            [226u8, 81u8, 125u8, 63u8],
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
            ::core::stringify!(OutputClientNotRegistered),
            ::core::stringify!(RoleChangeNotAllowed),
            ::core::stringify!(AccessControlBadConfirmation),
            ::core::stringify!(IndexOutOfBounds),
            ::core::stringify!(NoIndicesReserved),
            ::core::stringify!(IndexAlreadyReserved),
            ::core::stringify!(NotAnExistingParty),
            ::core::stringify!(NotAtRound),
            ::core::stringify!(ClientAlreadyReservedIndex),
            ::core::stringify!(AccessControlUnauthorizedAccount),
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
            <OutputClientNotRegistered as alloy_sol_types::SolError>::SIGNATURE,
            <RoleChangeNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <AccessControlBadConfirmation as alloy_sol_types::SolError>::SIGNATURE,
            <IndexOutOfBounds as alloy_sol_types::SolError>::SIGNATURE,
            <NoIndicesReserved as alloy_sol_types::SolError>::SIGNATURE,
            <IndexAlreadyReserved as alloy_sol_types::SolError>::SIGNATURE,
            <NotAnExistingParty as alloy_sol_types::SolError>::SIGNATURE,
            <NotAtRound as alloy_sol_types::SolError>::SIGNATURE,
            <ClientAlreadyReservedIndex as alloy_sol_types::SolError>::SIGNATURE,
            <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::SIGNATURE,
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
        const COUNT: usize = 17usize;
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
                Self::IndexAlreadyReserved(_) => {
                    <IndexAlreadyReserved as alloy_sol_types::SolError>::SELECTOR
                }
                Self::IndexNotReserved(_) => {
                    <IndexNotReserved as alloy_sol_types::SolError>::SELECTOR
                }
                Self::IndexOutOfBounds(_) => {
                    <IndexOutOfBounds as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoIndicesReserved(_) => {
                    <NoIndicesReserved as alloy_sol_types::SolError>::SELECTOR
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
                Self::OutputClientNotRegistered(_) => {
                    <OutputClientNotRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OwnableInvalidOwner(_) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OwnableUnauthorizedAccount(_) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::RoleChangeNotAllowed(_) => {
                    <RoleChangeNotAllowed as alloy_sol_types::SolError>::SELECTOR
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
                    fn OutputClientNotRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <OutputClientNotRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorErrors::OutputClientNotRegistered)
                    }
                    OutputClientNotRegistered
                },
                {
                    fn RoleChangeNotAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <RoleChangeNotAllowed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorErrors::RoleChangeNotAllowed)
                    }
                    RoleChangeNotAllowed
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
                    fn IndexOutOfBounds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <IndexOutOfBounds as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorErrors::IndexOutOfBounds)
                    }
                    IndexOutOfBounds
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
                    fn OutputClientNotRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <OutputClientNotRegistered as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::OutputClientNotRegistered)
                    }
                    OutputClientNotRegistered
                },
                {
                    fn RoleChangeNotAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <RoleChangeNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::RoleChangeNotAllowed)
                    }
                    RoleChangeNotAllowed
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
                    fn IndexOutOfBounds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorErrors> {
                        <IndexOutOfBounds as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorErrors::IndexOutOfBounds)
                    }
                    IndexOutOfBounds
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
                Self::IndexOutOfBounds(inner) => {
                    <IndexOutOfBounds as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NoIndicesReserved(inner) => {
                    <NoIndicesReserved as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
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
                Self::OutputClientNotRegistered(inner) => {
                    <OutputClientNotRegistered as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::RoleChangeNotAllowed(inner) => {
                    <RoleChangeNotAllowed as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::IndexOutOfBounds(inner) => {
                    <IndexOutOfBounds as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::OutputClientNotRegistered(inner) => {
                    <OutputClientNotRegistered as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::RoleChangeNotAllowed(inner) => {
                    <RoleChangeNotAllowed as alloy_sol_types::SolError>::abi_encode_raw(
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
        CoordinatorInitialized(CoordinatorInitialized),
        #[allow(missing_docs)]
        CoordinatorReset(CoordinatorReset),
        #[allow(missing_docs)]
        EnoughOutputShares(EnoughOutputShares),
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
                81u8, 251u8, 32u8, 218u8, 10u8, 175u8, 172u8, 235u8, 24u8, 217u8, 47u8,
                241u8, 164u8, 118u8, 5u8, 154u8, 10u8, 139u8, 191u8, 22u8, 160u8, 191u8,
                124u8, 56u8, 185u8, 74u8, 152u8, 179u8, 86u8, 172u8, 228u8, 87u8,
            ],
            [
                86u8, 208u8, 62u8, 95u8, 30u8, 190u8, 195u8, 212u8, 180u8, 249u8, 222u8,
                208u8, 126u8, 130u8, 198u8, 187u8, 104u8, 151u8, 193u8, 66u8, 207u8,
                186u8, 248u8, 223u8, 248u8, 249u8, 239u8, 137u8, 124u8, 228u8, 247u8,
                95u8,
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
                210u8, 57u8, 94u8, 22u8, 187u8, 226u8, 142u8, 105u8, 104u8, 9u8, 225u8,
                249u8, 177u8, 82u8, 7u8, 118u8, 201u8, 236u8, 89u8, 152u8, 252u8, 114u8,
                108u8, 84u8, 232u8, 157u8, 103u8, 221u8, 4u8, 31u8, 159u8, 241u8,
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
            ::core::stringify!(ExecutionDone),
            ::core::stringify!(RoleGranted),
            ::core::stringify!(OutputSendingStarted),
            ::core::stringify!(CoordinatorReset),
            ::core::stringify!(MaskedInputEvent),
            ::core::stringify!(InputCollectionStarted),
            ::core::stringify!(InputMaskReservationStarted),
            ::core::stringify!(OwnershipTransferred),
            ::core::stringify!(InitializeStoffelAccessControl),
            ::core::stringify!(ReservedInputEvent),
            ::core::stringify!(PreprocessingStarted),
            ::core::stringify!(RoleAdminChanged),
            ::core::stringify!(EnoughOutputShares),
            ::core::stringify!(CoordinatorInitialized),
            ::core::stringify!(RoleRevoked),
            ::core::stringify!(IndexBufferEvent),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <MPCStarted as alloy_sol_types::SolEvent>::SIGNATURE,
            <ExecutionDone as alloy_sol_types::SolEvent>::SIGNATURE,
            <RoleGranted as alloy_sol_types::SolEvent>::SIGNATURE,
            <OutputSendingStarted as alloy_sol_types::SolEvent>::SIGNATURE,
            <CoordinatorReset as alloy_sol_types::SolEvent>::SIGNATURE,
            <MaskedInputEvent as alloy_sol_types::SolEvent>::SIGNATURE,
            <InputCollectionStarted as alloy_sol_types::SolEvent>::SIGNATURE,
            <InputMaskReservationStarted as alloy_sol_types::SolEvent>::SIGNATURE,
            <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE,
            <InitializeStoffelAccessControl as alloy_sol_types::SolEvent>::SIGNATURE,
            <ReservedInputEvent as alloy_sol_types::SolEvent>::SIGNATURE,
            <PreprocessingStarted as alloy_sol_types::SolEvent>::SIGNATURE,
            <RoleAdminChanged as alloy_sol_types::SolEvent>::SIGNATURE,
            <EnoughOutputShares as alloy_sol_types::SolEvent>::SIGNATURE,
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
                    <CoordinatorInitialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <CoordinatorInitialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::CoordinatorInitialized)
                }
                Some(<CoordinatorReset as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <CoordinatorReset as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::CoordinatorReset)
                }
                Some(
                    <EnoughOutputShares as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <EnoughOutputShares as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::EnoughOutputShares)
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
                Self::CoordinatorInitialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::CoordinatorReset(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EnoughOutputShares(inner) => {
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
                Self::CoordinatorInitialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::CoordinatorReset(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EnoughOutputShares(inner) => {
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
        outputClients: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        reconstructionThreshold: alloy::sol_types::private::primitives::aliases::U256,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<FakeCoordinatorInstance<P, N>>,
    > {
        FakeCoordinatorInstance::<
            P,
            N,
        >::deploy(
            __provider,
            stoffelProgramHash,
            t,
            initialMpcNodes,
            nInputs,
            outputClients,
            reconstructionThreshold,
        )
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
        outputClients: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        reconstructionThreshold: alloy::sol_types::private::primitives::aliases::U256,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        FakeCoordinatorInstance::<
            P,
            N,
        >::deploy_builder(
            __provider,
            stoffelProgramHash,
            t,
            initialMpcNodes,
            nInputs,
            outputClients,
            reconstructionThreshold,
        )
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
            outputClients: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            reconstructionThreshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::Result<FakeCoordinatorInstance<P, N>> {
            let call_builder = Self::deploy_builder(
                __provider,
                stoffelProgramHash,
                t,
                initialMpcNodes,
                nInputs,
                outputClients,
                reconstructionThreshold,
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
            outputClients: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            reconstructionThreshold: alloy::sol_types::private::primitives::aliases::U256,
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
                            outputClients,
                            reconstructionThreshold,
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
        ///Creates a new call builder for the [`INPUT_CLIENT_ROLE`] function.
        pub fn INPUT_CLIENT_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, INPUT_CLIENT_ROLECall, N> {
            self.call_builder(&INPUT_CLIENT_ROLECall)
        }
        ///Creates a new call builder for the [`OUTPUT_CLIENT_ROLE`] function.
        pub fn OUTPUT_CLIENT_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, OUTPUT_CLIENT_ROLECall, N> {
            self.call_builder(&OUTPUT_CLIENT_ROLECall)
        }
        ///Creates a new call builder for the [`PARTY_ROLE`] function.
        pub fn PARTY_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, PARTY_ROLECall, N> {
            self.call_builder(&PARTY_ROLECall)
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
        ///Creates a new call builder for the [`lastResetBlock`] function.
        pub fn lastResetBlock(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, lastResetBlockCall, N> {
            self.call_builder(&lastResetBlockCall)
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
        ///Creates a new call builder for the [`resetCoordinator`] function.
        pub fn resetCoordinator(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, resetCoordinatorCall, N> {
            self.call_builder(&resetCoordinatorCall)
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
        ///Creates a new call builder for the [`sendOutputShares`] function.
        pub fn sendOutputShares(
            &self,
            client: alloy::sol_types::private::Address,
            shares: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, sendOutputSharesCall, N> {
            self.call_builder(
                &sendOutputSharesCall {
                    client,
                    shares,
                },
            )
        }
        ///Creates a new call builder for the [`sendOutputs`] function.
        pub fn sendOutputs(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, sendOutputsCall, N> {
            self.call_builder(&sendOutputsCall)
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
            maskedInput: alloy::sol_types::private::Bytes,
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
        ///Creates a new event filter for the [`CoordinatorInitialized`] event.
        pub fn CoordinatorInitialized_filter(
            &self,
        ) -> alloy_contract::Event<&P, CoordinatorInitialized, N> {
            self.event_filter::<CoordinatorInitialized>()
        }
        ///Creates a new event filter for the [`CoordinatorReset`] event.
        pub fn CoordinatorReset_filter(
            &self,
        ) -> alloy_contract::Event<&P, CoordinatorReset, N> {
            self.event_filter::<CoordinatorReset>()
        }
        ///Creates a new event filter for the [`EnoughOutputShares`] event.
        pub fn EnoughOutputShares_filter(
            &self,
        ) -> alloy_contract::Event<&P, EnoughOutputShares, N> {
            self.event_filter::<EnoughOutputShares>()
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

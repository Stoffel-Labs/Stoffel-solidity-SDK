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
    error ECDSAInvalidSignature();
    error ECDSAInvalidSignatureLength(uint256 length);
    error ECDSAInvalidSignatureS(bytes32 s);
    error IndexNotReserved(address client, uint256 index);
    error IndicesAlreadyReserved(address client);
    error NoIndicesReserved(address client);
    error NotAnExistingParty(address account);
    error NotAtRound(StoffelCoordinator.Round required, StoffelCoordinator.Round current);
    error NotEnoughIndices(uint256 requested, uint256 available);
    error NotEnoughMPCParties(uint256 current, uint256 required);
    error OwnableInvalidOwner(address owner);
    error OwnableUnauthorizedAccount(address account);

    event ClientAuthenticated(address indexed client, bool success);
    event CoordinatorInitialized(address coordinator, uint256 timeofInitialization, uint256 creationBlock, address designatedParty);
    event ExecutionDone(address executor, uint256 timeOfExecution);
    event IndexBufferEvent(uint256 totalIndices, address designatedParty);
    event InitializeStoffelAccessControl(uint256 nParties, uint256 t, address initializer);
    event InputCollectionStarted(address executor, uint256 timeOfExecution);
    event InputMaskReservationStarted(address executor, uint256 timeOfExecution);
    event MPCStarted(address executor, uint256 timeOfExecution);
    event MaskedInputEvent(address client, uint256 maskedInput, uint256 reservedIndex);
    event OutputSendingStarted(address executor, uint256 timeOfExecution);
    event OutputsPublished(bytes32 stoffelProgramHash, uint256 timeOfExecution);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event PreprocessingStarted(address designatedParty, uint256 timeOfExecution);
    event ReservedInputEvent(address client, uint256[] reservedIndices);
    event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole);
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);

    constructor(bytes32 stoffelProgramHash, uint256 n, uint256 t, address designatedParty, address[] initialMPCNodes, uint256 nInputs);

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
    function resetAccessControl(uint256 t, address[] memory initialMPCNodes) external;
    function resetCoordinator(bytes32 stoffelProgramHash, uint256 t, address[] memory initialMPCNodes, uint256 nInputs) external;
    function resetInputManager(uint256 nIndicesToReserve, uint256 t) external;
    function revokeRole(bytes32 role, address account) external;
    function round() external view returns (StoffelCoordinator.Round);
    function sendOutputs() external;
    function setPublicOutputs(bytes memory publicOutputs) external;
    function shareReceived(address from, address to) external;
    function startMPC() external;
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
        "name": "n",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "t",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "designatedParty",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "initialMPCNodes",
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
        "name": "initialMPCNodes",
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
        "name": "initialMPCNodes",
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
    "name": "setPublicOutputs",
    "inputs": [
      {
        "name": "publicOutputs",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "shareReceived",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "startMPC",
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
    "name": "OutputsPublished",
    "inputs": [
      {
        "name": "stoffelProgramHash",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
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
    ///0x608060405234801561000f575f5ffd5b50604051614d36380380614d3683398181016040528101906100319190610fa8565b85848383338184858561004a828261016d60201b60201c565b50505f600881905550610063828261046360201b60201c565b50505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16036100d5575f6040517f1e4fbdf70000000000000000000000000000000000000000000000000000000081526004016100cc919061105c565b60405180910390fd5b6100e48161063960201b60201c565b5061010f84835f815181106100fc576100fb611075565b5b60200260200101516106fc60201b60201c565b5050505042600d8190555043600e819055507fdef1f08eb655f4a75f60bd6fd7e971112004abd846a612e46dab270770d24ca530600d54600e543360405161015a94939291906110b1565b60405180910390a15050505050506112a5565b5f600183600361017d9190611121565b6101879190611162565b905080825110156101d2578151816040517f3a2362680000000000000000000000000000000000000000000000000000000081526004016101c9929190611195565b60405180910390fd5b826002819055505f6102097f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61077f60201b60201c565b90505f61023b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6107a760201b60201c565b90505f5f90505b818110156102a2576102947f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e84838151811061028157610280611075565b5b60200260200101516107ce60201b60201c565b508080600101915050610242565b505f6102d37fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46961077f60201b60201c565b90505f6103057fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696107a760201b60201c565b90505f5f90505b8181101561036c5761035e7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46984838151811061034b5761034a611075565b5b60200260200101516107ce60201b60201c565b50808060010191505061030c565b505f5f90505b86518110156103d3576103c57fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698883815181106103b2576103b1611075565b5b60200260200101516107e760201b60201c565b508080600101915050610372565b5061041e7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e875f8151811061040b5761040a611075565b5b60200260200101516107e760201b60201c565b507fa2df7830e0bedef7b1116bf547b467b16b50b3bd23146c9e099878d14e89301a858833604051610452939291906111bc565b60405180910390a150505050505050565b60055460085f8282546104769190611162565b92505081905550816005819055505f6006819055505f600781905550806009819055505f5f90505b6005548110156105f957600a5f60035f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f5f82015f9055600182015f9055505060045f60035f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f61059d9190610d7b565b5f60035f8381526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550808060010191505061049e565b507ff7f0872382dff5e698b284e12084e4e7894f830216dd80cb4e909b593a58f9356005543360405161062d9291906111f1565b60405180910390a15050565b5f600b5f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905081600b5f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a35050565b81600c8190555042600d8190555043600e819055505f600f5f6101000a81548160ff0219169083600581111561073557610734611218565b5b02179055507fdef1f08eb655f4a75f60bd6fd7e971112004abd846a612e46dab270770d24ca530600d54600e548460405161077394939291906110b1565b60405180910390a15050565b60606107a060015f8481526020019081526020015f2061080060201b60201c565b9050919050565b5f6107c760015f8481526020019081526020015f2061082560201b60201c565b9050919050565b5f6107df838361083e60201b60201c565b905092915050565b5f6107f8838361088760201b60201c565b905092915050565b60605f610814835f016108d060201b60201c565b905060608190508092505050919050565b5f610837825f0161092960201b60201c565b9050919050565b5f5f610850848461093860201b60201c565b9050801561087d5761087b8360015f8781526020019081526020015f20610a2d60201b90919060201c565b505b8091505092915050565b5f5f6108998484610a6060201b60201c565b905080156108c6576108c48360015f8781526020019081526020015f20610b5560201b90919060201c565b505b8091505092915050565b6060815f0180548060200260200160405190810160405280929190818152602001828054801561091d57602002820191905f5260205f20905b815481526020019060010190808311610909575b50505050509050919050565b5f815f01805490509050919050565b5f6109498383610b8860201b60201c565b15610a23575f5f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055506109c0610beb60201b60201c565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b60405160405180910390a460019050610a27565b5f90505b92915050565b5f610a58835f018373ffffffffffffffffffffffffffffffffffffffff165f1b610bf260201b60201c565b905092915050565b5f610a718383610b8860201b60201c565b610b4b5760015f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550610ae8610beb60201b60201c565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a460019050610b4f565b5f90505b92915050565b5f610b80835f018373ffffffffffffffffffffffffffffffffffffffff165f1b610cee60201b60201c565b905092915050565b5f5f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f33905090565b5f5f836001015f8481526020019081526020015f205490505f8114610ce3575f600182610c1f9190611245565b90505f6001865f0180549050610c359190611245565b9050808214610c9b575f865f018281548110610c5457610c53611075565b5b905f5260205f200154905080875f018481548110610c7557610c74611075565b5b905f5260205f20018190555083876001015f8381526020019081526020015f2081905550505b855f01805480610cae57610cad611278565b5b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f905560019350505050610ce8565b5f9150505b92915050565b5f610cff8383610d5b60201b60201c565b610d5157825f0182908060018154018082558091505060019003905f5260205f20015f9091909190915055825f0180549050836001015f8481526020019081526020015f208190555060019050610d55565b5f90505b92915050565b5f5f836001015f8481526020019081526020015f20541415905092915050565b505f81556001015f9055565b5f604051905090565b5f5ffd5b5f5ffd5b5f819050919050565b610daa81610d98565b8114610db4575f5ffd5b50565b5f81519050610dc581610da1565b92915050565b5f819050919050565b610ddd81610dcb565b8114610de7575f5ffd5b50565b5f81519050610df881610dd4565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610e2782610dfe565b9050919050565b610e3781610e1d565b8114610e41575f5ffd5b50565b5f81519050610e5281610e2e565b92915050565b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610ea282610e5c565b810181811067ffffffffffffffff82111715610ec157610ec0610e6c565b5b80604052505050565b5f610ed3610d87565b9050610edf8282610e99565b919050565b5f67ffffffffffffffff821115610efe57610efd610e6c565b5b602082029050602081019050919050565b5f5ffd5b5f610f25610f2084610ee4565b610eca565b90508083825260208201905060208402830185811115610f4857610f47610f0f565b5b835b81811015610f715780610f5d8882610e44565b845260208401935050602081019050610f4a565b5050509392505050565b5f82601f830112610f8f57610f8e610e58565b5b8151610f9f848260208601610f13565b91505092915050565b5f5f5f5f5f5f60c08789031215610fc257610fc1610d90565b5b5f610fcf89828a01610db7565b9650506020610fe089828a01610dea565b9550506040610ff189828a01610dea565b945050606061100289828a01610e44565b935050608087015167ffffffffffffffff81111561102357611022610d94565b5b61102f89828a01610f7b565b92505060a061104089828a01610dea565b9150509295509295509295565b61105681610e1d565b82525050565b5f60208201905061106f5f83018461104d565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b6110ab81610dcb565b82525050565b5f6080820190506110c45f83018761104d565b6110d160208301866110a2565b6110de60408301856110a2565b6110eb606083018461104d565b95945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61112b82610dcb565b915061113683610dcb565b925082820261114481610dcb565b9150828204841483151761115b5761115a6110f4565b5b5092915050565b5f61116c82610dcb565b915061117783610dcb565b925082820190508082111561118f5761118e6110f4565b5b92915050565b5f6040820190506111a85f8301856110a2565b6111b560208301846110a2565b9392505050565b5f6060820190506111cf5f8301866110a2565b6111dc60208301856110a2565b6111e9604083018461104d565b949350505050565b5f6040820190506112045f8301856110a2565b611211602083018461104d565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b5f61124f82610dcb565b915061125a83610dcb565b9250828203905081811115611272576112716110f4565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffd5b613a84806112b25f395ff3fe608060405234801561000f575f5ffd5b506004361061021a575f3560e01c8063715018a611610123578063c079f495116100ab578063d8270dce1161007a578063d8270dce146105a0578063ebae35e7146105be578063f2fde38b146105ee578063f6603c611461060a578063fc78b2e8146106265761021a565b8063c079f49514610540578063ca15c8731461054a578063cb9c4cc41461057a578063d547741f146105845761021a565b806391d14854116100f257806391d148541461049c578063a217fddf146104cc578063a3246ad3146104ea578063af206f281461051a578063bb51fef0146105365761021a565b8063715018a6146104265780637f35b560146104305780638da5cb5b1461044e5780639010d07c1461046c5761021a565b80632f2ff15d116101a65780634925e62a116101755780634925e62a146103d05780634b8e6488146103ec5780634bb278f3146103f6578063595f806e146104005780636b7447281461040a5761021a565b80632f2ff15d1461035e57806330104c3e1461037a57806336568abe146103985780633b4338d1146103b45761021a565b806317634514116101ed57806317634514146102b85780631c7453db146102d65780631ee4ee0f146102f45780632328bd1214610310578063248a9ca31461032e5761021a565b806301ffc9a71461021e5780630bda81cf1461024e57806313ff6dd51461026a578063146ca5311461029a575b5f5ffd5b61023860048036038101906102339190612ae6565b610656565b6040516102459190612b2b565b60405180910390f35b61026860048036038101906102639190612b77565b6106cf565b005b610284600480360381019061027f9190612c0f565b61082e565b6040516102919190612b2b565b60405180910390f35b6102a26108aa565b6040516102af9190612cad565b60405180910390f35b6102c06108bc565b6040516102cd9190612cd5565b60405180910390f35b6102de6108c2565b6040516102eb9190612cd5565b60405180910390f35b61030e60048036038101906103099190612d4f565b6108c8565b005b610318610cbe565b6040516103259190612cd5565b60405180910390f35b61034860048036038101906103439190612ddf565b610cd4565b6040516103559190612e19565b60405180910390f35b61037860048036038101906103739190612e32565b610cf0565b005b610382610d2a565b60405161038f9190612e19565b60405180910390f35b6103b260048036038101906103ad9190612e32565b610d4e565b005b6103ce60048036038101906103c99190612b77565b610e15565b005b6103ea60048036038101906103e59190612e70565b610e4e565b005b6103f4610f10565b005b6103fe610f8a565b005b610408611007565b005b610424600480360381019061041f9190612eae565b611081565b005b61042e6110c4565b005b6104386110d7565b6040516104459190612e19565b60405180910390f35b6104566110fb565b6040516104639190612f08565b60405180910390f35b61048660048036038101906104819190612f21565b611123565b6040516104939190612f08565b60405180910390f35b6104b660048036038101906104b19190612e32565b61114f565b6040516104c39190612b2b565b60405180910390f35b6104d46111b2565b6040516104e19190612e19565b60405180910390f35b61050460048036038101906104ff9190612ddf565b6111b8565b6040516105119190613016565b60405180910390f35b610534600480360381019061052f919061317e565b6111da565b005b61053e611213565b005b61054861128d565b005b610564600480360381019061055f9190612ddf565b611307565b6040516105719190612cd5565b60405180910390f35b610582611328565b005b61059e60048036038101906105999190612e32565b6113a1565b005b6105a8611494565b6040516105b59190612cd5565b60405180910390f35b6105d860048036038101906105d391906131d8565b61149a565b6040516105e591906132ba565b60405180910390f35b61060860048036038101906106039190612c0f565b61175e565b005b610624600480360381019061061f91906132da565b6117e2565b005b610640600480360381019061063b9190612c0f565b61184b565b60405161064d9190612b2b565b60405180910390f35b5f7f07effe0d000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191614806106c857506106c78261187d565b5b9050919050565b3373ffffffffffffffffffffffffffffffffffffffff1660035f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16146107715733816040517fffabbae700000000000000000000000000000000000000000000000000000000815260040161076892919061335a565b60405180910390fd5b604051806040016040528082815260200183815250600a5f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f820151815f0155602082015181600101559050507fb89addd937f44f902c849596641837cd7af2fcecef22d2a7866fdc1ad9c0ae2e33838360405161080b93929190613381565b60405180910390a160075f815480929190610825906133e3565b91905055505050565b5f6108388261184b565b61087957816040517fabdce06a0000000000000000000000000000000000000000000000000000000081526004016108709190612f08565b60405180910390fd5b6108a37f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e8361114f565b9050919050565b600f5f9054906101000a900460ff1681565b600e5481565b60085481565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696108f2816118f6565b5f60055490505f5f90505b60055481101561097f578573ffffffffffffffffffffffffffffffffffffffff1660035f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16036109725780915061097f565b80806001019150506108fd565b5060055481036109f65760035f5f81526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040517f6faf9f050000000000000000000000000000000000000000000000000000000081526004016109ed9190612f08565b60405180910390fd5b5f81600854610a05919061342a565b90505f610a3782604051602001610a1c9190612cd5565b6040516020818303038152906040528051906020012061190a565b90505f610a878288888080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505061193d565b905060045f8973ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f60028110610ad857610ad761345d565b5b015f8154610ae5906133e3565b919050819055508773ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603610b845760045f8973ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20600160028110610b6f57610b6e61345d565b5b015f8154610b7c906133e3565b919050819055505b60016009546002610b95919061348a565b610b9f919061342a565b60045f8a73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f60028110610bee57610bed61345d565b5b015403610cb4575f5f90506001600954610c08919061342a565b60045f8b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20600160028110610c5857610c5761345d565b5b015410610c6457600190505b8873ffffffffffffffffffffffffffffffffffffffff167f407009200520f9f10584813c0b95441ab320f6b08d97ebdaaf1e824eedd9d7c382604051610caa9190612b2b565b60405180910390a2505b5050505050505050565b5f600654600554610ccf91906134cb565b905090565b5f5f5f8381526020019081526020015f20600101549050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610d1a816118f6565b610d248383611967565b50505050565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46981565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46982148015610d835750610d82828261114f565b5b15610e07575f610d9283611307565b90505f60016002546003610da6919061348a565b610db0919061342a565b9050808203610e0457600181610dc691906134cb565b816040517f3a236268000000000000000000000000000000000000000000000000000000008152600401610dfb9291906134fe565b60405180910390fd5b50505b610e11828261197a565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610e3f816118f6565b610e4983836119f5565b505050565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469610e78816118f6565b600160106001015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550505050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610f3a816118f6565b6004610f4581611bcb565b7f301f8a3701f5b260197382dd7301078542144fe8fddd18083d6f6e09e4958a593342604051610f7692919061335a565b60405180910390a1610f86611c54565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610fb4816118f6565b6005610fbf81611bcb565b5f7f24a873651d26fb5a462cb37a91071cdd4d09abeebfe0ed144329bed1cc359d033342604051610ff192919061335a565b60405180910390a161100281611cbd565b505050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e611031816118f6565b600361103c81611bcb565b7f20f55ed0c92f2bb1c8825488e1e3c98463d024b2a42dbd24838c3f75260f43e9334260405161106d92919061335a565b60405180910390a161107d611c54565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6110ab816118f6565b828260105f0191826110be92919061373d565b50505050565b6110cc611ce9565b6110d55f611d70565b565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e81565b5f600b5f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b5f6111478260015f8681526020019081526020015f20611e3390919063ffffffff16565b905092915050565b5f5f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f5f1b81565b60606111d360015f8481526020019081526020015f20611e4a565b9050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e611204816118f6565b61120e8383611e69565b505050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61123d816118f6565b600261124881611bcb565b7f60edf9bdc7c4ea007cae1a9bbd03e41e5bfccd7231a6ec383c2edd7800f0d20c334260405161127992919061335a565b60405180910390a1611289611c54565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6112b7816118f6565b60016112c281611bcb565b7f67c4489f674d03c7d19a9e36735188de7c65e8d1e99eb3a2fd258a769eb14fff33426040516112f392919061335a565b60405180910390a1611303611c54565b5050565b5f61132160015f8481526020019081526020015f2061212f565b9050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e611352816118f6565b5f61135c81611bcb565b7fbb709dea744f06d1b26e824deec2f7140c511266ee15d7a217838b31d8b0123d334260405161138d92919061335a565b60405180910390a161139d611c54565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6113cb816118f6565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698314801561140057506113ff838361114f565b5b15611484575f61140f84611307565b90505f60016002546003611423919061348a565b61142d919061342a565b90508082036114815760018161144391906134cb565b816040517f3a2362680000000000000000000000000000000000000000000000000000000081526004016114789291906134fe565b60405180910390fd5b50505b61148e8383612142565b50505050565b600d5481565b6060600182146114df576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016114d69061388a565b60405180910390fd5b5f6006546005546114f091906134cb565b9050808311156115395782816040517fdf3d75e20000000000000000000000000000000000000000000000000000000081526004016115309291906134fe565b60405180910390fd5b5f5f90505b6005548110156115f5573373ffffffffffffffffffffffffffffffffffffffff1660035f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16036115e857336040517faca92f090000000000000000000000000000000000000000000000000000000081526004016115df9190612f08565b60405180910390fd5b808060010191505061153e565b505f600184600654611607919061342a565b61161191906134cb565b90505f8467ffffffffffffffff81111561162e5761162d613046565b5b60405190808252806020026020018201604052801561165c5781602001602082028036833780820191505090505b5090505f60065490505b826006541161170b573360035f60065481526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555060065482826006546116d291906134cb565b815181106116e3576116e261345d565b5b60200260200101818152505060065f815480929190611701906133e3565b9190505550611666565b858461171791906134cb565b93507f1e14abe5d0cdb96adde7b9eca9b14bc08df623b5805afde5a3f0acadc2bf4f5b338360405161174a9291906138a8565b60405180910390a181945050505050919050565b611766611ce9565b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16036117d6575f6040517f1e4fbdf70000000000000000000000000000000000000000000000000000000081526004016117cd9190612f08565b60405180910390fd5b6117df81611d70565b50565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61180c816118f6565b6118168484611e69565b61182082856119f5565b61184485845f815181106118375761183661345d565b5b6020026020010151612155565b5050505050565b5f6118767fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698361114f565b9050919050565b5f7f5a05180f000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191614806118ef57506118ee826121d8565b5b9050919050565b61190781611902612251565b612258565b50565b5f7f19457468657265756d205369676e6564204d6573736167653a0a3332000000005f5281601c52603c5f209050919050565b5f5f5f5f61194b86866122a9565b92509250925061195b82826122fe565b82935050505092915050565b5f6119728383612460565b905092915050565b611982612251565b73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16146119e6576040517f6697b23200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6119f08282612142565b505050565b60055460085f828254611a08919061342a565b92505081905550816005819055505f6006819055505f600781905550806009819055505f5f90505b600554811015611b8b57600a5f60035f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f5f82015f9055600182015f9055505060045f60035f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f611b2f9190612a74565b5f60035f8381526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508080600101915050611a30565b507ff7f0872382dff5e698b284e12084e4e7894f830216dd80cb4e909b593a58f93560055433604051611bbf9291906138d6565b60405180910390a15050565b806005811115611bde57611bdd612c3a565b5b600f5f9054906101000a900460ff166005811115611bff57611bfe612c3a565b5b14611c515780600f5f9054906101000a900460ff166040517fbfa217d8000000000000000000000000000000000000000000000000000000008152600401611c489291906138fd565b60405180910390fd5b50565b6001600f5f9054906101000a900460ff166005811115611c7757611c76612c3a565b5b611c81919061342a565b6005811115611c9357611c92612c3a565b5b600f5f6101000a81548160ff02191690836005811115611cb657611cb5612c3a565b5b0217905550565b80600f5f6101000a81548160ff02191690836005811115611ce157611ce0612c3a565b5b021790555050565b611cf1612251565b73ffffffffffffffffffffffffffffffffffffffff16611d0f6110fb565b73ffffffffffffffffffffffffffffffffffffffff1614611d6e57611d32612251565b6040517f118cdaa7000000000000000000000000000000000000000000000000000000008152600401611d659190612f08565b60405180910390fd5b565b5f600b5f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905081600b5f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a35050565b5f611e40835f01836124a3565b5f1c905092915050565b60605f611e58835f016124ca565b905060608190508092505050919050565b5f6001836003611e79919061348a565b611e83919061342a565b90508082511015611ece578151816040517f3a236268000000000000000000000000000000000000000000000000000000008152600401611ec59291906134fe565b60405180910390fd5b826002819055505f611eff7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6111b8565b90505f611f2b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e611307565b90505f5f90505b81811015611f8c57611f7e7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e848381518110611f7157611f7061345d565b5b6020026020010151612142565b508080600101915050611f32565b505f611fb77fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696111b8565b90505f611fe37fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469611307565b90505f5f90505b81811015612044576120367fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698483815181106120295761202861345d565b5b6020026020010151612142565b508080600101915050611fea565b505f5f90505b86518110156120a5576120977fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46988838151811061208a5761208961345d565b5b6020026020010151611967565b50808060010191505061204a565b506120ea7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e875f815181106120dd576120dc61345d565b5b6020026020010151611967565b507fa2df7830e0bedef7b1116bf547b467b16b50b3bd23146c9e099878d14e89301a85883360405161211e93929190613924565b60405180910390a150505050505050565b5f61213b825f01612523565b9050919050565b5f61214d8383612532565b905092915050565b81600c8190555042600d8190555043600e819055505f600f5f6101000a81548160ff0219169083600581111561218e5761218d612c3a565b5b02179055507fdef1f08eb655f4a75f60bd6fd7e971112004abd846a612e46dab270770d24ca530600d54600e54846040516121cc9493929190613959565b60405180910390a15050565b5f7f7965db0b000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916148061224a575061224982612575565b5b9050919050565b5f33905090565b612262828261114f565b6122a55780826040517fe2517d3f00000000000000000000000000000000000000000000000000000000815260040161229c92919061399c565b60405180910390fd5b5050565b5f5f5f60418451036122e9575f5f5f602087015192506040870151915060608701515f1a90506122db888285856125de565b9550955095505050506122f7565b5f600285515f1b9250925092505b9250925092565b5f600381111561231157612310612c3a565b5b82600381111561232457612323612c3a565b5b031561245c576001600381111561233e5761233d612c3a565b5b82600381111561235157612350612c3a565b5b03612388576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6002600381111561239c5761239b612c3a565b5b8260038111156123af576123ae612c3a565b5b036123f357805f1c6040517ffce698f70000000000000000000000000000000000000000000000000000000081526004016123ea9190612cd5565b60405180910390fd5b60038081111561240657612405612c3a565b5b82600381111561241957612418612c3a565b5b0361245b57806040517fd78bce0c0000000000000000000000000000000000000000000000000000000081526004016124529190612e19565b60405180910390fd5b5b5050565b5f5f61246c84846126c5565b90508015612499576124978360015f8781526020019081526020015f206127ae90919063ffffffff16565b505b8091505092915050565b5f825f0182815481106124b9576124b861345d565b5b905f5260205f200154905092915050565b6060815f0180548060200260200160405190810160405280929190818152602001828054801561251757602002820191905f5260205f20905b815481526020019060010190808311612503575b50505050509050919050565b5f815f01805490509050919050565b5f5f61253e84846127db565b9050801561256b576125698360015f8781526020019081526020015f206128c490919063ffffffff16565b505b8091505092915050565b5f7f01ffc9a7000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916149050919050565b5f5f5f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c111561261a575f6003859250925092506126bb565b5f6001888888886040515f815260200160405260405161263d94939291906139de565b6020604051602081039080840390855afa15801561265d573d5f5f3e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16036126ae575f60015f5f1b935093509350506126bb565b805f5f5f1b935093509350505b9450945094915050565b5f6126d0838361114f565b6127a45760015f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550612741612251565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a4600190506127a8565b5f90505b92915050565b5f6127d3835f018373ffffffffffffffffffffffffffffffffffffffff165f1b6128f1565b905092915050565b5f6127e6838361114f565b156128ba575f5f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550612857612251565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b60405160405180910390a4600190506128be565b5f90505b92915050565b5f6128e9835f018373ffffffffffffffffffffffffffffffffffffffff165f1b612958565b905092915050565b5f6128fc8383612a54565b61294e57825f0182908060018154018082558091505060019003905f5260205f20015f9091909190915055825f0180549050836001015f8481526020019081526020015f208190555060019050612952565b5f90505b92915050565b5f5f836001015f8481526020019081526020015f205490505f8114612a49575f60018261298591906134cb565b90505f6001865f018054905061299b91906134cb565b9050808214612a01575f865f0182815481106129ba576129b961345d565b5b905f5260205f200154905080875f0184815481106129db576129da61345d565b5b905f5260205f20018190555083876001015f8381526020019081526020015f2081905550505b855f01805480612a1457612a13613a21565b5b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f905560019350505050612a4e565b5f9150505b92915050565b5f5f836001015f8481526020019081526020015f20541415905092915050565b505f81556001015f9055565b5f604051905090565b5f5ffd5b5f5ffd5b5f7fffffffff0000000000000000000000000000000000000000000000000000000082169050919050565b612ac581612a91565b8114612acf575f5ffd5b50565b5f81359050612ae081612abc565b92915050565b5f60208284031215612afb57612afa612a89565b5b5f612b0884828501612ad2565b91505092915050565b5f8115159050919050565b612b2581612b11565b82525050565b5f602082019050612b3e5f830184612b1c565b92915050565b5f819050919050565b612b5681612b44565b8114612b60575f5ffd5b50565b5f81359050612b7181612b4d565b92915050565b5f5f60408385031215612b8d57612b8c612a89565b5b5f612b9a85828601612b63565b9250506020612bab85828601612b63565b9150509250929050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f612bde82612bb5565b9050919050565b612bee81612bd4565b8114612bf8575f5ffd5b50565b5f81359050612c0981612be5565b92915050565b5f60208284031215612c2457612c23612a89565b5b5f612c3184828501612bfb565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b60068110612c7857612c77612c3a565b5b50565b5f819050612c8882612c67565b919050565b5f612c9782612c7b565b9050919050565b612ca781612c8d565b82525050565b5f602082019050612cc05f830184612c9e565b92915050565b612ccf81612b44565b82525050565b5f602082019050612ce85f830184612cc6565b92915050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f840112612d0f57612d0e612cee565b5b8235905067ffffffffffffffff811115612d2c57612d2b612cf2565b5b602083019150836001820283011115612d4857612d47612cf6565b5b9250929050565b5f5f5f60408486031215612d6657612d65612a89565b5b5f612d7386828701612bfb565b935050602084013567ffffffffffffffff811115612d9457612d93612a8d565b5b612da086828701612cfa565b92509250509250925092565b5f819050919050565b612dbe81612dac565b8114612dc8575f5ffd5b50565b5f81359050612dd981612db5565b92915050565b5f60208284031215612df457612df3612a89565b5b5f612e0184828501612dcb565b91505092915050565b612e1381612dac565b82525050565b5f602082019050612e2c5f830184612e0a565b92915050565b5f5f60408385031215612e4857612e47612a89565b5b5f612e5585828601612dcb565b9250506020612e6685828601612bfb565b9150509250929050565b5f5f60408385031215612e8657612e85612a89565b5b5f612e9385828601612bfb565b9250506020612ea485828601612bfb565b9150509250929050565b5f5f60208385031215612ec457612ec3612a89565b5b5f83013567ffffffffffffffff811115612ee157612ee0612a8d565b5b612eed85828601612cfa565b92509250509250929050565b612f0281612bd4565b82525050565b5f602082019050612f1b5f830184612ef9565b92915050565b5f5f60408385031215612f3757612f36612a89565b5b5f612f4485828601612dcb565b9250506020612f5585828601612b63565b9150509250929050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b612f9181612bd4565b82525050565b5f612fa28383612f88565b60208301905092915050565b5f602082019050919050565b5f612fc482612f5f565b612fce8185612f69565b9350612fd983612f79565b805f5b83811015613009578151612ff08882612f97565b9750612ffb83612fae565b925050600181019050612fdc565b5085935050505092915050565b5f6020820190508181035f83015261302e8184612fba565b905092915050565b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61307c82613036565b810181811067ffffffffffffffff8211171561309b5761309a613046565b5b80604052505050565b5f6130ad612a80565b90506130b98282613073565b919050565b5f67ffffffffffffffff8211156130d8576130d7613046565b5b602082029050602081019050919050565b5f6130fb6130f6846130be565b6130a4565b9050808382526020820190506020840283018581111561311e5761311d612cf6565b5b835b8181101561314757806131338882612bfb565b845260208401935050602081019050613120565b5050509392505050565b5f82601f83011261316557613164612cee565b5b81356131758482602086016130e9565b91505092915050565b5f5f6040838503121561319457613193612a89565b5b5f6131a185828601612b63565b925050602083013567ffffffffffffffff8111156131c2576131c1612a8d565b5b6131ce85828601613151565b9150509250929050565b5f602082840312156131ed576131ec612a89565b5b5f6131fa84828501612b63565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b61323581612b44565b82525050565b5f613246838361322c565b60208301905092915050565b5f602082019050919050565b5f61326882613203565b613272818561320d565b935061327d8361321d565b805f5b838110156132ad578151613294888261323b565b975061329f83613252565b925050600181019050613280565b5085935050505092915050565b5f6020820190508181035f8301526132d2818461325e565b905092915050565b5f5f5f5f608085870312156132f2576132f1612a89565b5b5f6132ff87828801612dcb565b945050602061331087828801612b63565b935050604085013567ffffffffffffffff81111561333157613330612a8d565b5b61333d87828801613151565b925050606061334e87828801612b63565b91505092959194509250565b5f60408201905061336d5f830185612ef9565b61337a6020830184612cc6565b9392505050565b5f6060820190506133945f830186612ef9565b6133a16020830185612cc6565b6133ae6040830184612cc6565b949350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6133ed82612b44565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff820361341f5761341e6133b6565b5b600182019050919050565b5f61343482612b44565b915061343f83612b44565b9250828201905080821115613457576134566133b6565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f61349482612b44565b915061349f83612b44565b92508282026134ad81612b44565b915082820484148315176134c4576134c36133b6565b5b5092915050565b5f6134d582612b44565b91506134e083612b44565b92508282039050818111156134f8576134f76133b6565b5b92915050565b5f6040820190506135115f830185612cc6565b61351e6020830184612cc6565b9392505050565b5f82905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061357357607f821691505b6020821081036135865761358561352f565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026135e87fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff826135ad565b6135f286836135ad565b95508019841693508086168417925050509392505050565b5f819050919050565b5f61362d61362861362384612b44565b61360a565b612b44565b9050919050565b5f819050919050565b61364683613613565b61365a61365282613634565b8484546135b9565b825550505050565b5f5f905090565b613671613662565b61367c81848461363d565b505050565b5f5b828110156136a2576136975f828401613669565b600181019050613683565b505050565b601f8211156136f557828211156136f4576136c18161358c565b6136ca8361359e565b6136d38561359e565b60208610156136e0575f90505b8083016136ef82840382613681565b505050505b5b505050565b5f82821c905092915050565b5f6137155f19846008026136fa565b1980831691505092915050565b5f61372d8383613706565b9150826002028217905092915050565b6137478383613525565b67ffffffffffffffff8111156137605761375f613046565b5b61376a825461355c565b6137758282856136a7565b5f601f8311600181146137a2575f8415613790578287013590505b61379a8582613722565b865550613801565b601f1984166137b08661358c565b5f5b828110156137d7578489013582556001820191506020850194506020810190506137b2565b868310156137f457848901356137f0601f891682613706565b8355505b6001600288020188555050505b50505050505050565b5f82825260208201905092915050565b7f43555252454e544c59204f4e4c59204f4e4520494e4445582050455220434c495f8201527f454e5420414c4c4f574544000000000000000000000000000000000000000000602082015250565b5f613874602b8361380a565b915061387f8261381a565b604082019050919050565b5f6020820190508181035f8301526138a181613868565b9050919050565b5f6040820190506138bb5f830185612ef9565b81810360208301526138cd818461325e565b90509392505050565b5f6040820190506138e95f830185612cc6565b6138f66020830184612ef9565b9392505050565b5f6040820190506139105f830185612c9e565b61391d6020830184612c9e565b9392505050565b5f6060820190506139375f830186612cc6565b6139446020830185612cc6565b6139516040830184612ef9565b949350505050565b5f60808201905061396c5f830187612ef9565b6139796020830186612cc6565b6139866040830185612cc6565b6139936060830184612ef9565b95945050505050565b5f6040820190506139af5f830185612ef9565b6139bc6020830184612e0a565b9392505050565b5f60ff82169050919050565b6139d8816139c3565b82525050565b5f6080820190506139f15f830187612e0a565b6139fe60208301866139cf565b613a0b6040830185612e0a565b613a186060830184612e0a565b95945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffdfea2646970667358221220277555b51ff897ae65a1356ab2c167f249f574282c63267666c552884bd3e6be64736f6c63430008210033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`@QaM68\x03\x80aM6\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\x0F\xA8V[\x85\x84\x83\x833\x81\x84\x85\x85a\0J\x82\x82a\x01m` \x1B` \x1CV[PP_`\x08\x81\x90UPa\0c\x82\x82a\x04c` \x1B` \x1CV[PP_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\0\xD5W_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\0\xCC\x91\x90a\x10\\V[`@Q\x80\x91\x03\x90\xFD[a\0\xE4\x81a\x069` \x1B` \x1CV[Pa\x01\x0F\x84\x83_\x81Q\x81\x10a\0\xFCWa\0\xFBa\x10uV[[` \x02` \x01\x01Qa\x06\xFC` \x1B` \x1CV[PPPPB`\r\x81\x90UPC`\x0E\x81\x90UP\x7F\xDE\xF1\xF0\x8E\xB6U\xF4\xA7_`\xBDo\xD7\xE9q\x11 \x04\xAB\xD8F\xA6\x12\xE4m\xAB'\x07p\xD2L\xA50`\rT`\x0ET3`@Qa\x01Z\x94\x93\x92\x91\x90a\x10\xB1V[`@Q\x80\x91\x03\x90\xA1PPPPPPa\x12\xA5V[_`\x01\x83`\x03a\x01}\x91\x90a\x11!V[a\x01\x87\x91\x90a\x11bV[\x90P\x80\x82Q\x10\x15a\x01\xD2W\x81Q\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x01\xC9\x92\x91\x90a\x11\x95V[`@Q\x80\x91\x03\x90\xFD[\x82`\x02\x81\x90UP_a\x02\t\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x07\x7F` \x1B` \x1CV[\x90P_a\x02;\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x07\xA7` \x1B` \x1CV[\x90P__\x90P[\x81\x81\x10\x15a\x02\xA2Wa\x02\x94\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x84\x83\x81Q\x81\x10a\x02\x81Wa\x02\x80a\x10uV[[` \x02` \x01\x01Qa\x07\xCE` \x1B` \x1CV[P\x80\x80`\x01\x01\x91PPa\x02BV[P_a\x02\xD3\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x07\x7F` \x1B` \x1CV[\x90P_a\x03\x05\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x07\xA7` \x1B` \x1CV[\x90P__\x90P[\x81\x81\x10\x15a\x03lWa\x03^\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x84\x83\x81Q\x81\x10a\x03KWa\x03Ja\x10uV[[` \x02` \x01\x01Qa\x07\xCE` \x1B` \x1CV[P\x80\x80`\x01\x01\x91PPa\x03\x0CV[P__\x90P[\x86Q\x81\x10\x15a\x03\xD3Wa\x03\xC5\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x88\x83\x81Q\x81\x10a\x03\xB2Wa\x03\xB1a\x10uV[[` \x02` \x01\x01Qa\x07\xE7` \x1B` \x1CV[P\x80\x80`\x01\x01\x91PPa\x03rV[Pa\x04\x1E\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x87_\x81Q\x81\x10a\x04\x0BWa\x04\na\x10uV[[` \x02` \x01\x01Qa\x07\xE7` \x1B` \x1CV[P\x7F\xA2\xDFx0\xE0\xBE\xDE\xF7\xB1\x11k\xF5G\xB4g\xB1kP\xB3\xBD#\x14l\x9E\t\x98x\xD1N\x890\x1A\x85\x883`@Qa\x04R\x93\x92\x91\x90a\x11\xBCV[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[`\x05T`\x08_\x82\x82Ta\x04v\x91\x90a\x11bV[\x92PP\x81\x90UP\x81`\x05\x81\x90UP_`\x06\x81\x90UP_`\x07\x81\x90UP\x80`\t\x81\x90UP__\x90P[`\x05T\x81\x10\x15a\x05\xF9W`\n_`\x03_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ __\x82\x01_\x90U`\x01\x82\x01_\x90UPP`\x04_`\x03_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x05\x9D\x91\x90a\r{V[_`\x03_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80\x80`\x01\x01\x91PPa\x04\x9EV[P\x7F\xF7\xF0\x87#\x82\xDF\xF5\xE6\x98\xB2\x84\xE1 \x84\xE4\xE7\x89O\x83\x02\x16\xDD\x80\xCBN\x90\x9BY:X\xF95`\x05T3`@Qa\x06-\x92\x91\x90a\x11\xF1V[`@Q\x80\x91\x03\x90\xA1PPV[_`\x0B_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\x0B_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[\x81`\x0C\x81\x90UPB`\r\x81\x90UPC`\x0E\x81\x90UP_`\x0F_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x075Wa\x074a\x12\x18V[[\x02\x17\x90UP\x7F\xDE\xF1\xF0\x8E\xB6U\xF4\xA7_`\xBDo\xD7\xE9q\x11 \x04\xAB\xD8F\xA6\x12\xE4m\xAB'\x07p\xD2L\xA50`\rT`\x0ET\x84`@Qa\x07s\x94\x93\x92\x91\x90a\x10\xB1V[`@Q\x80\x91\x03\x90\xA1PPV[``a\x07\xA0`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a\x08\0` \x1B` \x1CV[\x90P\x91\x90PV[_a\x07\xC7`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a\x08%` \x1B` \x1CV[\x90P\x91\x90PV[_a\x07\xDF\x83\x83a\x08>` \x1B` \x1CV[\x90P\x92\x91PPV[_a\x07\xF8\x83\x83a\x08\x87` \x1B` \x1CV[\x90P\x92\x91PPV[``_a\x08\x14\x83_\x01a\x08\xD0` \x1B` \x1CV[\x90P``\x81\x90P\x80\x92PPP\x91\x90PV[_a\x087\x82_\x01a\t)` \x1B` \x1CV[\x90P\x91\x90PV[__a\x08P\x84\x84a\t8` \x1B` \x1CV[\x90P\x80\x15a\x08}Wa\x08{\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a\n-` \x1B\x90\x91\x90` \x1CV[P[\x80\x91PP\x92\x91PPV[__a\x08\x99\x84\x84a\n`` \x1B` \x1CV[\x90P\x80\x15a\x08\xC6Wa\x08\xC4\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a\x0BU` \x1B\x90\x91\x90` \x1CV[P[\x80\x91PP\x92\x91PPV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\t\x1DW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\t\tW[PPPPP\x90P\x91\x90PV[_\x81_\x01\x80T\x90P\x90P\x91\x90PV[_a\tI\x83\x83a\x0B\x88` \x1B` \x1CV[\x15a\n#W___\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\t\xC0a\x0B\xEB` \x1B` \x1CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\n'V[_\x90P[\x92\x91PPV[_a\nX\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba\x0B\xF2` \x1B` \x1CV[\x90P\x92\x91PPV[_a\nq\x83\x83a\x0B\x88` \x1B` \x1CV[a\x0BKW`\x01__\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\n\xE8a\x0B\xEB` \x1B` \x1CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\x0BOV[_\x90P[\x92\x91PPV[_a\x0B\x80\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba\x0C\xEE` \x1B` \x1CV[\x90P\x92\x91PPV[___\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[_3\x90P\x90V[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x14a\x0C\xE3W_`\x01\x82a\x0C\x1F\x91\x90a\x12EV[\x90P_`\x01\x86_\x01\x80T\x90Pa\x0C5\x91\x90a\x12EV[\x90P\x80\x82\x14a\x0C\x9BW_\x86_\x01\x82\x81T\x81\x10a\x0CTWa\x0CSa\x10uV[[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a\x0CuWa\x0Cta\x10uV[[\x90_R` _ \x01\x81\x90UP\x83\x87`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UPP[\x85_\x01\x80T\x80a\x0C\xAEWa\x0C\xADa\x12xV[[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\x0C\xE8V[_\x91PP[\x92\x91PPV[_a\x0C\xFF\x83\x83a\r[` \x1B` \x1CV[a\rQW\x82_\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x82_\x01\x80T\x90P\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP`\x01\x90Pa\rUV[_\x90P[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x14\x15\x90P\x92\x91PPV[P_\x81U`\x01\x01_\x90UV[_`@Q\x90P\x90V[__\xFD[__\xFD[_\x81\x90P\x91\x90PV[a\r\xAA\x81a\r\x98V[\x81\x14a\r\xB4W__\xFD[PV[_\x81Q\x90Pa\r\xC5\x81a\r\xA1V[\x92\x91PPV[_\x81\x90P\x91\x90PV[a\r\xDD\x81a\r\xCBV[\x81\x14a\r\xE7W__\xFD[PV[_\x81Q\x90Pa\r\xF8\x81a\r\xD4V[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x0E'\x82a\r\xFEV[\x90P\x91\x90PV[a\x0E7\x81a\x0E\x1DV[\x81\x14a\x0EAW__\xFD[PV[_\x81Q\x90Pa\x0ER\x81a\x0E.V[\x92\x91PPV[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x0E\xA2\x82a\x0E\\V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0E\xC1Wa\x0E\xC0a\x0ElV[[\x80`@RPPPV[_a\x0E\xD3a\r\x87V[\x90Pa\x0E\xDF\x82\x82a\x0E\x99V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0E\xFEWa\x0E\xFDa\x0ElV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[__\xFD[_a\x0F%a\x0F \x84a\x0E\xE4V[a\x0E\xCAV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x0FHWa\x0FGa\x0F\x0FV[[\x83[\x81\x81\x10\x15a\x0FqW\x80a\x0F]\x88\x82a\x0EDV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x0FJV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x0F\x8FWa\x0F\x8Ea\x0EXV[[\x81Qa\x0F\x9F\x84\x82` \x86\x01a\x0F\x13V[\x91PP\x92\x91PPV[______`\xC0\x87\x89\x03\x12\x15a\x0F\xC2Wa\x0F\xC1a\r\x90V[[_a\x0F\xCF\x89\x82\x8A\x01a\r\xB7V[\x96PP` a\x0F\xE0\x89\x82\x8A\x01a\r\xEAV[\x95PP`@a\x0F\xF1\x89\x82\x8A\x01a\r\xEAV[\x94PP``a\x10\x02\x89\x82\x8A\x01a\x0EDV[\x93PP`\x80\x87\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10#Wa\x10\"a\r\x94V[[a\x10/\x89\x82\x8A\x01a\x0F{V[\x92PP`\xA0a\x10@\x89\x82\x8A\x01a\r\xEAV[\x91PP\x92\x95P\x92\x95P\x92\x95V[a\x10V\x81a\x0E\x1DV[\x82RPPV[_` \x82\x01\x90Pa\x10o_\x83\x01\x84a\x10MV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[a\x10\xAB\x81a\r\xCBV[\x82RPPV[_`\x80\x82\x01\x90Pa\x10\xC4_\x83\x01\x87a\x10MV[a\x10\xD1` \x83\x01\x86a\x10\xA2V[a\x10\xDE`@\x83\x01\x85a\x10\xA2V[a\x10\xEB``\x83\x01\x84a\x10MV[\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x11+\x82a\r\xCBV[\x91Pa\x116\x83a\r\xCBV[\x92P\x82\x82\x02a\x11D\x81a\r\xCBV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x11[Wa\x11Za\x10\xF4V[[P\x92\x91PPV[_a\x11l\x82a\r\xCBV[\x91Pa\x11w\x83a\r\xCBV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x11\x8FWa\x11\x8Ea\x10\xF4V[[\x92\x91PPV[_`@\x82\x01\x90Pa\x11\xA8_\x83\x01\x85a\x10\xA2V[a\x11\xB5` \x83\x01\x84a\x10\xA2V[\x93\x92PPPV[_``\x82\x01\x90Pa\x11\xCF_\x83\x01\x86a\x10\xA2V[a\x11\xDC` \x83\x01\x85a\x10\xA2V[a\x11\xE9`@\x83\x01\x84a\x10MV[\x94\x93PPPPV[_`@\x82\x01\x90Pa\x12\x04_\x83\x01\x85a\x10\xA2V[a\x12\x11` \x83\x01\x84a\x10MV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_a\x12O\x82a\r\xCBV[\x91Pa\x12Z\x83a\r\xCBV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x12rWa\x12qa\x10\xF4V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD[a:\x84\x80a\x12\xB2_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02\x1AW_5`\xE0\x1C\x80cqP\x18\xA6\x11a\x01#W\x80c\xC0y\xF4\x95\x11a\0\xABW\x80c\xD8'\r\xCE\x11a\0zW\x80c\xD8'\r\xCE\x14a\x05\xA0W\x80c\xEB\xAE5\xE7\x14a\x05\xBEW\x80c\xF2\xFD\xE3\x8B\x14a\x05\xEEW\x80c\xF6`<a\x14a\x06\nW\x80c\xFCx\xB2\xE8\x14a\x06&Wa\x02\x1AV[\x80c\xC0y\xF4\x95\x14a\x05@W\x80c\xCA\x15\xC8s\x14a\x05JW\x80c\xCB\x9CL\xC4\x14a\x05zW\x80c\xD5Gt\x1F\x14a\x05\x84Wa\x02\x1AV[\x80c\x91\xD1HT\x11a\0\xF2W\x80c\x91\xD1HT\x14a\x04\x9CW\x80c\xA2\x17\xFD\xDF\x14a\x04\xCCW\x80c\xA3$j\xD3\x14a\x04\xEAW\x80c\xAF o(\x14a\x05\x1AW\x80c\xBBQ\xFE\xF0\x14a\x056Wa\x02\x1AV[\x80cqP\x18\xA6\x14a\x04&W\x80c\x7F5\xB5`\x14a\x040W\x80c\x8D\xA5\xCB[\x14a\x04NW\x80c\x90\x10\xD0|\x14a\x04lWa\x02\x1AV[\x80c//\xF1]\x11a\x01\xA6W\x80cI%\xE6*\x11a\x01uW\x80cI%\xE6*\x14a\x03\xD0W\x80cK\x8Ed\x88\x14a\x03\xECW\x80cK\xB2x\xF3\x14a\x03\xF6W\x80cY_\x80n\x14a\x04\0W\x80cktG(\x14a\x04\nWa\x02\x1AV[\x80c//\xF1]\x14a\x03^W\x80c0\x10L>\x14a\x03zW\x80c6V\x8A\xBE\x14a\x03\x98W\x80c;C8\xD1\x14a\x03\xB4Wa\x02\x1AV[\x80c\x17cE\x14\x11a\x01\xEDW\x80c\x17cE\x14\x14a\x02\xB8W\x80c\x1CtS\xDB\x14a\x02\xD6W\x80c\x1E\xE4\xEE\x0F\x14a\x02\xF4W\x80c#(\xBD\x12\x14a\x03\x10W\x80c$\x8A\x9C\xA3\x14a\x03.Wa\x02\x1AV[\x80c\x01\xFF\xC9\xA7\x14a\x02\x1EW\x80c\x0B\xDA\x81\xCF\x14a\x02NW\x80c\x13\xFFm\xD5\x14a\x02jW\x80c\x14l\xA51\x14a\x02\x9AW[__\xFD[a\x028`\x04\x806\x03\x81\x01\x90a\x023\x91\x90a*\xE6V[a\x06VV[`@Qa\x02E\x91\x90a++V[`@Q\x80\x91\x03\x90\xF3[a\x02h`\x04\x806\x03\x81\x01\x90a\x02c\x91\x90a+wV[a\x06\xCFV[\0[a\x02\x84`\x04\x806\x03\x81\x01\x90a\x02\x7F\x91\x90a,\x0FV[a\x08.V[`@Qa\x02\x91\x91\x90a++V[`@Q\x80\x91\x03\x90\xF3[a\x02\xA2a\x08\xAAV[`@Qa\x02\xAF\x91\x90a,\xADV[`@Q\x80\x91\x03\x90\xF3[a\x02\xC0a\x08\xBCV[`@Qa\x02\xCD\x91\x90a,\xD5V[`@Q\x80\x91\x03\x90\xF3[a\x02\xDEa\x08\xC2V[`@Qa\x02\xEB\x91\x90a,\xD5V[`@Q\x80\x91\x03\x90\xF3[a\x03\x0E`\x04\x806\x03\x81\x01\x90a\x03\t\x91\x90a-OV[a\x08\xC8V[\0[a\x03\x18a\x0C\xBEV[`@Qa\x03%\x91\x90a,\xD5V[`@Q\x80\x91\x03\x90\xF3[a\x03H`\x04\x806\x03\x81\x01\x90a\x03C\x91\x90a-\xDFV[a\x0C\xD4V[`@Qa\x03U\x91\x90a.\x19V[`@Q\x80\x91\x03\x90\xF3[a\x03x`\x04\x806\x03\x81\x01\x90a\x03s\x91\x90a.2V[a\x0C\xF0V[\0[a\x03\x82a\r*V[`@Qa\x03\x8F\x91\x90a.\x19V[`@Q\x80\x91\x03\x90\xF3[a\x03\xB2`\x04\x806\x03\x81\x01\x90a\x03\xAD\x91\x90a.2V[a\rNV[\0[a\x03\xCE`\x04\x806\x03\x81\x01\x90a\x03\xC9\x91\x90a+wV[a\x0E\x15V[\0[a\x03\xEA`\x04\x806\x03\x81\x01\x90a\x03\xE5\x91\x90a.pV[a\x0ENV[\0[a\x03\xF4a\x0F\x10V[\0[a\x03\xFEa\x0F\x8AV[\0[a\x04\x08a\x10\x07V[\0[a\x04$`\x04\x806\x03\x81\x01\x90a\x04\x1F\x91\x90a.\xAEV[a\x10\x81V[\0[a\x04.a\x10\xC4V[\0[a\x048a\x10\xD7V[`@Qa\x04E\x91\x90a.\x19V[`@Q\x80\x91\x03\x90\xF3[a\x04Va\x10\xFBV[`@Qa\x04c\x91\x90a/\x08V[`@Q\x80\x91\x03\x90\xF3[a\x04\x86`\x04\x806\x03\x81\x01\x90a\x04\x81\x91\x90a/!V[a\x11#V[`@Qa\x04\x93\x91\x90a/\x08V[`@Q\x80\x91\x03\x90\xF3[a\x04\xB6`\x04\x806\x03\x81\x01\x90a\x04\xB1\x91\x90a.2V[a\x11OV[`@Qa\x04\xC3\x91\x90a++V[`@Q\x80\x91\x03\x90\xF3[a\x04\xD4a\x11\xB2V[`@Qa\x04\xE1\x91\x90a.\x19V[`@Q\x80\x91\x03\x90\xF3[a\x05\x04`\x04\x806\x03\x81\x01\x90a\x04\xFF\x91\x90a-\xDFV[a\x11\xB8V[`@Qa\x05\x11\x91\x90a0\x16V[`@Q\x80\x91\x03\x90\xF3[a\x054`\x04\x806\x03\x81\x01\x90a\x05/\x91\x90a1~V[a\x11\xDAV[\0[a\x05>a\x12\x13V[\0[a\x05Ha\x12\x8DV[\0[a\x05d`\x04\x806\x03\x81\x01\x90a\x05_\x91\x90a-\xDFV[a\x13\x07V[`@Qa\x05q\x91\x90a,\xD5V[`@Q\x80\x91\x03\x90\xF3[a\x05\x82a\x13(V[\0[a\x05\x9E`\x04\x806\x03\x81\x01\x90a\x05\x99\x91\x90a.2V[a\x13\xA1V[\0[a\x05\xA8a\x14\x94V[`@Qa\x05\xB5\x91\x90a,\xD5V[`@Q\x80\x91\x03\x90\xF3[a\x05\xD8`\x04\x806\x03\x81\x01\x90a\x05\xD3\x91\x90a1\xD8V[a\x14\x9AV[`@Qa\x05\xE5\x91\x90a2\xBAV[`@Q\x80\x91\x03\x90\xF3[a\x06\x08`\x04\x806\x03\x81\x01\x90a\x06\x03\x91\x90a,\x0FV[a\x17^V[\0[a\x06$`\x04\x806\x03\x81\x01\x90a\x06\x1F\x91\x90a2\xDAV[a\x17\xE2V[\0[a\x06@`\x04\x806\x03\x81\x01\x90a\x06;\x91\x90a,\x0FV[a\x18KV[`@Qa\x06M\x91\x90a++V[`@Q\x80\x91\x03\x90\xF3[_\x7F\x07\xEF\xFE\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x06\xC8WPa\x06\xC7\x82a\x18}V[[\x90P\x91\x90PV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07qW3\x81`@Q\x7F\xFF\xAB\xBA\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07h\x92\x91\x90a3ZV[`@Q\x80\x91\x03\x90\xFD[`@Q\x80`@\x01`@R\x80\x82\x81R` \x01\x83\x81RP`\n_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01U` \x82\x01Q\x81`\x01\x01U\x90PP\x7F\xB8\x9A\xDD\xD97\xF4O\x90,\x84\x95\x96d\x187\xCDz\xF2\xFC\xEC\xEF\"\xD2\xA7\x86o\xDC\x1A\xD9\xC0\xAE.3\x83\x83`@Qa\x08\x0B\x93\x92\x91\x90a3\x81V[`@Q\x80\x91\x03\x90\xA1`\x07_\x81T\x80\x92\x91\x90a\x08%\x90a3\xE3V[\x91\x90PUPPPV[_a\x088\x82a\x18KV[a\x08yW\x81`@Q\x7F\xAB\xDC\xE0j\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08p\x91\x90a/\x08V[`@Q\x80\x91\x03\x90\xFD[a\x08\xA3\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x83a\x11OV[\x90P\x91\x90PV[`\x0F_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\x0ET\x81V[`\x08T\x81V[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x08\xF2\x81a\x18\xF6V[_`\x05T\x90P__\x90P[`\x05T\x81\x10\x15a\t\x7FW\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\trW\x80\x91Pa\t\x7FV[\x80\x80`\x01\x01\x91PPa\x08\xFDV[P`\x05T\x81\x03a\t\xF6W`\x03__\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x7Fo\xAF\x9F\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xED\x91\x90a/\x08V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x08Ta\n\x05\x91\x90a4*V[\x90P_a\n7\x82`@Q` \x01a\n\x1C\x91\x90a,\xD5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x19\nV[\x90P_a\n\x87\x82\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa\x19=V[\x90P`\x04_\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _`\x02\x81\x10a\n\xD8Wa\n\xD7a4]V[[\x01_\x81Ta\n\xE5\x90a3\xE3V[\x91\x90P\x81\x90UP\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0B\x84W`\x04_\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01`\x02\x81\x10a\x0BoWa\x0Bna4]V[[\x01_\x81Ta\x0B|\x90a3\xE3V[\x91\x90P\x81\x90UP[`\x01`\tT`\x02a\x0B\x95\x91\x90a4\x8AV[a\x0B\x9F\x91\x90a4*V[`\x04_\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _`\x02\x81\x10a\x0B\xEEWa\x0B\xEDa4]V[[\x01T\x03a\x0C\xB4W__\x90P`\x01`\tTa\x0C\x08\x91\x90a4*V[`\x04_\x8Bs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01`\x02\x81\x10a\x0CXWa\x0CWa4]V[[\x01T\x10a\x0CdW`\x01\x90P[\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F@p\t \x05 \xF9\xF1\x05\x84\x81<\x0B\x95D\x1A\xB3 \xF6\xB0\x8D\x97\xEB\xDA\xAF\x1E\x82N\xED\xD9\xD7\xC3\x82`@Qa\x0C\xAA\x91\x90a++V[`@Q\x80\x91\x03\x90\xA2P[PPPPPPPPV[_`\x06T`\x05Ta\x0C\xCF\x91\x90a4\xCBV[\x90P\x90V[___\x83\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\r\x1A\x81a\x18\xF6V[a\r$\x83\x83a\x19gV[PPPPV[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x81V[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x82\x14\x80\x15a\r\x83WPa\r\x82\x82\x82a\x11OV[[\x15a\x0E\x07W_a\r\x92\x83a\x13\x07V[\x90P_`\x01`\x02T`\x03a\r\xA6\x91\x90a4\x8AV[a\r\xB0\x91\x90a4*V[\x90P\x80\x82\x03a\x0E\x04W`\x01\x81a\r\xC6\x91\x90a4\xCBV[\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\xFB\x92\x91\x90a4\xFEV[`@Q\x80\x91\x03\x90\xFD[PP[a\x0E\x11\x82\x82a\x19zV[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0E?\x81a\x18\xF6V[a\x0EI\x83\x83a\x19\xF5V[PPPV[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x0Ex\x81a\x18\xF6V[`\x01`\x10`\x01\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPPPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0F:\x81a\x18\xF6V[`\x04a\x0FE\x81a\x1B\xCBV[\x7F0\x1F\x8A7\x01\xF5\xB2`\x19s\x82\xDDs\x01\x07\x85B\x14O\xE8\xFD\xDD\x18\x08=on\t\xE4\x95\x8AY3B`@Qa\x0Fv\x92\x91\x90a3ZV[`@Q\x80\x91\x03\x90\xA1a\x0F\x86a\x1CTV[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0F\xB4\x81a\x18\xF6V[`\x05a\x0F\xBF\x81a\x1B\xCBV[_\x7F$\xA8se\x1D&\xFBZF,\xB3z\x91\x07\x1C\xDDM\t\xAB\xEE\xBF\xE0\xED\x14C)\xBE\xD1\xCC5\x9D\x033B`@Qa\x0F\xF1\x92\x91\x90a3ZV[`@Q\x80\x91\x03\x90\xA1a\x10\x02\x81a\x1C\xBDV[PPPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x101\x81a\x18\xF6V[`\x03a\x10<\x81a\x1B\xCBV[\x7F \xF5^\xD0\xC9/+\xB1\xC8\x82T\x88\xE1\xE3\xC9\x84c\xD0$\xB2\xA4-\xBD$\x83\x8C?u&\x0FC\xE93B`@Qa\x10m\x92\x91\x90a3ZV[`@Q\x80\x91\x03\x90\xA1a\x10}a\x1CTV[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x10\xAB\x81a\x18\xF6V[\x82\x82`\x10_\x01\x91\x82a\x10\xBE\x92\x91\x90a7=V[PPPPV[a\x10\xCCa\x1C\xE9V[a\x10\xD5_a\x1DpV[V[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x81V[_`\x0B_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_a\x11G\x82`\x01_\x86\x81R` \x01\x90\x81R` \x01_ a\x1E3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[___\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[__\x1B\x81V[``a\x11\xD3`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a\x1EJV[\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x12\x04\x81a\x18\xF6V[a\x12\x0E\x83\x83a\x1EiV[PPPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x12=\x81a\x18\xF6V[`\x02a\x12H\x81a\x1B\xCBV[\x7F`\xED\xF9\xBD\xC7\xC4\xEA\0|\xAE\x1A\x9B\xBD\x03\xE4\x1E[\xFC\xCDr1\xA6\xEC8<.\xDDx\0\xF0\xD2\x0C3B`@Qa\x12y\x92\x91\x90a3ZV[`@Q\x80\x91\x03\x90\xA1a\x12\x89a\x1CTV[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x12\xB7\x81a\x18\xF6V[`\x01a\x12\xC2\x81a\x1B\xCBV[\x7Fg\xC4H\x9FgM\x03\xC7\xD1\x9A\x9E6sQ\x88\xDE|e\xE8\xD1\xE9\x9E\xB3\xA2\xFD%\x8Av\x9E\xB1O\xFF3B`@Qa\x12\xF3\x92\x91\x90a3ZV[`@Q\x80\x91\x03\x90\xA1a\x13\x03a\x1CTV[PPV[_a\x13!`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a!/V[\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x13R\x81a\x18\xF6V[_a\x13\\\x81a\x1B\xCBV[\x7F\xBBp\x9D\xEAtO\x06\xD1\xB2n\x82M\xEE\xC2\xF7\x14\x0CQ\x12f\xEE\x15\xD7\xA2\x17\x83\x8B1\xD8\xB0\x12=3B`@Qa\x13\x8D\x92\x91\x90a3ZV[`@Q\x80\x91\x03\x90\xA1a\x13\x9Da\x1CTV[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x13\xCB\x81a\x18\xF6V[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x83\x14\x80\x15a\x14\0WPa\x13\xFF\x83\x83a\x11OV[[\x15a\x14\x84W_a\x14\x0F\x84a\x13\x07V[\x90P_`\x01`\x02T`\x03a\x14#\x91\x90a4\x8AV[a\x14-\x91\x90a4*V[\x90P\x80\x82\x03a\x14\x81W`\x01\x81a\x14C\x91\x90a4\xCBV[\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14x\x92\x91\x90a4\xFEV[`@Q\x80\x91\x03\x90\xFD[PP[a\x14\x8E\x83\x83a!BV[PPPPV[`\rT\x81V[```\x01\x82\x14a\x14\xDFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14\xD6\x90a8\x8AV[`@Q\x80\x91\x03\x90\xFD[_`\x06T`\x05Ta\x14\xF0\x91\x90a4\xCBV[\x90P\x80\x83\x11\x15a\x159W\x82\x81`@Q\x7F\xDF=u\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x150\x92\x91\x90a4\xFEV[`@Q\x80\x91\x03\x90\xFD[__\x90P[`\x05T\x81\x10\x15a\x15\xF5W3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x15\xE8W3`@Q\x7F\xAC\xA9/\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15\xDF\x91\x90a/\x08V[`@Q\x80\x91\x03\x90\xFD[\x80\x80`\x01\x01\x91PPa\x15>V[P_`\x01\x84`\x06Ta\x16\x07\x91\x90a4*V[a\x16\x11\x91\x90a4\xCBV[\x90P_\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16.Wa\x16-a0FV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\\W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_`\x06T\x90P[\x82`\x06T\x11a\x17\x0BW3`\x03_`\x06T\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x06T\x82\x82`\x06Ta\x16\xD2\x91\x90a4\xCBV[\x81Q\x81\x10a\x16\xE3Wa\x16\xE2a4]V[[` \x02` \x01\x01\x81\x81RPP`\x06_\x81T\x80\x92\x91\x90a\x17\x01\x90a3\xE3V[\x91\x90PUPa\x16fV[\x85\x84a\x17\x17\x91\x90a4\xCBV[\x93P\x7F\x1E\x14\xAB\xE5\xD0\xCD\xB9j\xDD\xE7\xB9\xEC\xA9\xB1K\xC0\x8D\xF6#\xB5\x80Z\xFD\xE5\xA3\xF0\xAC\xAD\xC2\xBFO[3\x83`@Qa\x17J\x92\x91\x90a8\xA8V[`@Q\x80\x91\x03\x90\xA1\x81\x94PPPPP\x91\x90PV[a\x17fa\x1C\xE9V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x17\xD6W_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17\xCD\x91\x90a/\x08V[`@Q\x80\x91\x03\x90\xFD[a\x17\xDF\x81a\x1DpV[PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x18\x0C\x81a\x18\xF6V[a\x18\x16\x84\x84a\x1EiV[a\x18 \x82\x85a\x19\xF5V[a\x18D\x85\x84_\x81Q\x81\x10a\x187Wa\x186a4]V[[` \x02` \x01\x01Qa!UV[PPPPPV[_a\x18v\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x83a\x11OV[\x90P\x91\x90PV[_\x7FZ\x05\x18\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x18\xEFWPa\x18\xEE\x82a!\xD8V[[\x90P\x91\x90PV[a\x19\x07\x81a\x19\x02a\"QV[a\"XV[PV[_\x7F\x19Ethereum Signed Message:\n32\0\0\0\0_R\x81`\x1CR`<_ \x90P\x91\x90PV[____a\x19K\x86\x86a\"\xA9V[\x92P\x92P\x92Pa\x19[\x82\x82a\"\xFEV[\x82\x93PPPP\x92\x91PPV[_a\x19r\x83\x83a$`V[\x90P\x92\x91PPV[a\x19\x82a\"QV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x19\xE6W`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x19\xF0\x82\x82a!BV[PPPV[`\x05T`\x08_\x82\x82Ta\x1A\x08\x91\x90a4*V[\x92PP\x81\x90UP\x81`\x05\x81\x90UP_`\x06\x81\x90UP_`\x07\x81\x90UP\x80`\t\x81\x90UP__\x90P[`\x05T\x81\x10\x15a\x1B\x8BW`\n_`\x03_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ __\x82\x01_\x90U`\x01\x82\x01_\x90UPP`\x04_`\x03_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x1B/\x91\x90a*tV[_`\x03_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80\x80`\x01\x01\x91PPa\x1A0V[P\x7F\xF7\xF0\x87#\x82\xDF\xF5\xE6\x98\xB2\x84\xE1 \x84\xE4\xE7\x89O\x83\x02\x16\xDD\x80\xCBN\x90\x9BY:X\xF95`\x05T3`@Qa\x1B\xBF\x92\x91\x90a8\xD6V[`@Q\x80\x91\x03\x90\xA1PPV[\x80`\x05\x81\x11\x15a\x1B\xDEWa\x1B\xDDa,:V[[`\x0F_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x05\x81\x11\x15a\x1B\xFFWa\x1B\xFEa,:V[[\x14a\x1CQW\x80`\x0F_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x7F\xBF\xA2\x17\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1CH\x92\x91\x90a8\xFDV[`@Q\x80\x91\x03\x90\xFD[PV[`\x01`\x0F_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x05\x81\x11\x15a\x1CwWa\x1Cva,:V[[a\x1C\x81\x91\x90a4*V[`\x05\x81\x11\x15a\x1C\x93Wa\x1C\x92a,:V[[`\x0F_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x1C\xB6Wa\x1C\xB5a,:V[[\x02\x17\x90UPV[\x80`\x0F_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x1C\xE1Wa\x1C\xE0a,:V[[\x02\x17\x90UPPV[a\x1C\xF1a\"QV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1D\x0Fa\x10\xFBV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1DnWa\x1D2a\"QV[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1De\x91\x90a/\x08V[`@Q\x80\x91\x03\x90\xFD[V[_`\x0B_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\x0B_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[_a\x1E@\x83_\x01\x83a$\xA3V[_\x1C\x90P\x92\x91PPV[``_a\x1EX\x83_\x01a$\xCAV[\x90P``\x81\x90P\x80\x92PPP\x91\x90PV[_`\x01\x83`\x03a\x1Ey\x91\x90a4\x8AV[a\x1E\x83\x91\x90a4*V[\x90P\x80\x82Q\x10\x15a\x1E\xCEW\x81Q\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1E\xC5\x92\x91\x90a4\xFEV[`@Q\x80\x91\x03\x90\xFD[\x82`\x02\x81\x90UP_a\x1E\xFF\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x11\xB8V[\x90P_a\x1F+\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x13\x07V[\x90P__\x90P[\x81\x81\x10\x15a\x1F\x8CWa\x1F~\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x84\x83\x81Q\x81\x10a\x1FqWa\x1Fpa4]V[[` \x02` \x01\x01Qa!BV[P\x80\x80`\x01\x01\x91PPa\x1F2V[P_a\x1F\xB7\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x11\xB8V[\x90P_a\x1F\xE3\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x13\x07V[\x90P__\x90P[\x81\x81\x10\x15a DWa 6\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x84\x83\x81Q\x81\x10a )Wa (a4]V[[` \x02` \x01\x01Qa!BV[P\x80\x80`\x01\x01\x91PPa\x1F\xEAV[P__\x90P[\x86Q\x81\x10\x15a \xA5Wa \x97\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x88\x83\x81Q\x81\x10a \x8AWa \x89a4]V[[` \x02` \x01\x01Qa\x19gV[P\x80\x80`\x01\x01\x91PPa JV[Pa \xEA\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x87_\x81Q\x81\x10a \xDDWa \xDCa4]V[[` \x02` \x01\x01Qa\x19gV[P\x7F\xA2\xDFx0\xE0\xBE\xDE\xF7\xB1\x11k\xF5G\xB4g\xB1kP\xB3\xBD#\x14l\x9E\t\x98x\xD1N\x890\x1A\x85\x883`@Qa!\x1E\x93\x92\x91\x90a9$V[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[_a!;\x82_\x01a%#V[\x90P\x91\x90PV[_a!M\x83\x83a%2V[\x90P\x92\x91PPV[\x81`\x0C\x81\x90UPB`\r\x81\x90UPC`\x0E\x81\x90UP_`\x0F_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a!\x8EWa!\x8Da,:V[[\x02\x17\x90UP\x7F\xDE\xF1\xF0\x8E\xB6U\xF4\xA7_`\xBDo\xD7\xE9q\x11 \x04\xAB\xD8F\xA6\x12\xE4m\xAB'\x07p\xD2L\xA50`\rT`\x0ET\x84`@Qa!\xCC\x94\x93\x92\x91\x90a9YV[`@Q\x80\x91\x03\x90\xA1PPV[_\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\"JWPa\"I\x82a%uV[[\x90P\x91\x90PV[_3\x90P\x90V[a\"b\x82\x82a\x11OV[a\"\xA5W\x80\x82`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\"\x9C\x92\x91\x90a9\x9CV[`@Q\x80\x91\x03\x90\xFD[PPV[___`A\x84Q\x03a\"\xE9W___` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90Pa\"\xDB\x88\x82\x85\x85a%\xDEV[\x95P\x95P\x95PPPPa\"\xF7V[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15a#\x11Wa#\x10a,:V[[\x82`\x03\x81\x11\x15a#$Wa##a,:V[[\x03\x15a$\\W`\x01`\x03\x81\x11\x15a#>Wa#=a,:V[[\x82`\x03\x81\x11\x15a#QWa#Pa,:V[[\x03a#\x88W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15a#\x9CWa#\x9Ba,:V[[\x82`\x03\x81\x11\x15a#\xAFWa#\xAEa,:V[[\x03a#\xF3W\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a#\xEA\x91\x90a,\xD5V[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15a$\x06Wa$\x05a,:V[[\x82`\x03\x81\x11\x15a$\x19Wa$\x18a,:V[[\x03a$[W\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a$R\x91\x90a.\x19V[`@Q\x80\x91\x03\x90\xFD[[PPV[__a$l\x84\x84a&\xC5V[\x90P\x80\x15a$\x99Wa$\x97\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a'\xAE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[_\x82_\x01\x82\x81T\x81\x10a$\xB9Wa$\xB8a4]V[[\x90_R` _ \x01T\x90P\x92\x91PPV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a%\x17W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a%\x03W[PPPPP\x90P\x91\x90PV[_\x81_\x01\x80T\x90P\x90P\x91\x90PV[__a%>\x84\x84a'\xDBV[\x90P\x80\x15a%kWa%i\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a(\xC4\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[_\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[___\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15a&\x1AW_`\x03\x85\x92P\x92P\x92Pa&\xBBV[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@Qa&=\x94\x93\x92\x91\x90a9\xDEV[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a&]W=__>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a&\xAEW_`\x01__\x1B\x93P\x93P\x93PPa&\xBBV[\x80___\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[_a&\xD0\x83\x83a\x11OV[a'\xA4W`\x01__\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa'Aa\"QV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa'\xA8V[_\x90P[\x92\x91PPV[_a'\xD3\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba(\xF1V[\x90P\x92\x91PPV[_a'\xE6\x83\x83a\x11OV[\x15a(\xBAW___\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa(Wa\"QV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa(\xBEV[_\x90P[\x92\x91PPV[_a(\xE9\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba)XV[\x90P\x92\x91PPV[_a(\xFC\x83\x83a*TV[a)NW\x82_\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x82_\x01\x80T\x90P\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP`\x01\x90Pa)RV[_\x90P[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x14a*IW_`\x01\x82a)\x85\x91\x90a4\xCBV[\x90P_`\x01\x86_\x01\x80T\x90Pa)\x9B\x91\x90a4\xCBV[\x90P\x80\x82\x14a*\x01W_\x86_\x01\x82\x81T\x81\x10a)\xBAWa)\xB9a4]V[[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a)\xDBWa)\xDAa4]V[[\x90_R` _ \x01\x81\x90UP\x83\x87`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UPP[\x85_\x01\x80T\x80a*\x14Wa*\x13a:!V[[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa*NV[_\x91PP[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x14\x15\x90P\x92\x91PPV[P_\x81U`\x01\x01_\x90UV[_`@Q\x90P\x90V[__\xFD[__\xFD[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a*\xC5\x81a*\x91V[\x81\x14a*\xCFW__\xFD[PV[_\x815\x90Pa*\xE0\x81a*\xBCV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a*\xFBWa*\xFAa*\x89V[[_a+\x08\x84\x82\x85\x01a*\xD2V[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a+%\x81a+\x11V[\x82RPPV[_` \x82\x01\x90Pa+>_\x83\x01\x84a+\x1CV[\x92\x91PPV[_\x81\x90P\x91\x90PV[a+V\x81a+DV[\x81\x14a+`W__\xFD[PV[_\x815\x90Pa+q\x81a+MV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a+\x8DWa+\x8Ca*\x89V[[_a+\x9A\x85\x82\x86\x01a+cV[\x92PP` a+\xAB\x85\x82\x86\x01a+cV[\x91PP\x92P\x92\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a+\xDE\x82a+\xB5V[\x90P\x91\x90PV[a+\xEE\x81a+\xD4V[\x81\x14a+\xF8W__\xFD[PV[_\x815\x90Pa,\t\x81a+\xE5V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a,$Wa,#a*\x89V[[_a,1\x84\x82\x85\x01a+\xFBV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x06\x81\x10a,xWa,wa,:V[[PV[_\x81\x90Pa,\x88\x82a,gV[\x91\x90PV[_a,\x97\x82a,{V[\x90P\x91\x90PV[a,\xA7\x81a,\x8DV[\x82RPPV[_` \x82\x01\x90Pa,\xC0_\x83\x01\x84a,\x9EV[\x92\x91PPV[a,\xCF\x81a+DV[\x82RPPV[_` \x82\x01\x90Pa,\xE8_\x83\x01\x84a,\xC6V[\x92\x91PPV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a-\x0FWa-\x0Ea,\xEEV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-,Wa-+a,\xF2V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a-HWa-Ga,\xF6V[[\x92P\x92\x90PV[___`@\x84\x86\x03\x12\x15a-fWa-ea*\x89V[[_a-s\x86\x82\x87\x01a+\xFBV[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-\x94Wa-\x93a*\x8DV[[a-\xA0\x86\x82\x87\x01a,\xFAV[\x92P\x92PP\x92P\x92P\x92V[_\x81\x90P\x91\x90PV[a-\xBE\x81a-\xACV[\x81\x14a-\xC8W__\xFD[PV[_\x815\x90Pa-\xD9\x81a-\xB5V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a-\xF4Wa-\xF3a*\x89V[[_a.\x01\x84\x82\x85\x01a-\xCBV[\x91PP\x92\x91PPV[a.\x13\x81a-\xACV[\x82RPPV[_` \x82\x01\x90Pa.,_\x83\x01\x84a.\nV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a.HWa.Ga*\x89V[[_a.U\x85\x82\x86\x01a-\xCBV[\x92PP` a.f\x85\x82\x86\x01a+\xFBV[\x91PP\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15a.\x86Wa.\x85a*\x89V[[_a.\x93\x85\x82\x86\x01a+\xFBV[\x92PP` a.\xA4\x85\x82\x86\x01a+\xFBV[\x91PP\x92P\x92\x90PV[__` \x83\x85\x03\x12\x15a.\xC4Wa.\xC3a*\x89V[[_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.\xE1Wa.\xE0a*\x8DV[[a.\xED\x85\x82\x86\x01a,\xFAV[\x92P\x92PP\x92P\x92\x90PV[a/\x02\x81a+\xD4V[\x82RPPV[_` \x82\x01\x90Pa/\x1B_\x83\x01\x84a.\xF9V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a/7Wa/6a*\x89V[[_a/D\x85\x82\x86\x01a-\xCBV[\x92PP` a/U\x85\x82\x86\x01a+cV[\x91PP\x92P\x92\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a/\x91\x81a+\xD4V[\x82RPPV[_a/\xA2\x83\x83a/\x88V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a/\xC4\x82a/_V[a/\xCE\x81\x85a/iV[\x93Pa/\xD9\x83a/yV[\x80_[\x83\x81\x10\x15a0\tW\x81Qa/\xF0\x88\x82a/\x97V[\x97Pa/\xFB\x83a/\xAEV[\x92PP`\x01\x81\x01\x90Pa/\xDCV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra0.\x81\x84a/\xBAV[\x90P\x92\x91PPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a0|\x82a06V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a0\x9BWa0\x9Aa0FV[[\x80`@RPPPV[_a0\xADa*\x80V[\x90Pa0\xB9\x82\x82a0sV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a0\xD8Wa0\xD7a0FV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a0\xFBa0\xF6\x84a0\xBEV[a0\xA4V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a1\x1EWa1\x1Da,\xF6V[[\x83[\x81\x81\x10\x15a1GW\x80a13\x88\x82a+\xFBV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa1 V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a1eWa1da,\xEEV[[\x815a1u\x84\x82` \x86\x01a0\xE9V[\x91PP\x92\x91PPV[__`@\x83\x85\x03\x12\x15a1\x94Wa1\x93a*\x89V[[_a1\xA1\x85\x82\x86\x01a+cV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\xC2Wa1\xC1a*\x8DV[[a1\xCE\x85\x82\x86\x01a1QV[\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a1\xEDWa1\xECa*\x89V[[_a1\xFA\x84\x82\x85\x01a+cV[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a25\x81a+DV[\x82RPPV[_a2F\x83\x83a2,V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a2h\x82a2\x03V[a2r\x81\x85a2\rV[\x93Pa2}\x83a2\x1DV[\x80_[\x83\x81\x10\x15a2\xADW\x81Qa2\x94\x88\x82a2;V[\x97Pa2\x9F\x83a2RV[\x92PP`\x01\x81\x01\x90Pa2\x80V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra2\xD2\x81\x84a2^V[\x90P\x92\x91PPV[____`\x80\x85\x87\x03\x12\x15a2\xF2Wa2\xF1a*\x89V[[_a2\xFF\x87\x82\x88\x01a-\xCBV[\x94PP` a3\x10\x87\x82\x88\x01a+cV[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a31Wa30a*\x8DV[[a3=\x87\x82\x88\x01a1QV[\x92PP``a3N\x87\x82\x88\x01a+cV[\x91PP\x92\x95\x91\x94P\x92PV[_`@\x82\x01\x90Pa3m_\x83\x01\x85a.\xF9V[a3z` \x83\x01\x84a,\xC6V[\x93\x92PPPV[_``\x82\x01\x90Pa3\x94_\x83\x01\x86a.\xF9V[a3\xA1` \x83\x01\x85a,\xC6V[a3\xAE`@\x83\x01\x84a,\xC6V[\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a3\xED\x82a+DV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a4\x1FWa4\x1Ea3\xB6V[[`\x01\x82\x01\x90P\x91\x90PV[_a44\x82a+DV[\x91Pa4?\x83a+DV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a4WWa4Va3\xB6V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_a4\x94\x82a+DV[\x91Pa4\x9F\x83a+DV[\x92P\x82\x82\x02a4\xAD\x81a+DV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a4\xC4Wa4\xC3a3\xB6V[[P\x92\x91PPV[_a4\xD5\x82a+DV[\x91Pa4\xE0\x83a+DV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a4\xF8Wa4\xF7a3\xB6V[[\x92\x91PPV[_`@\x82\x01\x90Pa5\x11_\x83\x01\x85a,\xC6V[a5\x1E` \x83\x01\x84a,\xC6V[\x93\x92PPPV[_\x82\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a5sW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a5\x86Wa5\x85a5/V[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a5\xE8\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a5\xADV[a5\xF2\x86\x83a5\xADV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a6-a6(a6#\x84a+DV[a6\nV[a+DV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a6F\x83a6\x13V[a6Za6R\x82a64V[\x84\x84Ta5\xB9V[\x82UPPPPV[__\x90P\x90V[a6qa6bV[a6|\x81\x84\x84a6=V[PPPV[_[\x82\x81\x10\x15a6\xA2Wa6\x97_\x82\x84\x01a6iV[`\x01\x81\x01\x90Pa6\x83V[PPPV[`\x1F\x82\x11\x15a6\xF5W\x82\x82\x11\x15a6\xF4Wa6\xC1\x81a5\x8CV[a6\xCA\x83a5\x9EV[a6\xD3\x85a5\x9EV[` \x86\x10\x15a6\xE0W_\x90P[\x80\x83\x01a6\xEF\x82\x84\x03\x82a6\x81V[PPPP[[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_a7\x15_\x19\x84`\x08\x02a6\xFAV[\x19\x80\x83\x16\x91PP\x92\x91PPV[_a7-\x83\x83a7\x06V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a7G\x83\x83a5%V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7`Wa7_a0FV[[a7j\x82Ta5\\V[a7u\x82\x82\x85a6\xA7V[_`\x1F\x83\x11`\x01\x81\x14a7\xA2W_\x84\x15a7\x90W\x82\x87\x015\x90P[a7\x9A\x85\x82a7\"V[\x86UPa8\x01V[`\x1F\x19\x84\x16a7\xB0\x86a5\x8CV[_[\x82\x81\x10\x15a7\xD7W\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa7\xB2V[\x86\x83\x10\x15a7\xF4W\x84\x89\x015a7\xF0`\x1F\x89\x16\x82a7\x06V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FCURRENTLY ONLY ONE INDEX PER CLI_\x82\x01R\x7FENT ALLOWED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a8t`+\x83a8\nV[\x91Pa8\x7F\x82a8\x1AV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra8\xA1\x81a8hV[\x90P\x91\x90PV[_`@\x82\x01\x90Pa8\xBB_\x83\x01\x85a.\xF9V[\x81\x81\x03` \x83\x01Ra8\xCD\x81\x84a2^V[\x90P\x93\x92PPPV[_`@\x82\x01\x90Pa8\xE9_\x83\x01\x85a,\xC6V[a8\xF6` \x83\x01\x84a.\xF9V[\x93\x92PPPV[_`@\x82\x01\x90Pa9\x10_\x83\x01\x85a,\x9EV[a9\x1D` \x83\x01\x84a,\x9EV[\x93\x92PPPV[_``\x82\x01\x90Pa97_\x83\x01\x86a,\xC6V[a9D` \x83\x01\x85a,\xC6V[a9Q`@\x83\x01\x84a.\xF9V[\x94\x93PPPPV[_`\x80\x82\x01\x90Pa9l_\x83\x01\x87a.\xF9V[a9y` \x83\x01\x86a,\xC6V[a9\x86`@\x83\x01\x85a,\xC6V[a9\x93``\x83\x01\x84a.\xF9V[\x95\x94PPPPPV[_`@\x82\x01\x90Pa9\xAF_\x83\x01\x85a.\xF9V[a9\xBC` \x83\x01\x84a.\nV[\x93\x92PPPV[_`\xFF\x82\x16\x90P\x91\x90PV[a9\xD8\x81a9\xC3V[\x82RPPV[_`\x80\x82\x01\x90Pa9\xF1_\x83\x01\x87a.\nV[a9\xFE` \x83\x01\x86a9\xCFV[a:\x0B`@\x83\x01\x85a.\nV[a:\x18``\x83\x01\x84a.\nV[\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 'uU\xB5\x1F\xF8\x97\xAEe\xA15j\xB2\xC1g\xF2I\xF5t(,c&vf\xC5R\x88K\xD3\xE6\xBEdsolcC\0\x08!\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b506004361061021a575f3560e01c8063715018a611610123578063c079f495116100ab578063d8270dce1161007a578063d8270dce146105a0578063ebae35e7146105be578063f2fde38b146105ee578063f6603c611461060a578063fc78b2e8146106265761021a565b8063c079f49514610540578063ca15c8731461054a578063cb9c4cc41461057a578063d547741f146105845761021a565b806391d14854116100f257806391d148541461049c578063a217fddf146104cc578063a3246ad3146104ea578063af206f281461051a578063bb51fef0146105365761021a565b8063715018a6146104265780637f35b560146104305780638da5cb5b1461044e5780639010d07c1461046c5761021a565b80632f2ff15d116101a65780634925e62a116101755780634925e62a146103d05780634b8e6488146103ec5780634bb278f3146103f6578063595f806e146104005780636b7447281461040a5761021a565b80632f2ff15d1461035e57806330104c3e1461037a57806336568abe146103985780633b4338d1146103b45761021a565b806317634514116101ed57806317634514146102b85780631c7453db146102d65780631ee4ee0f146102f45780632328bd1214610310578063248a9ca31461032e5761021a565b806301ffc9a71461021e5780630bda81cf1461024e57806313ff6dd51461026a578063146ca5311461029a575b5f5ffd5b61023860048036038101906102339190612ae6565b610656565b6040516102459190612b2b565b60405180910390f35b61026860048036038101906102639190612b77565b6106cf565b005b610284600480360381019061027f9190612c0f565b61082e565b6040516102919190612b2b565b60405180910390f35b6102a26108aa565b6040516102af9190612cad565b60405180910390f35b6102c06108bc565b6040516102cd9190612cd5565b60405180910390f35b6102de6108c2565b6040516102eb9190612cd5565b60405180910390f35b61030e60048036038101906103099190612d4f565b6108c8565b005b610318610cbe565b6040516103259190612cd5565b60405180910390f35b61034860048036038101906103439190612ddf565b610cd4565b6040516103559190612e19565b60405180910390f35b61037860048036038101906103739190612e32565b610cf0565b005b610382610d2a565b60405161038f9190612e19565b60405180910390f35b6103b260048036038101906103ad9190612e32565b610d4e565b005b6103ce60048036038101906103c99190612b77565b610e15565b005b6103ea60048036038101906103e59190612e70565b610e4e565b005b6103f4610f10565b005b6103fe610f8a565b005b610408611007565b005b610424600480360381019061041f9190612eae565b611081565b005b61042e6110c4565b005b6104386110d7565b6040516104459190612e19565b60405180910390f35b6104566110fb565b6040516104639190612f08565b60405180910390f35b61048660048036038101906104819190612f21565b611123565b6040516104939190612f08565b60405180910390f35b6104b660048036038101906104b19190612e32565b61114f565b6040516104c39190612b2b565b60405180910390f35b6104d46111b2565b6040516104e19190612e19565b60405180910390f35b61050460048036038101906104ff9190612ddf565b6111b8565b6040516105119190613016565b60405180910390f35b610534600480360381019061052f919061317e565b6111da565b005b61053e611213565b005b61054861128d565b005b610564600480360381019061055f9190612ddf565b611307565b6040516105719190612cd5565b60405180910390f35b610582611328565b005b61059e60048036038101906105999190612e32565b6113a1565b005b6105a8611494565b6040516105b59190612cd5565b60405180910390f35b6105d860048036038101906105d391906131d8565b61149a565b6040516105e591906132ba565b60405180910390f35b61060860048036038101906106039190612c0f565b61175e565b005b610624600480360381019061061f91906132da565b6117e2565b005b610640600480360381019061063b9190612c0f565b61184b565b60405161064d9190612b2b565b60405180910390f35b5f7f07effe0d000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191614806106c857506106c78261187d565b5b9050919050565b3373ffffffffffffffffffffffffffffffffffffffff1660035f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16146107715733816040517fffabbae700000000000000000000000000000000000000000000000000000000815260040161076892919061335a565b60405180910390fd5b604051806040016040528082815260200183815250600a5f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f820151815f0155602082015181600101559050507fb89addd937f44f902c849596641837cd7af2fcecef22d2a7866fdc1ad9c0ae2e33838360405161080b93929190613381565b60405180910390a160075f815480929190610825906133e3565b91905055505050565b5f6108388261184b565b61087957816040517fabdce06a0000000000000000000000000000000000000000000000000000000081526004016108709190612f08565b60405180910390fd5b6108a37f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e8361114f565b9050919050565b600f5f9054906101000a900460ff1681565b600e5481565b60085481565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696108f2816118f6565b5f60055490505f5f90505b60055481101561097f578573ffffffffffffffffffffffffffffffffffffffff1660035f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16036109725780915061097f565b80806001019150506108fd565b5060055481036109f65760035f5f81526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040517f6faf9f050000000000000000000000000000000000000000000000000000000081526004016109ed9190612f08565b60405180910390fd5b5f81600854610a05919061342a565b90505f610a3782604051602001610a1c9190612cd5565b6040516020818303038152906040528051906020012061190a565b90505f610a878288888080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505061193d565b905060045f8973ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f60028110610ad857610ad761345d565b5b015f8154610ae5906133e3565b919050819055508773ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603610b845760045f8973ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20600160028110610b6f57610b6e61345d565b5b015f8154610b7c906133e3565b919050819055505b60016009546002610b95919061348a565b610b9f919061342a565b60045f8a73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f60028110610bee57610bed61345d565b5b015403610cb4575f5f90506001600954610c08919061342a565b60045f8b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20600160028110610c5857610c5761345d565b5b015410610c6457600190505b8873ffffffffffffffffffffffffffffffffffffffff167f407009200520f9f10584813c0b95441ab320f6b08d97ebdaaf1e824eedd9d7c382604051610caa9190612b2b565b60405180910390a2505b5050505050505050565b5f600654600554610ccf91906134cb565b905090565b5f5f5f8381526020019081526020015f20600101549050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610d1a816118f6565b610d248383611967565b50505050565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46981565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46982148015610d835750610d82828261114f565b5b15610e07575f610d9283611307565b90505f60016002546003610da6919061348a565b610db0919061342a565b9050808203610e0457600181610dc691906134cb565b816040517f3a236268000000000000000000000000000000000000000000000000000000008152600401610dfb9291906134fe565b60405180910390fd5b50505b610e11828261197a565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610e3f816118f6565b610e4983836119f5565b505050565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469610e78816118f6565b600160106001015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550505050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610f3a816118f6565b6004610f4581611bcb565b7f301f8a3701f5b260197382dd7301078542144fe8fddd18083d6f6e09e4958a593342604051610f7692919061335a565b60405180910390a1610f86611c54565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610fb4816118f6565b6005610fbf81611bcb565b5f7f24a873651d26fb5a462cb37a91071cdd4d09abeebfe0ed144329bed1cc359d033342604051610ff192919061335a565b60405180910390a161100281611cbd565b505050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e611031816118f6565b600361103c81611bcb565b7f20f55ed0c92f2bb1c8825488e1e3c98463d024b2a42dbd24838c3f75260f43e9334260405161106d92919061335a565b60405180910390a161107d611c54565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6110ab816118f6565b828260105f0191826110be92919061373d565b50505050565b6110cc611ce9565b6110d55f611d70565b565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e81565b5f600b5f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b5f6111478260015f8681526020019081526020015f20611e3390919063ffffffff16565b905092915050565b5f5f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f5f1b81565b60606111d360015f8481526020019081526020015f20611e4a565b9050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e611204816118f6565b61120e8383611e69565b505050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61123d816118f6565b600261124881611bcb565b7f60edf9bdc7c4ea007cae1a9bbd03e41e5bfccd7231a6ec383c2edd7800f0d20c334260405161127992919061335a565b60405180910390a1611289611c54565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6112b7816118f6565b60016112c281611bcb565b7f67c4489f674d03c7d19a9e36735188de7c65e8d1e99eb3a2fd258a769eb14fff33426040516112f392919061335a565b60405180910390a1611303611c54565b5050565b5f61132160015f8481526020019081526020015f2061212f565b9050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e611352816118f6565b5f61135c81611bcb565b7fbb709dea744f06d1b26e824deec2f7140c511266ee15d7a217838b31d8b0123d334260405161138d92919061335a565b60405180910390a161139d611c54565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6113cb816118f6565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698314801561140057506113ff838361114f565b5b15611484575f61140f84611307565b90505f60016002546003611423919061348a565b61142d919061342a565b90508082036114815760018161144391906134cb565b816040517f3a2362680000000000000000000000000000000000000000000000000000000081526004016114789291906134fe565b60405180910390fd5b50505b61148e8383612142565b50505050565b600d5481565b6060600182146114df576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016114d69061388a565b60405180910390fd5b5f6006546005546114f091906134cb565b9050808311156115395782816040517fdf3d75e20000000000000000000000000000000000000000000000000000000081526004016115309291906134fe565b60405180910390fd5b5f5f90505b6005548110156115f5573373ffffffffffffffffffffffffffffffffffffffff1660035f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16036115e857336040517faca92f090000000000000000000000000000000000000000000000000000000081526004016115df9190612f08565b60405180910390fd5b808060010191505061153e565b505f600184600654611607919061342a565b61161191906134cb565b90505f8467ffffffffffffffff81111561162e5761162d613046565b5b60405190808252806020026020018201604052801561165c5781602001602082028036833780820191505090505b5090505f60065490505b826006541161170b573360035f60065481526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555060065482826006546116d291906134cb565b815181106116e3576116e261345d565b5b60200260200101818152505060065f815480929190611701906133e3565b9190505550611666565b858461171791906134cb565b93507f1e14abe5d0cdb96adde7b9eca9b14bc08df623b5805afde5a3f0acadc2bf4f5b338360405161174a9291906138a8565b60405180910390a181945050505050919050565b611766611ce9565b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16036117d6575f6040517f1e4fbdf70000000000000000000000000000000000000000000000000000000081526004016117cd9190612f08565b60405180910390fd5b6117df81611d70565b50565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61180c816118f6565b6118168484611e69565b61182082856119f5565b61184485845f815181106118375761183661345d565b5b6020026020010151612155565b5050505050565b5f6118767fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698361114f565b9050919050565b5f7f5a05180f000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191614806118ef57506118ee826121d8565b5b9050919050565b61190781611902612251565b612258565b50565b5f7f19457468657265756d205369676e6564204d6573736167653a0a3332000000005f5281601c52603c5f209050919050565b5f5f5f5f61194b86866122a9565b92509250925061195b82826122fe565b82935050505092915050565b5f6119728383612460565b905092915050565b611982612251565b73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16146119e6576040517f6697b23200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6119f08282612142565b505050565b60055460085f828254611a08919061342a565b92505081905550816005819055505f6006819055505f600781905550806009819055505f5f90505b600554811015611b8b57600a5f60035f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f5f82015f9055600182015f9055505060045f60035f8481526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f611b2f9190612a74565b5f60035f8381526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508080600101915050611a30565b507ff7f0872382dff5e698b284e12084e4e7894f830216dd80cb4e909b593a58f93560055433604051611bbf9291906138d6565b60405180910390a15050565b806005811115611bde57611bdd612c3a565b5b600f5f9054906101000a900460ff166005811115611bff57611bfe612c3a565b5b14611c515780600f5f9054906101000a900460ff166040517fbfa217d8000000000000000000000000000000000000000000000000000000008152600401611c489291906138fd565b60405180910390fd5b50565b6001600f5f9054906101000a900460ff166005811115611c7757611c76612c3a565b5b611c81919061342a565b6005811115611c9357611c92612c3a565b5b600f5f6101000a81548160ff02191690836005811115611cb657611cb5612c3a565b5b0217905550565b80600f5f6101000a81548160ff02191690836005811115611ce157611ce0612c3a565b5b021790555050565b611cf1612251565b73ffffffffffffffffffffffffffffffffffffffff16611d0f6110fb565b73ffffffffffffffffffffffffffffffffffffffff1614611d6e57611d32612251565b6040517f118cdaa7000000000000000000000000000000000000000000000000000000008152600401611d659190612f08565b60405180910390fd5b565b5f600b5f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905081600b5f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a35050565b5f611e40835f01836124a3565b5f1c905092915050565b60605f611e58835f016124ca565b905060608190508092505050919050565b5f6001836003611e79919061348a565b611e83919061342a565b90508082511015611ece578151816040517f3a236268000000000000000000000000000000000000000000000000000000008152600401611ec59291906134fe565b60405180910390fd5b826002819055505f611eff7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6111b8565b90505f611f2b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e611307565b90505f5f90505b81811015611f8c57611f7e7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e848381518110611f7157611f7061345d565b5b6020026020010151612142565b508080600101915050611f32565b505f611fb77fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696111b8565b90505f611fe37fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469611307565b90505f5f90505b81811015612044576120367fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698483815181106120295761202861345d565b5b6020026020010151612142565b508080600101915050611fea565b505f5f90505b86518110156120a5576120977fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46988838151811061208a5761208961345d565b5b6020026020010151611967565b50808060010191505061204a565b506120ea7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e875f815181106120dd576120dc61345d565b5b6020026020010151611967565b507fa2df7830e0bedef7b1116bf547b467b16b50b3bd23146c9e099878d14e89301a85883360405161211e93929190613924565b60405180910390a150505050505050565b5f61213b825f01612523565b9050919050565b5f61214d8383612532565b905092915050565b81600c8190555042600d8190555043600e819055505f600f5f6101000a81548160ff0219169083600581111561218e5761218d612c3a565b5b02179055507fdef1f08eb655f4a75f60bd6fd7e971112004abd846a612e46dab270770d24ca530600d54600e54846040516121cc9493929190613959565b60405180910390a15050565b5f7f7965db0b000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916148061224a575061224982612575565b5b9050919050565b5f33905090565b612262828261114f565b6122a55780826040517fe2517d3f00000000000000000000000000000000000000000000000000000000815260040161229c92919061399c565b60405180910390fd5b5050565b5f5f5f60418451036122e9575f5f5f602087015192506040870151915060608701515f1a90506122db888285856125de565b9550955095505050506122f7565b5f600285515f1b9250925092505b9250925092565b5f600381111561231157612310612c3a565b5b82600381111561232457612323612c3a565b5b031561245c576001600381111561233e5761233d612c3a565b5b82600381111561235157612350612c3a565b5b03612388576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6002600381111561239c5761239b612c3a565b5b8260038111156123af576123ae612c3a565b5b036123f357805f1c6040517ffce698f70000000000000000000000000000000000000000000000000000000081526004016123ea9190612cd5565b60405180910390fd5b60038081111561240657612405612c3a565b5b82600381111561241957612418612c3a565b5b0361245b57806040517fd78bce0c0000000000000000000000000000000000000000000000000000000081526004016124529190612e19565b60405180910390fd5b5b5050565b5f5f61246c84846126c5565b90508015612499576124978360015f8781526020019081526020015f206127ae90919063ffffffff16565b505b8091505092915050565b5f825f0182815481106124b9576124b861345d565b5b905f5260205f200154905092915050565b6060815f0180548060200260200160405190810160405280929190818152602001828054801561251757602002820191905f5260205f20905b815481526020019060010190808311612503575b50505050509050919050565b5f815f01805490509050919050565b5f5f61253e84846127db565b9050801561256b576125698360015f8781526020019081526020015f206128c490919063ffffffff16565b505b8091505092915050565b5f7f01ffc9a7000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916149050919050565b5f5f5f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c111561261a575f6003859250925092506126bb565b5f6001888888886040515f815260200160405260405161263d94939291906139de565b6020604051602081039080840390855afa15801561265d573d5f5f3e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16036126ae575f60015f5f1b935093509350506126bb565b805f5f5f1b935093509350505b9450945094915050565b5f6126d0838361114f565b6127a45760015f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550612741612251565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a4600190506127a8565b5f90505b92915050565b5f6127d3835f018373ffffffffffffffffffffffffffffffffffffffff165f1b6128f1565b905092915050565b5f6127e6838361114f565b156128ba575f5f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550612857612251565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b60405160405180910390a4600190506128be565b5f90505b92915050565b5f6128e9835f018373ffffffffffffffffffffffffffffffffffffffff165f1b612958565b905092915050565b5f6128fc8383612a54565b61294e57825f0182908060018154018082558091505060019003905f5260205f20015f9091909190915055825f0180549050836001015f8481526020019081526020015f208190555060019050612952565b5f90505b92915050565b5f5f836001015f8481526020019081526020015f205490505f8114612a49575f60018261298591906134cb565b90505f6001865f018054905061299b91906134cb565b9050808214612a01575f865f0182815481106129ba576129b961345d565b5b905f5260205f200154905080875f0184815481106129db576129da61345d565b5b905f5260205f20018190555083876001015f8381526020019081526020015f2081905550505b855f01805480612a1457612a13613a21565b5b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f905560019350505050612a4e565b5f9150505b92915050565b5f5f836001015f8481526020019081526020015f20541415905092915050565b505f81556001015f9055565b5f604051905090565b5f5ffd5b5f5ffd5b5f7fffffffff0000000000000000000000000000000000000000000000000000000082169050919050565b612ac581612a91565b8114612acf575f5ffd5b50565b5f81359050612ae081612abc565b92915050565b5f60208284031215612afb57612afa612a89565b5b5f612b0884828501612ad2565b91505092915050565b5f8115159050919050565b612b2581612b11565b82525050565b5f602082019050612b3e5f830184612b1c565b92915050565b5f819050919050565b612b5681612b44565b8114612b60575f5ffd5b50565b5f81359050612b7181612b4d565b92915050565b5f5f60408385031215612b8d57612b8c612a89565b5b5f612b9a85828601612b63565b9250506020612bab85828601612b63565b9150509250929050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f612bde82612bb5565b9050919050565b612bee81612bd4565b8114612bf8575f5ffd5b50565b5f81359050612c0981612be5565b92915050565b5f60208284031215612c2457612c23612a89565b5b5f612c3184828501612bfb565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b60068110612c7857612c77612c3a565b5b50565b5f819050612c8882612c67565b919050565b5f612c9782612c7b565b9050919050565b612ca781612c8d565b82525050565b5f602082019050612cc05f830184612c9e565b92915050565b612ccf81612b44565b82525050565b5f602082019050612ce85f830184612cc6565b92915050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f840112612d0f57612d0e612cee565b5b8235905067ffffffffffffffff811115612d2c57612d2b612cf2565b5b602083019150836001820283011115612d4857612d47612cf6565b5b9250929050565b5f5f5f60408486031215612d6657612d65612a89565b5b5f612d7386828701612bfb565b935050602084013567ffffffffffffffff811115612d9457612d93612a8d565b5b612da086828701612cfa565b92509250509250925092565b5f819050919050565b612dbe81612dac565b8114612dc8575f5ffd5b50565b5f81359050612dd981612db5565b92915050565b5f60208284031215612df457612df3612a89565b5b5f612e0184828501612dcb565b91505092915050565b612e1381612dac565b82525050565b5f602082019050612e2c5f830184612e0a565b92915050565b5f5f60408385031215612e4857612e47612a89565b5b5f612e5585828601612dcb565b9250506020612e6685828601612bfb565b9150509250929050565b5f5f60408385031215612e8657612e85612a89565b5b5f612e9385828601612bfb565b9250506020612ea485828601612bfb565b9150509250929050565b5f5f60208385031215612ec457612ec3612a89565b5b5f83013567ffffffffffffffff811115612ee157612ee0612a8d565b5b612eed85828601612cfa565b92509250509250929050565b612f0281612bd4565b82525050565b5f602082019050612f1b5f830184612ef9565b92915050565b5f5f60408385031215612f3757612f36612a89565b5b5f612f4485828601612dcb565b9250506020612f5585828601612b63565b9150509250929050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b612f9181612bd4565b82525050565b5f612fa28383612f88565b60208301905092915050565b5f602082019050919050565b5f612fc482612f5f565b612fce8185612f69565b9350612fd983612f79565b805f5b83811015613009578151612ff08882612f97565b9750612ffb83612fae565b925050600181019050612fdc565b5085935050505092915050565b5f6020820190508181035f83015261302e8184612fba565b905092915050565b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61307c82613036565b810181811067ffffffffffffffff8211171561309b5761309a613046565b5b80604052505050565b5f6130ad612a80565b90506130b98282613073565b919050565b5f67ffffffffffffffff8211156130d8576130d7613046565b5b602082029050602081019050919050565b5f6130fb6130f6846130be565b6130a4565b9050808382526020820190506020840283018581111561311e5761311d612cf6565b5b835b8181101561314757806131338882612bfb565b845260208401935050602081019050613120565b5050509392505050565b5f82601f83011261316557613164612cee565b5b81356131758482602086016130e9565b91505092915050565b5f5f6040838503121561319457613193612a89565b5b5f6131a185828601612b63565b925050602083013567ffffffffffffffff8111156131c2576131c1612a8d565b5b6131ce85828601613151565b9150509250929050565b5f602082840312156131ed576131ec612a89565b5b5f6131fa84828501612b63565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b61323581612b44565b82525050565b5f613246838361322c565b60208301905092915050565b5f602082019050919050565b5f61326882613203565b613272818561320d565b935061327d8361321d565b805f5b838110156132ad578151613294888261323b565b975061329f83613252565b925050600181019050613280565b5085935050505092915050565b5f6020820190508181035f8301526132d2818461325e565b905092915050565b5f5f5f5f608085870312156132f2576132f1612a89565b5b5f6132ff87828801612dcb565b945050602061331087828801612b63565b935050604085013567ffffffffffffffff81111561333157613330612a8d565b5b61333d87828801613151565b925050606061334e87828801612b63565b91505092959194509250565b5f60408201905061336d5f830185612ef9565b61337a6020830184612cc6565b9392505050565b5f6060820190506133945f830186612ef9565b6133a16020830185612cc6565b6133ae6040830184612cc6565b949350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6133ed82612b44565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff820361341f5761341e6133b6565b5b600182019050919050565b5f61343482612b44565b915061343f83612b44565b9250828201905080821115613457576134566133b6565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f61349482612b44565b915061349f83612b44565b92508282026134ad81612b44565b915082820484148315176134c4576134c36133b6565b5b5092915050565b5f6134d582612b44565b91506134e083612b44565b92508282039050818111156134f8576134f76133b6565b5b92915050565b5f6040820190506135115f830185612cc6565b61351e6020830184612cc6565b9392505050565b5f82905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061357357607f821691505b6020821081036135865761358561352f565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026135e87fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff826135ad565b6135f286836135ad565b95508019841693508086168417925050509392505050565b5f819050919050565b5f61362d61362861362384612b44565b61360a565b612b44565b9050919050565b5f819050919050565b61364683613613565b61365a61365282613634565b8484546135b9565b825550505050565b5f5f905090565b613671613662565b61367c81848461363d565b505050565b5f5b828110156136a2576136975f828401613669565b600181019050613683565b505050565b601f8211156136f557828211156136f4576136c18161358c565b6136ca8361359e565b6136d38561359e565b60208610156136e0575f90505b8083016136ef82840382613681565b505050505b5b505050565b5f82821c905092915050565b5f6137155f19846008026136fa565b1980831691505092915050565b5f61372d8383613706565b9150826002028217905092915050565b6137478383613525565b67ffffffffffffffff8111156137605761375f613046565b5b61376a825461355c565b6137758282856136a7565b5f601f8311600181146137a2575f8415613790578287013590505b61379a8582613722565b865550613801565b601f1984166137b08661358c565b5f5b828110156137d7578489013582556001820191506020850194506020810190506137b2565b868310156137f457848901356137f0601f891682613706565b8355505b6001600288020188555050505b50505050505050565b5f82825260208201905092915050565b7f43555252454e544c59204f4e4c59204f4e4520494e4445582050455220434c495f8201527f454e5420414c4c4f574544000000000000000000000000000000000000000000602082015250565b5f613874602b8361380a565b915061387f8261381a565b604082019050919050565b5f6020820190508181035f8301526138a181613868565b9050919050565b5f6040820190506138bb5f830185612ef9565b81810360208301526138cd818461325e565b90509392505050565b5f6040820190506138e95f830185612cc6565b6138f66020830184612ef9565b9392505050565b5f6040820190506139105f830185612c9e565b61391d6020830184612c9e565b9392505050565b5f6060820190506139375f830186612cc6565b6139446020830185612cc6565b6139516040830184612ef9565b949350505050565b5f60808201905061396c5f830187612ef9565b6139796020830186612cc6565b6139866040830185612cc6565b6139936060830184612ef9565b95945050505050565b5f6040820190506139af5f830185612ef9565b6139bc6020830184612e0a565b9392505050565b5f60ff82169050919050565b6139d8816139c3565b82525050565b5f6080820190506139f15f830187612e0a565b6139fe60208301866139cf565b613a0b6040830185612e0a565b613a186060830184612e0a565b95945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffdfea2646970667358221220277555b51ff897ae65a1356ab2c167f249f574282c63267666c552884bd3e6be64736f6c63430008210033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02\x1AW_5`\xE0\x1C\x80cqP\x18\xA6\x11a\x01#W\x80c\xC0y\xF4\x95\x11a\0\xABW\x80c\xD8'\r\xCE\x11a\0zW\x80c\xD8'\r\xCE\x14a\x05\xA0W\x80c\xEB\xAE5\xE7\x14a\x05\xBEW\x80c\xF2\xFD\xE3\x8B\x14a\x05\xEEW\x80c\xF6`<a\x14a\x06\nW\x80c\xFCx\xB2\xE8\x14a\x06&Wa\x02\x1AV[\x80c\xC0y\xF4\x95\x14a\x05@W\x80c\xCA\x15\xC8s\x14a\x05JW\x80c\xCB\x9CL\xC4\x14a\x05zW\x80c\xD5Gt\x1F\x14a\x05\x84Wa\x02\x1AV[\x80c\x91\xD1HT\x11a\0\xF2W\x80c\x91\xD1HT\x14a\x04\x9CW\x80c\xA2\x17\xFD\xDF\x14a\x04\xCCW\x80c\xA3$j\xD3\x14a\x04\xEAW\x80c\xAF o(\x14a\x05\x1AW\x80c\xBBQ\xFE\xF0\x14a\x056Wa\x02\x1AV[\x80cqP\x18\xA6\x14a\x04&W\x80c\x7F5\xB5`\x14a\x040W\x80c\x8D\xA5\xCB[\x14a\x04NW\x80c\x90\x10\xD0|\x14a\x04lWa\x02\x1AV[\x80c//\xF1]\x11a\x01\xA6W\x80cI%\xE6*\x11a\x01uW\x80cI%\xE6*\x14a\x03\xD0W\x80cK\x8Ed\x88\x14a\x03\xECW\x80cK\xB2x\xF3\x14a\x03\xF6W\x80cY_\x80n\x14a\x04\0W\x80cktG(\x14a\x04\nWa\x02\x1AV[\x80c//\xF1]\x14a\x03^W\x80c0\x10L>\x14a\x03zW\x80c6V\x8A\xBE\x14a\x03\x98W\x80c;C8\xD1\x14a\x03\xB4Wa\x02\x1AV[\x80c\x17cE\x14\x11a\x01\xEDW\x80c\x17cE\x14\x14a\x02\xB8W\x80c\x1CtS\xDB\x14a\x02\xD6W\x80c\x1E\xE4\xEE\x0F\x14a\x02\xF4W\x80c#(\xBD\x12\x14a\x03\x10W\x80c$\x8A\x9C\xA3\x14a\x03.Wa\x02\x1AV[\x80c\x01\xFF\xC9\xA7\x14a\x02\x1EW\x80c\x0B\xDA\x81\xCF\x14a\x02NW\x80c\x13\xFFm\xD5\x14a\x02jW\x80c\x14l\xA51\x14a\x02\x9AW[__\xFD[a\x028`\x04\x806\x03\x81\x01\x90a\x023\x91\x90a*\xE6V[a\x06VV[`@Qa\x02E\x91\x90a++V[`@Q\x80\x91\x03\x90\xF3[a\x02h`\x04\x806\x03\x81\x01\x90a\x02c\x91\x90a+wV[a\x06\xCFV[\0[a\x02\x84`\x04\x806\x03\x81\x01\x90a\x02\x7F\x91\x90a,\x0FV[a\x08.V[`@Qa\x02\x91\x91\x90a++V[`@Q\x80\x91\x03\x90\xF3[a\x02\xA2a\x08\xAAV[`@Qa\x02\xAF\x91\x90a,\xADV[`@Q\x80\x91\x03\x90\xF3[a\x02\xC0a\x08\xBCV[`@Qa\x02\xCD\x91\x90a,\xD5V[`@Q\x80\x91\x03\x90\xF3[a\x02\xDEa\x08\xC2V[`@Qa\x02\xEB\x91\x90a,\xD5V[`@Q\x80\x91\x03\x90\xF3[a\x03\x0E`\x04\x806\x03\x81\x01\x90a\x03\t\x91\x90a-OV[a\x08\xC8V[\0[a\x03\x18a\x0C\xBEV[`@Qa\x03%\x91\x90a,\xD5V[`@Q\x80\x91\x03\x90\xF3[a\x03H`\x04\x806\x03\x81\x01\x90a\x03C\x91\x90a-\xDFV[a\x0C\xD4V[`@Qa\x03U\x91\x90a.\x19V[`@Q\x80\x91\x03\x90\xF3[a\x03x`\x04\x806\x03\x81\x01\x90a\x03s\x91\x90a.2V[a\x0C\xF0V[\0[a\x03\x82a\r*V[`@Qa\x03\x8F\x91\x90a.\x19V[`@Q\x80\x91\x03\x90\xF3[a\x03\xB2`\x04\x806\x03\x81\x01\x90a\x03\xAD\x91\x90a.2V[a\rNV[\0[a\x03\xCE`\x04\x806\x03\x81\x01\x90a\x03\xC9\x91\x90a+wV[a\x0E\x15V[\0[a\x03\xEA`\x04\x806\x03\x81\x01\x90a\x03\xE5\x91\x90a.pV[a\x0ENV[\0[a\x03\xF4a\x0F\x10V[\0[a\x03\xFEa\x0F\x8AV[\0[a\x04\x08a\x10\x07V[\0[a\x04$`\x04\x806\x03\x81\x01\x90a\x04\x1F\x91\x90a.\xAEV[a\x10\x81V[\0[a\x04.a\x10\xC4V[\0[a\x048a\x10\xD7V[`@Qa\x04E\x91\x90a.\x19V[`@Q\x80\x91\x03\x90\xF3[a\x04Va\x10\xFBV[`@Qa\x04c\x91\x90a/\x08V[`@Q\x80\x91\x03\x90\xF3[a\x04\x86`\x04\x806\x03\x81\x01\x90a\x04\x81\x91\x90a/!V[a\x11#V[`@Qa\x04\x93\x91\x90a/\x08V[`@Q\x80\x91\x03\x90\xF3[a\x04\xB6`\x04\x806\x03\x81\x01\x90a\x04\xB1\x91\x90a.2V[a\x11OV[`@Qa\x04\xC3\x91\x90a++V[`@Q\x80\x91\x03\x90\xF3[a\x04\xD4a\x11\xB2V[`@Qa\x04\xE1\x91\x90a.\x19V[`@Q\x80\x91\x03\x90\xF3[a\x05\x04`\x04\x806\x03\x81\x01\x90a\x04\xFF\x91\x90a-\xDFV[a\x11\xB8V[`@Qa\x05\x11\x91\x90a0\x16V[`@Q\x80\x91\x03\x90\xF3[a\x054`\x04\x806\x03\x81\x01\x90a\x05/\x91\x90a1~V[a\x11\xDAV[\0[a\x05>a\x12\x13V[\0[a\x05Ha\x12\x8DV[\0[a\x05d`\x04\x806\x03\x81\x01\x90a\x05_\x91\x90a-\xDFV[a\x13\x07V[`@Qa\x05q\x91\x90a,\xD5V[`@Q\x80\x91\x03\x90\xF3[a\x05\x82a\x13(V[\0[a\x05\x9E`\x04\x806\x03\x81\x01\x90a\x05\x99\x91\x90a.2V[a\x13\xA1V[\0[a\x05\xA8a\x14\x94V[`@Qa\x05\xB5\x91\x90a,\xD5V[`@Q\x80\x91\x03\x90\xF3[a\x05\xD8`\x04\x806\x03\x81\x01\x90a\x05\xD3\x91\x90a1\xD8V[a\x14\x9AV[`@Qa\x05\xE5\x91\x90a2\xBAV[`@Q\x80\x91\x03\x90\xF3[a\x06\x08`\x04\x806\x03\x81\x01\x90a\x06\x03\x91\x90a,\x0FV[a\x17^V[\0[a\x06$`\x04\x806\x03\x81\x01\x90a\x06\x1F\x91\x90a2\xDAV[a\x17\xE2V[\0[a\x06@`\x04\x806\x03\x81\x01\x90a\x06;\x91\x90a,\x0FV[a\x18KV[`@Qa\x06M\x91\x90a++V[`@Q\x80\x91\x03\x90\xF3[_\x7F\x07\xEF\xFE\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x06\xC8WPa\x06\xC7\x82a\x18}V[[\x90P\x91\x90PV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07qW3\x81`@Q\x7F\xFF\xAB\xBA\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07h\x92\x91\x90a3ZV[`@Q\x80\x91\x03\x90\xFD[`@Q\x80`@\x01`@R\x80\x82\x81R` \x01\x83\x81RP`\n_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01U` \x82\x01Q\x81`\x01\x01U\x90PP\x7F\xB8\x9A\xDD\xD97\xF4O\x90,\x84\x95\x96d\x187\xCDz\xF2\xFC\xEC\xEF\"\xD2\xA7\x86o\xDC\x1A\xD9\xC0\xAE.3\x83\x83`@Qa\x08\x0B\x93\x92\x91\x90a3\x81V[`@Q\x80\x91\x03\x90\xA1`\x07_\x81T\x80\x92\x91\x90a\x08%\x90a3\xE3V[\x91\x90PUPPPV[_a\x088\x82a\x18KV[a\x08yW\x81`@Q\x7F\xAB\xDC\xE0j\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08p\x91\x90a/\x08V[`@Q\x80\x91\x03\x90\xFD[a\x08\xA3\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x83a\x11OV[\x90P\x91\x90PV[`\x0F_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\x0ET\x81V[`\x08T\x81V[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x08\xF2\x81a\x18\xF6V[_`\x05T\x90P__\x90P[`\x05T\x81\x10\x15a\t\x7FW\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\trW\x80\x91Pa\t\x7FV[\x80\x80`\x01\x01\x91PPa\x08\xFDV[P`\x05T\x81\x03a\t\xF6W`\x03__\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x7Fo\xAF\x9F\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xED\x91\x90a/\x08V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x08Ta\n\x05\x91\x90a4*V[\x90P_a\n7\x82`@Q` \x01a\n\x1C\x91\x90a,\xD5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x19\nV[\x90P_a\n\x87\x82\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa\x19=V[\x90P`\x04_\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _`\x02\x81\x10a\n\xD8Wa\n\xD7a4]V[[\x01_\x81Ta\n\xE5\x90a3\xE3V[\x91\x90P\x81\x90UP\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0B\x84W`\x04_\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01`\x02\x81\x10a\x0BoWa\x0Bna4]V[[\x01_\x81Ta\x0B|\x90a3\xE3V[\x91\x90P\x81\x90UP[`\x01`\tT`\x02a\x0B\x95\x91\x90a4\x8AV[a\x0B\x9F\x91\x90a4*V[`\x04_\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _`\x02\x81\x10a\x0B\xEEWa\x0B\xEDa4]V[[\x01T\x03a\x0C\xB4W__\x90P`\x01`\tTa\x0C\x08\x91\x90a4*V[`\x04_\x8Bs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01`\x02\x81\x10a\x0CXWa\x0CWa4]V[[\x01T\x10a\x0CdW`\x01\x90P[\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F@p\t \x05 \xF9\xF1\x05\x84\x81<\x0B\x95D\x1A\xB3 \xF6\xB0\x8D\x97\xEB\xDA\xAF\x1E\x82N\xED\xD9\xD7\xC3\x82`@Qa\x0C\xAA\x91\x90a++V[`@Q\x80\x91\x03\x90\xA2P[PPPPPPPPV[_`\x06T`\x05Ta\x0C\xCF\x91\x90a4\xCBV[\x90P\x90V[___\x83\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\r\x1A\x81a\x18\xF6V[a\r$\x83\x83a\x19gV[PPPPV[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x81V[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x82\x14\x80\x15a\r\x83WPa\r\x82\x82\x82a\x11OV[[\x15a\x0E\x07W_a\r\x92\x83a\x13\x07V[\x90P_`\x01`\x02T`\x03a\r\xA6\x91\x90a4\x8AV[a\r\xB0\x91\x90a4*V[\x90P\x80\x82\x03a\x0E\x04W`\x01\x81a\r\xC6\x91\x90a4\xCBV[\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\xFB\x92\x91\x90a4\xFEV[`@Q\x80\x91\x03\x90\xFD[PP[a\x0E\x11\x82\x82a\x19zV[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0E?\x81a\x18\xF6V[a\x0EI\x83\x83a\x19\xF5V[PPPV[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x0Ex\x81a\x18\xF6V[`\x01`\x10`\x01\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPPPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0F:\x81a\x18\xF6V[`\x04a\x0FE\x81a\x1B\xCBV[\x7F0\x1F\x8A7\x01\xF5\xB2`\x19s\x82\xDDs\x01\x07\x85B\x14O\xE8\xFD\xDD\x18\x08=on\t\xE4\x95\x8AY3B`@Qa\x0Fv\x92\x91\x90a3ZV[`@Q\x80\x91\x03\x90\xA1a\x0F\x86a\x1CTV[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0F\xB4\x81a\x18\xF6V[`\x05a\x0F\xBF\x81a\x1B\xCBV[_\x7F$\xA8se\x1D&\xFBZF,\xB3z\x91\x07\x1C\xDDM\t\xAB\xEE\xBF\xE0\xED\x14C)\xBE\xD1\xCC5\x9D\x033B`@Qa\x0F\xF1\x92\x91\x90a3ZV[`@Q\x80\x91\x03\x90\xA1a\x10\x02\x81a\x1C\xBDV[PPPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x101\x81a\x18\xF6V[`\x03a\x10<\x81a\x1B\xCBV[\x7F \xF5^\xD0\xC9/+\xB1\xC8\x82T\x88\xE1\xE3\xC9\x84c\xD0$\xB2\xA4-\xBD$\x83\x8C?u&\x0FC\xE93B`@Qa\x10m\x92\x91\x90a3ZV[`@Q\x80\x91\x03\x90\xA1a\x10}a\x1CTV[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x10\xAB\x81a\x18\xF6V[\x82\x82`\x10_\x01\x91\x82a\x10\xBE\x92\x91\x90a7=V[PPPPV[a\x10\xCCa\x1C\xE9V[a\x10\xD5_a\x1DpV[V[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x81V[_`\x0B_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_a\x11G\x82`\x01_\x86\x81R` \x01\x90\x81R` \x01_ a\x1E3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[___\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[__\x1B\x81V[``a\x11\xD3`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a\x1EJV[\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x12\x04\x81a\x18\xF6V[a\x12\x0E\x83\x83a\x1EiV[PPPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x12=\x81a\x18\xF6V[`\x02a\x12H\x81a\x1B\xCBV[\x7F`\xED\xF9\xBD\xC7\xC4\xEA\0|\xAE\x1A\x9B\xBD\x03\xE4\x1E[\xFC\xCDr1\xA6\xEC8<.\xDDx\0\xF0\xD2\x0C3B`@Qa\x12y\x92\x91\x90a3ZV[`@Q\x80\x91\x03\x90\xA1a\x12\x89a\x1CTV[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x12\xB7\x81a\x18\xF6V[`\x01a\x12\xC2\x81a\x1B\xCBV[\x7Fg\xC4H\x9FgM\x03\xC7\xD1\x9A\x9E6sQ\x88\xDE|e\xE8\xD1\xE9\x9E\xB3\xA2\xFD%\x8Av\x9E\xB1O\xFF3B`@Qa\x12\xF3\x92\x91\x90a3ZV[`@Q\x80\x91\x03\x90\xA1a\x13\x03a\x1CTV[PPV[_a\x13!`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a!/V[\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x13R\x81a\x18\xF6V[_a\x13\\\x81a\x1B\xCBV[\x7F\xBBp\x9D\xEAtO\x06\xD1\xB2n\x82M\xEE\xC2\xF7\x14\x0CQ\x12f\xEE\x15\xD7\xA2\x17\x83\x8B1\xD8\xB0\x12=3B`@Qa\x13\x8D\x92\x91\x90a3ZV[`@Q\x80\x91\x03\x90\xA1a\x13\x9Da\x1CTV[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x13\xCB\x81a\x18\xF6V[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x83\x14\x80\x15a\x14\0WPa\x13\xFF\x83\x83a\x11OV[[\x15a\x14\x84W_a\x14\x0F\x84a\x13\x07V[\x90P_`\x01`\x02T`\x03a\x14#\x91\x90a4\x8AV[a\x14-\x91\x90a4*V[\x90P\x80\x82\x03a\x14\x81W`\x01\x81a\x14C\x91\x90a4\xCBV[\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14x\x92\x91\x90a4\xFEV[`@Q\x80\x91\x03\x90\xFD[PP[a\x14\x8E\x83\x83a!BV[PPPPV[`\rT\x81V[```\x01\x82\x14a\x14\xDFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14\xD6\x90a8\x8AV[`@Q\x80\x91\x03\x90\xFD[_`\x06T`\x05Ta\x14\xF0\x91\x90a4\xCBV[\x90P\x80\x83\x11\x15a\x159W\x82\x81`@Q\x7F\xDF=u\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x150\x92\x91\x90a4\xFEV[`@Q\x80\x91\x03\x90\xFD[__\x90P[`\x05T\x81\x10\x15a\x15\xF5W3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x15\xE8W3`@Q\x7F\xAC\xA9/\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15\xDF\x91\x90a/\x08V[`@Q\x80\x91\x03\x90\xFD[\x80\x80`\x01\x01\x91PPa\x15>V[P_`\x01\x84`\x06Ta\x16\x07\x91\x90a4*V[a\x16\x11\x91\x90a4\xCBV[\x90P_\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16.Wa\x16-a0FV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\\W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_`\x06T\x90P[\x82`\x06T\x11a\x17\x0BW3`\x03_`\x06T\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x06T\x82\x82`\x06Ta\x16\xD2\x91\x90a4\xCBV[\x81Q\x81\x10a\x16\xE3Wa\x16\xE2a4]V[[` \x02` \x01\x01\x81\x81RPP`\x06_\x81T\x80\x92\x91\x90a\x17\x01\x90a3\xE3V[\x91\x90PUPa\x16fV[\x85\x84a\x17\x17\x91\x90a4\xCBV[\x93P\x7F\x1E\x14\xAB\xE5\xD0\xCD\xB9j\xDD\xE7\xB9\xEC\xA9\xB1K\xC0\x8D\xF6#\xB5\x80Z\xFD\xE5\xA3\xF0\xAC\xAD\xC2\xBFO[3\x83`@Qa\x17J\x92\x91\x90a8\xA8V[`@Q\x80\x91\x03\x90\xA1\x81\x94PPPPP\x91\x90PV[a\x17fa\x1C\xE9V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x17\xD6W_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17\xCD\x91\x90a/\x08V[`@Q\x80\x91\x03\x90\xFD[a\x17\xDF\x81a\x1DpV[PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x18\x0C\x81a\x18\xF6V[a\x18\x16\x84\x84a\x1EiV[a\x18 \x82\x85a\x19\xF5V[a\x18D\x85\x84_\x81Q\x81\x10a\x187Wa\x186a4]V[[` \x02` \x01\x01Qa!UV[PPPPPV[_a\x18v\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x83a\x11OV[\x90P\x91\x90PV[_\x7FZ\x05\x18\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x18\xEFWPa\x18\xEE\x82a!\xD8V[[\x90P\x91\x90PV[a\x19\x07\x81a\x19\x02a\"QV[a\"XV[PV[_\x7F\x19Ethereum Signed Message:\n32\0\0\0\0_R\x81`\x1CR`<_ \x90P\x91\x90PV[____a\x19K\x86\x86a\"\xA9V[\x92P\x92P\x92Pa\x19[\x82\x82a\"\xFEV[\x82\x93PPPP\x92\x91PPV[_a\x19r\x83\x83a$`V[\x90P\x92\x91PPV[a\x19\x82a\"QV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x19\xE6W`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x19\xF0\x82\x82a!BV[PPPV[`\x05T`\x08_\x82\x82Ta\x1A\x08\x91\x90a4*V[\x92PP\x81\x90UP\x81`\x05\x81\x90UP_`\x06\x81\x90UP_`\x07\x81\x90UP\x80`\t\x81\x90UP__\x90P[`\x05T\x81\x10\x15a\x1B\x8BW`\n_`\x03_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ __\x82\x01_\x90U`\x01\x82\x01_\x90UPP`\x04_`\x03_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x1B/\x91\x90a*tV[_`\x03_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80\x80`\x01\x01\x91PPa\x1A0V[P\x7F\xF7\xF0\x87#\x82\xDF\xF5\xE6\x98\xB2\x84\xE1 \x84\xE4\xE7\x89O\x83\x02\x16\xDD\x80\xCBN\x90\x9BY:X\xF95`\x05T3`@Qa\x1B\xBF\x92\x91\x90a8\xD6V[`@Q\x80\x91\x03\x90\xA1PPV[\x80`\x05\x81\x11\x15a\x1B\xDEWa\x1B\xDDa,:V[[`\x0F_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x05\x81\x11\x15a\x1B\xFFWa\x1B\xFEa,:V[[\x14a\x1CQW\x80`\x0F_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x7F\xBF\xA2\x17\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1CH\x92\x91\x90a8\xFDV[`@Q\x80\x91\x03\x90\xFD[PV[`\x01`\x0F_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x05\x81\x11\x15a\x1CwWa\x1Cva,:V[[a\x1C\x81\x91\x90a4*V[`\x05\x81\x11\x15a\x1C\x93Wa\x1C\x92a,:V[[`\x0F_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x1C\xB6Wa\x1C\xB5a,:V[[\x02\x17\x90UPV[\x80`\x0F_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x1C\xE1Wa\x1C\xE0a,:V[[\x02\x17\x90UPPV[a\x1C\xF1a\"QV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1D\x0Fa\x10\xFBV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1DnWa\x1D2a\"QV[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1De\x91\x90a/\x08V[`@Q\x80\x91\x03\x90\xFD[V[_`\x0B_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\x0B_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[_a\x1E@\x83_\x01\x83a$\xA3V[_\x1C\x90P\x92\x91PPV[``_a\x1EX\x83_\x01a$\xCAV[\x90P``\x81\x90P\x80\x92PPP\x91\x90PV[_`\x01\x83`\x03a\x1Ey\x91\x90a4\x8AV[a\x1E\x83\x91\x90a4*V[\x90P\x80\x82Q\x10\x15a\x1E\xCEW\x81Q\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1E\xC5\x92\x91\x90a4\xFEV[`@Q\x80\x91\x03\x90\xFD[\x82`\x02\x81\x90UP_a\x1E\xFF\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x11\xB8V[\x90P_a\x1F+\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x13\x07V[\x90P__\x90P[\x81\x81\x10\x15a\x1F\x8CWa\x1F~\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x84\x83\x81Q\x81\x10a\x1FqWa\x1Fpa4]V[[` \x02` \x01\x01Qa!BV[P\x80\x80`\x01\x01\x91PPa\x1F2V[P_a\x1F\xB7\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x11\xB8V[\x90P_a\x1F\xE3\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x13\x07V[\x90P__\x90P[\x81\x81\x10\x15a DWa 6\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x84\x83\x81Q\x81\x10a )Wa (a4]V[[` \x02` \x01\x01Qa!BV[P\x80\x80`\x01\x01\x91PPa\x1F\xEAV[P__\x90P[\x86Q\x81\x10\x15a \xA5Wa \x97\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x88\x83\x81Q\x81\x10a \x8AWa \x89a4]V[[` \x02` \x01\x01Qa\x19gV[P\x80\x80`\x01\x01\x91PPa JV[Pa \xEA\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x87_\x81Q\x81\x10a \xDDWa \xDCa4]V[[` \x02` \x01\x01Qa\x19gV[P\x7F\xA2\xDFx0\xE0\xBE\xDE\xF7\xB1\x11k\xF5G\xB4g\xB1kP\xB3\xBD#\x14l\x9E\t\x98x\xD1N\x890\x1A\x85\x883`@Qa!\x1E\x93\x92\x91\x90a9$V[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[_a!;\x82_\x01a%#V[\x90P\x91\x90PV[_a!M\x83\x83a%2V[\x90P\x92\x91PPV[\x81`\x0C\x81\x90UPB`\r\x81\x90UPC`\x0E\x81\x90UP_`\x0F_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a!\x8EWa!\x8Da,:V[[\x02\x17\x90UP\x7F\xDE\xF1\xF0\x8E\xB6U\xF4\xA7_`\xBDo\xD7\xE9q\x11 \x04\xAB\xD8F\xA6\x12\xE4m\xAB'\x07p\xD2L\xA50`\rT`\x0ET\x84`@Qa!\xCC\x94\x93\x92\x91\x90a9YV[`@Q\x80\x91\x03\x90\xA1PPV[_\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\"JWPa\"I\x82a%uV[[\x90P\x91\x90PV[_3\x90P\x90V[a\"b\x82\x82a\x11OV[a\"\xA5W\x80\x82`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\"\x9C\x92\x91\x90a9\x9CV[`@Q\x80\x91\x03\x90\xFD[PPV[___`A\x84Q\x03a\"\xE9W___` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90Pa\"\xDB\x88\x82\x85\x85a%\xDEV[\x95P\x95P\x95PPPPa\"\xF7V[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15a#\x11Wa#\x10a,:V[[\x82`\x03\x81\x11\x15a#$Wa##a,:V[[\x03\x15a$\\W`\x01`\x03\x81\x11\x15a#>Wa#=a,:V[[\x82`\x03\x81\x11\x15a#QWa#Pa,:V[[\x03a#\x88W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15a#\x9CWa#\x9Ba,:V[[\x82`\x03\x81\x11\x15a#\xAFWa#\xAEa,:V[[\x03a#\xF3W\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a#\xEA\x91\x90a,\xD5V[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15a$\x06Wa$\x05a,:V[[\x82`\x03\x81\x11\x15a$\x19Wa$\x18a,:V[[\x03a$[W\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a$R\x91\x90a.\x19V[`@Q\x80\x91\x03\x90\xFD[[PPV[__a$l\x84\x84a&\xC5V[\x90P\x80\x15a$\x99Wa$\x97\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a'\xAE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[_\x82_\x01\x82\x81T\x81\x10a$\xB9Wa$\xB8a4]V[[\x90_R` _ \x01T\x90P\x92\x91PPV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a%\x17W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a%\x03W[PPPPP\x90P\x91\x90PV[_\x81_\x01\x80T\x90P\x90P\x91\x90PV[__a%>\x84\x84a'\xDBV[\x90P\x80\x15a%kWa%i\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a(\xC4\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[_\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[___\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15a&\x1AW_`\x03\x85\x92P\x92P\x92Pa&\xBBV[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@Qa&=\x94\x93\x92\x91\x90a9\xDEV[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a&]W=__>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a&\xAEW_`\x01__\x1B\x93P\x93P\x93PPa&\xBBV[\x80___\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[_a&\xD0\x83\x83a\x11OV[a'\xA4W`\x01__\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa'Aa\"QV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa'\xA8V[_\x90P[\x92\x91PPV[_a'\xD3\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba(\xF1V[\x90P\x92\x91PPV[_a'\xE6\x83\x83a\x11OV[\x15a(\xBAW___\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa(Wa\"QV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa(\xBEV[_\x90P[\x92\x91PPV[_a(\xE9\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba)XV[\x90P\x92\x91PPV[_a(\xFC\x83\x83a*TV[a)NW\x82_\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x82_\x01\x80T\x90P\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP`\x01\x90Pa)RV[_\x90P[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x14a*IW_`\x01\x82a)\x85\x91\x90a4\xCBV[\x90P_`\x01\x86_\x01\x80T\x90Pa)\x9B\x91\x90a4\xCBV[\x90P\x80\x82\x14a*\x01W_\x86_\x01\x82\x81T\x81\x10a)\xBAWa)\xB9a4]V[[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a)\xDBWa)\xDAa4]V[[\x90_R` _ \x01\x81\x90UP\x83\x87`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UPP[\x85_\x01\x80T\x80a*\x14Wa*\x13a:!V[[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa*NV[_\x91PP[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x14\x15\x90P\x92\x91PPV[P_\x81U`\x01\x01_\x90UV[_`@Q\x90P\x90V[__\xFD[__\xFD[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a*\xC5\x81a*\x91V[\x81\x14a*\xCFW__\xFD[PV[_\x815\x90Pa*\xE0\x81a*\xBCV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a*\xFBWa*\xFAa*\x89V[[_a+\x08\x84\x82\x85\x01a*\xD2V[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a+%\x81a+\x11V[\x82RPPV[_` \x82\x01\x90Pa+>_\x83\x01\x84a+\x1CV[\x92\x91PPV[_\x81\x90P\x91\x90PV[a+V\x81a+DV[\x81\x14a+`W__\xFD[PV[_\x815\x90Pa+q\x81a+MV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a+\x8DWa+\x8Ca*\x89V[[_a+\x9A\x85\x82\x86\x01a+cV[\x92PP` a+\xAB\x85\x82\x86\x01a+cV[\x91PP\x92P\x92\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a+\xDE\x82a+\xB5V[\x90P\x91\x90PV[a+\xEE\x81a+\xD4V[\x81\x14a+\xF8W__\xFD[PV[_\x815\x90Pa,\t\x81a+\xE5V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a,$Wa,#a*\x89V[[_a,1\x84\x82\x85\x01a+\xFBV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x06\x81\x10a,xWa,wa,:V[[PV[_\x81\x90Pa,\x88\x82a,gV[\x91\x90PV[_a,\x97\x82a,{V[\x90P\x91\x90PV[a,\xA7\x81a,\x8DV[\x82RPPV[_` \x82\x01\x90Pa,\xC0_\x83\x01\x84a,\x9EV[\x92\x91PPV[a,\xCF\x81a+DV[\x82RPPV[_` \x82\x01\x90Pa,\xE8_\x83\x01\x84a,\xC6V[\x92\x91PPV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a-\x0FWa-\x0Ea,\xEEV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-,Wa-+a,\xF2V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a-HWa-Ga,\xF6V[[\x92P\x92\x90PV[___`@\x84\x86\x03\x12\x15a-fWa-ea*\x89V[[_a-s\x86\x82\x87\x01a+\xFBV[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-\x94Wa-\x93a*\x8DV[[a-\xA0\x86\x82\x87\x01a,\xFAV[\x92P\x92PP\x92P\x92P\x92V[_\x81\x90P\x91\x90PV[a-\xBE\x81a-\xACV[\x81\x14a-\xC8W__\xFD[PV[_\x815\x90Pa-\xD9\x81a-\xB5V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a-\xF4Wa-\xF3a*\x89V[[_a.\x01\x84\x82\x85\x01a-\xCBV[\x91PP\x92\x91PPV[a.\x13\x81a-\xACV[\x82RPPV[_` \x82\x01\x90Pa.,_\x83\x01\x84a.\nV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a.HWa.Ga*\x89V[[_a.U\x85\x82\x86\x01a-\xCBV[\x92PP` a.f\x85\x82\x86\x01a+\xFBV[\x91PP\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15a.\x86Wa.\x85a*\x89V[[_a.\x93\x85\x82\x86\x01a+\xFBV[\x92PP` a.\xA4\x85\x82\x86\x01a+\xFBV[\x91PP\x92P\x92\x90PV[__` \x83\x85\x03\x12\x15a.\xC4Wa.\xC3a*\x89V[[_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.\xE1Wa.\xE0a*\x8DV[[a.\xED\x85\x82\x86\x01a,\xFAV[\x92P\x92PP\x92P\x92\x90PV[a/\x02\x81a+\xD4V[\x82RPPV[_` \x82\x01\x90Pa/\x1B_\x83\x01\x84a.\xF9V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a/7Wa/6a*\x89V[[_a/D\x85\x82\x86\x01a-\xCBV[\x92PP` a/U\x85\x82\x86\x01a+cV[\x91PP\x92P\x92\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a/\x91\x81a+\xD4V[\x82RPPV[_a/\xA2\x83\x83a/\x88V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a/\xC4\x82a/_V[a/\xCE\x81\x85a/iV[\x93Pa/\xD9\x83a/yV[\x80_[\x83\x81\x10\x15a0\tW\x81Qa/\xF0\x88\x82a/\x97V[\x97Pa/\xFB\x83a/\xAEV[\x92PP`\x01\x81\x01\x90Pa/\xDCV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra0.\x81\x84a/\xBAV[\x90P\x92\x91PPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a0|\x82a06V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a0\x9BWa0\x9Aa0FV[[\x80`@RPPPV[_a0\xADa*\x80V[\x90Pa0\xB9\x82\x82a0sV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a0\xD8Wa0\xD7a0FV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a0\xFBa0\xF6\x84a0\xBEV[a0\xA4V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a1\x1EWa1\x1Da,\xF6V[[\x83[\x81\x81\x10\x15a1GW\x80a13\x88\x82a+\xFBV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa1 V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a1eWa1da,\xEEV[[\x815a1u\x84\x82` \x86\x01a0\xE9V[\x91PP\x92\x91PPV[__`@\x83\x85\x03\x12\x15a1\x94Wa1\x93a*\x89V[[_a1\xA1\x85\x82\x86\x01a+cV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\xC2Wa1\xC1a*\x8DV[[a1\xCE\x85\x82\x86\x01a1QV[\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a1\xEDWa1\xECa*\x89V[[_a1\xFA\x84\x82\x85\x01a+cV[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a25\x81a+DV[\x82RPPV[_a2F\x83\x83a2,V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a2h\x82a2\x03V[a2r\x81\x85a2\rV[\x93Pa2}\x83a2\x1DV[\x80_[\x83\x81\x10\x15a2\xADW\x81Qa2\x94\x88\x82a2;V[\x97Pa2\x9F\x83a2RV[\x92PP`\x01\x81\x01\x90Pa2\x80V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra2\xD2\x81\x84a2^V[\x90P\x92\x91PPV[____`\x80\x85\x87\x03\x12\x15a2\xF2Wa2\xF1a*\x89V[[_a2\xFF\x87\x82\x88\x01a-\xCBV[\x94PP` a3\x10\x87\x82\x88\x01a+cV[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a31Wa30a*\x8DV[[a3=\x87\x82\x88\x01a1QV[\x92PP``a3N\x87\x82\x88\x01a+cV[\x91PP\x92\x95\x91\x94P\x92PV[_`@\x82\x01\x90Pa3m_\x83\x01\x85a.\xF9V[a3z` \x83\x01\x84a,\xC6V[\x93\x92PPPV[_``\x82\x01\x90Pa3\x94_\x83\x01\x86a.\xF9V[a3\xA1` \x83\x01\x85a,\xC6V[a3\xAE`@\x83\x01\x84a,\xC6V[\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a3\xED\x82a+DV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a4\x1FWa4\x1Ea3\xB6V[[`\x01\x82\x01\x90P\x91\x90PV[_a44\x82a+DV[\x91Pa4?\x83a+DV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a4WWa4Va3\xB6V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_a4\x94\x82a+DV[\x91Pa4\x9F\x83a+DV[\x92P\x82\x82\x02a4\xAD\x81a+DV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a4\xC4Wa4\xC3a3\xB6V[[P\x92\x91PPV[_a4\xD5\x82a+DV[\x91Pa4\xE0\x83a+DV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a4\xF8Wa4\xF7a3\xB6V[[\x92\x91PPV[_`@\x82\x01\x90Pa5\x11_\x83\x01\x85a,\xC6V[a5\x1E` \x83\x01\x84a,\xC6V[\x93\x92PPPV[_\x82\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a5sW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a5\x86Wa5\x85a5/V[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a5\xE8\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a5\xADV[a5\xF2\x86\x83a5\xADV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a6-a6(a6#\x84a+DV[a6\nV[a+DV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a6F\x83a6\x13V[a6Za6R\x82a64V[\x84\x84Ta5\xB9V[\x82UPPPPV[__\x90P\x90V[a6qa6bV[a6|\x81\x84\x84a6=V[PPPV[_[\x82\x81\x10\x15a6\xA2Wa6\x97_\x82\x84\x01a6iV[`\x01\x81\x01\x90Pa6\x83V[PPPV[`\x1F\x82\x11\x15a6\xF5W\x82\x82\x11\x15a6\xF4Wa6\xC1\x81a5\x8CV[a6\xCA\x83a5\x9EV[a6\xD3\x85a5\x9EV[` \x86\x10\x15a6\xE0W_\x90P[\x80\x83\x01a6\xEF\x82\x84\x03\x82a6\x81V[PPPP[[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_a7\x15_\x19\x84`\x08\x02a6\xFAV[\x19\x80\x83\x16\x91PP\x92\x91PPV[_a7-\x83\x83a7\x06V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a7G\x83\x83a5%V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7`Wa7_a0FV[[a7j\x82Ta5\\V[a7u\x82\x82\x85a6\xA7V[_`\x1F\x83\x11`\x01\x81\x14a7\xA2W_\x84\x15a7\x90W\x82\x87\x015\x90P[a7\x9A\x85\x82a7\"V[\x86UPa8\x01V[`\x1F\x19\x84\x16a7\xB0\x86a5\x8CV[_[\x82\x81\x10\x15a7\xD7W\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa7\xB2V[\x86\x83\x10\x15a7\xF4W\x84\x89\x015a7\xF0`\x1F\x89\x16\x82a7\x06V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FCURRENTLY ONLY ONE INDEX PER CLI_\x82\x01R\x7FENT ALLOWED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a8t`+\x83a8\nV[\x91Pa8\x7F\x82a8\x1AV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra8\xA1\x81a8hV[\x90P\x91\x90PV[_`@\x82\x01\x90Pa8\xBB_\x83\x01\x85a.\xF9V[\x81\x81\x03` \x83\x01Ra8\xCD\x81\x84a2^V[\x90P\x93\x92PPPV[_`@\x82\x01\x90Pa8\xE9_\x83\x01\x85a,\xC6V[a8\xF6` \x83\x01\x84a.\xF9V[\x93\x92PPPV[_`@\x82\x01\x90Pa9\x10_\x83\x01\x85a,\x9EV[a9\x1D` \x83\x01\x84a,\x9EV[\x93\x92PPPV[_``\x82\x01\x90Pa97_\x83\x01\x86a,\xC6V[a9D` \x83\x01\x85a,\xC6V[a9Q`@\x83\x01\x84a.\xF9V[\x94\x93PPPPV[_`\x80\x82\x01\x90Pa9l_\x83\x01\x87a.\xF9V[a9y` \x83\x01\x86a,\xC6V[a9\x86`@\x83\x01\x85a,\xC6V[a9\x93``\x83\x01\x84a.\xF9V[\x95\x94PPPPPV[_`@\x82\x01\x90Pa9\xAF_\x83\x01\x85a.\xF9V[a9\xBC` \x83\x01\x84a.\nV[\x93\x92PPPV[_`\xFF\x82\x16\x90P\x91\x90PV[a9\xD8\x81a9\xC3V[\x82RPPV[_`\x80\x82\x01\x90Pa9\xF1_\x83\x01\x87a.\nV[a9\xFE` \x83\x01\x86a9\xCFV[a:\x0B`@\x83\x01\x85a.\nV[a:\x18``\x83\x01\x84a.\nV[\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 'uU\xB5\x1F\xF8\x97\xAEe\xA15j\xB2\xC1g\xF2I\xF5t(,c&vf\xC5R\x88K\xD3\xE6\xBEdsolcC\0\x08!\x003",
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
    /**Event with signature `OutputsPublished(bytes32,uint256)` and selector `0xd4038e459e3263985c973011ca5aab7d2c6502934b89f42e9bbaeb2496eb20e6`.
```solidity
event OutputsPublished(bytes32 stoffelProgramHash, uint256 timeOfExecution);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OutputsPublished {
        #[allow(missing_docs)]
        pub stoffelProgramHash: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for OutputsPublished {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "OutputsPublished(bytes32,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                212u8, 3u8, 142u8, 69u8, 158u8, 50u8, 99u8, 152u8, 92u8, 151u8, 48u8,
                17u8, 202u8, 90u8, 171u8, 125u8, 44u8, 101u8, 2u8, 147u8, 75u8, 137u8,
                244u8, 46u8, 155u8, 186u8, 235u8, 36u8, 150u8, 235u8, 32u8, 230u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    stoffelProgramHash: data.0,
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.stoffelProgramHash),
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
        impl alloy_sol_types::private::IntoLogData for OutputsPublished {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OutputsPublished> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OutputsPublished) -> alloy_sol_types::private::LogData {
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
constructor(bytes32 stoffelProgramHash, uint256 n, uint256 t, address designatedParty, address[] initialMPCNodes, uint256 nInputs);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub stoffelProgramHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub n: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub t: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub designatedParty: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub initialMPCNodes: alloy::sol_types::private::Vec<
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
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
                        value.n,
                        value.t,
                        value.designatedParty,
                        value.initialMPCNodes,
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
                        n: tuple.1,
                        t: tuple.2,
                        designatedParty: tuple.3,
                        initialMPCNodes: tuple.4,
                        nInputs: tuple.5,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.n),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.t),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.designatedParty,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.initialMPCNodes),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nInputs),
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
function resetAccessControl(uint256 t, address[] memory initialMPCNodes) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resetAccessControlCall {
        #[allow(missing_docs)]
        pub t: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub initialMPCNodes: alloy::sol_types::private::Vec<
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
                    (value.t, value.initialMPCNodes)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for resetAccessControlCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        t: tuple.0,
                        initialMPCNodes: tuple.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.initialMPCNodes),
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
function resetCoordinator(bytes32 stoffelProgramHash, uint256 t, address[] memory initialMPCNodes, uint256 nInputs) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resetCoordinatorCall {
        #[allow(missing_docs)]
        pub stoffelProgramHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub t: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub initialMPCNodes: alloy::sol_types::private::Vec<
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
                        value.initialMPCNodes,
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
                        initialMPCNodes: tuple.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.initialMPCNodes),
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
    /**Function with signature `setPublicOutputs(bytes)` and selector `0x6b744728`.
```solidity
function setPublicOutputs(bytes memory publicOutputs) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPublicOutputsCall {
        #[allow(missing_docs)]
        pub publicOutputs: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`setPublicOutputs(bytes)`](setPublicOutputsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPublicOutputsReturn {}
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
            impl ::core::convert::From<setPublicOutputsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setPublicOutputsCall) -> Self {
                    (value.publicOutputs,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setPublicOutputsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { publicOutputs: tuple.0 }
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
            impl ::core::convert::From<setPublicOutputsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setPublicOutputsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setPublicOutputsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setPublicOutputsReturn {
            fn _tokenize(
                &self,
            ) -> <setPublicOutputsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setPublicOutputsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setPublicOutputsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setPublicOutputs(bytes)";
            const SELECTOR: [u8; 4] = [107u8, 116u8, 71u8, 40u8];
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
                        &self.publicOutputs,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setPublicOutputsReturn::_tokenize(ret)
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
    /**Function with signature `shareReceived(address,address)` and selector `0x4925e62a`.
```solidity
function shareReceived(address from, address to) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct shareReceivedCall {
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`shareReceived(address,address)`](shareReceivedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct shareReceivedReturn {}
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
            impl ::core::convert::From<shareReceivedCall> for UnderlyingRustTuple<'_> {
                fn from(value: shareReceivedCall) -> Self {
                    (value.from, value.to)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for shareReceivedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { from: tuple.0, to: tuple.1 }
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
            impl ::core::convert::From<shareReceivedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: shareReceivedReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for shareReceivedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl shareReceivedReturn {
            fn _tokenize(
                &self,
            ) -> <shareReceivedCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for shareReceivedCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = shareReceivedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "shareReceived(address,address)";
            const SELECTOR: [u8; 4] = [73u8, 37u8, 230u8, 42u8];
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
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                shareReceivedReturn::_tokenize(ret)
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
    /**Function with signature `startMPC()` and selector `0x595f806e`.
```solidity
function startMPC() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startMPCCall;
    ///Container type for the return parameters of the [`startMPC()`](startMPCCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startMPCReturn {}
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
            impl ::core::convert::From<startMPCCall> for UnderlyingRustTuple<'_> {
                fn from(value: startMPCCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for startMPCCall {
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
            impl ::core::convert::From<startMPCReturn> for UnderlyingRustTuple<'_> {
                fn from(value: startMPCReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for startMPCReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl startMPCReturn {
            fn _tokenize(
                &self,
            ) -> <startMPCCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for startMPCCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = startMPCReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "startMPC()";
            const SELECTOR: [u8; 4] = [89u8, 95u8, 128u8, 110u8];
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
                startMPCReturn::_tokenize(ret)
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
        setPublicOutputs(setPublicOutputsCall),
        #[allow(missing_docs)]
        shareReceived(shareReceivedCall),
        #[allow(missing_docs)]
        startMPC(startMPCCall),
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
            [19u8, 255u8, 109u8, 213u8],
            [20u8, 108u8, 165u8, 49u8],
            [23u8, 99u8, 69u8, 20u8],
            [28u8, 116u8, 83u8, 219u8],
            [30u8, 228u8, 238u8, 15u8],
            [35u8, 40u8, 189u8, 18u8],
            [36u8, 138u8, 156u8, 163u8],
            [47u8, 47u8, 241u8, 93u8],
            [48u8, 16u8, 76u8, 62u8],
            [54u8, 86u8, 138u8, 190u8],
            [59u8, 67u8, 56u8, 209u8],
            [73u8, 37u8, 230u8, 42u8],
            [75u8, 142u8, 100u8, 136u8],
            [75u8, 178u8, 120u8, 243u8],
            [89u8, 95u8, 128u8, 110u8],
            [107u8, 116u8, 71u8, 40u8],
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
            [235u8, 174u8, 53u8, 231u8],
            [242u8, 253u8, 227u8, 139u8],
            [246u8, 96u8, 60u8, 97u8],
            [252u8, 120u8, 178u8, 232u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(supportsInterface),
            ::core::stringify!(submitMaskedInput),
            ::core::stringify!(isDesignatedParty),
            ::core::stringify!(round),
            ::core::stringify!(creationBlock),
            ::core::stringify!(baseNonce),
            ::core::stringify!(authenticateClient),
            ::core::stringify!(availableInputMasks),
            ::core::stringify!(getRoleAdmin),
            ::core::stringify!(grantRole),
            ::core::stringify!(PARTY_ROLE),
            ::core::stringify!(renounceRole),
            ::core::stringify!(resetInputManager),
            ::core::stringify!(shareReceived),
            ::core::stringify!(sendOutputs),
            ::core::stringify!(finalize),
            ::core::stringify!(startMPC),
            ::core::stringify!(setPublicOutputs),
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
            ::core::stringify!(obtainInputMasks),
            ::core::stringify!(transferOwnership),
            ::core::stringify!(resetCoordinator),
            ::core::stringify!(isParty),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <supportsInterfaceCall as alloy_sol_types::SolCall>::SIGNATURE,
            <submitMaskedInputCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isDesignatedPartyCall as alloy_sol_types::SolCall>::SIGNATURE,
            <roundCall as alloy_sol_types::SolCall>::SIGNATURE,
            <creationBlockCall as alloy_sol_types::SolCall>::SIGNATURE,
            <baseNonceCall as alloy_sol_types::SolCall>::SIGNATURE,
            <authenticateClientCall as alloy_sol_types::SolCall>::SIGNATURE,
            <availableInputMasksCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRoleAdminCall as alloy_sol_types::SolCall>::SIGNATURE,
            <grantRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <PARTY_ROLECall as alloy_sol_types::SolCall>::SIGNATURE,
            <renounceRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <resetInputManagerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <shareReceivedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <sendOutputsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <finalizeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <startMPCCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setPublicOutputsCall as alloy_sol_types::SolCall>::SIGNATURE,
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
        const COUNT: usize = 36usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
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
                Self::setPublicOutputs(_) => {
                    <setPublicOutputsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::shareReceived(_) => {
                    <shareReceivedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::startMPC(_) => <startMPCCall as alloy_sol_types::SolCall>::SELECTOR,
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
                    fn shareReceived(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <shareReceivedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::shareReceived)
                    }
                    shareReceived
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
                    fn startMPC(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <startMPCCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(FakeCoordinatorCalls::startMPC)
                    }
                    startMPC
                },
                {
                    fn setPublicOutputs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <setPublicOutputsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(FakeCoordinatorCalls::setPublicOutputs)
                    }
                    setPublicOutputs
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
                    fn shareReceived(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <shareReceivedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::shareReceived)
                    }
                    shareReceived
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
                    fn startMPC(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <startMPCCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::startMPC)
                    }
                    startMPC
                },
                {
                    fn setPublicOutputs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<FakeCoordinatorCalls> {
                        <setPublicOutputsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(FakeCoordinatorCalls::setPublicOutputs)
                    }
                    setPublicOutputs
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
                Self::setPublicOutputs(inner) => {
                    <setPublicOutputsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::shareReceived(inner) => {
                    <shareReceivedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::startMPC(inner) => {
                    <startMPCCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::setPublicOutputs(inner) => {
                    <setPublicOutputsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::shareReceived(inner) => {
                    <shareReceivedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::startMPC(inner) => {
                    <startMPCCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    }
    impl FakeCoordinatorErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [17u8, 140u8, 218u8, 167u8],
            [30u8, 79u8, 189u8, 247u8],
            [58u8, 35u8, 98u8, 104u8],
            [102u8, 151u8, 178u8, 50u8],
            [111u8, 175u8, 159u8, 5u8],
            [171u8, 220u8, 224u8, 106u8],
            [172u8, 169u8, 47u8, 9u8],
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
            ::core::stringify!(OwnableUnauthorizedAccount),
            ::core::stringify!(OwnableInvalidOwner),
            ::core::stringify!(NotEnoughMPCParties),
            ::core::stringify!(AccessControlBadConfirmation),
            ::core::stringify!(NoIndicesReserved),
            ::core::stringify!(NotAnExistingParty),
            ::core::stringify!(IndicesAlreadyReserved),
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
            <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::SIGNATURE,
            <OwnableInvalidOwner as alloy_sol_types::SolError>::SIGNATURE,
            <NotEnoughMPCParties as alloy_sol_types::SolError>::SIGNATURE,
            <AccessControlBadConfirmation as alloy_sol_types::SolError>::SIGNATURE,
            <NoIndicesReserved as alloy_sol_types::SolError>::SIGNATURE,
            <NotAnExistingParty as alloy_sol_types::SolError>::SIGNATURE,
            <IndicesAlreadyReserved as alloy_sol_types::SolError>::SIGNATURE,
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
        const COUNT: usize = 14usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AccessControlBadConfirmation(_) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AccessControlUnauthorizedAccount(_) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::SELECTOR
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
        OutputsPublished(OutputsPublished),
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
                212u8, 3u8, 142u8, 69u8, 158u8, 50u8, 99u8, 152u8, 92u8, 151u8, 48u8,
                17u8, 202u8, 90u8, 171u8, 125u8, 44u8, 101u8, 2u8, 147u8, 75u8, 137u8,
                244u8, 46u8, 155u8, 186u8, 235u8, 36u8, 150u8, 235u8, 32u8, 230u8,
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
            ::core::stringify!(OutputsPublished),
            ::core::stringify!(CoordinatorInitialized),
            ::core::stringify!(RoleRevoked),
            ::core::stringify!(IndexBufferEvent),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <ReservedInputEvent as alloy_sol_types::SolEvent>::SIGNATURE,
            <MPCStarted as alloy_sol_types::SolEvent>::SIGNATURE,
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
            <OutputsPublished as alloy_sol_types::SolEvent>::SIGNATURE,
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
                Some(<OutputsPublished as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OutputsPublished as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OutputsPublished)
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
                Self::OutputsPublished(inner) => {
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
                Self::OutputsPublished(inner) => {
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
        n: alloy::sol_types::private::primitives::aliases::U256,
        t: alloy::sol_types::private::primitives::aliases::U256,
        designatedParty: alloy::sol_types::private::Address,
        initialMPCNodes: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        nInputs: alloy::sol_types::private::primitives::aliases::U256,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<FakeCoordinatorInstance<P, N>>,
    > {
        FakeCoordinatorInstance::<
            P,
            N,
        >::deploy(
            __provider,
            stoffelProgramHash,
            n,
            t,
            designatedParty,
            initialMPCNodes,
            nInputs,
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
        n: alloy::sol_types::private::primitives::aliases::U256,
        t: alloy::sol_types::private::primitives::aliases::U256,
        designatedParty: alloy::sol_types::private::Address,
        initialMPCNodes: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        nInputs: alloy::sol_types::private::primitives::aliases::U256,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        FakeCoordinatorInstance::<
            P,
            N,
        >::deploy_builder(
            __provider,
            stoffelProgramHash,
            n,
            t,
            designatedParty,
            initialMPCNodes,
            nInputs,
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
            n: alloy::sol_types::private::primitives::aliases::U256,
            t: alloy::sol_types::private::primitives::aliases::U256,
            designatedParty: alloy::sol_types::private::Address,
            initialMPCNodes: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            nInputs: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::Result<FakeCoordinatorInstance<P, N>> {
            let call_builder = Self::deploy_builder(
                __provider,
                stoffelProgramHash,
                n,
                t,
                designatedParty,
                initialMPCNodes,
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
            n: alloy::sol_types::private::primitives::aliases::U256,
            t: alloy::sol_types::private::primitives::aliases::U256,
            designatedParty: alloy::sol_types::private::Address,
            initialMPCNodes: alloy::sol_types::private::Vec<
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
                            n,
                            t,
                            designatedParty,
                            initialMPCNodes,
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
            initialMPCNodes: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, resetAccessControlCall, N> {
            self.call_builder(
                &resetAccessControlCall {
                    t,
                    initialMPCNodes,
                },
            )
        }
        ///Creates a new call builder for the [`resetCoordinator`] function.
        pub fn resetCoordinator(
            &self,
            stoffelProgramHash: alloy::sol_types::private::FixedBytes<32>,
            t: alloy::sol_types::private::primitives::aliases::U256,
            initialMPCNodes: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            nInputs: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, resetCoordinatorCall, N> {
            self.call_builder(
                &resetCoordinatorCall {
                    stoffelProgramHash,
                    t,
                    initialMPCNodes,
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
        ///Creates a new call builder for the [`setPublicOutputs`] function.
        pub fn setPublicOutputs(
            &self,
            publicOutputs: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, setPublicOutputsCall, N> {
            self.call_builder(
                &setPublicOutputsCall {
                    publicOutputs,
                },
            )
        }
        ///Creates a new call builder for the [`shareReceived`] function.
        pub fn shareReceived(
            &self,
            from: alloy::sol_types::private::Address,
            to: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, shareReceivedCall, N> {
            self.call_builder(&shareReceivedCall { from, to })
        }
        ///Creates a new call builder for the [`startMPC`] function.
        pub fn startMPC(&self) -> alloy_contract::SolCallBuilder<&P, startMPCCall, N> {
            self.call_builder(&startMPCCall)
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
        ///Creates a new event filter for the [`OutputsPublished`] event.
        pub fn OutputsPublished_filter(
            &self,
        ) -> alloy_contract::Event<&P, OutputsPublished, N> {
            self.event_filter::<OutputsPublished>()
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

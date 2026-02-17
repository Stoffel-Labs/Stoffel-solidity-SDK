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
    error NotAnExistingParty(address account);
    error NotAtRound(StoffelCoordinator.Round required, StoffelCoordinator.Round current);
    error NotEnoughIndices(uint256 requested, uint256 available);
    error NotEnoughMPCParties(uint256 current, uint256 required);
    error OwnableInvalidOwner(address owner);
    error OwnableUnauthorizedAccount(address account);

    event ClientAuthenticated(address indexed client, uint256 nonce);
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
    event ReservedInputEvent(address client, uint256 reservedIndex);
    event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole);
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);

    constructor(bytes32 stoffelProgramHash, uint256 n, uint256 t, address designatedParty, address[] initialMPCNodes, uint256 nInputs);

    function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
    function DESIGNATED_PARTY_ROLE() external view returns (bytes32);
    function PARTY_ROLE() external view returns (bytes32);
    function authenticateClient(uint256 nonce, address clientAddr, bytes memory signature) external returns (bool);
    function availableInputMasks() external view returns (uint256);
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
    function resetCoordinator(bytes32 stoffelProgramHash, uint256 n, uint256 t, address[] memory initialMPCNodes, uint256 nInputs) external;
    function resetInputManager(uint256 nIndicesToReserve) external;
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
        "name": "nonce",
        "type": "uint256",
        "internalType": "uint256"
      },
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
        "name": "nonce",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
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
    ///0x608060405234801561000f575f5ffd5b506040516145a03803806145a083398181016040528101906100319190610e0f565b8584838333818484610049828261016360201b60201c565b505061005a8161045960201b60201c565b505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16036100cb575f6040517f1e4fbdf70000000000000000000000000000000000000000000000000000000081526004016100c29190610ec3565b60405180910390fd5b6100da816104ac60201b60201c565b5061010584835f815181106100f2576100f1610edc565b5b602002602001015161056f60201b60201c565b5050505042600a8190555043600b819055507fdef1f08eb655f4a75f60bd6fd7e971112004abd846a612e46dab270770d24ca530600a54600b54336040516101509493929190610f18565b60405180910390a150505050505061110c565b5f60018360036101739190610f88565b61017d9190610fc9565b905080825110156101c8578151816040517f3a2362680000000000000000000000000000000000000000000000000000000081526004016101bf929190610ffc565b60405180910390fd5b826002819055505f6101ff7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6105f260201b60201c565b90505f6102317f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61061a60201b60201c565b90505f5f90505b818110156102985761028a7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e84838151811061027757610276610edc565b5b602002602001015161064160201b60201c565b508080600101915050610238565b505f6102c97fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696105f260201b60201c565b90505f6102fb7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46961061a60201b60201c565b90505f5f90505b81811015610362576103547fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46984838151811061034157610340610edc565b5b602002602001015161064160201b60201c565b508080600101915050610302565b505f5f90505b86518110156103c9576103bb7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698883815181106103a8576103a7610edc565b5b602002602001015161065a60201b60201c565b508080600101915050610368565b506104147f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e875f8151811061040157610400610edc565b5b602002602001015161065a60201b60201c565b507fa2df7830e0bedef7b1116bf547b467b16b50b3bd23146c9e099878d14e89301a85883360405161044893929190611023565b60405180910390a150505050505050565b806004819055505f6005819055505f6006819055507ff7f0872382dff5e698b284e12084e4e7894f830216dd80cb4e909b593a58f935600454336040516104a1929190611058565b60405180910390a150565b5f60085f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690508160085f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a35050565b8160098190555042600a8190555043600b819055505f600c5f6101000a81548160ff021916908360058111156105a8576105a761107f565b5b02179055507fdef1f08eb655f4a75f60bd6fd7e971112004abd846a612e46dab270770d24ca530600a54600b54846040516105e69493929190610f18565b60405180910390a15050565b606061061360015f8481526020019081526020015f2061067360201b60201c565b9050919050565b5f61063a60015f8481526020019081526020015f2061069860201b60201c565b9050919050565b5f61065283836106b160201b60201c565b905092915050565b5f61066b83836106fa60201b60201c565b905092915050565b60605f610687835f0161074360201b60201c565b905060608190508092505050919050565b5f6106aa825f0161079c60201b60201c565b9050919050565b5f5f6106c384846107ab60201b60201c565b905080156106f0576106ee8360015f8781526020019081526020015f206108a060201b90919060201c565b505b8091505092915050565b5f5f61070c84846108d360201b60201c565b90508015610739576107378360015f8781526020019081526020015f206109c860201b90919060201c565b505b8091505092915050565b6060815f0180548060200260200160405190810160405280929190818152602001828054801561079057602002820191905f5260205f20905b81548152602001906001019080831161077c575b50505050509050919050565b5f815f01805490509050919050565b5f6107bc83836109fb60201b60201c565b15610896575f5f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550610833610a5e60201b60201c565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b60405160405180910390a46001905061089a565b5f90505b92915050565b5f6108cb835f018373ffffffffffffffffffffffffffffffffffffffff165f1b610a6560201b60201c565b905092915050565b5f6108e483836109fb60201b60201c565b6109be5760015f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555061095b610a5e60201b60201c565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a4600190506109c2565b5f90505b92915050565b5f6109f3835f018373ffffffffffffffffffffffffffffffffffffffff165f1b610b6160201b60201c565b905092915050565b5f5f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f33905090565b5f5f836001015f8481526020019081526020015f205490505f8114610b56575f600182610a9291906110ac565b90505f6001865f0180549050610aa891906110ac565b9050808214610b0e575f865f018281548110610ac757610ac6610edc565b5b905f5260205f200154905080875f018481548110610ae857610ae7610edc565b5b905f5260205f20018190555083876001015f8381526020019081526020015f2081905550505b855f01805480610b2157610b206110df565b5b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f905560019350505050610b5b565b5f9150505b92915050565b5f610b728383610bce60201b60201c565b610bc457825f0182908060018154018082558091505060019003905f5260205f20015f9091909190915055825f0180549050836001015f8481526020019081526020015f208190555060019050610bc8565b5f90505b92915050565b5f5f836001015f8481526020019081526020015f20541415905092915050565b5f604051905090565b5f5ffd5b5f5ffd5b5f819050919050565b610c1181610bff565b8114610c1b575f5ffd5b50565b5f81519050610c2c81610c08565b92915050565b5f819050919050565b610c4481610c32565b8114610c4e575f5ffd5b50565b5f81519050610c5f81610c3b565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610c8e82610c65565b9050919050565b610c9e81610c84565b8114610ca8575f5ffd5b50565b5f81519050610cb981610c95565b92915050565b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610d0982610cc3565b810181811067ffffffffffffffff82111715610d2857610d27610cd3565b5b80604052505050565b5f610d3a610bee565b9050610d468282610d00565b919050565b5f67ffffffffffffffff821115610d6557610d64610cd3565b5b602082029050602081019050919050565b5f5ffd5b5f610d8c610d8784610d4b565b610d31565b90508083825260208201905060208402830185811115610daf57610dae610d76565b5b835b81811015610dd85780610dc48882610cab565b845260208401935050602081019050610db1565b5050509392505050565b5f82601f830112610df657610df5610cbf565b5b8151610e06848260208601610d7a565b91505092915050565b5f5f5f5f5f5f60c08789031215610e2957610e28610bf7565b5b5f610e3689828a01610c1e565b9650506020610e4789828a01610c51565b9550506040610e5889828a01610c51565b9450506060610e6989828a01610cab565b935050608087015167ffffffffffffffff811115610e8a57610e89610bfb565b5b610e9689828a01610de2565b92505060a0610ea789828a01610c51565b9150509295509295509295565b610ebd81610c84565b82525050565b5f602082019050610ed65f830184610eb4565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b610f1281610c32565b82525050565b5f608082019050610f2b5f830187610eb4565b610f386020830186610f09565b610f456040830185610f09565b610f526060830184610eb4565b95945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610f9282610c32565b9150610f9d83610c32565b9250828202610fab81610c32565b91508282048414831517610fc257610fc1610f5b565b5b5092915050565b5f610fd382610c32565b9150610fde83610c32565b9250828201905080821115610ff657610ff5610f5b565b5b92915050565b5f60408201905061100f5f830185610f09565b61101c6020830184610f09565b9392505050565b5f6060820190506110365f830186610f09565b6110436020830185610f09565b6110506040830184610eb4565b949350505050565b5f60408201905061106b5f830185610f09565b6110786020830184610eb4565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b5f6110b682610c32565b91506110c183610c32565b92508282039050818111156110d9576110d8610f5b565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffd5b613487806111195f395ff3fe608060405234801561000f575f5ffd5b506004361061020f575f3560e01c80638da5cb5b11610123578063ca15c873116100ab578063e23fb1f81161007a578063e23fb1f814610579578063ebae35e714610595578063f2fde38b146105c5578063f9d8ff15146105e1578063fc78b2e8146106115761020f565b8063ca15c87314610505578063cb9c4cc414610535578063d547741f1461053f578063d8270dce1461055b5761020f565b8063a217fddf116100f2578063a217fddf14610487578063a3246ad3146104a5578063af206f28146104d5578063bb51fef0146104f1578063c079f495146104fb5761020f565b80638da5cb5b146103ed5780639010d07c1461040b57806391d148541461043b578063930ceae61461046b5761020f565b806330104c3e116101a65780634bb278f3116101755780634bb278f314610395578063595f806e1461039f5780636b744728146103a9578063715018a6146103c55780637f35b560146103cf5761020f565b806330104c3e1461033557806336568abe146103535780634925e62a1461036f5780634b8e64881461038b5761020f565b806317634514116101e257806317634514146102ad5780632328bd12146102cb578063248a9ca3146102e95780632f2ff15d146103195761020f565b806301ffc9a7146102135780630bda81cf1461024357806313ff6dd51461025f578063146ca5311461028f575b5f5ffd5b61022d6004803603810190610228919061258e565b610641565b60405161023a91906125d3565b60405180910390f35b61025d6004803603810190610258919061261f565b6106ba565b005b610279600480360381019061027491906126b7565b610819565b60405161028691906125d3565b60405180910390f35b610297610895565b6040516102a49190612755565b60405180910390f35b6102b56108a7565b6040516102c2919061277d565b60405180910390f35b6102d36108ad565b6040516102e0919061277d565b60405180910390f35b61030360048036038101906102fe91906127c9565b6108c3565b6040516103109190612803565b60405180910390f35b610333600480360381019061032e919061281c565b6108df565b005b61033d610919565b60405161034a9190612803565b60405180910390f35b61036d6004803603810190610368919061281c565b61093d565b005b6103896004803603810190610384919061285a565b610a04565b005b610393610ac6565b005b61039d610b40565b005b6103a7610bbd565b005b6103c360048036038101906103be91906128f9565b610c37565b005b6103cd610c7a565b005b6103d7610c8d565b6040516103e49190612803565b60405180910390f35b6103f5610cb1565b6040516104029190612953565b60405180910390f35b6104256004803603810190610420919061296c565b610cd9565b6040516104329190612953565b60405180910390f35b6104556004803603810190610450919061281c565b610d05565b60405161046291906125d3565b60405180910390f35b61048560048036038101906104809190612af2565b610d68565b005b61048f610dd1565b60405161049c9190612803565b60405180910390f35b6104bf60048036038101906104ba91906127c9565b610dd7565b6040516104cc9190612c3c565b60405180910390f35b6104ef60048036038101906104ea9190612c5c565b610df9565b005b6104f9610e32565b005b610503610eac565b005b61051f600480360381019061051a91906127c9565b610f26565b60405161052c919061277d565b60405180910390f35b61053d610f47565b005b6105596004803603810190610554919061281c565b610fc0565b005b6105636110b3565b604051610570919061277d565b60405180910390f35b610593600480360381019061058e9190612cb6565b6110b9565b005b6105af60048036038101906105aa9190612cb6565b6110f0565b6040516105bc9190612d98565b60405180910390f35b6105df60048036038101906105da91906126b7565b6112b6565b005b6105fb60048036038101906105f69190612db8565b61133a565b60405161060891906125d3565b60405180910390f35b61062b600480360381019061062691906126b7565b611482565b60405161063891906125d3565b60405180910390f35b5f7f07effe0d000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191614806106b357506106b2826114b4565b5b9050919050565b3373ffffffffffffffffffffffffffffffffffffffff1660035f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff161461075c5733816040517fffabbae7000000000000000000000000000000000000000000000000000000008152600401610753929190612e29565b60405180910390fd5b60405180604001604052808281526020018381525060075f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f820151815f0155602082015181600101559050507fb89addd937f44f902c849596641837cd7af2fcecef22d2a7866fdc1ad9c0ae2e3383836040516107f693929190612e50565b60405180910390a160065f81548092919061081090612eb2565b91905055505050565b5f61082382611482565b61086457816040517fabdce06a00000000000000000000000000000000000000000000000000000000815260040161085b9190612953565b60405180910390fd5b61088e7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e83610d05565b9050919050565b600c5f9054906101000a900460ff1681565b600b5481565b5f6005546004546108be9190612ef9565b905090565b5f5f5f8381526020019081526020015f20600101549050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6109098161152d565b6109138383611541565b50505050565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46981565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698214801561097257506109718282610d05565b5b156109f6575f61098183610f26565b90505f600160025460036109959190612f2c565b61099f9190612f6d565b90508082036109f3576001816109b59190612ef9565b816040517f3a2362680000000000000000000000000000000000000000000000000000000081526004016109ea929190612fa0565b60405180910390fd5b50505b610a008282611554565b5050565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469610a2e8161152d565b6001600d6001015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550505050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610af08161152d565b6004610afb816115cf565b7f301f8a3701f5b260197382dd7301078542144fe8fddd18083d6f6e09e4958a593342604051610b2c929190612e29565b60405180910390a1610b3c611658565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610b6a8161152d565b6005610b75816115cf565b5f7f24a873651d26fb5a462cb37a91071cdd4d09abeebfe0ed144329bed1cc359d033342604051610ba7929190612e29565b60405180910390a1610bb8816116c1565b505050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610be78161152d565b6003610bf2816115cf565b7f20f55ed0c92f2bb1c8825488e1e3c98463d024b2a42dbd24838c3f75260f43e93342604051610c23929190612e29565b60405180910390a1610c33611658565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610c618161152d565b8282600d5f019182610c749291906131df565b50505050565b610c826116ed565b610c8b5f611774565b565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e81565b5f60085f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b5f610cfd8260015f8681526020019081526020015f2061183790919063ffffffff16565b905092915050565b5f5f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610d928161152d565b610d9c848461184e565b610da582611b14565b610dc986845f81518110610dbc57610dbb6132ac565b5b6020026020010151611b67565b505050505050565b5f5f1b81565b6060610df260015f8481526020019081526020015f20611bea565b9050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610e238161152d565b610e2d838361184e565b505050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610e5c8161152d565b6002610e67816115cf565b7f60edf9bdc7c4ea007cae1a9bbd03e41e5bfccd7231a6ec383c2edd7800f0d20c3342604051610e98929190612e29565b60405180910390a1610ea8611658565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610ed68161152d565b6001610ee1816115cf565b7f67c4489f674d03c7d19a9e36735188de7c65e8d1e99eb3a2fd258a769eb14fff3342604051610f12929190612e29565b60405180910390a1610f22611658565b5050565b5f610f4060015f8481526020019081526020015f20611c09565b9050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610f718161152d565b5f610f7b816115cf565b7fbb709dea744f06d1b26e824deec2f7140c511266ee15d7a217838b31d8b0123d3342604051610fac929190612e29565b60405180910390a1610fbc611658565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610fea8161152d565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698314801561101f575061101e8383610d05565b5b156110a3575f61102e84610f26565b90505f600160025460036110429190612f2c565b61104c9190612f6d565b90508082036110a0576001816110629190612ef9565b816040517f3a236268000000000000000000000000000000000000000000000000000000008152600401611097929190612fa0565b60405180910390fd5b50505b6110ad8383611c1c565b50505050565b600a5481565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6110e38161152d565b6110ec82611b14565b5050565b60605f6005546004546111039190612ef9565b90508083111561114c5782816040517fdf3d75e2000000000000000000000000000000000000000000000000000000008152600401611143929190612fa0565b60405180910390fd5b5f60018460055461115d9190612f6d565b6111679190612ef9565b90505f8467ffffffffffffffff811115611184576111836129ba565b5b6040519080825280602002602001820160405280156111b25781602001602082028036833780820191505090505b5090505f60055490505b826005541161129c573360035f60055481526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555060055482826005546112289190612ef9565b81518110611239576112386132ac565b5b6020026020010181815250507fabde16b7a9192c31c6231b1539bad6fed77635de4c008718dbdcafb7b8363afe33600554604051611278929190612e29565b60405180910390a160055f81548092919061129290612eb2565b91905055506111bc565b85846112a89190612ef9565b935081945050505050919050565b6112be6116ed565b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff160361132e575f6040517f1e4fbdf70000000000000000000000000000000000000000000000000000000081526004016113259190612953565b60405180910390fd5b61133781611774565b50565b5f7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696113658161152d565b5f6113958760405160200161137a919061277d565b60405160208183030381529060405280519060200120611c2f565b90505f6113e58287878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050611c62565b90508673ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603611473578673ffffffffffffffffffffffffffffffffffffffff167f23d369583ce23023a1ca4baf14a48693a3ee9b3abf2483e41d639e57cb1f9dab89604051611460919061277d565b60405180910390a2600193505050611479565b5f935050505b50949350505050565b5f6114ad7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46983610d05565b9050919050565b5f7f5a05180f000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19161480611526575061152582611c8c565b5b9050919050565b61153e81611539611d05565b611d0c565b50565b5f61154c8383611d5d565b905092915050565b61155c611d05565b73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16146115c0576040517f6697b23200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6115ca8282611c1c565b505050565b8060058111156115e2576115e16126e2565b5b600c5f9054906101000a900460ff166005811115611603576116026126e2565b5b146116555780600c5f9054906101000a900460ff166040517fbfa217d800000000000000000000000000000000000000000000000000000000815260040161164c9291906132d9565b60405180910390fd5b50565b6001600c5f9054906101000a900460ff16600581111561167b5761167a6126e2565b5b6116859190612f6d565b6005811115611697576116966126e2565b5b600c5f6101000a81548160ff021916908360058111156116ba576116b96126e2565b5b0217905550565b80600c5f6101000a81548160ff021916908360058111156116e5576116e46126e2565b5b021790555050565b6116f5611d05565b73ffffffffffffffffffffffffffffffffffffffff16611713610cb1565b73ffffffffffffffffffffffffffffffffffffffff161461177257611736611d05565b6040517f118cdaa70000000000000000000000000000000000000000000000000000000081526004016117699190612953565b60405180910390fd5b565b5f60085f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690508160085f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a35050565b5f611844835f0183611da0565b5f1c905092915050565b5f600183600361185e9190612f2c565b6118689190612f6d565b905080825110156118b3578151816040517f3a2362680000000000000000000000000000000000000000000000000000000081526004016118aa929190612fa0565b60405180910390fd5b826002819055505f6118e47f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610dd7565b90505f6119107f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610f26565b90505f5f90505b81811015611971576119637f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e848381518110611956576119556132ac565b5b6020026020010151611c1c565b508080600101915050611917565b505f61199c7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469610dd7565b90505f6119c87fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469610f26565b90505f5f90505b81811015611a2957611a1b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469848381518110611a0e57611a0d6132ac565b5b6020026020010151611c1c565b5080806001019150506119cf565b505f5f90505b8651811015611a8a57611a7c7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469888381518110611a6f57611a6e6132ac565b5b6020026020010151611541565b508080600101915050611a2f565b50611acf7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e875f81518110611ac257611ac16132ac565b5b6020026020010151611541565b507fa2df7830e0bedef7b1116bf547b467b16b50b3bd23146c9e099878d14e89301a858833604051611b0393929190613300565b60405180910390a150505050505050565b806004819055505f6005819055505f6006819055507ff7f0872382dff5e698b284e12084e4e7894f830216dd80cb4e909b593a58f93560045433604051611b5c929190613335565b60405180910390a150565b8160098190555042600a8190555043600b819055505f600c5f6101000a81548160ff02191690836005811115611ba057611b9f6126e2565b5b02179055507fdef1f08eb655f4a75f60bd6fd7e971112004abd846a612e46dab270770d24ca530600a54600b5484604051611bde949392919061335c565b60405180910390a15050565b60605f611bf8835f01611dc7565b905060608190508092505050919050565b5f611c15825f01611e20565b9050919050565b5f611c278383611e2f565b905092915050565b5f7f19457468657265756d205369676e6564204d6573736167653a0a3332000000005f5281601c52603c5f209050919050565b5f5f5f5f611c708686611e72565b925092509250611c808282611ec7565b82935050505092915050565b5f7f7965db0b000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19161480611cfe5750611cfd82612029565b5b9050919050565b5f33905090565b611d168282610d05565b611d595780826040517fe2517d3f000000000000000000000000000000000000000000000000000000008152600401611d5092919061339f565b60405180910390fd5b5050565b5f5f611d698484612092565b90508015611d9657611d948360015f8781526020019081526020015f2061217b90919063ffffffff16565b505b8091505092915050565b5f825f018281548110611db657611db56132ac565b5b905f5260205f200154905092915050565b6060815f01805480602002602001604051908101604052809291908181526020018280548015611e1457602002820191905f5260205f20905b815481526020019060010190808311611e00575b50505050509050919050565b5f815f01805490509050919050565b5f5f611e3b84846121a8565b90508015611e6857611e668360015f8781526020019081526020015f2061229190919063ffffffff16565b505b8091505092915050565b5f5f5f6041845103611eb2575f5f5f602087015192506040870151915060608701515f1a9050611ea4888285856122be565b955095509550505050611ec0565b5f600285515f1b9250925092505b9250925092565b5f6003811115611eda57611ed96126e2565b5b826003811115611eed57611eec6126e2565b5b03156120255760016003811115611f0757611f066126e2565b5b826003811115611f1a57611f196126e2565b5b03611f51576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60026003811115611f6557611f646126e2565b5b826003811115611f7857611f776126e2565b5b03611fbc57805f1c6040517ffce698f7000000000000000000000000000000000000000000000000000000008152600401611fb3919061277d565b60405180910390fd5b600380811115611fcf57611fce6126e2565b5b826003811115611fe257611fe16126e2565b5b0361202457806040517fd78bce0c00000000000000000000000000000000000000000000000000000000815260040161201b9190612803565b60405180910390fd5b5b5050565b5f7f01ffc9a7000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916149050919050565b5f61209d8383610d05565b6121715760015f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555061210e611d05565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a460019050612175565b5f90505b92915050565b5f6121a0835f018373ffffffffffffffffffffffffffffffffffffffff165f1b6123a5565b905092915050565b5f6121b38383610d05565b15612287575f5f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550612224611d05565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b60405160405180910390a46001905061228b565b5f90505b92915050565b5f6122b6835f018373ffffffffffffffffffffffffffffffffffffffff165f1b61240c565b905092915050565b5f5f5f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c11156122fa575f60038592509250925061239b565b5f6001888888886040515f815260200160405260405161231d94939291906133e1565b6020604051602081039080840390855afa15801561233d573d5f5f3e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff160361238e575f60015f5f1b9350935093505061239b565b805f5f5f1b935093509350505b9450945094915050565b5f6123b08383612508565b61240257825f0182908060018154018082558091505060019003905f5260205f20015f9091909190915055825f0180549050836001015f8481526020019081526020015f208190555060019050612406565b5f90505b92915050565b5f5f836001015f8481526020019081526020015f205490505f81146124fd575f6001826124399190612ef9565b90505f6001865f018054905061244f9190612ef9565b90508082146124b5575f865f01828154811061246e5761246d6132ac565b5b905f5260205f200154905080875f01848154811061248f5761248e6132ac565b5b905f5260205f20018190555083876001015f8381526020019081526020015f2081905550505b855f018054806124c8576124c7613424565b5b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f905560019350505050612502565b5f9150505b92915050565b5f5f836001015f8481526020019081526020015f20541415905092915050565b5f604051905090565b5f5ffd5b5f5ffd5b5f7fffffffff0000000000000000000000000000000000000000000000000000000082169050919050565b61256d81612539565b8114612577575f5ffd5b50565b5f8135905061258881612564565b92915050565b5f602082840312156125a3576125a2612531565b5b5f6125b08482850161257a565b91505092915050565b5f8115159050919050565b6125cd816125b9565b82525050565b5f6020820190506125e65f8301846125c4565b92915050565b5f819050919050565b6125fe816125ec565b8114612608575f5ffd5b50565b5f81359050612619816125f5565b92915050565b5f5f6040838503121561263557612634612531565b5b5f6126428582860161260b565b92505060206126538582860161260b565b9150509250929050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6126868261265d565b9050919050565b6126968161267c565b81146126a0575f5ffd5b50565b5f813590506126b18161268d565b92915050565b5f602082840312156126cc576126cb612531565b5b5f6126d9848285016126a3565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b600681106127205761271f6126e2565b5b50565b5f8190506127308261270f565b919050565b5f61273f82612723565b9050919050565b61274f81612735565b82525050565b5f6020820190506127685f830184612746565b92915050565b612777816125ec565b82525050565b5f6020820190506127905f83018461276e565b92915050565b5f819050919050565b6127a881612796565b81146127b2575f5ffd5b50565b5f813590506127c38161279f565b92915050565b5f602082840312156127de576127dd612531565b5b5f6127eb848285016127b5565b91505092915050565b6127fd81612796565b82525050565b5f6020820190506128165f8301846127f4565b92915050565b5f5f6040838503121561283257612831612531565b5b5f61283f858286016127b5565b9250506020612850858286016126a3565b9150509250929050565b5f5f604083850312156128705761286f612531565b5b5f61287d858286016126a3565b925050602061288e858286016126a3565b9150509250929050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f8401126128b9576128b8612898565b5b8235905067ffffffffffffffff8111156128d6576128d561289c565b5b6020830191508360018202830111156128f2576128f16128a0565b5b9250929050565b5f5f6020838503121561290f5761290e612531565b5b5f83013567ffffffffffffffff81111561292c5761292b612535565b5b612938858286016128a4565b92509250509250929050565b61294d8161267c565b82525050565b5f6020820190506129665f830184612944565b92915050565b5f5f6040838503121561298257612981612531565b5b5f61298f858286016127b5565b92505060206129a08582860161260b565b9150509250929050565b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6129f0826129aa565b810181811067ffffffffffffffff82111715612a0f57612a0e6129ba565b5b80604052505050565b5f612a21612528565b9050612a2d82826129e7565b919050565b5f67ffffffffffffffff821115612a4c57612a4b6129ba565b5b602082029050602081019050919050565b5f612a6f612a6a84612a32565b612a18565b90508083825260208201905060208402830185811115612a9257612a916128a0565b5b835b81811015612abb5780612aa788826126a3565b845260208401935050602081019050612a94565b5050509392505050565b5f82601f830112612ad957612ad8612898565b5b8135612ae9848260208601612a5d565b91505092915050565b5f5f5f5f5f60a08688031215612b0b57612b0a612531565b5b5f612b18888289016127b5565b9550506020612b298882890161260b565b9450506040612b3a8882890161260b565b935050606086013567ffffffffffffffff811115612b5b57612b5a612535565b5b612b6788828901612ac5565b9250506080612b788882890161260b565b9150509295509295909350565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b612bb78161267c565b82525050565b5f612bc88383612bae565b60208301905092915050565b5f602082019050919050565b5f612bea82612b85565b612bf48185612b8f565b9350612bff83612b9f565b805f5b83811015612c2f578151612c168882612bbd565b9750612c2183612bd4565b925050600181019050612c02565b5085935050505092915050565b5f6020820190508181035f830152612c548184612be0565b905092915050565b5f5f60408385031215612c7257612c71612531565b5b5f612c7f8582860161260b565b925050602083013567ffffffffffffffff811115612ca057612c9f612535565b5b612cac85828601612ac5565b9150509250929050565b5f60208284031215612ccb57612cca612531565b5b5f612cd88482850161260b565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b612d13816125ec565b82525050565b5f612d248383612d0a565b60208301905092915050565b5f602082019050919050565b5f612d4682612ce1565b612d508185612ceb565b9350612d5b83612cfb565b805f5b83811015612d8b578151612d728882612d19565b9750612d7d83612d30565b925050600181019050612d5e565b5085935050505092915050565b5f6020820190508181035f830152612db08184612d3c565b905092915050565b5f5f5f5f60608587031215612dd057612dcf612531565b5b5f612ddd8782880161260b565b9450506020612dee878288016126a3565b935050604085013567ffffffffffffffff811115612e0f57612e0e612535565b5b612e1b878288016128a4565b925092505092959194509250565b5f604082019050612e3c5f830185612944565b612e49602083018461276e565b9392505050565b5f606082019050612e635f830186612944565b612e70602083018561276e565b612e7d604083018461276e565b949350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f612ebc826125ec565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203612eee57612eed612e85565b5b600182019050919050565b5f612f03826125ec565b9150612f0e836125ec565b9250828203905081811115612f2657612f25612e85565b5b92915050565b5f612f36826125ec565b9150612f41836125ec565b9250828202612f4f816125ec565b91508282048414831517612f6657612f65612e85565b5b5092915050565b5f612f77826125ec565b9150612f82836125ec565b9250828201905080821115612f9a57612f99612e85565b5b92915050565b5f604082019050612fb35f83018561276e565b612fc0602083018461276e565b9392505050565b5f82905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061301557607f821691505b60208210810361302857613027612fd1565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f6008830261308a7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8261304f565b613094868361304f565b95508019841693508086168417925050509392505050565b5f819050919050565b5f6130cf6130ca6130c5846125ec565b6130ac565b6125ec565b9050919050565b5f819050919050565b6130e8836130b5565b6130fc6130f4826130d6565b84845461305b565b825550505050565b5f5f905090565b613113613104565b61311e8184846130df565b505050565b5f5b82811015613144576131395f82840161310b565b600181019050613125565b505050565b601f8211156131975782821115613196576131638161302e565b61316c83613040565b61317585613040565b6020861015613182575f90505b80830161319182840382613123565b505050505b5b505050565b5f82821c905092915050565b5f6131b75f198460080261319c565b1980831691505092915050565b5f6131cf83836131a8565b9150826002028217905092915050565b6131e98383612fc7565b67ffffffffffffffff811115613202576132016129ba565b5b61320c8254612ffe565b613217828285613149565b5f601f831160018114613244575f8415613232578287013590505b61323c85826131c4565b8655506132a3565b601f1984166132528661302e565b5f5b8281101561327957848901358255600182019150602085019450602081019050613254565b868310156132965784890135613292601f8916826131a8565b8355505b6001600288020188555050505b50505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f6040820190506132ec5f830185612746565b6132f96020830184612746565b9392505050565b5f6060820190506133135f83018661276e565b613320602083018561276e565b61332d6040830184612944565b949350505050565b5f6040820190506133485f83018561276e565b6133556020830184612944565b9392505050565b5f60808201905061336f5f830187612944565b61337c602083018661276e565b613389604083018561276e565b6133966060830184612944565b95945050505050565b5f6040820190506133b25f830185612944565b6133bf60208301846127f4565b9392505050565b5f60ff82169050919050565b6133db816133c6565b82525050565b5f6080820190506133f45f8301876127f4565b61340160208301866133d2565b61340e60408301856127f4565b61341b60608301846127f4565b95945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffdfea2646970667358221220173f86625c8f14b1d4252675f9c0e1604f04c9a25f74194eb1587c976fc8d50764736f6c63430008210033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`@QaE\xA08\x03\x80aE\xA0\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\x0E\x0FV[\x85\x84\x83\x833\x81\x84\x84a\0I\x82\x82a\x01c` \x1B` \x1CV[PPa\0Z\x81a\x04Y` \x1B` \x1CV[P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\0\xCBW_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\0\xC2\x91\x90a\x0E\xC3V[`@Q\x80\x91\x03\x90\xFD[a\0\xDA\x81a\x04\xAC` \x1B` \x1CV[Pa\x01\x05\x84\x83_\x81Q\x81\x10a\0\xF2Wa\0\xF1a\x0E\xDCV[[` \x02` \x01\x01Qa\x05o` \x1B` \x1CV[PPPPB`\n\x81\x90UPC`\x0B\x81\x90UP\x7F\xDE\xF1\xF0\x8E\xB6U\xF4\xA7_`\xBDo\xD7\xE9q\x11 \x04\xAB\xD8F\xA6\x12\xE4m\xAB'\x07p\xD2L\xA50`\nT`\x0BT3`@Qa\x01P\x94\x93\x92\x91\x90a\x0F\x18V[`@Q\x80\x91\x03\x90\xA1PPPPPPa\x11\x0CV[_`\x01\x83`\x03a\x01s\x91\x90a\x0F\x88V[a\x01}\x91\x90a\x0F\xC9V[\x90P\x80\x82Q\x10\x15a\x01\xC8W\x81Q\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x01\xBF\x92\x91\x90a\x0F\xFCV[`@Q\x80\x91\x03\x90\xFD[\x82`\x02\x81\x90UP_a\x01\xFF\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x05\xF2` \x1B` \x1CV[\x90P_a\x021\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x06\x1A` \x1B` \x1CV[\x90P__\x90P[\x81\x81\x10\x15a\x02\x98Wa\x02\x8A\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x84\x83\x81Q\x81\x10a\x02wWa\x02va\x0E\xDCV[[` \x02` \x01\x01Qa\x06A` \x1B` \x1CV[P\x80\x80`\x01\x01\x91PPa\x028V[P_a\x02\xC9\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x05\xF2` \x1B` \x1CV[\x90P_a\x02\xFB\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x06\x1A` \x1B` \x1CV[\x90P__\x90P[\x81\x81\x10\x15a\x03bWa\x03T\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x84\x83\x81Q\x81\x10a\x03AWa\x03@a\x0E\xDCV[[` \x02` \x01\x01Qa\x06A` \x1B` \x1CV[P\x80\x80`\x01\x01\x91PPa\x03\x02V[P__\x90P[\x86Q\x81\x10\x15a\x03\xC9Wa\x03\xBB\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x88\x83\x81Q\x81\x10a\x03\xA8Wa\x03\xA7a\x0E\xDCV[[` \x02` \x01\x01Qa\x06Z` \x1B` \x1CV[P\x80\x80`\x01\x01\x91PPa\x03hV[Pa\x04\x14\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x87_\x81Q\x81\x10a\x04\x01Wa\x04\0a\x0E\xDCV[[` \x02` \x01\x01Qa\x06Z` \x1B` \x1CV[P\x7F\xA2\xDFx0\xE0\xBE\xDE\xF7\xB1\x11k\xF5G\xB4g\xB1kP\xB3\xBD#\x14l\x9E\t\x98x\xD1N\x890\x1A\x85\x883`@Qa\x04H\x93\x92\x91\x90a\x10#V[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[\x80`\x04\x81\x90UP_`\x05\x81\x90UP_`\x06\x81\x90UP\x7F\xF7\xF0\x87#\x82\xDF\xF5\xE6\x98\xB2\x84\xE1 \x84\xE4\xE7\x89O\x83\x02\x16\xDD\x80\xCBN\x90\x9BY:X\xF95`\x04T3`@Qa\x04\xA1\x92\x91\x90a\x10XV[`@Q\x80\x91\x03\x90\xA1PV[_`\x08_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\x08_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[\x81`\t\x81\x90UPB`\n\x81\x90UPC`\x0B\x81\x90UP_`\x0C_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x05\xA8Wa\x05\xA7a\x10\x7FV[[\x02\x17\x90UP\x7F\xDE\xF1\xF0\x8E\xB6U\xF4\xA7_`\xBDo\xD7\xE9q\x11 \x04\xAB\xD8F\xA6\x12\xE4m\xAB'\x07p\xD2L\xA50`\nT`\x0BT\x84`@Qa\x05\xE6\x94\x93\x92\x91\x90a\x0F\x18V[`@Q\x80\x91\x03\x90\xA1PPV[``a\x06\x13`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a\x06s` \x1B` \x1CV[\x90P\x91\x90PV[_a\x06:`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a\x06\x98` \x1B` \x1CV[\x90P\x91\x90PV[_a\x06R\x83\x83a\x06\xB1` \x1B` \x1CV[\x90P\x92\x91PPV[_a\x06k\x83\x83a\x06\xFA` \x1B` \x1CV[\x90P\x92\x91PPV[``_a\x06\x87\x83_\x01a\x07C` \x1B` \x1CV[\x90P``\x81\x90P\x80\x92PPP\x91\x90PV[_a\x06\xAA\x82_\x01a\x07\x9C` \x1B` \x1CV[\x90P\x91\x90PV[__a\x06\xC3\x84\x84a\x07\xAB` \x1B` \x1CV[\x90P\x80\x15a\x06\xF0Wa\x06\xEE\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a\x08\xA0` \x1B\x90\x91\x90` \x1CV[P[\x80\x91PP\x92\x91PPV[__a\x07\x0C\x84\x84a\x08\xD3` \x1B` \x1CV[\x90P\x80\x15a\x079Wa\x077\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a\t\xC8` \x1B\x90\x91\x90` \x1CV[P[\x80\x91PP\x92\x91PPV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07\x90W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x07|W[PPPPP\x90P\x91\x90PV[_\x81_\x01\x80T\x90P\x90P\x91\x90PV[_a\x07\xBC\x83\x83a\t\xFB` \x1B` \x1CV[\x15a\x08\x96W___\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x083a\n^` \x1B` \x1CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\x08\x9AV[_\x90P[\x92\x91PPV[_a\x08\xCB\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba\ne` \x1B` \x1CV[\x90P\x92\x91PPV[_a\x08\xE4\x83\x83a\t\xFB` \x1B` \x1CV[a\t\xBEW`\x01__\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\t[a\n^` \x1B` \x1CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\t\xC2V[_\x90P[\x92\x91PPV[_a\t\xF3\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba\x0Ba` \x1B` \x1CV[\x90P\x92\x91PPV[___\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[_3\x90P\x90V[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x14a\x0BVW_`\x01\x82a\n\x92\x91\x90a\x10\xACV[\x90P_`\x01\x86_\x01\x80T\x90Pa\n\xA8\x91\x90a\x10\xACV[\x90P\x80\x82\x14a\x0B\x0EW_\x86_\x01\x82\x81T\x81\x10a\n\xC7Wa\n\xC6a\x0E\xDCV[[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a\n\xE8Wa\n\xE7a\x0E\xDCV[[\x90_R` _ \x01\x81\x90UP\x83\x87`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UPP[\x85_\x01\x80T\x80a\x0B!Wa\x0B a\x10\xDFV[[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\x0B[V[_\x91PP[\x92\x91PPV[_a\x0Br\x83\x83a\x0B\xCE` \x1B` \x1CV[a\x0B\xC4W\x82_\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x82_\x01\x80T\x90P\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP`\x01\x90Pa\x0B\xC8V[_\x90P[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x14\x15\x90P\x92\x91PPV[_`@Q\x90P\x90V[__\xFD[__\xFD[_\x81\x90P\x91\x90PV[a\x0C\x11\x81a\x0B\xFFV[\x81\x14a\x0C\x1BW__\xFD[PV[_\x81Q\x90Pa\x0C,\x81a\x0C\x08V[\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x0CD\x81a\x0C2V[\x81\x14a\x0CNW__\xFD[PV[_\x81Q\x90Pa\x0C_\x81a\x0C;V[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x0C\x8E\x82a\x0CeV[\x90P\x91\x90PV[a\x0C\x9E\x81a\x0C\x84V[\x81\x14a\x0C\xA8W__\xFD[PV[_\x81Q\x90Pa\x0C\xB9\x81a\x0C\x95V[\x92\x91PPV[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\r\t\x82a\x0C\xC3V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\r(Wa\r'a\x0C\xD3V[[\x80`@RPPPV[_a\r:a\x0B\xEEV[\x90Pa\rF\x82\x82a\r\0V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\reWa\rda\x0C\xD3V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[__\xFD[_a\r\x8Ca\r\x87\x84a\rKV[a\r1V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\r\xAFWa\r\xAEa\rvV[[\x83[\x81\x81\x10\x15a\r\xD8W\x80a\r\xC4\x88\x82a\x0C\xABV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\r\xB1V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\r\xF6Wa\r\xF5a\x0C\xBFV[[\x81Qa\x0E\x06\x84\x82` \x86\x01a\rzV[\x91PP\x92\x91PPV[______`\xC0\x87\x89\x03\x12\x15a\x0E)Wa\x0E(a\x0B\xF7V[[_a\x0E6\x89\x82\x8A\x01a\x0C\x1EV[\x96PP` a\x0EG\x89\x82\x8A\x01a\x0CQV[\x95PP`@a\x0EX\x89\x82\x8A\x01a\x0CQV[\x94PP``a\x0Ei\x89\x82\x8A\x01a\x0C\xABV[\x93PP`\x80\x87\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x8AWa\x0E\x89a\x0B\xFBV[[a\x0E\x96\x89\x82\x8A\x01a\r\xE2V[\x92PP`\xA0a\x0E\xA7\x89\x82\x8A\x01a\x0CQV[\x91PP\x92\x95P\x92\x95P\x92\x95V[a\x0E\xBD\x81a\x0C\x84V[\x82RPPV[_` \x82\x01\x90Pa\x0E\xD6_\x83\x01\x84a\x0E\xB4V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[a\x0F\x12\x81a\x0C2V[\x82RPPV[_`\x80\x82\x01\x90Pa\x0F+_\x83\x01\x87a\x0E\xB4V[a\x0F8` \x83\x01\x86a\x0F\tV[a\x0FE`@\x83\x01\x85a\x0F\tV[a\x0FR``\x83\x01\x84a\x0E\xB4V[\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x0F\x92\x82a\x0C2V[\x91Pa\x0F\x9D\x83a\x0C2V[\x92P\x82\x82\x02a\x0F\xAB\x81a\x0C2V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x0F\xC2Wa\x0F\xC1a\x0F[V[[P\x92\x91PPV[_a\x0F\xD3\x82a\x0C2V[\x91Pa\x0F\xDE\x83a\x0C2V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x0F\xF6Wa\x0F\xF5a\x0F[V[[\x92\x91PPV[_`@\x82\x01\x90Pa\x10\x0F_\x83\x01\x85a\x0F\tV[a\x10\x1C` \x83\x01\x84a\x0F\tV[\x93\x92PPPV[_``\x82\x01\x90Pa\x106_\x83\x01\x86a\x0F\tV[a\x10C` \x83\x01\x85a\x0F\tV[a\x10P`@\x83\x01\x84a\x0E\xB4V[\x94\x93PPPPV[_`@\x82\x01\x90Pa\x10k_\x83\x01\x85a\x0F\tV[a\x10x` \x83\x01\x84a\x0E\xB4V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_a\x10\xB6\x82a\x0C2V[\x91Pa\x10\xC1\x83a\x0C2V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x10\xD9Wa\x10\xD8a\x0F[V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD[a4\x87\x80a\x11\x19_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02\x0FW_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\x01#W\x80c\xCA\x15\xC8s\x11a\0\xABW\x80c\xE2?\xB1\xF8\x11a\0zW\x80c\xE2?\xB1\xF8\x14a\x05yW\x80c\xEB\xAE5\xE7\x14a\x05\x95W\x80c\xF2\xFD\xE3\x8B\x14a\x05\xC5W\x80c\xF9\xD8\xFF\x15\x14a\x05\xE1W\x80c\xFCx\xB2\xE8\x14a\x06\x11Wa\x02\x0FV[\x80c\xCA\x15\xC8s\x14a\x05\x05W\x80c\xCB\x9CL\xC4\x14a\x055W\x80c\xD5Gt\x1F\x14a\x05?W\x80c\xD8'\r\xCE\x14a\x05[Wa\x02\x0FV[\x80c\xA2\x17\xFD\xDF\x11a\0\xF2W\x80c\xA2\x17\xFD\xDF\x14a\x04\x87W\x80c\xA3$j\xD3\x14a\x04\xA5W\x80c\xAF o(\x14a\x04\xD5W\x80c\xBBQ\xFE\xF0\x14a\x04\xF1W\x80c\xC0y\xF4\x95\x14a\x04\xFBWa\x02\x0FV[\x80c\x8D\xA5\xCB[\x14a\x03\xEDW\x80c\x90\x10\xD0|\x14a\x04\x0BW\x80c\x91\xD1HT\x14a\x04;W\x80c\x93\x0C\xEA\xE6\x14a\x04kWa\x02\x0FV[\x80c0\x10L>\x11a\x01\xA6W\x80cK\xB2x\xF3\x11a\x01uW\x80cK\xB2x\xF3\x14a\x03\x95W\x80cY_\x80n\x14a\x03\x9FW\x80cktG(\x14a\x03\xA9W\x80cqP\x18\xA6\x14a\x03\xC5W\x80c\x7F5\xB5`\x14a\x03\xCFWa\x02\x0FV[\x80c0\x10L>\x14a\x035W\x80c6V\x8A\xBE\x14a\x03SW\x80cI%\xE6*\x14a\x03oW\x80cK\x8Ed\x88\x14a\x03\x8BWa\x02\x0FV[\x80c\x17cE\x14\x11a\x01\xE2W\x80c\x17cE\x14\x14a\x02\xADW\x80c#(\xBD\x12\x14a\x02\xCBW\x80c$\x8A\x9C\xA3\x14a\x02\xE9W\x80c//\xF1]\x14a\x03\x19Wa\x02\x0FV[\x80c\x01\xFF\xC9\xA7\x14a\x02\x13W\x80c\x0B\xDA\x81\xCF\x14a\x02CW\x80c\x13\xFFm\xD5\x14a\x02_W\x80c\x14l\xA51\x14a\x02\x8FW[__\xFD[a\x02-`\x04\x806\x03\x81\x01\x90a\x02(\x91\x90a%\x8EV[a\x06AV[`@Qa\x02:\x91\x90a%\xD3V[`@Q\x80\x91\x03\x90\xF3[a\x02]`\x04\x806\x03\x81\x01\x90a\x02X\x91\x90a&\x1FV[a\x06\xBAV[\0[a\x02y`\x04\x806\x03\x81\x01\x90a\x02t\x91\x90a&\xB7V[a\x08\x19V[`@Qa\x02\x86\x91\x90a%\xD3V[`@Q\x80\x91\x03\x90\xF3[a\x02\x97a\x08\x95V[`@Qa\x02\xA4\x91\x90a'UV[`@Q\x80\x91\x03\x90\xF3[a\x02\xB5a\x08\xA7V[`@Qa\x02\xC2\x91\x90a'}V[`@Q\x80\x91\x03\x90\xF3[a\x02\xD3a\x08\xADV[`@Qa\x02\xE0\x91\x90a'}V[`@Q\x80\x91\x03\x90\xF3[a\x03\x03`\x04\x806\x03\x81\x01\x90a\x02\xFE\x91\x90a'\xC9V[a\x08\xC3V[`@Qa\x03\x10\x91\x90a(\x03V[`@Q\x80\x91\x03\x90\xF3[a\x033`\x04\x806\x03\x81\x01\x90a\x03.\x91\x90a(\x1CV[a\x08\xDFV[\0[a\x03=a\t\x19V[`@Qa\x03J\x91\x90a(\x03V[`@Q\x80\x91\x03\x90\xF3[a\x03m`\x04\x806\x03\x81\x01\x90a\x03h\x91\x90a(\x1CV[a\t=V[\0[a\x03\x89`\x04\x806\x03\x81\x01\x90a\x03\x84\x91\x90a(ZV[a\n\x04V[\0[a\x03\x93a\n\xC6V[\0[a\x03\x9Da\x0B@V[\0[a\x03\xA7a\x0B\xBDV[\0[a\x03\xC3`\x04\x806\x03\x81\x01\x90a\x03\xBE\x91\x90a(\xF9V[a\x0C7V[\0[a\x03\xCDa\x0CzV[\0[a\x03\xD7a\x0C\x8DV[`@Qa\x03\xE4\x91\x90a(\x03V[`@Q\x80\x91\x03\x90\xF3[a\x03\xF5a\x0C\xB1V[`@Qa\x04\x02\x91\x90a)SV[`@Q\x80\x91\x03\x90\xF3[a\x04%`\x04\x806\x03\x81\x01\x90a\x04 \x91\x90a)lV[a\x0C\xD9V[`@Qa\x042\x91\x90a)SV[`@Q\x80\x91\x03\x90\xF3[a\x04U`\x04\x806\x03\x81\x01\x90a\x04P\x91\x90a(\x1CV[a\r\x05V[`@Qa\x04b\x91\x90a%\xD3V[`@Q\x80\x91\x03\x90\xF3[a\x04\x85`\x04\x806\x03\x81\x01\x90a\x04\x80\x91\x90a*\xF2V[a\rhV[\0[a\x04\x8Fa\r\xD1V[`@Qa\x04\x9C\x91\x90a(\x03V[`@Q\x80\x91\x03\x90\xF3[a\x04\xBF`\x04\x806\x03\x81\x01\x90a\x04\xBA\x91\x90a'\xC9V[a\r\xD7V[`@Qa\x04\xCC\x91\x90a,<V[`@Q\x80\x91\x03\x90\xF3[a\x04\xEF`\x04\x806\x03\x81\x01\x90a\x04\xEA\x91\x90a,\\V[a\r\xF9V[\0[a\x04\xF9a\x0E2V[\0[a\x05\x03a\x0E\xACV[\0[a\x05\x1F`\x04\x806\x03\x81\x01\x90a\x05\x1A\x91\x90a'\xC9V[a\x0F&V[`@Qa\x05,\x91\x90a'}V[`@Q\x80\x91\x03\x90\xF3[a\x05=a\x0FGV[\0[a\x05Y`\x04\x806\x03\x81\x01\x90a\x05T\x91\x90a(\x1CV[a\x0F\xC0V[\0[a\x05ca\x10\xB3V[`@Qa\x05p\x91\x90a'}V[`@Q\x80\x91\x03\x90\xF3[a\x05\x93`\x04\x806\x03\x81\x01\x90a\x05\x8E\x91\x90a,\xB6V[a\x10\xB9V[\0[a\x05\xAF`\x04\x806\x03\x81\x01\x90a\x05\xAA\x91\x90a,\xB6V[a\x10\xF0V[`@Qa\x05\xBC\x91\x90a-\x98V[`@Q\x80\x91\x03\x90\xF3[a\x05\xDF`\x04\x806\x03\x81\x01\x90a\x05\xDA\x91\x90a&\xB7V[a\x12\xB6V[\0[a\x05\xFB`\x04\x806\x03\x81\x01\x90a\x05\xF6\x91\x90a-\xB8V[a\x13:V[`@Qa\x06\x08\x91\x90a%\xD3V[`@Q\x80\x91\x03\x90\xF3[a\x06+`\x04\x806\x03\x81\x01\x90a\x06&\x91\x90a&\xB7V[a\x14\x82V[`@Qa\x068\x91\x90a%\xD3V[`@Q\x80\x91\x03\x90\xF3[_\x7F\x07\xEF\xFE\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x06\xB3WPa\x06\xB2\x82a\x14\xB4V[[\x90P\x91\x90PV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07\\W3\x81`@Q\x7F\xFF\xAB\xBA\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07S\x92\x91\x90a.)V[`@Q\x80\x91\x03\x90\xFD[`@Q\x80`@\x01`@R\x80\x82\x81R` \x01\x83\x81RP`\x07_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01U` \x82\x01Q\x81`\x01\x01U\x90PP\x7F\xB8\x9A\xDD\xD97\xF4O\x90,\x84\x95\x96d\x187\xCDz\xF2\xFC\xEC\xEF\"\xD2\xA7\x86o\xDC\x1A\xD9\xC0\xAE.3\x83\x83`@Qa\x07\xF6\x93\x92\x91\x90a.PV[`@Q\x80\x91\x03\x90\xA1`\x06_\x81T\x80\x92\x91\x90a\x08\x10\x90a.\xB2V[\x91\x90PUPPPV[_a\x08#\x82a\x14\x82V[a\x08dW\x81`@Q\x7F\xAB\xDC\xE0j\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08[\x91\x90a)SV[`@Q\x80\x91\x03\x90\xFD[a\x08\x8E\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x83a\r\x05V[\x90P\x91\x90PV[`\x0C_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\x0BT\x81V[_`\x05T`\x04Ta\x08\xBE\x91\x90a.\xF9V[\x90P\x90V[___\x83\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\t\t\x81a\x15-V[a\t\x13\x83\x83a\x15AV[PPPPV[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x81V[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x82\x14\x80\x15a\trWPa\tq\x82\x82a\r\x05V[[\x15a\t\xF6W_a\t\x81\x83a\x0F&V[\x90P_`\x01`\x02T`\x03a\t\x95\x91\x90a/,V[a\t\x9F\x91\x90a/mV[\x90P\x80\x82\x03a\t\xF3W`\x01\x81a\t\xB5\x91\x90a.\xF9V[\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xEA\x92\x91\x90a/\xA0V[`@Q\x80\x91\x03\x90\xFD[PP[a\n\0\x82\x82a\x15TV[PPV[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\n.\x81a\x15-V[`\x01`\r`\x01\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPPPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\n\xF0\x81a\x15-V[`\x04a\n\xFB\x81a\x15\xCFV[\x7F0\x1F\x8A7\x01\xF5\xB2`\x19s\x82\xDDs\x01\x07\x85B\x14O\xE8\xFD\xDD\x18\x08=on\t\xE4\x95\x8AY3B`@Qa\x0B,\x92\x91\x90a.)V[`@Q\x80\x91\x03\x90\xA1a\x0B<a\x16XV[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0Bj\x81a\x15-V[`\x05a\x0Bu\x81a\x15\xCFV[_\x7F$\xA8se\x1D&\xFBZF,\xB3z\x91\x07\x1C\xDDM\t\xAB\xEE\xBF\xE0\xED\x14C)\xBE\xD1\xCC5\x9D\x033B`@Qa\x0B\xA7\x92\x91\x90a.)V[`@Q\x80\x91\x03\x90\xA1a\x0B\xB8\x81a\x16\xC1V[PPPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0B\xE7\x81a\x15-V[`\x03a\x0B\xF2\x81a\x15\xCFV[\x7F \xF5^\xD0\xC9/+\xB1\xC8\x82T\x88\xE1\xE3\xC9\x84c\xD0$\xB2\xA4-\xBD$\x83\x8C?u&\x0FC\xE93B`@Qa\x0C#\x92\x91\x90a.)V[`@Q\x80\x91\x03\x90\xA1a\x0C3a\x16XV[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0Ca\x81a\x15-V[\x82\x82`\r_\x01\x91\x82a\x0Ct\x92\x91\x90a1\xDFV[PPPPV[a\x0C\x82a\x16\xEDV[a\x0C\x8B_a\x17tV[V[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x81V[_`\x08_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_a\x0C\xFD\x82`\x01_\x86\x81R` \x01\x90\x81R` \x01_ a\x187\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[___\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\r\x92\x81a\x15-V[a\r\x9C\x84\x84a\x18NV[a\r\xA5\x82a\x1B\x14V[a\r\xC9\x86\x84_\x81Q\x81\x10a\r\xBCWa\r\xBBa2\xACV[[` \x02` \x01\x01Qa\x1BgV[PPPPPPV[__\x1B\x81V[``a\r\xF2`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a\x1B\xEAV[\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0E#\x81a\x15-V[a\x0E-\x83\x83a\x18NV[PPPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0E\\\x81a\x15-V[`\x02a\x0Eg\x81a\x15\xCFV[\x7F`\xED\xF9\xBD\xC7\xC4\xEA\0|\xAE\x1A\x9B\xBD\x03\xE4\x1E[\xFC\xCDr1\xA6\xEC8<.\xDDx\0\xF0\xD2\x0C3B`@Qa\x0E\x98\x92\x91\x90a.)V[`@Q\x80\x91\x03\x90\xA1a\x0E\xA8a\x16XV[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0E\xD6\x81a\x15-V[`\x01a\x0E\xE1\x81a\x15\xCFV[\x7Fg\xC4H\x9FgM\x03\xC7\xD1\x9A\x9E6sQ\x88\xDE|e\xE8\xD1\xE9\x9E\xB3\xA2\xFD%\x8Av\x9E\xB1O\xFF3B`@Qa\x0F\x12\x92\x91\x90a.)V[`@Q\x80\x91\x03\x90\xA1a\x0F\"a\x16XV[PPV[_a\x0F@`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a\x1C\tV[\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0Fq\x81a\x15-V[_a\x0F{\x81a\x15\xCFV[\x7F\xBBp\x9D\xEAtO\x06\xD1\xB2n\x82M\xEE\xC2\xF7\x14\x0CQ\x12f\xEE\x15\xD7\xA2\x17\x83\x8B1\xD8\xB0\x12=3B`@Qa\x0F\xAC\x92\x91\x90a.)V[`@Q\x80\x91\x03\x90\xA1a\x0F\xBCa\x16XV[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0F\xEA\x81a\x15-V[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x83\x14\x80\x15a\x10\x1FWPa\x10\x1E\x83\x83a\r\x05V[[\x15a\x10\xA3W_a\x10.\x84a\x0F&V[\x90P_`\x01`\x02T`\x03a\x10B\x91\x90a/,V[a\x10L\x91\x90a/mV[\x90P\x80\x82\x03a\x10\xA0W`\x01\x81a\x10b\x91\x90a.\xF9V[\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\x97\x92\x91\x90a/\xA0V[`@Q\x80\x91\x03\x90\xFD[PP[a\x10\xAD\x83\x83a\x1C\x1CV[PPPPV[`\nT\x81V[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x10\xE3\x81a\x15-V[a\x10\xEC\x82a\x1B\x14V[PPV[``_`\x05T`\x04Ta\x11\x03\x91\x90a.\xF9V[\x90P\x80\x83\x11\x15a\x11LW\x82\x81`@Q\x7F\xDF=u\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11C\x92\x91\x90a/\xA0V[`@Q\x80\x91\x03\x90\xFD[_`\x01\x84`\x05Ta\x11]\x91\x90a/mV[a\x11g\x91\x90a.\xF9V[\x90P_\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\x84Wa\x11\x83a)\xBAV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11\xB2W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_`\x05T\x90P[\x82`\x05T\x11a\x12\x9CW3`\x03_`\x05T\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x05T\x82\x82`\x05Ta\x12(\x91\x90a.\xF9V[\x81Q\x81\x10a\x129Wa\x128a2\xACV[[` \x02` \x01\x01\x81\x81RPP\x7F\xAB\xDE\x16\xB7\xA9\x19,1\xC6#\x1B\x159\xBA\xD6\xFE\xD7v5\xDEL\0\x87\x18\xDB\xDC\xAF\xB7\xB86:\xFE3`\x05T`@Qa\x12x\x92\x91\x90a.)V[`@Q\x80\x91\x03\x90\xA1`\x05_\x81T\x80\x92\x91\x90a\x12\x92\x90a.\xB2V[\x91\x90PUPa\x11\xBCV[\x85\x84a\x12\xA8\x91\x90a.\xF9V[\x93P\x81\x94PPPPP\x91\x90PV[a\x12\xBEa\x16\xEDV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x13.W_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13%\x91\x90a)SV[`@Q\x80\x91\x03\x90\xFD[a\x137\x81a\x17tV[PV[_\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x13e\x81a\x15-V[_a\x13\x95\x87`@Q` \x01a\x13z\x91\x90a'}V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x1C/V[\x90P_a\x13\xE5\x82\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa\x1CbV[\x90P\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x14sW\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F#\xD3iX<\xE20#\xA1\xCAK\xAF\x14\xA4\x86\x93\xA3\xEE\x9B:\xBF$\x83\xE4\x1Dc\x9EW\xCB\x1F\x9D\xAB\x89`@Qa\x14`\x91\x90a'}V[`@Q\x80\x91\x03\x90\xA2`\x01\x93PPPa\x14yV[_\x93PPP[P\x94\x93PPPPV[_a\x14\xAD\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x83a\r\x05V[\x90P\x91\x90PV[_\x7FZ\x05\x18\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x15&WPa\x15%\x82a\x1C\x8CV[[\x90P\x91\x90PV[a\x15>\x81a\x159a\x1D\x05V[a\x1D\x0CV[PV[_a\x15L\x83\x83a\x1D]V[\x90P\x92\x91PPV[a\x15\\a\x1D\x05V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x15\xC0W`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x15\xCA\x82\x82a\x1C\x1CV[PPPV[\x80`\x05\x81\x11\x15a\x15\xE2Wa\x15\xE1a&\xE2V[[`\x0C_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x05\x81\x11\x15a\x16\x03Wa\x16\x02a&\xE2V[[\x14a\x16UW\x80`\x0C_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x7F\xBF\xA2\x17\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16L\x92\x91\x90a2\xD9V[`@Q\x80\x91\x03\x90\xFD[PV[`\x01`\x0C_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x05\x81\x11\x15a\x16{Wa\x16za&\xE2V[[a\x16\x85\x91\x90a/mV[`\x05\x81\x11\x15a\x16\x97Wa\x16\x96a&\xE2V[[`\x0C_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x16\xBAWa\x16\xB9a&\xE2V[[\x02\x17\x90UPV[\x80`\x0C_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x16\xE5Wa\x16\xE4a&\xE2V[[\x02\x17\x90UPPV[a\x16\xF5a\x1D\x05V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x17\x13a\x0C\xB1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x17rWa\x176a\x1D\x05V[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17i\x91\x90a)SV[`@Q\x80\x91\x03\x90\xFD[V[_`\x08_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\x08_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[_a\x18D\x83_\x01\x83a\x1D\xA0V[_\x1C\x90P\x92\x91PPV[_`\x01\x83`\x03a\x18^\x91\x90a/,V[a\x18h\x91\x90a/mV[\x90P\x80\x82Q\x10\x15a\x18\xB3W\x81Q\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\xAA\x92\x91\x90a/\xA0V[`@Q\x80\x91\x03\x90\xFD[\x82`\x02\x81\x90UP_a\x18\xE4\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\r\xD7V[\x90P_a\x19\x10\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0F&V[\x90P__\x90P[\x81\x81\x10\x15a\x19qWa\x19c\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x84\x83\x81Q\x81\x10a\x19VWa\x19Ua2\xACV[[` \x02` \x01\x01Qa\x1C\x1CV[P\x80\x80`\x01\x01\x91PPa\x19\x17V[P_a\x19\x9C\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\r\xD7V[\x90P_a\x19\xC8\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x0F&V[\x90P__\x90P[\x81\x81\x10\x15a\x1A)Wa\x1A\x1B\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x84\x83\x81Q\x81\x10a\x1A\x0EWa\x1A\ra2\xACV[[` \x02` \x01\x01Qa\x1C\x1CV[P\x80\x80`\x01\x01\x91PPa\x19\xCFV[P__\x90P[\x86Q\x81\x10\x15a\x1A\x8AWa\x1A|\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x88\x83\x81Q\x81\x10a\x1AoWa\x1Ana2\xACV[[` \x02` \x01\x01Qa\x15AV[P\x80\x80`\x01\x01\x91PPa\x1A/V[Pa\x1A\xCF\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x87_\x81Q\x81\x10a\x1A\xC2Wa\x1A\xC1a2\xACV[[` \x02` \x01\x01Qa\x15AV[P\x7F\xA2\xDFx0\xE0\xBE\xDE\xF7\xB1\x11k\xF5G\xB4g\xB1kP\xB3\xBD#\x14l\x9E\t\x98x\xD1N\x890\x1A\x85\x883`@Qa\x1B\x03\x93\x92\x91\x90a3\0V[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[\x80`\x04\x81\x90UP_`\x05\x81\x90UP_`\x06\x81\x90UP\x7F\xF7\xF0\x87#\x82\xDF\xF5\xE6\x98\xB2\x84\xE1 \x84\xE4\xE7\x89O\x83\x02\x16\xDD\x80\xCBN\x90\x9BY:X\xF95`\x04T3`@Qa\x1B\\\x92\x91\x90a35V[`@Q\x80\x91\x03\x90\xA1PV[\x81`\t\x81\x90UPB`\n\x81\x90UPC`\x0B\x81\x90UP_`\x0C_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x1B\xA0Wa\x1B\x9Fa&\xE2V[[\x02\x17\x90UP\x7F\xDE\xF1\xF0\x8E\xB6U\xF4\xA7_`\xBDo\xD7\xE9q\x11 \x04\xAB\xD8F\xA6\x12\xE4m\xAB'\x07p\xD2L\xA50`\nT`\x0BT\x84`@Qa\x1B\xDE\x94\x93\x92\x91\x90a3\\V[`@Q\x80\x91\x03\x90\xA1PPV[``_a\x1B\xF8\x83_\x01a\x1D\xC7V[\x90P``\x81\x90P\x80\x92PPP\x91\x90PV[_a\x1C\x15\x82_\x01a\x1E V[\x90P\x91\x90PV[_a\x1C'\x83\x83a\x1E/V[\x90P\x92\x91PPV[_\x7F\x19Ethereum Signed Message:\n32\0\0\0\0_R\x81`\x1CR`<_ \x90P\x91\x90PV[____a\x1Cp\x86\x86a\x1ErV[\x92P\x92P\x92Pa\x1C\x80\x82\x82a\x1E\xC7V[\x82\x93PPPP\x92\x91PPV[_\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x1C\xFEWPa\x1C\xFD\x82a )V[[\x90P\x91\x90PV[_3\x90P\x90V[a\x1D\x16\x82\x82a\r\x05V[a\x1DYW\x80\x82`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1DP\x92\x91\x90a3\x9FV[`@Q\x80\x91\x03\x90\xFD[PPV[__a\x1Di\x84\x84a \x92V[\x90P\x80\x15a\x1D\x96Wa\x1D\x94\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a!{\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[_\x82_\x01\x82\x81T\x81\x10a\x1D\xB6Wa\x1D\xB5a2\xACV[[\x90_R` _ \x01T\x90P\x92\x91PPV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1E\x14W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x1E\0W[PPPPP\x90P\x91\x90PV[_\x81_\x01\x80T\x90P\x90P\x91\x90PV[__a\x1E;\x84\x84a!\xA8V[\x90P\x80\x15a\x1EhWa\x1Ef\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a\"\x91\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[___`A\x84Q\x03a\x1E\xB2W___` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90Pa\x1E\xA4\x88\x82\x85\x85a\"\xBEV[\x95P\x95P\x95PPPPa\x1E\xC0V[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15a\x1E\xDAWa\x1E\xD9a&\xE2V[[\x82`\x03\x81\x11\x15a\x1E\xEDWa\x1E\xECa&\xE2V[[\x03\x15a %W`\x01`\x03\x81\x11\x15a\x1F\x07Wa\x1F\x06a&\xE2V[[\x82`\x03\x81\x11\x15a\x1F\x1AWa\x1F\x19a&\xE2V[[\x03a\x1FQW`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15a\x1FeWa\x1Fda&\xE2V[[\x82`\x03\x81\x11\x15a\x1FxWa\x1Fwa&\xE2V[[\x03a\x1F\xBCW\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1F\xB3\x91\x90a'}V[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15a\x1F\xCFWa\x1F\xCEa&\xE2V[[\x82`\x03\x81\x11\x15a\x1F\xE2Wa\x1F\xE1a&\xE2V[[\x03a $W\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a \x1B\x91\x90a(\x03V[`@Q\x80\x91\x03\x90\xFD[[PPV[_\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[_a \x9D\x83\x83a\r\x05V[a!qW`\x01__\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa!\x0Ea\x1D\x05V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa!uV[_\x90P[\x92\x91PPV[_a!\xA0\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba#\xA5V[\x90P\x92\x91PPV[_a!\xB3\x83\x83a\r\x05V[\x15a\"\x87W___\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\"$a\x1D\x05V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\"\x8BV[_\x90P[\x92\x91PPV[_a\"\xB6\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba$\x0CV[\x90P\x92\x91PPV[___\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15a\"\xFAW_`\x03\x85\x92P\x92P\x92Pa#\x9BV[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@Qa#\x1D\x94\x93\x92\x91\x90a3\xE1V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a#=W=__>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a#\x8EW_`\x01__\x1B\x93P\x93P\x93PPa#\x9BV[\x80___\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[_a#\xB0\x83\x83a%\x08V[a$\x02W\x82_\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x82_\x01\x80T\x90P\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP`\x01\x90Pa$\x06V[_\x90P[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x14a$\xFDW_`\x01\x82a$9\x91\x90a.\xF9V[\x90P_`\x01\x86_\x01\x80T\x90Pa$O\x91\x90a.\xF9V[\x90P\x80\x82\x14a$\xB5W_\x86_\x01\x82\x81T\x81\x10a$nWa$ma2\xACV[[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a$\x8FWa$\x8Ea2\xACV[[\x90_R` _ \x01\x81\x90UP\x83\x87`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UPP[\x85_\x01\x80T\x80a$\xC8Wa$\xC7a4$V[[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa%\x02V[_\x91PP[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x14\x15\x90P\x92\x91PPV[_`@Q\x90P\x90V[__\xFD[__\xFD[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a%m\x81a%9V[\x81\x14a%wW__\xFD[PV[_\x815\x90Pa%\x88\x81a%dV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a%\xA3Wa%\xA2a%1V[[_a%\xB0\x84\x82\x85\x01a%zV[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a%\xCD\x81a%\xB9V[\x82RPPV[_` \x82\x01\x90Pa%\xE6_\x83\x01\x84a%\xC4V[\x92\x91PPV[_\x81\x90P\x91\x90PV[a%\xFE\x81a%\xECV[\x81\x14a&\x08W__\xFD[PV[_\x815\x90Pa&\x19\x81a%\xF5V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a&5Wa&4a%1V[[_a&B\x85\x82\x86\x01a&\x0BV[\x92PP` a&S\x85\x82\x86\x01a&\x0BV[\x91PP\x92P\x92\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a&\x86\x82a&]V[\x90P\x91\x90PV[a&\x96\x81a&|V[\x81\x14a&\xA0W__\xFD[PV[_\x815\x90Pa&\xB1\x81a&\x8DV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a&\xCCWa&\xCBa%1V[[_a&\xD9\x84\x82\x85\x01a&\xA3V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x06\x81\x10a' Wa'\x1Fa&\xE2V[[PV[_\x81\x90Pa'0\x82a'\x0FV[\x91\x90PV[_a'?\x82a'#V[\x90P\x91\x90PV[a'O\x81a'5V[\x82RPPV[_` \x82\x01\x90Pa'h_\x83\x01\x84a'FV[\x92\x91PPV[a'w\x81a%\xECV[\x82RPPV[_` \x82\x01\x90Pa'\x90_\x83\x01\x84a'nV[\x92\x91PPV[_\x81\x90P\x91\x90PV[a'\xA8\x81a'\x96V[\x81\x14a'\xB2W__\xFD[PV[_\x815\x90Pa'\xC3\x81a'\x9FV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a'\xDEWa'\xDDa%1V[[_a'\xEB\x84\x82\x85\x01a'\xB5V[\x91PP\x92\x91PPV[a'\xFD\x81a'\x96V[\x82RPPV[_` \x82\x01\x90Pa(\x16_\x83\x01\x84a'\xF4V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a(2Wa(1a%1V[[_a(?\x85\x82\x86\x01a'\xB5V[\x92PP` a(P\x85\x82\x86\x01a&\xA3V[\x91PP\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15a(pWa(oa%1V[[_a(}\x85\x82\x86\x01a&\xA3V[\x92PP` a(\x8E\x85\x82\x86\x01a&\xA3V[\x91PP\x92P\x92\x90PV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a(\xB9Wa(\xB8a(\x98V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\xD6Wa(\xD5a(\x9CV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a(\xF2Wa(\xF1a(\xA0V[[\x92P\x92\x90PV[__` \x83\x85\x03\x12\x15a)\x0FWa)\x0Ea%1V[[_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a),Wa)+a%5V[[a)8\x85\x82\x86\x01a(\xA4V[\x92P\x92PP\x92P\x92\x90PV[a)M\x81a&|V[\x82RPPV[_` \x82\x01\x90Pa)f_\x83\x01\x84a)DV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a)\x82Wa)\x81a%1V[[_a)\x8F\x85\x82\x86\x01a'\xB5V[\x92PP` a)\xA0\x85\x82\x86\x01a&\x0BV[\x91PP\x92P\x92\x90PV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a)\xF0\x82a)\xAAV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a*\x0FWa*\x0Ea)\xBAV[[\x80`@RPPPV[_a*!a%(V[\x90Pa*-\x82\x82a)\xE7V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a*LWa*Ka)\xBAV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a*oa*j\x84a*2V[a*\x18V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a*\x92Wa*\x91a(\xA0V[[\x83[\x81\x81\x10\x15a*\xBBW\x80a*\xA7\x88\x82a&\xA3V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa*\x94V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a*\xD9Wa*\xD8a(\x98V[[\x815a*\xE9\x84\x82` \x86\x01a*]V[\x91PP\x92\x91PPV[_____`\xA0\x86\x88\x03\x12\x15a+\x0BWa+\na%1V[[_a+\x18\x88\x82\x89\x01a'\xB5V[\x95PP` a+)\x88\x82\x89\x01a&\x0BV[\x94PP`@a+:\x88\x82\x89\x01a&\x0BV[\x93PP``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+[Wa+Za%5V[[a+g\x88\x82\x89\x01a*\xC5V[\x92PP`\x80a+x\x88\x82\x89\x01a&\x0BV[\x91PP\x92\x95P\x92\x95\x90\x93PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a+\xB7\x81a&|V[\x82RPPV[_a+\xC8\x83\x83a+\xAEV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a+\xEA\x82a+\x85V[a+\xF4\x81\x85a+\x8FV[\x93Pa+\xFF\x83a+\x9FV[\x80_[\x83\x81\x10\x15a,/W\x81Qa,\x16\x88\x82a+\xBDV[\x97Pa,!\x83a+\xD4V[\x92PP`\x01\x81\x01\x90Pa,\x02V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra,T\x81\x84a+\xE0V[\x90P\x92\x91PPV[__`@\x83\x85\x03\x12\x15a,rWa,qa%1V[[_a,\x7F\x85\x82\x86\x01a&\x0BV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\xA0Wa,\x9Fa%5V[[a,\xAC\x85\x82\x86\x01a*\xC5V[\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a,\xCBWa,\xCAa%1V[[_a,\xD8\x84\x82\x85\x01a&\x0BV[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a-\x13\x81a%\xECV[\x82RPPV[_a-$\x83\x83a-\nV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a-F\x82a,\xE1V[a-P\x81\x85a,\xEBV[\x93Pa-[\x83a,\xFBV[\x80_[\x83\x81\x10\x15a-\x8BW\x81Qa-r\x88\x82a-\x19V[\x97Pa-}\x83a-0V[\x92PP`\x01\x81\x01\x90Pa-^V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra-\xB0\x81\x84a-<V[\x90P\x92\x91PPV[____``\x85\x87\x03\x12\x15a-\xD0Wa-\xCFa%1V[[_a-\xDD\x87\x82\x88\x01a&\x0BV[\x94PP` a-\xEE\x87\x82\x88\x01a&\xA3V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.\x0FWa.\x0Ea%5V[[a.\x1B\x87\x82\x88\x01a(\xA4V[\x92P\x92PP\x92\x95\x91\x94P\x92PV[_`@\x82\x01\x90Pa.<_\x83\x01\x85a)DV[a.I` \x83\x01\x84a'nV[\x93\x92PPPV[_``\x82\x01\x90Pa.c_\x83\x01\x86a)DV[a.p` \x83\x01\x85a'nV[a.}`@\x83\x01\x84a'nV[\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a.\xBC\x82a%\xECV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a.\xEEWa.\xEDa.\x85V[[`\x01\x82\x01\x90P\x91\x90PV[_a/\x03\x82a%\xECV[\x91Pa/\x0E\x83a%\xECV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a/&Wa/%a.\x85V[[\x92\x91PPV[_a/6\x82a%\xECV[\x91Pa/A\x83a%\xECV[\x92P\x82\x82\x02a/O\x81a%\xECV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a/fWa/ea.\x85V[[P\x92\x91PPV[_a/w\x82a%\xECV[\x91Pa/\x82\x83a%\xECV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a/\x9AWa/\x99a.\x85V[[\x92\x91PPV[_`@\x82\x01\x90Pa/\xB3_\x83\x01\x85a'nV[a/\xC0` \x83\x01\x84a'nV[\x93\x92PPPV[_\x82\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a0\x15W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a0(Wa0'a/\xD1V[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a0\x8A\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a0OV[a0\x94\x86\x83a0OV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a0\xCFa0\xCAa0\xC5\x84a%\xECV[a0\xACV[a%\xECV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a0\xE8\x83a0\xB5V[a0\xFCa0\xF4\x82a0\xD6V[\x84\x84Ta0[V[\x82UPPPPV[__\x90P\x90V[a1\x13a1\x04V[a1\x1E\x81\x84\x84a0\xDFV[PPPV[_[\x82\x81\x10\x15a1DWa19_\x82\x84\x01a1\x0BV[`\x01\x81\x01\x90Pa1%V[PPPV[`\x1F\x82\x11\x15a1\x97W\x82\x82\x11\x15a1\x96Wa1c\x81a0.V[a1l\x83a0@V[a1u\x85a0@V[` \x86\x10\x15a1\x82W_\x90P[\x80\x83\x01a1\x91\x82\x84\x03\x82a1#V[PPPP[[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_a1\xB7_\x19\x84`\x08\x02a1\x9CV[\x19\x80\x83\x16\x91PP\x92\x91PPV[_a1\xCF\x83\x83a1\xA8V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a1\xE9\x83\x83a/\xC7V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2\x02Wa2\x01a)\xBAV[[a2\x0C\x82Ta/\xFEV[a2\x17\x82\x82\x85a1IV[_`\x1F\x83\x11`\x01\x81\x14a2DW_\x84\x15a22W\x82\x87\x015\x90P[a2<\x85\x82a1\xC4V[\x86UPa2\xA3V[`\x1F\x19\x84\x16a2R\x86a0.V[_[\x82\x81\x10\x15a2yW\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa2TV[\x86\x83\x10\x15a2\x96W\x84\x89\x015a2\x92`\x1F\x89\x16\x82a1\xA8V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_`@\x82\x01\x90Pa2\xEC_\x83\x01\x85a'FV[a2\xF9` \x83\x01\x84a'FV[\x93\x92PPPV[_``\x82\x01\x90Pa3\x13_\x83\x01\x86a'nV[a3 ` \x83\x01\x85a'nV[a3-`@\x83\x01\x84a)DV[\x94\x93PPPPV[_`@\x82\x01\x90Pa3H_\x83\x01\x85a'nV[a3U` \x83\x01\x84a)DV[\x93\x92PPPV[_`\x80\x82\x01\x90Pa3o_\x83\x01\x87a)DV[a3|` \x83\x01\x86a'nV[a3\x89`@\x83\x01\x85a'nV[a3\x96``\x83\x01\x84a)DV[\x95\x94PPPPPV[_`@\x82\x01\x90Pa3\xB2_\x83\x01\x85a)DV[a3\xBF` \x83\x01\x84a'\xF4V[\x93\x92PPPV[_`\xFF\x82\x16\x90P\x91\x90PV[a3\xDB\x81a3\xC6V[\x82RPPV[_`\x80\x82\x01\x90Pa3\xF4_\x83\x01\x87a'\xF4V[a4\x01` \x83\x01\x86a3\xD2V[a4\x0E`@\x83\x01\x85a'\xF4V[a4\x1B``\x83\x01\x84a'\xF4V[\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \x17?\x86b\\\x8F\x14\xB1\xD4%&u\xF9\xC0\xE1`O\x04\xC9\xA2_t\x19N\xB1X|\x97o\xC8\xD5\x07dsolcC\0\x08!\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b506004361061020f575f3560e01c80638da5cb5b11610123578063ca15c873116100ab578063e23fb1f81161007a578063e23fb1f814610579578063ebae35e714610595578063f2fde38b146105c5578063f9d8ff15146105e1578063fc78b2e8146106115761020f565b8063ca15c87314610505578063cb9c4cc414610535578063d547741f1461053f578063d8270dce1461055b5761020f565b8063a217fddf116100f2578063a217fddf14610487578063a3246ad3146104a5578063af206f28146104d5578063bb51fef0146104f1578063c079f495146104fb5761020f565b80638da5cb5b146103ed5780639010d07c1461040b57806391d148541461043b578063930ceae61461046b5761020f565b806330104c3e116101a65780634bb278f3116101755780634bb278f314610395578063595f806e1461039f5780636b744728146103a9578063715018a6146103c55780637f35b560146103cf5761020f565b806330104c3e1461033557806336568abe146103535780634925e62a1461036f5780634b8e64881461038b5761020f565b806317634514116101e257806317634514146102ad5780632328bd12146102cb578063248a9ca3146102e95780632f2ff15d146103195761020f565b806301ffc9a7146102135780630bda81cf1461024357806313ff6dd51461025f578063146ca5311461028f575b5f5ffd5b61022d6004803603810190610228919061258e565b610641565b60405161023a91906125d3565b60405180910390f35b61025d6004803603810190610258919061261f565b6106ba565b005b610279600480360381019061027491906126b7565b610819565b60405161028691906125d3565b60405180910390f35b610297610895565b6040516102a49190612755565b60405180910390f35b6102b56108a7565b6040516102c2919061277d565b60405180910390f35b6102d36108ad565b6040516102e0919061277d565b60405180910390f35b61030360048036038101906102fe91906127c9565b6108c3565b6040516103109190612803565b60405180910390f35b610333600480360381019061032e919061281c565b6108df565b005b61033d610919565b60405161034a9190612803565b60405180910390f35b61036d6004803603810190610368919061281c565b61093d565b005b6103896004803603810190610384919061285a565b610a04565b005b610393610ac6565b005b61039d610b40565b005b6103a7610bbd565b005b6103c360048036038101906103be91906128f9565b610c37565b005b6103cd610c7a565b005b6103d7610c8d565b6040516103e49190612803565b60405180910390f35b6103f5610cb1565b6040516104029190612953565b60405180910390f35b6104256004803603810190610420919061296c565b610cd9565b6040516104329190612953565b60405180910390f35b6104556004803603810190610450919061281c565b610d05565b60405161046291906125d3565b60405180910390f35b61048560048036038101906104809190612af2565b610d68565b005b61048f610dd1565b60405161049c9190612803565b60405180910390f35b6104bf60048036038101906104ba91906127c9565b610dd7565b6040516104cc9190612c3c565b60405180910390f35b6104ef60048036038101906104ea9190612c5c565b610df9565b005b6104f9610e32565b005b610503610eac565b005b61051f600480360381019061051a91906127c9565b610f26565b60405161052c919061277d565b60405180910390f35b61053d610f47565b005b6105596004803603810190610554919061281c565b610fc0565b005b6105636110b3565b604051610570919061277d565b60405180910390f35b610593600480360381019061058e9190612cb6565b6110b9565b005b6105af60048036038101906105aa9190612cb6565b6110f0565b6040516105bc9190612d98565b60405180910390f35b6105df60048036038101906105da91906126b7565b6112b6565b005b6105fb60048036038101906105f69190612db8565b61133a565b60405161060891906125d3565b60405180910390f35b61062b600480360381019061062691906126b7565b611482565b60405161063891906125d3565b60405180910390f35b5f7f07effe0d000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191614806106b357506106b2826114b4565b5b9050919050565b3373ffffffffffffffffffffffffffffffffffffffff1660035f8381526020019081526020015f205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff161461075c5733816040517fffabbae7000000000000000000000000000000000000000000000000000000008152600401610753929190612e29565b60405180910390fd5b60405180604001604052808281526020018381525060075f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f820151815f0155602082015181600101559050507fb89addd937f44f902c849596641837cd7af2fcecef22d2a7866fdc1ad9c0ae2e3383836040516107f693929190612e50565b60405180910390a160065f81548092919061081090612eb2565b91905055505050565b5f61082382611482565b61086457816040517fabdce06a00000000000000000000000000000000000000000000000000000000815260040161085b9190612953565b60405180910390fd5b61088e7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e83610d05565b9050919050565b600c5f9054906101000a900460ff1681565b600b5481565b5f6005546004546108be9190612ef9565b905090565b5f5f5f8381526020019081526020015f20600101549050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6109098161152d565b6109138383611541565b50505050565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46981565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698214801561097257506109718282610d05565b5b156109f6575f61098183610f26565b90505f600160025460036109959190612f2c565b61099f9190612f6d565b90508082036109f3576001816109b59190612ef9565b816040517f3a2362680000000000000000000000000000000000000000000000000000000081526004016109ea929190612fa0565b60405180910390fd5b50505b610a008282611554565b5050565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469610a2e8161152d565b6001600d6001015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550505050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610af08161152d565b6004610afb816115cf565b7f301f8a3701f5b260197382dd7301078542144fe8fddd18083d6f6e09e4958a593342604051610b2c929190612e29565b60405180910390a1610b3c611658565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610b6a8161152d565b6005610b75816115cf565b5f7f24a873651d26fb5a462cb37a91071cdd4d09abeebfe0ed144329bed1cc359d033342604051610ba7929190612e29565b60405180910390a1610bb8816116c1565b505050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610be78161152d565b6003610bf2816115cf565b7f20f55ed0c92f2bb1c8825488e1e3c98463d024b2a42dbd24838c3f75260f43e93342604051610c23929190612e29565b60405180910390a1610c33611658565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610c618161152d565b8282600d5f019182610c749291906131df565b50505050565b610c826116ed565b610c8b5f611774565b565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e81565b5f60085f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b5f610cfd8260015f8681526020019081526020015f2061183790919063ffffffff16565b905092915050565b5f5f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610d928161152d565b610d9c848461184e565b610da582611b14565b610dc986845f81518110610dbc57610dbb6132ac565b5b6020026020010151611b67565b505050505050565b5f5f1b81565b6060610df260015f8481526020019081526020015f20611bea565b9050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610e238161152d565b610e2d838361184e565b505050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610e5c8161152d565b6002610e67816115cf565b7f60edf9bdc7c4ea007cae1a9bbd03e41e5bfccd7231a6ec383c2edd7800f0d20c3342604051610e98929190612e29565b60405180910390a1610ea8611658565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610ed68161152d565b6001610ee1816115cf565b7f67c4489f674d03c7d19a9e36735188de7c65e8d1e99eb3a2fd258a769eb14fff3342604051610f12929190612e29565b60405180910390a1610f22611658565b5050565b5f610f4060015f8481526020019081526020015f20611c09565b9050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610f718161152d565b5f610f7b816115cf565b7fbb709dea744f06d1b26e824deec2f7140c511266ee15d7a217838b31d8b0123d3342604051610fac929190612e29565b60405180910390a1610fbc611658565b5050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610fea8161152d565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698314801561101f575061101e8383610d05565b5b156110a3575f61102e84610f26565b90505f600160025460036110429190612f2c565b61104c9190612f6d565b90508082036110a0576001816110629190612ef9565b816040517f3a236268000000000000000000000000000000000000000000000000000000008152600401611097929190612fa0565b60405180910390fd5b50505b6110ad8383611c1c565b50505050565b600a5481565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e6110e38161152d565b6110ec82611b14565b5050565b60605f6005546004546111039190612ef9565b90508083111561114c5782816040517fdf3d75e2000000000000000000000000000000000000000000000000000000008152600401611143929190612fa0565b60405180910390fd5b5f60018460055461115d9190612f6d565b6111679190612ef9565b90505f8467ffffffffffffffff811115611184576111836129ba565b5b6040519080825280602002602001820160405280156111b25781602001602082028036833780820191505090505b5090505f60055490505b826005541161129c573360035f60055481526020019081526020015f205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555060055482826005546112289190612ef9565b81518110611239576112386132ac565b5b6020026020010181815250507fabde16b7a9192c31c6231b1539bad6fed77635de4c008718dbdcafb7b8363afe33600554604051611278929190612e29565b60405180910390a160055f81548092919061129290612eb2565b91905055506111bc565b85846112a89190612ef9565b935081945050505050919050565b6112be6116ed565b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff160361132e575f6040517f1e4fbdf70000000000000000000000000000000000000000000000000000000081526004016113259190612953565b60405180910390fd5b61133781611774565b50565b5f7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4696113658161152d565b5f6113958760405160200161137a919061277d565b60405160208183030381529060405280519060200120611c2f565b90505f6113e58287878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050611c62565b90508673ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603611473578673ffffffffffffffffffffffffffffffffffffffff167f23d369583ce23023a1ca4baf14a48693a3ee9b3abf2483e41d639e57cb1f9dab89604051611460919061277d565b60405180910390a2600193505050611479565b5f935050505b50949350505050565b5f6114ad7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46983610d05565b9050919050565b5f7f5a05180f000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19161480611526575061152582611c8c565b5b9050919050565b61153e81611539611d05565b611d0c565b50565b5f61154c8383611d5d565b905092915050565b61155c611d05565b73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16146115c0576040517f6697b23200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6115ca8282611c1c565b505050565b8060058111156115e2576115e16126e2565b5b600c5f9054906101000a900460ff166005811115611603576116026126e2565b5b146116555780600c5f9054906101000a900460ff166040517fbfa217d800000000000000000000000000000000000000000000000000000000815260040161164c9291906132d9565b60405180910390fd5b50565b6001600c5f9054906101000a900460ff16600581111561167b5761167a6126e2565b5b6116859190612f6d565b6005811115611697576116966126e2565b5b600c5f6101000a81548160ff021916908360058111156116ba576116b96126e2565b5b0217905550565b80600c5f6101000a81548160ff021916908360058111156116e5576116e46126e2565b5b021790555050565b6116f5611d05565b73ffffffffffffffffffffffffffffffffffffffff16611713610cb1565b73ffffffffffffffffffffffffffffffffffffffff161461177257611736611d05565b6040517f118cdaa70000000000000000000000000000000000000000000000000000000081526004016117699190612953565b60405180910390fd5b565b5f60085f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690508160085f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a35050565b5f611844835f0183611da0565b5f1c905092915050565b5f600183600361185e9190612f2c565b6118689190612f6d565b905080825110156118b3578151816040517f3a2362680000000000000000000000000000000000000000000000000000000081526004016118aa929190612fa0565b60405180910390fd5b826002819055505f6118e47f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610dd7565b90505f6119107f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e610f26565b90505f5f90505b81811015611971576119637f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e848381518110611956576119556132ac565b5b6020026020010151611c1c565b508080600101915050611917565b505f61199c7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469610dd7565b90505f6119c87fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469610f26565b90505f5f90505b81811015611a2957611a1b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469848381518110611a0e57611a0d6132ac565b5b6020026020010151611c1c565b5080806001019150506119cf565b505f5f90505b8651811015611a8a57611a7c7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc469888381518110611a6f57611a6e6132ac565b5b6020026020010151611541565b508080600101915050611a2f565b50611acf7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e875f81518110611ac257611ac16132ac565b5b6020026020010151611541565b507fa2df7830e0bedef7b1116bf547b467b16b50b3bd23146c9e099878d14e89301a858833604051611b0393929190613300565b60405180910390a150505050505050565b806004819055505f6005819055505f6006819055507ff7f0872382dff5e698b284e12084e4e7894f830216dd80cb4e909b593a58f93560045433604051611b5c929190613335565b60405180910390a150565b8160098190555042600a8190555043600b819055505f600c5f6101000a81548160ff02191690836005811115611ba057611b9f6126e2565b5b02179055507fdef1f08eb655f4a75f60bd6fd7e971112004abd846a612e46dab270770d24ca530600a54600b5484604051611bde949392919061335c565b60405180910390a15050565b60605f611bf8835f01611dc7565b905060608190508092505050919050565b5f611c15825f01611e20565b9050919050565b5f611c278383611e2f565b905092915050565b5f7f19457468657265756d205369676e6564204d6573736167653a0a3332000000005f5281601c52603c5f209050919050565b5f5f5f5f611c708686611e72565b925092509250611c808282611ec7565b82935050505092915050565b5f7f7965db0b000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19161480611cfe5750611cfd82612029565b5b9050919050565b5f33905090565b611d168282610d05565b611d595780826040517fe2517d3f000000000000000000000000000000000000000000000000000000008152600401611d5092919061339f565b60405180910390fd5b5050565b5f5f611d698484612092565b90508015611d9657611d948360015f8781526020019081526020015f2061217b90919063ffffffff16565b505b8091505092915050565b5f825f018281548110611db657611db56132ac565b5b905f5260205f200154905092915050565b6060815f01805480602002602001604051908101604052809291908181526020018280548015611e1457602002820191905f5260205f20905b815481526020019060010190808311611e00575b50505050509050919050565b5f815f01805490509050919050565b5f5f611e3b84846121a8565b90508015611e6857611e668360015f8781526020019081526020015f2061229190919063ffffffff16565b505b8091505092915050565b5f5f5f6041845103611eb2575f5f5f602087015192506040870151915060608701515f1a9050611ea4888285856122be565b955095509550505050611ec0565b5f600285515f1b9250925092505b9250925092565b5f6003811115611eda57611ed96126e2565b5b826003811115611eed57611eec6126e2565b5b03156120255760016003811115611f0757611f066126e2565b5b826003811115611f1a57611f196126e2565b5b03611f51576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60026003811115611f6557611f646126e2565b5b826003811115611f7857611f776126e2565b5b03611fbc57805f1c6040517ffce698f7000000000000000000000000000000000000000000000000000000008152600401611fb3919061277d565b60405180910390fd5b600380811115611fcf57611fce6126e2565b5b826003811115611fe257611fe16126e2565b5b0361202457806040517fd78bce0c00000000000000000000000000000000000000000000000000000000815260040161201b9190612803565b60405180910390fd5b5b5050565b5f7f01ffc9a7000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916149050919050565b5f61209d8383610d05565b6121715760015f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555061210e611d05565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a460019050612175565b5f90505b92915050565b5f6121a0835f018373ffffffffffffffffffffffffffffffffffffffff165f1b6123a5565b905092915050565b5f6121b38383610d05565b15612287575f5f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550612224611d05565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b60405160405180910390a46001905061228b565b5f90505b92915050565b5f6122b6835f018373ffffffffffffffffffffffffffffffffffffffff165f1b61240c565b905092915050565b5f5f5f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c11156122fa575f60038592509250925061239b565b5f6001888888886040515f815260200160405260405161231d94939291906133e1565b6020604051602081039080840390855afa15801561233d573d5f5f3e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff160361238e575f60015f5f1b9350935093505061239b565b805f5f5f1b935093509350505b9450945094915050565b5f6123b08383612508565b61240257825f0182908060018154018082558091505060019003905f5260205f20015f9091909190915055825f0180549050836001015f8481526020019081526020015f208190555060019050612406565b5f90505b92915050565b5f5f836001015f8481526020019081526020015f205490505f81146124fd575f6001826124399190612ef9565b90505f6001865f018054905061244f9190612ef9565b90508082146124b5575f865f01828154811061246e5761246d6132ac565b5b905f5260205f200154905080875f01848154811061248f5761248e6132ac565b5b905f5260205f20018190555083876001015f8381526020019081526020015f2081905550505b855f018054806124c8576124c7613424565b5b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f905560019350505050612502565b5f9150505b92915050565b5f5f836001015f8481526020019081526020015f20541415905092915050565b5f604051905090565b5f5ffd5b5f5ffd5b5f7fffffffff0000000000000000000000000000000000000000000000000000000082169050919050565b61256d81612539565b8114612577575f5ffd5b50565b5f8135905061258881612564565b92915050565b5f602082840312156125a3576125a2612531565b5b5f6125b08482850161257a565b91505092915050565b5f8115159050919050565b6125cd816125b9565b82525050565b5f6020820190506125e65f8301846125c4565b92915050565b5f819050919050565b6125fe816125ec565b8114612608575f5ffd5b50565b5f81359050612619816125f5565b92915050565b5f5f6040838503121561263557612634612531565b5b5f6126428582860161260b565b92505060206126538582860161260b565b9150509250929050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6126868261265d565b9050919050565b6126968161267c565b81146126a0575f5ffd5b50565b5f813590506126b18161268d565b92915050565b5f602082840312156126cc576126cb612531565b5b5f6126d9848285016126a3565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b600681106127205761271f6126e2565b5b50565b5f8190506127308261270f565b919050565b5f61273f82612723565b9050919050565b61274f81612735565b82525050565b5f6020820190506127685f830184612746565b92915050565b612777816125ec565b82525050565b5f6020820190506127905f83018461276e565b92915050565b5f819050919050565b6127a881612796565b81146127b2575f5ffd5b50565b5f813590506127c38161279f565b92915050565b5f602082840312156127de576127dd612531565b5b5f6127eb848285016127b5565b91505092915050565b6127fd81612796565b82525050565b5f6020820190506128165f8301846127f4565b92915050565b5f5f6040838503121561283257612831612531565b5b5f61283f858286016127b5565b9250506020612850858286016126a3565b9150509250929050565b5f5f604083850312156128705761286f612531565b5b5f61287d858286016126a3565b925050602061288e858286016126a3565b9150509250929050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f8401126128b9576128b8612898565b5b8235905067ffffffffffffffff8111156128d6576128d561289c565b5b6020830191508360018202830111156128f2576128f16128a0565b5b9250929050565b5f5f6020838503121561290f5761290e612531565b5b5f83013567ffffffffffffffff81111561292c5761292b612535565b5b612938858286016128a4565b92509250509250929050565b61294d8161267c565b82525050565b5f6020820190506129665f830184612944565b92915050565b5f5f6040838503121561298257612981612531565b5b5f61298f858286016127b5565b92505060206129a08582860161260b565b9150509250929050565b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6129f0826129aa565b810181811067ffffffffffffffff82111715612a0f57612a0e6129ba565b5b80604052505050565b5f612a21612528565b9050612a2d82826129e7565b919050565b5f67ffffffffffffffff821115612a4c57612a4b6129ba565b5b602082029050602081019050919050565b5f612a6f612a6a84612a32565b612a18565b90508083825260208201905060208402830185811115612a9257612a916128a0565b5b835b81811015612abb5780612aa788826126a3565b845260208401935050602081019050612a94565b5050509392505050565b5f82601f830112612ad957612ad8612898565b5b8135612ae9848260208601612a5d565b91505092915050565b5f5f5f5f5f60a08688031215612b0b57612b0a612531565b5b5f612b18888289016127b5565b9550506020612b298882890161260b565b9450506040612b3a8882890161260b565b935050606086013567ffffffffffffffff811115612b5b57612b5a612535565b5b612b6788828901612ac5565b9250506080612b788882890161260b565b9150509295509295909350565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b612bb78161267c565b82525050565b5f612bc88383612bae565b60208301905092915050565b5f602082019050919050565b5f612bea82612b85565b612bf48185612b8f565b9350612bff83612b9f565b805f5b83811015612c2f578151612c168882612bbd565b9750612c2183612bd4565b925050600181019050612c02565b5085935050505092915050565b5f6020820190508181035f830152612c548184612be0565b905092915050565b5f5f60408385031215612c7257612c71612531565b5b5f612c7f8582860161260b565b925050602083013567ffffffffffffffff811115612ca057612c9f612535565b5b612cac85828601612ac5565b9150509250929050565b5f60208284031215612ccb57612cca612531565b5b5f612cd88482850161260b565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b612d13816125ec565b82525050565b5f612d248383612d0a565b60208301905092915050565b5f602082019050919050565b5f612d4682612ce1565b612d508185612ceb565b9350612d5b83612cfb565b805f5b83811015612d8b578151612d728882612d19565b9750612d7d83612d30565b925050600181019050612d5e565b5085935050505092915050565b5f6020820190508181035f830152612db08184612d3c565b905092915050565b5f5f5f5f60608587031215612dd057612dcf612531565b5b5f612ddd8782880161260b565b9450506020612dee878288016126a3565b935050604085013567ffffffffffffffff811115612e0f57612e0e612535565b5b612e1b878288016128a4565b925092505092959194509250565b5f604082019050612e3c5f830185612944565b612e49602083018461276e565b9392505050565b5f606082019050612e635f830186612944565b612e70602083018561276e565b612e7d604083018461276e565b949350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f612ebc826125ec565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203612eee57612eed612e85565b5b600182019050919050565b5f612f03826125ec565b9150612f0e836125ec565b9250828203905081811115612f2657612f25612e85565b5b92915050565b5f612f36826125ec565b9150612f41836125ec565b9250828202612f4f816125ec565b91508282048414831517612f6657612f65612e85565b5b5092915050565b5f612f77826125ec565b9150612f82836125ec565b9250828201905080821115612f9a57612f99612e85565b5b92915050565b5f604082019050612fb35f83018561276e565b612fc0602083018461276e565b9392505050565b5f82905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061301557607f821691505b60208210810361302857613027612fd1565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f6008830261308a7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8261304f565b613094868361304f565b95508019841693508086168417925050509392505050565b5f819050919050565b5f6130cf6130ca6130c5846125ec565b6130ac565b6125ec565b9050919050565b5f819050919050565b6130e8836130b5565b6130fc6130f4826130d6565b84845461305b565b825550505050565b5f5f905090565b613113613104565b61311e8184846130df565b505050565b5f5b82811015613144576131395f82840161310b565b600181019050613125565b505050565b601f8211156131975782821115613196576131638161302e565b61316c83613040565b61317585613040565b6020861015613182575f90505b80830161319182840382613123565b505050505b5b505050565b5f82821c905092915050565b5f6131b75f198460080261319c565b1980831691505092915050565b5f6131cf83836131a8565b9150826002028217905092915050565b6131e98383612fc7565b67ffffffffffffffff811115613202576132016129ba565b5b61320c8254612ffe565b613217828285613149565b5f601f831160018114613244575f8415613232578287013590505b61323c85826131c4565b8655506132a3565b601f1984166132528661302e565b5f5b8281101561327957848901358255600182019150602085019450602081019050613254565b868310156132965784890135613292601f8916826131a8565b8355505b6001600288020188555050505b50505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f6040820190506132ec5f830185612746565b6132f96020830184612746565b9392505050565b5f6060820190506133135f83018661276e565b613320602083018561276e565b61332d6040830184612944565b949350505050565b5f6040820190506133485f83018561276e565b6133556020830184612944565b9392505050565b5f60808201905061336f5f830187612944565b61337c602083018661276e565b613389604083018561276e565b6133966060830184612944565b95945050505050565b5f6040820190506133b25f830185612944565b6133bf60208301846127f4565b9392505050565b5f60ff82169050919050565b6133db816133c6565b82525050565b5f6080820190506133f45f8301876127f4565b61340160208301866133d2565b61340e60408301856127f4565b61341b60608301846127f4565b95945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffdfea2646970667358221220173f86625c8f14b1d4252675f9c0e1604f04c9a25f74194eb1587c976fc8d50764736f6c63430008210033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02\x0FW_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\x01#W\x80c\xCA\x15\xC8s\x11a\0\xABW\x80c\xE2?\xB1\xF8\x11a\0zW\x80c\xE2?\xB1\xF8\x14a\x05yW\x80c\xEB\xAE5\xE7\x14a\x05\x95W\x80c\xF2\xFD\xE3\x8B\x14a\x05\xC5W\x80c\xF9\xD8\xFF\x15\x14a\x05\xE1W\x80c\xFCx\xB2\xE8\x14a\x06\x11Wa\x02\x0FV[\x80c\xCA\x15\xC8s\x14a\x05\x05W\x80c\xCB\x9CL\xC4\x14a\x055W\x80c\xD5Gt\x1F\x14a\x05?W\x80c\xD8'\r\xCE\x14a\x05[Wa\x02\x0FV[\x80c\xA2\x17\xFD\xDF\x11a\0\xF2W\x80c\xA2\x17\xFD\xDF\x14a\x04\x87W\x80c\xA3$j\xD3\x14a\x04\xA5W\x80c\xAF o(\x14a\x04\xD5W\x80c\xBBQ\xFE\xF0\x14a\x04\xF1W\x80c\xC0y\xF4\x95\x14a\x04\xFBWa\x02\x0FV[\x80c\x8D\xA5\xCB[\x14a\x03\xEDW\x80c\x90\x10\xD0|\x14a\x04\x0BW\x80c\x91\xD1HT\x14a\x04;W\x80c\x93\x0C\xEA\xE6\x14a\x04kWa\x02\x0FV[\x80c0\x10L>\x11a\x01\xA6W\x80cK\xB2x\xF3\x11a\x01uW\x80cK\xB2x\xF3\x14a\x03\x95W\x80cY_\x80n\x14a\x03\x9FW\x80cktG(\x14a\x03\xA9W\x80cqP\x18\xA6\x14a\x03\xC5W\x80c\x7F5\xB5`\x14a\x03\xCFWa\x02\x0FV[\x80c0\x10L>\x14a\x035W\x80c6V\x8A\xBE\x14a\x03SW\x80cI%\xE6*\x14a\x03oW\x80cK\x8Ed\x88\x14a\x03\x8BWa\x02\x0FV[\x80c\x17cE\x14\x11a\x01\xE2W\x80c\x17cE\x14\x14a\x02\xADW\x80c#(\xBD\x12\x14a\x02\xCBW\x80c$\x8A\x9C\xA3\x14a\x02\xE9W\x80c//\xF1]\x14a\x03\x19Wa\x02\x0FV[\x80c\x01\xFF\xC9\xA7\x14a\x02\x13W\x80c\x0B\xDA\x81\xCF\x14a\x02CW\x80c\x13\xFFm\xD5\x14a\x02_W\x80c\x14l\xA51\x14a\x02\x8FW[__\xFD[a\x02-`\x04\x806\x03\x81\x01\x90a\x02(\x91\x90a%\x8EV[a\x06AV[`@Qa\x02:\x91\x90a%\xD3V[`@Q\x80\x91\x03\x90\xF3[a\x02]`\x04\x806\x03\x81\x01\x90a\x02X\x91\x90a&\x1FV[a\x06\xBAV[\0[a\x02y`\x04\x806\x03\x81\x01\x90a\x02t\x91\x90a&\xB7V[a\x08\x19V[`@Qa\x02\x86\x91\x90a%\xD3V[`@Q\x80\x91\x03\x90\xF3[a\x02\x97a\x08\x95V[`@Qa\x02\xA4\x91\x90a'UV[`@Q\x80\x91\x03\x90\xF3[a\x02\xB5a\x08\xA7V[`@Qa\x02\xC2\x91\x90a'}V[`@Q\x80\x91\x03\x90\xF3[a\x02\xD3a\x08\xADV[`@Qa\x02\xE0\x91\x90a'}V[`@Q\x80\x91\x03\x90\xF3[a\x03\x03`\x04\x806\x03\x81\x01\x90a\x02\xFE\x91\x90a'\xC9V[a\x08\xC3V[`@Qa\x03\x10\x91\x90a(\x03V[`@Q\x80\x91\x03\x90\xF3[a\x033`\x04\x806\x03\x81\x01\x90a\x03.\x91\x90a(\x1CV[a\x08\xDFV[\0[a\x03=a\t\x19V[`@Qa\x03J\x91\x90a(\x03V[`@Q\x80\x91\x03\x90\xF3[a\x03m`\x04\x806\x03\x81\x01\x90a\x03h\x91\x90a(\x1CV[a\t=V[\0[a\x03\x89`\x04\x806\x03\x81\x01\x90a\x03\x84\x91\x90a(ZV[a\n\x04V[\0[a\x03\x93a\n\xC6V[\0[a\x03\x9Da\x0B@V[\0[a\x03\xA7a\x0B\xBDV[\0[a\x03\xC3`\x04\x806\x03\x81\x01\x90a\x03\xBE\x91\x90a(\xF9V[a\x0C7V[\0[a\x03\xCDa\x0CzV[\0[a\x03\xD7a\x0C\x8DV[`@Qa\x03\xE4\x91\x90a(\x03V[`@Q\x80\x91\x03\x90\xF3[a\x03\xF5a\x0C\xB1V[`@Qa\x04\x02\x91\x90a)SV[`@Q\x80\x91\x03\x90\xF3[a\x04%`\x04\x806\x03\x81\x01\x90a\x04 \x91\x90a)lV[a\x0C\xD9V[`@Qa\x042\x91\x90a)SV[`@Q\x80\x91\x03\x90\xF3[a\x04U`\x04\x806\x03\x81\x01\x90a\x04P\x91\x90a(\x1CV[a\r\x05V[`@Qa\x04b\x91\x90a%\xD3V[`@Q\x80\x91\x03\x90\xF3[a\x04\x85`\x04\x806\x03\x81\x01\x90a\x04\x80\x91\x90a*\xF2V[a\rhV[\0[a\x04\x8Fa\r\xD1V[`@Qa\x04\x9C\x91\x90a(\x03V[`@Q\x80\x91\x03\x90\xF3[a\x04\xBF`\x04\x806\x03\x81\x01\x90a\x04\xBA\x91\x90a'\xC9V[a\r\xD7V[`@Qa\x04\xCC\x91\x90a,<V[`@Q\x80\x91\x03\x90\xF3[a\x04\xEF`\x04\x806\x03\x81\x01\x90a\x04\xEA\x91\x90a,\\V[a\r\xF9V[\0[a\x04\xF9a\x0E2V[\0[a\x05\x03a\x0E\xACV[\0[a\x05\x1F`\x04\x806\x03\x81\x01\x90a\x05\x1A\x91\x90a'\xC9V[a\x0F&V[`@Qa\x05,\x91\x90a'}V[`@Q\x80\x91\x03\x90\xF3[a\x05=a\x0FGV[\0[a\x05Y`\x04\x806\x03\x81\x01\x90a\x05T\x91\x90a(\x1CV[a\x0F\xC0V[\0[a\x05ca\x10\xB3V[`@Qa\x05p\x91\x90a'}V[`@Q\x80\x91\x03\x90\xF3[a\x05\x93`\x04\x806\x03\x81\x01\x90a\x05\x8E\x91\x90a,\xB6V[a\x10\xB9V[\0[a\x05\xAF`\x04\x806\x03\x81\x01\x90a\x05\xAA\x91\x90a,\xB6V[a\x10\xF0V[`@Qa\x05\xBC\x91\x90a-\x98V[`@Q\x80\x91\x03\x90\xF3[a\x05\xDF`\x04\x806\x03\x81\x01\x90a\x05\xDA\x91\x90a&\xB7V[a\x12\xB6V[\0[a\x05\xFB`\x04\x806\x03\x81\x01\x90a\x05\xF6\x91\x90a-\xB8V[a\x13:V[`@Qa\x06\x08\x91\x90a%\xD3V[`@Q\x80\x91\x03\x90\xF3[a\x06+`\x04\x806\x03\x81\x01\x90a\x06&\x91\x90a&\xB7V[a\x14\x82V[`@Qa\x068\x91\x90a%\xD3V[`@Q\x80\x91\x03\x90\xF3[_\x7F\x07\xEF\xFE\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x06\xB3WPa\x06\xB2\x82a\x14\xB4V[[\x90P\x91\x90PV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07\\W3\x81`@Q\x7F\xFF\xAB\xBA\xE7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07S\x92\x91\x90a.)V[`@Q\x80\x91\x03\x90\xFD[`@Q\x80`@\x01`@R\x80\x82\x81R` \x01\x83\x81RP`\x07_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01U` \x82\x01Q\x81`\x01\x01U\x90PP\x7F\xB8\x9A\xDD\xD97\xF4O\x90,\x84\x95\x96d\x187\xCDz\xF2\xFC\xEC\xEF\"\xD2\xA7\x86o\xDC\x1A\xD9\xC0\xAE.3\x83\x83`@Qa\x07\xF6\x93\x92\x91\x90a.PV[`@Q\x80\x91\x03\x90\xA1`\x06_\x81T\x80\x92\x91\x90a\x08\x10\x90a.\xB2V[\x91\x90PUPPPV[_a\x08#\x82a\x14\x82V[a\x08dW\x81`@Q\x7F\xAB\xDC\xE0j\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08[\x91\x90a)SV[`@Q\x80\x91\x03\x90\xFD[a\x08\x8E\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x83a\r\x05V[\x90P\x91\x90PV[`\x0C_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\x0BT\x81V[_`\x05T`\x04Ta\x08\xBE\x91\x90a.\xF9V[\x90P\x90V[___\x83\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\t\t\x81a\x15-V[a\t\x13\x83\x83a\x15AV[PPPPV[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x81V[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x82\x14\x80\x15a\trWPa\tq\x82\x82a\r\x05V[[\x15a\t\xF6W_a\t\x81\x83a\x0F&V[\x90P_`\x01`\x02T`\x03a\t\x95\x91\x90a/,V[a\t\x9F\x91\x90a/mV[\x90P\x80\x82\x03a\t\xF3W`\x01\x81a\t\xB5\x91\x90a.\xF9V[\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xEA\x92\x91\x90a/\xA0V[`@Q\x80\x91\x03\x90\xFD[PP[a\n\0\x82\x82a\x15TV[PPV[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\n.\x81a\x15-V[`\x01`\r`\x01\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPPPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\n\xF0\x81a\x15-V[`\x04a\n\xFB\x81a\x15\xCFV[\x7F0\x1F\x8A7\x01\xF5\xB2`\x19s\x82\xDDs\x01\x07\x85B\x14O\xE8\xFD\xDD\x18\x08=on\t\xE4\x95\x8AY3B`@Qa\x0B,\x92\x91\x90a.)V[`@Q\x80\x91\x03\x90\xA1a\x0B<a\x16XV[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0Bj\x81a\x15-V[`\x05a\x0Bu\x81a\x15\xCFV[_\x7F$\xA8se\x1D&\xFBZF,\xB3z\x91\x07\x1C\xDDM\t\xAB\xEE\xBF\xE0\xED\x14C)\xBE\xD1\xCC5\x9D\x033B`@Qa\x0B\xA7\x92\x91\x90a.)V[`@Q\x80\x91\x03\x90\xA1a\x0B\xB8\x81a\x16\xC1V[PPPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0B\xE7\x81a\x15-V[`\x03a\x0B\xF2\x81a\x15\xCFV[\x7F \xF5^\xD0\xC9/+\xB1\xC8\x82T\x88\xE1\xE3\xC9\x84c\xD0$\xB2\xA4-\xBD$\x83\x8C?u&\x0FC\xE93B`@Qa\x0C#\x92\x91\x90a.)V[`@Q\x80\x91\x03\x90\xA1a\x0C3a\x16XV[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0Ca\x81a\x15-V[\x82\x82`\r_\x01\x91\x82a\x0Ct\x92\x91\x90a1\xDFV[PPPPV[a\x0C\x82a\x16\xEDV[a\x0C\x8B_a\x17tV[V[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x81V[_`\x08_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_a\x0C\xFD\x82`\x01_\x86\x81R` \x01\x90\x81R` \x01_ a\x187\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[___\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\r\x92\x81a\x15-V[a\r\x9C\x84\x84a\x18NV[a\r\xA5\x82a\x1B\x14V[a\r\xC9\x86\x84_\x81Q\x81\x10a\r\xBCWa\r\xBBa2\xACV[[` \x02` \x01\x01Qa\x1BgV[PPPPPPV[__\x1B\x81V[``a\r\xF2`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a\x1B\xEAV[\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0E#\x81a\x15-V[a\x0E-\x83\x83a\x18NV[PPPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0E\\\x81a\x15-V[`\x02a\x0Eg\x81a\x15\xCFV[\x7F`\xED\xF9\xBD\xC7\xC4\xEA\0|\xAE\x1A\x9B\xBD\x03\xE4\x1E[\xFC\xCDr1\xA6\xEC8<.\xDDx\0\xF0\xD2\x0C3B`@Qa\x0E\x98\x92\x91\x90a.)V[`@Q\x80\x91\x03\x90\xA1a\x0E\xA8a\x16XV[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0E\xD6\x81a\x15-V[`\x01a\x0E\xE1\x81a\x15\xCFV[\x7Fg\xC4H\x9FgM\x03\xC7\xD1\x9A\x9E6sQ\x88\xDE|e\xE8\xD1\xE9\x9E\xB3\xA2\xFD%\x8Av\x9E\xB1O\xFF3B`@Qa\x0F\x12\x92\x91\x90a.)V[`@Q\x80\x91\x03\x90\xA1a\x0F\"a\x16XV[PPV[_a\x0F@`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a\x1C\tV[\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0Fq\x81a\x15-V[_a\x0F{\x81a\x15\xCFV[\x7F\xBBp\x9D\xEAtO\x06\xD1\xB2n\x82M\xEE\xC2\xF7\x14\x0CQ\x12f\xEE\x15\xD7\xA2\x17\x83\x8B1\xD8\xB0\x12=3B`@Qa\x0F\xAC\x92\x91\x90a.)V[`@Q\x80\x91\x03\x90\xA1a\x0F\xBCa\x16XV[PPV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0F\xEA\x81a\x15-V[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x83\x14\x80\x15a\x10\x1FWPa\x10\x1E\x83\x83a\r\x05V[[\x15a\x10\xA3W_a\x10.\x84a\x0F&V[\x90P_`\x01`\x02T`\x03a\x10B\x91\x90a/,V[a\x10L\x91\x90a/mV[\x90P\x80\x82\x03a\x10\xA0W`\x01\x81a\x10b\x91\x90a.\xF9V[\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\x97\x92\x91\x90a/\xA0V[`@Q\x80\x91\x03\x90\xFD[PP[a\x10\xAD\x83\x83a\x1C\x1CV[PPPPV[`\nT\x81V[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x10\xE3\x81a\x15-V[a\x10\xEC\x82a\x1B\x14V[PPV[``_`\x05T`\x04Ta\x11\x03\x91\x90a.\xF9V[\x90P\x80\x83\x11\x15a\x11LW\x82\x81`@Q\x7F\xDF=u\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11C\x92\x91\x90a/\xA0V[`@Q\x80\x91\x03\x90\xFD[_`\x01\x84`\x05Ta\x11]\x91\x90a/mV[a\x11g\x91\x90a.\xF9V[\x90P_\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\x84Wa\x11\x83a)\xBAV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11\xB2W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_`\x05T\x90P[\x82`\x05T\x11a\x12\x9CW3`\x03_`\x05T\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x05T\x82\x82`\x05Ta\x12(\x91\x90a.\xF9V[\x81Q\x81\x10a\x129Wa\x128a2\xACV[[` \x02` \x01\x01\x81\x81RPP\x7F\xAB\xDE\x16\xB7\xA9\x19,1\xC6#\x1B\x159\xBA\xD6\xFE\xD7v5\xDEL\0\x87\x18\xDB\xDC\xAF\xB7\xB86:\xFE3`\x05T`@Qa\x12x\x92\x91\x90a.)V[`@Q\x80\x91\x03\x90\xA1`\x05_\x81T\x80\x92\x91\x90a\x12\x92\x90a.\xB2V[\x91\x90PUPa\x11\xBCV[\x85\x84a\x12\xA8\x91\x90a.\xF9V[\x93P\x81\x94PPPPP\x91\x90PV[a\x12\xBEa\x16\xEDV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x13.W_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13%\x91\x90a)SV[`@Q\x80\x91\x03\x90\xFD[a\x137\x81a\x17tV[PV[_\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x13e\x81a\x15-V[_a\x13\x95\x87`@Q` \x01a\x13z\x91\x90a'}V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x1C/V[\x90P_a\x13\xE5\x82\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa\x1CbV[\x90P\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x14sW\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F#\xD3iX<\xE20#\xA1\xCAK\xAF\x14\xA4\x86\x93\xA3\xEE\x9B:\xBF$\x83\xE4\x1Dc\x9EW\xCB\x1F\x9D\xAB\x89`@Qa\x14`\x91\x90a'}V[`@Q\x80\x91\x03\x90\xA2`\x01\x93PPPa\x14yV[_\x93PPP[P\x94\x93PPPPV[_a\x14\xAD\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x83a\r\x05V[\x90P\x91\x90PV[_\x7FZ\x05\x18\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x15&WPa\x15%\x82a\x1C\x8CV[[\x90P\x91\x90PV[a\x15>\x81a\x159a\x1D\x05V[a\x1D\x0CV[PV[_a\x15L\x83\x83a\x1D]V[\x90P\x92\x91PPV[a\x15\\a\x1D\x05V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x15\xC0W`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x15\xCA\x82\x82a\x1C\x1CV[PPPV[\x80`\x05\x81\x11\x15a\x15\xE2Wa\x15\xE1a&\xE2V[[`\x0C_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x05\x81\x11\x15a\x16\x03Wa\x16\x02a&\xE2V[[\x14a\x16UW\x80`\x0C_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`@Q\x7F\xBF\xA2\x17\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16L\x92\x91\x90a2\xD9V[`@Q\x80\x91\x03\x90\xFD[PV[`\x01`\x0C_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x05\x81\x11\x15a\x16{Wa\x16za&\xE2V[[a\x16\x85\x91\x90a/mV[`\x05\x81\x11\x15a\x16\x97Wa\x16\x96a&\xE2V[[`\x0C_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x16\xBAWa\x16\xB9a&\xE2V[[\x02\x17\x90UPV[\x80`\x0C_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x16\xE5Wa\x16\xE4a&\xE2V[[\x02\x17\x90UPPV[a\x16\xF5a\x1D\x05V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x17\x13a\x0C\xB1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x17rWa\x176a\x1D\x05V[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17i\x91\x90a)SV[`@Q\x80\x91\x03\x90\xFD[V[_`\x08_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\x08_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[_a\x18D\x83_\x01\x83a\x1D\xA0V[_\x1C\x90P\x92\x91PPV[_`\x01\x83`\x03a\x18^\x91\x90a/,V[a\x18h\x91\x90a/mV[\x90P\x80\x82Q\x10\x15a\x18\xB3W\x81Q\x81`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\xAA\x92\x91\x90a/\xA0V[`@Q\x80\x91\x03\x90\xFD[\x82`\x02\x81\x90UP_a\x18\xE4\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\r\xD7V[\x90P_a\x19\x10\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x0F&V[\x90P__\x90P[\x81\x81\x10\x15a\x19qWa\x19c\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x84\x83\x81Q\x81\x10a\x19VWa\x19Ua2\xACV[[` \x02` \x01\x01Qa\x1C\x1CV[P\x80\x80`\x01\x01\x91PPa\x19\x17V[P_a\x19\x9C\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\r\xD7V[\x90P_a\x19\xC8\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4ia\x0F&V[\x90P__\x90P[\x81\x81\x10\x15a\x1A)Wa\x1A\x1B\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x84\x83\x81Q\x81\x10a\x1A\x0EWa\x1A\ra2\xACV[[` \x02` \x01\x01Qa\x1C\x1CV[P\x80\x80`\x01\x01\x91PPa\x19\xCFV[P__\x90P[\x86Q\x81\x10\x15a\x1A\x8AWa\x1A|\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x88\x83\x81Q\x81\x10a\x1AoWa\x1Ana2\xACV[[` \x02` \x01\x01Qa\x15AV[P\x80\x80`\x01\x01\x91PPa\x1A/V[Pa\x1A\xCF\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x87_\x81Q\x81\x10a\x1A\xC2Wa\x1A\xC1a2\xACV[[` \x02` \x01\x01Qa\x15AV[P\x7F\xA2\xDFx0\xE0\xBE\xDE\xF7\xB1\x11k\xF5G\xB4g\xB1kP\xB3\xBD#\x14l\x9E\t\x98x\xD1N\x890\x1A\x85\x883`@Qa\x1B\x03\x93\x92\x91\x90a3\0V[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[\x80`\x04\x81\x90UP_`\x05\x81\x90UP_`\x06\x81\x90UP\x7F\xF7\xF0\x87#\x82\xDF\xF5\xE6\x98\xB2\x84\xE1 \x84\xE4\xE7\x89O\x83\x02\x16\xDD\x80\xCBN\x90\x9BY:X\xF95`\x04T3`@Qa\x1B\\\x92\x91\x90a35V[`@Q\x80\x91\x03\x90\xA1PV[\x81`\t\x81\x90UPB`\n\x81\x90UPC`\x0B\x81\x90UP_`\x0C_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x05\x81\x11\x15a\x1B\xA0Wa\x1B\x9Fa&\xE2V[[\x02\x17\x90UP\x7F\xDE\xF1\xF0\x8E\xB6U\xF4\xA7_`\xBDo\xD7\xE9q\x11 \x04\xAB\xD8F\xA6\x12\xE4m\xAB'\x07p\xD2L\xA50`\nT`\x0BT\x84`@Qa\x1B\xDE\x94\x93\x92\x91\x90a3\\V[`@Q\x80\x91\x03\x90\xA1PPV[``_a\x1B\xF8\x83_\x01a\x1D\xC7V[\x90P``\x81\x90P\x80\x92PPP\x91\x90PV[_a\x1C\x15\x82_\x01a\x1E V[\x90P\x91\x90PV[_a\x1C'\x83\x83a\x1E/V[\x90P\x92\x91PPV[_\x7F\x19Ethereum Signed Message:\n32\0\0\0\0_R\x81`\x1CR`<_ \x90P\x91\x90PV[____a\x1Cp\x86\x86a\x1ErV[\x92P\x92P\x92Pa\x1C\x80\x82\x82a\x1E\xC7V[\x82\x93PPPP\x92\x91PPV[_\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x1C\xFEWPa\x1C\xFD\x82a )V[[\x90P\x91\x90PV[_3\x90P\x90V[a\x1D\x16\x82\x82a\r\x05V[a\x1DYW\x80\x82`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1DP\x92\x91\x90a3\x9FV[`@Q\x80\x91\x03\x90\xFD[PPV[__a\x1Di\x84\x84a \x92V[\x90P\x80\x15a\x1D\x96Wa\x1D\x94\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a!{\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[_\x82_\x01\x82\x81T\x81\x10a\x1D\xB6Wa\x1D\xB5a2\xACV[[\x90_R` _ \x01T\x90P\x92\x91PPV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1E\x14W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x1E\0W[PPPPP\x90P\x91\x90PV[_\x81_\x01\x80T\x90P\x90P\x91\x90PV[__a\x1E;\x84\x84a!\xA8V[\x90P\x80\x15a\x1EhWa\x1Ef\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a\"\x91\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[___`A\x84Q\x03a\x1E\xB2W___` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90Pa\x1E\xA4\x88\x82\x85\x85a\"\xBEV[\x95P\x95P\x95PPPPa\x1E\xC0V[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15a\x1E\xDAWa\x1E\xD9a&\xE2V[[\x82`\x03\x81\x11\x15a\x1E\xEDWa\x1E\xECa&\xE2V[[\x03\x15a %W`\x01`\x03\x81\x11\x15a\x1F\x07Wa\x1F\x06a&\xE2V[[\x82`\x03\x81\x11\x15a\x1F\x1AWa\x1F\x19a&\xE2V[[\x03a\x1FQW`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15a\x1FeWa\x1Fda&\xE2V[[\x82`\x03\x81\x11\x15a\x1FxWa\x1Fwa&\xE2V[[\x03a\x1F\xBCW\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1F\xB3\x91\x90a'}V[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15a\x1F\xCFWa\x1F\xCEa&\xE2V[[\x82`\x03\x81\x11\x15a\x1F\xE2Wa\x1F\xE1a&\xE2V[[\x03a $W\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a \x1B\x91\x90a(\x03V[`@Q\x80\x91\x03\x90\xFD[[PPV[_\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[_a \x9D\x83\x83a\r\x05V[a!qW`\x01__\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa!\x0Ea\x1D\x05V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa!uV[_\x90P[\x92\x91PPV[_a!\xA0\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba#\xA5V[\x90P\x92\x91PPV[_a!\xB3\x83\x83a\r\x05V[\x15a\"\x87W___\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\"$a\x1D\x05V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\"\x8BV[_\x90P[\x92\x91PPV[_a\"\xB6\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba$\x0CV[\x90P\x92\x91PPV[___\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15a\"\xFAW_`\x03\x85\x92P\x92P\x92Pa#\x9BV[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@Qa#\x1D\x94\x93\x92\x91\x90a3\xE1V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a#=W=__>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a#\x8EW_`\x01__\x1B\x93P\x93P\x93PPa#\x9BV[\x80___\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[_a#\xB0\x83\x83a%\x08V[a$\x02W\x82_\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x82_\x01\x80T\x90P\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP`\x01\x90Pa$\x06V[_\x90P[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x14a$\xFDW_`\x01\x82a$9\x91\x90a.\xF9V[\x90P_`\x01\x86_\x01\x80T\x90Pa$O\x91\x90a.\xF9V[\x90P\x80\x82\x14a$\xB5W_\x86_\x01\x82\x81T\x81\x10a$nWa$ma2\xACV[[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a$\x8FWa$\x8Ea2\xACV[[\x90_R` _ \x01\x81\x90UP\x83\x87`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UPP[\x85_\x01\x80T\x80a$\xC8Wa$\xC7a4$V[[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa%\x02V[_\x91PP[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x14\x15\x90P\x92\x91PPV[_`@Q\x90P\x90V[__\xFD[__\xFD[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a%m\x81a%9V[\x81\x14a%wW__\xFD[PV[_\x815\x90Pa%\x88\x81a%dV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a%\xA3Wa%\xA2a%1V[[_a%\xB0\x84\x82\x85\x01a%zV[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a%\xCD\x81a%\xB9V[\x82RPPV[_` \x82\x01\x90Pa%\xE6_\x83\x01\x84a%\xC4V[\x92\x91PPV[_\x81\x90P\x91\x90PV[a%\xFE\x81a%\xECV[\x81\x14a&\x08W__\xFD[PV[_\x815\x90Pa&\x19\x81a%\xF5V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a&5Wa&4a%1V[[_a&B\x85\x82\x86\x01a&\x0BV[\x92PP` a&S\x85\x82\x86\x01a&\x0BV[\x91PP\x92P\x92\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a&\x86\x82a&]V[\x90P\x91\x90PV[a&\x96\x81a&|V[\x81\x14a&\xA0W__\xFD[PV[_\x815\x90Pa&\xB1\x81a&\x8DV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a&\xCCWa&\xCBa%1V[[_a&\xD9\x84\x82\x85\x01a&\xA3V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x06\x81\x10a' Wa'\x1Fa&\xE2V[[PV[_\x81\x90Pa'0\x82a'\x0FV[\x91\x90PV[_a'?\x82a'#V[\x90P\x91\x90PV[a'O\x81a'5V[\x82RPPV[_` \x82\x01\x90Pa'h_\x83\x01\x84a'FV[\x92\x91PPV[a'w\x81a%\xECV[\x82RPPV[_` \x82\x01\x90Pa'\x90_\x83\x01\x84a'nV[\x92\x91PPV[_\x81\x90P\x91\x90PV[a'\xA8\x81a'\x96V[\x81\x14a'\xB2W__\xFD[PV[_\x815\x90Pa'\xC3\x81a'\x9FV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a'\xDEWa'\xDDa%1V[[_a'\xEB\x84\x82\x85\x01a'\xB5V[\x91PP\x92\x91PPV[a'\xFD\x81a'\x96V[\x82RPPV[_` \x82\x01\x90Pa(\x16_\x83\x01\x84a'\xF4V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a(2Wa(1a%1V[[_a(?\x85\x82\x86\x01a'\xB5V[\x92PP` a(P\x85\x82\x86\x01a&\xA3V[\x91PP\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15a(pWa(oa%1V[[_a(}\x85\x82\x86\x01a&\xA3V[\x92PP` a(\x8E\x85\x82\x86\x01a&\xA3V[\x91PP\x92P\x92\x90PV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a(\xB9Wa(\xB8a(\x98V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\xD6Wa(\xD5a(\x9CV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a(\xF2Wa(\xF1a(\xA0V[[\x92P\x92\x90PV[__` \x83\x85\x03\x12\x15a)\x0FWa)\x0Ea%1V[[_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a),Wa)+a%5V[[a)8\x85\x82\x86\x01a(\xA4V[\x92P\x92PP\x92P\x92\x90PV[a)M\x81a&|V[\x82RPPV[_` \x82\x01\x90Pa)f_\x83\x01\x84a)DV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a)\x82Wa)\x81a%1V[[_a)\x8F\x85\x82\x86\x01a'\xB5V[\x92PP` a)\xA0\x85\x82\x86\x01a&\x0BV[\x91PP\x92P\x92\x90PV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a)\xF0\x82a)\xAAV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a*\x0FWa*\x0Ea)\xBAV[[\x80`@RPPPV[_a*!a%(V[\x90Pa*-\x82\x82a)\xE7V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a*LWa*Ka)\xBAV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a*oa*j\x84a*2V[a*\x18V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a*\x92Wa*\x91a(\xA0V[[\x83[\x81\x81\x10\x15a*\xBBW\x80a*\xA7\x88\x82a&\xA3V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa*\x94V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a*\xD9Wa*\xD8a(\x98V[[\x815a*\xE9\x84\x82` \x86\x01a*]V[\x91PP\x92\x91PPV[_____`\xA0\x86\x88\x03\x12\x15a+\x0BWa+\na%1V[[_a+\x18\x88\x82\x89\x01a'\xB5V[\x95PP` a+)\x88\x82\x89\x01a&\x0BV[\x94PP`@a+:\x88\x82\x89\x01a&\x0BV[\x93PP``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+[Wa+Za%5V[[a+g\x88\x82\x89\x01a*\xC5V[\x92PP`\x80a+x\x88\x82\x89\x01a&\x0BV[\x91PP\x92\x95P\x92\x95\x90\x93PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a+\xB7\x81a&|V[\x82RPPV[_a+\xC8\x83\x83a+\xAEV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a+\xEA\x82a+\x85V[a+\xF4\x81\x85a+\x8FV[\x93Pa+\xFF\x83a+\x9FV[\x80_[\x83\x81\x10\x15a,/W\x81Qa,\x16\x88\x82a+\xBDV[\x97Pa,!\x83a+\xD4V[\x92PP`\x01\x81\x01\x90Pa,\x02V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra,T\x81\x84a+\xE0V[\x90P\x92\x91PPV[__`@\x83\x85\x03\x12\x15a,rWa,qa%1V[[_a,\x7F\x85\x82\x86\x01a&\x0BV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\xA0Wa,\x9Fa%5V[[a,\xAC\x85\x82\x86\x01a*\xC5V[\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a,\xCBWa,\xCAa%1V[[_a,\xD8\x84\x82\x85\x01a&\x0BV[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a-\x13\x81a%\xECV[\x82RPPV[_a-$\x83\x83a-\nV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a-F\x82a,\xE1V[a-P\x81\x85a,\xEBV[\x93Pa-[\x83a,\xFBV[\x80_[\x83\x81\x10\x15a-\x8BW\x81Qa-r\x88\x82a-\x19V[\x97Pa-}\x83a-0V[\x92PP`\x01\x81\x01\x90Pa-^V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra-\xB0\x81\x84a-<V[\x90P\x92\x91PPV[____``\x85\x87\x03\x12\x15a-\xD0Wa-\xCFa%1V[[_a-\xDD\x87\x82\x88\x01a&\x0BV[\x94PP` a-\xEE\x87\x82\x88\x01a&\xA3V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.\x0FWa.\x0Ea%5V[[a.\x1B\x87\x82\x88\x01a(\xA4V[\x92P\x92PP\x92\x95\x91\x94P\x92PV[_`@\x82\x01\x90Pa.<_\x83\x01\x85a)DV[a.I` \x83\x01\x84a'nV[\x93\x92PPPV[_``\x82\x01\x90Pa.c_\x83\x01\x86a)DV[a.p` \x83\x01\x85a'nV[a.}`@\x83\x01\x84a'nV[\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a.\xBC\x82a%\xECV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a.\xEEWa.\xEDa.\x85V[[`\x01\x82\x01\x90P\x91\x90PV[_a/\x03\x82a%\xECV[\x91Pa/\x0E\x83a%\xECV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a/&Wa/%a.\x85V[[\x92\x91PPV[_a/6\x82a%\xECV[\x91Pa/A\x83a%\xECV[\x92P\x82\x82\x02a/O\x81a%\xECV[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a/fWa/ea.\x85V[[P\x92\x91PPV[_a/w\x82a%\xECV[\x91Pa/\x82\x83a%\xECV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a/\x9AWa/\x99a.\x85V[[\x92\x91PPV[_`@\x82\x01\x90Pa/\xB3_\x83\x01\x85a'nV[a/\xC0` \x83\x01\x84a'nV[\x93\x92PPPV[_\x82\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a0\x15W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a0(Wa0'a/\xD1V[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a0\x8A\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a0OV[a0\x94\x86\x83a0OV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a0\xCFa0\xCAa0\xC5\x84a%\xECV[a0\xACV[a%\xECV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a0\xE8\x83a0\xB5V[a0\xFCa0\xF4\x82a0\xD6V[\x84\x84Ta0[V[\x82UPPPPV[__\x90P\x90V[a1\x13a1\x04V[a1\x1E\x81\x84\x84a0\xDFV[PPPV[_[\x82\x81\x10\x15a1DWa19_\x82\x84\x01a1\x0BV[`\x01\x81\x01\x90Pa1%V[PPPV[`\x1F\x82\x11\x15a1\x97W\x82\x82\x11\x15a1\x96Wa1c\x81a0.V[a1l\x83a0@V[a1u\x85a0@V[` \x86\x10\x15a1\x82W_\x90P[\x80\x83\x01a1\x91\x82\x84\x03\x82a1#V[PPPP[[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_a1\xB7_\x19\x84`\x08\x02a1\x9CV[\x19\x80\x83\x16\x91PP\x92\x91PPV[_a1\xCF\x83\x83a1\xA8V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a1\xE9\x83\x83a/\xC7V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2\x02Wa2\x01a)\xBAV[[a2\x0C\x82Ta/\xFEV[a2\x17\x82\x82\x85a1IV[_`\x1F\x83\x11`\x01\x81\x14a2DW_\x84\x15a22W\x82\x87\x015\x90P[a2<\x85\x82a1\xC4V[\x86UPa2\xA3V[`\x1F\x19\x84\x16a2R\x86a0.V[_[\x82\x81\x10\x15a2yW\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa2TV[\x86\x83\x10\x15a2\x96W\x84\x89\x015a2\x92`\x1F\x89\x16\x82a1\xA8V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_`@\x82\x01\x90Pa2\xEC_\x83\x01\x85a'FV[a2\xF9` \x83\x01\x84a'FV[\x93\x92PPPV[_``\x82\x01\x90Pa3\x13_\x83\x01\x86a'nV[a3 ` \x83\x01\x85a'nV[a3-`@\x83\x01\x84a)DV[\x94\x93PPPPV[_`@\x82\x01\x90Pa3H_\x83\x01\x85a'nV[a3U` \x83\x01\x84a)DV[\x93\x92PPPV[_`\x80\x82\x01\x90Pa3o_\x83\x01\x87a)DV[a3|` \x83\x01\x86a'nV[a3\x89`@\x83\x01\x85a'nV[a3\x96``\x83\x01\x84a)DV[\x95\x94PPPPPV[_`@\x82\x01\x90Pa3\xB2_\x83\x01\x85a)DV[a3\xBF` \x83\x01\x84a'\xF4V[\x93\x92PPPV[_`\xFF\x82\x16\x90P\x91\x90PV[a3\xDB\x81a3\xC6V[\x82RPPV[_`\x80\x82\x01\x90Pa3\xF4_\x83\x01\x87a'\xF4V[a4\x01` \x83\x01\x86a3\xD2V[a4\x0E`@\x83\x01\x85a'\xF4V[a4\x1B``\x83\x01\x84a'\xF4V[\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \x17?\x86b\\\x8F\x14\xB1\xD4%&u\xF9\xC0\xE1`O\x04\xC9\xA2_t\x19N\xB1X|\x97o\xC8\xD5\x07dsolcC\0\x08!\x003",
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
    /**Event with signature `ClientAuthenticated(address,uint256)` and selector `0x23d369583ce23023a1ca4baf14a48693a3ee9b3abf2483e41d639e57cb1f9dab`.
```solidity
event ClientAuthenticated(address indexed client, uint256 nonce);
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
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
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
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ClientAuthenticated(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                35u8, 211u8, 105u8, 88u8, 60u8, 226u8, 48u8, 35u8, 161u8, 202u8, 75u8,
                175u8, 20u8, 164u8, 134u8, 147u8, 163u8, 238u8, 155u8, 58u8, 191u8, 36u8,
                131u8, 228u8, 29u8, 99u8, 158u8, 87u8, 203u8, 31u8, 157u8, 171u8,
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
                    nonce: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
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
    /**Function with signature `authenticateClient(uint256,address,bytes)` and selector `0xf9d8ff15`.
```solidity
function authenticateClient(uint256 nonce, address clientAddr, bytes memory signature) external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct authenticateClientCall {
        #[allow(missing_docs)]
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub clientAddr: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`authenticateClient(uint256,address,bytes)`](authenticateClientCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct authenticateClientReturn {
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
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
                    (value.nonce, value.clientAddr, value.signature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for authenticateClientCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        nonce: tuple.0,
                        clientAddr: tuple.1,
                        signature: tuple.2,
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
            impl ::core::convert::From<authenticateClientReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: authenticateClientReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for authenticateClientReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for authenticateClientCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "authenticateClient(uint256,address,bytes)";
            const SELECTOR: [u8; 4] = [249u8, 216u8, 255u8, 21u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
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
                        let r: authenticateClientReturn = r.into();
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
                        let r: authenticateClientReturn = r.into();
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
    /**Function with signature `resetCoordinator(bytes32,uint256,uint256,address[],uint256)` and selector `0x930ceae6`.
```solidity
function resetCoordinator(bytes32 stoffelProgramHash, uint256 n, uint256 t, address[] memory initialMPCNodes, uint256 nInputs) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resetCoordinatorCall {
        #[allow(missing_docs)]
        pub stoffelProgramHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub n: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub t: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub initialMPCNodes: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub nInputs: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`resetCoordinator(bytes32,uint256,uint256,address[],uint256)`](resetCoordinatorCall) function.
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U256,
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
                        value.n,
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
                        n: tuple.1,
                        t: tuple.2,
                        initialMPCNodes: tuple.3,
                        nInputs: tuple.4,
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
            const SIGNATURE: &'static str = "resetCoordinator(bytes32,uint256,uint256,address[],uint256)";
            const SELECTOR: [u8; 4] = [147u8, 12u8, 234u8, 230u8];
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
    /**Function with signature `resetInputManager(uint256)` and selector `0xe23fb1f8`.
```solidity
function resetInputManager(uint256 nIndicesToReserve) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct resetInputManagerCall {
        #[allow(missing_docs)]
        pub nIndicesToReserve: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`resetInputManager(uint256)`](resetInputManagerCall) function.
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
            impl ::core::convert::From<resetInputManagerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: resetInputManagerCall) -> Self {
                    (value.nIndicesToReserve,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for resetInputManagerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { nIndicesToReserve: tuple.0 }
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
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = resetInputManagerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "resetInputManager(uint256)";
            const SELECTOR: [u8; 4] = [226u8, 63u8, 177u8, 248u8];
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
            [35u8, 40u8, 189u8, 18u8],
            [36u8, 138u8, 156u8, 163u8],
            [47u8, 47u8, 241u8, 93u8],
            [48u8, 16u8, 76u8, 62u8],
            [54u8, 86u8, 138u8, 190u8],
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
            [147u8, 12u8, 234u8, 230u8],
            [162u8, 23u8, 253u8, 223u8],
            [163u8, 36u8, 106u8, 211u8],
            [175u8, 32u8, 111u8, 40u8],
            [187u8, 81u8, 254u8, 240u8],
            [192u8, 121u8, 244u8, 149u8],
            [202u8, 21u8, 200u8, 115u8],
            [203u8, 156u8, 76u8, 196u8],
            [213u8, 71u8, 116u8, 31u8],
            [216u8, 39u8, 13u8, 206u8],
            [226u8, 63u8, 177u8, 248u8],
            [235u8, 174u8, 53u8, 231u8],
            [242u8, 253u8, 227u8, 139u8],
            [249u8, 216u8, 255u8, 21u8],
            [252u8, 120u8, 178u8, 232u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(supportsInterface),
            ::core::stringify!(submitMaskedInput),
            ::core::stringify!(isDesignatedParty),
            ::core::stringify!(round),
            ::core::stringify!(creationBlock),
            ::core::stringify!(availableInputMasks),
            ::core::stringify!(getRoleAdmin),
            ::core::stringify!(grantRole),
            ::core::stringify!(PARTY_ROLE),
            ::core::stringify!(renounceRole),
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
            ::core::stringify!(resetCoordinator),
            ::core::stringify!(DEFAULT_ADMIN_ROLE),
            ::core::stringify!(getRoleMembers),
            ::core::stringify!(resetAccessControl),
            ::core::stringify!(collectInputs),
            ::core::stringify!(reserveInputMasks),
            ::core::stringify!(getRoleMemberCount),
            ::core::stringify!(startPreprocessing),
            ::core::stringify!(revokeRole),
            ::core::stringify!(creationTime),
            ::core::stringify!(resetInputManager),
            ::core::stringify!(obtainInputMasks),
            ::core::stringify!(transferOwnership),
            ::core::stringify!(authenticateClient),
            ::core::stringify!(isParty),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <supportsInterfaceCall as alloy_sol_types::SolCall>::SIGNATURE,
            <submitMaskedInputCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isDesignatedPartyCall as alloy_sol_types::SolCall>::SIGNATURE,
            <roundCall as alloy_sol_types::SolCall>::SIGNATURE,
            <creationBlockCall as alloy_sol_types::SolCall>::SIGNATURE,
            <availableInputMasksCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRoleAdminCall as alloy_sol_types::SolCall>::SIGNATURE,
            <grantRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <PARTY_ROLECall as alloy_sol_types::SolCall>::SIGNATURE,
            <renounceRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
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
            <resetCoordinatorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRoleMembersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <resetAccessControlCall as alloy_sol_types::SolCall>::SIGNATURE,
            <collectInputsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <reserveInputMasksCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRoleMemberCountCall as alloy_sol_types::SolCall>::SIGNATURE,
            <startPreprocessingCall as alloy_sol_types::SolCall>::SIGNATURE,
            <revokeRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <creationTimeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <resetInputManagerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <obtainInputMasksCall as alloy_sol_types::SolCall>::SIGNATURE,
            <transferOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <authenticateClientCall as alloy_sol_types::SolCall>::SIGNATURE,
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
                Self::PARTY_ROLE(_) => {
                    <PARTY_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::authenticateClient(_) => {
                    <authenticateClientCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::availableInputMasks(_) => {
                    <availableInputMasksCall as alloy_sol_types::SolCall>::SELECTOR
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
            [171u8, 220u8, 224u8, 106u8],
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
            ::core::stringify!(NotAnExistingParty),
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
            <NotAnExistingParty as alloy_sol_types::SolError>::SIGNATURE,
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
        const COUNT: usize = 12usize;
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
                32u8, 245u8, 94u8, 208u8, 201u8, 47u8, 43u8, 177u8, 200u8, 130u8, 84u8,
                136u8, 225u8, 227u8, 201u8, 132u8, 99u8, 208u8, 36u8, 178u8, 164u8, 45u8,
                189u8, 36u8, 131u8, 140u8, 63u8, 117u8, 38u8, 15u8, 67u8, 233u8,
            ],
            [
                35u8, 211u8, 105u8, 88u8, 60u8, 226u8, 48u8, 35u8, 161u8, 202u8, 75u8,
                175u8, 20u8, 164u8, 134u8, 147u8, 163u8, 238u8, 155u8, 58u8, 191u8, 36u8,
                131u8, 228u8, 29u8, 99u8, 158u8, 87u8, 203u8, 31u8, 157u8, 171u8,
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
            ::core::stringify!(MPCStarted),
            ::core::stringify!(ClientAuthenticated),
            ::core::stringify!(ExecutionDone),
            ::core::stringify!(RoleGranted),
            ::core::stringify!(OutputSendingStarted),
            ::core::stringify!(InputCollectionStarted),
            ::core::stringify!(InputMaskReservationStarted),
            ::core::stringify!(OwnershipTransferred),
            ::core::stringify!(InitializeStoffelAccessControl),
            ::core::stringify!(ReservedInputEvent),
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
            <MPCStarted as alloy_sol_types::SolEvent>::SIGNATURE,
            <ClientAuthenticated as alloy_sol_types::SolEvent>::SIGNATURE,
            <ExecutionDone as alloy_sol_types::SolEvent>::SIGNATURE,
            <RoleGranted as alloy_sol_types::SolEvent>::SIGNATURE,
            <OutputSendingStarted as alloy_sol_types::SolEvent>::SIGNATURE,
            <InputCollectionStarted as alloy_sol_types::SolEvent>::SIGNATURE,
            <InputMaskReservationStarted as alloy_sol_types::SolEvent>::SIGNATURE,
            <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE,
            <InitializeStoffelAccessControl as alloy_sol_types::SolEvent>::SIGNATURE,
            <ReservedInputEvent as alloy_sol_types::SolEvent>::SIGNATURE,
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
            nonce: alloy::sol_types::private::primitives::aliases::U256,
            clientAddr: alloy::sol_types::private::Address,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, authenticateClientCall, N> {
            self.call_builder(
                &authenticateClientCall {
                    nonce,
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
            n: alloy::sol_types::private::primitives::aliases::U256,
            t: alloy::sol_types::private::primitives::aliases::U256,
            initialMPCNodes: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            nInputs: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, resetCoordinatorCall, N> {
            self.call_builder(
                &resetCoordinatorCall {
                    stoffelProgramHash,
                    n,
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
        ) -> alloy_contract::SolCallBuilder<&P, resetInputManagerCall, N> {
            self.call_builder(
                &resetInputManagerCall {
                    nIndicesToReserve,
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

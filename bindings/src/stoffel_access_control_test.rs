///Module containing a contract's types and functions.
/**

```solidity
library StdInvariant {
    struct FuzzArtifactSelector { string artifact; bytes4[] selectors; }
    struct FuzzInterface { address addr; string[] artifacts; }
    struct FuzzSelector { address addr; bytes4[] selectors; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod StdInvariant {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct FuzzArtifactSelector { string artifact; bytes4[] selectors; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzArtifactSelector {
        #[allow(missing_docs)]
        pub artifact: alloy::sol_types::private::String,
        #[allow(missing_docs)]
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<4>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::String,
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
        impl ::core::convert::From<FuzzArtifactSelector> for UnderlyingRustTuple<'_> {
            fn from(value: FuzzArtifactSelector) -> Self {
                (value.artifact, value.selectors)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FuzzArtifactSelector {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    artifact: tuple.0,
                    selectors: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FuzzArtifactSelector {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FuzzArtifactSelector {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.artifact,
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
        impl alloy_sol_types::SolType for FuzzArtifactSelector {
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
        impl alloy_sol_types::SolStruct for FuzzArtifactSelector {
            const NAME: &'static str = "FuzzArtifactSelector";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzArtifactSelector(string artifact,bytes4[] selectors)",
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.artifact,
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
        impl alloy_sol_types::EventTopic for FuzzArtifactSelector {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.artifact,
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
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.artifact,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct FuzzInterface { address addr; string[] artifacts; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzInterface {
        #[allow(missing_docs)]
        pub addr: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
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
        #[allow(dead_code)]
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct FuzzSelector { address addr; bytes4[] selectors; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzSelector {
        #[allow(missing_docs)]
        pub addr: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
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
        #[allow(dead_code)]
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
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> StdInvariantInstance<P, N> {
        StdInvariantInstance::<P, N>::new(address, __provider)
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
    pub struct StdInvariantInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for StdInvariantInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("StdInvariantInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > StdInvariantInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`StdInvariant`](self) contract instance.

See the [wrapper's documentation](`StdInvariantInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> StdInvariantInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> StdInvariantInstance<P, N> {
            StdInvariantInstance {
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
    > StdInvariantInstance<P, N> {
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
    > StdInvariantInstance<P, N> {
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
library StdInvariant {
    struct FuzzArtifactSelector {
        string artifact;
        bytes4[] selectors;
    }
    struct FuzzInterface {
        address addr;
        string[] artifacts;
    }
    struct FuzzSelector {
        address addr;
        bytes4[] selectors;
    }
}

interface StoffelAccessControlTest {
    event InitializeStoffelAccessControl(uint256 nParties, uint256 t, address initializer);
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

    function IS_TEST() external view returns (bool);
    function ac() external view returns (address);
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function failed() external view returns (bool);
    function setUp() external;
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
    function test_constructor_emitsInitializeEvent() external;
    function test_constructor_grantRoles() external view;
    function test_grantRoleAgain() external;
    function test_grantRole_designatedPartyCanGrantRoles() external;
    function test_grantRole_revertsIfCallerLacksDesignatedPartyRole() external;
    function test_isDesignatedParty() external;
    function test_isParty() external;
    function test_renounceRole_partyCanRenounceOwnRole() external;
    function test_renounceRole_revertsIfRenouncingForAnotherAccount() external;
    function test_revokeRole_designatedPartyCanRevokePartyRole() external;
    function test_revokeRole_partyCountDecreases() external;
    function test_revokeRole_revertsIfCallerLacksDesignatedPartyRole() external;
    function test_supportsInterface_IStoffelAccessControl() external view;
}
```

...which was generated by the following JSON ABI:
```json
[
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
    "name": "ac",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract StoffelAccessControl"
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
    "name": "excludeSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedSelectors_",
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
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "setUp",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "targetArtifactSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedArtifactSelectors_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzArtifactSelector[]",
        "components": [
          {
            "name": "artifact",
            "type": "string",
            "internalType": "string"
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
    "name": "test_constructor_emitsInitializeEvent",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_constructor_grantRoles",
    "inputs": [],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "test_grantRoleAgain",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_grantRole_designatedPartyCanGrantRoles",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_grantRole_revertsIfCallerLacksDesignatedPartyRole",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_isDesignatedParty",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_isParty",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_renounceRole_partyCanRenounceOwnRole",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_renounceRole_revertsIfRenouncingForAnotherAccount",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_revokeRole_designatedPartyCanRevokePartyRole",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_revokeRole_partyCountDecreases",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_revokeRole_revertsIfCallerLacksDesignatedPartyRole",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_supportsInterface_IStoffelAccessControl",
    "inputs": [],
    "outputs": [],
    "stateMutability": "view"
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
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod StoffelAccessControlTest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60806040526001600c5f6101000a81548160ff0219169083151502179055506001601f5f6101000a81548160ff02191690831515021790555061007c6040518060400160405280600a81526020017f44455349474e415445440000000000000000000000000000000000000000000081525061025560201b60201c565b60205f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506100ff6040518060400160405280600681526020017f504152545931000000000000000000000000000000000000000000000000000081525061025560201b60201c565b60215f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506101826040518060400160405280600681526020017f504152545932000000000000000000000000000000000000000000000000000081525061025560201b60201c565b60225f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506102056040518060400160405280600681526020017f504152545933000000000000000000000000000000000000000000000000000081525061025560201b60201c565b60235f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555034801561024f575f5ffd5b5061057e565b5f6102658261026f60201b60201c565b5080915050919050565b5f5f826040516020016102829190610419565b604051602081830303815290604052805190602001205f1c90507f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c73ffffffffffffffffffffffffffffffffffffffff1663ffa18649826040518263ffffffff1660e01b81526004016102f79190610447565b602060405180830381865afa158015610312573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061033691906104be565b91507f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c73ffffffffffffffffffffffffffffffffffffffff1663c657c71883856040518363ffffffff1660e01b8152600401610395929190610550565b5f604051808303815f87803b1580156103ac575f5ffd5b505af11580156103be573d5f5f3e3d5ffd5b50505050915091565b5f81519050919050565b5f81905092915050565b8281835e5f83830152505050565b5f6103f3826103c7565b6103fd81856103d1565b935061040d8185602086016103db565b80840191505092915050565b5f61042482846103e9565b915081905092915050565b5f819050919050565b6104418161042f565b82525050565b5f60208201905061045a5f830184610438565b92915050565b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61048d82610464565b9050919050565b61049d81610483565b81146104a7575f5ffd5b50565b5f815190506104b881610494565b92915050565b5f602082840312156104d3576104d2610460565b5b5f6104e0848285016104aa565b91505092915050565b6104f281610483565b82525050565b5f82825260208201905092915050565b5f601f19601f8301169050919050565b5f610522826103c7565b61052c81856104f8565b935061053c8185602086016103db565b61054581610508565b840191505092915050565b5f6040820190506105635f8301856104e9565b81810360208301526105758184610518565b90509392505050565b6160d88061058b5f395ff3fe608060405234801561000f575f5ffd5b50600436106101a7575f3560e01c806385226c81116100f7578063cd099e6a11610095578063e05051991161006f578063e05051991461036d578063e20c9f7114610377578063e49e9ff514610395578063fa7626d41461039f576101a7565b8063cd099e6a1461034f578063d3fe9e7d14610359578063d7c3e53214610363576101a7565b8063b0464fdc116100d1578063b0464fdc146102eb578063b5508aa914610309578063ba2cbd3514610327578063ba414fa614610331576101a7565b806385226c81146102a5578063916a17c6146102c35780639bc8a619146102e1576101a7565b80633e5e3c231161016457806354b7c39b1161013e57806354b7c39b146102555780635c972be51461027357806365c9f4851461027d57806366d9a9a014610287576101a7565b80633e5e3c231461020f5780633f7286f41461022d578063484b3f361461024b576101a7565b80630a9254e4146101ab5780631ed7831c146101b5578063200a56ce146101d35780632ade3880146101dd578063374d1fd5146101fb5780633a5e4f2314610205575b5f5ffd5b6101b36103bd565b005b6101bd61076a565b6040516101ca919061398a565b60405180910390f35b6101db6107f5565b005b6101e5610a74565b6040516101f29190613bca565b60405180910390f35b610203610bf8565b005b61020d611232565b005b6102176113d6565b604051610224919061398a565b60405180910390f35b610235611461565b604051610242919061398a565b60405180910390f35b6102536114ec565b005b61025d6116fd565b60405161026a9190613c45565b60405180910390f35b61027b611723565b005b610285611934565b005b61028f611aed565b60405161029c9190613e3c565b60405180910390f35b6102ad611c6f565b6040516102ba9190613edf565b60405180910390f35b6102cb611d43565b6040516102d89190613ff4565b60405180910390f35b6102e9611e8a565b005b6102f36120f1565b6040516103009190613ff4565b60405180910390f35b610311612238565b60405161031e9190613edf565b60405180910390f35b61032f61230c565b005b6103396125fc565b604051610346919061402e565b60405180910390f35b610357612703565b005b610361612992565b005b61036b612c2d565b005b610375612e59565b005b61037f6130b7565b60405161038c919061398a565b60405180910390f35b61039d613142565b005b6103a7613562565b6040516103b4919061402e565b60405180910390f35b5f600467ffffffffffffffff8111156103d9576103d8614047565b5b6040519080825280602002602001820160405280156104075781602001602082028036833780820191505090505b50905060205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16815f8151811061043f5761043e614074565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505060215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16816001815181106104af576104ae614074565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505060225f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff168160028151811061051f5761051e614074565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505060235f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff168160038151811061058f5761058e614074565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250506001816040516105d890613896565b6105e39291906140e3565b604051809103905ff0801580156105fc573d5f5f3e3d5ffd5b50601f60016101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166330104c3e6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156106a8573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106cc9190614148565b602481905550601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16637f35b5606040518163ffffffff1660e01b8152600401602060405180830381865afa15801561073d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107619190614148565b60258190555050565b606060168054806020026020016040519081016040528092919081815260200182805480156107eb57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116107a2575b5050505050905090565b5f6108346040518060400160405280600981526020017f4e45575f50415254590000000000000000000000000000000000000000000000815250613574565b9050737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663ca669fa760215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b81526004016108a49190614182565b5f604051808303815f87803b1580156108bb575f5ffd5b505af11580156108cd573d5f5f3e3d5ffd5b50505050737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663f28dceb363e2517d3f60e01b60215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1660255460405160240161093f9291906141aa565b604051602081830303815290604052907bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040518263ffffffff1660e01b81526004016109b89190614223565b5f604051808303815f87803b1580156109cf575f5ffd5b505af11580156109e1573d5f5f3e3d5ffd5b50505050601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16632f2ff15d602454836040518363ffffffff1660e01b8152600401610a44929190614243565b5f604051808303815f87803b158015610a5b575f5ffd5b505af1158015610a6d573d5f5f3e3d5ffd5b5050505050565b6060601e805480602002602001604051908101604052809291908181526020015f905b82821015610bef578382905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200160018201805480602002602001604051908101604052809291908181526020015f905b82821015610bd8578382905f5260205f20018054610b4d90614297565b80601f0160208091040260200160405190810160405280929190818152602001828054610b7990614297565b8015610bc45780601f10610b9b57610100808354040283529160200191610bc4565b820191905f5260205f20905b815481529060010190602001808311610ba757829003601f168201915b505050505081526020019060010190610b30565b505050508152505081526020019060010190610a97565b50505050905090565b610cbf601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166391d1485460245460205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b8152600401610c7b929190614243565b602060405180830381865afa158015610c96573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cba91906142f1565b613588565b610d86601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166391d1485460245460215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b8152600401610d42929190614243565b602060405180830381865afa158015610d5d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d8191906142f1565b613588565b610e4d601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166391d1485460245460225f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b8152600401610e09929190614243565b602060405180830381865afa158015610e24573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e4891906142f1565b613588565b610f14601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166391d1485460245460235f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b8152600401610ed0929190614243565b602060405180830381865afa158015610eeb573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f0f91906142f1565b613588565b610fdb601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166391d1485460255460205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b8152600401610f97929190614243565b602060405180830381865afa158015610fb2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610fd691906142f1565b613588565b6110a2601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166391d1485460255460215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b815260040161105e929190614243565b602060405180830381865afa158015611079573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061109d91906142f1565b613618565b611169601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166391d1485460255460225f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b8152600401611125929190614243565b602060405180830381865afa158015611140573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061116491906142f1565b613618565b611230601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166391d1485460255460235f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b81526004016111ec929190614243565b602060405180830381865afa158015611207573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061122b91906142f1565b613618565b565b6112f5601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663fc78b2e860215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b81526004016112b19190614182565b602060405180830381865afa1580156112cc573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112f091906142f1565b613588565b6113d4601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663fc78b2e86113746040518060400160405280600881526020017f535452414e474552000000000000000000000000000000000000000000000000815250613574565b6040518263ffffffff1660e01b81526004016113909190614182565b602060405180830381865afa1580156113ab573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113cf91906142f1565b613618565b565b6060601880548060200260200160405190810160405280929190818152602001828054801561145757602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001906001019080831161140e575b5050505050905090565b606060178054806020026020016040519081016040528092919081815260200182805480156114e257602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311611499575b5050505050905090565b737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663ca669fa760215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b815260040161155a9190614182565b5f604051808303815f87803b158015611571575f5ffd5b505af1158015611583573d5f5f3e3d5ffd5b50505050601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166336568abe60245460215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b8152600401611607929190614243565b5f604051808303815f87803b15801561161e575f5ffd5b505af1158015611630573d5f5f3e3d5ffd5b505050506116fb601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166391d1485460245460215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b81526004016116b7929190614243565b602060405180830381865afa1580156116d2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116f691906142f1565b613618565b565b601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663ca669fa760205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b81526004016117919190614182565b5f604051808303815f87803b1580156117a8575f5ffd5b505af11580156117ba573d5f5f3e3d5ffd5b50505050601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663d547741f60245460215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b815260040161183e929190614243565b5f604051808303815f87803b158015611855575f5ffd5b505af1158015611867573d5f5f3e3d5ffd5b50505050611932601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166391d1485460245460215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b81526004016118ee929190614243565b602060405180830381865afa158015611909573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061192d91906142f1565b613618565b565b737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663ca669fa760215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b81526004016119a29190614182565b5f604051808303815f87803b1580156119b9575f5ffd5b505af11580156119cb573d5f5f3e3d5ffd5b50505050737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663f48448146040518163ffffffff1660e01b81526004015f604051808303815f87803b158015611a28575f5ffd5b505af1158015611a3a573d5f5f3e3d5ffd5b50505050601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166336568abe60245460225f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b8152600401611abe929190614243565b5f604051808303815f87803b158015611ad5575f5ffd5b505af1158015611ae7573d5f5f3e3d5ffd5b50505050565b6060601b805480602002602001604051908101604052809291908181526020015f905b82821015611c66578382905f5260205f2090600202016040518060400160405290815f82018054611b4090614297565b80601f0160208091040260200160405190810160405280929190818152602001828054611b6c90614297565b8015611bb75780601f10611b8e57610100808354040283529160200191611bb7565b820191905f5260205f20905b815481529060010190602001808311611b9a57829003601f168201915b5050505050815260200160018201805480602002602001604051908101604052809291908181526020018280548015611c4e57602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191681526020019060040190602082600301049283019260010382029150808411611bfb5790505b50505050508152505081526020019060010190611b10565b50505050905090565b6060601a805480602002602001604051908101604052809291908181526020015f905b82821015611d3a578382905f5260205f20018054611caf90614297565b80601f0160208091040260200160405190810160405280929190818152602001828054611cdb90614297565b8015611d265780601f10611cfd57610100808354040283529160200191611d26565b820191905f5260205f20905b815481529060010190602001808311611d0957829003601f168201915b505050505081526020019060010190611c92565b50505050905090565b6060601d805480602002602001604051908101604052809291908181526020015f905b82821015611e81578382905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200160018201805480602002602001604051908101604052809291908181526020018280548015611e6957602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191681526020019060040190602082600301049283019260010382029150808411611e165790505b50505050508152505081526020019060010190611d66565b50505050905090565b611f4d601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166313ff6dd560205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b8152600401611f099190614182565b602060405180830381865afa158015611f24573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f4891906142f1565b613588565b612010601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166313ff6dd560215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b8152600401611fcc9190614182565b602060405180830381865afa158015611fe7573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061200b91906142f1565b613618565b6120ef601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166313ff6dd561208f6040518060400160405280600881526020017f535452414e474552000000000000000000000000000000000000000000000000815250613574565b6040518263ffffffff1660e01b81526004016120ab9190614182565b602060405180830381865afa1580156120c6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906120ea91906142f1565b613618565b565b6060601c805480602002602001604051908101604052809291908181526020015f905b8282101561222f578382905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020016001820180548060200260200160405190810160405280929190818152602001828054801561221757602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916815260200190600401906020826003010492830192600103820291508084116121c45790505b50505050508152505081526020019060010190612114565b50505050905090565b60606019805480602002602001604051908101604052809291908181526020015f905b82821015612303578382905f5260205f2001805461227890614297565b80601f01602080910402602001604051908101604052809291908181526020018280546122a490614297565b80156122ef5780601f106122c6576101008083540402835291602001916122ef565b820191905f5260205f20905b8154815290600101906020018083116122d257829003601f168201915b50505050508152602001906001019061225b565b50505050905090565b5f600467ffffffffffffffff81111561232857612327614047565b5b6040519080825280602002602001820160405280156123565781602001602082028036833780820191505090505b50905060205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16815f8151811061238e5761238d614074565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505060215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16816001815181106123fe576123fd614074565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505060225f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff168160028151811061246e5761246d614074565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505060235f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16816003815181106124de576124dd614074565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff1681525050737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663440ed10d6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015612571575f5ffd5b505af1158015612583573d5f5f3e3d5ffd5b505050507fa2df7830e0bedef7b1116bf547b467b16b50b3bd23146c9e099878d14e89301a60046001306040516125bc93929190614355565b60405180910390a16001816040516125d390613896565b6125de9291906140e3565b604051809103905ff0801580156125f7573d5f5f3e3d5ffd5b505050565b5f60085f9054906101000a900460ff161561261a5760019050612700565b5f5f1b7f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c73ffffffffffffffffffffffffffffffffffffffff1663667f9d707f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c7f6661696c656400000000000000000000000000000000000000000000000000006040518363ffffffff1660e01b81526004016126bc9291906141aa565b602060405180830381865afa1580156126d7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906126fb9190614148565b141590505b90565b5f601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663ca15c8736024546040518263ffffffff1660e01b8152600401612761919061438a565b602060405180830381865afa15801561277c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906127a091906143cd565b9050737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663ca669fa760205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b81526004016128109190614182565b5f604051808303815f87803b158015612827575f5ffd5b505af1158015612839573d5f5f3e3d5ffd5b50505050601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16632f2ff15d60245460215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b81526004016128bd929190614243565b5f604051808303815f87803b1580156128d4575f5ffd5b505af11580156128e6573d5f5f3e3d5ffd5b5050505061298f601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663ca15c8736024546040518263ffffffff1660e01b815260040161294a919061438a565b602060405180830381865afa158015612965573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061298991906143cd565b826136a9565b50565b5f601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663ca15c8736024546040518263ffffffff1660e01b81526004016129f0919061438a565b602060405180830381865afa158015612a0b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612a2f91906143cd565b9050737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663ca669fa760205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b8152600401612a9f9190614182565b5f604051808303815f87803b158015612ab6575f5ffd5b505af1158015612ac8573d5f5f3e3d5ffd5b50505050601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663d547741f60245460215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b8152600401612b4c929190614243565b5f604051808303815f87803b158015612b63575f5ffd5b505af1158015612b75573d5f5f3e3d5ffd5b50505050612c2a601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663ca15c8736024546040518263ffffffff1660e01b8152600401612bd9919061438a565b602060405180830381865afa158015612bf4573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612c1891906143cd565b600183612c259190614425565b6136a9565b50565b612cef601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166301ffc9a77f07effe0d000000000000000000000000000000000000000000000000000000006040518263ffffffff1660e01b8152600401612cab9190614467565b602060405180830381865afa158015612cc6573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612cea91906142f1565b613588565b612db1601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166301ffc9a77f7965db0b000000000000000000000000000000000000000000000000000000006040518263ffffffff1660e01b8152600401612d6d9190614467565b602060405180830381865afa158015612d88573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612dac91906142f1565b613588565b612e57601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166301ffc9a763deadbeef6040518263ffffffff1660e01b8152600401612e1391906144c5565b602060405180830381865afa158015612e2e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612e5291906142f1565b613618565b565b737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663ca669fa760215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b8152600401612ec79190614182565b5f604051808303815f87803b158015612ede575f5ffd5b505af1158015612ef0573d5f5f3e3d5ffd5b50505050737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663f28dceb363e2517d3f60e01b60215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16602554604051602401612f629291906141aa565b604051602081830303815290604052907bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040518263ffffffff1660e01b8152600401612fdb9190614223565b5f604051808303815f87803b158015612ff2575f5ffd5b505af1158015613004573d5f5f3e3d5ffd5b50505050601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663d547741f60245460225f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b8152600401613088929190614243565b5f604051808303815f87803b15801561309f575f5ffd5b505af11580156130b1573d5f5f3e3d5ffd5b50505050565b6060601580548060200260200160405190810160405280929190818152602001828054801561313857602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116130ef575b5050505050905090565b5f6131816040518060400160405280600981526020017f4e45575f50415254590000000000000000000000000000000000000000000000815250613574565b9050737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663ca669fa760205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b81526004016131f19190614182565b5f604051808303815f87803b158015613208575f5ffd5b505af115801561321a573d5f5f3e3d5ffd5b50505050601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16632f2ff15d602454836040518363ffffffff1660e01b815260040161327d929190614243565b5f604051808303815f87803b158015613294575f5ffd5b505af11580156132a6573d5f5f3e3d5ffd5b50505050613350601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166391d14854602454846040518363ffffffff1660e01b815260040161330c929190614243565b602060405180830381865afa158015613327573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061334b91906142f1565b613588565b737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663ca669fa760205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b81526004016133be9190614182565b5f604051808303815f87803b1580156133d5575f5ffd5b505af11580156133e7573d5f5f3e3d5ffd5b50505050601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16632f2ff15d60255460215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b815260040161346b929190614243565b5f604051808303815f87803b158015613482575f5ffd5b505af1158015613494573d5f5f3e3d5ffd5b5050505061355f601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166391d1485460255460215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b815260040161351b929190614243565b602060405180830381865afa158015613536573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061355a91906142f1565b613588565b50565b601f5f9054906101000a900460ff1681565b5f61357e8261373e565b5080915050919050565b80613615577f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c73ffffffffffffffffffffffffffffffffffffffff16630c9fd581826040518263ffffffff1660e01b81526004016135e8919061402e565b5f6040518083038186803b1580156135fe575f5ffd5b505afa158015613610573d5f5f3e3d5ffd5b505050505b50565b80156136a6577f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c73ffffffffffffffffffffffffffffffffffffffff1663a5982885826040518263ffffffff1660e01b8152600401613679919061402e565b5f6040518083038186803b15801561368f575f5ffd5b505afa1580156136a1573d5f5f3e3d5ffd5b505050505b50565b80821461373a577f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c73ffffffffffffffffffffffffffffffffffffffff166398296c5483836040518363ffffffff1660e01b815260040161370d9291906144ed565b5f6040518083038186803b158015613723575f5ffd5b505afa158015613735573d5f5f3e3d5ffd5b505050505b5050565b5f5f82604051602001613751919061454e565b604051602081830303815290604052805190602001205f1c90507f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c73ffffffffffffffffffffffffffffffffffffffff1663ffa18649826040518263ffffffff1660e01b81526004016137c69190614564565b602060405180830381865afa1580156137e1573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061380591906145a7565b91507f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c73ffffffffffffffffffffffffffffffffffffffff1663c657c71883856040518363ffffffff1660e01b815260040161386492919061461a565b5f604051808303815f87803b15801561387b575f5ffd5b505af115801561388d573d5f5f3e3d5ffd5b50505050915091565b611a5a8061464983390190565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6138f5826138cc565b9050919050565b613905816138eb565b82525050565b5f61391683836138fc565b60208301905092915050565b5f602082019050919050565b5f613938826138a3565b61394281856138ad565b935061394d836138bd565b805f5b8381101561397d578151613964888261390b565b975061396f83613922565b925050600181019050613950565b5085935050505092915050565b5f6020820190508181035f8301526139a2818461392e565b905092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f601f19601f8301169050919050565b5f613a3e826139fc565b613a488185613a06565b9350613a58818560208601613a16565b613a6181613a24565b840191505092915050565b5f613a778383613a34565b905092915050565b5f602082019050919050565b5f613a95826139d3565b613a9f81856139dd565b935083602082028501613ab1856139ed565b805f5b85811015613aec5784840389528151613acd8582613a6c565b9450613ad883613a7f565b925060208a01995050600181019050613ab4565b50829750879550505050505092915050565b5f604083015f830151613b135f8601826138fc565b5060208301518482036020860152613b2b8282613a8b565b9150508091505092915050565b5f613b438383613afe565b905092915050565b5f602082019050919050565b5f613b61826139aa565b613b6b81856139b4565b935083602082028501613b7d856139c4565b805f5b85811015613bb85784840389528151613b998582613b38565b9450613ba483613b4b565b925060208a01995050600181019050613b80565b50829750879550505050505092915050565b5f6020820190508181035f830152613be28184613b57565b905092915050565b5f819050919050565b5f613c0d613c08613c03846138cc565b613bea565b6138cc565b9050919050565b5f613c1e82613bf3565b9050919050565b5f613c2f82613c14565b9050919050565b613c3f81613c25565b82525050565b5f602082019050613c585f830184613c36565b92915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f7fffffffff0000000000000000000000000000000000000000000000000000000082169050919050565b613ce481613cb0565b82525050565b5f613cf58383613cdb565b60208301905092915050565b5f602082019050919050565b5f613d1782613c87565b613d218185613c91565b9350613d2c83613ca1565b805f5b83811015613d5c578151613d438882613cea565b9750613d4e83613d01565b925050600181019050613d2f565b5085935050505092915050565b5f604083015f8301518482035f860152613d838282613a34565b91505060208301518482036020860152613d9d8282613d0d565b9150508091505092915050565b5f613db58383613d69565b905092915050565b5f602082019050919050565b5f613dd382613c5e565b613ddd8185613c68565b935083602082028501613def85613c78565b805f5b85811015613e2a5784840389528151613e0b8582613daa565b9450613e1683613dbd565b925060208a01995050600181019050613df2565b50829750879550505050505092915050565b5f6020820190508181035f830152613e548184613dc9565b905092915050565b5f82825260208201905092915050565b5f613e76826139d3565b613e808185613e5c565b935083602082028501613e92856139ed565b805f5b85811015613ecd5784840389528151613eae8582613a6c565b9450613eb983613a7f565b925060208a01995050600181019050613e95565b50829750879550505050505092915050565b5f6020820190508181035f830152613ef78184613e6c565b905092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f604083015f830151613f3d5f8601826138fc565b5060208301518482036020860152613f558282613d0d565b9150508091505092915050565b5f613f6d8383613f28565b905092915050565b5f602082019050919050565b5f613f8b82613eff565b613f958185613f09565b935083602082028501613fa785613f19565b805f5b85811015613fe25784840389528151613fc38582613f62565b9450613fce83613f75565b925060208a01995050600181019050613faa565b50829750879550505050505092915050565b5f6020820190508181035f83015261400c8184613f81565b905092915050565b5f8115159050919050565b61402881614014565b82525050565b5f6020820190506140415f83018461401f565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f819050919050565b5f819050919050565b5f6140cd6140c86140c3846140a1565b613bea565b6140aa565b9050919050565b6140dd816140b3565b82525050565b5f6040820190506140f65f8301856140d4565b8181036020830152614108818461392e565b90509392505050565b5f5ffd5b5f819050919050565b61412781614115565b8114614131575f5ffd5b50565b5f815190506141428161411e565b92915050565b5f6020828403121561415d5761415c614111565b5b5f61416a84828501614134565b91505092915050565b61417c816138eb565b82525050565b5f6020820190506141955f830184614173565b92915050565b6141a481614115565b82525050565b5f6040820190506141bd5f830185614173565b6141ca602083018461419b565b9392505050565b5f81519050919050565b5f82825260208201905092915050565b5f6141f5826141d1565b6141ff81856141db565b935061420f818560208601613a16565b61421881613a24565b840191505092915050565b5f6020820190508181035f83015261423b81846141eb565b905092915050565b5f6040820190506142565f83018561419b565b6142636020830184614173565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f60028204905060018216806142ae57607f821691505b6020821081036142c1576142c061426a565b5b50919050565b6142d081614014565b81146142da575f5ffd5b50565b5f815190506142eb816142c7565b92915050565b5f6020828403121561430657614305614111565b5b5f614313848285016142dd565b91505092915050565b5f819050919050565b5f61433f61433a6143358461431c565b613bea565b6140aa565b9050919050565b61434f81614325565b82525050565b5f6060820190506143685f830186614346565b61437560208301856140d4565b6143826040830184614173565b949350505050565b5f60208201905061439d5f83018461419b565b92915050565b6143ac816140aa565b81146143b6575f5ffd5b50565b5f815190506143c7816143a3565b92915050565b5f602082840312156143e2576143e1614111565b5b5f6143ef848285016143b9565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61442f826140aa565b915061443a836140aa565b9250828203905081811115614452576144516143f8565b5b92915050565b61446181613cb0565b82525050565b5f60208201905061447a5f830184614458565b92915050565b5f819050919050565b5f8160e01b9050919050565b5f6144af6144aa6144a584614480565b614489565b613cb0565b9050919050565b6144bf81614495565b82525050565b5f6020820190506144d85f8301846144b6565b92915050565b6144e7816140aa565b82525050565b5f6040820190506145005f8301856144de565b61450d60208301846144de565b9392505050565b5f81905092915050565b5f614528826139fc565b6145328185614514565b9350614542818560208601613a16565b80840191505092915050565b5f614559828461451e565b915081905092915050565b5f6020820190506145775f8301846144de565b92915050565b614586816138eb565b8114614590575f5ffd5b50565b5f815190506145a18161457d565b92915050565b5f602082840312156145bc576145bb614111565b5b5f6145c984828501614593565b91505092915050565b5f82825260208201905092915050565b5f6145ec826139fc565b6145f681856145d2565b9350614606818560208601613a16565b61460f81613a24565b840191505092915050565b5f60408201905061462d5f830185614173565b818103602083015261463f81846145e2565b9050939250505056fe608060405234801561000f575f5ffd5b50604051611a5a380380611a5a8339818101604052810190610031919061060d565b81600281905550600160025460036100499190610694565b61005391906106d5565b600381905550600354815110156100a65780516003546040517f3a23626800000000000000000000000000000000000000000000000000000000815260040161009d929190610717565b60405180910390fd5b5f5f90505b815181101561010c576100fe7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698383815181106100eb576100ea61073e565b5b602002602001015161019e60201b60201c565b5080806001019150506100ab565b506101577f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e825f815181106101445761014361073e565b5b602002602001015161019e60201b60201c565b507fa2df7830e0bedef7b1116bf547b467b16b50b3bd23146c9e099878d14e89301a6003546002543360405161018f9392919061077a565b60405180910390a150506107af565b5f6101af83836101b760201b60201c565b905092915050565b5f5f6101c9848461020060201b60201c565b905080156101f6576101f48360015f8781526020019081526020015f206102f560201b90919060201c565b505b8091505092915050565b5f610211838361032860201b60201c565b6102eb5760015f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555061028861038b60201b60201c565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a4600190506102ef565b5f90505b92915050565b5f610320835f018373ffffffffffffffffffffffffffffffffffffffff165f1b61039260201b60201c565b905092915050565b5f5f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f33905090565b5f6103a383836103ff60201b60201c565b6103f557825f0182908060018154018082558091505060019003905f5260205f20015f9091909190915055825f0180549050836001015f8481526020019081526020015f2081905550600190506103f9565b5f90505b92915050565b5f5f836001015f8481526020019081526020015f20541415905092915050565b5f604051905090565b5f5ffd5b5f5ffd5b5f819050919050565b61044281610430565b811461044c575f5ffd5b50565b5f8151905061045d81610439565b92915050565b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6104ad82610467565b810181811067ffffffffffffffff821117156104cc576104cb610477565b5b80604052505050565b5f6104de61041f565b90506104ea82826104a4565b919050565b5f67ffffffffffffffff82111561050957610508610477565b5b602082029050602081019050919050565b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6105478261051e565b9050919050565b6105578161053d565b8114610561575f5ffd5b50565b5f815190506105728161054e565b92915050565b5f61058a610585846104ef565b6104d5565b905080838252602082019050602084028301858111156105ad576105ac61051a565b5b835b818110156105d657806105c28882610564565b8452602084019350506020810190506105af565b5050509392505050565b5f82601f8301126105f4576105f3610463565b5b8151610604848260208601610578565b91505092915050565b5f5f6040838503121561062357610622610428565b5b5f6106308582860161044f565b925050602083015167ffffffffffffffff8111156106515761065061042c565b5b61065d858286016105e0565b9150509250929050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61069e82610430565b91506106a983610430565b92508282026106b781610430565b915082820484148315176106ce576106cd610667565b5b5092915050565b5f6106df82610430565b91506106ea83610430565b925082820190508082111561070257610701610667565b5b92915050565b61071181610430565b82525050565b5f60408201905061072a5f830185610708565b6107376020830184610708565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b6107748161053d565b82525050565b5f60608201905061078d5f830186610708565b61079a6020830185610708565b6107a7604083018461076b565b949350505050565b61129e806107bc5f395ff3fe608060405234801561000f575f5ffd5b50600436106100fe575f3560e01c80637f35b56011610095578063a3246ad311610064578063a3246ad3146102c0578063ca15c873146102f0578063d547741f14610320578063fc78b2e81461033c576100fe565b80637f35b560146102245780639010d07c1461024257806391d1485414610272578063a217fddf146102a2576100fe565b806330104c3e116100d157806330104c3e146101ae57806336568abe146101cc57806349f2ada0146101e857806358df0d0114610206576100fe565b806301ffc9a71461010257806313ff6dd514610132578063248a9ca3146101625780632f2ff15d14610192575b5f5ffd5b61011c60048036038101906101179190610e48565b61036c565b6040516101299190610e8d565b60405180910390f35b61014c60048036038101906101479190610f00565b6103e5565b6040516101599190610e8d565b60405180910390f35b61017c60048036038101906101779190610f5e565b610428565b6040516101899190610f98565b60405180910390f35b6101ac60048036038101906101a79190610fb1565b610444565b005b6101b6610486565b6040516101c39190610f98565b60405180910390f35b6101e660048036038101906101e19190610fb1565b6104aa565b005b6101f06104c0565b6040516101fd9190610f98565b60405180910390f35b61020e6104e4565b60405161021b9190610f98565b60405180910390f35b61022c610508565b6040516102399190610f98565b60405180910390f35b61025c60048036038101906102579190611022565b61052c565b604051610269919061106f565b60405180910390f35b61028c60048036038101906102879190610fb1565b610558565b6040516102999190610e8d565b60405180910390f35b6102aa6105bb565b6040516102b79190610f98565b60405180910390f35b6102da60048036038101906102d59190610f5e565b6105c1565b6040516102e7919061113f565b60405180910390f35b61030a60048036038101906103059190610f5e565b6105e3565b604051610317919061116e565b60405180910390f35b61033a60048036038101906103359190610fb1565b610604565b005b61035660048036038101906103519190610f00565b610646565b6040516103639190610e8d565b60405180910390f35b5f7f07effe0d000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191614806103de57506103dd82610678565b5b9050919050565b5f6103ef82610646565b801561042157506104207f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e83610558565b5b9050919050565b5f5f5f8381526020019081526020015f20600101549050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61046e816106f1565b610476610705565b6104808383610707565b50505050565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46981565b6104b2610705565b6104bc828261071a565b5050565b7fd68e3e5e367fee47ce11a5dab404596980e158eb9069330a8f775de7dc6bb64d81565b7f601a28e6abeea52b0ba15b54f6d15d789fd3e8c0080c042a009a9964f0e18f1c81565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e81565b5f6105508260015f8681526020019081526020015f2061079590919063ffffffff16565b905092915050565b5f5f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f5f1b81565b60606105dc60015f8481526020019081526020015f206107ac565b9050919050565b5f6105fd60015f8481526020019081526020015f206107cb565b9050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61062e816106f1565b610636610705565b61064083836107de565b50505050565b5f6106717fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46983610558565b9050919050565b5f7f5a05180f000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191614806106ea57506106e9826107f1565b5b9050919050565b610702816106fd61086a565b610871565b50565b565b5f61071283836108c2565b905092915050565b61072261086a565b73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614610786576040517f6697b23200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61079082826107de565b505050565b5f6107a2835f0183610905565b5f1c905092915050565b60605f6107ba835f0161092c565b905060608190508092505050919050565b5f6107d7825f01610985565b9050919050565b5f6107e98383610994565b905092915050565b5f7f7965db0b000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191614806108635750610862826109d7565b5b9050919050565b5f33905090565b61087b8282610558565b6108be5780826040517fe2517d3f0000000000000000000000000000000000000000000000000000000081526004016108b5929190611187565b60405180910390fd5b5050565b5f5f6108ce8484610a40565b905080156108fb576108f98360015f8781526020019081526020015f20610b2990919063ffffffff16565b505b8091505092915050565b5f825f01828154811061091b5761091a6111ae565b5b905f5260205f200154905092915050565b6060815f0180548060200260200160405190810160405280929190818152602001828054801561097957602002820191905f5260205f20905b815481526020019060010190808311610965575b50505050509050919050565b5f815f01805490509050919050565b5f5f6109a08484610b56565b905080156109cd576109cb8360015f8781526020019081526020015f20610c3f90919063ffffffff16565b505b8091505092915050565b5f7f01ffc9a7000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916149050919050565b5f610a4b8383610558565b610b1f5760015f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550610abc61086a565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a460019050610b23565b5f90505b92915050565b5f610b4e835f018373ffffffffffffffffffffffffffffffffffffffff165f1b610c6c565b905092915050565b5f610b618383610558565b15610c35575f5f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550610bd261086a565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b60405160405180910390a460019050610c39565b5f90505b92915050565b5f610c64835f018373ffffffffffffffffffffffffffffffffffffffff165f1b610cd3565b905092915050565b5f610c778383610dcf565b610cc957825f0182908060018154018082558091505060019003905f5260205f20015f9091909190915055825f0180549050836001015f8481526020019081526020015f208190555060019050610ccd565b5f90505b92915050565b5f5f836001015f8481526020019081526020015f205490505f8114610dc4575f600182610d009190611208565b90505f6001865f0180549050610d169190611208565b9050808214610d7c575f865f018281548110610d3557610d346111ae565b5b905f5260205f200154905080875f018481548110610d5657610d556111ae565b5b905f5260205f20018190555083876001015f8381526020019081526020015f2081905550505b855f01805480610d8f57610d8e61123b565b5b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f905560019350505050610dc9565b5f9150505b92915050565b5f5f836001015f8481526020019081526020015f20541415905092915050565b5f5ffd5b5f7fffffffff0000000000000000000000000000000000000000000000000000000082169050919050565b610e2781610df3565b8114610e31575f5ffd5b50565b5f81359050610e4281610e1e565b92915050565b5f60208284031215610e5d57610e5c610def565b5b5f610e6a84828501610e34565b91505092915050565b5f8115159050919050565b610e8781610e73565b82525050565b5f602082019050610ea05f830184610e7e565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610ecf82610ea6565b9050919050565b610edf81610ec5565b8114610ee9575f5ffd5b50565b5f81359050610efa81610ed6565b92915050565b5f60208284031215610f1557610f14610def565b5b5f610f2284828501610eec565b91505092915050565b5f819050919050565b610f3d81610f2b565b8114610f47575f5ffd5b50565b5f81359050610f5881610f34565b92915050565b5f60208284031215610f7357610f72610def565b5b5f610f8084828501610f4a565b91505092915050565b610f9281610f2b565b82525050565b5f602082019050610fab5f830184610f89565b92915050565b5f5f60408385031215610fc757610fc6610def565b5b5f610fd485828601610f4a565b9250506020610fe585828601610eec565b9150509250929050565b5f819050919050565b61100181610fef565b811461100b575f5ffd5b50565b5f8135905061101c81610ff8565b92915050565b5f5f6040838503121561103857611037610def565b5b5f61104585828601610f4a565b92505060206110568582860161100e565b9150509250929050565b61106981610ec5565b82525050565b5f6020820190506110825f830184611060565b92915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b6110ba81610ec5565b82525050565b5f6110cb83836110b1565b60208301905092915050565b5f602082019050919050565b5f6110ed82611088565b6110f78185611092565b9350611102836110a2565b805f5b8381101561113257815161111988826110c0565b9750611124836110d7565b925050600181019050611105565b5085935050505092915050565b5f6020820190508181035f83015261115781846110e3565b905092915050565b61116881610fef565b82525050565b5f6020820190506111815f83018461115f565b92915050565b5f60408201905061119a5f830185611060565b6111a76020830184610f89565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61121282610fef565b915061121d83610fef565b9250828203905081811115611235576112346111db565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffdfea26469706673582212203fe44ace55c90e3cd17d0c6780b937fac6f484e546ef974c552a23fa0924a6ae64736f6c63430008210033a264697066735822122078d8bd34b57a6b9a98061819254f2f432cde80ec5eac8a3b6d79d28214b50e5064736f6c63430008210033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x01`\x0C_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01`\x1F_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\0|`@Q\x80`@\x01`@R\x80`\n\x81R` \x01\x7FDESIGNATED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x02U` \x1B` \x1CV[` _a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPa\0\xFF`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7FPARTY1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x02U` \x1B` \x1CV[`!_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPa\x01\x82`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7FPARTY2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x02U` \x1B` \x1CV[`\"_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPa\x02\x05`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7FPARTY3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x02U` \x1B` \x1CV[`#_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP4\x80\x15a\x02OW__\xFD[Pa\x05~V[_a\x02e\x82a\x02o` \x1B` \x1CV[P\x80\x91PP\x91\x90PV[__\x82`@Q` \x01a\x02\x82\x91\x90a\x04\x19V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1C\x90P\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xFF\xA1\x86I\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xF7\x91\x90a\x04GV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x12W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x036\x91\x90a\x04\xBEV[\x91P\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC6W\xC7\x18\x83\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\x95\x92\x91\x90a\x05PV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x03\xACW__\xFD[PZ\xF1\x15\x80\x15a\x03\xBEW=__>=_\xFD[PPPP\x91P\x91V[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x03\xF3\x82a\x03\xC7V[a\x03\xFD\x81\x85a\x03\xD1V[\x93Pa\x04\r\x81\x85` \x86\x01a\x03\xDBV[\x80\x84\x01\x91PP\x92\x91PPV[_a\x04$\x82\x84a\x03\xE9V[\x91P\x81\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x04A\x81a\x04/V[\x82RPPV[_` \x82\x01\x90Pa\x04Z_\x83\x01\x84a\x048V[\x92\x91PPV[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x04\x8D\x82a\x04dV[\x90P\x91\x90PV[a\x04\x9D\x81a\x04\x83V[\x81\x14a\x04\xA7W__\xFD[PV[_\x81Q\x90Pa\x04\xB8\x81a\x04\x94V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x04\xD3Wa\x04\xD2a\x04`V[[_a\x04\xE0\x84\x82\x85\x01a\x04\xAAV[\x91PP\x92\x91PPV[a\x04\xF2\x81a\x04\x83V[\x82RPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a\x05\"\x82a\x03\xC7V[a\x05,\x81\x85a\x04\xF8V[\x93Pa\x05<\x81\x85` \x86\x01a\x03\xDBV[a\x05E\x81a\x05\x08V[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90Pa\x05c_\x83\x01\x85a\x04\xE9V[\x81\x81\x03` \x83\x01Ra\x05u\x81\x84a\x05\x18V[\x90P\x93\x92PPPV[a`\xD8\x80a\x05\x8B_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\xA7W_5`\xE0\x1C\x80c\x85\"l\x81\x11a\0\xF7W\x80c\xCD\t\x9Ej\x11a\0\x95W\x80c\xE0PQ\x99\x11a\0oW\x80c\xE0PQ\x99\x14a\x03mW\x80c\xE2\x0C\x9Fq\x14a\x03wW\x80c\xE4\x9E\x9F\xF5\x14a\x03\x95W\x80c\xFAv&\xD4\x14a\x03\x9FWa\x01\xA7V[\x80c\xCD\t\x9Ej\x14a\x03OW\x80c\xD3\xFE\x9E}\x14a\x03YW\x80c\xD7\xC3\xE52\x14a\x03cWa\x01\xA7V[\x80c\xB0FO\xDC\x11a\0\xD1W\x80c\xB0FO\xDC\x14a\x02\xEBW\x80c\xB5P\x8A\xA9\x14a\x03\tW\x80c\xBA,\xBD5\x14a\x03'W\x80c\xBAAO\xA6\x14a\x031Wa\x01\xA7V[\x80c\x85\"l\x81\x14a\x02\xA5W\x80c\x91j\x17\xC6\x14a\x02\xC3W\x80c\x9B\xC8\xA6\x19\x14a\x02\xE1Wa\x01\xA7V[\x80c>^<#\x11a\x01dW\x80cT\xB7\xC3\x9B\x11a\x01>W\x80cT\xB7\xC3\x9B\x14a\x02UW\x80c\\\x97+\xE5\x14a\x02sW\x80ce\xC9\xF4\x85\x14a\x02}W\x80cf\xD9\xA9\xA0\x14a\x02\x87Wa\x01\xA7V[\x80c>^<#\x14a\x02\x0FW\x80c?r\x86\xF4\x14a\x02-W\x80cHK?6\x14a\x02KWa\x01\xA7V[\x80c\n\x92T\xE4\x14a\x01\xABW\x80c\x1E\xD7\x83\x1C\x14a\x01\xB5W\x80c \nV\xCE\x14a\x01\xD3W\x80c*\xDE8\x80\x14a\x01\xDDW\x80c7M\x1F\xD5\x14a\x01\xFBW\x80c:^O#\x14a\x02\x05W[__\xFD[a\x01\xB3a\x03\xBDV[\0[a\x01\xBDa\x07jV[`@Qa\x01\xCA\x91\x90a9\x8AV[`@Q\x80\x91\x03\x90\xF3[a\x01\xDBa\x07\xF5V[\0[a\x01\xE5a\ntV[`@Qa\x01\xF2\x91\x90a;\xCAV[`@Q\x80\x91\x03\x90\xF3[a\x02\x03a\x0B\xF8V[\0[a\x02\ra\x122V[\0[a\x02\x17a\x13\xD6V[`@Qa\x02$\x91\x90a9\x8AV[`@Q\x80\x91\x03\x90\xF3[a\x025a\x14aV[`@Qa\x02B\x91\x90a9\x8AV[`@Q\x80\x91\x03\x90\xF3[a\x02Sa\x14\xECV[\0[a\x02]a\x16\xFDV[`@Qa\x02j\x91\x90a<EV[`@Q\x80\x91\x03\x90\xF3[a\x02{a\x17#V[\0[a\x02\x85a\x194V[\0[a\x02\x8Fa\x1A\xEDV[`@Qa\x02\x9C\x91\x90a><V[`@Q\x80\x91\x03\x90\xF3[a\x02\xADa\x1CoV[`@Qa\x02\xBA\x91\x90a>\xDFV[`@Q\x80\x91\x03\x90\xF3[a\x02\xCBa\x1DCV[`@Qa\x02\xD8\x91\x90a?\xF4V[`@Q\x80\x91\x03\x90\xF3[a\x02\xE9a\x1E\x8AV[\0[a\x02\xF3a \xF1V[`@Qa\x03\0\x91\x90a?\xF4V[`@Q\x80\x91\x03\x90\xF3[a\x03\x11a\"8V[`@Qa\x03\x1E\x91\x90a>\xDFV[`@Q\x80\x91\x03\x90\xF3[a\x03/a#\x0CV[\0[a\x039a%\xFCV[`@Qa\x03F\x91\x90a@.V[`@Q\x80\x91\x03\x90\xF3[a\x03Wa'\x03V[\0[a\x03aa)\x92V[\0[a\x03ka,-V[\0[a\x03ua.YV[\0[a\x03\x7Fa0\xB7V[`@Qa\x03\x8C\x91\x90a9\x8AV[`@Q\x80\x91\x03\x90\xF3[a\x03\x9Da1BV[\0[a\x03\xA7a5bV[`@Qa\x03\xB4\x91\x90a@.V[`@Q\x80\x91\x03\x90\xF3[_`\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xD9Wa\x03\xD8a@GV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x04\x07W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P` _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x81Q\x81\x10a\x04?Wa\x04>a@tV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x01\x81Q\x81\x10a\x04\xAFWa\x04\xAEa@tV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`\"_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x02\x81Q\x81\x10a\x05\x1FWa\x05\x1Ea@tV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`#_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x03\x81Q\x81\x10a\x05\x8FWa\x05\x8Ea@tV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`\x01\x81`@Qa\x05\xD8\x90a8\x96V[a\x05\xE3\x92\x91\x90a@\xE3V[`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x05\xFCW=__>=_\xFD[P`\x1F`\x01a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c0\x10L>`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xA8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xCC\x91\x90aAHV[`$\x81\x90UP`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x7F5\xB5``@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07=W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07a\x91\x90aAHV[`%\x81\x90UPPV[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07\xEBW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x07\xA2W[PPPPP\x90P\x90V[_a\x084`@Q\x80`@\x01`@R\x80`\t\x81R` \x01\x7FNEW_PARTY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa5tV[\x90Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCAf\x9F\xA7`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\xA4\x91\x90aA\x82V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08\xBBW__\xFD[PZ\xF1\x15\x80\x15a\x08\xCDW=__>=_\xFD[PPPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF2\x8D\xCE\xB3c\xE2Q}?`\xE0\x1B`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`%T`@Q`$\x01a\t?\x92\x91\x90aA\xAAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\xB8\x91\x90aB#V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\t\xCFW__\xFD[PZ\xF1\x15\x80\x15a\t\xE1W=__>=_\xFD[PPPP`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c//\xF1]`$T\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\nD\x92\x91\x90aBCV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\n[W__\xFD[PZ\xF1\x15\x80\x15a\nmW=__>=_\xFD[PPPPPV[```\x1E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0B\xEFW\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0B\xD8W\x83\x82\x90_R` _ \x01\x80Ta\x0BM\x90aB\x97V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0By\x90aB\x97V[\x80\x15a\x0B\xC4W\x80`\x1F\x10a\x0B\x9BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xC4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xA7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x0B0V[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\n\x97V[PPPP\x90P\x90V[a\x0C\xBF`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xD1HT`$T` _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C{\x92\x91\x90aBCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x96W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xBA\x91\x90aB\xF1V[a5\x88V[a\r\x86`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xD1HT`$T`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\rB\x92\x91\x90aBCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r]W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x81\x91\x90aB\xF1V[a5\x88V[a\x0EM`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xD1HT`$T`\"_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\t\x92\x91\x90aBCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E$W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EH\x91\x90aB\xF1V[a5\x88V[a\x0F\x14`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xD1HT`$T`#_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xD0\x92\x91\x90aBCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xEBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x0F\x91\x90aB\xF1V[a5\x88V[a\x0F\xDB`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xD1HT`%T` _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\x97\x92\x91\x90aBCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xB2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xD6\x91\x90aB\xF1V[a5\x88V[a\x10\xA2`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xD1HT`%T`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10^\x92\x91\x90aBCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10yW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x9D\x91\x90aB\xF1V[a6\x18V[a\x11i`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xD1HT`%T`\"_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11%\x92\x91\x90aBCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11@W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11d\x91\x90aB\xF1V[a6\x18V[a\x120`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xD1HT`%T`#_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\xEC\x92\x91\x90aBCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x07W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12+\x91\x90aB\xF1V[a6\x18V[V[a\x12\xF5`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xFCx\xB2\xE8`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\xB1\x91\x90aA\x82V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xCCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xF0\x91\x90aB\xF1V[a5\x88V[a\x13\xD4`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xFCx\xB2\xE8a\x13t`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01\x7FSTRANGER\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa5tV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\x90\x91\x90aA\x82V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xABW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xCF\x91\x90aB\xF1V[a6\x18V[V[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x14WW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x14\x0EW[PPPPP\x90P\x90V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x14\xE2W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x14\x99W[PPPPP\x90P\x90V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCAf\x9F\xA7`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15Z\x91\x90aA\x82V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x15qW__\xFD[PZ\xF1\x15\x80\x15a\x15\x83W=__>=_\xFD[PPPP`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c6V\x8A\xBE`$T`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\x07\x92\x91\x90aBCV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x16\x1EW__\xFD[PZ\xF1\x15\x80\x15a\x160W=__>=_\xFD[PPPPa\x16\xFB`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xD1HT`$T`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\xB7\x92\x91\x90aBCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xD2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xF6\x91\x90aB\xF1V[a6\x18V[V[`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCAf\x9F\xA7` _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\x91\x91\x90aA\x82V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x17\xA8W__\xFD[PZ\xF1\x15\x80\x15a\x17\xBAW=__>=_\xFD[PPPP`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD5Gt\x1F`$T`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18>\x92\x91\x90aBCV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x18UW__\xFD[PZ\xF1\x15\x80\x15a\x18gW=__>=_\xFD[PPPPa\x192`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xD1HT`$T`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\xEE\x92\x91\x90aBCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\tW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19-\x91\x90aB\xF1V[a6\x18V[V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCAf\x9F\xA7`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\xA2\x91\x90aA\x82V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x19\xB9W__\xFD[PZ\xF1\x15\x80\x15a\x19\xCBW=__>=_\xFD[PPPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF4\x84H\x14`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1A(W__\xFD[PZ\xF1\x15\x80\x15a\x1A:W=__>=_\xFD[PPPP`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c6V\x8A\xBE`$T`\"_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A\xBE\x92\x91\x90aBCV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1A\xD5W__\xFD[PZ\xF1\x15\x80\x15a\x1A\xE7W=__>=_\xFD[PPPPV[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1CfW\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x1B@\x90aB\x97V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1Bl\x90aB\x97V[\x80\x15a\x1B\xB7W\x80`\x1F\x10a\x1B\x8EWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1B\xB7V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1B\x9AW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1CNW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x1B\xFBW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1B\x10V[PPPP\x90P\x90V[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1D:W\x83\x82\x90_R` _ \x01\x80Ta\x1C\xAF\x90aB\x97V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1C\xDB\x90aB\x97V[\x80\x15a\x1D&W\x80`\x1F\x10a\x1C\xFDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1D&V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1D\tW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x1C\x92V[PPPP\x90P\x90V[```\x1D\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1E\x81W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1EiW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x1E\x16W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1DfV[PPPP\x90P\x90V[a\x1FM`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x13\xFFm\xD5` _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1F\t\x91\x90aA\x82V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F$W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1FH\x91\x90aB\xF1V[a5\x88V[a \x10`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x13\xFFm\xD5`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1F\xCC\x91\x90aA\x82V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xE7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \x0B\x91\x90aB\xF1V[a6\x18V[a \xEF`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x13\xFFm\xD5a \x8F`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01\x7FSTRANGER\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa5tV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a \xAB\x91\x90aA\x82V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xC6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xEA\x91\x90aB\xF1V[a6\x18V[V[```\x1C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\"/W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\"\x17W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a!\xC4W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a!\x14V[PPPP\x90P\x90V[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a#\x03W\x83\x82\x90_R` _ \x01\x80Ta\"x\x90aB\x97V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\"\xA4\x90aB\x97V[\x80\x15a\"\xEFW\x80`\x1F\x10a\"\xC6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\"\xEFV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\"\xD2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\"[V[PPPP\x90P\x90V[_`\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#(Wa#'a@GV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a#VW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P` _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x81Q\x81\x10a#\x8EWa#\x8Da@tV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x01\x81Q\x81\x10a#\xFEWa#\xFDa@tV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`\"_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x02\x81Q\x81\x10a$nWa$ma@tV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`#_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x03\x81Q\x81\x10a$\xDEWa$\xDDa@tV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cD\x0E\xD1\r`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a%qW__\xFD[PZ\xF1\x15\x80\x15a%\x83W=__>=_\xFD[PPPP\x7F\xA2\xDFx0\xE0\xBE\xDE\xF7\xB1\x11k\xF5G\xB4g\xB1kP\xB3\xBD#\x14l\x9E\t\x98x\xD1N\x890\x1A`\x04`\x010`@Qa%\xBC\x93\x92\x91\x90aCUV[`@Q\x80\x91\x03\x90\xA1`\x01\x81`@Qa%\xD3\x90a8\x96V[a%\xDE\x92\x91\x90a@\xE3V[`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a%\xF7W=__>=_\xFD[PPPV[_`\x08_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a&\x1AW`\x01\x90Pa'\0V[__\x1B\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cf\x7F\x9Dp\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1C\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a&\xBC\x92\x91\x90aA\xAAV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\xD7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xFB\x91\x90aAHV[\x14\x15\x90P[\x90V[_`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCA\x15\xC8s`$T`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a'a\x91\x90aC\x8AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'|W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\xA0\x91\x90aC\xCDV[\x90Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCAf\x9F\xA7` _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a(\x10\x91\x90aA\x82V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a('W__\xFD[PZ\xF1\x15\x80\x15a(9W=__>=_\xFD[PPPP`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c//\xF1]`$T`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a(\xBD\x92\x91\x90aBCV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a(\xD4W__\xFD[PZ\xF1\x15\x80\x15a(\xE6W=__>=_\xFD[PPPPa)\x8F`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCA\x15\xC8s`$T`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a)J\x91\x90aC\x8AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)eW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\x89\x91\x90aC\xCDV[\x82a6\xA9V[PV[_`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCA\x15\xC8s`$T`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a)\xF0\x91\x90aC\x8AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\x0BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*/\x91\x90aC\xCDV[\x90Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCAf\x9F\xA7` _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a*\x9F\x91\x90aA\x82V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a*\xB6W__\xFD[PZ\xF1\x15\x80\x15a*\xC8W=__>=_\xFD[PPPP`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD5Gt\x1F`$T`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+L\x92\x91\x90aBCV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a+cW__\xFD[PZ\xF1\x15\x80\x15a+uW=__>=_\xFD[PPPPa,*`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCA\x15\xC8s`$T`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+\xD9\x91\x90aC\x8AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\xF4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\x18\x91\x90aC\xCDV[`\x01\x83a,%\x91\x90aD%V[a6\xA9V[PV[a,\xEF`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x01\xFF\xC9\xA7\x7F\x07\xEF\xFE\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\xAB\x91\x90aDgV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xC6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xEA\x91\x90aB\xF1V[a5\x88V[a-\xB1`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x01\xFF\xC9\xA7\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a-m\x91\x90aDgV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\x88W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\xAC\x91\x90aB\xF1V[a5\x88V[a.W`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x01\xFF\xC9\xA7c\xDE\xAD\xBE\xEF`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a.\x13\x91\x90aD\xC5V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a..W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.R\x91\x90aB\xF1V[a6\x18V[V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCAf\x9F\xA7`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a.\xC7\x91\x90aA\x82V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a.\xDEW__\xFD[PZ\xF1\x15\x80\x15a.\xF0W=__>=_\xFD[PPPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF2\x8D\xCE\xB3c\xE2Q}?`\xE0\x1B`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`%T`@Q`$\x01a/b\x92\x91\x90aA\xAAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a/\xDB\x91\x90aB#V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a/\xF2W__\xFD[PZ\xF1\x15\x80\x15a0\x04W=__>=_\xFD[PPPP`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD5Gt\x1F`$T`\"_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a0\x88\x92\x91\x90aBCV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a0\x9FW__\xFD[PZ\xF1\x15\x80\x15a0\xB1W=__>=_\xFD[PPPPV[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a18W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a0\xEFW[PPPPP\x90P\x90V[_a1\x81`@Q\x80`@\x01`@R\x80`\t\x81R` \x01\x7FNEW_PARTY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa5tV[\x90Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCAf\x9F\xA7` _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a1\xF1\x91\x90aA\x82V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a2\x08W__\xFD[PZ\xF1\x15\x80\x15a2\x1AW=__>=_\xFD[PPPP`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c//\xF1]`$T\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a2}\x92\x91\x90aBCV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a2\x94W__\xFD[PZ\xF1\x15\x80\x15a2\xA6W=__>=_\xFD[PPPPa3P`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xD1HT`$T\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a3\x0C\x92\x91\x90aBCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3'W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3K\x91\x90aB\xF1V[a5\x88V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCAf\x9F\xA7` _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a3\xBE\x91\x90aA\x82V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a3\xD5W__\xFD[PZ\xF1\x15\x80\x15a3\xE7W=__>=_\xFD[PPPP`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c//\xF1]`%T`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a4k\x92\x91\x90aBCV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a4\x82W__\xFD[PZ\xF1\x15\x80\x15a4\x94W=__>=_\xFD[PPPPa5_`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xD1HT`%T`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a5\x1B\x92\x91\x90aBCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a56W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5Z\x91\x90aB\xF1V[a5\x88V[PV[`\x1F_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[_a5~\x82a7>V[P\x80\x91PP\x91\x90PV[\x80a6\x15W\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x0C\x9F\xD5\x81\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a5\xE8\x91\x90a@.V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a5\xFEW__\xFD[PZ\xFA\x15\x80\x15a6\x10W=__>=_\xFD[PPPP[PV[\x80\x15a6\xA6W\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA5\x98(\x85\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a6y\x91\x90a@.V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a6\x8FW__\xFD[PZ\xFA\x15\x80\x15a6\xA1W=__>=_\xFD[PPPP[PV[\x80\x82\x14a7:W\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x98)lT\x83\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a7\r\x92\x91\x90aD\xEDV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a7#W__\xFD[PZ\xFA\x15\x80\x15a75W=__>=_\xFD[PPPP[PPV[__\x82`@Q` \x01a7Q\x91\x90aENV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1C\x90P\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xFF\xA1\x86I\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a7\xC6\x91\x90aEdV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\xE1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\x05\x91\x90aE\xA7V[\x91P\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC6W\xC7\x18\x83\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a8d\x92\x91\x90aF\x1AV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a8{W__\xFD[PZ\xF1\x15\x80\x15a8\x8DW=__>=_\xFD[PPPP\x91P\x91V[a\x1AZ\x80aFI\x839\x01\x90V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a8\xF5\x82a8\xCCV[\x90P\x91\x90PV[a9\x05\x81a8\xEBV[\x82RPPV[_a9\x16\x83\x83a8\xFCV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a98\x82a8\xA3V[a9B\x81\x85a8\xADV[\x93Pa9M\x83a8\xBDV[\x80_[\x83\x81\x10\x15a9}W\x81Qa9d\x88\x82a9\x0BV[\x97Pa9o\x83a9\"V[\x92PP`\x01\x81\x01\x90Pa9PV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra9\xA2\x81\x84a9.V[\x90P\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a:>\x82a9\xFCV[a:H\x81\x85a:\x06V[\x93Pa:X\x81\x85` \x86\x01a:\x16V[a:a\x81a:$V[\x84\x01\x91PP\x92\x91PPV[_a:w\x83\x83a:4V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a:\x95\x82a9\xD3V[a:\x9F\x81\x85a9\xDDV[\x93P\x83` \x82\x02\x85\x01a:\xB1\x85a9\xEDV[\x80_[\x85\x81\x10\x15a:\xECW\x84\x84\x03\x89R\x81Qa:\xCD\x85\x82a:lV[\x94Pa:\xD8\x83a:\x7FV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa:\xB4V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`@\x83\x01_\x83\x01Qa;\x13_\x86\x01\x82a8\xFCV[P` \x83\x01Q\x84\x82\x03` \x86\x01Ra;+\x82\x82a:\x8BV[\x91PP\x80\x91PP\x92\x91PPV[_a;C\x83\x83a:\xFEV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a;a\x82a9\xAAV[a;k\x81\x85a9\xB4V[\x93P\x83` \x82\x02\x85\x01a;}\x85a9\xC4V[\x80_[\x85\x81\x10\x15a;\xB8W\x84\x84\x03\x89R\x81Qa;\x99\x85\x82a;8V[\x94Pa;\xA4\x83a;KV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa;\x80V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra;\xE2\x81\x84a;WV[\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[_a<\ra<\x08a<\x03\x84a8\xCCV[a;\xEAV[a8\xCCV[\x90P\x91\x90PV[_a<\x1E\x82a;\xF3V[\x90P\x91\x90PV[_a</\x82a<\x14V[\x90P\x91\x90PV[a<?\x81a<%V[\x82RPPV[_` \x82\x01\x90Pa<X_\x83\x01\x84a<6V[\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a<\xE4\x81a<\xB0V[\x82RPPV[_a<\xF5\x83\x83a<\xDBV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a=\x17\x82a<\x87V[a=!\x81\x85a<\x91V[\x93Pa=,\x83a<\xA1V[\x80_[\x83\x81\x10\x15a=\\W\x81Qa=C\x88\x82a<\xEAV[\x97Pa=N\x83a=\x01V[\x92PP`\x01\x81\x01\x90Pa=/V[P\x85\x93PPPP\x92\x91PPV[_`@\x83\x01_\x83\x01Q\x84\x82\x03_\x86\x01Ra=\x83\x82\x82a:4V[\x91PP` \x83\x01Q\x84\x82\x03` \x86\x01Ra=\x9D\x82\x82a=\rV[\x91PP\x80\x91PP\x92\x91PPV[_a=\xB5\x83\x83a=iV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a=\xD3\x82a<^V[a=\xDD\x81\x85a<hV[\x93P\x83` \x82\x02\x85\x01a=\xEF\x85a<xV[\x80_[\x85\x81\x10\x15a>*W\x84\x84\x03\x89R\x81Qa>\x0B\x85\x82a=\xAAV[\x94Pa>\x16\x83a=\xBDV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa=\xF2V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra>T\x81\x84a=\xC9V[\x90P\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a>v\x82a9\xD3V[a>\x80\x81\x85a>\\V[\x93P\x83` \x82\x02\x85\x01a>\x92\x85a9\xEDV[\x80_[\x85\x81\x10\x15a>\xCDW\x84\x84\x03\x89R\x81Qa>\xAE\x85\x82a:lV[\x94Pa>\xB9\x83a:\x7FV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa>\x95V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra>\xF7\x81\x84a>lV[\x90P\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_`@\x83\x01_\x83\x01Qa?=_\x86\x01\x82a8\xFCV[P` \x83\x01Q\x84\x82\x03` \x86\x01Ra?U\x82\x82a=\rV[\x91PP\x80\x91PP\x92\x91PPV[_a?m\x83\x83a?(V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a?\x8B\x82a>\xFFV[a?\x95\x81\x85a?\tV[\x93P\x83` \x82\x02\x85\x01a?\xA7\x85a?\x19V[\x80_[\x85\x81\x10\x15a?\xE2W\x84\x84\x03\x89R\x81Qa?\xC3\x85\x82a?bV[\x94Pa?\xCE\x83a?uV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa?\xAAV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra@\x0C\x81\x84a?\x81V[\x90P\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a@(\x81a@\x14V[\x82RPPV[_` \x82\x01\x90Pa@A_\x83\x01\x84a@\x1FV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a@\xCDa@\xC8a@\xC3\x84a@\xA1V[a;\xEAV[a@\xAAV[\x90P\x91\x90PV[a@\xDD\x81a@\xB3V[\x82RPPV[_`@\x82\x01\x90Pa@\xF6_\x83\x01\x85a@\xD4V[\x81\x81\x03` \x83\x01RaA\x08\x81\x84a9.V[\x90P\x93\x92PPPV[__\xFD[_\x81\x90P\x91\x90PV[aA'\x81aA\x15V[\x81\x14aA1W__\xFD[PV[_\x81Q\x90PaAB\x81aA\x1EV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aA]WaA\\aA\x11V[[_aAj\x84\x82\x85\x01aA4V[\x91PP\x92\x91PPV[aA|\x81a8\xEBV[\x82RPPV[_` \x82\x01\x90PaA\x95_\x83\x01\x84aAsV[\x92\x91PPV[aA\xA4\x81aA\x15V[\x82RPPV[_`@\x82\x01\x90PaA\xBD_\x83\x01\x85aAsV[aA\xCA` \x83\x01\x84aA\x9BV[\x93\x92PPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aA\xF5\x82aA\xD1V[aA\xFF\x81\x85aA\xDBV[\x93PaB\x0F\x81\x85` \x86\x01a:\x16V[aB\x18\x81a:$V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaB;\x81\x84aA\xEBV[\x90P\x92\x91PPV[_`@\x82\x01\x90PaBV_\x83\x01\x85aA\x9BV[aBc` \x83\x01\x84aAsV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80aB\xAEW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aB\xC1WaB\xC0aBjV[[P\x91\x90PV[aB\xD0\x81a@\x14V[\x81\x14aB\xDAW__\xFD[PV[_\x81Q\x90PaB\xEB\x81aB\xC7V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aC\x06WaC\x05aA\x11V[[_aC\x13\x84\x82\x85\x01aB\xDDV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_aC?aC:aC5\x84aC\x1CV[a;\xEAV[a@\xAAV[\x90P\x91\x90PV[aCO\x81aC%V[\x82RPPV[_``\x82\x01\x90PaCh_\x83\x01\x86aCFV[aCu` \x83\x01\x85a@\xD4V[aC\x82`@\x83\x01\x84aAsV[\x94\x93PPPPV[_` \x82\x01\x90PaC\x9D_\x83\x01\x84aA\x9BV[\x92\x91PPV[aC\xAC\x81a@\xAAV[\x81\x14aC\xB6W__\xFD[PV[_\x81Q\x90PaC\xC7\x81aC\xA3V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aC\xE2WaC\xE1aA\x11V[[_aC\xEF\x84\x82\x85\x01aC\xB9V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_aD/\x82a@\xAAV[\x91PaD:\x83a@\xAAV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15aDRWaDQaC\xF8V[[\x92\x91PPV[aDa\x81a<\xB0V[\x82RPPV[_` \x82\x01\x90PaDz_\x83\x01\x84aDXV[\x92\x91PPV[_\x81\x90P\x91\x90PV[_\x81`\xE0\x1B\x90P\x91\x90PV[_aD\xAFaD\xAAaD\xA5\x84aD\x80V[aD\x89V[a<\xB0V[\x90P\x91\x90PV[aD\xBF\x81aD\x95V[\x82RPPV[_` \x82\x01\x90PaD\xD8_\x83\x01\x84aD\xB6V[\x92\x91PPV[aD\xE7\x81a@\xAAV[\x82RPPV[_`@\x82\x01\x90PaE\0_\x83\x01\x85aD\xDEV[aE\r` \x83\x01\x84aD\xDEV[\x93\x92PPPV[_\x81\x90P\x92\x91PPV[_aE(\x82a9\xFCV[aE2\x81\x85aE\x14V[\x93PaEB\x81\x85` \x86\x01a:\x16V[\x80\x84\x01\x91PP\x92\x91PPV[_aEY\x82\x84aE\x1EV[\x91P\x81\x90P\x92\x91PPV[_` \x82\x01\x90PaEw_\x83\x01\x84aD\xDEV[\x92\x91PPV[aE\x86\x81a8\xEBV[\x81\x14aE\x90W__\xFD[PV[_\x81Q\x90PaE\xA1\x81aE}V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aE\xBCWaE\xBBaA\x11V[[_aE\xC9\x84\x82\x85\x01aE\x93V[\x91PP\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aE\xEC\x82a9\xFCV[aE\xF6\x81\x85aE\xD2V[\x93PaF\x06\x81\x85` \x86\x01a:\x16V[aF\x0F\x81a:$V[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90PaF-_\x83\x01\x85aAsV[\x81\x81\x03` \x83\x01RaF?\x81\x84aE\xE2V[\x90P\x93\x92PPPV\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x1AZ8\x03\x80a\x1AZ\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\x06\rV[\x81`\x02\x81\x90UP`\x01`\x02T`\x03a\0I\x91\x90a\x06\x94V[a\0S\x91\x90a\x06\xD5V[`\x03\x81\x90UP`\x03T\x81Q\x10\x15a\0\xA6W\x80Q`\x03T`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\0\x9D\x92\x91\x90a\x07\x17V[`@Q\x80\x91\x03\x90\xFD[__\x90P[\x81Q\x81\x10\x15a\x01\x0CWa\0\xFE\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x83\x83\x81Q\x81\x10a\0\xEBWa\0\xEAa\x07>V[[` \x02` \x01\x01Qa\x01\x9E` \x1B` \x1CV[P\x80\x80`\x01\x01\x91PPa\0\xABV[Pa\x01W\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x82_\x81Q\x81\x10a\x01DWa\x01Ca\x07>V[[` \x02` \x01\x01Qa\x01\x9E` \x1B` \x1CV[P\x7F\xA2\xDFx0\xE0\xBE\xDE\xF7\xB1\x11k\xF5G\xB4g\xB1kP\xB3\xBD#\x14l\x9E\t\x98x\xD1N\x890\x1A`\x03T`\x02T3`@Qa\x01\x8F\x93\x92\x91\x90a\x07zV[`@Q\x80\x91\x03\x90\xA1PPa\x07\xAFV[_a\x01\xAF\x83\x83a\x01\xB7` \x1B` \x1CV[\x90P\x92\x91PPV[__a\x01\xC9\x84\x84a\x02\0` \x1B` \x1CV[\x90P\x80\x15a\x01\xF6Wa\x01\xF4\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a\x02\xF5` \x1B\x90\x91\x90` \x1CV[P[\x80\x91PP\x92\x91PPV[_a\x02\x11\x83\x83a\x03(` \x1B` \x1CV[a\x02\xEBW`\x01__\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x02\x88a\x03\x8B` \x1B` \x1CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\x02\xEFV[_\x90P[\x92\x91PPV[_a\x03 \x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba\x03\x92` \x1B` \x1CV[\x90P\x92\x91PPV[___\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[_3\x90P\x90V[_a\x03\xA3\x83\x83a\x03\xFF` \x1B` \x1CV[a\x03\xF5W\x82_\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x82_\x01\x80T\x90P\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP`\x01\x90Pa\x03\xF9V[_\x90P[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x14\x15\x90P\x92\x91PPV[_`@Q\x90P\x90V[__\xFD[__\xFD[_\x81\x90P\x91\x90PV[a\x04B\x81a\x040V[\x81\x14a\x04LW__\xFD[PV[_\x81Q\x90Pa\x04]\x81a\x049V[\x92\x91PPV[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x04\xAD\x82a\x04gV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x04\xCCWa\x04\xCBa\x04wV[[\x80`@RPPPV[_a\x04\xDEa\x04\x1FV[\x90Pa\x04\xEA\x82\x82a\x04\xA4V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x05\tWa\x05\x08a\x04wV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x05G\x82a\x05\x1EV[\x90P\x91\x90PV[a\x05W\x81a\x05=V[\x81\x14a\x05aW__\xFD[PV[_\x81Q\x90Pa\x05r\x81a\x05NV[\x92\x91PPV[_a\x05\x8Aa\x05\x85\x84a\x04\xEFV[a\x04\xD5V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x05\xADWa\x05\xACa\x05\x1AV[[\x83[\x81\x81\x10\x15a\x05\xD6W\x80a\x05\xC2\x88\x82a\x05dV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x05\xAFV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x05\xF4Wa\x05\xF3a\x04cV[[\x81Qa\x06\x04\x84\x82` \x86\x01a\x05xV[\x91PP\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x06#Wa\x06\"a\x04(V[[_a\x060\x85\x82\x86\x01a\x04OV[\x92PP` \x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06QWa\x06Pa\x04,V[[a\x06]\x85\x82\x86\x01a\x05\xE0V[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x06\x9E\x82a\x040V[\x91Pa\x06\xA9\x83a\x040V[\x92P\x82\x82\x02a\x06\xB7\x81a\x040V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x06\xCEWa\x06\xCDa\x06gV[[P\x92\x91PPV[_a\x06\xDF\x82a\x040V[\x91Pa\x06\xEA\x83a\x040V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x07\x02Wa\x07\x01a\x06gV[[\x92\x91PPV[a\x07\x11\x81a\x040V[\x82RPPV[_`@\x82\x01\x90Pa\x07*_\x83\x01\x85a\x07\x08V[a\x077` \x83\x01\x84a\x07\x08V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[a\x07t\x81a\x05=V[\x82RPPV[_``\x82\x01\x90Pa\x07\x8D_\x83\x01\x86a\x07\x08V[a\x07\x9A` \x83\x01\x85a\x07\x08V[a\x07\xA7`@\x83\x01\x84a\x07kV[\x94\x93PPPPV[a\x12\x9E\x80a\x07\xBC_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xFEW_5`\xE0\x1C\x80c\x7F5\xB5`\x11a\0\x95W\x80c\xA3$j\xD3\x11a\0dW\x80c\xA3$j\xD3\x14a\x02\xC0W\x80c\xCA\x15\xC8s\x14a\x02\xF0W\x80c\xD5Gt\x1F\x14a\x03 W\x80c\xFCx\xB2\xE8\x14a\x03<Wa\0\xFEV[\x80c\x7F5\xB5`\x14a\x02$W\x80c\x90\x10\xD0|\x14a\x02BW\x80c\x91\xD1HT\x14a\x02rW\x80c\xA2\x17\xFD\xDF\x14a\x02\xA2Wa\0\xFEV[\x80c0\x10L>\x11a\0\xD1W\x80c0\x10L>\x14a\x01\xAEW\x80c6V\x8A\xBE\x14a\x01\xCCW\x80cI\xF2\xAD\xA0\x14a\x01\xE8W\x80cX\xDF\r\x01\x14a\x02\x06Wa\0\xFEV[\x80c\x01\xFF\xC9\xA7\x14a\x01\x02W\x80c\x13\xFFm\xD5\x14a\x012W\x80c$\x8A\x9C\xA3\x14a\x01bW\x80c//\xF1]\x14a\x01\x92W[__\xFD[a\x01\x1C`\x04\x806\x03\x81\x01\x90a\x01\x17\x91\x90a\x0EHV[a\x03lV[`@Qa\x01)\x91\x90a\x0E\x8DV[`@Q\x80\x91\x03\x90\xF3[a\x01L`\x04\x806\x03\x81\x01\x90a\x01G\x91\x90a\x0F\0V[a\x03\xE5V[`@Qa\x01Y\x91\x90a\x0E\x8DV[`@Q\x80\x91\x03\x90\xF3[a\x01|`\x04\x806\x03\x81\x01\x90a\x01w\x91\x90a\x0F^V[a\x04(V[`@Qa\x01\x89\x91\x90a\x0F\x98V[`@Q\x80\x91\x03\x90\xF3[a\x01\xAC`\x04\x806\x03\x81\x01\x90a\x01\xA7\x91\x90a\x0F\xB1V[a\x04DV[\0[a\x01\xB6a\x04\x86V[`@Qa\x01\xC3\x91\x90a\x0F\x98V[`@Q\x80\x91\x03\x90\xF3[a\x01\xE6`\x04\x806\x03\x81\x01\x90a\x01\xE1\x91\x90a\x0F\xB1V[a\x04\xAAV[\0[a\x01\xF0a\x04\xC0V[`@Qa\x01\xFD\x91\x90a\x0F\x98V[`@Q\x80\x91\x03\x90\xF3[a\x02\x0Ea\x04\xE4V[`@Qa\x02\x1B\x91\x90a\x0F\x98V[`@Q\x80\x91\x03\x90\xF3[a\x02,a\x05\x08V[`@Qa\x029\x91\x90a\x0F\x98V[`@Q\x80\x91\x03\x90\xF3[a\x02\\`\x04\x806\x03\x81\x01\x90a\x02W\x91\x90a\x10\"V[a\x05,V[`@Qa\x02i\x91\x90a\x10oV[`@Q\x80\x91\x03\x90\xF3[a\x02\x8C`\x04\x806\x03\x81\x01\x90a\x02\x87\x91\x90a\x0F\xB1V[a\x05XV[`@Qa\x02\x99\x91\x90a\x0E\x8DV[`@Q\x80\x91\x03\x90\xF3[a\x02\xAAa\x05\xBBV[`@Qa\x02\xB7\x91\x90a\x0F\x98V[`@Q\x80\x91\x03\x90\xF3[a\x02\xDA`\x04\x806\x03\x81\x01\x90a\x02\xD5\x91\x90a\x0F^V[a\x05\xC1V[`@Qa\x02\xE7\x91\x90a\x11?V[`@Q\x80\x91\x03\x90\xF3[a\x03\n`\x04\x806\x03\x81\x01\x90a\x03\x05\x91\x90a\x0F^V[a\x05\xE3V[`@Qa\x03\x17\x91\x90a\x11nV[`@Q\x80\x91\x03\x90\xF3[a\x03:`\x04\x806\x03\x81\x01\x90a\x035\x91\x90a\x0F\xB1V[a\x06\x04V[\0[a\x03V`\x04\x806\x03\x81\x01\x90a\x03Q\x91\x90a\x0F\0V[a\x06FV[`@Qa\x03c\x91\x90a\x0E\x8DV[`@Q\x80\x91\x03\x90\xF3[_\x7F\x07\xEF\xFE\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x03\xDEWPa\x03\xDD\x82a\x06xV[[\x90P\x91\x90PV[_a\x03\xEF\x82a\x06FV[\x80\x15a\x04!WPa\x04 \x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x83a\x05XV[[\x90P\x91\x90PV[___\x83\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x04n\x81a\x06\xF1V[a\x04va\x07\x05V[a\x04\x80\x83\x83a\x07\x07V[PPPPV[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x81V[a\x04\xB2a\x07\x05V[a\x04\xBC\x82\x82a\x07\x1AV[PPV[\x7F\xD6\x8E>^6\x7F\xEEG\xCE\x11\xA5\xDA\xB4\x04Yi\x80\xE1X\xEB\x90i3\n\x8Fw]\xE7\xDCk\xB6M\x81V[\x7F`\x1A(\xE6\xAB\xEE\xA5+\x0B\xA1[T\xF6\xD1]x\x9F\xD3\xE8\xC0\x08\x0C\x04*\0\x9A\x99d\xF0\xE1\x8F\x1C\x81V[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x81V[_a\x05P\x82`\x01_\x86\x81R` \x01\x90\x81R` \x01_ a\x07\x95\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[___\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[__\x1B\x81V[``a\x05\xDC`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a\x07\xACV[\x90P\x91\x90PV[_a\x05\xFD`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a\x07\xCBV[\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x06.\x81a\x06\xF1V[a\x066a\x07\x05V[a\x06@\x83\x83a\x07\xDEV[PPPPV[_a\x06q\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x83a\x05XV[\x90P\x91\x90PV[_\x7FZ\x05\x18\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x06\xEAWPa\x06\xE9\x82a\x07\xF1V[[\x90P\x91\x90PV[a\x07\x02\x81a\x06\xFDa\x08jV[a\x08qV[PV[V[_a\x07\x12\x83\x83a\x08\xC2V[\x90P\x92\x91PPV[a\x07\"a\x08jV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07\x86W`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\x90\x82\x82a\x07\xDEV[PPPV[_a\x07\xA2\x83_\x01\x83a\t\x05V[_\x1C\x90P\x92\x91PPV[``_a\x07\xBA\x83_\x01a\t,V[\x90P``\x81\x90P\x80\x92PPP\x91\x90PV[_a\x07\xD7\x82_\x01a\t\x85V[\x90P\x91\x90PV[_a\x07\xE9\x83\x83a\t\x94V[\x90P\x92\x91PPV[_\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x08cWPa\x08b\x82a\t\xD7V[[\x90P\x91\x90PV[_3\x90P\x90V[a\x08{\x82\x82a\x05XV[a\x08\xBEW\x80\x82`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xB5\x92\x91\x90a\x11\x87V[`@Q\x80\x91\x03\x90\xFD[PPV[__a\x08\xCE\x84\x84a\n@V[\x90P\x80\x15a\x08\xFBWa\x08\xF9\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a\x0B)\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[_\x82_\x01\x82\x81T\x81\x10a\t\x1BWa\t\x1Aa\x11\xAEV[[\x90_R` _ \x01T\x90P\x92\x91PPV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\tyW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\teW[PPPPP\x90P\x91\x90PV[_\x81_\x01\x80T\x90P\x90P\x91\x90PV[__a\t\xA0\x84\x84a\x0BVV[\x90P\x80\x15a\t\xCDWa\t\xCB\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a\x0C?\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[_\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[_a\nK\x83\x83a\x05XV[a\x0B\x1FW`\x01__\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\n\xBCa\x08jV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\x0B#V[_\x90P[\x92\x91PPV[_a\x0BN\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba\x0ClV[\x90P\x92\x91PPV[_a\x0Ba\x83\x83a\x05XV[\x15a\x0C5W___\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x0B\xD2a\x08jV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\x0C9V[_\x90P[\x92\x91PPV[_a\x0Cd\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba\x0C\xD3V[\x90P\x92\x91PPV[_a\x0Cw\x83\x83a\r\xCFV[a\x0C\xC9W\x82_\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x82_\x01\x80T\x90P\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP`\x01\x90Pa\x0C\xCDV[_\x90P[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x14a\r\xC4W_`\x01\x82a\r\0\x91\x90a\x12\x08V[\x90P_`\x01\x86_\x01\x80T\x90Pa\r\x16\x91\x90a\x12\x08V[\x90P\x80\x82\x14a\r|W_\x86_\x01\x82\x81T\x81\x10a\r5Wa\r4a\x11\xAEV[[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a\rVWa\rUa\x11\xAEV[[\x90_R` _ \x01\x81\x90UP\x83\x87`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UPP[\x85_\x01\x80T\x80a\r\x8FWa\r\x8Ea\x12;V[[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\r\xC9V[_\x91PP[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x14\x15\x90P\x92\x91PPV[__\xFD[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a\x0E'\x81a\r\xF3V[\x81\x14a\x0E1W__\xFD[PV[_\x815\x90Pa\x0EB\x81a\x0E\x1EV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0E]Wa\x0E\\a\r\xEFV[[_a\x0Ej\x84\x82\x85\x01a\x0E4V[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\x0E\x87\x81a\x0EsV[\x82RPPV[_` \x82\x01\x90Pa\x0E\xA0_\x83\x01\x84a\x0E~V[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x0E\xCF\x82a\x0E\xA6V[\x90P\x91\x90PV[a\x0E\xDF\x81a\x0E\xC5V[\x81\x14a\x0E\xE9W__\xFD[PV[_\x815\x90Pa\x0E\xFA\x81a\x0E\xD6V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0F\x15Wa\x0F\x14a\r\xEFV[[_a\x0F\"\x84\x82\x85\x01a\x0E\xECV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x0F=\x81a\x0F+V[\x81\x14a\x0FGW__\xFD[PV[_\x815\x90Pa\x0FX\x81a\x0F4V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0FsWa\x0Fra\r\xEFV[[_a\x0F\x80\x84\x82\x85\x01a\x0FJV[\x91PP\x92\x91PPV[a\x0F\x92\x81a\x0F+V[\x82RPPV[_` \x82\x01\x90Pa\x0F\xAB_\x83\x01\x84a\x0F\x89V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x0F\xC7Wa\x0F\xC6a\r\xEFV[[_a\x0F\xD4\x85\x82\x86\x01a\x0FJV[\x92PP` a\x0F\xE5\x85\x82\x86\x01a\x0E\xECV[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[a\x10\x01\x81a\x0F\xEFV[\x81\x14a\x10\x0BW__\xFD[PV[_\x815\x90Pa\x10\x1C\x81a\x0F\xF8V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x108Wa\x107a\r\xEFV[[_a\x10E\x85\x82\x86\x01a\x0FJV[\x92PP` a\x10V\x85\x82\x86\x01a\x10\x0EV[\x91PP\x92P\x92\x90PV[a\x10i\x81a\x0E\xC5V[\x82RPPV[_` \x82\x01\x90Pa\x10\x82_\x83\x01\x84a\x10`V[\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x10\xBA\x81a\x0E\xC5V[\x82RPPV[_a\x10\xCB\x83\x83a\x10\xB1V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x10\xED\x82a\x10\x88V[a\x10\xF7\x81\x85a\x10\x92V[\x93Pa\x11\x02\x83a\x10\xA2V[\x80_[\x83\x81\x10\x15a\x112W\x81Qa\x11\x19\x88\x82a\x10\xC0V[\x97Pa\x11$\x83a\x10\xD7V[\x92PP`\x01\x81\x01\x90Pa\x11\x05V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x11W\x81\x84a\x10\xE3V[\x90P\x92\x91PPV[a\x11h\x81a\x0F\xEFV[\x82RPPV[_` \x82\x01\x90Pa\x11\x81_\x83\x01\x84a\x11_V[\x92\x91PPV[_`@\x82\x01\x90Pa\x11\x9A_\x83\x01\x85a\x10`V[a\x11\xA7` \x83\x01\x84a\x0F\x89V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x12\x12\x82a\x0F\xEFV[\x91Pa\x12\x1D\x83a\x0F\xEFV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x125Wa\x124a\x11\xDBV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 ?\xE4J\xCEU\xC9\x0E<\xD1}\x0Cg\x80\xB97\xFA\xC6\xF4\x84\xE5F\xEF\x97LU*#\xFA\t$\xA6\xAEdsolcC\0\x08!\x003\xA2dipfsX\"\x12 x\xD8\xBD4\xB5zk\x9A\x98\x06\x18\x19%O/C,\xDE\x80\xEC^\xAC\x8A;my\xD2\x82\x14\xB5\x0EPdsolcC\0\x08!\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106101a7575f3560e01c806385226c81116100f7578063cd099e6a11610095578063e05051991161006f578063e05051991461036d578063e20c9f7114610377578063e49e9ff514610395578063fa7626d41461039f576101a7565b8063cd099e6a1461034f578063d3fe9e7d14610359578063d7c3e53214610363576101a7565b8063b0464fdc116100d1578063b0464fdc146102eb578063b5508aa914610309578063ba2cbd3514610327578063ba414fa614610331576101a7565b806385226c81146102a5578063916a17c6146102c35780639bc8a619146102e1576101a7565b80633e5e3c231161016457806354b7c39b1161013e57806354b7c39b146102555780635c972be51461027357806365c9f4851461027d57806366d9a9a014610287576101a7565b80633e5e3c231461020f5780633f7286f41461022d578063484b3f361461024b576101a7565b80630a9254e4146101ab5780631ed7831c146101b5578063200a56ce146101d35780632ade3880146101dd578063374d1fd5146101fb5780633a5e4f2314610205575b5f5ffd5b6101b36103bd565b005b6101bd61076a565b6040516101ca919061398a565b60405180910390f35b6101db6107f5565b005b6101e5610a74565b6040516101f29190613bca565b60405180910390f35b610203610bf8565b005b61020d611232565b005b6102176113d6565b604051610224919061398a565b60405180910390f35b610235611461565b604051610242919061398a565b60405180910390f35b6102536114ec565b005b61025d6116fd565b60405161026a9190613c45565b60405180910390f35b61027b611723565b005b610285611934565b005b61028f611aed565b60405161029c9190613e3c565b60405180910390f35b6102ad611c6f565b6040516102ba9190613edf565b60405180910390f35b6102cb611d43565b6040516102d89190613ff4565b60405180910390f35b6102e9611e8a565b005b6102f36120f1565b6040516103009190613ff4565b60405180910390f35b610311612238565b60405161031e9190613edf565b60405180910390f35b61032f61230c565b005b6103396125fc565b604051610346919061402e565b60405180910390f35b610357612703565b005b610361612992565b005b61036b612c2d565b005b610375612e59565b005b61037f6130b7565b60405161038c919061398a565b60405180910390f35b61039d613142565b005b6103a7613562565b6040516103b4919061402e565b60405180910390f35b5f600467ffffffffffffffff8111156103d9576103d8614047565b5b6040519080825280602002602001820160405280156104075781602001602082028036833780820191505090505b50905060205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16815f8151811061043f5761043e614074565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505060215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16816001815181106104af576104ae614074565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505060225f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff168160028151811061051f5761051e614074565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505060235f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff168160038151811061058f5761058e614074565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250506001816040516105d890613896565b6105e39291906140e3565b604051809103905ff0801580156105fc573d5f5f3e3d5ffd5b50601f60016101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166330104c3e6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156106a8573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106cc9190614148565b602481905550601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16637f35b5606040518163ffffffff1660e01b8152600401602060405180830381865afa15801561073d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107619190614148565b60258190555050565b606060168054806020026020016040519081016040528092919081815260200182805480156107eb57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116107a2575b5050505050905090565b5f6108346040518060400160405280600981526020017f4e45575f50415254590000000000000000000000000000000000000000000000815250613574565b9050737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663ca669fa760215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b81526004016108a49190614182565b5f604051808303815f87803b1580156108bb575f5ffd5b505af11580156108cd573d5f5f3e3d5ffd5b50505050737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663f28dceb363e2517d3f60e01b60215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1660255460405160240161093f9291906141aa565b604051602081830303815290604052907bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040518263ffffffff1660e01b81526004016109b89190614223565b5f604051808303815f87803b1580156109cf575f5ffd5b505af11580156109e1573d5f5f3e3d5ffd5b50505050601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16632f2ff15d602454836040518363ffffffff1660e01b8152600401610a44929190614243565b5f604051808303815f87803b158015610a5b575f5ffd5b505af1158015610a6d573d5f5f3e3d5ffd5b5050505050565b6060601e805480602002602001604051908101604052809291908181526020015f905b82821015610bef578382905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200160018201805480602002602001604051908101604052809291908181526020015f905b82821015610bd8578382905f5260205f20018054610b4d90614297565b80601f0160208091040260200160405190810160405280929190818152602001828054610b7990614297565b8015610bc45780601f10610b9b57610100808354040283529160200191610bc4565b820191905f5260205f20905b815481529060010190602001808311610ba757829003601f168201915b505050505081526020019060010190610b30565b505050508152505081526020019060010190610a97565b50505050905090565b610cbf601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166391d1485460245460205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b8152600401610c7b929190614243565b602060405180830381865afa158015610c96573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cba91906142f1565b613588565b610d86601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166391d1485460245460215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b8152600401610d42929190614243565b602060405180830381865afa158015610d5d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d8191906142f1565b613588565b610e4d601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166391d1485460245460225f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b8152600401610e09929190614243565b602060405180830381865afa158015610e24573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e4891906142f1565b613588565b610f14601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166391d1485460245460235f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b8152600401610ed0929190614243565b602060405180830381865afa158015610eeb573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f0f91906142f1565b613588565b610fdb601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166391d1485460255460205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b8152600401610f97929190614243565b602060405180830381865afa158015610fb2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610fd691906142f1565b613588565b6110a2601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166391d1485460255460215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b815260040161105e929190614243565b602060405180830381865afa158015611079573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061109d91906142f1565b613618565b611169601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166391d1485460255460225f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b8152600401611125929190614243565b602060405180830381865afa158015611140573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061116491906142f1565b613618565b611230601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166391d1485460255460235f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b81526004016111ec929190614243565b602060405180830381865afa158015611207573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061122b91906142f1565b613618565b565b6112f5601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663fc78b2e860215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b81526004016112b19190614182565b602060405180830381865afa1580156112cc573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112f091906142f1565b613588565b6113d4601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663fc78b2e86113746040518060400160405280600881526020017f535452414e474552000000000000000000000000000000000000000000000000815250613574565b6040518263ffffffff1660e01b81526004016113909190614182565b602060405180830381865afa1580156113ab573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113cf91906142f1565b613618565b565b6060601880548060200260200160405190810160405280929190818152602001828054801561145757602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001906001019080831161140e575b5050505050905090565b606060178054806020026020016040519081016040528092919081815260200182805480156114e257602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311611499575b5050505050905090565b737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663ca669fa760215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b815260040161155a9190614182565b5f604051808303815f87803b158015611571575f5ffd5b505af1158015611583573d5f5f3e3d5ffd5b50505050601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166336568abe60245460215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b8152600401611607929190614243565b5f604051808303815f87803b15801561161e575f5ffd5b505af1158015611630573d5f5f3e3d5ffd5b505050506116fb601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166391d1485460245460215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b81526004016116b7929190614243565b602060405180830381865afa1580156116d2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116f691906142f1565b613618565b565b601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663ca669fa760205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b81526004016117919190614182565b5f604051808303815f87803b1580156117a8575f5ffd5b505af11580156117ba573d5f5f3e3d5ffd5b50505050601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663d547741f60245460215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b815260040161183e929190614243565b5f604051808303815f87803b158015611855575f5ffd5b505af1158015611867573d5f5f3e3d5ffd5b50505050611932601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166391d1485460245460215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b81526004016118ee929190614243565b602060405180830381865afa158015611909573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061192d91906142f1565b613618565b565b737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663ca669fa760215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b81526004016119a29190614182565b5f604051808303815f87803b1580156119b9575f5ffd5b505af11580156119cb573d5f5f3e3d5ffd5b50505050737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663f48448146040518163ffffffff1660e01b81526004015f604051808303815f87803b158015611a28575f5ffd5b505af1158015611a3a573d5f5f3e3d5ffd5b50505050601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166336568abe60245460225f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b8152600401611abe929190614243565b5f604051808303815f87803b158015611ad5575f5ffd5b505af1158015611ae7573d5f5f3e3d5ffd5b50505050565b6060601b805480602002602001604051908101604052809291908181526020015f905b82821015611c66578382905f5260205f2090600202016040518060400160405290815f82018054611b4090614297565b80601f0160208091040260200160405190810160405280929190818152602001828054611b6c90614297565b8015611bb75780601f10611b8e57610100808354040283529160200191611bb7565b820191905f5260205f20905b815481529060010190602001808311611b9a57829003601f168201915b5050505050815260200160018201805480602002602001604051908101604052809291908181526020018280548015611c4e57602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191681526020019060040190602082600301049283019260010382029150808411611bfb5790505b50505050508152505081526020019060010190611b10565b50505050905090565b6060601a805480602002602001604051908101604052809291908181526020015f905b82821015611d3a578382905f5260205f20018054611caf90614297565b80601f0160208091040260200160405190810160405280929190818152602001828054611cdb90614297565b8015611d265780601f10611cfd57610100808354040283529160200191611d26565b820191905f5260205f20905b815481529060010190602001808311611d0957829003601f168201915b505050505081526020019060010190611c92565b50505050905090565b6060601d805480602002602001604051908101604052809291908181526020015f905b82821015611e81578382905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200160018201805480602002602001604051908101604052809291908181526020018280548015611e6957602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191681526020019060040190602082600301049283019260010382029150808411611e165790505b50505050508152505081526020019060010190611d66565b50505050905090565b611f4d601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166313ff6dd560205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b8152600401611f099190614182565b602060405180830381865afa158015611f24573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f4891906142f1565b613588565b612010601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166313ff6dd560215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b8152600401611fcc9190614182565b602060405180830381865afa158015611fe7573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061200b91906142f1565b613618565b6120ef601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166313ff6dd561208f6040518060400160405280600881526020017f535452414e474552000000000000000000000000000000000000000000000000815250613574565b6040518263ffffffff1660e01b81526004016120ab9190614182565b602060405180830381865afa1580156120c6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906120ea91906142f1565b613618565b565b6060601c805480602002602001604051908101604052809291908181526020015f905b8282101561222f578382905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020016001820180548060200260200160405190810160405280929190818152602001828054801561221757602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916815260200190600401906020826003010492830192600103820291508084116121c45790505b50505050508152505081526020019060010190612114565b50505050905090565b60606019805480602002602001604051908101604052809291908181526020015f905b82821015612303578382905f5260205f2001805461227890614297565b80601f01602080910402602001604051908101604052809291908181526020018280546122a490614297565b80156122ef5780601f106122c6576101008083540402835291602001916122ef565b820191905f5260205f20905b8154815290600101906020018083116122d257829003601f168201915b50505050508152602001906001019061225b565b50505050905090565b5f600467ffffffffffffffff81111561232857612327614047565b5b6040519080825280602002602001820160405280156123565781602001602082028036833780820191505090505b50905060205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16815f8151811061238e5761238d614074565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505060215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16816001815181106123fe576123fd614074565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505060225f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff168160028151811061246e5761246d614074565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505060235f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16816003815181106124de576124dd614074565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff1681525050737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663440ed10d6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015612571575f5ffd5b505af1158015612583573d5f5f3e3d5ffd5b505050507fa2df7830e0bedef7b1116bf547b467b16b50b3bd23146c9e099878d14e89301a60046001306040516125bc93929190614355565b60405180910390a16001816040516125d390613896565b6125de9291906140e3565b604051809103905ff0801580156125f7573d5f5f3e3d5ffd5b505050565b5f60085f9054906101000a900460ff161561261a5760019050612700565b5f5f1b7f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c73ffffffffffffffffffffffffffffffffffffffff1663667f9d707f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c7f6661696c656400000000000000000000000000000000000000000000000000006040518363ffffffff1660e01b81526004016126bc9291906141aa565b602060405180830381865afa1580156126d7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906126fb9190614148565b141590505b90565b5f601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663ca15c8736024546040518263ffffffff1660e01b8152600401612761919061438a565b602060405180830381865afa15801561277c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906127a091906143cd565b9050737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663ca669fa760205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b81526004016128109190614182565b5f604051808303815f87803b158015612827575f5ffd5b505af1158015612839573d5f5f3e3d5ffd5b50505050601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16632f2ff15d60245460215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b81526004016128bd929190614243565b5f604051808303815f87803b1580156128d4575f5ffd5b505af11580156128e6573d5f5f3e3d5ffd5b5050505061298f601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663ca15c8736024546040518263ffffffff1660e01b815260040161294a919061438a565b602060405180830381865afa158015612965573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061298991906143cd565b826136a9565b50565b5f601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663ca15c8736024546040518263ffffffff1660e01b81526004016129f0919061438a565b602060405180830381865afa158015612a0b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612a2f91906143cd565b9050737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663ca669fa760205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b8152600401612a9f9190614182565b5f604051808303815f87803b158015612ab6575f5ffd5b505af1158015612ac8573d5f5f3e3d5ffd5b50505050601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663d547741f60245460215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b8152600401612b4c929190614243565b5f604051808303815f87803b158015612b63575f5ffd5b505af1158015612b75573d5f5f3e3d5ffd5b50505050612c2a601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663ca15c8736024546040518263ffffffff1660e01b8152600401612bd9919061438a565b602060405180830381865afa158015612bf4573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612c1891906143cd565b600183612c259190614425565b6136a9565b50565b612cef601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166301ffc9a77f07effe0d000000000000000000000000000000000000000000000000000000006040518263ffffffff1660e01b8152600401612cab9190614467565b602060405180830381865afa158015612cc6573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612cea91906142f1565b613588565b612db1601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166301ffc9a77f7965db0b000000000000000000000000000000000000000000000000000000006040518263ffffffff1660e01b8152600401612d6d9190614467565b602060405180830381865afa158015612d88573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612dac91906142f1565b613588565b612e57601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166301ffc9a763deadbeef6040518263ffffffff1660e01b8152600401612e1391906144c5565b602060405180830381865afa158015612e2e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612e5291906142f1565b613618565b565b737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663ca669fa760215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b8152600401612ec79190614182565b5f604051808303815f87803b158015612ede575f5ffd5b505af1158015612ef0573d5f5f3e3d5ffd5b50505050737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663f28dceb363e2517d3f60e01b60215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16602554604051602401612f629291906141aa565b604051602081830303815290604052907bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040518263ffffffff1660e01b8152600401612fdb9190614223565b5f604051808303815f87803b158015612ff2575f5ffd5b505af1158015613004573d5f5f3e3d5ffd5b50505050601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663d547741f60245460225f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b8152600401613088929190614243565b5f604051808303815f87803b15801561309f575f5ffd5b505af11580156130b1573d5f5f3e3d5ffd5b50505050565b6060601580548060200260200160405190810160405280929190818152602001828054801561313857602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116130ef575b5050505050905090565b5f6131816040518060400160405280600981526020017f4e45575f50415254590000000000000000000000000000000000000000000000815250613574565b9050737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663ca669fa760205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b81526004016131f19190614182565b5f604051808303815f87803b158015613208575f5ffd5b505af115801561321a573d5f5f3e3d5ffd5b50505050601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16632f2ff15d602454836040518363ffffffff1660e01b815260040161327d929190614243565b5f604051808303815f87803b158015613294575f5ffd5b505af11580156132a6573d5f5f3e3d5ffd5b50505050613350601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166391d14854602454846040518363ffffffff1660e01b815260040161330c929190614243565b602060405180830381865afa158015613327573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061334b91906142f1565b613588565b737109709ecfa91a80626ff3989d68f67f5b1dd12d73ffffffffffffffffffffffffffffffffffffffff1663ca669fa760205f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518263ffffffff1660e01b81526004016133be9190614182565b5f604051808303815f87803b1580156133d5575f5ffd5b505af11580156133e7573d5f5f3e3d5ffd5b50505050601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16632f2ff15d60255460215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b815260040161346b929190614243565b5f604051808303815f87803b158015613482575f5ffd5b505af1158015613494573d5f5f3e3d5ffd5b5050505061355f601f60019054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166391d1485460255460215f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b815260040161351b929190614243565b602060405180830381865afa158015613536573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061355a91906142f1565b613588565b50565b601f5f9054906101000a900460ff1681565b5f61357e8261373e565b5080915050919050565b80613615577f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c73ffffffffffffffffffffffffffffffffffffffff16630c9fd581826040518263ffffffff1660e01b81526004016135e8919061402e565b5f6040518083038186803b1580156135fe575f5ffd5b505afa158015613610573d5f5f3e3d5ffd5b505050505b50565b80156136a6577f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c73ffffffffffffffffffffffffffffffffffffffff1663a5982885826040518263ffffffff1660e01b8152600401613679919061402e565b5f6040518083038186803b15801561368f575f5ffd5b505afa1580156136a1573d5f5f3e3d5ffd5b505050505b50565b80821461373a577f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c73ffffffffffffffffffffffffffffffffffffffff166398296c5483836040518363ffffffff1660e01b815260040161370d9291906144ed565b5f6040518083038186803b158015613723575f5ffd5b505afa158015613735573d5f5f3e3d5ffd5b505050505b5050565b5f5f82604051602001613751919061454e565b604051602081830303815290604052805190602001205f1c90507f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c73ffffffffffffffffffffffffffffffffffffffff1663ffa18649826040518263ffffffff1660e01b81526004016137c69190614564565b602060405180830381865afa1580156137e1573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061380591906145a7565b91507f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c73ffffffffffffffffffffffffffffffffffffffff1663c657c71883856040518363ffffffff1660e01b815260040161386492919061461a565b5f604051808303815f87803b15801561387b575f5ffd5b505af115801561388d573d5f5f3e3d5ffd5b50505050915091565b611a5a8061464983390190565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6138f5826138cc565b9050919050565b613905816138eb565b82525050565b5f61391683836138fc565b60208301905092915050565b5f602082019050919050565b5f613938826138a3565b61394281856138ad565b935061394d836138bd565b805f5b8381101561397d578151613964888261390b565b975061396f83613922565b925050600181019050613950565b5085935050505092915050565b5f6020820190508181035f8301526139a2818461392e565b905092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f601f19601f8301169050919050565b5f613a3e826139fc565b613a488185613a06565b9350613a58818560208601613a16565b613a6181613a24565b840191505092915050565b5f613a778383613a34565b905092915050565b5f602082019050919050565b5f613a95826139d3565b613a9f81856139dd565b935083602082028501613ab1856139ed565b805f5b85811015613aec5784840389528151613acd8582613a6c565b9450613ad883613a7f565b925060208a01995050600181019050613ab4565b50829750879550505050505092915050565b5f604083015f830151613b135f8601826138fc565b5060208301518482036020860152613b2b8282613a8b565b9150508091505092915050565b5f613b438383613afe565b905092915050565b5f602082019050919050565b5f613b61826139aa565b613b6b81856139b4565b935083602082028501613b7d856139c4565b805f5b85811015613bb85784840389528151613b998582613b38565b9450613ba483613b4b565b925060208a01995050600181019050613b80565b50829750879550505050505092915050565b5f6020820190508181035f830152613be28184613b57565b905092915050565b5f819050919050565b5f613c0d613c08613c03846138cc565b613bea565b6138cc565b9050919050565b5f613c1e82613bf3565b9050919050565b5f613c2f82613c14565b9050919050565b613c3f81613c25565b82525050565b5f602082019050613c585f830184613c36565b92915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f7fffffffff0000000000000000000000000000000000000000000000000000000082169050919050565b613ce481613cb0565b82525050565b5f613cf58383613cdb565b60208301905092915050565b5f602082019050919050565b5f613d1782613c87565b613d218185613c91565b9350613d2c83613ca1565b805f5b83811015613d5c578151613d438882613cea565b9750613d4e83613d01565b925050600181019050613d2f565b5085935050505092915050565b5f604083015f8301518482035f860152613d838282613a34565b91505060208301518482036020860152613d9d8282613d0d565b9150508091505092915050565b5f613db58383613d69565b905092915050565b5f602082019050919050565b5f613dd382613c5e565b613ddd8185613c68565b935083602082028501613def85613c78565b805f5b85811015613e2a5784840389528151613e0b8582613daa565b9450613e1683613dbd565b925060208a01995050600181019050613df2565b50829750879550505050505092915050565b5f6020820190508181035f830152613e548184613dc9565b905092915050565b5f82825260208201905092915050565b5f613e76826139d3565b613e808185613e5c565b935083602082028501613e92856139ed565b805f5b85811015613ecd5784840389528151613eae8582613a6c565b9450613eb983613a7f565b925060208a01995050600181019050613e95565b50829750879550505050505092915050565b5f6020820190508181035f830152613ef78184613e6c565b905092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f604083015f830151613f3d5f8601826138fc565b5060208301518482036020860152613f558282613d0d565b9150508091505092915050565b5f613f6d8383613f28565b905092915050565b5f602082019050919050565b5f613f8b82613eff565b613f958185613f09565b935083602082028501613fa785613f19565b805f5b85811015613fe25784840389528151613fc38582613f62565b9450613fce83613f75565b925060208a01995050600181019050613faa565b50829750879550505050505092915050565b5f6020820190508181035f83015261400c8184613f81565b905092915050565b5f8115159050919050565b61402881614014565b82525050565b5f6020820190506140415f83018461401f565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f819050919050565b5f819050919050565b5f6140cd6140c86140c3846140a1565b613bea565b6140aa565b9050919050565b6140dd816140b3565b82525050565b5f6040820190506140f65f8301856140d4565b8181036020830152614108818461392e565b90509392505050565b5f5ffd5b5f819050919050565b61412781614115565b8114614131575f5ffd5b50565b5f815190506141428161411e565b92915050565b5f6020828403121561415d5761415c614111565b5b5f61416a84828501614134565b91505092915050565b61417c816138eb565b82525050565b5f6020820190506141955f830184614173565b92915050565b6141a481614115565b82525050565b5f6040820190506141bd5f830185614173565b6141ca602083018461419b565b9392505050565b5f81519050919050565b5f82825260208201905092915050565b5f6141f5826141d1565b6141ff81856141db565b935061420f818560208601613a16565b61421881613a24565b840191505092915050565b5f6020820190508181035f83015261423b81846141eb565b905092915050565b5f6040820190506142565f83018561419b565b6142636020830184614173565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f60028204905060018216806142ae57607f821691505b6020821081036142c1576142c061426a565b5b50919050565b6142d081614014565b81146142da575f5ffd5b50565b5f815190506142eb816142c7565b92915050565b5f6020828403121561430657614305614111565b5b5f614313848285016142dd565b91505092915050565b5f819050919050565b5f61433f61433a6143358461431c565b613bea565b6140aa565b9050919050565b61434f81614325565b82525050565b5f6060820190506143685f830186614346565b61437560208301856140d4565b6143826040830184614173565b949350505050565b5f60208201905061439d5f83018461419b565b92915050565b6143ac816140aa565b81146143b6575f5ffd5b50565b5f815190506143c7816143a3565b92915050565b5f602082840312156143e2576143e1614111565b5b5f6143ef848285016143b9565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61442f826140aa565b915061443a836140aa565b9250828203905081811115614452576144516143f8565b5b92915050565b61446181613cb0565b82525050565b5f60208201905061447a5f830184614458565b92915050565b5f819050919050565b5f8160e01b9050919050565b5f6144af6144aa6144a584614480565b614489565b613cb0565b9050919050565b6144bf81614495565b82525050565b5f6020820190506144d85f8301846144b6565b92915050565b6144e7816140aa565b82525050565b5f6040820190506145005f8301856144de565b61450d60208301846144de565b9392505050565b5f81905092915050565b5f614528826139fc565b6145328185614514565b9350614542818560208601613a16565b80840191505092915050565b5f614559828461451e565b915081905092915050565b5f6020820190506145775f8301846144de565b92915050565b614586816138eb565b8114614590575f5ffd5b50565b5f815190506145a18161457d565b92915050565b5f602082840312156145bc576145bb614111565b5b5f6145c984828501614593565b91505092915050565b5f82825260208201905092915050565b5f6145ec826139fc565b6145f681856145d2565b9350614606818560208601613a16565b61460f81613a24565b840191505092915050565b5f60408201905061462d5f830185614173565b818103602083015261463f81846145e2565b9050939250505056fe608060405234801561000f575f5ffd5b50604051611a5a380380611a5a8339818101604052810190610031919061060d565b81600281905550600160025460036100499190610694565b61005391906106d5565b600381905550600354815110156100a65780516003546040517f3a23626800000000000000000000000000000000000000000000000000000000815260040161009d929190610717565b60405180910390fd5b5f5f90505b815181101561010c576100fe7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc4698383815181106100eb576100ea61073e565b5b602002602001015161019e60201b60201c565b5080806001019150506100ab565b506101577f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e825f815181106101445761014361073e565b5b602002602001015161019e60201b60201c565b507fa2df7830e0bedef7b1116bf547b467b16b50b3bd23146c9e099878d14e89301a6003546002543360405161018f9392919061077a565b60405180910390a150506107af565b5f6101af83836101b760201b60201c565b905092915050565b5f5f6101c9848461020060201b60201c565b905080156101f6576101f48360015f8781526020019081526020015f206102f560201b90919060201c565b505b8091505092915050565b5f610211838361032860201b60201c565b6102eb5760015f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555061028861038b60201b60201c565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a4600190506102ef565b5f90505b92915050565b5f610320835f018373ffffffffffffffffffffffffffffffffffffffff165f1b61039260201b60201c565b905092915050565b5f5f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f33905090565b5f6103a383836103ff60201b60201c565b6103f557825f0182908060018154018082558091505060019003905f5260205f20015f9091909190915055825f0180549050836001015f8481526020019081526020015f2081905550600190506103f9565b5f90505b92915050565b5f5f836001015f8481526020019081526020015f20541415905092915050565b5f604051905090565b5f5ffd5b5f5ffd5b5f819050919050565b61044281610430565b811461044c575f5ffd5b50565b5f8151905061045d81610439565b92915050565b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6104ad82610467565b810181811067ffffffffffffffff821117156104cc576104cb610477565b5b80604052505050565b5f6104de61041f565b90506104ea82826104a4565b919050565b5f67ffffffffffffffff82111561050957610508610477565b5b602082029050602081019050919050565b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6105478261051e565b9050919050565b6105578161053d565b8114610561575f5ffd5b50565b5f815190506105728161054e565b92915050565b5f61058a610585846104ef565b6104d5565b905080838252602082019050602084028301858111156105ad576105ac61051a565b5b835b818110156105d657806105c28882610564565b8452602084019350506020810190506105af565b5050509392505050565b5f82601f8301126105f4576105f3610463565b5b8151610604848260208601610578565b91505092915050565b5f5f6040838503121561062357610622610428565b5b5f6106308582860161044f565b925050602083015167ffffffffffffffff8111156106515761065061042c565b5b61065d858286016105e0565b9150509250929050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61069e82610430565b91506106a983610430565b92508282026106b781610430565b915082820484148315176106ce576106cd610667565b5b5092915050565b5f6106df82610430565b91506106ea83610430565b925082820190508082111561070257610701610667565b5b92915050565b61071181610430565b82525050565b5f60408201905061072a5f830185610708565b6107376020830184610708565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b6107748161053d565b82525050565b5f60608201905061078d5f830186610708565b61079a6020830185610708565b6107a7604083018461076b565b949350505050565b61129e806107bc5f395ff3fe608060405234801561000f575f5ffd5b50600436106100fe575f3560e01c80637f35b56011610095578063a3246ad311610064578063a3246ad3146102c0578063ca15c873146102f0578063d547741f14610320578063fc78b2e81461033c576100fe565b80637f35b560146102245780639010d07c1461024257806391d1485414610272578063a217fddf146102a2576100fe565b806330104c3e116100d157806330104c3e146101ae57806336568abe146101cc57806349f2ada0146101e857806358df0d0114610206576100fe565b806301ffc9a71461010257806313ff6dd514610132578063248a9ca3146101625780632f2ff15d14610192575b5f5ffd5b61011c60048036038101906101179190610e48565b61036c565b6040516101299190610e8d565b60405180910390f35b61014c60048036038101906101479190610f00565b6103e5565b6040516101599190610e8d565b60405180910390f35b61017c60048036038101906101779190610f5e565b610428565b6040516101899190610f98565b60405180910390f35b6101ac60048036038101906101a79190610fb1565b610444565b005b6101b6610486565b6040516101c39190610f98565b60405180910390f35b6101e660048036038101906101e19190610fb1565b6104aa565b005b6101f06104c0565b6040516101fd9190610f98565b60405180910390f35b61020e6104e4565b60405161021b9190610f98565b60405180910390f35b61022c610508565b6040516102399190610f98565b60405180910390f35b61025c60048036038101906102579190611022565b61052c565b604051610269919061106f565b60405180910390f35b61028c60048036038101906102879190610fb1565b610558565b6040516102999190610e8d565b60405180910390f35b6102aa6105bb565b6040516102b79190610f98565b60405180910390f35b6102da60048036038101906102d59190610f5e565b6105c1565b6040516102e7919061113f565b60405180910390f35b61030a60048036038101906103059190610f5e565b6105e3565b604051610317919061116e565b60405180910390f35b61033a60048036038101906103359190610fb1565b610604565b005b61035660048036038101906103519190610f00565b610646565b6040516103639190610e8d565b60405180910390f35b5f7f07effe0d000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191614806103de57506103dd82610678565b5b9050919050565b5f6103ef82610646565b801561042157506104207f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e83610558565b5b9050919050565b5f5f5f8381526020019081526020015f20600101549050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61046e816106f1565b610476610705565b6104808383610707565b50505050565b7fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46981565b6104b2610705565b6104bc828261071a565b5050565b7fd68e3e5e367fee47ce11a5dab404596980e158eb9069330a8f775de7dc6bb64d81565b7f601a28e6abeea52b0ba15b54f6d15d789fd3e8c0080c042a009a9964f0e18f1c81565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e81565b5f6105508260015f8681526020019081526020015f2061079590919063ffffffff16565b905092915050565b5f5f5f8481526020019081526020015f205f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f5f1b81565b60606105dc60015f8481526020019081526020015f206107ac565b9050919050565b5f6105fd60015f8481526020019081526020015f206107cb565b9050919050565b7f1fa0f8d8c153d9da6a47e777288191058d53f440ebdef2b49bcaba73e298ac4e61062e816106f1565b610636610705565b61064083836107de565b50505050565b5f6106717fb922200089753d631c36b6eac46c115660e057765c8df5c796796636265fc46983610558565b9050919050565b5f7f5a05180f000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191614806106ea57506106e9826107f1565b5b9050919050565b610702816106fd61086a565b610871565b50565b565b5f61071283836108c2565b905092915050565b61072261086a565b73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614610786576040517f6697b23200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61079082826107de565b505050565b5f6107a2835f0183610905565b5f1c905092915050565b60605f6107ba835f0161092c565b905060608190508092505050919050565b5f6107d7825f01610985565b9050919050565b5f6107e98383610994565b905092915050565b5f7f7965db0b000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191614806108635750610862826109d7565b5b9050919050565b5f33905090565b61087b8282610558565b6108be5780826040517fe2517d3f0000000000000000000000000000000000000000000000000000000081526004016108b5929190611187565b60405180910390fd5b5050565b5f5f6108ce8484610a40565b905080156108fb576108f98360015f8781526020019081526020015f20610b2990919063ffffffff16565b505b8091505092915050565b5f825f01828154811061091b5761091a6111ae565b5b905f5260205f200154905092915050565b6060815f0180548060200260200160405190810160405280929190818152602001828054801561097957602002820191905f5260205f20905b815481526020019060010190808311610965575b50505050509050919050565b5f815f01805490509050919050565b5f5f6109a08484610b56565b905080156109cd576109cb8360015f8781526020019081526020015f20610c3f90919063ffffffff16565b505b8091505092915050565b5f7f01ffc9a7000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916827bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916149050919050565b5f610a4b8383610558565b610b1f5760015f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550610abc61086a565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a460019050610b23565b5f90505b92915050565b5f610b4e835f018373ffffffffffffffffffffffffffffffffffffffff165f1b610c6c565b905092915050565b5f610b618383610558565b15610c35575f5f5f8581526020019081526020015f205f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550610bd261086a565b73ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16847ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b60405160405180910390a460019050610c39565b5f90505b92915050565b5f610c64835f018373ffffffffffffffffffffffffffffffffffffffff165f1b610cd3565b905092915050565b5f610c778383610dcf565b610cc957825f0182908060018154018082558091505060019003905f5260205f20015f9091909190915055825f0180549050836001015f8481526020019081526020015f208190555060019050610ccd565b5f90505b92915050565b5f5f836001015f8481526020019081526020015f205490505f8114610dc4575f600182610d009190611208565b90505f6001865f0180549050610d169190611208565b9050808214610d7c575f865f018281548110610d3557610d346111ae565b5b905f5260205f200154905080875f018481548110610d5657610d556111ae565b5b905f5260205f20018190555083876001015f8381526020019081526020015f2081905550505b855f01805480610d8f57610d8e61123b565b5b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f905560019350505050610dc9565b5f9150505b92915050565b5f5f836001015f8481526020019081526020015f20541415905092915050565b5f5ffd5b5f7fffffffff0000000000000000000000000000000000000000000000000000000082169050919050565b610e2781610df3565b8114610e31575f5ffd5b50565b5f81359050610e4281610e1e565b92915050565b5f60208284031215610e5d57610e5c610def565b5b5f610e6a84828501610e34565b91505092915050565b5f8115159050919050565b610e8781610e73565b82525050565b5f602082019050610ea05f830184610e7e565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610ecf82610ea6565b9050919050565b610edf81610ec5565b8114610ee9575f5ffd5b50565b5f81359050610efa81610ed6565b92915050565b5f60208284031215610f1557610f14610def565b5b5f610f2284828501610eec565b91505092915050565b5f819050919050565b610f3d81610f2b565b8114610f47575f5ffd5b50565b5f81359050610f5881610f34565b92915050565b5f60208284031215610f7357610f72610def565b5b5f610f8084828501610f4a565b91505092915050565b610f9281610f2b565b82525050565b5f602082019050610fab5f830184610f89565b92915050565b5f5f60408385031215610fc757610fc6610def565b5b5f610fd485828601610f4a565b9250506020610fe585828601610eec565b9150509250929050565b5f819050919050565b61100181610fef565b811461100b575f5ffd5b50565b5f8135905061101c81610ff8565b92915050565b5f5f6040838503121561103857611037610def565b5b5f61104585828601610f4a565b92505060206110568582860161100e565b9150509250929050565b61106981610ec5565b82525050565b5f6020820190506110825f830184611060565b92915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b6110ba81610ec5565b82525050565b5f6110cb83836110b1565b60208301905092915050565b5f602082019050919050565b5f6110ed82611088565b6110f78185611092565b9350611102836110a2565b805f5b8381101561113257815161111988826110c0565b9750611124836110d7565b925050600181019050611105565b5085935050505092915050565b5f6020820190508181035f83015261115781846110e3565b905092915050565b61116881610fef565b82525050565b5f6020820190506111815f83018461115f565b92915050565b5f60408201905061119a5f830185611060565b6111a76020830184610f89565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61121282610fef565b915061121d83610fef565b9250828203905081811115611235576112346111db565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffdfea26469706673582212203fe44ace55c90e3cd17d0c6780b937fac6f484e546ef974c552a23fa0924a6ae64736f6c63430008210033a264697066735822122078d8bd34b57a6b9a98061819254f2f432cde80ec5eac8a3b6d79d28214b50e5064736f6c63430008210033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\xA7W_5`\xE0\x1C\x80c\x85\"l\x81\x11a\0\xF7W\x80c\xCD\t\x9Ej\x11a\0\x95W\x80c\xE0PQ\x99\x11a\0oW\x80c\xE0PQ\x99\x14a\x03mW\x80c\xE2\x0C\x9Fq\x14a\x03wW\x80c\xE4\x9E\x9F\xF5\x14a\x03\x95W\x80c\xFAv&\xD4\x14a\x03\x9FWa\x01\xA7V[\x80c\xCD\t\x9Ej\x14a\x03OW\x80c\xD3\xFE\x9E}\x14a\x03YW\x80c\xD7\xC3\xE52\x14a\x03cWa\x01\xA7V[\x80c\xB0FO\xDC\x11a\0\xD1W\x80c\xB0FO\xDC\x14a\x02\xEBW\x80c\xB5P\x8A\xA9\x14a\x03\tW\x80c\xBA,\xBD5\x14a\x03'W\x80c\xBAAO\xA6\x14a\x031Wa\x01\xA7V[\x80c\x85\"l\x81\x14a\x02\xA5W\x80c\x91j\x17\xC6\x14a\x02\xC3W\x80c\x9B\xC8\xA6\x19\x14a\x02\xE1Wa\x01\xA7V[\x80c>^<#\x11a\x01dW\x80cT\xB7\xC3\x9B\x11a\x01>W\x80cT\xB7\xC3\x9B\x14a\x02UW\x80c\\\x97+\xE5\x14a\x02sW\x80ce\xC9\xF4\x85\x14a\x02}W\x80cf\xD9\xA9\xA0\x14a\x02\x87Wa\x01\xA7V[\x80c>^<#\x14a\x02\x0FW\x80c?r\x86\xF4\x14a\x02-W\x80cHK?6\x14a\x02KWa\x01\xA7V[\x80c\n\x92T\xE4\x14a\x01\xABW\x80c\x1E\xD7\x83\x1C\x14a\x01\xB5W\x80c \nV\xCE\x14a\x01\xD3W\x80c*\xDE8\x80\x14a\x01\xDDW\x80c7M\x1F\xD5\x14a\x01\xFBW\x80c:^O#\x14a\x02\x05W[__\xFD[a\x01\xB3a\x03\xBDV[\0[a\x01\xBDa\x07jV[`@Qa\x01\xCA\x91\x90a9\x8AV[`@Q\x80\x91\x03\x90\xF3[a\x01\xDBa\x07\xF5V[\0[a\x01\xE5a\ntV[`@Qa\x01\xF2\x91\x90a;\xCAV[`@Q\x80\x91\x03\x90\xF3[a\x02\x03a\x0B\xF8V[\0[a\x02\ra\x122V[\0[a\x02\x17a\x13\xD6V[`@Qa\x02$\x91\x90a9\x8AV[`@Q\x80\x91\x03\x90\xF3[a\x025a\x14aV[`@Qa\x02B\x91\x90a9\x8AV[`@Q\x80\x91\x03\x90\xF3[a\x02Sa\x14\xECV[\0[a\x02]a\x16\xFDV[`@Qa\x02j\x91\x90a<EV[`@Q\x80\x91\x03\x90\xF3[a\x02{a\x17#V[\0[a\x02\x85a\x194V[\0[a\x02\x8Fa\x1A\xEDV[`@Qa\x02\x9C\x91\x90a><V[`@Q\x80\x91\x03\x90\xF3[a\x02\xADa\x1CoV[`@Qa\x02\xBA\x91\x90a>\xDFV[`@Q\x80\x91\x03\x90\xF3[a\x02\xCBa\x1DCV[`@Qa\x02\xD8\x91\x90a?\xF4V[`@Q\x80\x91\x03\x90\xF3[a\x02\xE9a\x1E\x8AV[\0[a\x02\xF3a \xF1V[`@Qa\x03\0\x91\x90a?\xF4V[`@Q\x80\x91\x03\x90\xF3[a\x03\x11a\"8V[`@Qa\x03\x1E\x91\x90a>\xDFV[`@Q\x80\x91\x03\x90\xF3[a\x03/a#\x0CV[\0[a\x039a%\xFCV[`@Qa\x03F\x91\x90a@.V[`@Q\x80\x91\x03\x90\xF3[a\x03Wa'\x03V[\0[a\x03aa)\x92V[\0[a\x03ka,-V[\0[a\x03ua.YV[\0[a\x03\x7Fa0\xB7V[`@Qa\x03\x8C\x91\x90a9\x8AV[`@Q\x80\x91\x03\x90\xF3[a\x03\x9Da1BV[\0[a\x03\xA7a5bV[`@Qa\x03\xB4\x91\x90a@.V[`@Q\x80\x91\x03\x90\xF3[_`\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xD9Wa\x03\xD8a@GV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x04\x07W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P` _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x81Q\x81\x10a\x04?Wa\x04>a@tV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x01\x81Q\x81\x10a\x04\xAFWa\x04\xAEa@tV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`\"_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x02\x81Q\x81\x10a\x05\x1FWa\x05\x1Ea@tV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`#_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x03\x81Q\x81\x10a\x05\x8FWa\x05\x8Ea@tV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`\x01\x81`@Qa\x05\xD8\x90a8\x96V[a\x05\xE3\x92\x91\x90a@\xE3V[`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x05\xFCW=__>=_\xFD[P`\x1F`\x01a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c0\x10L>`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xA8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xCC\x91\x90aAHV[`$\x81\x90UP`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x7F5\xB5``@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07=W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07a\x91\x90aAHV[`%\x81\x90UPPV[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07\xEBW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x07\xA2W[PPPPP\x90P\x90V[_a\x084`@Q\x80`@\x01`@R\x80`\t\x81R` \x01\x7FNEW_PARTY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa5tV[\x90Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCAf\x9F\xA7`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\xA4\x91\x90aA\x82V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08\xBBW__\xFD[PZ\xF1\x15\x80\x15a\x08\xCDW=__>=_\xFD[PPPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF2\x8D\xCE\xB3c\xE2Q}?`\xE0\x1B`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`%T`@Q`$\x01a\t?\x92\x91\x90aA\xAAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\xB8\x91\x90aB#V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\t\xCFW__\xFD[PZ\xF1\x15\x80\x15a\t\xE1W=__>=_\xFD[PPPP`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c//\xF1]`$T\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\nD\x92\x91\x90aBCV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\n[W__\xFD[PZ\xF1\x15\x80\x15a\nmW=__>=_\xFD[PPPPPV[```\x1E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0B\xEFW\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0B\xD8W\x83\x82\x90_R` _ \x01\x80Ta\x0BM\x90aB\x97V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0By\x90aB\x97V[\x80\x15a\x0B\xC4W\x80`\x1F\x10a\x0B\x9BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xC4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xA7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x0B0V[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\n\x97V[PPPP\x90P\x90V[a\x0C\xBF`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xD1HT`$T` _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C{\x92\x91\x90aBCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x96W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xBA\x91\x90aB\xF1V[a5\x88V[a\r\x86`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xD1HT`$T`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\rB\x92\x91\x90aBCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r]W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x81\x91\x90aB\xF1V[a5\x88V[a\x0EM`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xD1HT`$T`\"_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\t\x92\x91\x90aBCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E$W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EH\x91\x90aB\xF1V[a5\x88V[a\x0F\x14`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xD1HT`$T`#_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xD0\x92\x91\x90aBCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xEBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x0F\x91\x90aB\xF1V[a5\x88V[a\x0F\xDB`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xD1HT`%T` _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\x97\x92\x91\x90aBCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xB2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xD6\x91\x90aB\xF1V[a5\x88V[a\x10\xA2`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xD1HT`%T`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10^\x92\x91\x90aBCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10yW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x9D\x91\x90aB\xF1V[a6\x18V[a\x11i`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xD1HT`%T`\"_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11%\x92\x91\x90aBCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11@W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11d\x91\x90aB\xF1V[a6\x18V[a\x120`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xD1HT`%T`#_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\xEC\x92\x91\x90aBCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x07W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12+\x91\x90aB\xF1V[a6\x18V[V[a\x12\xF5`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xFCx\xB2\xE8`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\xB1\x91\x90aA\x82V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xCCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xF0\x91\x90aB\xF1V[a5\x88V[a\x13\xD4`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xFCx\xB2\xE8a\x13t`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01\x7FSTRANGER\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa5tV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\x90\x91\x90aA\x82V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xABW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xCF\x91\x90aB\xF1V[a6\x18V[V[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x14WW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x14\x0EW[PPPPP\x90P\x90V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x14\xE2W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x14\x99W[PPPPP\x90P\x90V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCAf\x9F\xA7`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15Z\x91\x90aA\x82V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x15qW__\xFD[PZ\xF1\x15\x80\x15a\x15\x83W=__>=_\xFD[PPPP`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c6V\x8A\xBE`$T`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\x07\x92\x91\x90aBCV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x16\x1EW__\xFD[PZ\xF1\x15\x80\x15a\x160W=__>=_\xFD[PPPPa\x16\xFB`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xD1HT`$T`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\xB7\x92\x91\x90aBCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xD2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xF6\x91\x90aB\xF1V[a6\x18V[V[`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCAf\x9F\xA7` _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\x91\x91\x90aA\x82V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x17\xA8W__\xFD[PZ\xF1\x15\x80\x15a\x17\xBAW=__>=_\xFD[PPPP`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD5Gt\x1F`$T`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18>\x92\x91\x90aBCV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x18UW__\xFD[PZ\xF1\x15\x80\x15a\x18gW=__>=_\xFD[PPPPa\x192`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xD1HT`$T`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\xEE\x92\x91\x90aBCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\tW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19-\x91\x90aB\xF1V[a6\x18V[V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCAf\x9F\xA7`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\xA2\x91\x90aA\x82V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x19\xB9W__\xFD[PZ\xF1\x15\x80\x15a\x19\xCBW=__>=_\xFD[PPPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF4\x84H\x14`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1A(W__\xFD[PZ\xF1\x15\x80\x15a\x1A:W=__>=_\xFD[PPPP`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c6V\x8A\xBE`$T`\"_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A\xBE\x92\x91\x90aBCV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1A\xD5W__\xFD[PZ\xF1\x15\x80\x15a\x1A\xE7W=__>=_\xFD[PPPPV[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1CfW\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x1B@\x90aB\x97V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1Bl\x90aB\x97V[\x80\x15a\x1B\xB7W\x80`\x1F\x10a\x1B\x8EWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1B\xB7V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1B\x9AW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1CNW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x1B\xFBW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1B\x10V[PPPP\x90P\x90V[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1D:W\x83\x82\x90_R` _ \x01\x80Ta\x1C\xAF\x90aB\x97V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1C\xDB\x90aB\x97V[\x80\x15a\x1D&W\x80`\x1F\x10a\x1C\xFDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1D&V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1D\tW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x1C\x92V[PPPP\x90P\x90V[```\x1D\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1E\x81W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1EiW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x1E\x16W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1DfV[PPPP\x90P\x90V[a\x1FM`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x13\xFFm\xD5` _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1F\t\x91\x90aA\x82V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F$W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1FH\x91\x90aB\xF1V[a5\x88V[a \x10`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x13\xFFm\xD5`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1F\xCC\x91\x90aA\x82V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xE7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \x0B\x91\x90aB\xF1V[a6\x18V[a \xEF`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x13\xFFm\xD5a \x8F`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01\x7FSTRANGER\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa5tV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a \xAB\x91\x90aA\x82V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xC6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xEA\x91\x90aB\xF1V[a6\x18V[V[```\x1C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\"/W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\"\x17W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a!\xC4W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a!\x14V[PPPP\x90P\x90V[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a#\x03W\x83\x82\x90_R` _ \x01\x80Ta\"x\x90aB\x97V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\"\xA4\x90aB\x97V[\x80\x15a\"\xEFW\x80`\x1F\x10a\"\xC6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\"\xEFV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\"\xD2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\"[V[PPPP\x90P\x90V[_`\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#(Wa#'a@GV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a#VW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P` _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x81Q\x81\x10a#\x8EWa#\x8Da@tV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x01\x81Q\x81\x10a#\xFEWa#\xFDa@tV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`\"_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x02\x81Q\x81\x10a$nWa$ma@tV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP`#_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\x03\x81Q\x81\x10a$\xDEWa$\xDDa@tV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cD\x0E\xD1\r`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a%qW__\xFD[PZ\xF1\x15\x80\x15a%\x83W=__>=_\xFD[PPPP\x7F\xA2\xDFx0\xE0\xBE\xDE\xF7\xB1\x11k\xF5G\xB4g\xB1kP\xB3\xBD#\x14l\x9E\t\x98x\xD1N\x890\x1A`\x04`\x010`@Qa%\xBC\x93\x92\x91\x90aCUV[`@Q\x80\x91\x03\x90\xA1`\x01\x81`@Qa%\xD3\x90a8\x96V[a%\xDE\x92\x91\x90a@\xE3V[`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a%\xF7W=__>=_\xFD[PPPV[_`\x08_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a&\x1AW`\x01\x90Pa'\0V[__\x1B\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cf\x7F\x9Dp\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1C\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a&\xBC\x92\x91\x90aA\xAAV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\xD7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xFB\x91\x90aAHV[\x14\x15\x90P[\x90V[_`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCA\x15\xC8s`$T`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a'a\x91\x90aC\x8AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'|W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\xA0\x91\x90aC\xCDV[\x90Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCAf\x9F\xA7` _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a(\x10\x91\x90aA\x82V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a('W__\xFD[PZ\xF1\x15\x80\x15a(9W=__>=_\xFD[PPPP`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c//\xF1]`$T`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a(\xBD\x92\x91\x90aBCV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a(\xD4W__\xFD[PZ\xF1\x15\x80\x15a(\xE6W=__>=_\xFD[PPPPa)\x8F`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCA\x15\xC8s`$T`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a)J\x91\x90aC\x8AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)eW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\x89\x91\x90aC\xCDV[\x82a6\xA9V[PV[_`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCA\x15\xC8s`$T`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a)\xF0\x91\x90aC\x8AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\x0BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*/\x91\x90aC\xCDV[\x90Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCAf\x9F\xA7` _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a*\x9F\x91\x90aA\x82V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a*\xB6W__\xFD[PZ\xF1\x15\x80\x15a*\xC8W=__>=_\xFD[PPPP`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD5Gt\x1F`$T`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+L\x92\x91\x90aBCV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a+cW__\xFD[PZ\xF1\x15\x80\x15a+uW=__>=_\xFD[PPPPa,*`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCA\x15\xC8s`$T`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+\xD9\x91\x90aC\x8AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\xF4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\x18\x91\x90aC\xCDV[`\x01\x83a,%\x91\x90aD%V[a6\xA9V[PV[a,\xEF`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x01\xFF\xC9\xA7\x7F\x07\xEF\xFE\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\xAB\x91\x90aDgV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xC6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xEA\x91\x90aB\xF1V[a5\x88V[a-\xB1`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x01\xFF\xC9\xA7\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a-m\x91\x90aDgV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\x88W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\xAC\x91\x90aB\xF1V[a5\x88V[a.W`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x01\xFF\xC9\xA7c\xDE\xAD\xBE\xEF`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a.\x13\x91\x90aD\xC5V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a..W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.R\x91\x90aB\xF1V[a6\x18V[V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCAf\x9F\xA7`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a.\xC7\x91\x90aA\x82V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a.\xDEW__\xFD[PZ\xF1\x15\x80\x15a.\xF0W=__>=_\xFD[PPPPsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF2\x8D\xCE\xB3c\xE2Q}?`\xE0\x1B`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`%T`@Q`$\x01a/b\x92\x91\x90aA\xAAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a/\xDB\x91\x90aB#V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a/\xF2W__\xFD[PZ\xF1\x15\x80\x15a0\x04W=__>=_\xFD[PPPP`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD5Gt\x1F`$T`\"_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a0\x88\x92\x91\x90aBCV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a0\x9FW__\xFD[PZ\xF1\x15\x80\x15a0\xB1W=__>=_\xFD[PPPPV[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a18W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a0\xEFW[PPPPP\x90P\x90V[_a1\x81`@Q\x80`@\x01`@R\x80`\t\x81R` \x01\x7FNEW_PARTY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa5tV[\x90Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCAf\x9F\xA7` _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a1\xF1\x91\x90aA\x82V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a2\x08W__\xFD[PZ\xF1\x15\x80\x15a2\x1AW=__>=_\xFD[PPPP`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c//\xF1]`$T\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a2}\x92\x91\x90aBCV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a2\x94W__\xFD[PZ\xF1\x15\x80\x15a2\xA6W=__>=_\xFD[PPPPa3P`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xD1HT`$T\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a3\x0C\x92\x91\x90aBCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3'W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3K\x91\x90aB\xF1V[a5\x88V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCAf\x9F\xA7` _\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a3\xBE\x91\x90aA\x82V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a3\xD5W__\xFD[PZ\xF1\x15\x80\x15a3\xE7W=__>=_\xFD[PPPP`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c//\xF1]`%T`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a4k\x92\x91\x90aBCV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a4\x82W__\xFD[PZ\xF1\x15\x80\x15a4\x94W=__>=_\xFD[PPPPa5_`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x91\xD1HT`%T`!_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a5\x1B\x92\x91\x90aBCV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a56W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5Z\x91\x90aB\xF1V[a5\x88V[PV[`\x1F_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[_a5~\x82a7>V[P\x80\x91PP\x91\x90PV[\x80a6\x15W\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x0C\x9F\xD5\x81\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a5\xE8\x91\x90a@.V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a5\xFEW__\xFD[PZ\xFA\x15\x80\x15a6\x10W=__>=_\xFD[PPPP[PV[\x80\x15a6\xA6W\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA5\x98(\x85\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a6y\x91\x90a@.V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a6\x8FW__\xFD[PZ\xFA\x15\x80\x15a6\xA1W=__>=_\xFD[PPPP[PV[\x80\x82\x14a7:W\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x98)lT\x83\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a7\r\x92\x91\x90aD\xEDV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a7#W__\xFD[PZ\xFA\x15\x80\x15a75W=__>=_\xFD[PPPP[PPV[__\x82`@Q` \x01a7Q\x91\x90aENV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1C\x90P\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xFF\xA1\x86I\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a7\xC6\x91\x90aEdV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\xE1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\x05\x91\x90aE\xA7V[\x91P\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC6W\xC7\x18\x83\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a8d\x92\x91\x90aF\x1AV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a8{W__\xFD[PZ\xF1\x15\x80\x15a8\x8DW=__>=_\xFD[PPPP\x91P\x91V[a\x1AZ\x80aFI\x839\x01\x90V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a8\xF5\x82a8\xCCV[\x90P\x91\x90PV[a9\x05\x81a8\xEBV[\x82RPPV[_a9\x16\x83\x83a8\xFCV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a98\x82a8\xA3V[a9B\x81\x85a8\xADV[\x93Pa9M\x83a8\xBDV[\x80_[\x83\x81\x10\x15a9}W\x81Qa9d\x88\x82a9\x0BV[\x97Pa9o\x83a9\"V[\x92PP`\x01\x81\x01\x90Pa9PV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra9\xA2\x81\x84a9.V[\x90P\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a:>\x82a9\xFCV[a:H\x81\x85a:\x06V[\x93Pa:X\x81\x85` \x86\x01a:\x16V[a:a\x81a:$V[\x84\x01\x91PP\x92\x91PPV[_a:w\x83\x83a:4V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a:\x95\x82a9\xD3V[a:\x9F\x81\x85a9\xDDV[\x93P\x83` \x82\x02\x85\x01a:\xB1\x85a9\xEDV[\x80_[\x85\x81\x10\x15a:\xECW\x84\x84\x03\x89R\x81Qa:\xCD\x85\x82a:lV[\x94Pa:\xD8\x83a:\x7FV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa:\xB4V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`@\x83\x01_\x83\x01Qa;\x13_\x86\x01\x82a8\xFCV[P` \x83\x01Q\x84\x82\x03` \x86\x01Ra;+\x82\x82a:\x8BV[\x91PP\x80\x91PP\x92\x91PPV[_a;C\x83\x83a:\xFEV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a;a\x82a9\xAAV[a;k\x81\x85a9\xB4V[\x93P\x83` \x82\x02\x85\x01a;}\x85a9\xC4V[\x80_[\x85\x81\x10\x15a;\xB8W\x84\x84\x03\x89R\x81Qa;\x99\x85\x82a;8V[\x94Pa;\xA4\x83a;KV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa;\x80V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra;\xE2\x81\x84a;WV[\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[_a<\ra<\x08a<\x03\x84a8\xCCV[a;\xEAV[a8\xCCV[\x90P\x91\x90PV[_a<\x1E\x82a;\xF3V[\x90P\x91\x90PV[_a</\x82a<\x14V[\x90P\x91\x90PV[a<?\x81a<%V[\x82RPPV[_` \x82\x01\x90Pa<X_\x83\x01\x84a<6V[\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a<\xE4\x81a<\xB0V[\x82RPPV[_a<\xF5\x83\x83a<\xDBV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a=\x17\x82a<\x87V[a=!\x81\x85a<\x91V[\x93Pa=,\x83a<\xA1V[\x80_[\x83\x81\x10\x15a=\\W\x81Qa=C\x88\x82a<\xEAV[\x97Pa=N\x83a=\x01V[\x92PP`\x01\x81\x01\x90Pa=/V[P\x85\x93PPPP\x92\x91PPV[_`@\x83\x01_\x83\x01Q\x84\x82\x03_\x86\x01Ra=\x83\x82\x82a:4V[\x91PP` \x83\x01Q\x84\x82\x03` \x86\x01Ra=\x9D\x82\x82a=\rV[\x91PP\x80\x91PP\x92\x91PPV[_a=\xB5\x83\x83a=iV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a=\xD3\x82a<^V[a=\xDD\x81\x85a<hV[\x93P\x83` \x82\x02\x85\x01a=\xEF\x85a<xV[\x80_[\x85\x81\x10\x15a>*W\x84\x84\x03\x89R\x81Qa>\x0B\x85\x82a=\xAAV[\x94Pa>\x16\x83a=\xBDV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa=\xF2V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra>T\x81\x84a=\xC9V[\x90P\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a>v\x82a9\xD3V[a>\x80\x81\x85a>\\V[\x93P\x83` \x82\x02\x85\x01a>\x92\x85a9\xEDV[\x80_[\x85\x81\x10\x15a>\xCDW\x84\x84\x03\x89R\x81Qa>\xAE\x85\x82a:lV[\x94Pa>\xB9\x83a:\x7FV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa>\x95V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra>\xF7\x81\x84a>lV[\x90P\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_`@\x83\x01_\x83\x01Qa?=_\x86\x01\x82a8\xFCV[P` \x83\x01Q\x84\x82\x03` \x86\x01Ra?U\x82\x82a=\rV[\x91PP\x80\x91PP\x92\x91PPV[_a?m\x83\x83a?(V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a?\x8B\x82a>\xFFV[a?\x95\x81\x85a?\tV[\x93P\x83` \x82\x02\x85\x01a?\xA7\x85a?\x19V[\x80_[\x85\x81\x10\x15a?\xE2W\x84\x84\x03\x89R\x81Qa?\xC3\x85\x82a?bV[\x94Pa?\xCE\x83a?uV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa?\xAAV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra@\x0C\x81\x84a?\x81V[\x90P\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a@(\x81a@\x14V[\x82RPPV[_` \x82\x01\x90Pa@A_\x83\x01\x84a@\x1FV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a@\xCDa@\xC8a@\xC3\x84a@\xA1V[a;\xEAV[a@\xAAV[\x90P\x91\x90PV[a@\xDD\x81a@\xB3V[\x82RPPV[_`@\x82\x01\x90Pa@\xF6_\x83\x01\x85a@\xD4V[\x81\x81\x03` \x83\x01RaA\x08\x81\x84a9.V[\x90P\x93\x92PPPV[__\xFD[_\x81\x90P\x91\x90PV[aA'\x81aA\x15V[\x81\x14aA1W__\xFD[PV[_\x81Q\x90PaAB\x81aA\x1EV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aA]WaA\\aA\x11V[[_aAj\x84\x82\x85\x01aA4V[\x91PP\x92\x91PPV[aA|\x81a8\xEBV[\x82RPPV[_` \x82\x01\x90PaA\x95_\x83\x01\x84aAsV[\x92\x91PPV[aA\xA4\x81aA\x15V[\x82RPPV[_`@\x82\x01\x90PaA\xBD_\x83\x01\x85aAsV[aA\xCA` \x83\x01\x84aA\x9BV[\x93\x92PPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aA\xF5\x82aA\xD1V[aA\xFF\x81\x85aA\xDBV[\x93PaB\x0F\x81\x85` \x86\x01a:\x16V[aB\x18\x81a:$V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaB;\x81\x84aA\xEBV[\x90P\x92\x91PPV[_`@\x82\x01\x90PaBV_\x83\x01\x85aA\x9BV[aBc` \x83\x01\x84aAsV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80aB\xAEW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aB\xC1WaB\xC0aBjV[[P\x91\x90PV[aB\xD0\x81a@\x14V[\x81\x14aB\xDAW__\xFD[PV[_\x81Q\x90PaB\xEB\x81aB\xC7V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aC\x06WaC\x05aA\x11V[[_aC\x13\x84\x82\x85\x01aB\xDDV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_aC?aC:aC5\x84aC\x1CV[a;\xEAV[a@\xAAV[\x90P\x91\x90PV[aCO\x81aC%V[\x82RPPV[_``\x82\x01\x90PaCh_\x83\x01\x86aCFV[aCu` \x83\x01\x85a@\xD4V[aC\x82`@\x83\x01\x84aAsV[\x94\x93PPPPV[_` \x82\x01\x90PaC\x9D_\x83\x01\x84aA\x9BV[\x92\x91PPV[aC\xAC\x81a@\xAAV[\x81\x14aC\xB6W__\xFD[PV[_\x81Q\x90PaC\xC7\x81aC\xA3V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aC\xE2WaC\xE1aA\x11V[[_aC\xEF\x84\x82\x85\x01aC\xB9V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_aD/\x82a@\xAAV[\x91PaD:\x83a@\xAAV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15aDRWaDQaC\xF8V[[\x92\x91PPV[aDa\x81a<\xB0V[\x82RPPV[_` \x82\x01\x90PaDz_\x83\x01\x84aDXV[\x92\x91PPV[_\x81\x90P\x91\x90PV[_\x81`\xE0\x1B\x90P\x91\x90PV[_aD\xAFaD\xAAaD\xA5\x84aD\x80V[aD\x89V[a<\xB0V[\x90P\x91\x90PV[aD\xBF\x81aD\x95V[\x82RPPV[_` \x82\x01\x90PaD\xD8_\x83\x01\x84aD\xB6V[\x92\x91PPV[aD\xE7\x81a@\xAAV[\x82RPPV[_`@\x82\x01\x90PaE\0_\x83\x01\x85aD\xDEV[aE\r` \x83\x01\x84aD\xDEV[\x93\x92PPPV[_\x81\x90P\x92\x91PPV[_aE(\x82a9\xFCV[aE2\x81\x85aE\x14V[\x93PaEB\x81\x85` \x86\x01a:\x16V[\x80\x84\x01\x91PP\x92\x91PPV[_aEY\x82\x84aE\x1EV[\x91P\x81\x90P\x92\x91PPV[_` \x82\x01\x90PaEw_\x83\x01\x84aD\xDEV[\x92\x91PPV[aE\x86\x81a8\xEBV[\x81\x14aE\x90W__\xFD[PV[_\x81Q\x90PaE\xA1\x81aE}V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aE\xBCWaE\xBBaA\x11V[[_aE\xC9\x84\x82\x85\x01aE\x93V[\x91PP\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aE\xEC\x82a9\xFCV[aE\xF6\x81\x85aE\xD2V[\x93PaF\x06\x81\x85` \x86\x01a:\x16V[aF\x0F\x81a:$V[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90PaF-_\x83\x01\x85aAsV[\x81\x81\x03` \x83\x01RaF?\x81\x84aE\xE2V[\x90P\x93\x92PPPV\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x1AZ8\x03\x80a\x1AZ\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\x06\rV[\x81`\x02\x81\x90UP`\x01`\x02T`\x03a\0I\x91\x90a\x06\x94V[a\0S\x91\x90a\x06\xD5V[`\x03\x81\x90UP`\x03T\x81Q\x10\x15a\0\xA6W\x80Q`\x03T`@Q\x7F:#bh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\0\x9D\x92\x91\x90a\x07\x17V[`@Q\x80\x91\x03\x90\xFD[__\x90P[\x81Q\x81\x10\x15a\x01\x0CWa\0\xFE\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x83\x83\x81Q\x81\x10a\0\xEBWa\0\xEAa\x07>V[[` \x02` \x01\x01Qa\x01\x9E` \x1B` \x1CV[P\x80\x80`\x01\x01\x91PPa\0\xABV[Pa\x01W\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x82_\x81Q\x81\x10a\x01DWa\x01Ca\x07>V[[` \x02` \x01\x01Qa\x01\x9E` \x1B` \x1CV[P\x7F\xA2\xDFx0\xE0\xBE\xDE\xF7\xB1\x11k\xF5G\xB4g\xB1kP\xB3\xBD#\x14l\x9E\t\x98x\xD1N\x890\x1A`\x03T`\x02T3`@Qa\x01\x8F\x93\x92\x91\x90a\x07zV[`@Q\x80\x91\x03\x90\xA1PPa\x07\xAFV[_a\x01\xAF\x83\x83a\x01\xB7` \x1B` \x1CV[\x90P\x92\x91PPV[__a\x01\xC9\x84\x84a\x02\0` \x1B` \x1CV[\x90P\x80\x15a\x01\xF6Wa\x01\xF4\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a\x02\xF5` \x1B\x90\x91\x90` \x1CV[P[\x80\x91PP\x92\x91PPV[_a\x02\x11\x83\x83a\x03(` \x1B` \x1CV[a\x02\xEBW`\x01__\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x02\x88a\x03\x8B` \x1B` \x1CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\x02\xEFV[_\x90P[\x92\x91PPV[_a\x03 \x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba\x03\x92` \x1B` \x1CV[\x90P\x92\x91PPV[___\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[_3\x90P\x90V[_a\x03\xA3\x83\x83a\x03\xFF` \x1B` \x1CV[a\x03\xF5W\x82_\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x82_\x01\x80T\x90P\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP`\x01\x90Pa\x03\xF9V[_\x90P[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x14\x15\x90P\x92\x91PPV[_`@Q\x90P\x90V[__\xFD[__\xFD[_\x81\x90P\x91\x90PV[a\x04B\x81a\x040V[\x81\x14a\x04LW__\xFD[PV[_\x81Q\x90Pa\x04]\x81a\x049V[\x92\x91PPV[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x04\xAD\x82a\x04gV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x04\xCCWa\x04\xCBa\x04wV[[\x80`@RPPPV[_a\x04\xDEa\x04\x1FV[\x90Pa\x04\xEA\x82\x82a\x04\xA4V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x05\tWa\x05\x08a\x04wV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x05G\x82a\x05\x1EV[\x90P\x91\x90PV[a\x05W\x81a\x05=V[\x81\x14a\x05aW__\xFD[PV[_\x81Q\x90Pa\x05r\x81a\x05NV[\x92\x91PPV[_a\x05\x8Aa\x05\x85\x84a\x04\xEFV[a\x04\xD5V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x05\xADWa\x05\xACa\x05\x1AV[[\x83[\x81\x81\x10\x15a\x05\xD6W\x80a\x05\xC2\x88\x82a\x05dV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x05\xAFV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x05\xF4Wa\x05\xF3a\x04cV[[\x81Qa\x06\x04\x84\x82` \x86\x01a\x05xV[\x91PP\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x06#Wa\x06\"a\x04(V[[_a\x060\x85\x82\x86\x01a\x04OV[\x92PP` \x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06QWa\x06Pa\x04,V[[a\x06]\x85\x82\x86\x01a\x05\xE0V[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x06\x9E\x82a\x040V[\x91Pa\x06\xA9\x83a\x040V[\x92P\x82\x82\x02a\x06\xB7\x81a\x040V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x06\xCEWa\x06\xCDa\x06gV[[P\x92\x91PPV[_a\x06\xDF\x82a\x040V[\x91Pa\x06\xEA\x83a\x040V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x07\x02Wa\x07\x01a\x06gV[[\x92\x91PPV[a\x07\x11\x81a\x040V[\x82RPPV[_`@\x82\x01\x90Pa\x07*_\x83\x01\x85a\x07\x08V[a\x077` \x83\x01\x84a\x07\x08V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[a\x07t\x81a\x05=V[\x82RPPV[_``\x82\x01\x90Pa\x07\x8D_\x83\x01\x86a\x07\x08V[a\x07\x9A` \x83\x01\x85a\x07\x08V[a\x07\xA7`@\x83\x01\x84a\x07kV[\x94\x93PPPPV[a\x12\x9E\x80a\x07\xBC_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xFEW_5`\xE0\x1C\x80c\x7F5\xB5`\x11a\0\x95W\x80c\xA3$j\xD3\x11a\0dW\x80c\xA3$j\xD3\x14a\x02\xC0W\x80c\xCA\x15\xC8s\x14a\x02\xF0W\x80c\xD5Gt\x1F\x14a\x03 W\x80c\xFCx\xB2\xE8\x14a\x03<Wa\0\xFEV[\x80c\x7F5\xB5`\x14a\x02$W\x80c\x90\x10\xD0|\x14a\x02BW\x80c\x91\xD1HT\x14a\x02rW\x80c\xA2\x17\xFD\xDF\x14a\x02\xA2Wa\0\xFEV[\x80c0\x10L>\x11a\0\xD1W\x80c0\x10L>\x14a\x01\xAEW\x80c6V\x8A\xBE\x14a\x01\xCCW\x80cI\xF2\xAD\xA0\x14a\x01\xE8W\x80cX\xDF\r\x01\x14a\x02\x06Wa\0\xFEV[\x80c\x01\xFF\xC9\xA7\x14a\x01\x02W\x80c\x13\xFFm\xD5\x14a\x012W\x80c$\x8A\x9C\xA3\x14a\x01bW\x80c//\xF1]\x14a\x01\x92W[__\xFD[a\x01\x1C`\x04\x806\x03\x81\x01\x90a\x01\x17\x91\x90a\x0EHV[a\x03lV[`@Qa\x01)\x91\x90a\x0E\x8DV[`@Q\x80\x91\x03\x90\xF3[a\x01L`\x04\x806\x03\x81\x01\x90a\x01G\x91\x90a\x0F\0V[a\x03\xE5V[`@Qa\x01Y\x91\x90a\x0E\x8DV[`@Q\x80\x91\x03\x90\xF3[a\x01|`\x04\x806\x03\x81\x01\x90a\x01w\x91\x90a\x0F^V[a\x04(V[`@Qa\x01\x89\x91\x90a\x0F\x98V[`@Q\x80\x91\x03\x90\xF3[a\x01\xAC`\x04\x806\x03\x81\x01\x90a\x01\xA7\x91\x90a\x0F\xB1V[a\x04DV[\0[a\x01\xB6a\x04\x86V[`@Qa\x01\xC3\x91\x90a\x0F\x98V[`@Q\x80\x91\x03\x90\xF3[a\x01\xE6`\x04\x806\x03\x81\x01\x90a\x01\xE1\x91\x90a\x0F\xB1V[a\x04\xAAV[\0[a\x01\xF0a\x04\xC0V[`@Qa\x01\xFD\x91\x90a\x0F\x98V[`@Q\x80\x91\x03\x90\xF3[a\x02\x0Ea\x04\xE4V[`@Qa\x02\x1B\x91\x90a\x0F\x98V[`@Q\x80\x91\x03\x90\xF3[a\x02,a\x05\x08V[`@Qa\x029\x91\x90a\x0F\x98V[`@Q\x80\x91\x03\x90\xF3[a\x02\\`\x04\x806\x03\x81\x01\x90a\x02W\x91\x90a\x10\"V[a\x05,V[`@Qa\x02i\x91\x90a\x10oV[`@Q\x80\x91\x03\x90\xF3[a\x02\x8C`\x04\x806\x03\x81\x01\x90a\x02\x87\x91\x90a\x0F\xB1V[a\x05XV[`@Qa\x02\x99\x91\x90a\x0E\x8DV[`@Q\x80\x91\x03\x90\xF3[a\x02\xAAa\x05\xBBV[`@Qa\x02\xB7\x91\x90a\x0F\x98V[`@Q\x80\x91\x03\x90\xF3[a\x02\xDA`\x04\x806\x03\x81\x01\x90a\x02\xD5\x91\x90a\x0F^V[a\x05\xC1V[`@Qa\x02\xE7\x91\x90a\x11?V[`@Q\x80\x91\x03\x90\xF3[a\x03\n`\x04\x806\x03\x81\x01\x90a\x03\x05\x91\x90a\x0F^V[a\x05\xE3V[`@Qa\x03\x17\x91\x90a\x11nV[`@Q\x80\x91\x03\x90\xF3[a\x03:`\x04\x806\x03\x81\x01\x90a\x035\x91\x90a\x0F\xB1V[a\x06\x04V[\0[a\x03V`\x04\x806\x03\x81\x01\x90a\x03Q\x91\x90a\x0F\0V[a\x06FV[`@Qa\x03c\x91\x90a\x0E\x8DV[`@Q\x80\x91\x03\x90\xF3[_\x7F\x07\xEF\xFE\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x03\xDEWPa\x03\xDD\x82a\x06xV[[\x90P\x91\x90PV[_a\x03\xEF\x82a\x06FV[\x80\x15a\x04!WPa\x04 \x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x83a\x05XV[[\x90P\x91\x90PV[___\x83\x81R` \x01\x90\x81R` \x01_ `\x01\x01T\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x04n\x81a\x06\xF1V[a\x04va\x07\x05V[a\x04\x80\x83\x83a\x07\x07V[PPPPV[\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x81V[a\x04\xB2a\x07\x05V[a\x04\xBC\x82\x82a\x07\x1AV[PPV[\x7F\xD6\x8E>^6\x7F\xEEG\xCE\x11\xA5\xDA\xB4\x04Yi\x80\xE1X\xEB\x90i3\n\x8Fw]\xE7\xDCk\xB6M\x81V[\x7F`\x1A(\xE6\xAB\xEE\xA5+\x0B\xA1[T\xF6\xD1]x\x9F\xD3\xE8\xC0\x08\x0C\x04*\0\x9A\x99d\xF0\xE1\x8F\x1C\x81V[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACN\x81V[_a\x05P\x82`\x01_\x86\x81R` \x01\x90\x81R` \x01_ a\x07\x95\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x92\x91PPV[___\x84\x81R` \x01\x90\x81R` \x01_ _\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[__\x1B\x81V[``a\x05\xDC`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a\x07\xACV[\x90P\x91\x90PV[_a\x05\xFD`\x01_\x84\x81R` \x01\x90\x81R` \x01_ a\x07\xCBV[\x90P\x91\x90PV[\x7F\x1F\xA0\xF8\xD8\xC1S\xD9\xDAjG\xE7w(\x81\x91\x05\x8DS\xF4@\xEB\xDE\xF2\xB4\x9B\xCA\xBAs\xE2\x98\xACNa\x06.\x81a\x06\xF1V[a\x066a\x07\x05V[a\x06@\x83\x83a\x07\xDEV[PPPPV[_a\x06q\x7F\xB9\" \0\x89u=c\x1C6\xB6\xEA\xC4l\x11V`\xE0Wv\\\x8D\xF5\xC7\x96yf6&_\xC4i\x83a\x05XV[\x90P\x91\x90PV[_\x7FZ\x05\x18\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x06\xEAWPa\x06\xE9\x82a\x07\xF1V[[\x90P\x91\x90PV[a\x07\x02\x81a\x06\xFDa\x08jV[a\x08qV[PV[V[_a\x07\x12\x83\x83a\x08\xC2V[\x90P\x92\x91PPV[a\x07\"a\x08jV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07\x86W`@Q\x7Ff\x97\xB22\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\x90\x82\x82a\x07\xDEV[PPPV[_a\x07\xA2\x83_\x01\x83a\t\x05V[_\x1C\x90P\x92\x91PPV[``_a\x07\xBA\x83_\x01a\t,V[\x90P``\x81\x90P\x80\x92PPP\x91\x90PV[_a\x07\xD7\x82_\x01a\t\x85V[\x90P\x91\x90PV[_a\x07\xE9\x83\x83a\t\x94V[\x90P\x92\x91PPV[_\x7Fye\xDB\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x80a\x08cWPa\x08b\x82a\t\xD7V[[\x90P\x91\x90PV[_3\x90P\x90V[a\x08{\x82\x82a\x05XV[a\x08\xBEW\x80\x82`@Q\x7F\xE2Q}?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xB5\x92\x91\x90a\x11\x87V[`@Q\x80\x91\x03\x90\xFD[PPV[__a\x08\xCE\x84\x84a\n@V[\x90P\x80\x15a\x08\xFBWa\x08\xF9\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a\x0B)\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[_\x82_\x01\x82\x81T\x81\x10a\t\x1BWa\t\x1Aa\x11\xAEV[[\x90_R` _ \x01T\x90P\x92\x91PPV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\tyW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\teW[PPPPP\x90P\x91\x90PV[_\x81_\x01\x80T\x90P\x90P\x91\x90PV[__a\t\xA0\x84\x84a\x0BVV[\x90P\x80\x15a\t\xCDWa\t\xCB\x83`\x01_\x87\x81R` \x01\x90\x81R` \x01_ a\x0C?\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P[\x80\x91PP\x92\x91PPV[_\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14\x90P\x91\x90PV[_a\nK\x83\x83a\x05XV[a\x0B\x1FW`\x01__\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\n\xBCa\x08jV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\x0B#V[_\x90P[\x92\x91PPV[_a\x0BN\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba\x0ClV[\x90P\x92\x91PPV[_a\x0Ba\x83\x83a\x05XV[\x15a\x0C5W___\x85\x81R` \x01\x90\x81R` \x01_ _\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x0B\xD2a\x08jV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B`@Q`@Q\x80\x91\x03\x90\xA4`\x01\x90Pa\x0C9V[_\x90P[\x92\x91PPV[_a\x0Cd\x83_\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x1Ba\x0C\xD3V[\x90P\x92\x91PPV[_a\x0Cw\x83\x83a\r\xCFV[a\x0C\xC9W\x82_\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x82_\x01\x80T\x90P\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP`\x01\x90Pa\x0C\xCDV[_\x90P[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P_\x81\x14a\r\xC4W_`\x01\x82a\r\0\x91\x90a\x12\x08V[\x90P_`\x01\x86_\x01\x80T\x90Pa\r\x16\x91\x90a\x12\x08V[\x90P\x80\x82\x14a\r|W_\x86_\x01\x82\x81T\x81\x10a\r5Wa\r4a\x11\xAEV[[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a\rVWa\rUa\x11\xAEV[[\x90_R` _ \x01\x81\x90UP\x83\x87`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UPP[\x85_\x01\x80T\x80a\r\x8FWa\r\x8Ea\x12;V[[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\r\xC9V[_\x91PP[\x92\x91PPV[__\x83`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x14\x15\x90P\x92\x91PPV[__\xFD[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a\x0E'\x81a\r\xF3V[\x81\x14a\x0E1W__\xFD[PV[_\x815\x90Pa\x0EB\x81a\x0E\x1EV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0E]Wa\x0E\\a\r\xEFV[[_a\x0Ej\x84\x82\x85\x01a\x0E4V[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\x0E\x87\x81a\x0EsV[\x82RPPV[_` \x82\x01\x90Pa\x0E\xA0_\x83\x01\x84a\x0E~V[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x0E\xCF\x82a\x0E\xA6V[\x90P\x91\x90PV[a\x0E\xDF\x81a\x0E\xC5V[\x81\x14a\x0E\xE9W__\xFD[PV[_\x815\x90Pa\x0E\xFA\x81a\x0E\xD6V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0F\x15Wa\x0F\x14a\r\xEFV[[_a\x0F\"\x84\x82\x85\x01a\x0E\xECV[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x0F=\x81a\x0F+V[\x81\x14a\x0FGW__\xFD[PV[_\x815\x90Pa\x0FX\x81a\x0F4V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0FsWa\x0Fra\r\xEFV[[_a\x0F\x80\x84\x82\x85\x01a\x0FJV[\x91PP\x92\x91PPV[a\x0F\x92\x81a\x0F+V[\x82RPPV[_` \x82\x01\x90Pa\x0F\xAB_\x83\x01\x84a\x0F\x89V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x0F\xC7Wa\x0F\xC6a\r\xEFV[[_a\x0F\xD4\x85\x82\x86\x01a\x0FJV[\x92PP` a\x0F\xE5\x85\x82\x86\x01a\x0E\xECV[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[a\x10\x01\x81a\x0F\xEFV[\x81\x14a\x10\x0BW__\xFD[PV[_\x815\x90Pa\x10\x1C\x81a\x0F\xF8V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x108Wa\x107a\r\xEFV[[_a\x10E\x85\x82\x86\x01a\x0FJV[\x92PP` a\x10V\x85\x82\x86\x01a\x10\x0EV[\x91PP\x92P\x92\x90PV[a\x10i\x81a\x0E\xC5V[\x82RPPV[_` \x82\x01\x90Pa\x10\x82_\x83\x01\x84a\x10`V[\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x10\xBA\x81a\x0E\xC5V[\x82RPPV[_a\x10\xCB\x83\x83a\x10\xB1V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x10\xED\x82a\x10\x88V[a\x10\xF7\x81\x85a\x10\x92V[\x93Pa\x11\x02\x83a\x10\xA2V[\x80_[\x83\x81\x10\x15a\x112W\x81Qa\x11\x19\x88\x82a\x10\xC0V[\x97Pa\x11$\x83a\x10\xD7V[\x92PP`\x01\x81\x01\x90Pa\x11\x05V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x11W\x81\x84a\x10\xE3V[\x90P\x92\x91PPV[a\x11h\x81a\x0F\xEFV[\x82RPPV[_` \x82\x01\x90Pa\x11\x81_\x83\x01\x84a\x11_V[\x92\x91PPV[_`@\x82\x01\x90Pa\x11\x9A_\x83\x01\x85a\x10`V[a\x11\xA7` \x83\x01\x84a\x0F\x89V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x12\x12\x82a\x0F\xEFV[\x91Pa\x12\x1D\x83a\x0F\xEFV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x125Wa\x124a\x11\xDBV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 ?\xE4J\xCEU\xC9\x0E<\xD1}\x0Cg\x80\xB97\xFA\xC6\xF4\x84\xE5F\xEF\x97LU*#\xFA\t$\xA6\xAEdsolcC\0\x08!\x003\xA2dipfsX\"\x12 x\xD8\xBD4\xB5zk\x9A\x98\x06\x18\x19%O/C,\xDE\x80\xEC^\xAC\x8A;my\xD2\x82\x14\xB5\x0EPdsolcC\0\x08!\x003",
    );
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
                65u8, 48u8, 79u8, 172u8, 217u8, 50u8, 61u8, 117u8, 177u8, 27u8, 205u8,
                214u8, 9u8, 203u8, 56u8, 239u8, 255u8, 253u8, 176u8, 87u8, 16u8, 247u8,
                202u8, 240u8, 233u8, 177u8, 108u8, 109u8, 157u8, 112u8, 159u8, 80u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                122u8, 231u8, 76u8, 82u8, 116u8, 20u8, 174u8, 19u8, 95u8, 217u8, 112u8,
                71u8, 177u8, 41u8, 33u8, 165u8, 236u8, 57u8, 17u8, 184u8, 4u8, 25u8,
                120u8, 85u8, 214u8, 126u8, 37u8, 199u8, 183u8, 94u8, 230u8, 243u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                251u8, 16u8, 40u8, 101u8, 213u8, 10u8, 221u8, 221u8, 246u8, 157u8, 169u8,
                181u8, 170u8, 27u8, 206u8, 214u8, 108u8, 128u8, 207u8, 134u8, 154u8,
                92u8, 141u8, 4u8, 113u8, 164u8, 103u8, 225u8, 140u8, 233u8, 202u8, 177u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                137u8, 10u8, 130u8, 103u8, 155u8, 71u8, 15u8, 43u8, 216u8, 40u8, 22u8,
                237u8, 155u8, 22u8, 31u8, 151u8, 216u8, 185u8, 103u8, 243u8, 127u8,
                163u8, 100u8, 124u8, 33u8, 213u8, 191u8, 57u8, 116u8, 158u8, 45u8, 213u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                64u8, 225u8, 132u8, 15u8, 87u8, 105u8, 7u8, 61u8, 97u8, 189u8, 1u8, 55u8,
                45u8, 155u8, 117u8, 186u8, 169u8, 132u8, 45u8, 86u8, 41u8, 160u8, 201u8,
                159u8, 241u8, 3u8, 190u8, 17u8, 120u8, 168u8, 233u8, 226u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                35u8, 182u8, 42u8, 208u8, 88u8, 77u8, 36u8, 167u8, 95u8, 11u8, 243u8,
                86u8, 3u8, 145u8, 239u8, 86u8, 89u8, 236u8, 109u8, 177u8, 38u8, 156u8,
                86u8, 225u8, 26u8, 162u8, 65u8, 214u8, 55u8, 241u8, 155u8, 32u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                232u8, 22u8, 153u8, 184u8, 81u8, 19u8, 238u8, 161u8, 199u8, 62u8, 16u8,
                88u8, 139u8, 43u8, 3u8, 94u8, 85u8, 137u8, 51u8, 105u8, 99u8, 33u8,
                115u8, 175u8, 212u8, 63u8, 235u8, 25u8, 47u8, 172u8, 100u8, 227u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                14u8, 181u8, 213u8, 38u8, 36u8, 200u8, 210u8, 138u8, 218u8, 159u8, 197u8,
                90u8, 140u8, 80u8, 46u8, 213u8, 170u8, 63u8, 190u8, 47u8, 182u8, 233u8,
                27u8, 113u8, 181u8, 243u8, 118u8, 136u8, 43u8, 29u8, 47u8, 184u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                156u8, 78u8, 133u8, 65u8, 202u8, 143u8, 13u8, 193u8, 196u8, 19u8, 249u8,
                16u8, 143u8, 102u8, 216u8, 45u8, 60u8, 236u8, 177u8, 189u8, 219u8, 206u8,
                67u8, 122u8, 97u8, 202u8, 163u8, 23u8, 92u8, 76u8, 201u8, 111u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                0u8, 170u8, 163u8, 156u8, 159u8, 251u8, 95u8, 86u8, 122u8, 69u8, 52u8,
                56u8, 12u8, 115u8, 112u8, 117u8, 112u8, 46u8, 31u8, 127u8, 20u8, 16u8,
                127u8, 201u8, 83u8, 40u8, 227u8, 181u8, 108u8, 3u8, 37u8, 251u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                167u8, 62u8, 218u8, 9u8, 102u8, 47u8, 70u8, 221u8, 231u8, 41u8, 190u8,
                70u8, 17u8, 56u8, 95u8, 243u8, 79u8, 230u8, 196u8, 79u8, 187u8, 198u8,
                247u8, 225u8, 123u8, 4u8, 43u8, 89u8, 163u8, 68u8, 91u8, 87u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                59u8, 207u8, 178u8, 174u8, 46u8, 141u8, 19u8, 45u8, 209u8, 252u8, 231u8,
                207u8, 39u8, 138u8, 154u8, 25u8, 117u8, 106u8, 159u8, 206u8, 171u8,
                228u8, 112u8, 223u8, 59u8, 218u8, 187u8, 75u8, 197u8, 119u8, 209u8, 189u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                210u8, 110u8, 22u8, 202u8, 212u8, 84u8, 135u8, 5u8, 228u8, 201u8, 226u8,
                217u8, 79u8, 152u8, 238u8, 145u8, 194u8, 137u8, 8u8, 94u8, 228u8, 37u8,
                89u8, 79u8, 213u8, 99u8, 95u8, 162u8, 150u8, 76u8, 207u8, 24u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                175u8, 183u8, 149u8, 201u8, 198u8, 30u8, 79u8, 231u8, 70u8, 140u8, 56u8,
                111u8, 146u8, 93u8, 122u8, 84u8, 41u8, 236u8, 173u8, 156u8, 4u8, 149u8,
                221u8, 184u8, 211u8, 141u8, 105u8, 6u8, 20u8, 211u8, 47u8, 153u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                93u8, 166u8, 206u8, 157u8, 81u8, 21u8, 27u8, 161u8, 12u8, 9u8, 165u8,
                89u8, 239u8, 36u8, 213u8, 32u8, 185u8, 218u8, 197u8, 197u8, 184u8, 129u8,
                10u8, 232u8, 67u8, 78u8, 77u8, 13u8, 134u8, 65u8, 26u8, 149u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                235u8, 139u8, 164u8, 60u8, 237u8, 117u8, 55u8, 66u8, 25u8, 70u8, 189u8,
                67u8, 232u8, 40u8, 184u8, 178u8, 184u8, 66u8, 137u8, 39u8, 170u8, 143u8,
                128u8, 28u8, 19u8, 217u8, 52u8, 191u8, 17u8, 172u8, 165u8, 123u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                47u8, 230u8, 50u8, 119u8, 145u8, 116u8, 55u8, 67u8, 120u8, 68u8, 42u8,
                142u8, 151u8, 139u8, 204u8, 251u8, 220u8, 193u8, 214u8, 178u8, 176u8,
                216u8, 31u8, 126u8, 142u8, 183u8, 118u8, 171u8, 34u8, 134u8, 241u8, 104u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                40u8, 15u8, 68u8, 70u8, 178u8, 138u8, 19u8, 114u8, 65u8, 125u8, 218u8,
                101u8, 141u8, 48u8, 185u8, 91u8, 41u8, 146u8, 177u8, 42u8, 201u8, 199u8,
                243u8, 120u8, 83u8, 95u8, 41u8, 169u8, 122u8, 207u8, 53u8, 131u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                178u8, 222u8, 47u8, 190u8, 128u8, 26u8, 13u8, 246u8, 192u8, 203u8, 221u8,
                253u8, 68u8, 139u8, 163u8, 196u8, 29u8, 72u8, 160u8, 64u8, 202u8, 53u8,
                197u8, 108u8, 129u8, 150u8, 239u8, 15u8, 202u8, 231u8, 33u8, 168u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                11u8, 46u8, 19u8, 255u8, 32u8, 172u8, 123u8, 71u8, 65u8, 152u8, 101u8,
                85u8, 131u8, 237u8, 247u8, 13u8, 237u8, 210u8, 193u8, 220u8, 152u8, 14u8,
                50u8, 156u8, 79u8, 187u8, 47u8, 192u8, 116u8, 139u8, 121u8, 107u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                44u8, 171u8, 151u8, 144u8, 81u8, 15u8, 216u8, 189u8, 251u8, 210u8, 17u8,
                82u8, 136u8, 219u8, 51u8, 254u8, 198u8, 102u8, 145u8, 212u8, 118u8,
                239u8, 197u8, 66u8, 124u8, 253u8, 76u8, 9u8, 105u8, 48u8, 23u8, 85u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                231u8, 149u8, 14u8, 222u8, 3u8, 148u8, 185u8, 242u8, 206u8, 74u8, 90u8,
                27u8, 245u8, 167u8, 225u8, 133u8, 36u8, 17u8, 247u8, 230u8, 102u8, 27u8,
                67u8, 8u8, 201u8, 19u8, 196u8, 191u8, 209u8, 16u8, 39u8, 228u8,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `IS_TEST()` and selector `0xfa7626d4`.
```solidity
function IS_TEST() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IS_TESTCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`IS_TEST()`](IS_TESTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IS_TESTReturn {
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
                    Self
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
            type Return = bool;
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
                        let r: IS_TESTReturn = r.into();
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
                        let r: IS_TESTReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `ac()` and selector `0x54b7c39b`.
```solidity
function ac() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`ac()`](acCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acReturn {
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
            impl ::core::convert::From<acCall> for UnderlyingRustTuple<'_> {
                fn from(value: acCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for acCall {
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
            impl ::core::convert::From<acReturn> for UnderlyingRustTuple<'_> {
                fn from(value: acReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for acReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for acCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ac()";
            const SELECTOR: [u8; 4] = [84u8, 183u8, 195u8, 155u8];
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
                        let r: acReturn = r.into();
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
                        let r: acReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `excludeArtifacts()` and selector `0xb5508aa9`.
```solidity
function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeArtifactsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`excludeArtifacts()`](excludeArtifactsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeArtifactsReturn {
        #[allow(missing_docs)]
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
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::String,
            >;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: excludeArtifactsReturn = r.into();
                        r.excludedArtifacts_
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
                        let r: excludeArtifactsReturn = r.into();
                        r.excludedArtifacts_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `excludeContracts()` and selector `0xe20c9f71`.
```solidity
function excludeContracts() external view returns (address[] memory excludedContracts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeContractsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`excludeContracts()`](excludeContractsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeContractsReturn {
        #[allow(missing_docs)]
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
                    Self
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
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >;
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
                        let r: excludeContractsReturn = r.into();
                        r.excludedContracts_
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
                        let r: excludeContractsReturn = r.into();
                        r.excludedContracts_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `excludeSelectors()` and selector `0xb0464fdc`.
```solidity
function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSelectorsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`excludeSelectors()`](excludeSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSelectorsReturn {
        #[allow(missing_docs)]
        pub excludedSelectors_: alloy::sol_types::private::Vec<
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
            impl ::core::convert::From<excludeSelectorsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<excludeSelectorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeSelectorsReturn) -> Self {
                    (value.excludedSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeSelectorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedSelectors_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeSelectorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeSelectors()";
            const SELECTOR: [u8; 4] = [176u8, 70u8, 79u8, 220u8];
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
                    <alloy::sol_types::sol_data::Array<
                        StdInvariant::FuzzSelector,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: excludeSelectorsReturn = r.into();
                        r.excludedSelectors_
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
                        let r: excludeSelectorsReturn = r.into();
                        r.excludedSelectors_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `excludeSenders()` and selector `0x1ed7831c`.
```solidity
function excludeSenders() external view returns (address[] memory excludedSenders_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSendersCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`excludeSenders()`](excludeSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSendersReturn {
        #[allow(missing_docs)]
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
            impl ::core::convert::From<excludeSendersCall> for UnderlyingRustTuple<'_> {
                fn from(value: excludeSendersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for excludeSendersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >;
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
                        let r: excludeSendersReturn = r.into();
                        r.excludedSenders_
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
                        let r: excludeSendersReturn = r.into();
                        r.excludedSenders_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `failed()` and selector `0xba414fa6`.
```solidity
function failed() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct failedCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`failed()`](failedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct failedReturn {
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
                    Self
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
            type Return = bool;
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
                        let r: failedReturn = r.into();
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
                        let r: failedReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setUp()` and selector `0x0a9254e4`.
```solidity
function setUp() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setUpCall;
    ///Container type for the return parameters of the [`setUp()`](setUpCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setUpReturn {}
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
            impl ::core::convert::From<setUpCall> for UnderlyingRustTuple<'_> {
                fn from(value: setUpCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setUpCall {
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
            impl ::core::convert::From<setUpReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setUpReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setUpReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setUpReturn {
            fn _tokenize(
                &self,
            ) -> <setUpCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setUpCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setUpReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setUp()";
            const SELECTOR: [u8; 4] = [10u8, 146u8, 84u8, 228u8];
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
                setUpReturn::_tokenize(ret)
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
    /**Function with signature `targetArtifactSelectors()` and selector `0x66d9a9a0`.
```solidity
function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactSelectorsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetArtifactSelectors()`](targetArtifactSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactSelectorsReturn {
        #[allow(missing_docs)]
        pub targetedArtifactSelectors_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzArtifactSelector as alloy::sol_types::SolType>::RustType,
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
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzArtifactSelector>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzArtifactSelector as alloy::sol_types::SolType>::RustType,
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
            type Return = alloy::sol_types::private::Vec<
                <StdInvariant::FuzzArtifactSelector as alloy::sol_types::SolType>::RustType,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzArtifactSelector>,
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StdInvariant::FuzzArtifactSelector,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: targetArtifactSelectorsReturn = r.into();
                        r.targetedArtifactSelectors_
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
                        let r: targetArtifactSelectorsReturn = r.into();
                        r.targetedArtifactSelectors_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetArtifacts()` and selector `0x85226c81`.
```solidity
function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetArtifacts()`](targetArtifactsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactsReturn {
        #[allow(missing_docs)]
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
            impl ::core::convert::From<targetArtifactsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetArtifactsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::String,
            >;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: targetArtifactsReturn = r.into();
                        r.targetedArtifacts_
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
                        let r: targetArtifactsReturn = r.into();
                        r.targetedArtifacts_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetContracts()` and selector `0x3f7286f4`.
```solidity
function targetContracts() external view returns (address[] memory targetedContracts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetContractsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetContracts()`](targetContractsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetContractsReturn {
        #[allow(missing_docs)]
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
            impl ::core::convert::From<targetContractsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetContractsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetContractsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >;
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
                        let r: targetContractsReturn = r.into();
                        r.targetedContracts_
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
                        let r: targetContractsReturn = r.into();
                        r.targetedContracts_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetInterfaces()` and selector `0x2ade3880`.
```solidity
function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetInterfacesCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetInterfaces()`](targetInterfacesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetInterfacesReturn {
        #[allow(missing_docs)]
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
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            type Return = alloy::sol_types::private::Vec<
                <StdInvariant::FuzzInterface as alloy::sol_types::SolType>::RustType,
            >;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StdInvariant::FuzzInterface,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: targetInterfacesReturn = r.into();
                        r.targetedInterfaces_
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
                        let r: targetInterfacesReturn = r.into();
                        r.targetedInterfaces_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetSelectors()` and selector `0x916a17c6`.
```solidity
function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSelectorsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetSelectors()`](targetSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSelectorsReturn {
        #[allow(missing_docs)]
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
            impl ::core::convert::From<targetSelectorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            type Return = alloy::sol_types::private::Vec<
                <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
            >;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StdInvariant::FuzzSelector,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: targetSelectorsReturn = r.into();
                        r.targetedSelectors_
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
                        let r: targetSelectorsReturn = r.into();
                        r.targetedSelectors_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetSenders()` and selector `0x3e5e3c23`.
```solidity
function targetSenders() external view returns (address[] memory targetedSenders_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSendersCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetSenders()`](targetSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSendersReturn {
        #[allow(missing_docs)]
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
            impl ::core::convert::From<targetSendersCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetSendersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSendersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >;
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
                        let r: targetSendersReturn = r.into();
                        r.targetedSenders_
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
                        let r: targetSendersReturn = r.into();
                        r.targetedSenders_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `test_constructor_emitsInitializeEvent()` and selector `0xba2cbd35`.
```solidity
function test_constructor_emitsInitializeEvent() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_constructor_emitsInitializeEventCall;
    ///Container type for the return parameters of the [`test_constructor_emitsInitializeEvent()`](test_constructor_emitsInitializeEventCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_constructor_emitsInitializeEventReturn {}
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
            impl ::core::convert::From<test_constructor_emitsInitializeEventCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_constructor_emitsInitializeEventCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_constructor_emitsInitializeEventCall {
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
            impl ::core::convert::From<test_constructor_emitsInitializeEventReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_constructor_emitsInitializeEventReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_constructor_emitsInitializeEventReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_constructor_emitsInitializeEventReturn {
            fn _tokenize(
                &self,
            ) -> <test_constructor_emitsInitializeEventCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_constructor_emitsInitializeEventCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_constructor_emitsInitializeEventReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_constructor_emitsInitializeEvent()";
            const SELECTOR: [u8; 4] = [186u8, 44u8, 189u8, 53u8];
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
                test_constructor_emitsInitializeEventReturn::_tokenize(ret)
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
    /**Function with signature `test_constructor_grantRoles()` and selector `0x374d1fd5`.
```solidity
function test_constructor_grantRoles() external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_constructor_grantRolesCall;
    ///Container type for the return parameters of the [`test_constructor_grantRoles()`](test_constructor_grantRolesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_constructor_grantRolesReturn {}
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
            impl ::core::convert::From<test_constructor_grantRolesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_constructor_grantRolesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_constructor_grantRolesCall {
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
            impl ::core::convert::From<test_constructor_grantRolesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_constructor_grantRolesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_constructor_grantRolesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_constructor_grantRolesReturn {
            fn _tokenize(
                &self,
            ) -> <test_constructor_grantRolesCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_constructor_grantRolesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_constructor_grantRolesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_constructor_grantRoles()";
            const SELECTOR: [u8; 4] = [55u8, 77u8, 31u8, 213u8];
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
                test_constructor_grantRolesReturn::_tokenize(ret)
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
    /**Function with signature `test_grantRoleAgain()` and selector `0xcd099e6a`.
```solidity
function test_grantRoleAgain() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_grantRoleAgainCall;
    ///Container type for the return parameters of the [`test_grantRoleAgain()`](test_grantRoleAgainCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_grantRoleAgainReturn {}
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
            impl ::core::convert::From<test_grantRoleAgainCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_grantRoleAgainCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_grantRoleAgainCall {
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
            impl ::core::convert::From<test_grantRoleAgainReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_grantRoleAgainReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_grantRoleAgainReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_grantRoleAgainReturn {
            fn _tokenize(
                &self,
            ) -> <test_grantRoleAgainCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_grantRoleAgainCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_grantRoleAgainReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_grantRoleAgain()";
            const SELECTOR: [u8; 4] = [205u8, 9u8, 158u8, 106u8];
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
                test_grantRoleAgainReturn::_tokenize(ret)
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
    /**Function with signature `test_grantRole_designatedPartyCanGrantRoles()` and selector `0xe49e9ff5`.
```solidity
function test_grantRole_designatedPartyCanGrantRoles() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_grantRole_designatedPartyCanGrantRolesCall;
    ///Container type for the return parameters of the [`test_grantRole_designatedPartyCanGrantRoles()`](test_grantRole_designatedPartyCanGrantRolesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_grantRole_designatedPartyCanGrantRolesReturn {}
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
            impl ::core::convert::From<test_grantRole_designatedPartyCanGrantRolesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_grantRole_designatedPartyCanGrantRolesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_grantRole_designatedPartyCanGrantRolesCall {
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
            impl ::core::convert::From<test_grantRole_designatedPartyCanGrantRolesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_grantRole_designatedPartyCanGrantRolesReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_grantRole_designatedPartyCanGrantRolesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_grantRole_designatedPartyCanGrantRolesReturn {
            fn _tokenize(
                &self,
            ) -> <test_grantRole_designatedPartyCanGrantRolesCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for test_grantRole_designatedPartyCanGrantRolesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_grantRole_designatedPartyCanGrantRolesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_grantRole_designatedPartyCanGrantRoles()";
            const SELECTOR: [u8; 4] = [228u8, 158u8, 159u8, 245u8];
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
                test_grantRole_designatedPartyCanGrantRolesReturn::_tokenize(ret)
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
    /**Function with signature `test_grantRole_revertsIfCallerLacksDesignatedPartyRole()` and selector `0x200a56ce`.
```solidity
function test_grantRole_revertsIfCallerLacksDesignatedPartyRole() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_grantRole_revertsIfCallerLacksDesignatedPartyRoleCall;
    ///Container type for the return parameters of the [`test_grantRole_revertsIfCallerLacksDesignatedPartyRole()`](test_grantRole_revertsIfCallerLacksDesignatedPartyRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_grantRole_revertsIfCallerLacksDesignatedPartyRoleReturn {}
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
            impl ::core::convert::From<
                test_grantRole_revertsIfCallerLacksDesignatedPartyRoleCall,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_grantRole_revertsIfCallerLacksDesignatedPartyRoleCall,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_grantRole_revertsIfCallerLacksDesignatedPartyRoleCall {
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
            impl ::core::convert::From<
                test_grantRole_revertsIfCallerLacksDesignatedPartyRoleReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_grantRole_revertsIfCallerLacksDesignatedPartyRoleReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_grantRole_revertsIfCallerLacksDesignatedPartyRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_grantRole_revertsIfCallerLacksDesignatedPartyRoleReturn {
            fn _tokenize(
                &self,
            ) -> <test_grantRole_revertsIfCallerLacksDesignatedPartyRoleCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for test_grantRole_revertsIfCallerLacksDesignatedPartyRoleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_grantRole_revertsIfCallerLacksDesignatedPartyRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_grantRole_revertsIfCallerLacksDesignatedPartyRole()";
            const SELECTOR: [u8; 4] = [32u8, 10u8, 86u8, 206u8];
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
                test_grantRole_revertsIfCallerLacksDesignatedPartyRoleReturn::_tokenize(
                    ret,
                )
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
    /**Function with signature `test_isDesignatedParty()` and selector `0x9bc8a619`.
```solidity
function test_isDesignatedParty() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_isDesignatedPartyCall;
    ///Container type for the return parameters of the [`test_isDesignatedParty()`](test_isDesignatedPartyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_isDesignatedPartyReturn {}
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
            impl ::core::convert::From<test_isDesignatedPartyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_isDesignatedPartyCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_isDesignatedPartyCall {
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
            impl ::core::convert::From<test_isDesignatedPartyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_isDesignatedPartyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_isDesignatedPartyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_isDesignatedPartyReturn {
            fn _tokenize(
                &self,
            ) -> <test_isDesignatedPartyCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_isDesignatedPartyCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_isDesignatedPartyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_isDesignatedParty()";
            const SELECTOR: [u8; 4] = [155u8, 200u8, 166u8, 25u8];
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
                test_isDesignatedPartyReturn::_tokenize(ret)
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
    /**Function with signature `test_isParty()` and selector `0x3a5e4f23`.
```solidity
function test_isParty() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_isPartyCall;
    ///Container type for the return parameters of the [`test_isParty()`](test_isPartyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_isPartyReturn {}
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
            impl ::core::convert::From<test_isPartyCall> for UnderlyingRustTuple<'_> {
                fn from(value: test_isPartyCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for test_isPartyCall {
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
            impl ::core::convert::From<test_isPartyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: test_isPartyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for test_isPartyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_isPartyReturn {
            fn _tokenize(
                &self,
            ) -> <test_isPartyCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_isPartyCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_isPartyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_isParty()";
            const SELECTOR: [u8; 4] = [58u8, 94u8, 79u8, 35u8];
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
                test_isPartyReturn::_tokenize(ret)
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
    /**Function with signature `test_renounceRole_partyCanRenounceOwnRole()` and selector `0x484b3f36`.
```solidity
function test_renounceRole_partyCanRenounceOwnRole() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_renounceRole_partyCanRenounceOwnRoleCall;
    ///Container type for the return parameters of the [`test_renounceRole_partyCanRenounceOwnRole()`](test_renounceRole_partyCanRenounceOwnRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_renounceRole_partyCanRenounceOwnRoleReturn {}
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
            impl ::core::convert::From<test_renounceRole_partyCanRenounceOwnRoleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_renounceRole_partyCanRenounceOwnRoleCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_renounceRole_partyCanRenounceOwnRoleCall {
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
            impl ::core::convert::From<test_renounceRole_partyCanRenounceOwnRoleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_renounceRole_partyCanRenounceOwnRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_renounceRole_partyCanRenounceOwnRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_renounceRole_partyCanRenounceOwnRoleReturn {
            fn _tokenize(
                &self,
            ) -> <test_renounceRole_partyCanRenounceOwnRoleCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_renounceRole_partyCanRenounceOwnRoleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_renounceRole_partyCanRenounceOwnRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_renounceRole_partyCanRenounceOwnRole()";
            const SELECTOR: [u8; 4] = [72u8, 75u8, 63u8, 54u8];
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
                test_renounceRole_partyCanRenounceOwnRoleReturn::_tokenize(ret)
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
    /**Function with signature `test_renounceRole_revertsIfRenouncingForAnotherAccount()` and selector `0x65c9f485`.
```solidity
function test_renounceRole_revertsIfRenouncingForAnotherAccount() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_renounceRole_revertsIfRenouncingForAnotherAccountCall;
    ///Container type for the return parameters of the [`test_renounceRole_revertsIfRenouncingForAnotherAccount()`](test_renounceRole_revertsIfRenouncingForAnotherAccountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_renounceRole_revertsIfRenouncingForAnotherAccountReturn {}
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
            impl ::core::convert::From<
                test_renounceRole_revertsIfRenouncingForAnotherAccountCall,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_renounceRole_revertsIfRenouncingForAnotherAccountCall,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_renounceRole_revertsIfRenouncingForAnotherAccountCall {
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
            impl ::core::convert::From<
                test_renounceRole_revertsIfRenouncingForAnotherAccountReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_renounceRole_revertsIfRenouncingForAnotherAccountReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_renounceRole_revertsIfRenouncingForAnotherAccountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_renounceRole_revertsIfRenouncingForAnotherAccountReturn {
            fn _tokenize(
                &self,
            ) -> <test_renounceRole_revertsIfRenouncingForAnotherAccountCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for test_renounceRole_revertsIfRenouncingForAnotherAccountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_renounceRole_revertsIfRenouncingForAnotherAccountReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_renounceRole_revertsIfRenouncingForAnotherAccount()";
            const SELECTOR: [u8; 4] = [101u8, 201u8, 244u8, 133u8];
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
                test_renounceRole_revertsIfRenouncingForAnotherAccountReturn::_tokenize(
                    ret,
                )
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
    /**Function with signature `test_revokeRole_designatedPartyCanRevokePartyRole()` and selector `0x5c972be5`.
```solidity
function test_revokeRole_designatedPartyCanRevokePartyRole() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_revokeRole_designatedPartyCanRevokePartyRoleCall;
    ///Container type for the return parameters of the [`test_revokeRole_designatedPartyCanRevokePartyRole()`](test_revokeRole_designatedPartyCanRevokePartyRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_revokeRole_designatedPartyCanRevokePartyRoleReturn {}
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
            impl ::core::convert::From<
                test_revokeRole_designatedPartyCanRevokePartyRoleCall,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_revokeRole_designatedPartyCanRevokePartyRoleCall,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_revokeRole_designatedPartyCanRevokePartyRoleCall {
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
            impl ::core::convert::From<
                test_revokeRole_designatedPartyCanRevokePartyRoleReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_revokeRole_designatedPartyCanRevokePartyRoleReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_revokeRole_designatedPartyCanRevokePartyRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_revokeRole_designatedPartyCanRevokePartyRoleReturn {
            fn _tokenize(
                &self,
            ) -> <test_revokeRole_designatedPartyCanRevokePartyRoleCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for test_revokeRole_designatedPartyCanRevokePartyRoleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_revokeRole_designatedPartyCanRevokePartyRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_revokeRole_designatedPartyCanRevokePartyRole()";
            const SELECTOR: [u8; 4] = [92u8, 151u8, 43u8, 229u8];
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
                test_revokeRole_designatedPartyCanRevokePartyRoleReturn::_tokenize(ret)
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
    /**Function with signature `test_revokeRole_partyCountDecreases()` and selector `0xd3fe9e7d`.
```solidity
function test_revokeRole_partyCountDecreases() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_revokeRole_partyCountDecreasesCall;
    ///Container type for the return parameters of the [`test_revokeRole_partyCountDecreases()`](test_revokeRole_partyCountDecreasesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_revokeRole_partyCountDecreasesReturn {}
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
            impl ::core::convert::From<test_revokeRole_partyCountDecreasesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_revokeRole_partyCountDecreasesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_revokeRole_partyCountDecreasesCall {
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
            impl ::core::convert::From<test_revokeRole_partyCountDecreasesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_revokeRole_partyCountDecreasesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_revokeRole_partyCountDecreasesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_revokeRole_partyCountDecreasesReturn {
            fn _tokenize(
                &self,
            ) -> <test_revokeRole_partyCountDecreasesCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_revokeRole_partyCountDecreasesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_revokeRole_partyCountDecreasesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_revokeRole_partyCountDecreases()";
            const SELECTOR: [u8; 4] = [211u8, 254u8, 158u8, 125u8];
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
                test_revokeRole_partyCountDecreasesReturn::_tokenize(ret)
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
    /**Function with signature `test_revokeRole_revertsIfCallerLacksDesignatedPartyRole()` and selector `0xe0505199`.
```solidity
function test_revokeRole_revertsIfCallerLacksDesignatedPartyRole() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_revokeRole_revertsIfCallerLacksDesignatedPartyRoleCall;
    ///Container type for the return parameters of the [`test_revokeRole_revertsIfCallerLacksDesignatedPartyRole()`](test_revokeRole_revertsIfCallerLacksDesignatedPartyRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_revokeRole_revertsIfCallerLacksDesignatedPartyRoleReturn {}
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
            impl ::core::convert::From<
                test_revokeRole_revertsIfCallerLacksDesignatedPartyRoleCall,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_revokeRole_revertsIfCallerLacksDesignatedPartyRoleCall,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_revokeRole_revertsIfCallerLacksDesignatedPartyRoleCall {
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
            impl ::core::convert::From<
                test_revokeRole_revertsIfCallerLacksDesignatedPartyRoleReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_revokeRole_revertsIfCallerLacksDesignatedPartyRoleReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_revokeRole_revertsIfCallerLacksDesignatedPartyRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_revokeRole_revertsIfCallerLacksDesignatedPartyRoleReturn {
            fn _tokenize(
                &self,
            ) -> <test_revokeRole_revertsIfCallerLacksDesignatedPartyRoleCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for test_revokeRole_revertsIfCallerLacksDesignatedPartyRoleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_revokeRole_revertsIfCallerLacksDesignatedPartyRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_revokeRole_revertsIfCallerLacksDesignatedPartyRole()";
            const SELECTOR: [u8; 4] = [224u8, 80u8, 81u8, 153u8];
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
                test_revokeRole_revertsIfCallerLacksDesignatedPartyRoleReturn::_tokenize(
                    ret,
                )
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
    /**Function with signature `test_supportsInterface_IStoffelAccessControl()` and selector `0xd7c3e532`.
```solidity
function test_supportsInterface_IStoffelAccessControl() external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_supportsInterface_IStoffelAccessControlCall;
    ///Container type for the return parameters of the [`test_supportsInterface_IStoffelAccessControl()`](test_supportsInterface_IStoffelAccessControlCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_supportsInterface_IStoffelAccessControlReturn {}
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
            impl ::core::convert::From<test_supportsInterface_IStoffelAccessControlCall>
            for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_supportsInterface_IStoffelAccessControlCall,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_supportsInterface_IStoffelAccessControlCall {
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
            impl ::core::convert::From<
                test_supportsInterface_IStoffelAccessControlReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: test_supportsInterface_IStoffelAccessControlReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_supportsInterface_IStoffelAccessControlReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl test_supportsInterface_IStoffelAccessControlReturn {
            fn _tokenize(
                &self,
            ) -> <test_supportsInterface_IStoffelAccessControlCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for test_supportsInterface_IStoffelAccessControlCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_supportsInterface_IStoffelAccessControlReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_supportsInterface_IStoffelAccessControl()";
            const SELECTOR: [u8; 4] = [215u8, 195u8, 229u8, 50u8];
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
                test_supportsInterface_IStoffelAccessControlReturn::_tokenize(ret)
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
    ///Container for all the [`StoffelAccessControlTest`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum StoffelAccessControlTestCalls {
        #[allow(missing_docs)]
        IS_TEST(IS_TESTCall),
        #[allow(missing_docs)]
        ac(acCall),
        #[allow(missing_docs)]
        excludeArtifacts(excludeArtifactsCall),
        #[allow(missing_docs)]
        excludeContracts(excludeContractsCall),
        #[allow(missing_docs)]
        excludeSelectors(excludeSelectorsCall),
        #[allow(missing_docs)]
        excludeSenders(excludeSendersCall),
        #[allow(missing_docs)]
        failed(failedCall),
        #[allow(missing_docs)]
        setUp(setUpCall),
        #[allow(missing_docs)]
        targetArtifactSelectors(targetArtifactSelectorsCall),
        #[allow(missing_docs)]
        targetArtifacts(targetArtifactsCall),
        #[allow(missing_docs)]
        targetContracts(targetContractsCall),
        #[allow(missing_docs)]
        targetInterfaces(targetInterfacesCall),
        #[allow(missing_docs)]
        targetSelectors(targetSelectorsCall),
        #[allow(missing_docs)]
        targetSenders(targetSendersCall),
        #[allow(missing_docs)]
        test_constructor_emitsInitializeEvent(test_constructor_emitsInitializeEventCall),
        #[allow(missing_docs)]
        test_constructor_grantRoles(test_constructor_grantRolesCall),
        #[allow(missing_docs)]
        test_grantRoleAgain(test_grantRoleAgainCall),
        #[allow(missing_docs)]
        test_grantRole_designatedPartyCanGrantRoles(
            test_grantRole_designatedPartyCanGrantRolesCall,
        ),
        #[allow(missing_docs)]
        test_grantRole_revertsIfCallerLacksDesignatedPartyRole(
            test_grantRole_revertsIfCallerLacksDesignatedPartyRoleCall,
        ),
        #[allow(missing_docs)]
        test_isDesignatedParty(test_isDesignatedPartyCall),
        #[allow(missing_docs)]
        test_isParty(test_isPartyCall),
        #[allow(missing_docs)]
        test_renounceRole_partyCanRenounceOwnRole(
            test_renounceRole_partyCanRenounceOwnRoleCall,
        ),
        #[allow(missing_docs)]
        test_renounceRole_revertsIfRenouncingForAnotherAccount(
            test_renounceRole_revertsIfRenouncingForAnotherAccountCall,
        ),
        #[allow(missing_docs)]
        test_revokeRole_designatedPartyCanRevokePartyRole(
            test_revokeRole_designatedPartyCanRevokePartyRoleCall,
        ),
        #[allow(missing_docs)]
        test_revokeRole_partyCountDecreases(test_revokeRole_partyCountDecreasesCall),
        #[allow(missing_docs)]
        test_revokeRole_revertsIfCallerLacksDesignatedPartyRole(
            test_revokeRole_revertsIfCallerLacksDesignatedPartyRoleCall,
        ),
        #[allow(missing_docs)]
        test_supportsInterface_IStoffelAccessControl(
            test_supportsInterface_IStoffelAccessControlCall,
        ),
    }
    impl StoffelAccessControlTestCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [10u8, 146u8, 84u8, 228u8],
            [30u8, 215u8, 131u8, 28u8],
            [32u8, 10u8, 86u8, 206u8],
            [42u8, 222u8, 56u8, 128u8],
            [55u8, 77u8, 31u8, 213u8],
            [58u8, 94u8, 79u8, 35u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [72u8, 75u8, 63u8, 54u8],
            [84u8, 183u8, 195u8, 155u8],
            [92u8, 151u8, 43u8, 229u8],
            [101u8, 201u8, 244u8, 133u8],
            [102u8, 217u8, 169u8, 160u8],
            [133u8, 34u8, 108u8, 129u8],
            [145u8, 106u8, 23u8, 198u8],
            [155u8, 200u8, 166u8, 25u8],
            [176u8, 70u8, 79u8, 220u8],
            [181u8, 80u8, 138u8, 169u8],
            [186u8, 44u8, 189u8, 53u8],
            [186u8, 65u8, 79u8, 166u8],
            [205u8, 9u8, 158u8, 106u8],
            [211u8, 254u8, 158u8, 125u8],
            [215u8, 195u8, 229u8, 50u8],
            [224u8, 80u8, 81u8, 153u8],
            [226u8, 12u8, 159u8, 113u8],
            [228u8, 158u8, 159u8, 245u8],
            [250u8, 118u8, 38u8, 212u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(setUp),
            ::core::stringify!(excludeSenders),
            ::core::stringify!(test_grantRole_revertsIfCallerLacksDesignatedPartyRole),
            ::core::stringify!(targetInterfaces),
            ::core::stringify!(test_constructor_grantRoles),
            ::core::stringify!(test_isParty),
            ::core::stringify!(targetSenders),
            ::core::stringify!(targetContracts),
            ::core::stringify!(test_renounceRole_partyCanRenounceOwnRole),
            ::core::stringify!(ac),
            ::core::stringify!(test_revokeRole_designatedPartyCanRevokePartyRole),
            ::core::stringify!(test_renounceRole_revertsIfRenouncingForAnotherAccount),
            ::core::stringify!(targetArtifactSelectors),
            ::core::stringify!(targetArtifacts),
            ::core::stringify!(targetSelectors),
            ::core::stringify!(test_isDesignatedParty),
            ::core::stringify!(excludeSelectors),
            ::core::stringify!(excludeArtifacts),
            ::core::stringify!(test_constructor_emitsInitializeEvent),
            ::core::stringify!(failed),
            ::core::stringify!(test_grantRoleAgain),
            ::core::stringify!(test_revokeRole_partyCountDecreases),
            ::core::stringify!(test_supportsInterface_IStoffelAccessControl),
            ::core::stringify!(test_revokeRole_revertsIfCallerLacksDesignatedPartyRole),
            ::core::stringify!(excludeContracts),
            ::core::stringify!(test_grantRole_designatedPartyCanGrantRoles),
            ::core::stringify!(IS_TEST),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <setUpCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeSendersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_grantRole_revertsIfCallerLacksDesignatedPartyRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetInterfacesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_constructor_grantRolesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_isPartyCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetSendersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetContractsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_renounceRole_partyCanRenounceOwnRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <acCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_revokeRole_designatedPartyCanRevokePartyRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_renounceRole_revertsIfRenouncingForAnotherAccountCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetArtifactsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_isDesignatedPartyCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeArtifactsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_constructor_emitsInitializeEventCall as alloy_sol_types::SolCall>::SIGNATURE,
            <failedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_grantRoleAgainCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_revokeRole_partyCountDecreasesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_supportsInterface_IStoffelAccessControlCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_revokeRole_revertsIfCallerLacksDesignatedPartyRoleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeContractsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <test_grantRole_designatedPartyCanGrantRolesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <IS_TESTCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for StoffelAccessControlTestCalls {
        const NAME: &'static str = "StoffelAccessControlTestCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 27usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::ac(_) => <acCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::excludeArtifacts(_) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeContracts(_) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeSelectors(_) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeSenders(_) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::failed(_) => <failedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::setUp(_) => <setUpCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::test_constructor_emitsInitializeEvent(_) => {
                    <test_constructor_emitsInitializeEventCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_constructor_grantRoles(_) => {
                    <test_constructor_grantRolesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_grantRoleAgain(_) => {
                    <test_grantRoleAgainCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_grantRole_designatedPartyCanGrantRoles(_) => {
                    <test_grantRole_designatedPartyCanGrantRolesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_grantRole_revertsIfCallerLacksDesignatedPartyRole(_) => {
                    <test_grantRole_revertsIfCallerLacksDesignatedPartyRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_isDesignatedParty(_) => {
                    <test_isDesignatedPartyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_isParty(_) => {
                    <test_isPartyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_renounceRole_partyCanRenounceOwnRole(_) => {
                    <test_renounceRole_partyCanRenounceOwnRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_renounceRole_revertsIfRenouncingForAnotherAccount(_) => {
                    <test_renounceRole_revertsIfRenouncingForAnotherAccountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_revokeRole_designatedPartyCanRevokePartyRole(_) => {
                    <test_revokeRole_designatedPartyCanRevokePartyRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_revokeRole_partyCountDecreases(_) => {
                    <test_revokeRole_partyCountDecreasesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_revokeRole_revertsIfCallerLacksDesignatedPartyRole(_) => {
                    <test_revokeRole_revertsIfCallerLacksDesignatedPartyRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_supportsInterface_IStoffelAccessControl(_) => {
                    <test_supportsInterface_IStoffelAccessControlCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls>] = &[
                {
                    fn setUp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(StoffelAccessControlTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn test_grantRole_revertsIfCallerLacksDesignatedPartyRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_grantRole_revertsIfCallerLacksDesignatedPartyRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StoffelAccessControlTestCalls::test_grantRole_revertsIfCallerLacksDesignatedPartyRole,
                            )
                    }
                    test_grantRole_revertsIfCallerLacksDesignatedPartyRole
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn test_constructor_grantRoles(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_constructor_grantRolesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StoffelAccessControlTestCalls::test_constructor_grantRoles,
                            )
                    }
                    test_constructor_grantRoles
                },
                {
                    fn test_isParty(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_isPartyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::test_isParty)
                    }
                    test_isParty
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn test_renounceRole_partyCanRenounceOwnRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_renounceRole_partyCanRenounceOwnRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StoffelAccessControlTestCalls::test_renounceRole_partyCanRenounceOwnRole,
                            )
                    }
                    test_renounceRole_partyCanRenounceOwnRole
                },
                {
                    fn ac(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <acCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(StoffelAccessControlTestCalls::ac)
                    }
                    ac
                },
                {
                    fn test_revokeRole_designatedPartyCanRevokePartyRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_revokeRole_designatedPartyCanRevokePartyRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StoffelAccessControlTestCalls::test_revokeRole_designatedPartyCanRevokePartyRole,
                            )
                    }
                    test_revokeRole_designatedPartyCanRevokePartyRole
                },
                {
                    fn test_renounceRole_revertsIfRenouncingForAnotherAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_renounceRole_revertsIfRenouncingForAnotherAccountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StoffelAccessControlTestCalls::test_renounceRole_revertsIfRenouncingForAnotherAccount,
                            )
                    }
                    test_renounceRole_revertsIfRenouncingForAnotherAccount
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn test_isDesignatedParty(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_isDesignatedPartyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::test_isDesignatedParty)
                    }
                    test_isDesignatedParty
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn test_constructor_emitsInitializeEvent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_constructor_emitsInitializeEventCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StoffelAccessControlTestCalls::test_constructor_emitsInitializeEvent,
                            )
                    }
                    test_constructor_emitsInitializeEvent
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(StoffelAccessControlTestCalls::failed)
                    }
                    failed
                },
                {
                    fn test_grantRoleAgain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_grantRoleAgainCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::test_grantRoleAgain)
                    }
                    test_grantRoleAgain
                },
                {
                    fn test_revokeRole_partyCountDecreases(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_revokeRole_partyCountDecreasesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StoffelAccessControlTestCalls::test_revokeRole_partyCountDecreases,
                            )
                    }
                    test_revokeRole_partyCountDecreases
                },
                {
                    fn test_supportsInterface_IStoffelAccessControl(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_supportsInterface_IStoffelAccessControlCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StoffelAccessControlTestCalls::test_supportsInterface_IStoffelAccessControl,
                            )
                    }
                    test_supportsInterface_IStoffelAccessControl
                },
                {
                    fn test_revokeRole_revertsIfCallerLacksDesignatedPartyRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_revokeRole_revertsIfCallerLacksDesignatedPartyRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StoffelAccessControlTestCalls::test_revokeRole_revertsIfCallerLacksDesignatedPartyRole,
                            )
                    }
                    test_revokeRole_revertsIfCallerLacksDesignatedPartyRole
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn test_grantRole_designatedPartyCanGrantRoles(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_grantRole_designatedPartyCanGrantRolesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StoffelAccessControlTestCalls::test_grantRole_designatedPartyCanGrantRoles,
                            )
                    }
                    test_grantRole_designatedPartyCanGrantRoles
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(StoffelAccessControlTestCalls::IS_TEST)
                    }
                    IS_TEST
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
            ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls>] = &[
                {
                    fn setUp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn test_grantRole_revertsIfCallerLacksDesignatedPartyRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_grantRole_revertsIfCallerLacksDesignatedPartyRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StoffelAccessControlTestCalls::test_grantRole_revertsIfCallerLacksDesignatedPartyRole,
                            )
                    }
                    test_grantRole_revertsIfCallerLacksDesignatedPartyRole
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn test_constructor_grantRoles(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_constructor_grantRolesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StoffelAccessControlTestCalls::test_constructor_grantRoles,
                            )
                    }
                    test_constructor_grantRoles
                },
                {
                    fn test_isParty(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_isPartyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::test_isParty)
                    }
                    test_isParty
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn test_renounceRole_partyCanRenounceOwnRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_renounceRole_partyCanRenounceOwnRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StoffelAccessControlTestCalls::test_renounceRole_partyCanRenounceOwnRole,
                            )
                    }
                    test_renounceRole_partyCanRenounceOwnRole
                },
                {
                    fn ac(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <acCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::ac)
                    }
                    ac
                },
                {
                    fn test_revokeRole_designatedPartyCanRevokePartyRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_revokeRole_designatedPartyCanRevokePartyRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StoffelAccessControlTestCalls::test_revokeRole_designatedPartyCanRevokePartyRole,
                            )
                    }
                    test_revokeRole_designatedPartyCanRevokePartyRole
                },
                {
                    fn test_renounceRole_revertsIfRenouncingForAnotherAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_renounceRole_revertsIfRenouncingForAnotherAccountCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StoffelAccessControlTestCalls::test_renounceRole_revertsIfRenouncingForAnotherAccount,
                            )
                    }
                    test_renounceRole_revertsIfRenouncingForAnotherAccount
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn test_isDesignatedParty(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_isDesignatedPartyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::test_isDesignatedParty)
                    }
                    test_isDesignatedParty
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn test_constructor_emitsInitializeEvent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_constructor_emitsInitializeEventCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StoffelAccessControlTestCalls::test_constructor_emitsInitializeEvent,
                            )
                    }
                    test_constructor_emitsInitializeEvent
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::failed)
                    }
                    failed
                },
                {
                    fn test_grantRoleAgain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_grantRoleAgainCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::test_grantRoleAgain)
                    }
                    test_grantRoleAgain
                },
                {
                    fn test_revokeRole_partyCountDecreases(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_revokeRole_partyCountDecreasesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StoffelAccessControlTestCalls::test_revokeRole_partyCountDecreases,
                            )
                    }
                    test_revokeRole_partyCountDecreases
                },
                {
                    fn test_supportsInterface_IStoffelAccessControl(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_supportsInterface_IStoffelAccessControlCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StoffelAccessControlTestCalls::test_supportsInterface_IStoffelAccessControl,
                            )
                    }
                    test_supportsInterface_IStoffelAccessControl
                },
                {
                    fn test_revokeRole_revertsIfCallerLacksDesignatedPartyRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_revokeRole_revertsIfCallerLacksDesignatedPartyRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StoffelAccessControlTestCalls::test_revokeRole_revertsIfCallerLacksDesignatedPartyRole,
                            )
                    }
                    test_revokeRole_revertsIfCallerLacksDesignatedPartyRole
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn test_grantRole_designatedPartyCanGrantRoles(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <test_grantRole_designatedPartyCanGrantRolesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StoffelAccessControlTestCalls::test_grantRole_designatedPartyCanGrantRoles,
                            )
                    }
                    test_grantRole_designatedPartyCanGrantRoles
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StoffelAccessControlTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StoffelAccessControlTestCalls::IS_TEST)
                    }
                    IS_TEST
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
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::ac(inner) => {
                    <acCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::excludeSelectors(inner) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::setUp(inner) => {
                    <setUpCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::test_constructor_emitsInitializeEvent(inner) => {
                    <test_constructor_emitsInitializeEventCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_constructor_grantRoles(inner) => {
                    <test_constructor_grantRolesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_grantRoleAgain(inner) => {
                    <test_grantRoleAgainCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_grantRole_designatedPartyCanGrantRoles(inner) => {
                    <test_grantRole_designatedPartyCanGrantRolesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_grantRole_revertsIfCallerLacksDesignatedPartyRole(inner) => {
                    <test_grantRole_revertsIfCallerLacksDesignatedPartyRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_isDesignatedParty(inner) => {
                    <test_isDesignatedPartyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_isParty(inner) => {
                    <test_isPartyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_renounceRole_partyCanRenounceOwnRole(inner) => {
                    <test_renounceRole_partyCanRenounceOwnRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_renounceRole_revertsIfRenouncingForAnotherAccount(inner) => {
                    <test_renounceRole_revertsIfRenouncingForAnotherAccountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_revokeRole_designatedPartyCanRevokePartyRole(inner) => {
                    <test_revokeRole_designatedPartyCanRevokePartyRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_revokeRole_partyCountDecreases(inner) => {
                    <test_revokeRole_partyCountDecreasesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_revokeRole_revertsIfCallerLacksDesignatedPartyRole(inner) => {
                    <test_revokeRole_revertsIfCallerLacksDesignatedPartyRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_supportsInterface_IStoffelAccessControl(inner) => {
                    <test_supportsInterface_IStoffelAccessControlCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::ac(inner) => {
                    <acCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::excludeSelectors(inner) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setUp(inner) => {
                    <setUpCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::test_constructor_emitsInitializeEvent(inner) => {
                    <test_constructor_emitsInitializeEventCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_constructor_grantRoles(inner) => {
                    <test_constructor_grantRolesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_grantRoleAgain(inner) => {
                    <test_grantRoleAgainCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_grantRole_designatedPartyCanGrantRoles(inner) => {
                    <test_grantRole_designatedPartyCanGrantRolesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_grantRole_revertsIfCallerLacksDesignatedPartyRole(inner) => {
                    <test_grantRole_revertsIfCallerLacksDesignatedPartyRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_isDesignatedParty(inner) => {
                    <test_isDesignatedPartyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_isParty(inner) => {
                    <test_isPartyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_renounceRole_partyCanRenounceOwnRole(inner) => {
                    <test_renounceRole_partyCanRenounceOwnRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_renounceRole_revertsIfRenouncingForAnotherAccount(inner) => {
                    <test_renounceRole_revertsIfRenouncingForAnotherAccountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_revokeRole_designatedPartyCanRevokePartyRole(inner) => {
                    <test_revokeRole_designatedPartyCanRevokePartyRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_revokeRole_partyCountDecreases(inner) => {
                    <test_revokeRole_partyCountDecreasesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_revokeRole_revertsIfCallerLacksDesignatedPartyRole(inner) => {
                    <test_revokeRole_revertsIfCallerLacksDesignatedPartyRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_supportsInterface_IStoffelAccessControl(inner) => {
                    <test_supportsInterface_IStoffelAccessControlCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`StoffelAccessControlTest`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum StoffelAccessControlTestEvents {
        #[allow(missing_docs)]
        InitializeStoffelAccessControl(InitializeStoffelAccessControl),
        #[allow(missing_docs)]
        log(log),
        #[allow(missing_docs)]
        log_address(log_address),
        #[allow(missing_docs)]
        log_array_0(log_array_0),
        #[allow(missing_docs)]
        log_array_1(log_array_1),
        #[allow(missing_docs)]
        log_array_2(log_array_2),
        #[allow(missing_docs)]
        log_bytes(log_bytes),
        #[allow(missing_docs)]
        log_bytes32(log_bytes32),
        #[allow(missing_docs)]
        log_int(log_int),
        #[allow(missing_docs)]
        log_named_address(log_named_address),
        #[allow(missing_docs)]
        log_named_array_0(log_named_array_0),
        #[allow(missing_docs)]
        log_named_array_1(log_named_array_1),
        #[allow(missing_docs)]
        log_named_array_2(log_named_array_2),
        #[allow(missing_docs)]
        log_named_bytes(log_named_bytes),
        #[allow(missing_docs)]
        log_named_bytes32(log_named_bytes32),
        #[allow(missing_docs)]
        log_named_decimal_int(log_named_decimal_int),
        #[allow(missing_docs)]
        log_named_decimal_uint(log_named_decimal_uint),
        #[allow(missing_docs)]
        log_named_int(log_named_int),
        #[allow(missing_docs)]
        log_named_string(log_named_string),
        #[allow(missing_docs)]
        log_named_uint(log_named_uint),
        #[allow(missing_docs)]
        log_string(log_string),
        #[allow(missing_docs)]
        log_uint(log_uint),
        #[allow(missing_docs)]
        logs(logs),
    }
    impl StoffelAccessControlTestEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                0u8, 170u8, 163u8, 156u8, 159u8, 251u8, 95u8, 86u8, 122u8, 69u8, 52u8,
                56u8, 12u8, 115u8, 112u8, 117u8, 112u8, 46u8, 31u8, 127u8, 20u8, 16u8,
                127u8, 201u8, 83u8, 40u8, 227u8, 181u8, 108u8, 3u8, 37u8, 251u8,
            ],
            [
                11u8, 46u8, 19u8, 255u8, 32u8, 172u8, 123u8, 71u8, 65u8, 152u8, 101u8,
                85u8, 131u8, 237u8, 247u8, 13u8, 237u8, 210u8, 193u8, 220u8, 152u8, 14u8,
                50u8, 156u8, 79u8, 187u8, 47u8, 192u8, 116u8, 139u8, 121u8, 107u8,
            ],
            [
                14u8, 181u8, 213u8, 38u8, 36u8, 200u8, 210u8, 138u8, 218u8, 159u8, 197u8,
                90u8, 140u8, 80u8, 46u8, 213u8, 170u8, 63u8, 190u8, 47u8, 182u8, 233u8,
                27u8, 113u8, 181u8, 243u8, 118u8, 136u8, 43u8, 29u8, 47u8, 184u8,
            ],
            [
                35u8, 182u8, 42u8, 208u8, 88u8, 77u8, 36u8, 167u8, 95u8, 11u8, 243u8,
                86u8, 3u8, 145u8, 239u8, 86u8, 89u8, 236u8, 109u8, 177u8, 38u8, 156u8,
                86u8, 225u8, 26u8, 162u8, 65u8, 214u8, 55u8, 241u8, 155u8, 32u8,
            ],
            [
                40u8, 15u8, 68u8, 70u8, 178u8, 138u8, 19u8, 114u8, 65u8, 125u8, 218u8,
                101u8, 141u8, 48u8, 185u8, 91u8, 41u8, 146u8, 177u8, 42u8, 201u8, 199u8,
                243u8, 120u8, 83u8, 95u8, 41u8, 169u8, 122u8, 207u8, 53u8, 131u8,
            ],
            [
                44u8, 171u8, 151u8, 144u8, 81u8, 15u8, 216u8, 189u8, 251u8, 210u8, 17u8,
                82u8, 136u8, 219u8, 51u8, 254u8, 198u8, 102u8, 145u8, 212u8, 118u8,
                239u8, 197u8, 66u8, 124u8, 253u8, 76u8, 9u8, 105u8, 48u8, 23u8, 85u8,
            ],
            [
                47u8, 230u8, 50u8, 119u8, 145u8, 116u8, 55u8, 67u8, 120u8, 68u8, 42u8,
                142u8, 151u8, 139u8, 204u8, 251u8, 220u8, 193u8, 214u8, 178u8, 176u8,
                216u8, 31u8, 126u8, 142u8, 183u8, 118u8, 171u8, 34u8, 134u8, 241u8, 104u8,
            ],
            [
                59u8, 207u8, 178u8, 174u8, 46u8, 141u8, 19u8, 45u8, 209u8, 252u8, 231u8,
                207u8, 39u8, 138u8, 154u8, 25u8, 117u8, 106u8, 159u8, 206u8, 171u8,
                228u8, 112u8, 223u8, 59u8, 218u8, 187u8, 75u8, 197u8, 119u8, 209u8, 189u8,
            ],
            [
                64u8, 225u8, 132u8, 15u8, 87u8, 105u8, 7u8, 61u8, 97u8, 189u8, 1u8, 55u8,
                45u8, 155u8, 117u8, 186u8, 169u8, 132u8, 45u8, 86u8, 41u8, 160u8, 201u8,
                159u8, 241u8, 3u8, 190u8, 17u8, 120u8, 168u8, 233u8, 226u8,
            ],
            [
                65u8, 48u8, 79u8, 172u8, 217u8, 50u8, 61u8, 117u8, 177u8, 27u8, 205u8,
                214u8, 9u8, 203u8, 56u8, 239u8, 255u8, 253u8, 176u8, 87u8, 16u8, 247u8,
                202u8, 240u8, 233u8, 177u8, 108u8, 109u8, 157u8, 112u8, 159u8, 80u8,
            ],
            [
                93u8, 166u8, 206u8, 157u8, 81u8, 21u8, 27u8, 161u8, 12u8, 9u8, 165u8,
                89u8, 239u8, 36u8, 213u8, 32u8, 185u8, 218u8, 197u8, 197u8, 184u8, 129u8,
                10u8, 232u8, 67u8, 78u8, 77u8, 13u8, 134u8, 65u8, 26u8, 149u8,
            ],
            [
                122u8, 231u8, 76u8, 82u8, 116u8, 20u8, 174u8, 19u8, 95u8, 217u8, 112u8,
                71u8, 177u8, 41u8, 33u8, 165u8, 236u8, 57u8, 17u8, 184u8, 4u8, 25u8,
                120u8, 85u8, 214u8, 126u8, 37u8, 199u8, 183u8, 94u8, 230u8, 243u8,
            ],
            [
                137u8, 10u8, 130u8, 103u8, 155u8, 71u8, 15u8, 43u8, 216u8, 40u8, 22u8,
                237u8, 155u8, 22u8, 31u8, 151u8, 216u8, 185u8, 103u8, 243u8, 127u8,
                163u8, 100u8, 124u8, 33u8, 213u8, 191u8, 57u8, 116u8, 158u8, 45u8, 213u8,
            ],
            [
                156u8, 78u8, 133u8, 65u8, 202u8, 143u8, 13u8, 193u8, 196u8, 19u8, 249u8,
                16u8, 143u8, 102u8, 216u8, 45u8, 60u8, 236u8, 177u8, 189u8, 219u8, 206u8,
                67u8, 122u8, 97u8, 202u8, 163u8, 23u8, 92u8, 76u8, 201u8, 111u8,
            ],
            [
                162u8, 223u8, 120u8, 48u8, 224u8, 190u8, 222u8, 247u8, 177u8, 17u8,
                107u8, 245u8, 71u8, 180u8, 103u8, 177u8, 107u8, 80u8, 179u8, 189u8, 35u8,
                20u8, 108u8, 158u8, 9u8, 152u8, 120u8, 209u8, 78u8, 137u8, 48u8, 26u8,
            ],
            [
                167u8, 62u8, 218u8, 9u8, 102u8, 47u8, 70u8, 221u8, 231u8, 41u8, 190u8,
                70u8, 17u8, 56u8, 95u8, 243u8, 79u8, 230u8, 196u8, 79u8, 187u8, 198u8,
                247u8, 225u8, 123u8, 4u8, 43u8, 89u8, 163u8, 68u8, 91u8, 87u8,
            ],
            [
                175u8, 183u8, 149u8, 201u8, 198u8, 30u8, 79u8, 231u8, 70u8, 140u8, 56u8,
                111u8, 146u8, 93u8, 122u8, 84u8, 41u8, 236u8, 173u8, 156u8, 4u8, 149u8,
                221u8, 184u8, 211u8, 141u8, 105u8, 6u8, 20u8, 211u8, 47u8, 153u8,
            ],
            [
                178u8, 222u8, 47u8, 190u8, 128u8, 26u8, 13u8, 246u8, 192u8, 203u8, 221u8,
                253u8, 68u8, 139u8, 163u8, 196u8, 29u8, 72u8, 160u8, 64u8, 202u8, 53u8,
                197u8, 108u8, 129u8, 150u8, 239u8, 15u8, 202u8, 231u8, 33u8, 168u8,
            ],
            [
                210u8, 110u8, 22u8, 202u8, 212u8, 84u8, 135u8, 5u8, 228u8, 201u8, 226u8,
                217u8, 79u8, 152u8, 238u8, 145u8, 194u8, 137u8, 8u8, 94u8, 228u8, 37u8,
                89u8, 79u8, 213u8, 99u8, 95u8, 162u8, 150u8, 76u8, 207u8, 24u8,
            ],
            [
                231u8, 149u8, 14u8, 222u8, 3u8, 148u8, 185u8, 242u8, 206u8, 74u8, 90u8,
                27u8, 245u8, 167u8, 225u8, 133u8, 36u8, 17u8, 247u8, 230u8, 102u8, 27u8,
                67u8, 8u8, 201u8, 19u8, 196u8, 191u8, 209u8, 16u8, 39u8, 228u8,
            ],
            [
                232u8, 22u8, 153u8, 184u8, 81u8, 19u8, 238u8, 161u8, 199u8, 62u8, 16u8,
                88u8, 139u8, 43u8, 3u8, 94u8, 85u8, 137u8, 51u8, 105u8, 99u8, 33u8,
                115u8, 175u8, 212u8, 63u8, 235u8, 25u8, 47u8, 172u8, 100u8, 227u8,
            ],
            [
                235u8, 139u8, 164u8, 60u8, 237u8, 117u8, 55u8, 66u8, 25u8, 70u8, 189u8,
                67u8, 232u8, 40u8, 184u8, 178u8, 184u8, 66u8, 137u8, 39u8, 170u8, 143u8,
                128u8, 28u8, 19u8, 217u8, 52u8, 191u8, 17u8, 172u8, 165u8, 123u8,
            ],
            [
                251u8, 16u8, 40u8, 101u8, 213u8, 10u8, 221u8, 221u8, 246u8, 157u8, 169u8,
                181u8, 170u8, 27u8, 206u8, 214u8, 108u8, 128u8, 207u8, 134u8, 154u8,
                92u8, 141u8, 4u8, 113u8, 164u8, 103u8, 225u8, 140u8, 233u8, 202u8, 177u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(log_named_array_0),
            ::core::stringify!(log_string),
            ::core::stringify!(log_int),
            ::core::stringify!(log_bytes),
            ::core::stringify!(log_named_string),
            ::core::stringify!(log_uint),
            ::core::stringify!(log_named_int),
            ::core::stringify!(log_named_array_2),
            ::core::stringify!(log_array_2),
            ::core::stringify!(log),
            ::core::stringify!(log_named_decimal_int),
            ::core::stringify!(log_address),
            ::core::stringify!(log_array_1),
            ::core::stringify!(log_named_address),
            ::core::stringify!(InitializeStoffelAccessControl),
            ::core::stringify!(log_named_array_1),
            ::core::stringify!(log_named_bytes32),
            ::core::stringify!(log_named_uint),
            ::core::stringify!(log_named_bytes),
            ::core::stringify!(logs),
            ::core::stringify!(log_bytes32),
            ::core::stringify!(log_named_decimal_uint),
            ::core::stringify!(log_array_0),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <log_named_array_0 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_string as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_int as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_bytes as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_string as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_uint as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_int as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_array_2 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_array_2 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_decimal_int as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_address as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_array_1 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_address as alloy_sol_types::SolEvent>::SIGNATURE,
            <InitializeStoffelAccessControl as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_array_1 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_uint as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_bytes as alloy_sol_types::SolEvent>::SIGNATURE,
            <logs as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_decimal_uint as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_array_0 as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for StoffelAccessControlTestEvents {
        const NAME: &'static str = "StoffelAccessControlTestEvents";
        const COUNT: usize = 23usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <InitializeStoffelAccessControl as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <InitializeStoffelAccessControl as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::InitializeStoffelAccessControl)
                }
                Some(<log as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::log)
                }
                Some(<log_address as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_address as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_address)
                }
                Some(<log_array_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_array_0)
                }
                Some(<log_array_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_array_1)
                }
                Some(<log_array_2 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_2 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_array_2)
                }
                Some(<log_bytes as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_bytes as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_bytes)
                }
                Some(<log_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_bytes32 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_bytes32)
                }
                Some(<log_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_int as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::log_int)
                }
                Some(
                    <log_named_address as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_address as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_address)
                }
                Some(
                    <log_named_array_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_array_0)
                }
                Some(
                    <log_named_array_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_array_1)
                }
                Some(
                    <log_named_array_2 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_2 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_array_2)
                }
                Some(<log_named_bytes as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_bytes as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_bytes)
                }
                Some(
                    <log_named_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_bytes32 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_bytes32)
                }
                Some(
                    <log_named_decimal_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_decimal_int as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_decimal_int)
                }
                Some(
                    <log_named_decimal_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_decimal_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_decimal_uint)
                }
                Some(<log_named_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_int as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_int)
                }
                Some(<log_named_string as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_string as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_string)
                }
                Some(<log_named_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_uint)
                }
                Some(<log_string as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_string as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_string)
                }
                Some(<log_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_uint as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::log_uint)
                }
                Some(<logs as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <logs as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
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
    impl alloy_sol_types::private::IntoLogData for StoffelAccessControlTestEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::InitializeStoffelAccessControl(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
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
                Self::InitializeStoffelAccessControl(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
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
    /**Creates a new wrapper around an on-chain [`StoffelAccessControlTest`](self) contract instance.

See the [wrapper's documentation](`StoffelAccessControlTestInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> StoffelAccessControlTestInstance<P, N> {
        StoffelAccessControlTestInstance::<P, N>::new(address, __provider)
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<StoffelAccessControlTestInstance<P, N>>,
    > {
        StoffelAccessControlTestInstance::<P, N>::deploy(__provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
        StoffelAccessControlTestInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`StoffelAccessControlTest`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`StoffelAccessControlTest`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct StoffelAccessControlTestInstance<
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for StoffelAccessControlTestInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("StoffelAccessControlTestInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > StoffelAccessControlTestInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`StoffelAccessControlTest`](self) contract instance.

See the [wrapper's documentation](`StoffelAccessControlTestInstance`) for more details.*/
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
        ) -> alloy_contract::Result<StoffelAccessControlTestInstance<P, N>> {
            let call_builder = Self::deploy_builder(__provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
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
    impl<P: ::core::clone::Clone, N> StoffelAccessControlTestInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> StoffelAccessControlTestInstance<P, N> {
            StoffelAccessControlTestInstance {
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
    > StoffelAccessControlTestInstance<P, N> {
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
        ///Creates a new call builder for the [`IS_TEST`] function.
        pub fn IS_TEST(&self) -> alloy_contract::SolCallBuilder<&P, IS_TESTCall, N> {
            self.call_builder(&IS_TESTCall)
        }
        ///Creates a new call builder for the [`ac`] function.
        pub fn ac(&self) -> alloy_contract::SolCallBuilder<&P, acCall, N> {
            self.call_builder(&acCall)
        }
        ///Creates a new call builder for the [`excludeArtifacts`] function.
        pub fn excludeArtifacts(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, excludeArtifactsCall, N> {
            self.call_builder(&excludeArtifactsCall)
        }
        ///Creates a new call builder for the [`excludeContracts`] function.
        pub fn excludeContracts(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, excludeContractsCall, N> {
            self.call_builder(&excludeContractsCall)
        }
        ///Creates a new call builder for the [`excludeSelectors`] function.
        pub fn excludeSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, excludeSelectorsCall, N> {
            self.call_builder(&excludeSelectorsCall)
        }
        ///Creates a new call builder for the [`excludeSenders`] function.
        pub fn excludeSenders(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, excludeSendersCall, N> {
            self.call_builder(&excludeSendersCall)
        }
        ///Creates a new call builder for the [`failed`] function.
        pub fn failed(&self) -> alloy_contract::SolCallBuilder<&P, failedCall, N> {
            self.call_builder(&failedCall)
        }
        ///Creates a new call builder for the [`setUp`] function.
        pub fn setUp(&self) -> alloy_contract::SolCallBuilder<&P, setUpCall, N> {
            self.call_builder(&setUpCall)
        }
        ///Creates a new call builder for the [`targetArtifactSelectors`] function.
        pub fn targetArtifactSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, targetArtifactSelectorsCall, N> {
            self.call_builder(&targetArtifactSelectorsCall)
        }
        ///Creates a new call builder for the [`targetArtifacts`] function.
        pub fn targetArtifacts(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, targetArtifactsCall, N> {
            self.call_builder(&targetArtifactsCall)
        }
        ///Creates a new call builder for the [`targetContracts`] function.
        pub fn targetContracts(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, targetContractsCall, N> {
            self.call_builder(&targetContractsCall)
        }
        ///Creates a new call builder for the [`targetInterfaces`] function.
        pub fn targetInterfaces(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, targetInterfacesCall, N> {
            self.call_builder(&targetInterfacesCall)
        }
        ///Creates a new call builder for the [`targetSelectors`] function.
        pub fn targetSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, targetSelectorsCall, N> {
            self.call_builder(&targetSelectorsCall)
        }
        ///Creates a new call builder for the [`targetSenders`] function.
        pub fn targetSenders(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, targetSendersCall, N> {
            self.call_builder(&targetSendersCall)
        }
        ///Creates a new call builder for the [`test_constructor_emitsInitializeEvent`] function.
        pub fn test_constructor_emitsInitializeEvent(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_constructor_emitsInitializeEventCall,
            N,
        > {
            self.call_builder(&test_constructor_emitsInitializeEventCall)
        }
        ///Creates a new call builder for the [`test_constructor_grantRoles`] function.
        pub fn test_constructor_grantRoles(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_constructor_grantRolesCall, N> {
            self.call_builder(&test_constructor_grantRolesCall)
        }
        ///Creates a new call builder for the [`test_grantRoleAgain`] function.
        pub fn test_grantRoleAgain(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_grantRoleAgainCall, N> {
            self.call_builder(&test_grantRoleAgainCall)
        }
        ///Creates a new call builder for the [`test_grantRole_designatedPartyCanGrantRoles`] function.
        pub fn test_grantRole_designatedPartyCanGrantRoles(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_grantRole_designatedPartyCanGrantRolesCall,
            N,
        > {
            self.call_builder(&test_grantRole_designatedPartyCanGrantRolesCall)
        }
        ///Creates a new call builder for the [`test_grantRole_revertsIfCallerLacksDesignatedPartyRole`] function.
        pub fn test_grantRole_revertsIfCallerLacksDesignatedPartyRole(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_grantRole_revertsIfCallerLacksDesignatedPartyRoleCall,
            N,
        > {
            self.call_builder(
                &test_grantRole_revertsIfCallerLacksDesignatedPartyRoleCall,
            )
        }
        ///Creates a new call builder for the [`test_isDesignatedParty`] function.
        pub fn test_isDesignatedParty(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_isDesignatedPartyCall, N> {
            self.call_builder(&test_isDesignatedPartyCall)
        }
        ///Creates a new call builder for the [`test_isParty`] function.
        pub fn test_isParty(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, test_isPartyCall, N> {
            self.call_builder(&test_isPartyCall)
        }
        ///Creates a new call builder for the [`test_renounceRole_partyCanRenounceOwnRole`] function.
        pub fn test_renounceRole_partyCanRenounceOwnRole(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_renounceRole_partyCanRenounceOwnRoleCall,
            N,
        > {
            self.call_builder(&test_renounceRole_partyCanRenounceOwnRoleCall)
        }
        ///Creates a new call builder for the [`test_renounceRole_revertsIfRenouncingForAnotherAccount`] function.
        pub fn test_renounceRole_revertsIfRenouncingForAnotherAccount(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_renounceRole_revertsIfRenouncingForAnotherAccountCall,
            N,
        > {
            self.call_builder(
                &test_renounceRole_revertsIfRenouncingForAnotherAccountCall,
            )
        }
        ///Creates a new call builder for the [`test_revokeRole_designatedPartyCanRevokePartyRole`] function.
        pub fn test_revokeRole_designatedPartyCanRevokePartyRole(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_revokeRole_designatedPartyCanRevokePartyRoleCall,
            N,
        > {
            self.call_builder(&test_revokeRole_designatedPartyCanRevokePartyRoleCall)
        }
        ///Creates a new call builder for the [`test_revokeRole_partyCountDecreases`] function.
        pub fn test_revokeRole_partyCountDecreases(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_revokeRole_partyCountDecreasesCall,
            N,
        > {
            self.call_builder(&test_revokeRole_partyCountDecreasesCall)
        }
        ///Creates a new call builder for the [`test_revokeRole_revertsIfCallerLacksDesignatedPartyRole`] function.
        pub fn test_revokeRole_revertsIfCallerLacksDesignatedPartyRole(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_revokeRole_revertsIfCallerLacksDesignatedPartyRoleCall,
            N,
        > {
            self.call_builder(
                &test_revokeRole_revertsIfCallerLacksDesignatedPartyRoleCall,
            )
        }
        ///Creates a new call builder for the [`test_supportsInterface_IStoffelAccessControl`] function.
        pub fn test_supportsInterface_IStoffelAccessControl(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            test_supportsInterface_IStoffelAccessControlCall,
            N,
        > {
            self.call_builder(&test_supportsInterface_IStoffelAccessControlCall)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > StoffelAccessControlTestInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`InitializeStoffelAccessControl`] event.
        pub fn InitializeStoffelAccessControl_filter(
            &self,
        ) -> alloy_contract::Event<&P, InitializeStoffelAccessControl, N> {
            self.event_filter::<InitializeStoffelAccessControl>()
        }
        ///Creates a new event filter for the [`log`] event.
        pub fn log_filter(&self) -> alloy_contract::Event<&P, log, N> {
            self.event_filter::<log>()
        }
        ///Creates a new event filter for the [`log_address`] event.
        pub fn log_address_filter(&self) -> alloy_contract::Event<&P, log_address, N> {
            self.event_filter::<log_address>()
        }
        ///Creates a new event filter for the [`log_array_0`] event.
        pub fn log_array_0_filter(&self) -> alloy_contract::Event<&P, log_array_0, N> {
            self.event_filter::<log_array_0>()
        }
        ///Creates a new event filter for the [`log_array_1`] event.
        pub fn log_array_1_filter(&self) -> alloy_contract::Event<&P, log_array_1, N> {
            self.event_filter::<log_array_1>()
        }
        ///Creates a new event filter for the [`log_array_2`] event.
        pub fn log_array_2_filter(&self) -> alloy_contract::Event<&P, log_array_2, N> {
            self.event_filter::<log_array_2>()
        }
        ///Creates a new event filter for the [`log_bytes`] event.
        pub fn log_bytes_filter(&self) -> alloy_contract::Event<&P, log_bytes, N> {
            self.event_filter::<log_bytes>()
        }
        ///Creates a new event filter for the [`log_bytes32`] event.
        pub fn log_bytes32_filter(&self) -> alloy_contract::Event<&P, log_bytes32, N> {
            self.event_filter::<log_bytes32>()
        }
        ///Creates a new event filter for the [`log_int`] event.
        pub fn log_int_filter(&self) -> alloy_contract::Event<&P, log_int, N> {
            self.event_filter::<log_int>()
        }
        ///Creates a new event filter for the [`log_named_address`] event.
        pub fn log_named_address_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_address, N> {
            self.event_filter::<log_named_address>()
        }
        ///Creates a new event filter for the [`log_named_array_0`] event.
        pub fn log_named_array_0_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_array_0, N> {
            self.event_filter::<log_named_array_0>()
        }
        ///Creates a new event filter for the [`log_named_array_1`] event.
        pub fn log_named_array_1_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_array_1, N> {
            self.event_filter::<log_named_array_1>()
        }
        ///Creates a new event filter for the [`log_named_array_2`] event.
        pub fn log_named_array_2_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_array_2, N> {
            self.event_filter::<log_named_array_2>()
        }
        ///Creates a new event filter for the [`log_named_bytes`] event.
        pub fn log_named_bytes_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_bytes, N> {
            self.event_filter::<log_named_bytes>()
        }
        ///Creates a new event filter for the [`log_named_bytes32`] event.
        pub fn log_named_bytes32_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_bytes32, N> {
            self.event_filter::<log_named_bytes32>()
        }
        ///Creates a new event filter for the [`log_named_decimal_int`] event.
        pub fn log_named_decimal_int_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_decimal_int, N> {
            self.event_filter::<log_named_decimal_int>()
        }
        ///Creates a new event filter for the [`log_named_decimal_uint`] event.
        pub fn log_named_decimal_uint_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_decimal_uint, N> {
            self.event_filter::<log_named_decimal_uint>()
        }
        ///Creates a new event filter for the [`log_named_int`] event.
        pub fn log_named_int_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_int, N> {
            self.event_filter::<log_named_int>()
        }
        ///Creates a new event filter for the [`log_named_string`] event.
        pub fn log_named_string_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_string, N> {
            self.event_filter::<log_named_string>()
        }
        ///Creates a new event filter for the [`log_named_uint`] event.
        pub fn log_named_uint_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_uint, N> {
            self.event_filter::<log_named_uint>()
        }
        ///Creates a new event filter for the [`log_string`] event.
        pub fn log_string_filter(&self) -> alloy_contract::Event<&P, log_string, N> {
            self.event_filter::<log_string>()
        }
        ///Creates a new event filter for the [`log_uint`] event.
        pub fn log_uint_filter(&self) -> alloy_contract::Event<&P, log_uint, N> {
            self.event_filter::<log_uint>()
        }
        ///Creates a new event filter for the [`logs`] event.
        pub fn logs_filter(&self) -> alloy_contract::Event<&P, logs, N> {
            self.event_filter::<logs>()
        }
    }
}

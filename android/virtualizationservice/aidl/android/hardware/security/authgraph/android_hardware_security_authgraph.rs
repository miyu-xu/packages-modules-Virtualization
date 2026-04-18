#![allow(non_snake_case)]
#![allow(missing_docs)]
#[deprecated(note = "Please access via libbinder_rs binder::")]
pub use binder;
pub mod aidl {
  pub mod android {
    pub mod hardware {
      pub mod security {
        pub mod authgraph {
          pub mod Arc {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 1 --hash 3758824e7b75acdb1ca66620fb8a8aec0ec6dfcc --stability vintf --min_sdk_version current --ninja -d out/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph-V1-rust-source/gen/android/hardware/security/authgraph/Arc.rs.d -o out/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph-V1-rust-source/gen -Nhardware/interfaces/security/authgraph/aidl/aidl_api/android.hardware.security.authgraph/1 hardware/interfaces/security/authgraph/aidl/aidl_api/android.hardware.security.authgraph/1/android/hardware/security/authgraph/Arc.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug, Clone, Eq, PartialEq)]
            pub struct r#Arc {
              pub r#arc: Vec<u8>,
            }
            impl Default for r#Arc {
              fn default() -> Self {
                Self {
                  r#arc: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#Arc {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#arc)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#arc = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#Arc);
            binder::impl_deserialize_for_parcelable!(r#Arc);
            impl binder::binder_impl::ParcelableMetadata for r#Arc {
              fn get_descriptor() -> &'static str { "android.hardware.security.authgraph.Arc" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#Arc as _7_android_8_hardware_8_security_9_authgraph_3_Arc;
            }
          }
          pub mod Error {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 1 --hash 3758824e7b75acdb1ca66620fb8a8aec0ec6dfcc --stability vintf --min_sdk_version current --ninja -d out/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph-V1-rust-source/gen/android/hardware/security/authgraph/Error.rs.d -o out/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph-V1-rust-source/gen -Nhardware/interfaces/security/authgraph/aidl/aidl_api/android.hardware.security.authgraph/1 hardware/interfaces/security/authgraph/aidl/aidl_api/android.hardware.security.authgraph/1/android/hardware/security/authgraph/Error.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(4))]
              r#Error : [i32; 12] {
                r#OK = 0,
                r#INVALID_PEER_NONCE = -1,
                r#INVALID_PEER_KE_KEY = -2,
                r#INVALID_IDENTITY = -3,
                r#INVALID_CERT_CHAIN = -4,
                r#INVALID_SIGNATURE = -5,
                r#INVALID_KE_KEY = -6,
                r#INVALID_PUB_KEY_IN_KEY = -7,
                r#INVALID_PRIV_KEY_ARC_IN_KEY = -8,
                r#INVALID_SHARED_KEY_ARCS = -9,
                r#MEMORY_ALLOCATION_FAILED = -10,
                r#INCOMPATIBLE_PROTOCOL_VERSION = -11,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#Error as _7_android_8_hardware_8_security_9_authgraph_5_Error;
            }
          }
          pub mod IAuthGraphKeyExchange {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 1 --hash 3758824e7b75acdb1ca66620fb8a8aec0ec6dfcc --stability vintf --min_sdk_version current --ninja -d out/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph-V1-rust-source/gen/android/hardware/security/authgraph/IAuthGraphKeyExchange.rs.d -o out/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph-V1-rust-source/gen -Nhardware/interfaces/security/authgraph/aidl/aidl_api/android.hardware.security.authgraph/1 hardware/interfaces/security/authgraph/aidl/aidl_api/android.hardware.security.authgraph/1/android/hardware/security/authgraph/IAuthGraphKeyExchange.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              IAuthGraphKeyExchange["android.hardware.security.authgraph.IAuthGraphKeyExchange"] {
                native: BnAuthGraphKeyExchange(on_transact),
                proxy: BpAuthGraphKeyExchange {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IAuthGraphKeyExchangeAsync(try_into_local_async),
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IAuthGraphKeyExchange: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.security.authgraph.IAuthGraphKeyExchange" }
              fn r#create(&self) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_21_SessionInitiationInfo>;
              fn r#init(&self, _arg_peerPubKey: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey, _arg_peerId: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity, _arg_peerNonce: &[u8], _arg_peerVersion: i32) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_12_KeInitResult>;
              fn r#finish(&self, _arg_peerPubKey: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey, _arg_peerId: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity, _arg_peerSignature: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature, _arg_peerNonce: &[u8], _arg_peerVersion: i32, _arg_ownKey: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Key) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_11_SessionInfo>;
              fn r#authenticationComplete(&self, _arg_peerSignature: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature, _arg_sharedKeys: &[crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc; 2]) -> binder::Result<[crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc; 2]>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IAuthGraphKeyExchangeDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IAuthGraphKeyExchangeDefaultRef) -> IAuthGraphKeyExchangeDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
              fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn IAuthGraphKeyExchangeAsyncServer + Send + Sync)> {
                None
              }
            }
            pub trait IAuthGraphKeyExchangeAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.security.authgraph.IAuthGraphKeyExchange" }
              fn r#create<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_21_SessionInitiationInfo>>;
              fn r#init<'a>(&'a self, _arg_peerPubKey: &'a crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey, _arg_peerId: &'a crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity, _arg_peerNonce: &'a [u8], _arg_peerVersion: i32) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_12_KeInitResult>>;
              fn r#finish<'a>(&'a self, _arg_peerPubKey: &'a crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey, _arg_peerId: &'a crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity, _arg_peerSignature: &'a crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature, _arg_peerNonce: &'a [u8], _arg_peerVersion: i32, _arg_ownKey: &'a crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Key) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_11_SessionInfo>>;
              fn r#authenticationComplete<'a>(&'a self, _arg_peerSignature: &'a crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature, _arg_sharedKeys: &'a [crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc; 2]) -> binder::BoxFuture<'a, binder::Result<[crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc; 2]>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IAuthGraphKeyExchangeAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.security.authgraph.IAuthGraphKeyExchange" }
              async fn r#create(&self) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_21_SessionInitiationInfo>;
              async fn r#init(&self, _arg_peerPubKey: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey, _arg_peerId: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity, _arg_peerNonce: &[u8], _arg_peerVersion: i32) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_12_KeInitResult>;
              async fn r#finish(&self, _arg_peerPubKey: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey, _arg_peerId: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity, _arg_peerSignature: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature, _arg_peerNonce: &[u8], _arg_peerVersion: i32, _arg_ownKey: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Key) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_11_SessionInfo>;
              async fn r#authenticationComplete(&self, _arg_peerSignature: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature, _arg_sharedKeys: &[crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc; 2]) -> binder::Result<[crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc; 2]>;
            }
            impl BnAuthGraphKeyExchange {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IAuthGraphKeyExchange>
              where
                T: IAuthGraphKeyExchangeAsyncServer + binder::Interface + Send + Sync + 'static,
                R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
              {
                struct Wrapper<T, R> {
                  _inner: T,
                  _rt: R,
                }
                impl<T, R> binder::Interface for Wrapper<T, R> where T: binder::Interface, R: Send + Sync + 'static {
                  fn as_binder(&self) -> binder::SpIBinder { self._inner.as_binder() }
                  fn dump(&self, _writer: &mut dyn std::io::Write, _args: &[&std::ffi::CStr]) -> std::result::Result<(), binder::StatusCode> { self._inner.dump(_writer, _args) }
                }
                impl<T, R> IAuthGraphKeyExchange for Wrapper<T, R>
                where
                  T: IAuthGraphKeyExchangeAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#create(&self) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_21_SessionInitiationInfo> {
                    self._rt.block_on(self._inner.r#create())
                  }
                  fn r#init(&self, _arg_peerPubKey: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey, _arg_peerId: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity, _arg_peerNonce: &[u8], _arg_peerVersion: i32) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_12_KeInitResult> {
                    self._rt.block_on(self._inner.r#init(_arg_peerPubKey, _arg_peerId, _arg_peerNonce, _arg_peerVersion))
                  }
                  fn r#finish(&self, _arg_peerPubKey: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey, _arg_peerId: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity, _arg_peerSignature: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature, _arg_peerNonce: &[u8], _arg_peerVersion: i32, _arg_ownKey: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Key) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_11_SessionInfo> {
                    self._rt.block_on(self._inner.r#finish(_arg_peerPubKey, _arg_peerId, _arg_peerSignature, _arg_peerNonce, _arg_peerVersion, _arg_ownKey))
                  }
                  fn r#authenticationComplete(&self, _arg_peerSignature: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature, _arg_sharedKeys: &[crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc; 2]) -> binder::Result<[crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc; 2]> {
                    self._rt.block_on(self._inner.r#authenticationComplete(_arg_peerSignature, _arg_sharedKeys))
                  }
                  fn try_as_async_server(&self) -> Option<&(dyn IAuthGraphKeyExchangeAsyncServer + Send + Sync)> {
                    Some(&self._inner)
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
              pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn IAuthGraphKeyExchangeAsync<P>>> {
                struct Wrapper {
                  _native: binder::binder_impl::Binder<BnAuthGraphKeyExchange>
                }
                impl binder::Interface for Wrapper {}
                impl<P: binder::BinderAsyncPool> IAuthGraphKeyExchangeAsync<P> for Wrapper {
                  fn r#create<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_21_SessionInitiationInfo>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#create())
                  }
                  fn r#init<'a>(&'a self, _arg_peerPubKey: &'a crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey, _arg_peerId: &'a crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity, _arg_peerNonce: &'a [u8], _arg_peerVersion: i32) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_12_KeInitResult>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#init(_arg_peerPubKey, _arg_peerId, _arg_peerNonce, _arg_peerVersion))
                  }
                  fn r#finish<'a>(&'a self, _arg_peerPubKey: &'a crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey, _arg_peerId: &'a crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity, _arg_peerSignature: &'a crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature, _arg_peerNonce: &'a [u8], _arg_peerVersion: i32, _arg_ownKey: &'a crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Key) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_11_SessionInfo>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#finish(_arg_peerPubKey, _arg_peerId, _arg_peerSignature, _arg_peerNonce, _arg_peerVersion, _arg_ownKey))
                  }
                  fn r#authenticationComplete<'a>(&'a self, _arg_peerSignature: &'a crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature, _arg_sharedKeys: &'a [crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc; 2]) -> binder::BoxFuture<'a, binder::Result<[crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc; 2]>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#authenticationComplete(_arg_peerSignature, _arg_sharedKeys))
                  }
                }
                if _native.try_as_async_server().is_some() {
                  Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn IAuthGraphKeyExchangeAsync<P>>))
                } else {
                  None
                }
              }
            }
            pub trait IAuthGraphKeyExchangeDefault: Send + Sync {
              fn r#create(&self) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_21_SessionInitiationInfo> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#init(&self, _arg_peerPubKey: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey, _arg_peerId: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity, _arg_peerNonce: &[u8], _arg_peerVersion: i32) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_12_KeInitResult> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#finish(&self, _arg_peerPubKey: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey, _arg_peerId: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity, _arg_peerSignature: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature, _arg_peerNonce: &[u8], _arg_peerVersion: i32, _arg_ownKey: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Key) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_11_SessionInfo> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#authenticationComplete(&self, _arg_peerSignature: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature, _arg_sharedKeys: &[crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc; 2]) -> binder::Result<[crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc; 2]> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#create: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#init: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#finish: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#authenticationComplete: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IAuthGraphKeyExchangeDefaultRef = Option<std::sync::Arc<dyn IAuthGraphKeyExchangeDefault>>;
            static DEFAULT_IMPL: std::sync::Mutex<IAuthGraphKeyExchangeDefaultRef> = std::sync::Mutex::new(None);
            pub const VERSION: i32 = 1;
            pub const HASH: &str = "3758824e7b75acdb1ca66620fb8a8aec0ec6dfcc";
            impl BpAuthGraphKeyExchange {
              fn build_parcel_create(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_create(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_21_SessionInitiationInfo> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IAuthGraphKeyExchange>::getDefaultImpl() {
                    return _aidl_default_impl.r#create();
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: crate::mangled::_7_android_8_hardware_8_security_9_authgraph_21_SessionInitiationInfo = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_init(&self, _arg_peerPubKey: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey, _arg_peerId: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity, _arg_peerNonce: &[u8], _arg_peerVersion: i32) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_peerPubKey)?;
                aidl_data.write(_arg_peerId)?;
                aidl_data.write(_arg_peerNonce)?;
                aidl_data.write(&_arg_peerVersion)?;
                Ok(aidl_data)
              }
              fn read_response_init(&self, _arg_peerPubKey: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey, _arg_peerId: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity, _arg_peerNonce: &[u8], _arg_peerVersion: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_12_KeInitResult> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IAuthGraphKeyExchange>::getDefaultImpl() {
                    return _aidl_default_impl.r#init(_arg_peerPubKey, _arg_peerId, _arg_peerNonce, _arg_peerVersion);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: crate::mangled::_7_android_8_hardware_8_security_9_authgraph_12_KeInitResult = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_finish(&self, _arg_peerPubKey: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey, _arg_peerId: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity, _arg_peerSignature: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature, _arg_peerNonce: &[u8], _arg_peerVersion: i32, _arg_ownKey: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Key) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_peerPubKey)?;
                aidl_data.write(_arg_peerId)?;
                aidl_data.write(_arg_peerSignature)?;
                aidl_data.write(_arg_peerNonce)?;
                aidl_data.write(&_arg_peerVersion)?;
                aidl_data.write(_arg_ownKey)?;
                Ok(aidl_data)
              }
              fn read_response_finish(&self, _arg_peerPubKey: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey, _arg_peerId: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity, _arg_peerSignature: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature, _arg_peerNonce: &[u8], _arg_peerVersion: i32, _arg_ownKey: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Key, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_11_SessionInfo> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IAuthGraphKeyExchange>::getDefaultImpl() {
                    return _aidl_default_impl.r#finish(_arg_peerPubKey, _arg_peerId, _arg_peerSignature, _arg_peerNonce, _arg_peerVersion, _arg_ownKey);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: crate::mangled::_7_android_8_hardware_8_security_9_authgraph_11_SessionInfo = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_authenticationComplete(&self, _arg_peerSignature: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature, _arg_sharedKeys: &[crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc; 2]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_peerSignature)?;
                aidl_data.write(_arg_sharedKeys)?;
                Ok(aidl_data)
              }
              fn read_response_authenticationComplete(&self, _arg_peerSignature: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature, _arg_sharedKeys: &[crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc; 2], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<[crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc; 2]> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IAuthGraphKeyExchange>::getDefaultImpl() {
                    return _aidl_default_impl.r#authenticationComplete(_arg_peerSignature, _arg_sharedKeys);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: [crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc; 2] = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getInterfaceVersion(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_getInterfaceVersion(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: i32 = _aidl_reply.read()?;
                self.cached_version.store(_aidl_return, std::sync::atomic::Ordering::Relaxed);
                Ok(_aidl_return)
              }
              fn build_parcel_getInterfaceHash(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_getInterfaceHash(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: String = _aidl_reply.read()?;
                *self.cached_hash.lock().unwrap() = Some(_aidl_return.clone());
                Ok(_aidl_return)
              }
            }
            impl IAuthGraphKeyExchange for BpAuthGraphKeyExchange {
              fn r#create(&self) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_21_SessionInitiationInfo> {
                let _aidl_data = self.build_parcel_create()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#create, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_create(_aidl_reply)
              }
              fn r#init(&self, _arg_peerPubKey: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey, _arg_peerId: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity, _arg_peerNonce: &[u8], _arg_peerVersion: i32) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_12_KeInitResult> {
                let _aidl_data = self.build_parcel_init(_arg_peerPubKey, _arg_peerId, _arg_peerNonce, _arg_peerVersion)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#init, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_init(_arg_peerPubKey, _arg_peerId, _arg_peerNonce, _arg_peerVersion, _aidl_reply)
              }
              fn r#finish(&self, _arg_peerPubKey: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey, _arg_peerId: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity, _arg_peerSignature: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature, _arg_peerNonce: &[u8], _arg_peerVersion: i32, _arg_ownKey: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Key) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_11_SessionInfo> {
                let _aidl_data = self.build_parcel_finish(_arg_peerPubKey, _arg_peerId, _arg_peerSignature, _arg_peerNonce, _arg_peerVersion, _arg_ownKey)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#finish, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_finish(_arg_peerPubKey, _arg_peerId, _arg_peerSignature, _arg_peerNonce, _arg_peerVersion, _arg_ownKey, _aidl_reply)
              }
              fn r#authenticationComplete(&self, _arg_peerSignature: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature, _arg_sharedKeys: &[crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc; 2]) -> binder::Result<[crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc; 2]> {
                let _aidl_data = self.build_parcel_authenticationComplete(_arg_peerSignature, _arg_sharedKeys)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#authenticationComplete, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_authenticationComplete(_arg_peerSignature, _arg_sharedKeys, _aidl_reply)
              }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                let _aidl_version = self.cached_version.load(std::sync::atomic::Ordering::Relaxed);
                if _aidl_version != -1 { return Ok(_aidl_version); }
                let _aidl_data = self.build_parcel_getInterfaceVersion()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getInterfaceVersion, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getInterfaceVersion(_aidl_reply)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                {
                  let _aidl_hash_lock = self.cached_hash.lock().unwrap();
                  if let Some(ref _aidl_hash) = *_aidl_hash_lock {
                    return Ok(_aidl_hash.clone());
                  }
                }
                let _aidl_data = self.build_parcel_getInterfaceHash()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getInterfaceHash, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getInterfaceHash(_aidl_reply)
              }
            }
            impl<P: binder::BinderAsyncPool> IAuthGraphKeyExchangeAsync<P> for BpAuthGraphKeyExchange {
              fn r#create<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_21_SessionInitiationInfo>> {
                let _aidl_data = match self.build_parcel_create() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#create, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_create(_aidl_reply)
                  }
                )
              }
              fn r#init<'a>(&'a self, _arg_peerPubKey: &'a crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey, _arg_peerId: &'a crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity, _arg_peerNonce: &'a [u8], _arg_peerVersion: i32) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_12_KeInitResult>> {
                let _aidl_data = match self.build_parcel_init(_arg_peerPubKey, _arg_peerId, _arg_peerNonce, _arg_peerVersion) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#init, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_init(_arg_peerPubKey, _arg_peerId, _arg_peerNonce, _arg_peerVersion, _aidl_reply)
                  }
                )
              }
              fn r#finish<'a>(&'a self, _arg_peerPubKey: &'a crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey, _arg_peerId: &'a crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity, _arg_peerSignature: &'a crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature, _arg_peerNonce: &'a [u8], _arg_peerVersion: i32, _arg_ownKey: &'a crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Key) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_11_SessionInfo>> {
                let _aidl_data = match self.build_parcel_finish(_arg_peerPubKey, _arg_peerId, _arg_peerSignature, _arg_peerNonce, _arg_peerVersion, _arg_ownKey) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#finish, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_finish(_arg_peerPubKey, _arg_peerId, _arg_peerSignature, _arg_peerNonce, _arg_peerVersion, _arg_ownKey, _aidl_reply)
                  }
                )
              }
              fn r#authenticationComplete<'a>(&'a self, _arg_peerSignature: &'a crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature, _arg_sharedKeys: &'a [crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc; 2]) -> binder::BoxFuture<'a, binder::Result<[crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc; 2]>> {
                let _aidl_data = match self.build_parcel_authenticationComplete(_arg_peerSignature, _arg_sharedKeys) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#authenticationComplete, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_authenticationComplete(_arg_peerSignature, _arg_sharedKeys, _aidl_reply)
                  }
                )
              }
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                let _aidl_version = self.cached_version.load(std::sync::atomic::Ordering::Relaxed);
                if _aidl_version != -1 { return Box::pin(std::future::ready(Ok(_aidl_version))); }
                let _aidl_data = match self.build_parcel_getInterfaceVersion() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getInterfaceVersion, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getInterfaceVersion(_aidl_reply)
                  }
                )
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                {
                  let _aidl_hash_lock = self.cached_hash.lock().unwrap();
                  if let Some(ref _aidl_hash) = *_aidl_hash_lock {
                    return Box::pin(std::future::ready(Ok(_aidl_hash.clone())));
                  }
                }
                let _aidl_data = match self.build_parcel_getInterfaceHash() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getInterfaceHash, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getInterfaceHash(_aidl_reply)
                  }
                )
              }
            }
            impl IAuthGraphKeyExchange for binder::binder_impl::Binder<BnAuthGraphKeyExchange> {
              fn r#create(&self) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_21_SessionInitiationInfo> { self.0.r#create() }
              fn r#init(&self, _arg_peerPubKey: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey, _arg_peerId: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity, _arg_peerNonce: &[u8], _arg_peerVersion: i32) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_12_KeInitResult> { self.0.r#init(_arg_peerPubKey, _arg_peerId, _arg_peerNonce, _arg_peerVersion) }
              fn r#finish(&self, _arg_peerPubKey: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey, _arg_peerId: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity, _arg_peerSignature: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature, _arg_peerNonce: &[u8], _arg_peerVersion: i32, _arg_ownKey: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Key) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_11_SessionInfo> { self.0.r#finish(_arg_peerPubKey, _arg_peerId, _arg_peerSignature, _arg_peerNonce, _arg_peerVersion, _arg_ownKey) }
              fn r#authenticationComplete(&self, _arg_peerSignature: &crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature, _arg_sharedKeys: &[crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc; 2]) -> binder::Result<[crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc; 2]> { self.0.r#authenticationComplete(_arg_peerSignature, _arg_sharedKeys) }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IAuthGraphKeyExchange, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#create => {
                  let _aidl_return = _aidl_service.r#create();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#init => {
                  let _arg_peerPubKey: crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey = _aidl_data.read()?;
                  let _arg_peerId: crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity = _aidl_data.read()?;
                  let _arg_peerNonce: Vec<u8> = _aidl_data.read()?;
                  let _arg_peerVersion: i32 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#init(&_arg_peerPubKey, &_arg_peerId, &_arg_peerNonce, _arg_peerVersion);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#finish => {
                  let _arg_peerPubKey: crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey = _aidl_data.read()?;
                  let _arg_peerId: crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity = _aidl_data.read()?;
                  let _arg_peerSignature: crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature = _aidl_data.read()?;
                  let _arg_peerNonce: Vec<u8> = _aidl_data.read()?;
                  let _arg_peerVersion: i32 = _aidl_data.read()?;
                  let _arg_ownKey: crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Key = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#finish(&_arg_peerPubKey, &_arg_peerId, &_arg_peerSignature, &_arg_peerNonce, _arg_peerVersion, &_arg_ownKey);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#authenticationComplete => {
                  let _arg_peerSignature: crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature = _aidl_data.read()?;
                  let _arg_sharedKeys: [crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc; 2] = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#authenticationComplete(&_arg_peerSignature, &_arg_sharedKeys);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getInterfaceVersion => {
                  let _aidl_return = _aidl_service.r#getInterfaceVersion();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getInterfaceHash => {
                  let _aidl_return = _aidl_service.r#getInterfaceHash();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
              }
            }
            pub(crate) mod mangled {
             pub use super::r#IAuthGraphKeyExchange as _7_android_8_hardware_8_security_9_authgraph_21_IAuthGraphKeyExchange;
            }
          }
          pub mod Identity {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 1 --hash 3758824e7b75acdb1ca66620fb8a8aec0ec6dfcc --stability vintf --min_sdk_version current --ninja -d out/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph-V1-rust-source/gen/android/hardware/security/authgraph/Identity.rs.d -o out/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph-V1-rust-source/gen -Nhardware/interfaces/security/authgraph/aidl/aidl_api/android.hardware.security.authgraph/1 hardware/interfaces/security/authgraph/aidl/aidl_api/android.hardware.security.authgraph/1/android/hardware/security/authgraph/Identity.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug, Clone, Eq, PartialEq)]
            pub struct r#Identity {
              pub r#identity: Vec<u8>,
            }
            impl Default for r#Identity {
              fn default() -> Self {
                Self {
                  r#identity: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#Identity {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#identity)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#identity = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#Identity);
            binder::impl_deserialize_for_parcelable!(r#Identity);
            impl binder::binder_impl::ParcelableMetadata for r#Identity {
              fn get_descriptor() -> &'static str { "android.hardware.security.authgraph.Identity" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#Identity as _7_android_8_hardware_8_security_9_authgraph_8_Identity;
            }
          }
          pub mod KeInitResult {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 1 --hash 3758824e7b75acdb1ca66620fb8a8aec0ec6dfcc --stability vintf --min_sdk_version current --ninja -d out/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph-V1-rust-source/gen/android/hardware/security/authgraph/KeInitResult.rs.d -o out/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph-V1-rust-source/gen -Nhardware/interfaces/security/authgraph/aidl/aidl_api/android.hardware.security.authgraph/1 hardware/interfaces/security/authgraph/aidl/aidl_api/android.hardware.security.authgraph/1/android/hardware/security/authgraph/KeInitResult.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug, Clone, Eq, PartialEq)]
            pub struct r#KeInitResult {
              pub r#sessionInitiationInfo: crate::mangled::_7_android_8_hardware_8_security_9_authgraph_21_SessionInitiationInfo,
              pub r#sessionInfo: crate::mangled::_7_android_8_hardware_8_security_9_authgraph_11_SessionInfo,
            }
            impl Default for r#KeInitResult {
              fn default() -> Self {
                Self {
                  r#sessionInitiationInfo: Default::default(),
                  r#sessionInfo: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#KeInitResult {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#sessionInitiationInfo)?;
                  subparcel.write(&self.r#sessionInfo)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#sessionInitiationInfo = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#sessionInfo = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#KeInitResult);
            binder::impl_deserialize_for_parcelable!(r#KeInitResult);
            impl binder::binder_impl::ParcelableMetadata for r#KeInitResult {
              fn get_descriptor() -> &'static str { "android.hardware.security.authgraph.KeInitResult" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#KeInitResult as _7_android_8_hardware_8_security_9_authgraph_12_KeInitResult;
            }
          }
          pub mod Key {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 1 --hash 3758824e7b75acdb1ca66620fb8a8aec0ec6dfcc --stability vintf --min_sdk_version current --ninja -d out/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph-V1-rust-source/gen/android/hardware/security/authgraph/Key.rs.d -o out/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph-V1-rust-source/gen -Nhardware/interfaces/security/authgraph/aidl/aidl_api/android.hardware.security.authgraph/1 hardware/interfaces/security/authgraph/aidl/aidl_api/android.hardware.security.authgraph/1/android/hardware/security/authgraph/Key.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug, Clone, Eq, PartialEq)]
            pub struct r#Key {
              pub r#pubKey: Option<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_6_PubKey>,
              pub r#arcFromPBK: Option<crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc>,
            }
            impl Default for r#Key {
              fn default() -> Self {
                Self {
                  r#pubKey: Default::default(),
                  r#arcFromPBK: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#Key {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#pubKey)?;
                  subparcel.write(&self.r#arcFromPBK)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#pubKey = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#arcFromPBK = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#Key);
            binder::impl_deserialize_for_parcelable!(r#Key);
            impl binder::binder_impl::ParcelableMetadata for r#Key {
              fn get_descriptor() -> &'static str { "android.hardware.security.authgraph.Key" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#Key as _7_android_8_hardware_8_security_9_authgraph_3_Key;
            }
          }
          pub mod PlainPubKey {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 1 --hash 3758824e7b75acdb1ca66620fb8a8aec0ec6dfcc --stability vintf --min_sdk_version current --ninja -d out/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph-V1-rust-source/gen/android/hardware/security/authgraph/PlainPubKey.rs.d -o out/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph-V1-rust-source/gen -Nhardware/interfaces/security/authgraph/aidl/aidl_api/android.hardware.security.authgraph/1 hardware/interfaces/security/authgraph/aidl/aidl_api/android.hardware.security.authgraph/1/android/hardware/security/authgraph/PlainPubKey.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug, Clone, Eq, PartialEq)]
            pub struct r#PlainPubKey {
              pub r#plainPubKey: Vec<u8>,
            }
            impl Default for r#PlainPubKey {
              fn default() -> Self {
                Self {
                  r#plainPubKey: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#PlainPubKey {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#plainPubKey)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#plainPubKey = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#PlainPubKey);
            binder::impl_deserialize_for_parcelable!(r#PlainPubKey);
            impl binder::binder_impl::ParcelableMetadata for r#PlainPubKey {
              fn get_descriptor() -> &'static str { "android.hardware.security.authgraph.PlainPubKey" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#PlainPubKey as _7_android_8_hardware_8_security_9_authgraph_11_PlainPubKey;
            }
          }
          pub mod PubKey {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 1 --hash 3758824e7b75acdb1ca66620fb8a8aec0ec6dfcc --stability vintf --min_sdk_version current --ninja -d out/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph-V1-rust-source/gen/android/hardware/security/authgraph/PubKey.rs.d -o out/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph-V1-rust-source/gen -Nhardware/interfaces/security/authgraph/aidl/aidl_api/android.hardware.security.authgraph/1 hardware/interfaces/security/authgraph/aidl/aidl_api/android.hardware.security.authgraph/1/android/hardware/security/authgraph/PubKey.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug, Clone, Eq, PartialEq)]
            pub enum r#PubKey {
              PlainKey(crate::mangled::_7_android_8_hardware_8_security_9_authgraph_11_PlainPubKey),
              SignedKey(crate::mangled::_7_android_8_hardware_8_security_9_authgraph_12_SignedPubKey),
            }
            impl Default for r#PubKey {
              fn default() -> Self {
                Self::PlainKey(Default::default())
              }
            }
            impl binder::Parcelable for r#PubKey {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                match self {
                  Self::PlainKey(v) => {
                    parcel.write(&0i32)?;
                    parcel.write(v)
                  }
                  Self::SignedKey(v) => {
                    parcel.write(&1i32)?;
                    parcel.write(v)
                  }
                }
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                let tag: i32 = parcel.read()?;
                match tag {
                  0 => {
                    let value: crate::mangled::_7_android_8_hardware_8_security_9_authgraph_11_PlainPubKey = parcel.read()?;
                    *self = Self::PlainKey(value);
                    Ok(())
                  }
                  1 => {
                    let value: crate::mangled::_7_android_8_hardware_8_security_9_authgraph_12_SignedPubKey = parcel.read()?;
                    *self = Self::SignedKey(value);
                    Ok(())
                  }
                  _ => {
                    Err(binder::StatusCode::BAD_VALUE)
                  }
                }
              }
            }
            binder::impl_serialize_for_parcelable!(r#PubKey);
            binder::impl_deserialize_for_parcelable!(r#PubKey);
            impl binder::binder_impl::ParcelableMetadata for r#PubKey {
              fn get_descriptor() -> &'static str { "android.hardware.security.authgraph.PubKey" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub mod r#Tag {
              #![allow(non_upper_case_globals)]
              use binder::declare_binder_enum;
              declare_binder_enum! {
                #[repr(C, align(4))]
                r#Tag : [i32; 2] {
                  r#plainKey = 0,
                  r#signedKey = 1,
                }
              }
            }
            pub(crate) mod mangled {
             pub use super::r#PubKey as _7_android_8_hardware_8_security_9_authgraph_6_PubKey;
             pub use super::r#Tag::r#Tag as _7_android_8_hardware_8_security_9_authgraph_6_PubKey_3_Tag;
            }
          }
          pub mod SessionIdSignature {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 1 --hash 3758824e7b75acdb1ca66620fb8a8aec0ec6dfcc --stability vintf --min_sdk_version current --ninja -d out/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph-V1-rust-source/gen/android/hardware/security/authgraph/SessionIdSignature.rs.d -o out/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph-V1-rust-source/gen -Nhardware/interfaces/security/authgraph/aidl/aidl_api/android.hardware.security.authgraph/1 hardware/interfaces/security/authgraph/aidl/aidl_api/android.hardware.security.authgraph/1/android/hardware/security/authgraph/SessionIdSignature.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug, Clone, Eq, PartialEq)]
            pub struct r#SessionIdSignature {
              pub r#signature: Vec<u8>,
            }
            impl Default for r#SessionIdSignature {
              fn default() -> Self {
                Self {
                  r#signature: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#SessionIdSignature {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#signature)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#signature = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#SessionIdSignature);
            binder::impl_deserialize_for_parcelable!(r#SessionIdSignature);
            impl binder::binder_impl::ParcelableMetadata for r#SessionIdSignature {
              fn get_descriptor() -> &'static str { "android.hardware.security.authgraph.SessionIdSignature" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#SessionIdSignature as _7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature;
            }
          }
          pub mod SessionInfo {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 1 --hash 3758824e7b75acdb1ca66620fb8a8aec0ec6dfcc --stability vintf --min_sdk_version current --ninja -d out/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph-V1-rust-source/gen/android/hardware/security/authgraph/SessionInfo.rs.d -o out/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph-V1-rust-source/gen -Nhardware/interfaces/security/authgraph/aidl/aidl_api/android.hardware.security.authgraph/1 hardware/interfaces/security/authgraph/aidl/aidl_api/android.hardware.security.authgraph/1/android/hardware/security/authgraph/SessionInfo.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug, Clone, Eq, PartialEq)]
            pub struct r#SessionInfo {
              pub r#sharedKeys: [crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Arc; 2],
              pub r#sessionId: Vec<u8>,
              pub r#signature: crate::mangled::_7_android_8_hardware_8_security_9_authgraph_18_SessionIdSignature,
            }
            impl Default for r#SessionInfo {
              fn default() -> Self {
                Self {
                  r#sharedKeys: [Default::default(), Default::default()],
                  r#sessionId: Default::default(),
                  r#signature: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#SessionInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#sharedKeys)?;
                  subparcel.write(&self.r#sessionId)?;
                  subparcel.write(&self.r#signature)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#sharedKeys = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#sessionId = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#signature = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#SessionInfo);
            binder::impl_deserialize_for_parcelable!(r#SessionInfo);
            impl binder::binder_impl::ParcelableMetadata for r#SessionInfo {
              fn get_descriptor() -> &'static str { "android.hardware.security.authgraph.SessionInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#SessionInfo as _7_android_8_hardware_8_security_9_authgraph_11_SessionInfo;
            }
          }
          pub mod SessionInitiationInfo {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 1 --hash 3758824e7b75acdb1ca66620fb8a8aec0ec6dfcc --stability vintf --min_sdk_version current --ninja -d out/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph-V1-rust-source/gen/android/hardware/security/authgraph/SessionInitiationInfo.rs.d -o out/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph-V1-rust-source/gen -Nhardware/interfaces/security/authgraph/aidl/aidl_api/android.hardware.security.authgraph/1 hardware/interfaces/security/authgraph/aidl/aidl_api/android.hardware.security.authgraph/1/android/hardware/security/authgraph/SessionInitiationInfo.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug, Clone, Eq, PartialEq)]
            pub struct r#SessionInitiationInfo {
              pub r#key: crate::mangled::_7_android_8_hardware_8_security_9_authgraph_3_Key,
              pub r#identity: crate::mangled::_7_android_8_hardware_8_security_9_authgraph_8_Identity,
              pub r#nonce: Vec<u8>,
              pub r#version: i32,
            }
            impl Default for r#SessionInitiationInfo {
              fn default() -> Self {
                Self {
                  r#key: Default::default(),
                  r#identity: Default::default(),
                  r#nonce: Default::default(),
                  r#version: 0,
                }
              }
            }
            impl binder::Parcelable for r#SessionInitiationInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#key)?;
                  subparcel.write(&self.r#identity)?;
                  subparcel.write(&self.r#nonce)?;
                  subparcel.write(&self.r#version)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#key = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#identity = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#nonce = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#version = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#SessionInitiationInfo);
            binder::impl_deserialize_for_parcelable!(r#SessionInitiationInfo);
            impl binder::binder_impl::ParcelableMetadata for r#SessionInitiationInfo {
              fn get_descriptor() -> &'static str { "android.hardware.security.authgraph.SessionInitiationInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#SessionInitiationInfo as _7_android_8_hardware_8_security_9_authgraph_21_SessionInitiationInfo;
            }
          }
          pub mod SignedPubKey {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 1 --hash 3758824e7b75acdb1ca66620fb8a8aec0ec6dfcc --stability vintf --min_sdk_version current --ninja -d out/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph-V1-rust-source/gen/android/hardware/security/authgraph/SignedPubKey.rs.d -o out/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph-V1-rust-source/gen -Nhardware/interfaces/security/authgraph/aidl/aidl_api/android.hardware.security.authgraph/1 hardware/interfaces/security/authgraph/aidl/aidl_api/android.hardware.security.authgraph/1/android/hardware/security/authgraph/SignedPubKey.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug, Clone, Eq, PartialEq)]
            pub struct r#SignedPubKey {
              pub r#signedPubKey: Vec<u8>,
            }
            impl Default for r#SignedPubKey {
              fn default() -> Self {
                Self {
                  r#signedPubKey: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#SignedPubKey {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#signedPubKey)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#signedPubKey = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#SignedPubKey);
            binder::impl_deserialize_for_parcelable!(r#SignedPubKey);
            impl binder::binder_impl::ParcelableMetadata for r#SignedPubKey {
              fn get_descriptor() -> &'static str { "android.hardware.security.authgraph.SignedPubKey" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#SignedPubKey as _7_android_8_hardware_8_security_9_authgraph_12_SignedPubKey;
            }
          }
        }
      }
    }
  }
}
pub mod mangled {
  pub use super::aidl::android::hardware::security::authgraph::Arc::mangled::*;
  pub use super::aidl::android::hardware::security::authgraph::Error::mangled::*;
  pub use super::aidl::android::hardware::security::authgraph::IAuthGraphKeyExchange::mangled::*;
  pub use super::aidl::android::hardware::security::authgraph::Identity::mangled::*;
  pub use super::aidl::android::hardware::security::authgraph::KeInitResult::mangled::*;
  pub use super::aidl::android::hardware::security::authgraph::Key::mangled::*;
  pub use super::aidl::android::hardware::security::authgraph::PlainPubKey::mangled::*;
  pub use super::aidl::android::hardware::security::authgraph::PubKey::mangled::*;
  pub use super::aidl::android::hardware::security::authgraph::SessionIdSignature::mangled::*;
  pub use super::aidl::android::hardware::security::authgraph::SessionInfo::mangled::*;
  pub use super::aidl::android::hardware::security::authgraph::SessionInitiationInfo::mangled::*;
  pub use super::aidl::android::hardware::security::authgraph::SignedPubKey::mangled::*;
}

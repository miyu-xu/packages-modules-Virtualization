#![allow(non_snake_case)]
#![allow(missing_docs)]
#[deprecated(note = "Please access via libbinder_rs binder::")]
pub use binder;
pub mod aidl {
  pub mod android {
    pub mod hardware {
      pub mod security {
        pub mod secretkeeper {
          pub mod ISecretkeeper {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 1 --hash 347439bd6088bd24a72e789a616a1586863e43b8 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/secretkeeper/aidl/android.hardware.security.secretkeeper-V1-rust-source/gen/android/hardware/security/secretkeeper/ISecretkeeper.rs.d -o out/soong/.intermediates/hardware/interfaces/security/secretkeeper/aidl/android.hardware.security.secretkeeper-V1-rust-source/gen -Nhardware/interfaces/security/secretkeeper/aidl/aidl_api/android.hardware.security.secretkeeper/1 hardware/interfaces/security/secretkeeper/aidl/aidl_api/android.hardware.security.secretkeeper/1/android/hardware/security/secretkeeper/ISecretkeeper.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              ISecretkeeper["android.hardware.security.secretkeeper.ISecretkeeper"] {
                native: BnSecretkeeper(on_transact),
                proxy: BpSecretkeeper {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: ISecretkeeperAsync(try_into_local_async),
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait ISecretkeeper: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.security.secretkeeper.ISecretkeeper" }
              fn r#getAuthGraphKe(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_security_9_authgraph_21_IAuthGraphKeyExchange>>;
              fn r#processSecretManagementRequest(&self, _arg_request: &[u8]) -> binder::Result<Vec<u8>>;
              fn r#deleteIds(&self, _arg_ids: &[crate::mangled::_7_android_8_hardware_8_security_12_secretkeeper_8_SecretId]) -> binder::Result<()>;
              fn r#deleteAll(&self) -> binder::Result<()>;
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash(&self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> ISecretkeeperDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: ISecretkeeperDefaultRef) -> ISecretkeeperDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
              fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn ISecretkeeperAsyncServer + Send + Sync)> {
                None
              }
            }
            pub trait ISecretkeeperAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.security.secretkeeper.ISecretkeeper" }
              fn r#getAuthGraphKe<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_security_9_authgraph_21_IAuthGraphKeyExchange>>>;
              fn r#processSecretManagementRequest<'a>(&'a self, _arg_request: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<Vec<u8>>>;
              fn r#deleteIds<'a>(&'a self, _arg_ids: &'a [crate::mangled::_7_android_8_hardware_8_security_12_secretkeeper_8_SecretId]) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#deleteAll<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#getInterfaceVersion<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait ISecretkeeperAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.security.secretkeeper.ISecretkeeper" }
              async fn r#getAuthGraphKe(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_security_9_authgraph_21_IAuthGraphKeyExchange>>;
              async fn r#processSecretManagementRequest(&self, _arg_request: &[u8]) -> binder::Result<Vec<u8>>;
              async fn r#deleteIds(&self, _arg_ids: &[crate::mangled::_7_android_8_hardware_8_security_12_secretkeeper_8_SecretId]) -> binder::Result<()>;
              async fn r#deleteAll(&self) -> binder::Result<()>;
            }
            impl BnSecretkeeper {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn ISecretkeeper>
              where
                T: ISecretkeeperAsyncServer + binder::Interface + Send + Sync + 'static,
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
                impl<T, R> ISecretkeeper for Wrapper<T, R>
                where
                  T: ISecretkeeperAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#getAuthGraphKe(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_security_9_authgraph_21_IAuthGraphKeyExchange>> {
                    self._rt.block_on(self._inner.r#getAuthGraphKe())
                  }
                  fn r#processSecretManagementRequest(&self, _arg_request: &[u8]) -> binder::Result<Vec<u8>> {
                    self._rt.block_on(self._inner.r#processSecretManagementRequest(_arg_request))
                  }
                  fn r#deleteIds(&self, _arg_ids: &[crate::mangled::_7_android_8_hardware_8_security_12_secretkeeper_8_SecretId]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#deleteIds(_arg_ids))
                  }
                  fn r#deleteAll(&self) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#deleteAll())
                  }
                  fn try_as_async_server(&self) -> Option<&(dyn ISecretkeeperAsyncServer + Send + Sync)> {
                    Some(&self._inner)
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
              pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn ISecretkeeperAsync<P>>> {
                struct Wrapper {
                  _native: binder::binder_impl::Binder<BnSecretkeeper>
                }
                impl binder::Interface for Wrapper {}
                impl<P: binder::BinderAsyncPool> ISecretkeeperAsync<P> for Wrapper {
                  fn r#getAuthGraphKe<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_security_9_authgraph_21_IAuthGraphKeyExchange>>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getAuthGraphKe())
                  }
                  fn r#processSecretManagementRequest<'a>(&'a self, _arg_request: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<Vec<u8>>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#processSecretManagementRequest(_arg_request))
                  }
                  fn r#deleteIds<'a>(&'a self, _arg_ids: &'a [crate::mangled::_7_android_8_hardware_8_security_12_secretkeeper_8_SecretId]) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#deleteIds(_arg_ids))
                  }
                  fn r#deleteAll<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#deleteAll())
                  }
                }
                if _native.try_as_async_server().is_some() {
                  Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn ISecretkeeperAsync<P>>))
                } else {
                  None
                }
              }
            }
            pub trait ISecretkeeperDefault: Send + Sync {
              fn r#getAuthGraphKe(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_security_9_authgraph_21_IAuthGraphKeyExchange>> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#processSecretManagementRequest(&self, _arg_request: &[u8]) -> binder::Result<Vec<u8>> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#deleteIds(&self, _arg_ids: &[crate::mangled::_7_android_8_hardware_8_security_12_secretkeeper_8_SecretId]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#deleteAll(&self) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#getAuthGraphKe: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#processSecretManagementRequest: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#deleteIds: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#deleteAll: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type ISecretkeeperDefaultRef = Option<std::sync::Arc<dyn ISecretkeeperDefault>>;
            static DEFAULT_IMPL: std::sync::Mutex<ISecretkeeperDefaultRef> = std::sync::Mutex::new(None);
            pub const r#ERROR_UNKNOWN_KEY_ID: i32 = 1;
            pub const r#ERROR_INTERNAL_ERROR: i32 = 2;
            pub const r#ERROR_REQUEST_MALFORMED: i32 = 3;
            pub const VERSION: i32 = 1;
            pub const HASH: &str = "347439bd6088bd24a72e789a616a1586863e43b8";
            impl BpSecretkeeper {
              fn build_parcel_getAuthGraphKe(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_getAuthGraphKe(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_security_9_authgraph_21_IAuthGraphKeyExchange>> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as ISecretkeeper>::getDefaultImpl() {
                    return _aidl_default_impl.r#getAuthGraphKe();
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_security_9_authgraph_21_IAuthGraphKeyExchange> = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_processSecretManagementRequest(&self, _arg_request: &[u8]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_request)?;
                Ok(aidl_data)
              }
              fn read_response_processSecretManagementRequest(&self, _arg_request: &[u8], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<u8>> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as ISecretkeeper>::getDefaultImpl() {
                    return _aidl_default_impl.r#processSecretManagementRequest(_arg_request);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: Vec<u8> = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_deleteIds(&self, _arg_ids: &[crate::mangled::_7_android_8_hardware_8_security_12_secretkeeper_8_SecretId]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(_arg_ids)?;
                Ok(aidl_data)
              }
              fn read_response_deleteIds(&self, _arg_ids: &[crate::mangled::_7_android_8_hardware_8_security_12_secretkeeper_8_SecretId], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as ISecretkeeper>::getDefaultImpl() {
                    return _aidl_default_impl.r#deleteIds(_arg_ids);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_deleteAll(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_deleteAll(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as ISecretkeeper>::getDefaultImpl() {
                    return _aidl_default_impl.r#deleteAll();
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
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
            impl ISecretkeeper for BpSecretkeeper {
              fn r#getAuthGraphKe(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_security_9_authgraph_21_IAuthGraphKeyExchange>> {
                let _aidl_data = self.build_parcel_getAuthGraphKe()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getAuthGraphKe, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_getAuthGraphKe(_aidl_reply)
              }
              fn r#processSecretManagementRequest(&self, _arg_request: &[u8]) -> binder::Result<Vec<u8>> {
                let _aidl_data = self.build_parcel_processSecretManagementRequest(_arg_request)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#processSecretManagementRequest, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_processSecretManagementRequest(_arg_request, _aidl_reply)
              }
              fn r#deleteIds(&self, _arg_ids: &[crate::mangled::_7_android_8_hardware_8_security_12_secretkeeper_8_SecretId]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_deleteIds(_arg_ids)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#deleteIds, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_deleteIds(_arg_ids, _aidl_reply)
              }
              fn r#deleteAll(&self) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_deleteAll()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#deleteAll, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
                self.read_response_deleteAll(_aidl_reply)
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
            impl<P: binder::BinderAsyncPool> ISecretkeeperAsync<P> for BpSecretkeeper {
              fn r#getAuthGraphKe<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_security_9_authgraph_21_IAuthGraphKeyExchange>>> {
                let _aidl_data = match self.build_parcel_getAuthGraphKe() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getAuthGraphKe, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getAuthGraphKe(_aidl_reply)
                  }
                )
              }
              fn r#processSecretManagementRequest<'a>(&'a self, _arg_request: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<Vec<u8>>> {
                let _aidl_data = match self.build_parcel_processSecretManagementRequest(_arg_request) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#processSecretManagementRequest, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_processSecretManagementRequest(_arg_request, _aidl_reply)
                  }
                )
              }
              fn r#deleteIds<'a>(&'a self, _arg_ids: &'a [crate::mangled::_7_android_8_hardware_8_security_12_secretkeeper_8_SecretId]) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_deleteIds(_arg_ids) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#deleteIds, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_deleteIds(_arg_ids, _aidl_reply)
                  }
                )
              }
              fn r#deleteAll<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_deleteAll() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#deleteAll, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_deleteAll(_aidl_reply)
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
            impl ISecretkeeper for binder::binder_impl::Binder<BnSecretkeeper> {
              fn r#getAuthGraphKe(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_security_9_authgraph_21_IAuthGraphKeyExchange>> { self.0.r#getAuthGraphKe() }
              fn r#processSecretManagementRequest(&self, _arg_request: &[u8]) -> binder::Result<Vec<u8>> { self.0.r#processSecretManagementRequest(_arg_request) }
              fn r#deleteIds(&self, _arg_ids: &[crate::mangled::_7_android_8_hardware_8_security_12_secretkeeper_8_SecretId]) -> binder::Result<()> { self.0.r#deleteIds(_arg_ids) }
              fn r#deleteAll(&self) -> binder::Result<()> { self.0.r#deleteAll() }
              fn r#getInterfaceVersion(&self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash(&self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn ISecretkeeper, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#getAuthGraphKe => {
                  let _aidl_return = _aidl_service.r#getAuthGraphKe();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#processSecretManagementRequest => {
                  let _arg_request: Vec<u8> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#processSecretManagementRequest(&_arg_request);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#deleteIds => {
                  let _arg_ids: Vec<crate::mangled::_7_android_8_hardware_8_security_12_secretkeeper_8_SecretId> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#deleteIds(&_arg_ids);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#deleteAll => {
                  let _aidl_return = _aidl_service.r#deleteAll();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
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
             pub use super::r#ISecretkeeper as _7_android_8_hardware_8_security_12_secretkeeper_13_ISecretkeeper;
            }
          }
          pub mod SecretId {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 1 --hash 347439bd6088bd24a72e789a616a1586863e43b8 --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/security/authgraph/aidl/android.hardware.security.authgraph_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/secretkeeper/aidl/android.hardware.security.secretkeeper-V1-rust-source/gen/android/hardware/security/secretkeeper/SecretId.rs.d -o out/soong/.intermediates/hardware/interfaces/security/secretkeeper/aidl/android.hardware.security.secretkeeper-V1-rust-source/gen -Nhardware/interfaces/security/secretkeeper/aidl/aidl_api/android.hardware.security.secretkeeper/1 hardware/interfaces/security/secretkeeper/aidl/aidl_api/android.hardware.security.secretkeeper/1/android/hardware/security/secretkeeper/SecretId.aidl
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#SecretId {
              pub r#id: [u8; 64],
            }
            impl Default for r#SecretId {
              fn default() -> Self {
                Self {
                  r#id: [Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default()],
                }
              }
            }
            impl binder::Parcelable for r#SecretId {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#id)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#id = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#SecretId);
            binder::impl_deserialize_for_parcelable!(r#SecretId);
            impl binder::binder_impl::ParcelableMetadata for r#SecretId {
              fn get_descriptor() -> &'static str { "android.hardware.security.secretkeeper.SecretId" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#SecretId as _7_android_8_hardware_8_security_12_secretkeeper_8_SecretId;
            }
          }
        }
      }
    }
  }
}
pub mod mangled {
  pub use super::aidl::android::hardware::security::secretkeeper::ISecretkeeper::mangled::*;
  pub use super::aidl::android::hardware::security::secretkeeper::SecretId::mangled::*;
  pub(crate) use android_hardware_security_authgraph::mangled::*;
}

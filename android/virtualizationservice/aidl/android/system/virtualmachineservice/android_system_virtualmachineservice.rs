#![allow(non_snake_case)]
#![allow(missing_docs)]
#[deprecated(note = "Please access via libbinder_rs binder::")]
pub use binder;
pub mod aidl {
  pub mod android {
    pub mod system {
      pub mod virtualmachineservice {
        pub mod IVirtualMachineService {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/security/secretkeeper/aidl/android.hardware.security.secretkeeper_interface/1/preprocessed.aidl -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualmachineservice-rust-source/gen/android/system/virtualmachineservice/IVirtualMachineService.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualmachineservice-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualmachineservice/IVirtualMachineService.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #![allow(non_upper_case_globals)]
          #![allow(non_snake_case)]
          #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
          use binder::declare_binder_interface;
          declare_binder_interface! {
            IVirtualMachineService["android.system.virtualmachineservice.IVirtualMachineService"] {
              native: BnVirtualMachineService(on_transact),
              proxy: BpVirtualMachineService {
              },
              async: IVirtualMachineServiceAsync(try_into_local_async),
            }
          }
          pub trait IVirtualMachineService: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualmachineservice.IVirtualMachineService" }
            fn r#notifyPayloadStarted(&self) -> binder::Result<()>;
            fn r#notifyPayloadReady(&self) -> binder::Result<()>;
            fn r#notifyPayloadFinished(&self, _arg_exitCode: i32) -> binder::Result<()>;
            fn r#notifyError(&self, _arg_errorCode: crate::mangled::_7_android_6_system_20_virtualizationcommon_9_ErrorCode, _arg_message: &str) -> binder::Result<()>;
            fn r#requestAttestation(&self, _arg_csr: &[u8], _arg_testMode: bool) -> binder::Result<Vec<crate::mangled::_7_android_6_system_20_virtualizationcommon_11_Certificate>>;
            fn r#getSecretkeeper(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_security_12_secretkeeper_13_ISecretkeeper>>;
            fn getDefaultImpl() -> IVirtualMachineServiceDefaultRef where Self: Sized {
              DEFAULT_IMPL.lock().unwrap().clone()
            }
            fn setDefaultImpl(d: IVirtualMachineServiceDefaultRef) -> IVirtualMachineServiceDefaultRef where Self: Sized {
              std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
            }
            fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn IVirtualMachineServiceAsyncServer + Send + Sync)> {
              None
            }
          }
          pub trait IVirtualMachineServiceAsync<P>: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualmachineservice.IVirtualMachineService" }
            fn r#notifyPayloadStarted<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#notifyPayloadReady<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#notifyPayloadFinished<'a>(&'a self, _arg_exitCode: i32) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#notifyError<'a>(&'a self, _arg_errorCode: crate::mangled::_7_android_6_system_20_virtualizationcommon_9_ErrorCode, _arg_message: &'a str) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#requestAttestation<'a>(&'a self, _arg_csr: &'a [u8], _arg_testMode: bool) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_6_system_20_virtualizationcommon_11_Certificate>>>;
            fn r#getSecretkeeper<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_security_12_secretkeeper_13_ISecretkeeper>>>;
          }
          #[::async_trait::async_trait]
          pub trait IVirtualMachineServiceAsyncServer: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualmachineservice.IVirtualMachineService" }
            async fn r#notifyPayloadStarted(&self) -> binder::Result<()>;
            async fn r#notifyPayloadReady(&self) -> binder::Result<()>;
            async fn r#notifyPayloadFinished(&self, _arg_exitCode: i32) -> binder::Result<()>;
            async fn r#notifyError(&self, _arg_errorCode: crate::mangled::_7_android_6_system_20_virtualizationcommon_9_ErrorCode, _arg_message: &str) -> binder::Result<()>;
            async fn r#requestAttestation(&self, _arg_csr: &[u8], _arg_testMode: bool) -> binder::Result<Vec<crate::mangled::_7_android_6_system_20_virtualizationcommon_11_Certificate>>;
            async fn r#getSecretkeeper(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_security_12_secretkeeper_13_ISecretkeeper>>;
          }
          impl BnVirtualMachineService {
            /// Create a new async binder service.
            pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IVirtualMachineService>
            where
              T: IVirtualMachineServiceAsyncServer + binder::Interface + Send + Sync + 'static,
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
              impl<T, R> IVirtualMachineService for Wrapper<T, R>
              where
                T: IVirtualMachineServiceAsyncServer + Send + Sync + 'static,
                R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
              {
                fn r#notifyPayloadStarted(&self) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#notifyPayloadStarted())
                }
                fn r#notifyPayloadReady(&self) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#notifyPayloadReady())
                }
                fn r#notifyPayloadFinished(&self, _arg_exitCode: i32) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#notifyPayloadFinished(_arg_exitCode))
                }
                fn r#notifyError(&self, _arg_errorCode: crate::mangled::_7_android_6_system_20_virtualizationcommon_9_ErrorCode, _arg_message: &str) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#notifyError(_arg_errorCode, _arg_message))
                }
                fn r#requestAttestation(&self, _arg_csr: &[u8], _arg_testMode: bool) -> binder::Result<Vec<crate::mangled::_7_android_6_system_20_virtualizationcommon_11_Certificate>> {
                  self._rt.block_on(self._inner.r#requestAttestation(_arg_csr, _arg_testMode))
                }
                fn r#getSecretkeeper(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_security_12_secretkeeper_13_ISecretkeeper>> {
                  self._rt.block_on(self._inner.r#getSecretkeeper())
                }
                fn try_as_async_server(&self) -> Option<&(dyn IVirtualMachineServiceAsyncServer + Send + Sync)> {
                  Some(&self._inner)
                }
              }
              let wrapped = Wrapper { _inner: inner, _rt: rt };
              Self::new_binder(wrapped, features)
            }
            pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn IVirtualMachineServiceAsync<P>>> {
              struct Wrapper {
                _native: binder::binder_impl::Binder<BnVirtualMachineService>
              }
              impl binder::Interface for Wrapper {}
              impl<P: binder::BinderAsyncPool> IVirtualMachineServiceAsync<P> for Wrapper {
                fn r#notifyPayloadStarted<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#notifyPayloadStarted())
                }
                fn r#notifyPayloadReady<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#notifyPayloadReady())
                }
                fn r#notifyPayloadFinished<'a>(&'a self, _arg_exitCode: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#notifyPayloadFinished(_arg_exitCode))
                }
                fn r#notifyError<'a>(&'a self, _arg_errorCode: crate::mangled::_7_android_6_system_20_virtualizationcommon_9_ErrorCode, _arg_message: &'a str) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#notifyError(_arg_errorCode, _arg_message))
                }
                fn r#requestAttestation<'a>(&'a self, _arg_csr: &'a [u8], _arg_testMode: bool) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_6_system_20_virtualizationcommon_11_Certificate>>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#requestAttestation(_arg_csr, _arg_testMode))
                }
                fn r#getSecretkeeper<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_security_12_secretkeeper_13_ISecretkeeper>>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#getSecretkeeper())
                }
              }
              if _native.try_as_async_server().is_some() {
                Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn IVirtualMachineServiceAsync<P>>))
              } else {
                None
              }
            }
          }
          pub trait IVirtualMachineServiceDefault: Send + Sync {
            fn r#notifyPayloadStarted(&self) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#notifyPayloadReady(&self) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#notifyPayloadFinished(&self, _arg_exitCode: i32) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#notifyError(&self, _arg_errorCode: crate::mangled::_7_android_6_system_20_virtualizationcommon_9_ErrorCode, _arg_message: &str) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#requestAttestation(&self, _arg_csr: &[u8], _arg_testMode: bool) -> binder::Result<Vec<crate::mangled::_7_android_6_system_20_virtualizationcommon_11_Certificate>> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#getSecretkeeper(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_security_12_secretkeeper_13_ISecretkeeper>> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
          }
          pub mod transactions {
            pub const r#notifyPayloadStarted: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
            pub const r#notifyPayloadReady: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
            pub const r#notifyPayloadFinished: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
            pub const r#notifyError: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
            pub const r#requestAttestation: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
            pub const r#getSecretkeeper: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
          }
          pub type IVirtualMachineServiceDefaultRef = Option<std::sync::Arc<dyn IVirtualMachineServiceDefault>>;
          static DEFAULT_IMPL: std::sync::Mutex<IVirtualMachineServiceDefaultRef> = std::sync::Mutex::new(None);
          pub const r#VM_TOMBSTONES_SERVICE_PORT: i32 = 2000;
          impl BpVirtualMachineService {
            fn build_parcel_notifyPayloadStarted(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_notifyPayloadStarted(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualMachineService>::getDefaultImpl() {
                  return _aidl_default_impl.r#notifyPayloadStarted();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_notifyPayloadReady(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_notifyPayloadReady(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualMachineService>::getDefaultImpl() {
                  return _aidl_default_impl.r#notifyPayloadReady();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_notifyPayloadFinished(&self, _arg_exitCode: i32) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(&_arg_exitCode)?;
              Ok(aidl_data)
            }
            fn read_response_notifyPayloadFinished(&self, _arg_exitCode: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualMachineService>::getDefaultImpl() {
                  return _aidl_default_impl.r#notifyPayloadFinished(_arg_exitCode);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_notifyError(&self, _arg_errorCode: crate::mangled::_7_android_6_system_20_virtualizationcommon_9_ErrorCode, _arg_message: &str) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(&_arg_errorCode)?;
              aidl_data.write(_arg_message)?;
              Ok(aidl_data)
            }
            fn read_response_notifyError(&self, _arg_errorCode: crate::mangled::_7_android_6_system_20_virtualizationcommon_9_ErrorCode, _arg_message: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualMachineService>::getDefaultImpl() {
                  return _aidl_default_impl.r#notifyError(_arg_errorCode, _arg_message);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_requestAttestation(&self, _arg_csr: &[u8], _arg_testMode: bool) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_csr)?;
              aidl_data.write(&_arg_testMode)?;
              Ok(aidl_data)
            }
            fn read_response_requestAttestation(&self, _arg_csr: &[u8], _arg_testMode: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_6_system_20_virtualizationcommon_11_Certificate>> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualMachineService>::getDefaultImpl() {
                  return _aidl_default_impl.r#requestAttestation(_arg_csr, _arg_testMode);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: Vec<crate::mangled::_7_android_6_system_20_virtualizationcommon_11_Certificate> = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_getSecretkeeper(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_getSecretkeeper(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_security_12_secretkeeper_13_ISecretkeeper>> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualMachineService>::getDefaultImpl() {
                  return _aidl_default_impl.r#getSecretkeeper();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_security_12_secretkeeper_13_ISecretkeeper> = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
          }
          impl IVirtualMachineService for BpVirtualMachineService {
            fn r#notifyPayloadStarted(&self) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_notifyPayloadStarted()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#notifyPayloadStarted, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_notifyPayloadStarted(_aidl_reply)
            }
            fn r#notifyPayloadReady(&self) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_notifyPayloadReady()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#notifyPayloadReady, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_notifyPayloadReady(_aidl_reply)
            }
            fn r#notifyPayloadFinished(&self, _arg_exitCode: i32) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_notifyPayloadFinished(_arg_exitCode)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#notifyPayloadFinished, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_notifyPayloadFinished(_arg_exitCode, _aidl_reply)
            }
            fn r#notifyError(&self, _arg_errorCode: crate::mangled::_7_android_6_system_20_virtualizationcommon_9_ErrorCode, _arg_message: &str) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_notifyError(_arg_errorCode, _arg_message)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#notifyError, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_notifyError(_arg_errorCode, _arg_message, _aidl_reply)
            }
            fn r#requestAttestation(&self, _arg_csr: &[u8], _arg_testMode: bool) -> binder::Result<Vec<crate::mangled::_7_android_6_system_20_virtualizationcommon_11_Certificate>> {
              let _aidl_data = self.build_parcel_requestAttestation(_arg_csr, _arg_testMode)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#requestAttestation, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_requestAttestation(_arg_csr, _arg_testMode, _aidl_reply)
            }
            fn r#getSecretkeeper(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_security_12_secretkeeper_13_ISecretkeeper>> {
              let _aidl_data = self.build_parcel_getSecretkeeper()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#getSecretkeeper, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_getSecretkeeper(_aidl_reply)
            }
          }
          impl<P: binder::BinderAsyncPool> IVirtualMachineServiceAsync<P> for BpVirtualMachineService {
            fn r#notifyPayloadStarted<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_notifyPayloadStarted() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#notifyPayloadStarted, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_notifyPayloadStarted(_aidl_reply)
                }
              )
            }
            fn r#notifyPayloadReady<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_notifyPayloadReady() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#notifyPayloadReady, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_notifyPayloadReady(_aidl_reply)
                }
              )
            }
            fn r#notifyPayloadFinished<'a>(&'a self, _arg_exitCode: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_notifyPayloadFinished(_arg_exitCode) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#notifyPayloadFinished, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_notifyPayloadFinished(_arg_exitCode, _aidl_reply)
                }
              )
            }
            fn r#notifyError<'a>(&'a self, _arg_errorCode: crate::mangled::_7_android_6_system_20_virtualizationcommon_9_ErrorCode, _arg_message: &'a str) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_notifyError(_arg_errorCode, _arg_message) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#notifyError, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_notifyError(_arg_errorCode, _arg_message, _aidl_reply)
                }
              )
            }
            fn r#requestAttestation<'a>(&'a self, _arg_csr: &'a [u8], _arg_testMode: bool) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_6_system_20_virtualizationcommon_11_Certificate>>> {
              let _aidl_data = match self.build_parcel_requestAttestation(_arg_csr, _arg_testMode) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#requestAttestation, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_requestAttestation(_arg_csr, _arg_testMode, _aidl_reply)
                }
              )
            }
            fn r#getSecretkeeper<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_security_12_secretkeeper_13_ISecretkeeper>>> {
              let _aidl_data = match self.build_parcel_getSecretkeeper() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#getSecretkeeper, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_getSecretkeeper(_aidl_reply)
                }
              )
            }
          }
          impl IVirtualMachineService for binder::binder_impl::Binder<BnVirtualMachineService> {
            fn r#notifyPayloadStarted(&self) -> binder::Result<()> { self.0.r#notifyPayloadStarted() }
            fn r#notifyPayloadReady(&self) -> binder::Result<()> { self.0.r#notifyPayloadReady() }
            fn r#notifyPayloadFinished(&self, _arg_exitCode: i32) -> binder::Result<()> { self.0.r#notifyPayloadFinished(_arg_exitCode) }
            fn r#notifyError(&self, _arg_errorCode: crate::mangled::_7_android_6_system_20_virtualizationcommon_9_ErrorCode, _arg_message: &str) -> binder::Result<()> { self.0.r#notifyError(_arg_errorCode, _arg_message) }
            fn r#requestAttestation(&self, _arg_csr: &[u8], _arg_testMode: bool) -> binder::Result<Vec<crate::mangled::_7_android_6_system_20_virtualizationcommon_11_Certificate>> { self.0.r#requestAttestation(_arg_csr, _arg_testMode) }
            fn r#getSecretkeeper(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_security_12_secretkeeper_13_ISecretkeeper>> { self.0.r#getSecretkeeper() }
          }
          fn on_transact(_aidl_service: &dyn IVirtualMachineService, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
            match _aidl_code {
              transactions::r#notifyPayloadStarted => {
                let _aidl_return = _aidl_service.r#notifyPayloadStarted();
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#notifyPayloadReady => {
                let _aidl_return = _aidl_service.r#notifyPayloadReady();
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#notifyPayloadFinished => {
                let _arg_exitCode: i32 = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#notifyPayloadFinished(_arg_exitCode);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#notifyError => {
                let _arg_errorCode: crate::mangled::_7_android_6_system_20_virtualizationcommon_9_ErrorCode = _aidl_data.read()?;
                let _arg_message: String = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#notifyError(_arg_errorCode, &_arg_message);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#requestAttestation => {
                let _arg_csr: Vec<u8> = _aidl_data.read()?;
                let _arg_testMode: bool = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#requestAttestation(&_arg_csr, _arg_testMode);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#getSecretkeeper => {
                let _aidl_return = _aidl_service.r#getSecretkeeper();
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              7 => {
                _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                Ok(())
              }
              8 => {
                _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                let _aidl_return: Option<binder::SpIBinder> = None;
                _aidl_reply.write(&_aidl_return)?;
                Ok(())
              }
              9 => {
                let _arg_guestAgent: Option<binder::SpIBinder> = _aidl_data.read()?;
                let _ = _arg_guestAgent;
                _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                Ok(())
              }
              10 => {
                _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                let _aidl_return: Option<binder::SpIBinder> = None;
                _aidl_reply.write(&_aidl_return)?;
                Ok(())
              }
              11 => {
                _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                Ok(())
              }
              _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
            }
          }
          pub(crate) mod mangled {
           pub use super::r#IVirtualMachineService as _7_android_6_system_21_virtualmachineservice_22_IVirtualMachineService;
          }
        }
      }
    }
  }
}
pub mod mangled {
  pub use super::aidl::android::system::virtualmachineservice::IVirtualMachineService::mangled::*;
  pub(crate) use android_hardware_security_secretkeeper::mangled::*;
  pub(crate) use android_system_virtualizationcommon::mangled::*;
}

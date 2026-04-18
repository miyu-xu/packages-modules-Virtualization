#![allow(non_snake_case)]
#![allow(missing_docs)]
#[deprecated(note = "Please access via libbinder_rs binder::")]
pub use binder;
pub mod aidl {
  pub mod android {
    pub mod system {
      pub mod virtualizationmaintenance {
        pub mod IVirtualizationMaintenance {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationmaintenance-rust-source/gen/android/system/virtualizationmaintenance/IVirtualizationMaintenance.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationmaintenance-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationmaintenance/IVirtualizationMaintenance.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #![allow(non_upper_case_globals)]
          #![allow(non_snake_case)]
          #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
          use binder::declare_binder_interface;
          declare_binder_interface! {
            IVirtualizationMaintenance["android.system.virtualizationmaintenance.IVirtualizationMaintenance"] {
              native: BnVirtualizationMaintenance(on_transact),
              proxy: BpVirtualizationMaintenance {
              },
              async: IVirtualizationMaintenanceAsync(try_into_local_async),
            }
          }
          pub trait IVirtualizationMaintenance: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationmaintenance.IVirtualizationMaintenance" }
            fn r#appRemoved(&self, _arg_userId: i32, _arg_appId: i32) -> binder::Result<()>;
            fn r#userRemoved(&self, _arg_userId: i32) -> binder::Result<()>;
            fn r#performReconciliation(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_6_system_25_virtualizationmaintenance_37_IVirtualizationReconciliationCallback>) -> binder::Result<()>;
            fn getDefaultImpl() -> IVirtualizationMaintenanceDefaultRef where Self: Sized {
              DEFAULT_IMPL.lock().unwrap().clone()
            }
            fn setDefaultImpl(d: IVirtualizationMaintenanceDefaultRef) -> IVirtualizationMaintenanceDefaultRef where Self: Sized {
              std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
            }
            fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn IVirtualizationMaintenanceAsyncServer + Send + Sync)> {
              None
            }
          }
          pub trait IVirtualizationMaintenanceAsync<P>: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationmaintenance.IVirtualizationMaintenance" }
            fn r#appRemoved<'a>(&'a self, _arg_userId: i32, _arg_appId: i32) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#userRemoved<'a>(&'a self, _arg_userId: i32) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#performReconciliation<'a>(&'a self, _arg_callback: &'a binder::Strong<dyn crate::mangled::_7_android_6_system_25_virtualizationmaintenance_37_IVirtualizationReconciliationCallback>) -> binder::BoxFuture<'a, binder::Result<()>>;
          }
          #[::async_trait::async_trait]
          pub trait IVirtualizationMaintenanceAsyncServer: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationmaintenance.IVirtualizationMaintenance" }
            async fn r#appRemoved(&self, _arg_userId: i32, _arg_appId: i32) -> binder::Result<()>;
            async fn r#userRemoved(&self, _arg_userId: i32) -> binder::Result<()>;
            async fn r#performReconciliation(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_6_system_25_virtualizationmaintenance_37_IVirtualizationReconciliationCallback>) -> binder::Result<()>;
          }
          impl BnVirtualizationMaintenance {
            /// Create a new async binder service.
            pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IVirtualizationMaintenance>
            where
              T: IVirtualizationMaintenanceAsyncServer + binder::Interface + Send + Sync + 'static,
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
              impl<T, R> IVirtualizationMaintenance for Wrapper<T, R>
              where
                T: IVirtualizationMaintenanceAsyncServer + Send + Sync + 'static,
                R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
              {
                fn r#appRemoved(&self, _arg_userId: i32, _arg_appId: i32) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#appRemoved(_arg_userId, _arg_appId))
                }
                fn r#userRemoved(&self, _arg_userId: i32) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#userRemoved(_arg_userId))
                }
                fn r#performReconciliation(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_6_system_25_virtualizationmaintenance_37_IVirtualizationReconciliationCallback>) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#performReconciliation(_arg_callback))
                }
                fn try_as_async_server(&self) -> Option<&(dyn IVirtualizationMaintenanceAsyncServer + Send + Sync)> {
                  Some(&self._inner)
                }
              }
              let wrapped = Wrapper { _inner: inner, _rt: rt };
              Self::new_binder(wrapped, features)
            }
            pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn IVirtualizationMaintenanceAsync<P>>> {
              struct Wrapper {
                _native: binder::binder_impl::Binder<BnVirtualizationMaintenance>
              }
              impl binder::Interface for Wrapper {}
              impl<P: binder::BinderAsyncPool> IVirtualizationMaintenanceAsync<P> for Wrapper {
                fn r#appRemoved<'a>(&'a self, _arg_userId: i32, _arg_appId: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#appRemoved(_arg_userId, _arg_appId))
                }
                fn r#userRemoved<'a>(&'a self, _arg_userId: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#userRemoved(_arg_userId))
                }
                fn r#performReconciliation<'a>(&'a self, _arg_callback: &'a binder::Strong<dyn crate::mangled::_7_android_6_system_25_virtualizationmaintenance_37_IVirtualizationReconciliationCallback>) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#performReconciliation(_arg_callback))
                }
              }
              if _native.try_as_async_server().is_some() {
                Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn IVirtualizationMaintenanceAsync<P>>))
              } else {
                None
              }
            }
          }
          pub trait IVirtualizationMaintenanceDefault: Send + Sync {
            fn r#appRemoved(&self, _arg_userId: i32, _arg_appId: i32) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#userRemoved(&self, _arg_userId: i32) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#performReconciliation(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_6_system_25_virtualizationmaintenance_37_IVirtualizationReconciliationCallback>) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
          }
          pub mod transactions {
            pub const r#appRemoved: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
            pub const r#userRemoved: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
            pub const r#performReconciliation: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
          }
          pub type IVirtualizationMaintenanceDefaultRef = Option<std::sync::Arc<dyn IVirtualizationMaintenanceDefault>>;
          static DEFAULT_IMPL: std::sync::Mutex<IVirtualizationMaintenanceDefaultRef> = std::sync::Mutex::new(None);
          impl BpVirtualizationMaintenance {
            fn build_parcel_appRemoved(&self, _arg_userId: i32, _arg_appId: i32) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(&_arg_userId)?;
              aidl_data.write(&_arg_appId)?;
              Ok(aidl_data)
            }
            fn read_response_appRemoved(&self, _arg_userId: i32, _arg_appId: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationMaintenance>::getDefaultImpl() {
                  return _aidl_default_impl.r#appRemoved(_arg_userId, _arg_appId);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_userRemoved(&self, _arg_userId: i32) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(&_arg_userId)?;
              Ok(aidl_data)
            }
            fn read_response_userRemoved(&self, _arg_userId: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationMaintenance>::getDefaultImpl() {
                  return _aidl_default_impl.r#userRemoved(_arg_userId);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_performReconciliation(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_6_system_25_virtualizationmaintenance_37_IVirtualizationReconciliationCallback>) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_callback)?;
              Ok(aidl_data)
            }
            fn read_response_performReconciliation(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_6_system_25_virtualizationmaintenance_37_IVirtualizationReconciliationCallback>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationMaintenance>::getDefaultImpl() {
                  return _aidl_default_impl.r#performReconciliation(_arg_callback);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
          }
          impl IVirtualizationMaintenance for BpVirtualizationMaintenance {
            fn r#appRemoved(&self, _arg_userId: i32, _arg_appId: i32) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_appRemoved(_arg_userId, _arg_appId)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#appRemoved, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_appRemoved(_arg_userId, _arg_appId, _aidl_reply)
            }
            fn r#userRemoved(&self, _arg_userId: i32) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_userRemoved(_arg_userId)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#userRemoved, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_userRemoved(_arg_userId, _aidl_reply)
            }
            fn r#performReconciliation(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_6_system_25_virtualizationmaintenance_37_IVirtualizationReconciliationCallback>) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_performReconciliation(_arg_callback)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#performReconciliation, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_performReconciliation(_arg_callback, _aidl_reply)
            }
          }
          impl<P: binder::BinderAsyncPool> IVirtualizationMaintenanceAsync<P> for BpVirtualizationMaintenance {
            fn r#appRemoved<'a>(&'a self, _arg_userId: i32, _arg_appId: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_appRemoved(_arg_userId, _arg_appId) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#appRemoved, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_appRemoved(_arg_userId, _arg_appId, _aidl_reply)
                }
              )
            }
            fn r#userRemoved<'a>(&'a self, _arg_userId: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_userRemoved(_arg_userId) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#userRemoved, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_userRemoved(_arg_userId, _aidl_reply)
                }
              )
            }
            fn r#performReconciliation<'a>(&'a self, _arg_callback: &'a binder::Strong<dyn crate::mangled::_7_android_6_system_25_virtualizationmaintenance_37_IVirtualizationReconciliationCallback>) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_performReconciliation(_arg_callback) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#performReconciliation, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_performReconciliation(_arg_callback, _aidl_reply)
                }
              )
            }
          }
          impl IVirtualizationMaintenance for binder::binder_impl::Binder<BnVirtualizationMaintenance> {
            fn r#appRemoved(&self, _arg_userId: i32, _arg_appId: i32) -> binder::Result<()> { self.0.r#appRemoved(_arg_userId, _arg_appId) }
            fn r#userRemoved(&self, _arg_userId: i32) -> binder::Result<()> { self.0.r#userRemoved(_arg_userId) }
            fn r#performReconciliation(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_6_system_25_virtualizationmaintenance_37_IVirtualizationReconciliationCallback>) -> binder::Result<()> { self.0.r#performReconciliation(_arg_callback) }
          }
          fn on_transact(_aidl_service: &dyn IVirtualizationMaintenance, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
            match _aidl_code {
              transactions::r#appRemoved => {
                let _arg_userId: i32 = _aidl_data.read()?;
                let _arg_appId: i32 = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#appRemoved(_arg_userId, _arg_appId);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#userRemoved => {
                let _arg_userId: i32 = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#userRemoved(_arg_userId);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#performReconciliation => {
                let _arg_callback: binder::Strong<dyn crate::mangled::_7_android_6_system_25_virtualizationmaintenance_37_IVirtualizationReconciliationCallback> = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#performReconciliation(&_arg_callback);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
            }
          }
          pub(crate) mod mangled {
           pub use super::r#IVirtualizationMaintenance as _7_android_6_system_25_virtualizationmaintenance_26_IVirtualizationMaintenance;
          }
        }
        pub mod IVirtualizationReconciliationCallback {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationmaintenance-rust-source/gen/android/system/virtualizationmaintenance/IVirtualizationReconciliationCallback.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationmaintenance-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationmaintenance/IVirtualizationReconciliationCallback.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #![allow(non_upper_case_globals)]
          #![allow(non_snake_case)]
          #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
          use binder::declare_binder_interface;
          declare_binder_interface! {
            IVirtualizationReconciliationCallback["android.system.virtualizationmaintenance.IVirtualizationReconciliationCallback"] {
              native: BnVirtualizationReconciliationCallback(on_transact),
              proxy: BpVirtualizationReconciliationCallback {
              },
              async: IVirtualizationReconciliationCallbackAsync(try_into_local_async),
            }
          }
          pub trait IVirtualizationReconciliationCallback: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationmaintenance.IVirtualizationReconciliationCallback" }
            fn r#doUsersExist(&self, _arg_userIds: &[i32]) -> binder::Result<Vec<bool>>;
            fn r#doAppsExist(&self, _arg_userId: i32, _arg_appIds: &[i32]) -> binder::Result<Vec<bool>>;
            fn getDefaultImpl() -> IVirtualizationReconciliationCallbackDefaultRef where Self: Sized {
              DEFAULT_IMPL.lock().unwrap().clone()
            }
            fn setDefaultImpl(d: IVirtualizationReconciliationCallbackDefaultRef) -> IVirtualizationReconciliationCallbackDefaultRef where Self: Sized {
              std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
            }
            fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn IVirtualizationReconciliationCallbackAsyncServer + Send + Sync)> {
              None
            }
          }
          pub trait IVirtualizationReconciliationCallbackAsync<P>: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationmaintenance.IVirtualizationReconciliationCallback" }
            fn r#doUsersExist<'a>(&'a self, _arg_userIds: &'a [i32]) -> binder::BoxFuture<'a, binder::Result<Vec<bool>>>;
            fn r#doAppsExist<'a>(&'a self, _arg_userId: i32, _arg_appIds: &'a [i32]) -> binder::BoxFuture<'a, binder::Result<Vec<bool>>>;
          }
          #[::async_trait::async_trait]
          pub trait IVirtualizationReconciliationCallbackAsyncServer: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationmaintenance.IVirtualizationReconciliationCallback" }
            async fn r#doUsersExist(&self, _arg_userIds: &[i32]) -> binder::Result<Vec<bool>>;
            async fn r#doAppsExist(&self, _arg_userId: i32, _arg_appIds: &[i32]) -> binder::Result<Vec<bool>>;
          }
          impl BnVirtualizationReconciliationCallback {
            /// Create a new async binder service.
            pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IVirtualizationReconciliationCallback>
            where
              T: IVirtualizationReconciliationCallbackAsyncServer + binder::Interface + Send + Sync + 'static,
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
              impl<T, R> IVirtualizationReconciliationCallback for Wrapper<T, R>
              where
                T: IVirtualizationReconciliationCallbackAsyncServer + Send + Sync + 'static,
                R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
              {
                fn r#doUsersExist(&self, _arg_userIds: &[i32]) -> binder::Result<Vec<bool>> {
                  self._rt.block_on(self._inner.r#doUsersExist(_arg_userIds))
                }
                fn r#doAppsExist(&self, _arg_userId: i32, _arg_appIds: &[i32]) -> binder::Result<Vec<bool>> {
                  self._rt.block_on(self._inner.r#doAppsExist(_arg_userId, _arg_appIds))
                }
                fn try_as_async_server(&self) -> Option<&(dyn IVirtualizationReconciliationCallbackAsyncServer + Send + Sync)> {
                  Some(&self._inner)
                }
              }
              let wrapped = Wrapper { _inner: inner, _rt: rt };
              Self::new_binder(wrapped, features)
            }
            pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn IVirtualizationReconciliationCallbackAsync<P>>> {
              struct Wrapper {
                _native: binder::binder_impl::Binder<BnVirtualizationReconciliationCallback>
              }
              impl binder::Interface for Wrapper {}
              impl<P: binder::BinderAsyncPool> IVirtualizationReconciliationCallbackAsync<P> for Wrapper {
                fn r#doUsersExist<'a>(&'a self, _arg_userIds: &'a [i32]) -> binder::BoxFuture<'a, binder::Result<Vec<bool>>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#doUsersExist(_arg_userIds))
                }
                fn r#doAppsExist<'a>(&'a self, _arg_userId: i32, _arg_appIds: &'a [i32]) -> binder::BoxFuture<'a, binder::Result<Vec<bool>>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#doAppsExist(_arg_userId, _arg_appIds))
                }
              }
              if _native.try_as_async_server().is_some() {
                Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn IVirtualizationReconciliationCallbackAsync<P>>))
              } else {
                None
              }
            }
          }
          pub trait IVirtualizationReconciliationCallbackDefault: Send + Sync {
            fn r#doUsersExist(&self, _arg_userIds: &[i32]) -> binder::Result<Vec<bool>> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#doAppsExist(&self, _arg_userId: i32, _arg_appIds: &[i32]) -> binder::Result<Vec<bool>> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
          }
          pub mod transactions {
            pub const r#doUsersExist: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
            pub const r#doAppsExist: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
          }
          pub type IVirtualizationReconciliationCallbackDefaultRef = Option<std::sync::Arc<dyn IVirtualizationReconciliationCallbackDefault>>;
          static DEFAULT_IMPL: std::sync::Mutex<IVirtualizationReconciliationCallbackDefaultRef> = std::sync::Mutex::new(None);
          pub const r#ERROR_STOP_REQUESTED: i32 = 1;
          impl BpVirtualizationReconciliationCallback {
            fn build_parcel_doUsersExist(&self, _arg_userIds: &[i32]) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_userIds)?;
              Ok(aidl_data)
            }
            fn read_response_doUsersExist(&self, _arg_userIds: &[i32], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<bool>> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationReconciliationCallback>::getDefaultImpl() {
                  return _aidl_default_impl.r#doUsersExist(_arg_userIds);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: Vec<bool> = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_doAppsExist(&self, _arg_userId: i32, _arg_appIds: &[i32]) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(&_arg_userId)?;
              aidl_data.write(_arg_appIds)?;
              Ok(aidl_data)
            }
            fn read_response_doAppsExist(&self, _arg_userId: i32, _arg_appIds: &[i32], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<bool>> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationReconciliationCallback>::getDefaultImpl() {
                  return _aidl_default_impl.r#doAppsExist(_arg_userId, _arg_appIds);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: Vec<bool> = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
          }
          impl IVirtualizationReconciliationCallback for BpVirtualizationReconciliationCallback {
            fn r#doUsersExist(&self, _arg_userIds: &[i32]) -> binder::Result<Vec<bool>> {
              let _aidl_data = self.build_parcel_doUsersExist(_arg_userIds)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#doUsersExist, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_doUsersExist(_arg_userIds, _aidl_reply)
            }
            fn r#doAppsExist(&self, _arg_userId: i32, _arg_appIds: &[i32]) -> binder::Result<Vec<bool>> {
              let _aidl_data = self.build_parcel_doAppsExist(_arg_userId, _arg_appIds)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#doAppsExist, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_doAppsExist(_arg_userId, _arg_appIds, _aidl_reply)
            }
          }
          impl<P: binder::BinderAsyncPool> IVirtualizationReconciliationCallbackAsync<P> for BpVirtualizationReconciliationCallback {
            fn r#doUsersExist<'a>(&'a self, _arg_userIds: &'a [i32]) -> binder::BoxFuture<'a, binder::Result<Vec<bool>>> {
              let _aidl_data = match self.build_parcel_doUsersExist(_arg_userIds) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#doUsersExist, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_doUsersExist(_arg_userIds, _aidl_reply)
                }
              )
            }
            fn r#doAppsExist<'a>(&'a self, _arg_userId: i32, _arg_appIds: &'a [i32]) -> binder::BoxFuture<'a, binder::Result<Vec<bool>>> {
              let _aidl_data = match self.build_parcel_doAppsExist(_arg_userId, _arg_appIds) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#doAppsExist, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_doAppsExist(_arg_userId, _arg_appIds, _aidl_reply)
                }
              )
            }
          }
          impl IVirtualizationReconciliationCallback for binder::binder_impl::Binder<BnVirtualizationReconciliationCallback> {
            fn r#doUsersExist(&self, _arg_userIds: &[i32]) -> binder::Result<Vec<bool>> { self.0.r#doUsersExist(_arg_userIds) }
            fn r#doAppsExist(&self, _arg_userId: i32, _arg_appIds: &[i32]) -> binder::Result<Vec<bool>> { self.0.r#doAppsExist(_arg_userId, _arg_appIds) }
          }
          fn on_transact(_aidl_service: &dyn IVirtualizationReconciliationCallback, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
            match _aidl_code {
              transactions::r#doUsersExist => {
                let _arg_userIds: Vec<i32> = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#doUsersExist(&_arg_userIds);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#doAppsExist => {
                let _arg_userId: i32 = _aidl_data.read()?;
                let _arg_appIds: Vec<i32> = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#doAppsExist(_arg_userId, &_arg_appIds);
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
           pub use super::r#IVirtualizationReconciliationCallback as _7_android_6_system_25_virtualizationmaintenance_37_IVirtualizationReconciliationCallback;
          }
        }
      }
    }
  }
}
pub mod mangled {
  pub use super::aidl::android::system::virtualizationmaintenance::IVirtualizationMaintenance::mangled::*;
  pub use super::aidl::android::system::virtualizationmaintenance::IVirtualizationReconciliationCallback::mangled::*;
}

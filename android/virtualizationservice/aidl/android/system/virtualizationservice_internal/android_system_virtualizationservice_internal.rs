#![allow(non_snake_case)]
#![allow(missing_docs)]
#[deprecated(note = "Please access via libbinder_rs binder::")]
pub use binder;
pub mod aidl {
  pub mod android {
    pub mod system {
      pub mod virtualizationservice_internal {
        pub mod AtomVmBooted {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice_internal-rust-source/gen/android/system/virtualizationservice_internal/AtomVmBooted.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice_internal-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice_internal/AtomVmBooted.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug)]
          pub struct r#AtomVmBooted {
            pub r#uid: i32,
            pub r#vmIdentifier: String,
            pub r#elapsedTimeMillis: i64,
          }
          impl Default for r#AtomVmBooted {
            fn default() -> Self {
              Self {
                r#uid: 0,
                r#vmIdentifier: Default::default(),
                r#elapsedTimeMillis: 0,
              }
            }
          }
          impl binder::Parcelable for r#AtomVmBooted {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#uid)?;
                subparcel.write(&self.r#vmIdentifier)?;
                subparcel.write(&self.r#elapsedTimeMillis)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#uid = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#vmIdentifier = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#elapsedTimeMillis = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#AtomVmBooted);
          binder::impl_deserialize_for_parcelable!(r#AtomVmBooted);
          impl binder::binder_impl::ParcelableMetadata for r#AtomVmBooted {
            fn get_descriptor() -> &'static str { "android.system.virtualizationservice_internal.AtomVmBooted" }
          }
          pub(crate) mod mangled {
           pub use super::r#AtomVmBooted as _7_android_6_system_30_virtualizationservice_internal_12_AtomVmBooted;
          }
        }
        pub mod AtomVmCreationRequested {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice_internal-rust-source/gen/android/system/virtualizationservice_internal/AtomVmCreationRequested.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice_internal-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice_internal/AtomVmCreationRequested.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug)]
          pub struct r#AtomVmCreationRequested {
            pub r#uid: i32,
            pub r#vmIdentifier: String,
            pub r#isProtected: bool,
            pub r#creationSucceeded: bool,
            pub r#binderExceptionCode: i32,
            pub r#configType: i32,
            pub r#numCpus: i32,
            pub r#memoryMib: i32,
            pub r#apexes: String,
          }
          impl Default for r#AtomVmCreationRequested {
            fn default() -> Self {
              Self {
                r#uid: 0,
                r#vmIdentifier: Default::default(),
                r#isProtected: false,
                r#creationSucceeded: false,
                r#binderExceptionCode: 0,
                r#configType: 0,
                r#numCpus: 0,
                r#memoryMib: 0,
                r#apexes: Default::default(),
              }
            }
          }
          impl binder::Parcelable for r#AtomVmCreationRequested {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#uid)?;
                subparcel.write(&self.r#vmIdentifier)?;
                subparcel.write(&self.r#isProtected)?;
                subparcel.write(&self.r#creationSucceeded)?;
                subparcel.write(&self.r#binderExceptionCode)?;
                subparcel.write(&self.r#configType)?;
                subparcel.write(&self.r#numCpus)?;
                subparcel.write(&self.r#memoryMib)?;
                subparcel.write(&self.r#apexes)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#uid = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#vmIdentifier = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#isProtected = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#creationSucceeded = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#binderExceptionCode = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#configType = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#numCpus = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#memoryMib = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#apexes = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#AtomVmCreationRequested);
          binder::impl_deserialize_for_parcelable!(r#AtomVmCreationRequested);
          impl binder::binder_impl::ParcelableMetadata for r#AtomVmCreationRequested {
            fn get_descriptor() -> &'static str { "android.system.virtualizationservice_internal.AtomVmCreationRequested" }
          }
          pub(crate) mod mangled {
           pub use super::r#AtomVmCreationRequested as _7_android_6_system_30_virtualizationservice_internal_23_AtomVmCreationRequested;
          }
        }
        pub mod AtomVmExited {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice_internal-rust-source/gen/android/system/virtualizationservice_internal/AtomVmExited.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice_internal-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice_internal/AtomVmExited.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug)]
          pub struct r#AtomVmExited {
            pub r#uid: i32,
            pub r#vmIdentifier: String,
            pub r#deathReason: crate::mangled::_7_android_6_system_20_virtualizationcommon_11_DeathReason,
            pub r#exitSignal: i32,
            pub r#elapsedTimeMillis: i64,
            pub r#guestTimeMillis: i64,
            pub r#rssVmKb: i64,
            pub r#rssCrosvmKb: i64,
          }
          impl Default for r#AtomVmExited {
            fn default() -> Self {
              Self {
                r#uid: 0,
                r#vmIdentifier: Default::default(),
                r#deathReason: Default::default(),
                r#exitSignal: 0,
                r#elapsedTimeMillis: 0,
                r#guestTimeMillis: 0,
                r#rssVmKb: 0,
                r#rssCrosvmKb: 0,
              }
            }
          }
          impl binder::Parcelable for r#AtomVmExited {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#uid)?;
                subparcel.write(&self.r#vmIdentifier)?;
                subparcel.write(&self.r#deathReason)?;
                subparcel.write(&self.r#exitSignal)?;
                subparcel.write(&self.r#elapsedTimeMillis)?;
                subparcel.write(&self.r#guestTimeMillis)?;
                subparcel.write(&self.r#rssVmKb)?;
                subparcel.write(&self.r#rssCrosvmKb)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#uid = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#vmIdentifier = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#deathReason = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#exitSignal = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#elapsedTimeMillis = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#guestTimeMillis = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#rssVmKb = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#rssCrosvmKb = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#AtomVmExited);
          binder::impl_deserialize_for_parcelable!(r#AtomVmExited);
          impl binder::binder_impl::ParcelableMetadata for r#AtomVmExited {
            fn get_descriptor() -> &'static str { "android.system.virtualizationservice_internal.AtomVmExited" }
          }
          pub(crate) mod mangled {
           pub use super::r#AtomVmExited as _7_android_6_system_30_virtualizationservice_internal_12_AtomVmExited;
          }
        }
        pub mod IBoundDevice {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice_internal-rust-source/gen/android/system/virtualizationservice_internal/IBoundDevice.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice_internal-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice_internal/IBoundDevice.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #![allow(non_upper_case_globals)]
          #![allow(non_snake_case)]
          #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
          use binder::declare_binder_interface;
          declare_binder_interface! {
            IBoundDevice["android.system.virtualizationservice_internal.IBoundDevice"] {
              native: BnBoundDevice(on_transact),
              proxy: BpBoundDevice {
              },
              async: IBoundDeviceAsync(try_into_local_async),
            }
          }
          pub trait IBoundDevice: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationservice_internal.IBoundDevice" }
            fn r#getSysfsPath(&self) -> binder::Result<String>;
            fn r#getDtboLabel(&self) -> binder::Result<String>;
            fn getDefaultImpl() -> IBoundDeviceDefaultRef where Self: Sized {
              DEFAULT_IMPL.lock().unwrap().clone()
            }
            fn setDefaultImpl(d: IBoundDeviceDefaultRef) -> IBoundDeviceDefaultRef where Self: Sized {
              std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
            }
            fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn IBoundDeviceAsyncServer + Send + Sync)> {
              None
            }
          }
          pub trait IBoundDeviceAsync<P>: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationservice_internal.IBoundDevice" }
            fn r#getSysfsPath<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>>;
            fn r#getDtboLabel<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>>;
          }
          #[::async_trait::async_trait]
          pub trait IBoundDeviceAsyncServer: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationservice_internal.IBoundDevice" }
            async fn r#getSysfsPath(&self) -> binder::Result<String>;
            async fn r#getDtboLabel(&self) -> binder::Result<String>;
          }
          impl BnBoundDevice {
            /// Create a new async binder service.
            pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IBoundDevice>
            where
              T: IBoundDeviceAsyncServer + binder::Interface + Send + Sync + 'static,
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
              impl<T, R> IBoundDevice for Wrapper<T, R>
              where
                T: IBoundDeviceAsyncServer + Send + Sync + 'static,
                R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
              {
                fn r#getSysfsPath(&self) -> binder::Result<String> {
                  self._rt.block_on(self._inner.r#getSysfsPath())
                }
                fn r#getDtboLabel(&self) -> binder::Result<String> {
                  self._rt.block_on(self._inner.r#getDtboLabel())
                }
                fn try_as_async_server(&self) -> Option<&(dyn IBoundDeviceAsyncServer + Send + Sync)> {
                  Some(&self._inner)
                }
              }
              let wrapped = Wrapper { _inner: inner, _rt: rt };
              Self::new_binder(wrapped, features)
            }
            pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn IBoundDeviceAsync<P>>> {
              struct Wrapper {
                _native: binder::binder_impl::Binder<BnBoundDevice>
              }
              impl binder::Interface for Wrapper {}
              impl<P: binder::BinderAsyncPool> IBoundDeviceAsync<P> for Wrapper {
                fn r#getSysfsPath<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#getSysfsPath())
                }
                fn r#getDtboLabel<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#getDtboLabel())
                }
              }
              if _native.try_as_async_server().is_some() {
                Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn IBoundDeviceAsync<P>>))
              } else {
                None
              }
            }
          }
          pub trait IBoundDeviceDefault: Send + Sync {
            fn r#getSysfsPath(&self) -> binder::Result<String> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#getDtboLabel(&self) -> binder::Result<String> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
          }
          pub mod transactions {
            pub const r#getSysfsPath: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
            pub const r#getDtboLabel: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
          }
          pub type IBoundDeviceDefaultRef = Option<std::sync::Arc<dyn IBoundDeviceDefault>>;
          static DEFAULT_IMPL: std::sync::Mutex<IBoundDeviceDefaultRef> = std::sync::Mutex::new(None);
          impl BpBoundDevice {
            fn build_parcel_getSysfsPath(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_getSysfsPath(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IBoundDevice>::getDefaultImpl() {
                  return _aidl_default_impl.r#getSysfsPath();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: String = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_getDtboLabel(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_getDtboLabel(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IBoundDevice>::getDefaultImpl() {
                  return _aidl_default_impl.r#getDtboLabel();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: String = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
          }
          impl IBoundDevice for BpBoundDevice {
            fn r#getSysfsPath(&self) -> binder::Result<String> {
              let _aidl_data = self.build_parcel_getSysfsPath()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#getSysfsPath, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_getSysfsPath(_aidl_reply)
            }
            fn r#getDtboLabel(&self) -> binder::Result<String> {
              let _aidl_data = self.build_parcel_getDtboLabel()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#getDtboLabel, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_getDtboLabel(_aidl_reply)
            }
          }
          impl<P: binder::BinderAsyncPool> IBoundDeviceAsync<P> for BpBoundDevice {
            fn r#getSysfsPath<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
              let _aidl_data = match self.build_parcel_getSysfsPath() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#getSysfsPath, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_getSysfsPath(_aidl_reply)
                }
              )
            }
            fn r#getDtboLabel<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
              let _aidl_data = match self.build_parcel_getDtboLabel() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#getDtboLabel, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_getDtboLabel(_aidl_reply)
                }
              )
            }
          }
          impl IBoundDevice for binder::binder_impl::Binder<BnBoundDevice> {
            fn r#getSysfsPath(&self) -> binder::Result<String> { self.0.r#getSysfsPath() }
            fn r#getDtboLabel(&self) -> binder::Result<String> { self.0.r#getDtboLabel() }
          }
          fn on_transact(_aidl_service: &dyn IBoundDevice, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
            match _aidl_code {
              transactions::r#getSysfsPath => {
                let _aidl_return = _aidl_service.r#getSysfsPath();
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#getDtboLabel => {
                let _aidl_return = _aidl_service.r#getDtboLabel();
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
           pub use super::r#IBoundDevice as _7_android_6_system_30_virtualizationservice_internal_12_IBoundDevice;
          }
        }
        pub mod IGlobalVmContext {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice_internal-rust-source/gen/android/system/virtualizationservice_internal/IGlobalVmContext.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice_internal-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice_internal/IGlobalVmContext.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #![allow(non_upper_case_globals)]
          #![allow(non_snake_case)]
          #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
          use binder::declare_binder_interface;
          declare_binder_interface! {
            IGlobalVmContext["android.system.virtualizationservice_internal.IGlobalVmContext"] {
              native: BnGlobalVmContext(on_transact),
              proxy: BpGlobalVmContext {
              },
              async: IGlobalVmContextAsync(try_into_local_async),
            }
          }
          pub trait IGlobalVmContext: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationservice_internal.IGlobalVmContext" }
            fn r#getCid(&self) -> binder::Result<i32>;
            fn r#getTemporaryDirectory(&self) -> binder::Result<String>;
            fn r#setHostConsoleName(&self, _arg_pathname: &str) -> binder::Result<()>;
            fn getDefaultImpl() -> IGlobalVmContextDefaultRef where Self: Sized {
              DEFAULT_IMPL.lock().unwrap().clone()
            }
            fn setDefaultImpl(d: IGlobalVmContextDefaultRef) -> IGlobalVmContextDefaultRef where Self: Sized {
              std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
            }
            fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn IGlobalVmContextAsyncServer + Send + Sync)> {
              None
            }
          }
          pub trait IGlobalVmContextAsync<P>: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationservice_internal.IGlobalVmContext" }
            fn r#getCid<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>>;
            fn r#getTemporaryDirectory<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>>;
            fn r#setHostConsoleName<'a>(&'a self, _arg_pathname: &'a str) -> binder::BoxFuture<'a, binder::Result<()>>;
          }
          #[::async_trait::async_trait]
          pub trait IGlobalVmContextAsyncServer: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationservice_internal.IGlobalVmContext" }
            async fn r#getCid(&self) -> binder::Result<i32>;
            async fn r#getTemporaryDirectory(&self) -> binder::Result<String>;
            async fn r#setHostConsoleName(&self, _arg_pathname: &str) -> binder::Result<()>;
          }
          impl BnGlobalVmContext {
            /// Create a new async binder service.
            pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IGlobalVmContext>
            where
              T: IGlobalVmContextAsyncServer + binder::Interface + Send + Sync + 'static,
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
              impl<T, R> IGlobalVmContext for Wrapper<T, R>
              where
                T: IGlobalVmContextAsyncServer + Send + Sync + 'static,
                R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
              {
                fn r#getCid(&self) -> binder::Result<i32> {
                  self._rt.block_on(self._inner.r#getCid())
                }
                fn r#getTemporaryDirectory(&self) -> binder::Result<String> {
                  self._rt.block_on(self._inner.r#getTemporaryDirectory())
                }
                fn r#setHostConsoleName(&self, _arg_pathname: &str) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#setHostConsoleName(_arg_pathname))
                }
                fn try_as_async_server(&self) -> Option<&(dyn IGlobalVmContextAsyncServer + Send + Sync)> {
                  Some(&self._inner)
                }
              }
              let wrapped = Wrapper { _inner: inner, _rt: rt };
              Self::new_binder(wrapped, features)
            }
            pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn IGlobalVmContextAsync<P>>> {
              struct Wrapper {
                _native: binder::binder_impl::Binder<BnGlobalVmContext>
              }
              impl binder::Interface for Wrapper {}
              impl<P: binder::BinderAsyncPool> IGlobalVmContextAsync<P> for Wrapper {
                fn r#getCid<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#getCid())
                }
                fn r#getTemporaryDirectory<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#getTemporaryDirectory())
                }
                fn r#setHostConsoleName<'a>(&'a self, _arg_pathname: &'a str) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#setHostConsoleName(_arg_pathname))
                }
              }
              if _native.try_as_async_server().is_some() {
                Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn IGlobalVmContextAsync<P>>))
              } else {
                None
              }
            }
          }
          pub trait IGlobalVmContextDefault: Send + Sync {
            fn r#getCid(&self) -> binder::Result<i32> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#getTemporaryDirectory(&self) -> binder::Result<String> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#setHostConsoleName(&self, _arg_pathname: &str) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
          }
          pub mod transactions {
            pub const r#getCid: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
            pub const r#getTemporaryDirectory: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
            pub const r#setHostConsoleName: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
          }
          pub type IGlobalVmContextDefaultRef = Option<std::sync::Arc<dyn IGlobalVmContextDefault>>;
          static DEFAULT_IMPL: std::sync::Mutex<IGlobalVmContextDefaultRef> = std::sync::Mutex::new(None);
          impl BpGlobalVmContext {
            fn build_parcel_getCid(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_getCid(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IGlobalVmContext>::getDefaultImpl() {
                  return _aidl_default_impl.r#getCid();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: i32 = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_getTemporaryDirectory(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_getTemporaryDirectory(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IGlobalVmContext>::getDefaultImpl() {
                  return _aidl_default_impl.r#getTemporaryDirectory();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: String = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_setHostConsoleName(&self, _arg_pathname: &str) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_pathname)?;
              Ok(aidl_data)
            }
            fn read_response_setHostConsoleName(&self, _arg_pathname: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IGlobalVmContext>::getDefaultImpl() {
                  return _aidl_default_impl.r#setHostConsoleName(_arg_pathname);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
          }
          impl IGlobalVmContext for BpGlobalVmContext {
            fn r#getCid(&self) -> binder::Result<i32> {
              let _aidl_data = self.build_parcel_getCid()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#getCid, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_getCid(_aidl_reply)
            }
            fn r#getTemporaryDirectory(&self) -> binder::Result<String> {
              let _aidl_data = self.build_parcel_getTemporaryDirectory()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#getTemporaryDirectory, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_getTemporaryDirectory(_aidl_reply)
            }
            fn r#setHostConsoleName(&self, _arg_pathname: &str) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_setHostConsoleName(_arg_pathname)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#setHostConsoleName, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_setHostConsoleName(_arg_pathname, _aidl_reply)
            }
          }
          impl<P: binder::BinderAsyncPool> IGlobalVmContextAsync<P> for BpGlobalVmContext {
            fn r#getCid<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
              let _aidl_data = match self.build_parcel_getCid() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#getCid, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_getCid(_aidl_reply)
                }
              )
            }
            fn r#getTemporaryDirectory<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
              let _aidl_data = match self.build_parcel_getTemporaryDirectory() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#getTemporaryDirectory, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_getTemporaryDirectory(_aidl_reply)
                }
              )
            }
            fn r#setHostConsoleName<'a>(&'a self, _arg_pathname: &'a str) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_setHostConsoleName(_arg_pathname) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#setHostConsoleName, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_setHostConsoleName(_arg_pathname, _aidl_reply)
                }
              )
            }
          }
          impl IGlobalVmContext for binder::binder_impl::Binder<BnGlobalVmContext> {
            fn r#getCid(&self) -> binder::Result<i32> { self.0.r#getCid() }
            fn r#getTemporaryDirectory(&self) -> binder::Result<String> { self.0.r#getTemporaryDirectory() }
            fn r#setHostConsoleName(&self, _arg_pathname: &str) -> binder::Result<()> { self.0.r#setHostConsoleName(_arg_pathname) }
          }
          fn on_transact(_aidl_service: &dyn IGlobalVmContext, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
            match _aidl_code {
              transactions::r#getCid => {
                let _aidl_return = _aidl_service.r#getCid();
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#getTemporaryDirectory => {
                let _aidl_return = _aidl_service.r#getTemporaryDirectory();
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#setHostConsoleName => {
                let _arg_pathname: String = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#setHostConsoleName(&_arg_pathname);
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
           pub use super::r#IGlobalVmContext as _7_android_6_system_30_virtualizationservice_internal_16_IGlobalVmContext;
          }
        }
        pub mod IVfioHandler {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice_internal-rust-source/gen/android/system/virtualizationservice_internal/IVfioHandler.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice_internal-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice_internal/IVfioHandler.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #![allow(non_upper_case_globals)]
          #![allow(non_snake_case)]
          #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
          use binder::declare_binder_interface;
          declare_binder_interface! {
            IVfioHandler["android.system.virtualizationservice_internal.IVfioHandler"] {
              native: BnVfioHandler(on_transact),
              proxy: BpVfioHandler {
              },
              async: IVfioHandlerAsync(try_into_local_async),
            }
          }
          pub trait IVfioHandler: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationservice_internal.IVfioHandler" }
            fn r#bindDevicesToVfioDriver(&self, _arg_devices: &[crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IVfioHandler_7_VfioDev]) -> binder::Result<Vec<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IBoundDevice>>>;
            fn r#writeVmDtbo(&self, _arg_dtbo: &binder::ParcelFileDescriptor) -> binder::Result<()>;
            fn getDefaultImpl() -> IVfioHandlerDefaultRef where Self: Sized {
              DEFAULT_IMPL.lock().unwrap().clone()
            }
            fn setDefaultImpl(d: IVfioHandlerDefaultRef) -> IVfioHandlerDefaultRef where Self: Sized {
              std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
            }
            fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn IVfioHandlerAsyncServer + Send + Sync)> {
              None
            }
          }
          pub trait IVfioHandlerAsync<P>: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationservice_internal.IVfioHandler" }
            fn r#bindDevicesToVfioDriver<'a>(&'a self, _arg_devices: &'a [crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IVfioHandler_7_VfioDev]) -> binder::BoxFuture<'a, binder::Result<Vec<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IBoundDevice>>>>;
            fn r#writeVmDtbo<'a>(&'a self, _arg_dtbo: &'a binder::ParcelFileDescriptor) -> binder::BoxFuture<'a, binder::Result<()>>;
          }
          #[::async_trait::async_trait]
          pub trait IVfioHandlerAsyncServer: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationservice_internal.IVfioHandler" }
            async fn r#bindDevicesToVfioDriver(&self, _arg_devices: &[crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IVfioHandler_7_VfioDev]) -> binder::Result<Vec<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IBoundDevice>>>;
            async fn r#writeVmDtbo(&self, _arg_dtbo: &binder::ParcelFileDescriptor) -> binder::Result<()>;
          }
          impl BnVfioHandler {
            /// Create a new async binder service.
            pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IVfioHandler>
            where
              T: IVfioHandlerAsyncServer + binder::Interface + Send + Sync + 'static,
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
              impl<T, R> IVfioHandler for Wrapper<T, R>
              where
                T: IVfioHandlerAsyncServer + Send + Sync + 'static,
                R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
              {
                fn r#bindDevicesToVfioDriver(&self, _arg_devices: &[crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IVfioHandler_7_VfioDev]) -> binder::Result<Vec<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IBoundDevice>>> {
                  self._rt.block_on(self._inner.r#bindDevicesToVfioDriver(_arg_devices))
                }
                fn r#writeVmDtbo(&self, _arg_dtbo: &binder::ParcelFileDescriptor) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#writeVmDtbo(_arg_dtbo))
                }
                fn try_as_async_server(&self) -> Option<&(dyn IVfioHandlerAsyncServer + Send + Sync)> {
                  Some(&self._inner)
                }
              }
              let wrapped = Wrapper { _inner: inner, _rt: rt };
              Self::new_binder(wrapped, features)
            }
            pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn IVfioHandlerAsync<P>>> {
              struct Wrapper {
                _native: binder::binder_impl::Binder<BnVfioHandler>
              }
              impl binder::Interface for Wrapper {}
              impl<P: binder::BinderAsyncPool> IVfioHandlerAsync<P> for Wrapper {
                fn r#bindDevicesToVfioDriver<'a>(&'a self, _arg_devices: &'a [crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IVfioHandler_7_VfioDev]) -> binder::BoxFuture<'a, binder::Result<Vec<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IBoundDevice>>>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#bindDevicesToVfioDriver(_arg_devices))
                }
                fn r#writeVmDtbo<'a>(&'a self, _arg_dtbo: &'a binder::ParcelFileDescriptor) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#writeVmDtbo(_arg_dtbo))
                }
              }
              if _native.try_as_async_server().is_some() {
                Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn IVfioHandlerAsync<P>>))
              } else {
                None
              }
            }
          }
          pub trait IVfioHandlerDefault: Send + Sync {
            fn r#bindDevicesToVfioDriver(&self, _arg_devices: &[crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IVfioHandler_7_VfioDev]) -> binder::Result<Vec<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IBoundDevice>>> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#writeVmDtbo(&self, _arg_dtbo: &binder::ParcelFileDescriptor) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
          }
          pub mod transactions {
            pub const r#bindDevicesToVfioDriver: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
            pub const r#writeVmDtbo: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
          }
          pub type IVfioHandlerDefaultRef = Option<std::sync::Arc<dyn IVfioHandlerDefault>>;
          static DEFAULT_IMPL: std::sync::Mutex<IVfioHandlerDefaultRef> = std::sync::Mutex::new(None);
          impl BpVfioHandler {
            fn build_parcel_bindDevicesToVfioDriver(&self, _arg_devices: &[crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IVfioHandler_7_VfioDev]) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_devices)?;
              Ok(aidl_data)
            }
            fn read_response_bindDevicesToVfioDriver(&self, _arg_devices: &[crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IVfioHandler_7_VfioDev], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IBoundDevice>>> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVfioHandler>::getDefaultImpl() {
                  return _aidl_default_impl.r#bindDevicesToVfioDriver(_arg_devices);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: Vec<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IBoundDevice>> = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_writeVmDtbo(&self, _arg_dtbo: &binder::ParcelFileDescriptor) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_dtbo)?;
              Ok(aidl_data)
            }
            fn read_response_writeVmDtbo(&self, _arg_dtbo: &binder::ParcelFileDescriptor, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVfioHandler>::getDefaultImpl() {
                  return _aidl_default_impl.r#writeVmDtbo(_arg_dtbo);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
          }
          impl IVfioHandler for BpVfioHandler {
            fn r#bindDevicesToVfioDriver(&self, _arg_devices: &[crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IVfioHandler_7_VfioDev]) -> binder::Result<Vec<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IBoundDevice>>> {
              let _aidl_data = self.build_parcel_bindDevicesToVfioDriver(_arg_devices)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#bindDevicesToVfioDriver, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_bindDevicesToVfioDriver(_arg_devices, _aidl_reply)
            }
            fn r#writeVmDtbo(&self, _arg_dtbo: &binder::ParcelFileDescriptor) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_writeVmDtbo(_arg_dtbo)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#writeVmDtbo, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_writeVmDtbo(_arg_dtbo, _aidl_reply)
            }
          }
          impl<P: binder::BinderAsyncPool> IVfioHandlerAsync<P> for BpVfioHandler {
            fn r#bindDevicesToVfioDriver<'a>(&'a self, _arg_devices: &'a [crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IVfioHandler_7_VfioDev]) -> binder::BoxFuture<'a, binder::Result<Vec<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IBoundDevice>>>> {
              let _aidl_data = match self.build_parcel_bindDevicesToVfioDriver(_arg_devices) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#bindDevicesToVfioDriver, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_bindDevicesToVfioDriver(_arg_devices, _aidl_reply)
                }
              )
            }
            fn r#writeVmDtbo<'a>(&'a self, _arg_dtbo: &'a binder::ParcelFileDescriptor) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_writeVmDtbo(_arg_dtbo) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#writeVmDtbo, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_writeVmDtbo(_arg_dtbo, _aidl_reply)
                }
              )
            }
          }
          impl IVfioHandler for binder::binder_impl::Binder<BnVfioHandler> {
            fn r#bindDevicesToVfioDriver(&self, _arg_devices: &[crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IVfioHandler_7_VfioDev]) -> binder::Result<Vec<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IBoundDevice>>> { self.0.r#bindDevicesToVfioDriver(_arg_devices) }
            fn r#writeVmDtbo(&self, _arg_dtbo: &binder::ParcelFileDescriptor) -> binder::Result<()> { self.0.r#writeVmDtbo(_arg_dtbo) }
          }
          fn on_transact(_aidl_service: &dyn IVfioHandler, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
            match _aidl_code {
              transactions::r#bindDevicesToVfioDriver => {
                let _arg_devices: Vec<crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IVfioHandler_7_VfioDev> = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#bindDevicesToVfioDriver(&_arg_devices);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#writeVmDtbo => {
                let _arg_dtbo: binder::ParcelFileDescriptor = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#writeVmDtbo(&_arg_dtbo);
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
          pub mod r#VfioDev {
            #[derive(Debug)]
            pub struct r#VfioDev {
              pub r#sysfsPath: String,
              pub r#dtboLabel: String,
            }
            impl Default for r#VfioDev {
              fn default() -> Self {
                Self {
                  r#sysfsPath: Default::default(),
                  r#dtboLabel: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#VfioDev {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#sysfsPath)?;
                  subparcel.write(&self.r#dtboLabel)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#sysfsPath = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#dtboLabel = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#VfioDev);
            binder::impl_deserialize_for_parcelable!(r#VfioDev);
            impl binder::binder_impl::ParcelableMetadata for r#VfioDev {
              fn get_descriptor() -> &'static str { "android.system.virtualizationservice_internal.IVfioHandler.VfioDev" }
            }
          }
          pub(crate) mod mangled {
           pub use super::r#IVfioHandler as _7_android_6_system_30_virtualizationservice_internal_12_IVfioHandler;
           pub use super::r#VfioDev::r#VfioDev as _7_android_6_system_30_virtualizationservice_internal_12_IVfioHandler_7_VfioDev;
          }
        }
        pub mod IVirtualizationServiceInternal {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice_internal-rust-source/gen/android/system/virtualizationservice_internal/IVirtualizationServiceInternal.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice_internal-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice_internal/IVirtualizationServiceInternal.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #![allow(non_upper_case_globals)]
          #![allow(non_snake_case)]
          #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
          use binder::declare_binder_interface;
          declare_binder_interface! {
            IVirtualizationServiceInternal["android.system.virtualizationservice_internal.IVirtualizationServiceInternal"] {
              native: BnVirtualizationServiceInternal(on_transact),
              proxy: BpVirtualizationServiceInternal {
              },
              async: IVirtualizationServiceInternalAsync(try_into_local_async),
            }
          }
          pub trait IVirtualizationServiceInternal: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationservice_internal.IVirtualizationServiceInternal" }
            fn r#removeMemlockRlimit(&self) -> binder::Result<()>;
            fn r#allocateGlobalVmContext(&self, _arg_requesterDebugPid: i32) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_16_IGlobalVmContext>>;
            fn r#atomVmBooted(&self, _arg_atom: &crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_AtomVmBooted) -> binder::Result<()>;
            fn r#atomVmCreationRequested(&self, _arg_atom: &crate::mangled::_7_android_6_system_30_virtualizationservice_internal_23_AtomVmCreationRequested) -> binder::Result<()>;
            fn r#atomVmExited(&self, _arg_atom: &crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_AtomVmExited) -> binder::Result<()>;
            fn r#debugListVms(&self) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineDebugInfo>>;
            fn r#requestAttestation(&self, _arg_csr: &[u8], _arg_requesterUid: i32, _arg_testMode: bool) -> binder::Result<Vec<crate::mangled::_7_android_6_system_20_virtualizationcommon_11_Certificate>>;
            fn r#enableTestAttestation(&self) -> binder::Result<()>;
            fn r#isRemoteAttestationSupported(&self) -> binder::Result<bool>;
            fn r#getAssignableDevices(&self) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_16_AssignableDevice>>;
            fn r#bindDevicesToVfioDriver(&self, _arg_devices: &[String]) -> binder::Result<Vec<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IBoundDevice>>>;
            fn r#getDtboFile(&self) -> binder::Result<binder::ParcelFileDescriptor>;
            fn r#allocateInstanceId(&self) -> binder::Result<[u8; 64]>;
            fn r#removeVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<()>;
            fn r#claimVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<()>;
            fn r#setDisplayService(&self, _arg_ibinder: &binder::SpIBinder) -> binder::Result<()>;
            fn r#clearDisplayService(&self) -> binder::Result<()>;
            fn r#waitDisplayService(&self) -> binder::Result<binder::SpIBinder>;
            fn r#createTapInterface(&self, _arg_ifaceNameSuffix: &str) -> binder::Result<binder::ParcelFileDescriptor>;
            fn r#deleteTapInterface(&self, _arg_tapFd: &binder::ParcelFileDescriptor) -> binder::Result<()>;
            fn getDefaultImpl() -> IVirtualizationServiceInternalDefaultRef where Self: Sized {
              DEFAULT_IMPL.lock().unwrap().clone()
            }
            fn setDefaultImpl(d: IVirtualizationServiceInternalDefaultRef) -> IVirtualizationServiceInternalDefaultRef where Self: Sized {
              std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
            }
            fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn IVirtualizationServiceInternalAsyncServer + Send + Sync)> {
              None
            }
          }
          pub trait IVirtualizationServiceInternalAsync<P>: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationservice_internal.IVirtualizationServiceInternal" }
            fn r#removeMemlockRlimit<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#allocateGlobalVmContext<'a>(&'a self, _arg_requesterDebugPid: i32) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_16_IGlobalVmContext>>>;
            fn r#atomVmBooted<'a>(&'a self, _arg_atom: &'a crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_AtomVmBooted) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#atomVmCreationRequested<'a>(&'a self, _arg_atom: &'a crate::mangled::_7_android_6_system_30_virtualizationservice_internal_23_AtomVmCreationRequested) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#atomVmExited<'a>(&'a self, _arg_atom: &'a crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_AtomVmExited) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#debugListVms<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineDebugInfo>>>;
            fn r#requestAttestation<'a>(&'a self, _arg_csr: &'a [u8], _arg_requesterUid: i32, _arg_testMode: bool) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_6_system_20_virtualizationcommon_11_Certificate>>>;
            fn r#enableTestAttestation<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#isRemoteAttestationSupported<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<bool>>;
            fn r#getAssignableDevices<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_16_AssignableDevice>>>;
            fn r#bindDevicesToVfioDriver<'a>(&'a self, _arg_devices: &'a [String]) -> binder::BoxFuture<'a, binder::Result<Vec<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IBoundDevice>>>>;
            fn r#getDtboFile<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<binder::ParcelFileDescriptor>>;
            fn r#allocateInstanceId<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<[u8; 64]>>;
            fn r#removeVmInstance<'a>(&'a self, _arg_instanceId: &'a [u8; 64]) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#claimVmInstance<'a>(&'a self, _arg_instanceId: &'a [u8; 64]) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#setDisplayService<'a>(&'a self, _arg_ibinder: &'a binder::SpIBinder) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#clearDisplayService<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#waitDisplayService<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<binder::SpIBinder>>;
            fn r#createTapInterface<'a>(&'a self, _arg_ifaceNameSuffix: &'a str) -> binder::BoxFuture<'a, binder::Result<binder::ParcelFileDescriptor>>;
            fn r#deleteTapInterface<'a>(&'a self, _arg_tapFd: &'a binder::ParcelFileDescriptor) -> binder::BoxFuture<'a, binder::Result<()>>;
          }
          #[::async_trait::async_trait]
          pub trait IVirtualizationServiceInternalAsyncServer: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationservice_internal.IVirtualizationServiceInternal" }
            async fn r#removeMemlockRlimit(&self) -> binder::Result<()>;
            async fn r#allocateGlobalVmContext(&self, _arg_requesterDebugPid: i32) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_16_IGlobalVmContext>>;
            async fn r#atomVmBooted(&self, _arg_atom: &crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_AtomVmBooted) -> binder::Result<()>;
            async fn r#atomVmCreationRequested(&self, _arg_atom: &crate::mangled::_7_android_6_system_30_virtualizationservice_internal_23_AtomVmCreationRequested) -> binder::Result<()>;
            async fn r#atomVmExited(&self, _arg_atom: &crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_AtomVmExited) -> binder::Result<()>;
            async fn r#debugListVms(&self) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineDebugInfo>>;
            async fn r#requestAttestation(&self, _arg_csr: &[u8], _arg_requesterUid: i32, _arg_testMode: bool) -> binder::Result<Vec<crate::mangled::_7_android_6_system_20_virtualizationcommon_11_Certificate>>;
            async fn r#enableTestAttestation(&self) -> binder::Result<()>;
            async fn r#isRemoteAttestationSupported(&self) -> binder::Result<bool>;
            async fn r#getAssignableDevices(&self) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_16_AssignableDevice>>;
            async fn r#bindDevicesToVfioDriver(&self, _arg_devices: &[String]) -> binder::Result<Vec<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IBoundDevice>>>;
            async fn r#getDtboFile(&self) -> binder::Result<binder::ParcelFileDescriptor>;
            async fn r#allocateInstanceId(&self) -> binder::Result<[u8; 64]>;
            async fn r#removeVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<()>;
            async fn r#claimVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<()>;
            async fn r#setDisplayService(&self, _arg_ibinder: &binder::SpIBinder) -> binder::Result<()>;
            async fn r#clearDisplayService(&self) -> binder::Result<()>;
            async fn r#waitDisplayService(&self) -> binder::Result<binder::SpIBinder>;
            async fn r#createTapInterface(&self, _arg_ifaceNameSuffix: &str) -> binder::Result<binder::ParcelFileDescriptor>;
            async fn r#deleteTapInterface(&self, _arg_tapFd: &binder::ParcelFileDescriptor) -> binder::Result<()>;
          }
          impl BnVirtualizationServiceInternal {
            /// Create a new async binder service.
            pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IVirtualizationServiceInternal>
            where
              T: IVirtualizationServiceInternalAsyncServer + binder::Interface + Send + Sync + 'static,
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
              impl<T, R> IVirtualizationServiceInternal for Wrapper<T, R>
              where
                T: IVirtualizationServiceInternalAsyncServer + Send + Sync + 'static,
                R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
              {
                fn r#removeMemlockRlimit(&self) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#removeMemlockRlimit())
                }
                fn r#allocateGlobalVmContext(&self, _arg_requesterDebugPid: i32) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_16_IGlobalVmContext>> {
                  self._rt.block_on(self._inner.r#allocateGlobalVmContext(_arg_requesterDebugPid))
                }
                fn r#atomVmBooted(&self, _arg_atom: &crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_AtomVmBooted) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#atomVmBooted(_arg_atom))
                }
                fn r#atomVmCreationRequested(&self, _arg_atom: &crate::mangled::_7_android_6_system_30_virtualizationservice_internal_23_AtomVmCreationRequested) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#atomVmCreationRequested(_arg_atom))
                }
                fn r#atomVmExited(&self, _arg_atom: &crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_AtomVmExited) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#atomVmExited(_arg_atom))
                }
                fn r#debugListVms(&self) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineDebugInfo>> {
                  self._rt.block_on(self._inner.r#debugListVms())
                }
                fn r#requestAttestation(&self, _arg_csr: &[u8], _arg_requesterUid: i32, _arg_testMode: bool) -> binder::Result<Vec<crate::mangled::_7_android_6_system_20_virtualizationcommon_11_Certificate>> {
                  self._rt.block_on(self._inner.r#requestAttestation(_arg_csr, _arg_requesterUid, _arg_testMode))
                }
                fn r#enableTestAttestation(&self) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#enableTestAttestation())
                }
                fn r#isRemoteAttestationSupported(&self) -> binder::Result<bool> {
                  self._rt.block_on(self._inner.r#isRemoteAttestationSupported())
                }
                fn r#getAssignableDevices(&self) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_16_AssignableDevice>> {
                  self._rt.block_on(self._inner.r#getAssignableDevices())
                }
                fn r#bindDevicesToVfioDriver(&self, _arg_devices: &[String]) -> binder::Result<Vec<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IBoundDevice>>> {
                  self._rt.block_on(self._inner.r#bindDevicesToVfioDriver(_arg_devices))
                }
                fn r#getDtboFile(&self) -> binder::Result<binder::ParcelFileDescriptor> {
                  self._rt.block_on(self._inner.r#getDtboFile())
                }
                fn r#allocateInstanceId(&self) -> binder::Result<[u8; 64]> {
                  self._rt.block_on(self._inner.r#allocateInstanceId())
                }
                fn r#removeVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#removeVmInstance(_arg_instanceId))
                }
                fn r#claimVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#claimVmInstance(_arg_instanceId))
                }
                fn r#setDisplayService(&self, _arg_ibinder: &binder::SpIBinder) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#setDisplayService(_arg_ibinder))
                }
                fn r#clearDisplayService(&self) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#clearDisplayService())
                }
                fn r#waitDisplayService(&self) -> binder::Result<binder::SpIBinder> {
                  self._rt.block_on(self._inner.r#waitDisplayService())
                }
                fn r#createTapInterface(&self, _arg_ifaceNameSuffix: &str) -> binder::Result<binder::ParcelFileDescriptor> {
                  self._rt.block_on(self._inner.r#createTapInterface(_arg_ifaceNameSuffix))
                }
                fn r#deleteTapInterface(&self, _arg_tapFd: &binder::ParcelFileDescriptor) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#deleteTapInterface(_arg_tapFd))
                }
                fn try_as_async_server(&self) -> Option<&(dyn IVirtualizationServiceInternalAsyncServer + Send + Sync)> {
                  Some(&self._inner)
                }
              }
              let wrapped = Wrapper { _inner: inner, _rt: rt };
              Self::new_binder(wrapped, features)
            }
            pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn IVirtualizationServiceInternalAsync<P>>> {
              struct Wrapper {
                _native: binder::binder_impl::Binder<BnVirtualizationServiceInternal>
              }
              impl binder::Interface for Wrapper {}
              impl<P: binder::BinderAsyncPool> IVirtualizationServiceInternalAsync<P> for Wrapper {
                fn r#removeMemlockRlimit<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#removeMemlockRlimit())
                }
                fn r#allocateGlobalVmContext<'a>(&'a self, _arg_requesterDebugPid: i32) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_16_IGlobalVmContext>>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#allocateGlobalVmContext(_arg_requesterDebugPid))
                }
                fn r#atomVmBooted<'a>(&'a self, _arg_atom: &'a crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_AtomVmBooted) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#atomVmBooted(_arg_atom))
                }
                fn r#atomVmCreationRequested<'a>(&'a self, _arg_atom: &'a crate::mangled::_7_android_6_system_30_virtualizationservice_internal_23_AtomVmCreationRequested) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#atomVmCreationRequested(_arg_atom))
                }
                fn r#atomVmExited<'a>(&'a self, _arg_atom: &'a crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_AtomVmExited) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#atomVmExited(_arg_atom))
                }
                fn r#debugListVms<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineDebugInfo>>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#debugListVms())
                }
                fn r#requestAttestation<'a>(&'a self, _arg_csr: &'a [u8], _arg_requesterUid: i32, _arg_testMode: bool) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_6_system_20_virtualizationcommon_11_Certificate>>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#requestAttestation(_arg_csr, _arg_requesterUid, _arg_testMode))
                }
                fn r#enableTestAttestation<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#enableTestAttestation())
                }
                fn r#isRemoteAttestationSupported<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<bool>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#isRemoteAttestationSupported())
                }
                fn r#getAssignableDevices<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_16_AssignableDevice>>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#getAssignableDevices())
                }
                fn r#bindDevicesToVfioDriver<'a>(&'a self, _arg_devices: &'a [String]) -> binder::BoxFuture<'a, binder::Result<Vec<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IBoundDevice>>>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#bindDevicesToVfioDriver(_arg_devices))
                }
                fn r#getDtboFile<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<binder::ParcelFileDescriptor>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#getDtboFile())
                }
                fn r#allocateInstanceId<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<[u8; 64]>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#allocateInstanceId())
                }
                fn r#removeVmInstance<'a>(&'a self, _arg_instanceId: &'a [u8; 64]) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#removeVmInstance(_arg_instanceId))
                }
                fn r#claimVmInstance<'a>(&'a self, _arg_instanceId: &'a [u8; 64]) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#claimVmInstance(_arg_instanceId))
                }
                fn r#setDisplayService<'a>(&'a self, _arg_ibinder: &'a binder::SpIBinder) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#setDisplayService(_arg_ibinder))
                }
                fn r#clearDisplayService<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#clearDisplayService())
                }
                fn r#waitDisplayService<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<binder::SpIBinder>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#waitDisplayService())
                }
                fn r#createTapInterface<'a>(&'a self, _arg_ifaceNameSuffix: &'a str) -> binder::BoxFuture<'a, binder::Result<binder::ParcelFileDescriptor>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#createTapInterface(_arg_ifaceNameSuffix))
                }
                fn r#deleteTapInterface<'a>(&'a self, _arg_tapFd: &'a binder::ParcelFileDescriptor) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#deleteTapInterface(_arg_tapFd))
                }
              }
              if _native.try_as_async_server().is_some() {
                Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn IVirtualizationServiceInternalAsync<P>>))
              } else {
                None
              }
            }
          }
          pub trait IVirtualizationServiceInternalDefault: Send + Sync {
            fn r#removeMemlockRlimit(&self) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#allocateGlobalVmContext(&self, _arg_requesterDebugPid: i32) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_16_IGlobalVmContext>> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#atomVmBooted(&self, _arg_atom: &crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_AtomVmBooted) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#atomVmCreationRequested(&self, _arg_atom: &crate::mangled::_7_android_6_system_30_virtualizationservice_internal_23_AtomVmCreationRequested) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#atomVmExited(&self, _arg_atom: &crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_AtomVmExited) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#debugListVms(&self) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineDebugInfo>> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#requestAttestation(&self, _arg_csr: &[u8], _arg_requesterUid: i32, _arg_testMode: bool) -> binder::Result<Vec<crate::mangled::_7_android_6_system_20_virtualizationcommon_11_Certificate>> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#enableTestAttestation(&self) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#isRemoteAttestationSupported(&self) -> binder::Result<bool> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#getAssignableDevices(&self) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_16_AssignableDevice>> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#bindDevicesToVfioDriver(&self, _arg_devices: &[String]) -> binder::Result<Vec<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IBoundDevice>>> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#getDtboFile(&self) -> binder::Result<binder::ParcelFileDescriptor> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#allocateInstanceId(&self) -> binder::Result<[u8; 64]> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#removeVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#claimVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#setDisplayService(&self, _arg_ibinder: &binder::SpIBinder) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#clearDisplayService(&self) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#waitDisplayService(&self) -> binder::Result<binder::SpIBinder> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#createTapInterface(&self, _arg_ifaceNameSuffix: &str) -> binder::Result<binder::ParcelFileDescriptor> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#deleteTapInterface(&self, _arg_tapFd: &binder::ParcelFileDescriptor) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
          }
          pub mod transactions {
            pub const r#removeMemlockRlimit: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
            pub const r#allocateGlobalVmContext: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
            pub const r#atomVmBooted: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
            pub const r#atomVmCreationRequested: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
            pub const r#atomVmExited: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
            pub const r#debugListVms: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
            pub const r#requestAttestation: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
            pub const r#enableTestAttestation: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
            pub const r#isRemoteAttestationSupported: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
            pub const r#getAssignableDevices: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
            pub const r#bindDevicesToVfioDriver: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
            pub const r#getDtboFile: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 11;
            pub const r#allocateInstanceId: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 12;
            pub const r#removeVmInstance: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 13;
            pub const r#claimVmInstance: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 14;
            pub const r#setDisplayService: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 15;
            pub const r#clearDisplayService: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16;
            pub const r#waitDisplayService: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 17;
            pub const r#createTapInterface: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 18;
            pub const r#deleteTapInterface: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 19;
          }
          pub type IVirtualizationServiceInternalDefaultRef = Option<std::sync::Arc<dyn IVirtualizationServiceInternalDefault>>;
          static DEFAULT_IMPL: std::sync::Mutex<IVirtualizationServiceInternalDefaultRef> = std::sync::Mutex::new(None);
          impl BpVirtualizationServiceInternal {
            fn build_parcel_removeMemlockRlimit(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_removeMemlockRlimit(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationServiceInternal>::getDefaultImpl() {
                  return _aidl_default_impl.r#removeMemlockRlimit();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_allocateGlobalVmContext(&self, _arg_requesterDebugPid: i32) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(&_arg_requesterDebugPid)?;
              Ok(aidl_data)
            }
            fn read_response_allocateGlobalVmContext(&self, _arg_requesterDebugPid: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_16_IGlobalVmContext>> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationServiceInternal>::getDefaultImpl() {
                  return _aidl_default_impl.r#allocateGlobalVmContext(_arg_requesterDebugPid);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_16_IGlobalVmContext> = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_atomVmBooted(&self, _arg_atom: &crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_AtomVmBooted) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_atom)?;
              Ok(aidl_data)
            }
            fn read_response_atomVmBooted(&self, _arg_atom: &crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_AtomVmBooted, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationServiceInternal>::getDefaultImpl() {
                  return _aidl_default_impl.r#atomVmBooted(_arg_atom);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_atomVmCreationRequested(&self, _arg_atom: &crate::mangled::_7_android_6_system_30_virtualizationservice_internal_23_AtomVmCreationRequested) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_atom)?;
              Ok(aidl_data)
            }
            fn read_response_atomVmCreationRequested(&self, _arg_atom: &crate::mangled::_7_android_6_system_30_virtualizationservice_internal_23_AtomVmCreationRequested, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationServiceInternal>::getDefaultImpl() {
                  return _aidl_default_impl.r#atomVmCreationRequested(_arg_atom);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_atomVmExited(&self, _arg_atom: &crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_AtomVmExited) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_atom)?;
              Ok(aidl_data)
            }
            fn read_response_atomVmExited(&self, _arg_atom: &crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_AtomVmExited, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationServiceInternal>::getDefaultImpl() {
                  return _aidl_default_impl.r#atomVmExited(_arg_atom);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_debugListVms(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_debugListVms(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineDebugInfo>> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationServiceInternal>::getDefaultImpl() {
                  return _aidl_default_impl.r#debugListVms();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineDebugInfo> = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_requestAttestation(&self, _arg_csr: &[u8], _arg_requesterUid: i32, _arg_testMode: bool) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_csr)?;
              aidl_data.write(&_arg_requesterUid)?;
              aidl_data.write(&_arg_testMode)?;
              Ok(aidl_data)
            }
            fn read_response_requestAttestation(&self, _arg_csr: &[u8], _arg_requesterUid: i32, _arg_testMode: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_6_system_20_virtualizationcommon_11_Certificate>> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationServiceInternal>::getDefaultImpl() {
                  return _aidl_default_impl.r#requestAttestation(_arg_csr, _arg_requesterUid, _arg_testMode);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: Vec<crate::mangled::_7_android_6_system_20_virtualizationcommon_11_Certificate> = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_enableTestAttestation(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_enableTestAttestation(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationServiceInternal>::getDefaultImpl() {
                  return _aidl_default_impl.r#enableTestAttestation();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_isRemoteAttestationSupported(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_isRemoteAttestationSupported(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<bool> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationServiceInternal>::getDefaultImpl() {
                  return _aidl_default_impl.r#isRemoteAttestationSupported();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: bool = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_getAssignableDevices(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_getAssignableDevices(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_16_AssignableDevice>> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationServiceInternal>::getDefaultImpl() {
                  return _aidl_default_impl.r#getAssignableDevices();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_16_AssignableDevice> = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_bindDevicesToVfioDriver(&self, _arg_devices: &[String]) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_devices)?;
              Ok(aidl_data)
            }
            fn read_response_bindDevicesToVfioDriver(&self, _arg_devices: &[String], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IBoundDevice>>> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationServiceInternal>::getDefaultImpl() {
                  return _aidl_default_impl.r#bindDevicesToVfioDriver(_arg_devices);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: Vec<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IBoundDevice>> = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_getDtboFile(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_getDtboFile(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<binder::ParcelFileDescriptor> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationServiceInternal>::getDefaultImpl() {
                  return _aidl_default_impl.r#getDtboFile();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: binder::ParcelFileDescriptor = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_allocateInstanceId(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_allocateInstanceId(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<[u8; 64]> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationServiceInternal>::getDefaultImpl() {
                  return _aidl_default_impl.r#allocateInstanceId();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: [u8; 64] = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_removeVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_instanceId)?;
              Ok(aidl_data)
            }
            fn read_response_removeVmInstance(&self, _arg_instanceId: &[u8; 64], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationServiceInternal>::getDefaultImpl() {
                  return _aidl_default_impl.r#removeVmInstance(_arg_instanceId);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_claimVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_instanceId)?;
              Ok(aidl_data)
            }
            fn read_response_claimVmInstance(&self, _arg_instanceId: &[u8; 64], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationServiceInternal>::getDefaultImpl() {
                  return _aidl_default_impl.r#claimVmInstance(_arg_instanceId);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_setDisplayService(&self, _arg_ibinder: &binder::SpIBinder) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_ibinder)?;
              Ok(aidl_data)
            }
            fn read_response_setDisplayService(&self, _arg_ibinder: &binder::SpIBinder, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationServiceInternal>::getDefaultImpl() {
                  return _aidl_default_impl.r#setDisplayService(_arg_ibinder);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_clearDisplayService(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_clearDisplayService(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationServiceInternal>::getDefaultImpl() {
                  return _aidl_default_impl.r#clearDisplayService();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_waitDisplayService(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_waitDisplayService(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<binder::SpIBinder> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationServiceInternal>::getDefaultImpl() {
                  return _aidl_default_impl.r#waitDisplayService();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: binder::SpIBinder = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_createTapInterface(&self, _arg_ifaceNameSuffix: &str) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_ifaceNameSuffix)?;
              Ok(aidl_data)
            }
            fn read_response_createTapInterface(&self, _arg_ifaceNameSuffix: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<binder::ParcelFileDescriptor> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationServiceInternal>::getDefaultImpl() {
                  return _aidl_default_impl.r#createTapInterface(_arg_ifaceNameSuffix);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: binder::ParcelFileDescriptor = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_deleteTapInterface(&self, _arg_tapFd: &binder::ParcelFileDescriptor) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_tapFd)?;
              Ok(aidl_data)
            }
            fn read_response_deleteTapInterface(&self, _arg_tapFd: &binder::ParcelFileDescriptor, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationServiceInternal>::getDefaultImpl() {
                  return _aidl_default_impl.r#deleteTapInterface(_arg_tapFd);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
          }
          impl IVirtualizationServiceInternal for BpVirtualizationServiceInternal {
            fn r#removeMemlockRlimit(&self) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_removeMemlockRlimit()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#removeMemlockRlimit, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_removeMemlockRlimit(_aidl_reply)
            }
            fn r#allocateGlobalVmContext(&self, _arg_requesterDebugPid: i32) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_16_IGlobalVmContext>> {
              let _aidl_data = self.build_parcel_allocateGlobalVmContext(_arg_requesterDebugPid)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#allocateGlobalVmContext, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_allocateGlobalVmContext(_arg_requesterDebugPid, _aidl_reply)
            }
            fn r#atomVmBooted(&self, _arg_atom: &crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_AtomVmBooted) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_atomVmBooted(_arg_atom)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#atomVmBooted, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_atomVmBooted(_arg_atom, _aidl_reply)
            }
            fn r#atomVmCreationRequested(&self, _arg_atom: &crate::mangled::_7_android_6_system_30_virtualizationservice_internal_23_AtomVmCreationRequested) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_atomVmCreationRequested(_arg_atom)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#atomVmCreationRequested, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_atomVmCreationRequested(_arg_atom, _aidl_reply)
            }
            fn r#atomVmExited(&self, _arg_atom: &crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_AtomVmExited) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_atomVmExited(_arg_atom)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#atomVmExited, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_atomVmExited(_arg_atom, _aidl_reply)
            }
            fn r#debugListVms(&self) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineDebugInfo>> {
              let _aidl_data = self.build_parcel_debugListVms()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#debugListVms, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_debugListVms(_aidl_reply)
            }
            fn r#requestAttestation(&self, _arg_csr: &[u8], _arg_requesterUid: i32, _arg_testMode: bool) -> binder::Result<Vec<crate::mangled::_7_android_6_system_20_virtualizationcommon_11_Certificate>> {
              let _aidl_data = self.build_parcel_requestAttestation(_arg_csr, _arg_requesterUid, _arg_testMode)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#requestAttestation, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_requestAttestation(_arg_csr, _arg_requesterUid, _arg_testMode, _aidl_reply)
            }
            fn r#enableTestAttestation(&self) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_enableTestAttestation()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#enableTestAttestation, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_enableTestAttestation(_aidl_reply)
            }
            fn r#isRemoteAttestationSupported(&self) -> binder::Result<bool> {
              let _aidl_data = self.build_parcel_isRemoteAttestationSupported()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#isRemoteAttestationSupported, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_isRemoteAttestationSupported(_aidl_reply)
            }
            fn r#getAssignableDevices(&self) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_16_AssignableDevice>> {
              let _aidl_data = self.build_parcel_getAssignableDevices()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#getAssignableDevices, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_getAssignableDevices(_aidl_reply)
            }
            fn r#bindDevicesToVfioDriver(&self, _arg_devices: &[String]) -> binder::Result<Vec<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IBoundDevice>>> {
              let _aidl_data = self.build_parcel_bindDevicesToVfioDriver(_arg_devices)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#bindDevicesToVfioDriver, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_bindDevicesToVfioDriver(_arg_devices, _aidl_reply)
            }
            fn r#getDtboFile(&self) -> binder::Result<binder::ParcelFileDescriptor> {
              let _aidl_data = self.build_parcel_getDtboFile()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#getDtboFile, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_getDtboFile(_aidl_reply)
            }
            fn r#allocateInstanceId(&self) -> binder::Result<[u8; 64]> {
              let _aidl_data = self.build_parcel_allocateInstanceId()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#allocateInstanceId, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_allocateInstanceId(_aidl_reply)
            }
            fn r#removeVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_removeVmInstance(_arg_instanceId)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#removeVmInstance, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_removeVmInstance(_arg_instanceId, _aidl_reply)
            }
            fn r#claimVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_claimVmInstance(_arg_instanceId)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#claimVmInstance, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_claimVmInstance(_arg_instanceId, _aidl_reply)
            }
            fn r#setDisplayService(&self, _arg_ibinder: &binder::SpIBinder) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_setDisplayService(_arg_ibinder)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#setDisplayService, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_setDisplayService(_arg_ibinder, _aidl_reply)
            }
            fn r#clearDisplayService(&self) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_clearDisplayService()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#clearDisplayService, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_clearDisplayService(_aidl_reply)
            }
            fn r#waitDisplayService(&self) -> binder::Result<binder::SpIBinder> {
              let _aidl_data = self.build_parcel_waitDisplayService()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#waitDisplayService, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_waitDisplayService(_aidl_reply)
            }
            fn r#createTapInterface(&self, _arg_ifaceNameSuffix: &str) -> binder::Result<binder::ParcelFileDescriptor> {
              let _aidl_data = self.build_parcel_createTapInterface(_arg_ifaceNameSuffix)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#createTapInterface, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_createTapInterface(_arg_ifaceNameSuffix, _aidl_reply)
            }
            fn r#deleteTapInterface(&self, _arg_tapFd: &binder::ParcelFileDescriptor) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_deleteTapInterface(_arg_tapFd)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#deleteTapInterface, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_deleteTapInterface(_arg_tapFd, _aidl_reply)
            }
          }
          impl<P: binder::BinderAsyncPool> IVirtualizationServiceInternalAsync<P> for BpVirtualizationServiceInternal {
            fn r#removeMemlockRlimit<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_removeMemlockRlimit() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#removeMemlockRlimit, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_removeMemlockRlimit(_aidl_reply)
                }
              )
            }
            fn r#allocateGlobalVmContext<'a>(&'a self, _arg_requesterDebugPid: i32) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_16_IGlobalVmContext>>> {
              let _aidl_data = match self.build_parcel_allocateGlobalVmContext(_arg_requesterDebugPid) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#allocateGlobalVmContext, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_allocateGlobalVmContext(_arg_requesterDebugPid, _aidl_reply)
                }
              )
            }
            fn r#atomVmBooted<'a>(&'a self, _arg_atom: &'a crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_AtomVmBooted) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_atomVmBooted(_arg_atom) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#atomVmBooted, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_atomVmBooted(_arg_atom, _aidl_reply)
                }
              )
            }
            fn r#atomVmCreationRequested<'a>(&'a self, _arg_atom: &'a crate::mangled::_7_android_6_system_30_virtualizationservice_internal_23_AtomVmCreationRequested) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_atomVmCreationRequested(_arg_atom) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#atomVmCreationRequested, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_atomVmCreationRequested(_arg_atom, _aidl_reply)
                }
              )
            }
            fn r#atomVmExited<'a>(&'a self, _arg_atom: &'a crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_AtomVmExited) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_atomVmExited(_arg_atom) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#atomVmExited, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_atomVmExited(_arg_atom, _aidl_reply)
                }
              )
            }
            fn r#debugListVms<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineDebugInfo>>> {
              let _aidl_data = match self.build_parcel_debugListVms() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#debugListVms, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_debugListVms(_aidl_reply)
                }
              )
            }
            fn r#requestAttestation<'a>(&'a self, _arg_csr: &'a [u8], _arg_requesterUid: i32, _arg_testMode: bool) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_6_system_20_virtualizationcommon_11_Certificate>>> {
              let _aidl_data = match self.build_parcel_requestAttestation(_arg_csr, _arg_requesterUid, _arg_testMode) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#requestAttestation, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_requestAttestation(_arg_csr, _arg_requesterUid, _arg_testMode, _aidl_reply)
                }
              )
            }
            fn r#enableTestAttestation<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_enableTestAttestation() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#enableTestAttestation, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_enableTestAttestation(_aidl_reply)
                }
              )
            }
            fn r#isRemoteAttestationSupported<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<bool>> {
              let _aidl_data = match self.build_parcel_isRemoteAttestationSupported() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#isRemoteAttestationSupported, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_isRemoteAttestationSupported(_aidl_reply)
                }
              )
            }
            fn r#getAssignableDevices<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_16_AssignableDevice>>> {
              let _aidl_data = match self.build_parcel_getAssignableDevices() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#getAssignableDevices, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_getAssignableDevices(_aidl_reply)
                }
              )
            }
            fn r#bindDevicesToVfioDriver<'a>(&'a self, _arg_devices: &'a [String]) -> binder::BoxFuture<'a, binder::Result<Vec<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IBoundDevice>>>> {
              let _aidl_data = match self.build_parcel_bindDevicesToVfioDriver(_arg_devices) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#bindDevicesToVfioDriver, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_bindDevicesToVfioDriver(_arg_devices, _aidl_reply)
                }
              )
            }
            fn r#getDtboFile<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<binder::ParcelFileDescriptor>> {
              let _aidl_data = match self.build_parcel_getDtboFile() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#getDtboFile, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_getDtboFile(_aidl_reply)
                }
              )
            }
            fn r#allocateInstanceId<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<[u8; 64]>> {
              let _aidl_data = match self.build_parcel_allocateInstanceId() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#allocateInstanceId, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_allocateInstanceId(_aidl_reply)
                }
              )
            }
            fn r#removeVmInstance<'a>(&'a self, _arg_instanceId: &'a [u8; 64]) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_removeVmInstance(_arg_instanceId) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#removeVmInstance, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_removeVmInstance(_arg_instanceId, _aidl_reply)
                }
              )
            }
            fn r#claimVmInstance<'a>(&'a self, _arg_instanceId: &'a [u8; 64]) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_claimVmInstance(_arg_instanceId) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#claimVmInstance, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_claimVmInstance(_arg_instanceId, _aidl_reply)
                }
              )
            }
            fn r#setDisplayService<'a>(&'a self, _arg_ibinder: &'a binder::SpIBinder) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_setDisplayService(_arg_ibinder) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#setDisplayService, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_setDisplayService(_arg_ibinder, _aidl_reply)
                }
              )
            }
            fn r#clearDisplayService<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_clearDisplayService() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#clearDisplayService, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_clearDisplayService(_aidl_reply)
                }
              )
            }
            fn r#waitDisplayService<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<binder::SpIBinder>> {
              let _aidl_data = match self.build_parcel_waitDisplayService() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#waitDisplayService, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_waitDisplayService(_aidl_reply)
                }
              )
            }
            fn r#createTapInterface<'a>(&'a self, _arg_ifaceNameSuffix: &'a str) -> binder::BoxFuture<'a, binder::Result<binder::ParcelFileDescriptor>> {
              let _aidl_data = match self.build_parcel_createTapInterface(_arg_ifaceNameSuffix) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#createTapInterface, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_createTapInterface(_arg_ifaceNameSuffix, _aidl_reply)
                }
              )
            }
            fn r#deleteTapInterface<'a>(&'a self, _arg_tapFd: &'a binder::ParcelFileDescriptor) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_deleteTapInterface(_arg_tapFd) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#deleteTapInterface, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_deleteTapInterface(_arg_tapFd, _aidl_reply)
                }
              )
            }
          }
          impl IVirtualizationServiceInternal for binder::binder_impl::Binder<BnVirtualizationServiceInternal> {
            fn r#removeMemlockRlimit(&self) -> binder::Result<()> { self.0.r#removeMemlockRlimit() }
            fn r#allocateGlobalVmContext(&self, _arg_requesterDebugPid: i32) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_16_IGlobalVmContext>> { self.0.r#allocateGlobalVmContext(_arg_requesterDebugPid) }
            fn r#atomVmBooted(&self, _arg_atom: &crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_AtomVmBooted) -> binder::Result<()> { self.0.r#atomVmBooted(_arg_atom) }
            fn r#atomVmCreationRequested(&self, _arg_atom: &crate::mangled::_7_android_6_system_30_virtualizationservice_internal_23_AtomVmCreationRequested) -> binder::Result<()> { self.0.r#atomVmCreationRequested(_arg_atom) }
            fn r#atomVmExited(&self, _arg_atom: &crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_AtomVmExited) -> binder::Result<()> { self.0.r#atomVmExited(_arg_atom) }
            fn r#debugListVms(&self) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineDebugInfo>> { self.0.r#debugListVms() }
            fn r#requestAttestation(&self, _arg_csr: &[u8], _arg_requesterUid: i32, _arg_testMode: bool) -> binder::Result<Vec<crate::mangled::_7_android_6_system_20_virtualizationcommon_11_Certificate>> { self.0.r#requestAttestation(_arg_csr, _arg_requesterUid, _arg_testMode) }
            fn r#enableTestAttestation(&self) -> binder::Result<()> { self.0.r#enableTestAttestation() }
            fn r#isRemoteAttestationSupported(&self) -> binder::Result<bool> { self.0.r#isRemoteAttestationSupported() }
            fn r#getAssignableDevices(&self) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_16_AssignableDevice>> { self.0.r#getAssignableDevices() }
            fn r#bindDevicesToVfioDriver(&self, _arg_devices: &[String]) -> binder::Result<Vec<binder::Strong<dyn crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_IBoundDevice>>> { self.0.r#bindDevicesToVfioDriver(_arg_devices) }
            fn r#getDtboFile(&self) -> binder::Result<binder::ParcelFileDescriptor> { self.0.r#getDtboFile() }
            fn r#allocateInstanceId(&self) -> binder::Result<[u8; 64]> { self.0.r#allocateInstanceId() }
            fn r#removeVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<()> { self.0.r#removeVmInstance(_arg_instanceId) }
            fn r#claimVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<()> { self.0.r#claimVmInstance(_arg_instanceId) }
            fn r#setDisplayService(&self, _arg_ibinder: &binder::SpIBinder) -> binder::Result<()> { self.0.r#setDisplayService(_arg_ibinder) }
            fn r#clearDisplayService(&self) -> binder::Result<()> { self.0.r#clearDisplayService() }
            fn r#waitDisplayService(&self) -> binder::Result<binder::SpIBinder> { self.0.r#waitDisplayService() }
            fn r#createTapInterface(&self, _arg_ifaceNameSuffix: &str) -> binder::Result<binder::ParcelFileDescriptor> { self.0.r#createTapInterface(_arg_ifaceNameSuffix) }
            fn r#deleteTapInterface(&self, _arg_tapFd: &binder::ParcelFileDescriptor) -> binder::Result<()> { self.0.r#deleteTapInterface(_arg_tapFd) }
          }
          fn on_transact(_aidl_service: &dyn IVirtualizationServiceInternal, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
            match _aidl_code {
              transactions::r#removeMemlockRlimit => {
                let _aidl_return = _aidl_service.r#removeMemlockRlimit();
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#allocateGlobalVmContext => {
                let _arg_requesterDebugPid: i32 = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#allocateGlobalVmContext(_arg_requesterDebugPid);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#atomVmBooted => {
                let _arg_atom: crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_AtomVmBooted = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#atomVmBooted(&_arg_atom);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#atomVmCreationRequested => {
                let _arg_atom: crate::mangled::_7_android_6_system_30_virtualizationservice_internal_23_AtomVmCreationRequested = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#atomVmCreationRequested(&_arg_atom);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#atomVmExited => {
                let _arg_atom: crate::mangled::_7_android_6_system_30_virtualizationservice_internal_12_AtomVmExited = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#atomVmExited(&_arg_atom);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#debugListVms => {
                let _aidl_return = _aidl_service.r#debugListVms();
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#requestAttestation => {
                let _arg_csr: Vec<u8> = _aidl_data.read()?;
                let _arg_requesterUid: i32 = _aidl_data.read()?;
                let _arg_testMode: bool = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#requestAttestation(&_arg_csr, _arg_requesterUid, _arg_testMode);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#enableTestAttestation => {
                let _aidl_return = _aidl_service.r#enableTestAttestation();
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#isRemoteAttestationSupported => {
                let _aidl_return = _aidl_service.r#isRemoteAttestationSupported();
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#getAssignableDevices => {
                let _aidl_return = _aidl_service.r#getAssignableDevices();
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#bindDevicesToVfioDriver => {
                let _arg_devices: Vec<String> = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#bindDevicesToVfioDriver(&_arg_devices);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#getDtboFile => {
                let _aidl_return = _aidl_service.r#getDtboFile();
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#allocateInstanceId => {
                let _aidl_return = _aidl_service.r#allocateInstanceId();
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#removeVmInstance => {
                let _arg_instanceId: [u8; 64] = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#removeVmInstance(&_arg_instanceId);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#claimVmInstance => {
                let _arg_instanceId: [u8; 64] = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#claimVmInstance(&_arg_instanceId);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#setDisplayService => {
                let _arg_ibinder: binder::SpIBinder = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#setDisplayService(&_arg_ibinder);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#clearDisplayService => {
                let _aidl_return = _aidl_service.r#clearDisplayService();
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#waitDisplayService => {
                let _aidl_return = _aidl_service.r#waitDisplayService();
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#createTapInterface => {
                let _arg_ifaceNameSuffix: String = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#createTapInterface(&_arg_ifaceNameSuffix);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#deleteTapInterface => {
                let _arg_tapFd: binder::ParcelFileDescriptor = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#deleteTapInterface(&_arg_tapFd);
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
           pub use super::r#IVirtualizationServiceInternal as _7_android_6_system_30_virtualizationservice_internal_30_IVirtualizationServiceInternal;
          }
        }
        pub mod IVmnic {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice_internal-rust-source/gen/android/system/virtualizationservice_internal/IVmnic.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice_internal-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice_internal/IVmnic.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #![allow(non_upper_case_globals)]
          #![allow(non_snake_case)]
          #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
          use binder::declare_binder_interface;
          declare_binder_interface! {
            IVmnic["android.system.virtualizationservice_internal.IVmnic"] {
              native: BnVmnic(on_transact),
              proxy: BpVmnic {
              },
              async: IVmnicAsync(try_into_local_async),
            }
          }
          pub trait IVmnic: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationservice_internal.IVmnic" }
            fn r#createTapInterface(&self, _arg_ifaceNameSuffix: &str) -> binder::Result<binder::ParcelFileDescriptor>;
            fn r#deleteTapInterface(&self, _arg_tapFd: &binder::ParcelFileDescriptor) -> binder::Result<()>;
            fn getDefaultImpl() -> IVmnicDefaultRef where Self: Sized {
              DEFAULT_IMPL.lock().unwrap().clone()
            }
            fn setDefaultImpl(d: IVmnicDefaultRef) -> IVmnicDefaultRef where Self: Sized {
              std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
            }
            fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn IVmnicAsyncServer + Send + Sync)> {
              None
            }
          }
          pub trait IVmnicAsync<P>: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationservice_internal.IVmnic" }
            fn r#createTapInterface<'a>(&'a self, _arg_ifaceNameSuffix: &'a str) -> binder::BoxFuture<'a, binder::Result<binder::ParcelFileDescriptor>>;
            fn r#deleteTapInterface<'a>(&'a self, _arg_tapFd: &'a binder::ParcelFileDescriptor) -> binder::BoxFuture<'a, binder::Result<()>>;
          }
          #[::async_trait::async_trait]
          pub trait IVmnicAsyncServer: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationservice_internal.IVmnic" }
            async fn r#createTapInterface(&self, _arg_ifaceNameSuffix: &str) -> binder::Result<binder::ParcelFileDescriptor>;
            async fn r#deleteTapInterface(&self, _arg_tapFd: &binder::ParcelFileDescriptor) -> binder::Result<()>;
          }
          impl BnVmnic {
            /// Create a new async binder service.
            pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IVmnic>
            where
              T: IVmnicAsyncServer + binder::Interface + Send + Sync + 'static,
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
              impl<T, R> IVmnic for Wrapper<T, R>
              where
                T: IVmnicAsyncServer + Send + Sync + 'static,
                R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
              {
                fn r#createTapInterface(&self, _arg_ifaceNameSuffix: &str) -> binder::Result<binder::ParcelFileDescriptor> {
                  self._rt.block_on(self._inner.r#createTapInterface(_arg_ifaceNameSuffix))
                }
                fn r#deleteTapInterface(&self, _arg_tapFd: &binder::ParcelFileDescriptor) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#deleteTapInterface(_arg_tapFd))
                }
                fn try_as_async_server(&self) -> Option<&(dyn IVmnicAsyncServer + Send + Sync)> {
                  Some(&self._inner)
                }
              }
              let wrapped = Wrapper { _inner: inner, _rt: rt };
              Self::new_binder(wrapped, features)
            }
            pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn IVmnicAsync<P>>> {
              struct Wrapper {
                _native: binder::binder_impl::Binder<BnVmnic>
              }
              impl binder::Interface for Wrapper {}
              impl<P: binder::BinderAsyncPool> IVmnicAsync<P> for Wrapper {
                fn r#createTapInterface<'a>(&'a self, _arg_ifaceNameSuffix: &'a str) -> binder::BoxFuture<'a, binder::Result<binder::ParcelFileDescriptor>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#createTapInterface(_arg_ifaceNameSuffix))
                }
                fn r#deleteTapInterface<'a>(&'a self, _arg_tapFd: &'a binder::ParcelFileDescriptor) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#deleteTapInterface(_arg_tapFd))
                }
              }
              if _native.try_as_async_server().is_some() {
                Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn IVmnicAsync<P>>))
              } else {
                None
              }
            }
          }
          pub trait IVmnicDefault: Send + Sync {
            fn r#createTapInterface(&self, _arg_ifaceNameSuffix: &str) -> binder::Result<binder::ParcelFileDescriptor> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#deleteTapInterface(&self, _arg_tapFd: &binder::ParcelFileDescriptor) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
          }
          pub mod transactions {
            pub const r#createTapInterface: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
            pub const r#deleteTapInterface: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
          }
          pub type IVmnicDefaultRef = Option<std::sync::Arc<dyn IVmnicDefault>>;
          static DEFAULT_IMPL: std::sync::Mutex<IVmnicDefaultRef> = std::sync::Mutex::new(None);
          impl BpVmnic {
            fn build_parcel_createTapInterface(&self, _arg_ifaceNameSuffix: &str) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_ifaceNameSuffix)?;
              Ok(aidl_data)
            }
            fn read_response_createTapInterface(&self, _arg_ifaceNameSuffix: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<binder::ParcelFileDescriptor> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVmnic>::getDefaultImpl() {
                  return _aidl_default_impl.r#createTapInterface(_arg_ifaceNameSuffix);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: binder::ParcelFileDescriptor = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_deleteTapInterface(&self, _arg_tapFd: &binder::ParcelFileDescriptor) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_tapFd)?;
              Ok(aidl_data)
            }
            fn read_response_deleteTapInterface(&self, _arg_tapFd: &binder::ParcelFileDescriptor, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVmnic>::getDefaultImpl() {
                  return _aidl_default_impl.r#deleteTapInterface(_arg_tapFd);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
          }
          impl IVmnic for BpVmnic {
            fn r#createTapInterface(&self, _arg_ifaceNameSuffix: &str) -> binder::Result<binder::ParcelFileDescriptor> {
              let _aidl_data = self.build_parcel_createTapInterface(_arg_ifaceNameSuffix)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#createTapInterface, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_createTapInterface(_arg_ifaceNameSuffix, _aidl_reply)
            }
            fn r#deleteTapInterface(&self, _arg_tapFd: &binder::ParcelFileDescriptor) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_deleteTapInterface(_arg_tapFd)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#deleteTapInterface, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_deleteTapInterface(_arg_tapFd, _aidl_reply)
            }
          }
          impl<P: binder::BinderAsyncPool> IVmnicAsync<P> for BpVmnic {
            fn r#createTapInterface<'a>(&'a self, _arg_ifaceNameSuffix: &'a str) -> binder::BoxFuture<'a, binder::Result<binder::ParcelFileDescriptor>> {
              let _aidl_data = match self.build_parcel_createTapInterface(_arg_ifaceNameSuffix) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#createTapInterface, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_createTapInterface(_arg_ifaceNameSuffix, _aidl_reply)
                }
              )
            }
            fn r#deleteTapInterface<'a>(&'a self, _arg_tapFd: &'a binder::ParcelFileDescriptor) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_deleteTapInterface(_arg_tapFd) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#deleteTapInterface, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_deleteTapInterface(_arg_tapFd, _aidl_reply)
                }
              )
            }
          }
          impl IVmnic for binder::binder_impl::Binder<BnVmnic> {
            fn r#createTapInterface(&self, _arg_ifaceNameSuffix: &str) -> binder::Result<binder::ParcelFileDescriptor> { self.0.r#createTapInterface(_arg_ifaceNameSuffix) }
            fn r#deleteTapInterface(&self, _arg_tapFd: &binder::ParcelFileDescriptor) -> binder::Result<()> { self.0.r#deleteTapInterface(_arg_tapFd) }
          }
          fn on_transact(_aidl_service: &dyn IVmnic, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
            match _aidl_code {
              transactions::r#createTapInterface => {
                let _arg_ifaceNameSuffix: String = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#createTapInterface(&_arg_ifaceNameSuffix);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#deleteTapInterface => {
                let _arg_tapFd: binder::ParcelFileDescriptor = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#deleteTapInterface(&_arg_tapFd);
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
           pub use super::r#IVmnic as _7_android_6_system_30_virtualizationservice_internal_6_IVmnic;
          }
        }
      }
    }
  }
}
pub mod mangled {
  pub use super::aidl::android::system::virtualizationservice_internal::AtomVmBooted::mangled::*;
  pub use super::aidl::android::system::virtualizationservice_internal::AtomVmCreationRequested::mangled::*;
  pub use super::aidl::android::system::virtualizationservice_internal::AtomVmExited::mangled::*;
  pub use super::aidl::android::system::virtualizationservice_internal::IBoundDevice::mangled::*;
  pub use super::aidl::android::system::virtualizationservice_internal::IGlobalVmContext::mangled::*;
  pub use super::aidl::android::system::virtualizationservice_internal::IVfioHandler::mangled::*;
  pub use super::aidl::android::system::virtualizationservice_internal::IVirtualizationServiceInternal::mangled::*;
  pub use super::aidl::android::system::virtualizationservice_internal::IVmnic::mangled::*;
  pub(crate) use android_system_virtualizationcommon::mangled::*;
  pub(crate) use android_system_virtualizationservice::mangled::*;
}

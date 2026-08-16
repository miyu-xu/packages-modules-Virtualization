#![allow(non_snake_case)]
#![allow(missing_docs)]
#[deprecated(note = "Please access via libbinder_rs binder::")]
pub use binder;
pub mod aidl {
  pub mod android {
    pub mod system {
      pub mod virtualizationservice {
        pub mod AssignableDevice {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen/android/system/virtualizationservice/AssignableDevice.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice/AssignableDevice.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug, Clone)]
          pub struct r#AssignableDevice {
            pub r#node: String,
            pub r#dtbo_label: String,
          }
          impl Default for r#AssignableDevice {
            fn default() -> Self {
              Self {
                r#node: Default::default(),
                r#dtbo_label: Default::default(),
              }
            }
          }
          impl binder::Parcelable for r#AssignableDevice {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#node)?;
                subparcel.write(&self.r#dtbo_label)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#node = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#dtbo_label = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#AssignableDevice);
          binder::impl_deserialize_for_parcelable!(r#AssignableDevice);
          impl binder::binder_impl::ParcelableMetadata for r#AssignableDevice {
            fn get_descriptor() -> &'static str { "android.system.virtualizationservice.AssignableDevice" }
          }
          pub(crate) mod mangled {
           pub use super::r#AssignableDevice as _7_android_6_system_21_virtualizationservice_16_AssignableDevice;
          }
        }
        pub mod AudioConfig {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen/android/system/virtualizationservice/AudioConfig.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice/AudioConfig.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug)]
          pub struct r#AudioConfig {
            pub r#useMicrophone: bool,
            pub r#useSpeaker: bool,
          }
          impl Default for r#AudioConfig {
            fn default() -> Self {
              Self {
                r#useMicrophone: false,
                r#useSpeaker: false,
              }
            }
          }
          impl binder::Parcelable for r#AudioConfig {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#useMicrophone)?;
                subparcel.write(&self.r#useSpeaker)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#useMicrophone = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#useSpeaker = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#AudioConfig);
          binder::impl_deserialize_for_parcelable!(r#AudioConfig);
          impl binder::binder_impl::ParcelableMetadata for r#AudioConfig {
            fn get_descriptor() -> &'static str { "android.system.virtualizationservice.AudioConfig" }
          }
          pub(crate) mod mangled {
           pub use super::r#AudioConfig as _7_android_6_system_21_virtualizationservice_11_AudioConfig;
          }
        }
        pub mod CpuTopology {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen/android/system/virtualizationservice/CpuTopology.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice/CpuTopology.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #![allow(non_upper_case_globals)]
          use binder::declare_binder_enum;
          declare_binder_enum! {
            #[repr(C, align(1))]
            r#CpuTopology : [i8; 2] {
              r#ONE_CPU = 0,
              r#MATCH_HOST = 1,
            }
          }
          pub(crate) mod mangled {
           pub use super::r#CpuTopology as _7_android_6_system_21_virtualizationservice_11_CpuTopology;
          }
        }
        pub mod DiskImage {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen/android/system/virtualizationservice/DiskImage.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice/DiskImage.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug)]
          pub struct r#DiskImage {
            pub r#image: Option<binder::ParcelFileDescriptor>,
            pub r#writable: bool,
            pub r#partitions: Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_9_Partition>,
          }
          impl Default for r#DiskImage {
            fn default() -> Self {
              Self {
                r#image: Default::default(),
                r#writable: false,
                r#partitions: Default::default(),
              }
            }
          }
          impl binder::Parcelable for r#DiskImage {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#image)?;
                subparcel.write(&self.r#writable)?;
                subparcel.write(&self.r#partitions)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#image = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#writable = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#partitions = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#DiskImage);
          binder::impl_deserialize_for_parcelable!(r#DiskImage);
          impl binder::binder_impl::ParcelableMetadata for r#DiskImage {
            fn get_descriptor() -> &'static str { "android.system.virtualizationservice.DiskImage" }
          }
          pub(crate) mod mangled {
           pub use super::r#DiskImage as _7_android_6_system_21_virtualizationservice_9_DiskImage;
          }
        }
        pub mod DisplayConfig {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen/android/system/virtualizationservice/DisplayConfig.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice/DisplayConfig.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug)]
          pub struct r#DisplayConfig {
            pub r#width: i32,
            pub r#height: i32,
            pub r#horizontalDpi: i32,
            pub r#verticalDpi: i32,
            pub r#refreshRate: i32,
          }
          impl Default for r#DisplayConfig {
            fn default() -> Self {
              Self {
                r#width: 0,
                r#height: 0,
                r#horizontalDpi: 0,
                r#verticalDpi: 0,
                r#refreshRate: 0,
              }
            }
          }
          impl binder::Parcelable for r#DisplayConfig {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#width)?;
                subparcel.write(&self.r#height)?;
                subparcel.write(&self.r#horizontalDpi)?;
                subparcel.write(&self.r#verticalDpi)?;
                subparcel.write(&self.r#refreshRate)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#width = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#height = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#horizontalDpi = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#verticalDpi = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#refreshRate = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#DisplayConfig);
          binder::impl_deserialize_for_parcelable!(r#DisplayConfig);
          impl binder::binder_impl::ParcelableMetadata for r#DisplayConfig {
            fn get_descriptor() -> &'static str { "android.system.virtualizationservice.DisplayConfig" }
          }
          pub(crate) mod mangled {
           pub use super::r#DisplayConfig as _7_android_6_system_21_virtualizationservice_13_DisplayConfig;
          }
        }
        pub mod GpuConfig {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen/android/system/virtualizationservice/GpuConfig.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice/GpuConfig.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug)]
          pub struct r#GpuConfig {
            pub r#backend: Option<String>,
            pub r#contextTypes: Option<Vec<Option<String>>>,
            pub r#pciAddress: Option<String>,
            pub r#rendererFeatures: Option<String>,
            pub r#rendererUseEgl: bool,
            pub r#rendererUseGles: bool,
            pub r#rendererUseGlx: bool,
            pub r#rendererUseSurfaceless: bool,
            pub r#rendererUseVulkan: bool,
          }
          impl Default for r#GpuConfig {
            fn default() -> Self {
              Self {
                r#backend: Default::default(),
                r#contextTypes: Default::default(),
                r#pciAddress: Default::default(),
                r#rendererFeatures: Default::default(),
                r#rendererUseEgl: false,
                r#rendererUseGles: false,
                r#rendererUseGlx: false,
                r#rendererUseSurfaceless: false,
                r#rendererUseVulkan: false,
              }
            }
          }
          impl binder::Parcelable for r#GpuConfig {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#backend)?;
                subparcel.write(&self.r#contextTypes)?;
                subparcel.write(&self.r#pciAddress)?;
                subparcel.write(&self.r#rendererFeatures)?;
                subparcel.write(&self.r#rendererUseEgl)?;
                subparcel.write(&self.r#rendererUseGles)?;
                subparcel.write(&self.r#rendererUseGlx)?;
                subparcel.write(&self.r#rendererUseSurfaceless)?;
                subparcel.write(&self.r#rendererUseVulkan)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#backend = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#contextTypes = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#pciAddress = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#rendererFeatures = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#rendererUseEgl = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#rendererUseGles = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#rendererUseGlx = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#rendererUseSurfaceless = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#rendererUseVulkan = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#GpuConfig);
          binder::impl_deserialize_for_parcelable!(r#GpuConfig);
          impl binder::binder_impl::ParcelableMetadata for r#GpuConfig {
            fn get_descriptor() -> &'static str { "android.system.virtualizationservice.GpuConfig" }
          }
          pub(crate) mod mangled {
           pub use super::r#GpuConfig as _7_android_6_system_21_virtualizationservice_9_GpuConfig;
          }
        }
        pub mod IVirtualMachine {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen/android/system/virtualizationservice/IVirtualMachine.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice/IVirtualMachine.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #![allow(non_upper_case_globals)]
          #![allow(non_snake_case)]
          #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
          use binder::declare_binder_interface;
          declare_binder_interface! {
            IVirtualMachine["android.system.virtualizationservice.IVirtualMachine"] {
              native: BnVirtualMachine(on_transact),
              proxy: BpVirtualMachine {
              },
              async: IVirtualMachineAsync(try_into_local_async),
            }
          }
          pub trait IVirtualMachine: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationservice.IVirtualMachine" }
            fn r#getCid(&self) -> binder::Result<i32>;
            fn r#getState(&self) -> binder::Result<crate::mangled::_7_android_6_system_21_virtualizationservice_19_VirtualMachineState>;
            fn r#registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_6_system_21_virtualizationservice_23_IVirtualMachineCallback>) -> binder::Result<()>;
            fn r#start(&self) -> binder::Result<()>;
            fn r#stop(&self) -> binder::Result<()>;
            fn r#getMemoryBalloon(&self) -> binder::Result<i64>;
            fn r#setMemoryBalloon(&self, _arg_num_bytes: i64) -> binder::Result<()>;
            fn r#connectVsock(&self, _arg_port: i32) -> binder::Result<binder::ParcelFileDescriptor>;
            fn r#setHostConsoleName(&self, _arg_pathname: &str) -> binder::Result<()>;
            fn r#suspend(&self) -> binder::Result<()>;
            fn r#resume(&self) -> binder::Result<()>;
            fn r#startHostVsockTcpBridge(&self, _arg_hostPort: i32, _arg_guestPort: i32) -> binder::Result<()>;
            fn getDefaultImpl() -> IVirtualMachineDefaultRef where Self: Sized {
              DEFAULT_IMPL.lock().unwrap().clone()
            }
            fn setDefaultImpl(d: IVirtualMachineDefaultRef) -> IVirtualMachineDefaultRef where Self: Sized {
              std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
            }
            fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn IVirtualMachineAsyncServer + Send + Sync)> {
              None
            }
          }
          pub trait IVirtualMachineAsync<P>: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationservice.IVirtualMachine" }
            fn r#getCid<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>>;
            fn r#getState<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_6_system_21_virtualizationservice_19_VirtualMachineState>>;
            fn r#registerCallback<'a>(&'a self, _arg_callback: &'a binder::Strong<dyn crate::mangled::_7_android_6_system_21_virtualizationservice_23_IVirtualMachineCallback>) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#start<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#stop<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#getMemoryBalloon<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i64>>;
            fn r#setMemoryBalloon<'a>(&'a self, _arg_num_bytes: i64) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#connectVsock<'a>(&'a self, _arg_port: i32) -> binder::BoxFuture<'a, binder::Result<binder::ParcelFileDescriptor>>;
            fn r#setHostConsoleName<'a>(&'a self, _arg_pathname: &'a str) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#suspend<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#resume<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#startHostVsockTcpBridge<'a>(&'a self, _arg_hostPort: i32, _arg_guestPort: i32) -> binder::BoxFuture<'a, binder::Result<()>>;
          }
          #[::async_trait::async_trait]
          pub trait IVirtualMachineAsyncServer: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationservice.IVirtualMachine" }
            async fn r#getCid(&self) -> binder::Result<i32>;
            async fn r#getState(&self) -> binder::Result<crate::mangled::_7_android_6_system_21_virtualizationservice_19_VirtualMachineState>;
            async fn r#registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_6_system_21_virtualizationservice_23_IVirtualMachineCallback>) -> binder::Result<()>;
            async fn r#start(&self) -> binder::Result<()>;
            async fn r#stop(&self) -> binder::Result<()>;
            async fn r#getMemoryBalloon(&self) -> binder::Result<i64>;
            async fn r#setMemoryBalloon(&self, _arg_num_bytes: i64) -> binder::Result<()>;
            async fn r#connectVsock(&self, _arg_port: i32) -> binder::Result<binder::ParcelFileDescriptor>;
            async fn r#setHostConsoleName(&self, _arg_pathname: &str) -> binder::Result<()>;
            async fn r#suspend(&self) -> binder::Result<()>;
            async fn r#resume(&self) -> binder::Result<()>;
            async fn r#startHostVsockTcpBridge(&self, _arg_hostPort: i32, _arg_guestPort: i32) -> binder::Result<()>;
          }
          impl BnVirtualMachine {
            /// Create a new async binder service.
            pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IVirtualMachine>
            where
              T: IVirtualMachineAsyncServer + binder::Interface + Send + Sync + 'static,
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
              impl<T, R> IVirtualMachine for Wrapper<T, R>
              where
                T: IVirtualMachineAsyncServer + Send + Sync + 'static,
                R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
              {
                fn r#getCid(&self) -> binder::Result<i32> {
                  self._rt.block_on(self._inner.r#getCid())
                }
                fn r#getState(&self) -> binder::Result<crate::mangled::_7_android_6_system_21_virtualizationservice_19_VirtualMachineState> {
                  self._rt.block_on(self._inner.r#getState())
                }
                fn r#registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_6_system_21_virtualizationservice_23_IVirtualMachineCallback>) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#registerCallback(_arg_callback))
                }
                fn r#start(&self) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#start())
                }
                fn r#stop(&self) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#stop())
                }
                fn r#getMemoryBalloon(&self) -> binder::Result<i64> {
                  self._rt.block_on(self._inner.r#getMemoryBalloon())
                }
                fn r#setMemoryBalloon(&self, _arg_num_bytes: i64) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#setMemoryBalloon(_arg_num_bytes))
                }
                fn r#connectVsock(&self, _arg_port: i32) -> binder::Result<binder::ParcelFileDescriptor> {
                  self._rt.block_on(self._inner.r#connectVsock(_arg_port))
                }
                fn r#setHostConsoleName(&self, _arg_pathname: &str) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#setHostConsoleName(_arg_pathname))
                }
                fn r#suspend(&self) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#suspend())
                }
                fn r#resume(&self) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#resume())
                }
                fn r#startHostVsockTcpBridge(&self, _arg_hostPort: i32, _arg_guestPort: i32) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#startHostVsockTcpBridge(_arg_hostPort, _arg_guestPort))
                }
                fn try_as_async_server(&self) -> Option<&(dyn IVirtualMachineAsyncServer + Send + Sync)> {
                  Some(&self._inner)
                }
              }
              let wrapped = Wrapper { _inner: inner, _rt: rt };
              Self::new_binder(wrapped, features)
            }
            pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn IVirtualMachineAsync<P>>> {
              struct Wrapper {
                _native: binder::binder_impl::Binder<BnVirtualMachine>
              }
              impl binder::Interface for Wrapper {}
              impl<P: binder::BinderAsyncPool> IVirtualMachineAsync<P> for Wrapper {
                fn r#getCid<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#getCid())
                }
                fn r#getState<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_6_system_21_virtualizationservice_19_VirtualMachineState>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#getState())
                }
                fn r#registerCallback<'a>(&'a self, _arg_callback: &'a binder::Strong<dyn crate::mangled::_7_android_6_system_21_virtualizationservice_23_IVirtualMachineCallback>) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#registerCallback(_arg_callback))
                }
                fn r#start<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#start())
                }
                fn r#stop<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#stop())
                }
                fn r#getMemoryBalloon<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i64>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#getMemoryBalloon())
                }
                fn r#setMemoryBalloon<'a>(&'a self, _arg_num_bytes: i64) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#setMemoryBalloon(_arg_num_bytes))
                }
                fn r#connectVsock<'a>(&'a self, _arg_port: i32) -> binder::BoxFuture<'a, binder::Result<binder::ParcelFileDescriptor>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#connectVsock(_arg_port))
                }
                fn r#setHostConsoleName<'a>(&'a self, _arg_pathname: &'a str) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#setHostConsoleName(_arg_pathname))
                }
                fn r#suspend<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#suspend())
                }
                fn r#resume<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#resume())
                }
                fn r#startHostVsockTcpBridge<'a>(&'a self, _arg_hostPort: i32, _arg_guestPort: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#startHostVsockTcpBridge(_arg_hostPort, _arg_guestPort))
                }
              }
              if _native.try_as_async_server().is_some() {
                Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn IVirtualMachineAsync<P>>))
              } else {
                None
              }
            }
          }
          pub trait IVirtualMachineDefault: Send + Sync {
            fn r#getCid(&self) -> binder::Result<i32> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#getState(&self) -> binder::Result<crate::mangled::_7_android_6_system_21_virtualizationservice_19_VirtualMachineState> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_6_system_21_virtualizationservice_23_IVirtualMachineCallback>) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#start(&self) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#stop(&self) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#getMemoryBalloon(&self) -> binder::Result<i64> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#setMemoryBalloon(&self, _arg_num_bytes: i64) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#connectVsock(&self, _arg_port: i32) -> binder::Result<binder::ParcelFileDescriptor> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#setHostConsoleName(&self, _arg_pathname: &str) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#suspend(&self) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#resume(&self) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#startHostVsockTcpBridge(&self, _arg_hostPort: i32, _arg_guestPort: i32) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
          }
          pub mod transactions {
            pub const r#getCid: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
            pub const r#getState: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
            pub const r#registerCallback: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
            pub const r#start: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
            pub const r#stop: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
            pub const r#getMemoryBalloon: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
            pub const r#setMemoryBalloon: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
            pub const r#connectVsock: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
            pub const r#setHostConsoleName: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
            pub const r#suspend: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
            pub const r#resume: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
            pub const r#startHostVsockTcpBridge: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 11;
          }
          pub type IVirtualMachineDefaultRef = Option<std::sync::Arc<dyn IVirtualMachineDefault>>;
          static DEFAULT_IMPL: std::sync::Mutex<IVirtualMachineDefaultRef> = std::sync::Mutex::new(None);
          impl BpVirtualMachine {
            fn build_parcel_getCid(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_getCid(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualMachine>::getDefaultImpl() {
                  return _aidl_default_impl.r#getCid();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: i32 = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_getState(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_getState(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_6_system_21_virtualizationservice_19_VirtualMachineState> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualMachine>::getDefaultImpl() {
                  return _aidl_default_impl.r#getState();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: crate::mangled::_7_android_6_system_21_virtualizationservice_19_VirtualMachineState = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_6_system_21_virtualizationservice_23_IVirtualMachineCallback>) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_callback)?;
              Ok(aidl_data)
            }
            fn read_response_registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_6_system_21_virtualizationservice_23_IVirtualMachineCallback>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualMachine>::getDefaultImpl() {
                  return _aidl_default_impl.r#registerCallback(_arg_callback);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_start(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_start(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualMachine>::getDefaultImpl() {
                  return _aidl_default_impl.r#start();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_stop(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_stop(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualMachine>::getDefaultImpl() {
                  return _aidl_default_impl.r#stop();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_getMemoryBalloon(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_getMemoryBalloon(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i64> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualMachine>::getDefaultImpl() {
                  return _aidl_default_impl.r#getMemoryBalloon();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: i64 = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_setMemoryBalloon(&self, _arg_num_bytes: i64) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(&_arg_num_bytes)?;
              Ok(aidl_data)
            }
            fn read_response_setMemoryBalloon(&self, _arg_num_bytes: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualMachine>::getDefaultImpl() {
                  return _aidl_default_impl.r#setMemoryBalloon(_arg_num_bytes);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_connectVsock(&self, _arg_port: i32) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(&_arg_port)?;
              Ok(aidl_data)
            }
            fn read_response_connectVsock(&self, _arg_port: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<binder::ParcelFileDescriptor> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualMachine>::getDefaultImpl() {
                  return _aidl_default_impl.r#connectVsock(_arg_port);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: binder::ParcelFileDescriptor = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_setHostConsoleName(&self, _arg_pathname: &str) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_pathname)?;
              Ok(aidl_data)
            }
            fn read_response_setHostConsoleName(&self, _arg_pathname: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualMachine>::getDefaultImpl() {
                  return _aidl_default_impl.r#setHostConsoleName(_arg_pathname);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_suspend(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_suspend(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualMachine>::getDefaultImpl() {
                  return _aidl_default_impl.r#suspend();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_resume(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_resume(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualMachine>::getDefaultImpl() {
                  return _aidl_default_impl.r#resume();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_startHostVsockTcpBridge(&self, _arg_hostPort: i32, _arg_guestPort: i32) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(&_arg_hostPort)?;
              aidl_data.write(&_arg_guestPort)?;
              Ok(aidl_data)
            }
            fn read_response_startHostVsockTcpBridge(&self, _arg_hostPort: i32, _arg_guestPort: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualMachine>::getDefaultImpl() {
                  return _aidl_default_impl.r#startHostVsockTcpBridge(_arg_hostPort, _arg_guestPort);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
          }
          impl IVirtualMachine for BpVirtualMachine {
            fn r#getCid(&self) -> binder::Result<i32> {
              let _aidl_data = self.build_parcel_getCid()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#getCid, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_getCid(_aidl_reply)
            }
            fn r#getState(&self) -> binder::Result<crate::mangled::_7_android_6_system_21_virtualizationservice_19_VirtualMachineState> {
              let _aidl_data = self.build_parcel_getState()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#getState, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_getState(_aidl_reply)
            }
            fn r#registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_6_system_21_virtualizationservice_23_IVirtualMachineCallback>) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_registerCallback(_arg_callback)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#registerCallback, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_registerCallback(_arg_callback, _aidl_reply)
            }
            fn r#start(&self) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_start()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#start, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_start(_aidl_reply)
            }
            fn r#stop(&self) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_stop()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#stop, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_stop(_aidl_reply)
            }
            fn r#getMemoryBalloon(&self) -> binder::Result<i64> {
              let _aidl_data = self.build_parcel_getMemoryBalloon()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#getMemoryBalloon, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_getMemoryBalloon(_aidl_reply)
            }
            fn r#setMemoryBalloon(&self, _arg_num_bytes: i64) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_setMemoryBalloon(_arg_num_bytes)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#setMemoryBalloon, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_setMemoryBalloon(_arg_num_bytes, _aidl_reply)
            }
            fn r#connectVsock(&self, _arg_port: i32) -> binder::Result<binder::ParcelFileDescriptor> {
              let _aidl_data = self.build_parcel_connectVsock(_arg_port)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#connectVsock, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_connectVsock(_arg_port, _aidl_reply)
            }
            fn r#setHostConsoleName(&self, _arg_pathname: &str) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_setHostConsoleName(_arg_pathname)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#setHostConsoleName, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_setHostConsoleName(_arg_pathname, _aidl_reply)
            }
            fn r#suspend(&self) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_suspend()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#suspend, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_suspend(_aidl_reply)
            }
            fn r#resume(&self) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_resume()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#resume, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_resume(_aidl_reply)
            }
            fn r#startHostVsockTcpBridge(&self, _arg_hostPort: i32, _arg_guestPort: i32) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_startHostVsockTcpBridge(_arg_hostPort, _arg_guestPort)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#startHostVsockTcpBridge, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_startHostVsockTcpBridge(_arg_hostPort, _arg_guestPort, _aidl_reply)
            }
          }
          impl<P: binder::BinderAsyncPool> IVirtualMachineAsync<P> for BpVirtualMachine {
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
            fn r#getState<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_6_system_21_virtualizationservice_19_VirtualMachineState>> {
              let _aidl_data = match self.build_parcel_getState() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#getState, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_getState(_aidl_reply)
                }
              )
            }
            fn r#registerCallback<'a>(&'a self, _arg_callback: &'a binder::Strong<dyn crate::mangled::_7_android_6_system_21_virtualizationservice_23_IVirtualMachineCallback>) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_registerCallback(_arg_callback) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#registerCallback, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_registerCallback(_arg_callback, _aidl_reply)
                }
              )
            }
            fn r#start<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_start() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#start, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_start(_aidl_reply)
                }
              )
            }
            fn r#stop<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_stop() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#stop, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_stop(_aidl_reply)
                }
              )
            }
            fn r#getMemoryBalloon<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i64>> {
              let _aidl_data = match self.build_parcel_getMemoryBalloon() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#getMemoryBalloon, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_getMemoryBalloon(_aidl_reply)
                }
              )
            }
            fn r#setMemoryBalloon<'a>(&'a self, _arg_num_bytes: i64) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_setMemoryBalloon(_arg_num_bytes) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#setMemoryBalloon, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_setMemoryBalloon(_arg_num_bytes, _aidl_reply)
                }
              )
            }
            fn r#connectVsock<'a>(&'a self, _arg_port: i32) -> binder::BoxFuture<'a, binder::Result<binder::ParcelFileDescriptor>> {
              let _aidl_data = match self.build_parcel_connectVsock(_arg_port) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#connectVsock, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_connectVsock(_arg_port, _aidl_reply)
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
            fn r#suspend<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_suspend() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#suspend, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_suspend(_aidl_reply)
                }
              )
            }
            fn r#resume<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_resume() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#resume, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_resume(_aidl_reply)
                }
              )
            }
            fn r#startHostVsockTcpBridge<'a>(&'a self, _arg_hostPort: i32, _arg_guestPort: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_startHostVsockTcpBridge(_arg_hostPort, _arg_guestPort) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#startHostVsockTcpBridge, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_startHostVsockTcpBridge(_arg_hostPort, _arg_guestPort, _aidl_reply)
                }
              )
            }
          }
          impl IVirtualMachine for binder::binder_impl::Binder<BnVirtualMachine> {
            fn r#getCid(&self) -> binder::Result<i32> { self.0.r#getCid() }
            fn r#getState(&self) -> binder::Result<crate::mangled::_7_android_6_system_21_virtualizationservice_19_VirtualMachineState> { self.0.r#getState() }
            fn r#registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_6_system_21_virtualizationservice_23_IVirtualMachineCallback>) -> binder::Result<()> { self.0.r#registerCallback(_arg_callback) }
            fn r#start(&self) -> binder::Result<()> { self.0.r#start() }
            fn r#stop(&self) -> binder::Result<()> { self.0.r#stop() }
            fn r#getMemoryBalloon(&self) -> binder::Result<i64> { self.0.r#getMemoryBalloon() }
            fn r#setMemoryBalloon(&self, _arg_num_bytes: i64) -> binder::Result<()> { self.0.r#setMemoryBalloon(_arg_num_bytes) }
            fn r#connectVsock(&self, _arg_port: i32) -> binder::Result<binder::ParcelFileDescriptor> { self.0.r#connectVsock(_arg_port) }
            fn r#setHostConsoleName(&self, _arg_pathname: &str) -> binder::Result<()> { self.0.r#setHostConsoleName(_arg_pathname) }
            fn r#suspend(&self) -> binder::Result<()> { self.0.r#suspend() }
            fn r#resume(&self) -> binder::Result<()> { self.0.r#resume() }
            fn r#startHostVsockTcpBridge(&self, _arg_hostPort: i32, _arg_guestPort: i32) -> binder::Result<()> { self.0.r#startHostVsockTcpBridge(_arg_hostPort, _arg_guestPort) }
          }
          fn on_transact(_aidl_service: &dyn IVirtualMachine, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
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
              transactions::r#getState => {
                let _aidl_return = _aidl_service.r#getState();
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#registerCallback => {
                let _arg_callback: binder::Strong<dyn crate::mangled::_7_android_6_system_21_virtualizationservice_23_IVirtualMachineCallback> = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#registerCallback(&_arg_callback);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#start => {
                let _aidl_return = _aidl_service.r#start();
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#stop => {
                let _aidl_return = _aidl_service.r#stop();
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#getMemoryBalloon => {
                let _aidl_return = _aidl_service.r#getMemoryBalloon();
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#setMemoryBalloon => {
                let _arg_num_bytes: i64 = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#setMemoryBalloon(_arg_num_bytes);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#connectVsock => {
                let _arg_port: i32 = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#connectVsock(_arg_port);
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
              transactions::r#suspend => {
                let _aidl_return = _aidl_service.r#suspend();
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#resume => {
                let _aidl_return = _aidl_service.r#resume();
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#startHostVsockTcpBridge => {
                let _arg_hostPort: i32 = _aidl_data.read()?;
                let _arg_guestPort: i32 = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#startHostVsockTcpBridge(_arg_hostPort, _arg_guestPort);
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
           pub use super::r#IVirtualMachine as _7_android_6_system_21_virtualizationservice_15_IVirtualMachine;
          }
        }
        pub mod IVirtualMachineCallback {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen/android/system/virtualizationservice/IVirtualMachineCallback.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice/IVirtualMachineCallback.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #![allow(non_upper_case_globals)]
          #![allow(non_snake_case)]
          #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
          use binder::declare_binder_interface;
          declare_binder_interface! {
            IVirtualMachineCallback["android.system.virtualizationservice.IVirtualMachineCallback"] {
              native: BnVirtualMachineCallback(on_transact),
              proxy: BpVirtualMachineCallback {
              },
              async: IVirtualMachineCallbackAsync(try_into_local_async),
            }
          }
          pub trait IVirtualMachineCallback: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationservice.IVirtualMachineCallback" }
            fn r#onPayloadStarted(&self, _arg_cid: i32) -> binder::Result<()>;
            fn r#onPayloadReady(&self, _arg_cid: i32) -> binder::Result<()>;
            fn r#onPayloadFinished(&self, _arg_cid: i32, _arg_exitCode: i32) -> binder::Result<()>;
            fn r#onError(&self, _arg_cid: i32, _arg_errorCode: crate::mangled::_7_android_6_system_20_virtualizationcommon_9_ErrorCode, _arg_message: &str) -> binder::Result<()>;
            fn r#onDied(&self, _arg_cid: i32, _arg_reason: crate::mangled::_7_android_6_system_20_virtualizationcommon_11_DeathReason) -> binder::Result<()>;
            fn getDefaultImpl() -> IVirtualMachineCallbackDefaultRef where Self: Sized {
              DEFAULT_IMPL.lock().unwrap().clone()
            }
            fn setDefaultImpl(d: IVirtualMachineCallbackDefaultRef) -> IVirtualMachineCallbackDefaultRef where Self: Sized {
              std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
            }
            fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn IVirtualMachineCallbackAsyncServer + Send + Sync)> {
              None
            }
          }
          pub trait IVirtualMachineCallbackAsync<P>: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationservice.IVirtualMachineCallback" }
            fn r#onPayloadStarted<'a>(&'a self, _arg_cid: i32) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#onPayloadReady<'a>(&'a self, _arg_cid: i32) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#onPayloadFinished<'a>(&'a self, _arg_cid: i32, _arg_exitCode: i32) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#onError<'a>(&'a self, _arg_cid: i32, _arg_errorCode: crate::mangled::_7_android_6_system_20_virtualizationcommon_9_ErrorCode, _arg_message: &'a str) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#onDied<'a>(&'a self, _arg_cid: i32, _arg_reason: crate::mangled::_7_android_6_system_20_virtualizationcommon_11_DeathReason) -> binder::BoxFuture<'a, binder::Result<()>>;
          }
          #[::async_trait::async_trait]
          pub trait IVirtualMachineCallbackAsyncServer: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationservice.IVirtualMachineCallback" }
            async fn r#onPayloadStarted(&self, _arg_cid: i32) -> binder::Result<()>;
            async fn r#onPayloadReady(&self, _arg_cid: i32) -> binder::Result<()>;
            async fn r#onPayloadFinished(&self, _arg_cid: i32, _arg_exitCode: i32) -> binder::Result<()>;
            async fn r#onError(&self, _arg_cid: i32, _arg_errorCode: crate::mangled::_7_android_6_system_20_virtualizationcommon_9_ErrorCode, _arg_message: &str) -> binder::Result<()>;
            async fn r#onDied(&self, _arg_cid: i32, _arg_reason: crate::mangled::_7_android_6_system_20_virtualizationcommon_11_DeathReason) -> binder::Result<()>;
          }
          impl BnVirtualMachineCallback {
            /// Create a new async binder service.
            pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IVirtualMachineCallback>
            where
              T: IVirtualMachineCallbackAsyncServer + binder::Interface + Send + Sync + 'static,
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
              impl<T, R> IVirtualMachineCallback for Wrapper<T, R>
              where
                T: IVirtualMachineCallbackAsyncServer + Send + Sync + 'static,
                R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
              {
                fn r#onPayloadStarted(&self, _arg_cid: i32) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#onPayloadStarted(_arg_cid))
                }
                fn r#onPayloadReady(&self, _arg_cid: i32) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#onPayloadReady(_arg_cid))
                }
                fn r#onPayloadFinished(&self, _arg_cid: i32, _arg_exitCode: i32) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#onPayloadFinished(_arg_cid, _arg_exitCode))
                }
                fn r#onError(&self, _arg_cid: i32, _arg_errorCode: crate::mangled::_7_android_6_system_20_virtualizationcommon_9_ErrorCode, _arg_message: &str) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#onError(_arg_cid, _arg_errorCode, _arg_message))
                }
                fn r#onDied(&self, _arg_cid: i32, _arg_reason: crate::mangled::_7_android_6_system_20_virtualizationcommon_11_DeathReason) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#onDied(_arg_cid, _arg_reason))
                }
                fn try_as_async_server(&self) -> Option<&(dyn IVirtualMachineCallbackAsyncServer + Send + Sync)> {
                  Some(&self._inner)
                }
              }
              let wrapped = Wrapper { _inner: inner, _rt: rt };
              Self::new_binder(wrapped, features)
            }
            pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn IVirtualMachineCallbackAsync<P>>> {
              struct Wrapper {
                _native: binder::binder_impl::Binder<BnVirtualMachineCallback>
              }
              impl binder::Interface for Wrapper {}
              impl<P: binder::BinderAsyncPool> IVirtualMachineCallbackAsync<P> for Wrapper {
                fn r#onPayloadStarted<'a>(&'a self, _arg_cid: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#onPayloadStarted(_arg_cid))
                }
                fn r#onPayloadReady<'a>(&'a self, _arg_cid: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#onPayloadReady(_arg_cid))
                }
                fn r#onPayloadFinished<'a>(&'a self, _arg_cid: i32, _arg_exitCode: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#onPayloadFinished(_arg_cid, _arg_exitCode))
                }
                fn r#onError<'a>(&'a self, _arg_cid: i32, _arg_errorCode: crate::mangled::_7_android_6_system_20_virtualizationcommon_9_ErrorCode, _arg_message: &'a str) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#onError(_arg_cid, _arg_errorCode, _arg_message))
                }
                fn r#onDied<'a>(&'a self, _arg_cid: i32, _arg_reason: crate::mangled::_7_android_6_system_20_virtualizationcommon_11_DeathReason) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#onDied(_arg_cid, _arg_reason))
                }
              }
              if _native.try_as_async_server().is_some() {
                Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn IVirtualMachineCallbackAsync<P>>))
              } else {
                None
              }
            }
          }
          pub trait IVirtualMachineCallbackDefault: Send + Sync {
            fn r#onPayloadStarted(&self, _arg_cid: i32) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#onPayloadReady(&self, _arg_cid: i32) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#onPayloadFinished(&self, _arg_cid: i32, _arg_exitCode: i32) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#onError(&self, _arg_cid: i32, _arg_errorCode: crate::mangled::_7_android_6_system_20_virtualizationcommon_9_ErrorCode, _arg_message: &str) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#onDied(&self, _arg_cid: i32, _arg_reason: crate::mangled::_7_android_6_system_20_virtualizationcommon_11_DeathReason) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
          }
          pub mod transactions {
            pub const r#onPayloadStarted: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
            pub const r#onPayloadReady: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
            pub const r#onPayloadFinished: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
            pub const r#onError: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
            pub const r#onDied: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
          }
          pub type IVirtualMachineCallbackDefaultRef = Option<std::sync::Arc<dyn IVirtualMachineCallbackDefault>>;
          static DEFAULT_IMPL: std::sync::Mutex<IVirtualMachineCallbackDefaultRef> = std::sync::Mutex::new(None);
          impl BpVirtualMachineCallback {
            fn build_parcel_onPayloadStarted(&self, _arg_cid: i32) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(&_arg_cid)?;
              Ok(aidl_data)
            }
            fn read_response_onPayloadStarted(&self, _arg_cid: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualMachineCallback>::getDefaultImpl() {
                  return _aidl_default_impl.r#onPayloadStarted(_arg_cid);
                }
              }
              let _aidl_reply = _aidl_reply?;
              Ok(())
            }
            fn build_parcel_onPayloadReady(&self, _arg_cid: i32) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(&_arg_cid)?;
              Ok(aidl_data)
            }
            fn read_response_onPayloadReady(&self, _arg_cid: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualMachineCallback>::getDefaultImpl() {
                  return _aidl_default_impl.r#onPayloadReady(_arg_cid);
                }
              }
              let _aidl_reply = _aidl_reply?;
              Ok(())
            }
            fn build_parcel_onPayloadFinished(&self, _arg_cid: i32, _arg_exitCode: i32) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(&_arg_cid)?;
              aidl_data.write(&_arg_exitCode)?;
              Ok(aidl_data)
            }
            fn read_response_onPayloadFinished(&self, _arg_cid: i32, _arg_exitCode: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualMachineCallback>::getDefaultImpl() {
                  return _aidl_default_impl.r#onPayloadFinished(_arg_cid, _arg_exitCode);
                }
              }
              let _aidl_reply = _aidl_reply?;
              Ok(())
            }
            fn build_parcel_onError(&self, _arg_cid: i32, _arg_errorCode: crate::mangled::_7_android_6_system_20_virtualizationcommon_9_ErrorCode, _arg_message: &str) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(&_arg_cid)?;
              aidl_data.write(&_arg_errorCode)?;
              aidl_data.write(_arg_message)?;
              Ok(aidl_data)
            }
            fn read_response_onError(&self, _arg_cid: i32, _arg_errorCode: crate::mangled::_7_android_6_system_20_virtualizationcommon_9_ErrorCode, _arg_message: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualMachineCallback>::getDefaultImpl() {
                  return _aidl_default_impl.r#onError(_arg_cid, _arg_errorCode, _arg_message);
                }
              }
              let _aidl_reply = _aidl_reply?;
              Ok(())
            }
            fn build_parcel_onDied(&self, _arg_cid: i32, _arg_reason: crate::mangled::_7_android_6_system_20_virtualizationcommon_11_DeathReason) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(&_arg_cid)?;
              aidl_data.write(&_arg_reason)?;
              Ok(aidl_data)
            }
            fn read_response_onDied(&self, _arg_cid: i32, _arg_reason: crate::mangled::_7_android_6_system_20_virtualizationcommon_11_DeathReason, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualMachineCallback>::getDefaultImpl() {
                  return _aidl_default_impl.r#onDied(_arg_cid, _arg_reason);
                }
              }
              let _aidl_reply = _aidl_reply?;
              Ok(())
            }
          }
          impl IVirtualMachineCallback for BpVirtualMachineCallback {
            fn r#onPayloadStarted(&self, _arg_cid: i32) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_onPayloadStarted(_arg_cid)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#onPayloadStarted, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_onPayloadStarted(_arg_cid, _aidl_reply)
            }
            fn r#onPayloadReady(&self, _arg_cid: i32) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_onPayloadReady(_arg_cid)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#onPayloadReady, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_onPayloadReady(_arg_cid, _aidl_reply)
            }
            fn r#onPayloadFinished(&self, _arg_cid: i32, _arg_exitCode: i32) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_onPayloadFinished(_arg_cid, _arg_exitCode)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#onPayloadFinished, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_onPayloadFinished(_arg_cid, _arg_exitCode, _aidl_reply)
            }
            fn r#onError(&self, _arg_cid: i32, _arg_errorCode: crate::mangled::_7_android_6_system_20_virtualizationcommon_9_ErrorCode, _arg_message: &str) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_onError(_arg_cid, _arg_errorCode, _arg_message)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#onError, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_onError(_arg_cid, _arg_errorCode, _arg_message, _aidl_reply)
            }
            fn r#onDied(&self, _arg_cid: i32, _arg_reason: crate::mangled::_7_android_6_system_20_virtualizationcommon_11_DeathReason) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_onDied(_arg_cid, _arg_reason)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#onDied, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_onDied(_arg_cid, _arg_reason, _aidl_reply)
            }
          }
          impl<P: binder::BinderAsyncPool> IVirtualMachineCallbackAsync<P> for BpVirtualMachineCallback {
            fn r#onPayloadStarted<'a>(&'a self, _arg_cid: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_onPayloadStarted(_arg_cid) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#onPayloadStarted, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_onPayloadStarted(_arg_cid, _aidl_reply)
                }
              )
            }
            fn r#onPayloadReady<'a>(&'a self, _arg_cid: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_onPayloadReady(_arg_cid) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#onPayloadReady, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_onPayloadReady(_arg_cid, _aidl_reply)
                }
              )
            }
            fn r#onPayloadFinished<'a>(&'a self, _arg_cid: i32, _arg_exitCode: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_onPayloadFinished(_arg_cid, _arg_exitCode) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#onPayloadFinished, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_onPayloadFinished(_arg_cid, _arg_exitCode, _aidl_reply)
                }
              )
            }
            fn r#onError<'a>(&'a self, _arg_cid: i32, _arg_errorCode: crate::mangled::_7_android_6_system_20_virtualizationcommon_9_ErrorCode, _arg_message: &'a str) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_onError(_arg_cid, _arg_errorCode, _arg_message) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#onError, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_onError(_arg_cid, _arg_errorCode, _arg_message, _aidl_reply)
                }
              )
            }
            fn r#onDied<'a>(&'a self, _arg_cid: i32, _arg_reason: crate::mangled::_7_android_6_system_20_virtualizationcommon_11_DeathReason) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_onDied(_arg_cid, _arg_reason) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#onDied, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_onDied(_arg_cid, _arg_reason, _aidl_reply)
                }
              )
            }
          }
          impl IVirtualMachineCallback for binder::binder_impl::Binder<BnVirtualMachineCallback> {
            fn r#onPayloadStarted(&self, _arg_cid: i32) -> binder::Result<()> { self.0.r#onPayloadStarted(_arg_cid) }
            fn r#onPayloadReady(&self, _arg_cid: i32) -> binder::Result<()> { self.0.r#onPayloadReady(_arg_cid) }
            fn r#onPayloadFinished(&self, _arg_cid: i32, _arg_exitCode: i32) -> binder::Result<()> { self.0.r#onPayloadFinished(_arg_cid, _arg_exitCode) }
            fn r#onError(&self, _arg_cid: i32, _arg_errorCode: crate::mangled::_7_android_6_system_20_virtualizationcommon_9_ErrorCode, _arg_message: &str) -> binder::Result<()> { self.0.r#onError(_arg_cid, _arg_errorCode, _arg_message) }
            fn r#onDied(&self, _arg_cid: i32, _arg_reason: crate::mangled::_7_android_6_system_20_virtualizationcommon_11_DeathReason) -> binder::Result<()> { self.0.r#onDied(_arg_cid, _arg_reason) }
          }
          fn on_transact(_aidl_service: &dyn IVirtualMachineCallback, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
            match _aidl_code {
              transactions::r#onPayloadStarted => {
                let _arg_cid: i32 = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#onPayloadStarted(_arg_cid);
                Ok(())
              }
              transactions::r#onPayloadReady => {
                let _arg_cid: i32 = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#onPayloadReady(_arg_cid);
                Ok(())
              }
              transactions::r#onPayloadFinished => {
                let _arg_cid: i32 = _aidl_data.read()?;
                let _arg_exitCode: i32 = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#onPayloadFinished(_arg_cid, _arg_exitCode);
                Ok(())
              }
              transactions::r#onError => {
                let _arg_cid: i32 = _aidl_data.read()?;
                let _arg_errorCode: crate::mangled::_7_android_6_system_20_virtualizationcommon_9_ErrorCode = _aidl_data.read()?;
                let _arg_message: String = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#onError(_arg_cid, _arg_errorCode, &_arg_message);
                Ok(())
              }
              transactions::r#onDied => {
                let _arg_cid: i32 = _aidl_data.read()?;
                let _arg_reason: crate::mangled::_7_android_6_system_20_virtualizationcommon_11_DeathReason = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#onDied(_arg_cid, _arg_reason);
                Ok(())
              }
              _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
            }
          }
          pub(crate) mod mangled {
           pub use super::r#IVirtualMachineCallback as _7_android_6_system_21_virtualizationservice_23_IVirtualMachineCallback;
          }
        }
        pub mod IVirtualizationService {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen/android/system/virtualizationservice/IVirtualizationService.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice/IVirtualizationService.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #![allow(non_upper_case_globals)]
          #![allow(non_snake_case)]
          #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
          use binder::declare_binder_interface;
          declare_binder_interface! {
            IVirtualizationService["android.system.virtualizationservice.IVirtualizationService"] {
              native: BnVirtualizationService(on_transact),
              proxy: BpVirtualizationService {
              },
              async: IVirtualizationServiceAsync(try_into_local_async),
            }
          }
          pub trait IVirtualizationService: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationservice.IVirtualizationService" }
            fn r#createVm(&self, _arg_config: &crate::mangled::_7_android_6_system_21_virtualizationservice_20_VirtualMachineConfig, _arg_consoleOutFd: Option<&binder::ParcelFileDescriptor>, _arg_consoleInFd: Option<&binder::ParcelFileDescriptor>, _arg_osLogFd: Option<&binder::ParcelFileDescriptor>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_21_virtualizationservice_15_IVirtualMachine>>;
            fn r#allocateInstanceId(&self) -> binder::Result<[u8; 64]>;
            fn r#initializeWritablePartition(&self, _arg_imageFd: &binder::ParcelFileDescriptor, _arg_sizeBytes: i64, _arg_type: crate::mangled::_7_android_6_system_21_virtualizationservice_13_PartitionType) -> binder::Result<()>;
            fn r#createOrUpdateIdsigFile(&self, _arg_inputFd: &binder::ParcelFileDescriptor, _arg_idsigFd: &binder::ParcelFileDescriptor) -> binder::Result<()>;
            fn r#debugListVms(&self) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineDebugInfo>>;
            fn r#getAssignableDevices(&self) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_16_AssignableDevice>>;
            fn r#getSupportedOSList(&self) -> binder::Result<Vec<String>>;
            fn r#isFeatureEnabled(&self, _arg_feature: &str) -> binder::Result<bool>;
            fn r#enableTestAttestation(&self) -> binder::Result<()>;
            fn r#isRemoteAttestationSupported(&self) -> binder::Result<bool>;
            fn r#isUpdatableVmSupported(&self) -> binder::Result<bool>;
            fn r#removeVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<()>;
            fn r#claimVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<()>;
            fn getDefaultImpl() -> IVirtualizationServiceDefaultRef where Self: Sized {
              DEFAULT_IMPL.lock().unwrap().clone()
            }
            fn setDefaultImpl(d: IVirtualizationServiceDefaultRef) -> IVirtualizationServiceDefaultRef where Self: Sized {
              std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
            }
            fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn IVirtualizationServiceAsyncServer + Send + Sync)> {
              None
            }
          }
          pub trait IVirtualizationServiceAsync<P>: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationservice.IVirtualizationService" }
            fn r#createVm<'a>(&'a self, _arg_config: &'a crate::mangled::_7_android_6_system_21_virtualizationservice_20_VirtualMachineConfig, _arg_consoleOutFd: Option<&'a binder::ParcelFileDescriptor>, _arg_consoleInFd: Option<&'a binder::ParcelFileDescriptor>, _arg_osLogFd: Option<&'a binder::ParcelFileDescriptor>) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_21_virtualizationservice_15_IVirtualMachine>>>;
            fn r#allocateInstanceId<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<[u8; 64]>>;
            fn r#initializeWritablePartition<'a>(&'a self, _arg_imageFd: &'a binder::ParcelFileDescriptor, _arg_sizeBytes: i64, _arg_type: crate::mangled::_7_android_6_system_21_virtualizationservice_13_PartitionType) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#createOrUpdateIdsigFile<'a>(&'a self, _arg_inputFd: &'a binder::ParcelFileDescriptor, _arg_idsigFd: &'a binder::ParcelFileDescriptor) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#debugListVms<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineDebugInfo>>>;
            fn r#getAssignableDevices<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_16_AssignableDevice>>>;
            fn r#getSupportedOSList<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<String>>>;
            fn r#isFeatureEnabled<'a>(&'a self, _arg_feature: &'a str) -> binder::BoxFuture<'a, binder::Result<bool>>;
            fn r#enableTestAttestation<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#isRemoteAttestationSupported<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<bool>>;
            fn r#isUpdatableVmSupported<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<bool>>;
            fn r#removeVmInstance<'a>(&'a self, _arg_instanceId: &'a [u8; 64]) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#claimVmInstance<'a>(&'a self, _arg_instanceId: &'a [u8; 64]) -> binder::BoxFuture<'a, binder::Result<()>>;
          }
          #[::async_trait::async_trait]
          pub trait IVirtualizationServiceAsyncServer: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.virtualizationservice.IVirtualizationService" }
            async fn r#createVm(&self, _arg_config: &crate::mangled::_7_android_6_system_21_virtualizationservice_20_VirtualMachineConfig, _arg_consoleOutFd: Option<&binder::ParcelFileDescriptor>, _arg_consoleInFd: Option<&binder::ParcelFileDescriptor>, _arg_osLogFd: Option<&binder::ParcelFileDescriptor>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_21_virtualizationservice_15_IVirtualMachine>>;
            async fn r#allocateInstanceId(&self) -> binder::Result<[u8; 64]>;
            async fn r#initializeWritablePartition(&self, _arg_imageFd: &binder::ParcelFileDescriptor, _arg_sizeBytes: i64, _arg_type: crate::mangled::_7_android_6_system_21_virtualizationservice_13_PartitionType) -> binder::Result<()>;
            async fn r#createOrUpdateIdsigFile(&self, _arg_inputFd: &binder::ParcelFileDescriptor, _arg_idsigFd: &binder::ParcelFileDescriptor) -> binder::Result<()>;
            async fn r#debugListVms(&self) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineDebugInfo>>;
            async fn r#getAssignableDevices(&self) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_16_AssignableDevice>>;
            async fn r#getSupportedOSList(&self) -> binder::Result<Vec<String>>;
            async fn r#isFeatureEnabled(&self, _arg_feature: &str) -> binder::Result<bool>;
            async fn r#enableTestAttestation(&self) -> binder::Result<()>;
            async fn r#isRemoteAttestationSupported(&self) -> binder::Result<bool>;
            async fn r#isUpdatableVmSupported(&self) -> binder::Result<bool>;
            async fn r#removeVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<()>;
            async fn r#claimVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<()>;
          }
          impl BnVirtualizationService {
            /// Create a new async binder service.
            pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IVirtualizationService>
            where
              T: IVirtualizationServiceAsyncServer + binder::Interface + Send + Sync + 'static,
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
              impl<T, R> IVirtualizationService for Wrapper<T, R>
              where
                T: IVirtualizationServiceAsyncServer + Send + Sync + 'static,
                R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
              {
                fn r#createVm(&self, _arg_config: &crate::mangled::_7_android_6_system_21_virtualizationservice_20_VirtualMachineConfig, _arg_consoleOutFd: Option<&binder::ParcelFileDescriptor>, _arg_consoleInFd: Option<&binder::ParcelFileDescriptor>, _arg_osLogFd: Option<&binder::ParcelFileDescriptor>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_21_virtualizationservice_15_IVirtualMachine>> {
                  self._rt.block_on(self._inner.r#createVm(_arg_config, _arg_consoleOutFd, _arg_consoleInFd, _arg_osLogFd))
                }
                fn r#allocateInstanceId(&self) -> binder::Result<[u8; 64]> {
                  self._rt.block_on(self._inner.r#allocateInstanceId())
                }
                fn r#initializeWritablePartition(&self, _arg_imageFd: &binder::ParcelFileDescriptor, _arg_sizeBytes: i64, _arg_type: crate::mangled::_7_android_6_system_21_virtualizationservice_13_PartitionType) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#initializeWritablePartition(_arg_imageFd, _arg_sizeBytes, _arg_type))
                }
                fn r#createOrUpdateIdsigFile(&self, _arg_inputFd: &binder::ParcelFileDescriptor, _arg_idsigFd: &binder::ParcelFileDescriptor) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#createOrUpdateIdsigFile(_arg_inputFd, _arg_idsigFd))
                }
                fn r#debugListVms(&self) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineDebugInfo>> {
                  self._rt.block_on(self._inner.r#debugListVms())
                }
                fn r#getAssignableDevices(&self) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_16_AssignableDevice>> {
                  self._rt.block_on(self._inner.r#getAssignableDevices())
                }
                fn r#getSupportedOSList(&self) -> binder::Result<Vec<String>> {
                  self._rt.block_on(self._inner.r#getSupportedOSList())
                }
                fn r#isFeatureEnabled(&self, _arg_feature: &str) -> binder::Result<bool> {
                  self._rt.block_on(self._inner.r#isFeatureEnabled(_arg_feature))
                }
                fn r#enableTestAttestation(&self) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#enableTestAttestation())
                }
                fn r#isRemoteAttestationSupported(&self) -> binder::Result<bool> {
                  self._rt.block_on(self._inner.r#isRemoteAttestationSupported())
                }
                fn r#isUpdatableVmSupported(&self) -> binder::Result<bool> {
                  self._rt.block_on(self._inner.r#isUpdatableVmSupported())
                }
                fn r#removeVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#removeVmInstance(_arg_instanceId))
                }
                fn r#claimVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#claimVmInstance(_arg_instanceId))
                }
                fn try_as_async_server(&self) -> Option<&(dyn IVirtualizationServiceAsyncServer + Send + Sync)> {
                  Some(&self._inner)
                }
              }
              let wrapped = Wrapper { _inner: inner, _rt: rt };
              Self::new_binder(wrapped, features)
            }
            pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn IVirtualizationServiceAsync<P>>> {
              struct Wrapper {
                _native: binder::binder_impl::Binder<BnVirtualizationService>
              }
              impl binder::Interface for Wrapper {}
              impl<P: binder::BinderAsyncPool> IVirtualizationServiceAsync<P> for Wrapper {
                fn r#createVm<'a>(&'a self, _arg_config: &'a crate::mangled::_7_android_6_system_21_virtualizationservice_20_VirtualMachineConfig, _arg_consoleOutFd: Option<&'a binder::ParcelFileDescriptor>, _arg_consoleInFd: Option<&'a binder::ParcelFileDescriptor>, _arg_osLogFd: Option<&'a binder::ParcelFileDescriptor>) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_21_virtualizationservice_15_IVirtualMachine>>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#createVm(_arg_config, _arg_consoleOutFd, _arg_consoleInFd, _arg_osLogFd))
                }
                fn r#allocateInstanceId<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<[u8; 64]>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#allocateInstanceId())
                }
                fn r#initializeWritablePartition<'a>(&'a self, _arg_imageFd: &'a binder::ParcelFileDescriptor, _arg_sizeBytes: i64, _arg_type: crate::mangled::_7_android_6_system_21_virtualizationservice_13_PartitionType) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#initializeWritablePartition(_arg_imageFd, _arg_sizeBytes, _arg_type))
                }
                fn r#createOrUpdateIdsigFile<'a>(&'a self, _arg_inputFd: &'a binder::ParcelFileDescriptor, _arg_idsigFd: &'a binder::ParcelFileDescriptor) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#createOrUpdateIdsigFile(_arg_inputFd, _arg_idsigFd))
                }
                fn r#debugListVms<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineDebugInfo>>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#debugListVms())
                }
                fn r#getAssignableDevices<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_16_AssignableDevice>>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#getAssignableDevices())
                }
                fn r#getSupportedOSList<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<String>>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#getSupportedOSList())
                }
                fn r#isFeatureEnabled<'a>(&'a self, _arg_feature: &'a str) -> binder::BoxFuture<'a, binder::Result<bool>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#isFeatureEnabled(_arg_feature))
                }
                fn r#enableTestAttestation<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#enableTestAttestation())
                }
                fn r#isRemoteAttestationSupported<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<bool>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#isRemoteAttestationSupported())
                }
                fn r#isUpdatableVmSupported<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<bool>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#isUpdatableVmSupported())
                }
                fn r#removeVmInstance<'a>(&'a self, _arg_instanceId: &'a [u8; 64]) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#removeVmInstance(_arg_instanceId))
                }
                fn r#claimVmInstance<'a>(&'a self, _arg_instanceId: &'a [u8; 64]) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#claimVmInstance(_arg_instanceId))
                }
              }
              if _native.try_as_async_server().is_some() {
                Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn IVirtualizationServiceAsync<P>>))
              } else {
                None
              }
            }
          }
          pub trait IVirtualizationServiceDefault: Send + Sync {
            fn r#createVm(&self, _arg_config: &crate::mangled::_7_android_6_system_21_virtualizationservice_20_VirtualMachineConfig, _arg_consoleOutFd: Option<&binder::ParcelFileDescriptor>, _arg_consoleInFd: Option<&binder::ParcelFileDescriptor>, _arg_osLogFd: Option<&binder::ParcelFileDescriptor>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_21_virtualizationservice_15_IVirtualMachine>> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#allocateInstanceId(&self) -> binder::Result<[u8; 64]> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#initializeWritablePartition(&self, _arg_imageFd: &binder::ParcelFileDescriptor, _arg_sizeBytes: i64, _arg_type: crate::mangled::_7_android_6_system_21_virtualizationservice_13_PartitionType) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#createOrUpdateIdsigFile(&self, _arg_inputFd: &binder::ParcelFileDescriptor, _arg_idsigFd: &binder::ParcelFileDescriptor) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#debugListVms(&self) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineDebugInfo>> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#getAssignableDevices(&self) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_16_AssignableDevice>> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#getSupportedOSList(&self) -> binder::Result<Vec<String>> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#isFeatureEnabled(&self, _arg_feature: &str) -> binder::Result<bool> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#enableTestAttestation(&self) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#isRemoteAttestationSupported(&self) -> binder::Result<bool> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#isUpdatableVmSupported(&self) -> binder::Result<bool> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#removeVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#claimVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
          }
          pub mod transactions {
            pub const r#createVm: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
            pub const r#allocateInstanceId: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
            pub const r#initializeWritablePartition: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
            pub const r#createOrUpdateIdsigFile: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
            pub const r#debugListVms: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
            pub const r#getAssignableDevices: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
            pub const r#getSupportedOSList: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
            pub const r#isFeatureEnabled: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
            pub const r#enableTestAttestation: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
            pub const r#isRemoteAttestationSupported: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
            pub const r#isUpdatableVmSupported: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
            pub const r#removeVmInstance: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 11;
            pub const r#claimVmInstance: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 12;
          }
          pub type IVirtualizationServiceDefaultRef = Option<std::sync::Arc<dyn IVirtualizationServiceDefault>>;
          static DEFAULT_IMPL: std::sync::Mutex<IVirtualizationServiceDefaultRef> = std::sync::Mutex::new(None);
          pub const r#FEATURE_DICE_CHANGES: &str = "com.android.kvm.DICE_CHANGES";
          pub const r#FEATURE_LLPVM_CHANGES: &str = "com.android.kvm.LLPVM_CHANGES";
          pub const r#FEATURE_MULTI_TENANT: &str = "com.android.kvm.MULTI_TENANT";
          pub const r#FEATURE_NETWORK: &str = "com.android.kvm.NETWORK";
          pub const r#FEATURE_REMOTE_ATTESTATION: &str = "com.android.kvm.REMOTE_ATTESTATION";
          pub const r#FEATURE_VENDOR_MODULES: &str = "com.android.kvm.VENDOR_MODULES";
          impl BpVirtualizationService {
            fn build_parcel_createVm(&self, _arg_config: &crate::mangled::_7_android_6_system_21_virtualizationservice_20_VirtualMachineConfig, _arg_consoleOutFd: Option<&binder::ParcelFileDescriptor>, _arg_consoleInFd: Option<&binder::ParcelFileDescriptor>, _arg_osLogFd: Option<&binder::ParcelFileDescriptor>) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_config)?;
              aidl_data.write(&_arg_consoleOutFd)?;
              aidl_data.write(&_arg_consoleInFd)?;
              aidl_data.write(&_arg_osLogFd)?;
              Ok(aidl_data)
            }
            fn read_response_createVm(&self, _arg_config: &crate::mangled::_7_android_6_system_21_virtualizationservice_20_VirtualMachineConfig, _arg_consoleOutFd: Option<&binder::ParcelFileDescriptor>, _arg_consoleInFd: Option<&binder::ParcelFileDescriptor>, _arg_osLogFd: Option<&binder::ParcelFileDescriptor>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_21_virtualizationservice_15_IVirtualMachine>> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationService>::getDefaultImpl() {
                  return _aidl_default_impl.r#createVm(_arg_config, _arg_consoleOutFd, _arg_consoleInFd, _arg_osLogFd);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: binder::Strong<dyn crate::mangled::_7_android_6_system_21_virtualizationservice_15_IVirtualMachine> = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_allocateInstanceId(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_allocateInstanceId(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<[u8; 64]> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationService>::getDefaultImpl() {
                  return _aidl_default_impl.r#allocateInstanceId();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: [u8; 64] = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_initializeWritablePartition(&self, _arg_imageFd: &binder::ParcelFileDescriptor, _arg_sizeBytes: i64, _arg_type: crate::mangled::_7_android_6_system_21_virtualizationservice_13_PartitionType) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_imageFd)?;
              aidl_data.write(&_arg_sizeBytes)?;
              aidl_data.write(&_arg_type)?;
              Ok(aidl_data)
            }
            fn read_response_initializeWritablePartition(&self, _arg_imageFd: &binder::ParcelFileDescriptor, _arg_sizeBytes: i64, _arg_type: crate::mangled::_7_android_6_system_21_virtualizationservice_13_PartitionType, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationService>::getDefaultImpl() {
                  return _aidl_default_impl.r#initializeWritablePartition(_arg_imageFd, _arg_sizeBytes, _arg_type);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_createOrUpdateIdsigFile(&self, _arg_inputFd: &binder::ParcelFileDescriptor, _arg_idsigFd: &binder::ParcelFileDescriptor) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_inputFd)?;
              aidl_data.write(_arg_idsigFd)?;
              Ok(aidl_data)
            }
            fn read_response_createOrUpdateIdsigFile(&self, _arg_inputFd: &binder::ParcelFileDescriptor, _arg_idsigFd: &binder::ParcelFileDescriptor, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationService>::getDefaultImpl() {
                  return _aidl_default_impl.r#createOrUpdateIdsigFile(_arg_inputFd, _arg_idsigFd);
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
                if let Some(_aidl_default_impl) = <Self as IVirtualizationService>::getDefaultImpl() {
                  return _aidl_default_impl.r#debugListVms();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineDebugInfo> = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_getAssignableDevices(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_getAssignableDevices(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_16_AssignableDevice>> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationService>::getDefaultImpl() {
                  return _aidl_default_impl.r#getAssignableDevices();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_16_AssignableDevice> = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_getSupportedOSList(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_getSupportedOSList(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<String>> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationService>::getDefaultImpl() {
                  return _aidl_default_impl.r#getSupportedOSList();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: Vec<String> = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_isFeatureEnabled(&self, _arg_feature: &str) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_feature)?;
              Ok(aidl_data)
            }
            fn read_response_isFeatureEnabled(&self, _arg_feature: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<bool> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationService>::getDefaultImpl() {
                  return _aidl_default_impl.r#isFeatureEnabled(_arg_feature);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: bool = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_enableTestAttestation(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_enableTestAttestation(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationService>::getDefaultImpl() {
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
                if let Some(_aidl_default_impl) = <Self as IVirtualizationService>::getDefaultImpl() {
                  return _aidl_default_impl.r#isRemoteAttestationSupported();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: bool = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_isUpdatableVmSupported(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              Ok(aidl_data)
            }
            fn read_response_isUpdatableVmSupported(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<bool> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationService>::getDefaultImpl() {
                  return _aidl_default_impl.r#isUpdatableVmSupported();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: bool = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_removeVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_instanceId)?;
              Ok(aidl_data)
            }
            fn read_response_removeVmInstance(&self, _arg_instanceId: &[u8; 64], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IVirtualizationService>::getDefaultImpl() {
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
                if let Some(_aidl_default_impl) = <Self as IVirtualizationService>::getDefaultImpl() {
                  return _aidl_default_impl.r#claimVmInstance(_arg_instanceId);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
          }
          impl IVirtualizationService for BpVirtualizationService {
            fn r#createVm(&self, _arg_config: &crate::mangled::_7_android_6_system_21_virtualizationservice_20_VirtualMachineConfig, _arg_consoleOutFd: Option<&binder::ParcelFileDescriptor>, _arg_consoleInFd: Option<&binder::ParcelFileDescriptor>, _arg_osLogFd: Option<&binder::ParcelFileDescriptor>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_21_virtualizationservice_15_IVirtualMachine>> {
              let _aidl_data = self.build_parcel_createVm(_arg_config, _arg_consoleOutFd, _arg_consoleInFd, _arg_osLogFd)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#createVm, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_createVm(_arg_config, _arg_consoleOutFd, _arg_consoleInFd, _arg_osLogFd, _aidl_reply)
            }
            fn r#allocateInstanceId(&self) -> binder::Result<[u8; 64]> {
              let _aidl_data = self.build_parcel_allocateInstanceId()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#allocateInstanceId, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_allocateInstanceId(_aidl_reply)
            }
            fn r#initializeWritablePartition(&self, _arg_imageFd: &binder::ParcelFileDescriptor, _arg_sizeBytes: i64, _arg_type: crate::mangled::_7_android_6_system_21_virtualizationservice_13_PartitionType) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_initializeWritablePartition(_arg_imageFd, _arg_sizeBytes, _arg_type)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#initializeWritablePartition, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_initializeWritablePartition(_arg_imageFd, _arg_sizeBytes, _arg_type, _aidl_reply)
            }
            fn r#createOrUpdateIdsigFile(&self, _arg_inputFd: &binder::ParcelFileDescriptor, _arg_idsigFd: &binder::ParcelFileDescriptor) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_createOrUpdateIdsigFile(_arg_inputFd, _arg_idsigFd)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#createOrUpdateIdsigFile, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_createOrUpdateIdsigFile(_arg_inputFd, _arg_idsigFd, _aidl_reply)
            }
            fn r#debugListVms(&self) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineDebugInfo>> {
              let _aidl_data = self.build_parcel_debugListVms()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#debugListVms, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_debugListVms(_aidl_reply)
            }
            fn r#getAssignableDevices(&self) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_16_AssignableDevice>> {
              let _aidl_data = self.build_parcel_getAssignableDevices()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#getAssignableDevices, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_getAssignableDevices(_aidl_reply)
            }
            fn r#getSupportedOSList(&self) -> binder::Result<Vec<String>> {
              let _aidl_data = self.build_parcel_getSupportedOSList()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#getSupportedOSList, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_getSupportedOSList(_aidl_reply)
            }
            fn r#isFeatureEnabled(&self, _arg_feature: &str) -> binder::Result<bool> {
              let _aidl_data = self.build_parcel_isFeatureEnabled(_arg_feature)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#isFeatureEnabled, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_isFeatureEnabled(_arg_feature, _aidl_reply)
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
            fn r#isUpdatableVmSupported(&self) -> binder::Result<bool> {
              let _aidl_data = self.build_parcel_isUpdatableVmSupported()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#isUpdatableVmSupported, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
              self.read_response_isUpdatableVmSupported(_aidl_reply)
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
          }
          impl<P: binder::BinderAsyncPool> IVirtualizationServiceAsync<P> for BpVirtualizationService {
            fn r#createVm<'a>(&'a self, _arg_config: &'a crate::mangled::_7_android_6_system_21_virtualizationservice_20_VirtualMachineConfig, _arg_consoleOutFd: Option<&'a binder::ParcelFileDescriptor>, _arg_consoleInFd: Option<&'a binder::ParcelFileDescriptor>, _arg_osLogFd: Option<&'a binder::ParcelFileDescriptor>) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_21_virtualizationservice_15_IVirtualMachine>>> {
              let _aidl_data = match self.build_parcel_createVm(_arg_config, _arg_consoleOutFd, _arg_consoleInFd, _arg_osLogFd) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#createVm, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_createVm(_arg_config, _arg_consoleOutFd, _arg_consoleInFd, _arg_osLogFd, _aidl_reply)
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
            fn r#initializeWritablePartition<'a>(&'a self, _arg_imageFd: &'a binder::ParcelFileDescriptor, _arg_sizeBytes: i64, _arg_type: crate::mangled::_7_android_6_system_21_virtualizationservice_13_PartitionType) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_initializeWritablePartition(_arg_imageFd, _arg_sizeBytes, _arg_type) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#initializeWritablePartition, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_initializeWritablePartition(_arg_imageFd, _arg_sizeBytes, _arg_type, _aidl_reply)
                }
              )
            }
            fn r#createOrUpdateIdsigFile<'a>(&'a self, _arg_inputFd: &'a binder::ParcelFileDescriptor, _arg_idsigFd: &'a binder::ParcelFileDescriptor) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_createOrUpdateIdsigFile(_arg_inputFd, _arg_idsigFd) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#createOrUpdateIdsigFile, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_createOrUpdateIdsigFile(_arg_inputFd, _arg_idsigFd, _aidl_reply)
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
            fn r#getSupportedOSList<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<String>>> {
              let _aidl_data = match self.build_parcel_getSupportedOSList() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#getSupportedOSList, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_getSupportedOSList(_aidl_reply)
                }
              )
            }
            fn r#isFeatureEnabled<'a>(&'a self, _arg_feature: &'a str) -> binder::BoxFuture<'a, binder::Result<bool>> {
              let _aidl_data = match self.build_parcel_isFeatureEnabled(_arg_feature) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#isFeatureEnabled, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_isFeatureEnabled(_arg_feature, _aidl_reply)
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
            fn r#isUpdatableVmSupported<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<bool>> {
              let _aidl_data = match self.build_parcel_isUpdatableVmSupported() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#isUpdatableVmSupported, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_isUpdatableVmSupported(_aidl_reply)
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
          }
          impl IVirtualizationService for binder::binder_impl::Binder<BnVirtualizationService> {
            fn r#createVm(&self, _arg_config: &crate::mangled::_7_android_6_system_21_virtualizationservice_20_VirtualMachineConfig, _arg_consoleOutFd: Option<&binder::ParcelFileDescriptor>, _arg_consoleInFd: Option<&binder::ParcelFileDescriptor>, _arg_osLogFd: Option<&binder::ParcelFileDescriptor>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_21_virtualizationservice_15_IVirtualMachine>> { self.0.r#createVm(_arg_config, _arg_consoleOutFd, _arg_consoleInFd, _arg_osLogFd) }
            fn r#allocateInstanceId(&self) -> binder::Result<[u8; 64]> { self.0.r#allocateInstanceId() }
            fn r#initializeWritablePartition(&self, _arg_imageFd: &binder::ParcelFileDescriptor, _arg_sizeBytes: i64, _arg_type: crate::mangled::_7_android_6_system_21_virtualizationservice_13_PartitionType) -> binder::Result<()> { self.0.r#initializeWritablePartition(_arg_imageFd, _arg_sizeBytes, _arg_type) }
            fn r#createOrUpdateIdsigFile(&self, _arg_inputFd: &binder::ParcelFileDescriptor, _arg_idsigFd: &binder::ParcelFileDescriptor) -> binder::Result<()> { self.0.r#createOrUpdateIdsigFile(_arg_inputFd, _arg_idsigFd) }
            fn r#debugListVms(&self) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineDebugInfo>> { self.0.r#debugListVms() }
            fn r#getAssignableDevices(&self) -> binder::Result<Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_16_AssignableDevice>> { self.0.r#getAssignableDevices() }
            fn r#getSupportedOSList(&self) -> binder::Result<Vec<String>> { self.0.r#getSupportedOSList() }
            fn r#isFeatureEnabled(&self, _arg_feature: &str) -> binder::Result<bool> { self.0.r#isFeatureEnabled(_arg_feature) }
            fn r#enableTestAttestation(&self) -> binder::Result<()> { self.0.r#enableTestAttestation() }
            fn r#isRemoteAttestationSupported(&self) -> binder::Result<bool> { self.0.r#isRemoteAttestationSupported() }
            fn r#isUpdatableVmSupported(&self) -> binder::Result<bool> { self.0.r#isUpdatableVmSupported() }
            fn r#removeVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<()> { self.0.r#removeVmInstance(_arg_instanceId) }
            fn r#claimVmInstance(&self, _arg_instanceId: &[u8; 64]) -> binder::Result<()> { self.0.r#claimVmInstance(_arg_instanceId) }
          }
          fn on_transact(_aidl_service: &dyn IVirtualizationService, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
            match _aidl_code {
              transactions::r#createVm => {
                let _arg_config: crate::mangled::_7_android_6_system_21_virtualizationservice_20_VirtualMachineConfig = _aidl_data.read()?;
                let _arg_consoleOutFd: Option<binder::ParcelFileDescriptor> = _aidl_data.read()?;
                let _arg_consoleInFd: Option<binder::ParcelFileDescriptor> = _aidl_data.read()?;
                let _arg_osLogFd: Option<binder::ParcelFileDescriptor> = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#createVm(&_arg_config, _arg_consoleOutFd.as_ref(), _arg_consoleInFd.as_ref(), _arg_osLogFd.as_ref());
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
              transactions::r#initializeWritablePartition => {
                let _arg_imageFd: binder::ParcelFileDescriptor = _aidl_data.read()?;
                let _arg_sizeBytes: i64 = _aidl_data.read()?;
                let _arg_type: crate::mangled::_7_android_6_system_21_virtualizationservice_13_PartitionType = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#initializeWritablePartition(&_arg_imageFd, _arg_sizeBytes, _arg_type);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#createOrUpdateIdsigFile => {
                let _arg_inputFd: binder::ParcelFileDescriptor = _aidl_data.read()?;
                let _arg_idsigFd: binder::ParcelFileDescriptor = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#createOrUpdateIdsigFile(&_arg_inputFd, &_arg_idsigFd);
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
              transactions::r#getSupportedOSList => {
                let _aidl_return = _aidl_service.r#getSupportedOSList();
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#isFeatureEnabled => {
                let _arg_feature: String = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#isFeatureEnabled(&_arg_feature);
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
              transactions::r#isUpdatableVmSupported => {
                let _aidl_return = _aidl_service.r#isUpdatableVmSupported();
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
              _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
            }
          }
          pub(crate) mod mangled {
           pub use super::r#IVirtualizationService as _7_android_6_system_21_virtualizationservice_22_IVirtualizationService;
          }
        }
        pub mod InputDevice {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen/android/system/virtualizationservice/InputDevice.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice/InputDevice.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug)]
          pub enum r#InputDevice {
            SingleTouch(crate::mangled::_7_android_6_system_21_virtualizationservice_11_InputDevice_11_SingleTouch),
            EvDev(crate::mangled::_7_android_6_system_21_virtualizationservice_11_InputDevice_5_EvDev),
            Keyboard(crate::mangled::_7_android_6_system_21_virtualizationservice_11_InputDevice_8_Keyboard),
            Mouse(crate::mangled::_7_android_6_system_21_virtualizationservice_11_InputDevice_5_Mouse),
            Switches(crate::mangled::_7_android_6_system_21_virtualizationservice_11_InputDevice_8_Switches),
            Trackpad(crate::mangled::_7_android_6_system_21_virtualizationservice_11_InputDevice_8_Trackpad),
            MultiTouch(crate::mangled::_7_android_6_system_21_virtualizationservice_11_InputDevice_10_MultiTouch),
          }
          impl Default for r#InputDevice {
            fn default() -> Self {
              Self::SingleTouch(Default::default())
            }
          }
          impl binder::Parcelable for r#InputDevice {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              match self {
                Self::SingleTouch(v) => {
                  parcel.write(&0i32)?;
                  parcel.write(v)
                }
                Self::EvDev(v) => {
                  parcel.write(&1i32)?;
                  parcel.write(v)
                }
                Self::Keyboard(v) => {
                  parcel.write(&2i32)?;
                  parcel.write(v)
                }
                Self::Mouse(v) => {
                  parcel.write(&3i32)?;
                  parcel.write(v)
                }
                Self::Switches(v) => {
                  parcel.write(&4i32)?;
                  parcel.write(v)
                }
                Self::Trackpad(v) => {
                  parcel.write(&5i32)?;
                  parcel.write(v)
                }
                Self::MultiTouch(v) => {
                  parcel.write(&6i32)?;
                  parcel.write(v)
                }
              }
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              let tag: i32 = parcel.read()?;
              match tag {
                0 => {
                  let value: crate::mangled::_7_android_6_system_21_virtualizationservice_11_InputDevice_11_SingleTouch = parcel.read()?;
                  *self = Self::SingleTouch(value);
                  Ok(())
                }
                1 => {
                  let value: crate::mangled::_7_android_6_system_21_virtualizationservice_11_InputDevice_5_EvDev = parcel.read()?;
                  *self = Self::EvDev(value);
                  Ok(())
                }
                2 => {
                  let value: crate::mangled::_7_android_6_system_21_virtualizationservice_11_InputDevice_8_Keyboard = parcel.read()?;
                  *self = Self::Keyboard(value);
                  Ok(())
                }
                3 => {
                  let value: crate::mangled::_7_android_6_system_21_virtualizationservice_11_InputDevice_5_Mouse = parcel.read()?;
                  *self = Self::Mouse(value);
                  Ok(())
                }
                4 => {
                  let value: crate::mangled::_7_android_6_system_21_virtualizationservice_11_InputDevice_8_Switches = parcel.read()?;
                  *self = Self::Switches(value);
                  Ok(())
                }
                5 => {
                  let value: crate::mangled::_7_android_6_system_21_virtualizationservice_11_InputDevice_8_Trackpad = parcel.read()?;
                  *self = Self::Trackpad(value);
                  Ok(())
                }
                6 => {
                  let value: crate::mangled::_7_android_6_system_21_virtualizationservice_11_InputDevice_10_MultiTouch = parcel.read()?;
                  *self = Self::MultiTouch(value);
                  Ok(())
                }
                _ => {
                  Err(binder::StatusCode::BAD_VALUE)
                }
              }
            }
          }
          binder::impl_serialize_for_parcelable!(r#InputDevice);
          binder::impl_deserialize_for_parcelable!(r#InputDevice);
          impl binder::binder_impl::ParcelableMetadata for r#InputDevice {
            fn get_descriptor() -> &'static str { "android.system.virtualizationservice.InputDevice" }
          }
          pub mod r#SingleTouch {
            #[derive(Debug)]
            pub struct r#SingleTouch {
              pub r#pfd: Option<binder::ParcelFileDescriptor>,
              pub r#width: i32,
              pub r#height: i32,
              pub r#name: String,
            }
            impl Default for r#SingleTouch {
              fn default() -> Self {
                Self {
                  r#pfd: Default::default(),
                  r#width: 1280,
                  r#height: 1080,
                  r#name: "".into(),
                }
              }
            }
            impl binder::Parcelable for r#SingleTouch {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  let __field_ref = self.r#pfd.as_ref().ok_or(binder::StatusCode::UNEXPECTED_NULL)?;
                  subparcel.write(__field_ref)?;
                  subparcel.write(&self.r#width)?;
                  subparcel.write(&self.r#height)?;
                  subparcel.write(&self.r#name)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#pfd = Some(subparcel.read()?);
                  }
                  if subparcel.has_more_data() {
                    self.r#width = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#height = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#name = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#SingleTouch);
            binder::impl_deserialize_for_parcelable!(r#SingleTouch);
            impl binder::binder_impl::ParcelableMetadata for r#SingleTouch {
              fn get_descriptor() -> &'static str { "android.system.virtualizationservice.InputDevice.SingleTouch" }
            }
          }
          pub mod r#EvDev {
            #[derive(Debug)]
            pub struct r#EvDev {
              pub r#pfd: Option<binder::ParcelFileDescriptor>,
            }
            impl Default for r#EvDev {
              fn default() -> Self {
                Self {
                  r#pfd: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#EvDev {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  let __field_ref = self.r#pfd.as_ref().ok_or(binder::StatusCode::UNEXPECTED_NULL)?;
                  subparcel.write(__field_ref)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#pfd = Some(subparcel.read()?);
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#EvDev);
            binder::impl_deserialize_for_parcelable!(r#EvDev);
            impl binder::binder_impl::ParcelableMetadata for r#EvDev {
              fn get_descriptor() -> &'static str { "android.system.virtualizationservice.InputDevice.EvDev" }
            }
          }
          pub mod r#Keyboard {
            #[derive(Debug)]
            pub struct r#Keyboard {
              pub r#pfd: Option<binder::ParcelFileDescriptor>,
            }
            impl Default for r#Keyboard {
              fn default() -> Self {
                Self {
                  r#pfd: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#Keyboard {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  let __field_ref = self.r#pfd.as_ref().ok_or(binder::StatusCode::UNEXPECTED_NULL)?;
                  subparcel.write(__field_ref)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#pfd = Some(subparcel.read()?);
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#Keyboard);
            binder::impl_deserialize_for_parcelable!(r#Keyboard);
            impl binder::binder_impl::ParcelableMetadata for r#Keyboard {
              fn get_descriptor() -> &'static str { "android.system.virtualizationservice.InputDevice.Keyboard" }
            }
          }
          pub mod r#Mouse {
            #[derive(Debug)]
            pub struct r#Mouse {
              pub r#pfd: Option<binder::ParcelFileDescriptor>,
            }
            impl Default for r#Mouse {
              fn default() -> Self {
                Self {
                  r#pfd: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#Mouse {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  let __field_ref = self.r#pfd.as_ref().ok_or(binder::StatusCode::UNEXPECTED_NULL)?;
                  subparcel.write(__field_ref)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#pfd = Some(subparcel.read()?);
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#Mouse);
            binder::impl_deserialize_for_parcelable!(r#Mouse);
            impl binder::binder_impl::ParcelableMetadata for r#Mouse {
              fn get_descriptor() -> &'static str { "android.system.virtualizationservice.InputDevice.Mouse" }
            }
          }
          pub mod r#Switches {
            #[derive(Debug)]
            pub struct r#Switches {
              pub r#pfd: Option<binder::ParcelFileDescriptor>,
            }
            impl Default for r#Switches {
              fn default() -> Self {
                Self {
                  r#pfd: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#Switches {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  let __field_ref = self.r#pfd.as_ref().ok_or(binder::StatusCode::UNEXPECTED_NULL)?;
                  subparcel.write(__field_ref)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#pfd = Some(subparcel.read()?);
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#Switches);
            binder::impl_deserialize_for_parcelable!(r#Switches);
            impl binder::binder_impl::ParcelableMetadata for r#Switches {
              fn get_descriptor() -> &'static str { "android.system.virtualizationservice.InputDevice.Switches" }
            }
          }
          pub mod r#Trackpad {
            #[derive(Debug)]
            pub struct r#Trackpad {
              pub r#pfd: Option<binder::ParcelFileDescriptor>,
              pub r#width: i32,
              pub r#height: i32,
              pub r#name: String,
            }
            impl Default for r#Trackpad {
              fn default() -> Self {
                Self {
                  r#pfd: Default::default(),
                  r#width: 1280,
                  r#height: 1080,
                  r#name: "".into(),
                }
              }
            }
            impl binder::Parcelable for r#Trackpad {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  let __field_ref = self.r#pfd.as_ref().ok_or(binder::StatusCode::UNEXPECTED_NULL)?;
                  subparcel.write(__field_ref)?;
                  subparcel.write(&self.r#width)?;
                  subparcel.write(&self.r#height)?;
                  subparcel.write(&self.r#name)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#pfd = Some(subparcel.read()?);
                  }
                  if subparcel.has_more_data() {
                    self.r#width = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#height = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#name = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#Trackpad);
            binder::impl_deserialize_for_parcelable!(r#Trackpad);
            impl binder::binder_impl::ParcelableMetadata for r#Trackpad {
              fn get_descriptor() -> &'static str { "android.system.virtualizationservice.InputDevice.Trackpad" }
            }
          }
          pub mod r#MultiTouch {
            #[derive(Debug)]
            pub struct r#MultiTouch {
              pub r#pfd: Option<binder::ParcelFileDescriptor>,
              pub r#width: i32,
              pub r#height: i32,
              pub r#name: String,
            }
            impl Default for r#MultiTouch {
              fn default() -> Self {
                Self {
                  r#pfd: Default::default(),
                  r#width: 1280,
                  r#height: 1080,
                  r#name: "".into(),
                }
              }
            }
            impl binder::Parcelable for r#MultiTouch {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  let __field_ref = self.r#pfd.as_ref().ok_or(binder::StatusCode::UNEXPECTED_NULL)?;
                  subparcel.write(__field_ref)?;
                  subparcel.write(&self.r#width)?;
                  subparcel.write(&self.r#height)?;
                  subparcel.write(&self.r#name)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#pfd = Some(subparcel.read()?);
                  }
                  if subparcel.has_more_data() {
                    self.r#width = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#height = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#name = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#MultiTouch);
            binder::impl_deserialize_for_parcelable!(r#MultiTouch);
            impl binder::binder_impl::ParcelableMetadata for r#MultiTouch {
              fn get_descriptor() -> &'static str { "android.system.virtualizationservice.InputDevice.MultiTouch" }
            }
          }
          pub mod r#Tag {
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(4))]
              r#Tag : [i32; 7] {
                r#singleTouch = 0,
                r#evDev = 1,
                r#keyboard = 2,
                r#mouse = 3,
                r#switches = 4,
                r#trackpad = 5,
                r#multiTouch = 6,
              }
            }
          }
          pub(crate) mod mangled {
           pub use super::r#InputDevice as _7_android_6_system_21_virtualizationservice_11_InputDevice;
           pub use super::r#SingleTouch::r#SingleTouch as _7_android_6_system_21_virtualizationservice_11_InputDevice_11_SingleTouch;
           pub use super::r#EvDev::r#EvDev as _7_android_6_system_21_virtualizationservice_11_InputDevice_5_EvDev;
           pub use super::r#Keyboard::r#Keyboard as _7_android_6_system_21_virtualizationservice_11_InputDevice_8_Keyboard;
           pub use super::r#Mouse::r#Mouse as _7_android_6_system_21_virtualizationservice_11_InputDevice_5_Mouse;
           pub use super::r#Switches::r#Switches as _7_android_6_system_21_virtualizationservice_11_InputDevice_8_Switches;
           pub use super::r#Trackpad::r#Trackpad as _7_android_6_system_21_virtualizationservice_11_InputDevice_8_Trackpad;
           pub use super::r#MultiTouch::r#MultiTouch as _7_android_6_system_21_virtualizationservice_11_InputDevice_10_MultiTouch;
           pub use super::r#Tag::r#Tag as _7_android_6_system_21_virtualizationservice_11_InputDevice_3_Tag;
          }
        }
        pub mod Partition {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen/android/system/virtualizationservice/Partition.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice/Partition.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug)]
          pub struct r#Partition {
            pub r#label: String,
            pub r#image: Option<binder::ParcelFileDescriptor>,
            pub r#writable: bool,
            pub r#guid: Option<String>,
          }
          impl Default for r#Partition {
            fn default() -> Self {
              Self {
                r#label: Default::default(),
                r#image: Default::default(),
                r#writable: false,
                r#guid: Default::default(),
              }
            }
          }
          impl binder::Parcelable for r#Partition {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#label)?;
                let __field_ref = self.r#image.as_ref().ok_or(binder::StatusCode::UNEXPECTED_NULL)?;
                subparcel.write(__field_ref)?;
                subparcel.write(&self.r#writable)?;
                subparcel.write(&self.r#guid)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#label = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#image = Some(subparcel.read()?);
                }
                if subparcel.has_more_data() {
                  self.r#writable = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#guid = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#Partition);
          binder::impl_deserialize_for_parcelable!(r#Partition);
          impl binder::binder_impl::ParcelableMetadata for r#Partition {
            fn get_descriptor() -> &'static str { "android.system.virtualizationservice.Partition" }
          }
          pub(crate) mod mangled {
           pub use super::r#Partition as _7_android_6_system_21_virtualizationservice_9_Partition;
          }
        }
        pub mod PartitionType {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen/android/system/virtualizationservice/PartitionType.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice/PartitionType.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #![allow(non_upper_case_globals)]
          use binder::declare_binder_enum;
          declare_binder_enum! {
            #[repr(C, align(4))]
            r#PartitionType : [i32; 3] {
              r#RAW = 0,
              r#ANDROID_VM_INSTANCE = 1,
              r#ENCRYPTEDSTORE = 2,
            }
          }
          pub(crate) mod mangled {
           pub use super::r#PartitionType as _7_android_6_system_21_virtualizationservice_13_PartitionType;
          }
        }
        pub mod UsbConfig {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen/android/system/virtualizationservice/UsbConfig.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice/UsbConfig.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug)]
          pub struct r#UsbConfig {
            pub r#controller: bool,
          }
          impl Default for r#UsbConfig {
            fn default() -> Self {
              Self {
                r#controller: false,
              }
            }
          }
          impl binder::Parcelable for r#UsbConfig {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#controller)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#controller = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#UsbConfig);
          binder::impl_deserialize_for_parcelable!(r#UsbConfig);
          impl binder::binder_impl::ParcelableMetadata for r#UsbConfig {
            fn get_descriptor() -> &'static str { "android.system.virtualizationservice.UsbConfig" }
          }
          pub(crate) mod mangled {
           pub use super::r#UsbConfig as _7_android_6_system_21_virtualizationservice_9_UsbConfig;
          }
        }
        pub mod VirtualMachineAppConfig {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen/android/system/virtualizationservice/VirtualMachineAppConfig.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice/VirtualMachineAppConfig.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug)]
          pub struct r#VirtualMachineAppConfig {
            pub r#name: String,
            pub r#instanceId: [u8; 64],
            pub r#apk: Option<binder::ParcelFileDescriptor>,
            pub r#idsig: Option<binder::ParcelFileDescriptor>,
            pub r#extraIdsigs: Vec<binder::ParcelFileDescriptor>,
            pub r#instanceImage: Option<binder::ParcelFileDescriptor>,
            pub r#encryptedStorageImage: Option<binder::ParcelFileDescriptor>,
            pub r#payload: crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineAppConfig_7_Payload,
            pub r#osName: String,
            pub r#debugLevel: crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineAppConfig_10_DebugLevel,
            pub r#protectedVm: bool,
            pub r#memoryMib: i32,
            pub r#cpuTopology: crate::mangled::_7_android_6_system_21_virtualizationservice_11_CpuTopology,
            pub r#customConfig: Option<crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineAppConfig_12_CustomConfig>,
            pub r#hugePages: bool,
            pub r#boostUclamp: bool,
            pub r#extraApksOverride: Vec<binder::ParcelFileDescriptor>,
          }
          impl Default for r#VirtualMachineAppConfig {
            fn default() -> Self {
              Self {
                r#name: Default::default(),
                r#instanceId: [Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default()],
                r#apk: Default::default(),
                r#idsig: Default::default(),
                r#extraIdsigs: Default::default(),
                r#instanceImage: Default::default(),
                r#encryptedStorageImage: Default::default(),
                r#payload: Default::default(),
                r#osName: "microdroid".into(),
                r#debugLevel: crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineAppConfig_10_DebugLevel::NONE,
                r#protectedVm: false,
                r#memoryMib: 0,
                r#cpuTopology: crate::mangled::_7_android_6_system_21_virtualizationservice_11_CpuTopology::ONE_CPU,
                r#customConfig: Default::default(),
                r#hugePages: false,
                r#boostUclamp: false,
                r#extraApksOverride: Default::default(),
              }
            }
          }
          impl binder::Parcelable for r#VirtualMachineAppConfig {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#name)?;
                subparcel.write(&self.r#instanceId)?;
                let __field_ref = self.r#apk.as_ref().ok_or(binder::StatusCode::UNEXPECTED_NULL)?;
                subparcel.write(__field_ref)?;
                let __field_ref = self.r#idsig.as_ref().ok_or(binder::StatusCode::UNEXPECTED_NULL)?;
                subparcel.write(__field_ref)?;
                subparcel.write(&self.r#extraIdsigs)?;
                let __field_ref = self.r#instanceImage.as_ref().ok_or(binder::StatusCode::UNEXPECTED_NULL)?;
                subparcel.write(__field_ref)?;
                subparcel.write(&self.r#encryptedStorageImage)?;
                subparcel.write(&self.r#payload)?;
                subparcel.write(&self.r#osName)?;
                subparcel.write(&self.r#debugLevel)?;
                subparcel.write(&self.r#protectedVm)?;
                subparcel.write(&self.r#memoryMib)?;
                subparcel.write(&self.r#cpuTopology)?;
                subparcel.write(&self.r#customConfig)?;
                subparcel.write(&self.r#hugePages)?;
                subparcel.write(&self.r#boostUclamp)?;
                subparcel.write(&self.r#extraApksOverride)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#name = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#instanceId = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#apk = Some(subparcel.read()?);
                }
                if subparcel.has_more_data() {
                  self.r#idsig = Some(subparcel.read()?);
                }
                if subparcel.has_more_data() {
                  self.r#extraIdsigs = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#instanceImage = Some(subparcel.read()?);
                }
                if subparcel.has_more_data() {
                  self.r#encryptedStorageImage = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#payload = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#osName = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#debugLevel = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#protectedVm = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#memoryMib = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#cpuTopology = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#customConfig = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#hugePages = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#boostUclamp = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#extraApksOverride = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#VirtualMachineAppConfig);
          binder::impl_deserialize_for_parcelable!(r#VirtualMachineAppConfig);
          impl binder::binder_impl::ParcelableMetadata for r#VirtualMachineAppConfig {
            fn get_descriptor() -> &'static str { "android.system.virtualizationservice.VirtualMachineAppConfig" }
          }
          pub mod r#Payload {
            #[derive(Debug)]
            pub enum r#Payload {
              ConfigPath(String),
              PayloadConfig(crate::mangled::_7_android_6_system_21_virtualizationservice_27_VirtualMachinePayloadConfig),
            }
            impl Default for r#Payload {
              fn default() -> Self {
                Self::ConfigPath(Default::default())
              }
            }
            impl binder::Parcelable for r#Payload {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                match self {
                  Self::ConfigPath(v) => {
                    parcel.write(&0i32)?;
                    parcel.write(v)
                  }
                  Self::PayloadConfig(v) => {
                    parcel.write(&1i32)?;
                    parcel.write(v)
                  }
                }
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                let tag: i32 = parcel.read()?;
                match tag {
                  0 => {
                    let value: String = parcel.read()?;
                    *self = Self::ConfigPath(value);
                    Ok(())
                  }
                  1 => {
                    let value: crate::mangled::_7_android_6_system_21_virtualizationservice_27_VirtualMachinePayloadConfig = parcel.read()?;
                    *self = Self::PayloadConfig(value);
                    Ok(())
                  }
                  _ => {
                    Err(binder::StatusCode::BAD_VALUE)
                  }
                }
              }
            }
            binder::impl_serialize_for_parcelable!(r#Payload);
            binder::impl_deserialize_for_parcelable!(r#Payload);
            impl binder::binder_impl::ParcelableMetadata for r#Payload {
              fn get_descriptor() -> &'static str { "android.system.virtualizationservice.VirtualMachineAppConfig.Payload" }
            }
            pub mod r#Tag {
              #![allow(non_upper_case_globals)]
              use binder::declare_binder_enum;
              declare_binder_enum! {
                #[repr(C, align(4))]
                r#Tag : [i32; 2] {
                  r#configPath = 0,
                  r#payloadConfig = 1,
                }
              }
            }
          }
          pub mod r#DebugLevel {
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(1))]
              r#DebugLevel : [i8; 2] {
                r#NONE = 0,
                r#FULL = 1,
              }
            }
          }
          pub mod r#CustomConfig {
            #[derive(Debug)]
            pub struct r#CustomConfig {
              pub r#customKernelImage: Option<binder::ParcelFileDescriptor>,
              pub r#gdbPort: i32,
              pub r#vendorImage: Option<binder::ParcelFileDescriptor>,
              pub r#devices: Vec<String>,
              pub r#wantUpdatable: bool,
              pub r#networkSupported: bool,
              pub r#extraKernelCmdlineParams: Vec<String>,
            }
            impl Default for r#CustomConfig {
              fn default() -> Self {
                Self {
                  r#customKernelImage: Default::default(),
                  r#gdbPort: 0,
                  r#vendorImage: Default::default(),
                  r#devices: Default::default(),
                  r#wantUpdatable: true,
                  r#networkSupported: false,
                  r#extraKernelCmdlineParams: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#CustomConfig {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#customKernelImage)?;
                  subparcel.write(&self.r#gdbPort)?;
                  subparcel.write(&self.r#vendorImage)?;
                  subparcel.write(&self.r#devices)?;
                  subparcel.write(&self.r#wantUpdatable)?;
                  subparcel.write(&self.r#networkSupported)?;
                  subparcel.write(&self.r#extraKernelCmdlineParams)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#customKernelImage = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#gdbPort = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#vendorImage = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#devices = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#wantUpdatable = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#networkSupported = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#extraKernelCmdlineParams = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#CustomConfig);
            binder::impl_deserialize_for_parcelable!(r#CustomConfig);
            impl binder::binder_impl::ParcelableMetadata for r#CustomConfig {
              fn get_descriptor() -> &'static str { "android.system.virtualizationservice.VirtualMachineAppConfig.CustomConfig" }
            }
          }
          pub(crate) mod mangled {
           pub use super::r#VirtualMachineAppConfig as _7_android_6_system_21_virtualizationservice_23_VirtualMachineAppConfig;
           pub use super::r#Payload::r#Payload as _7_android_6_system_21_virtualizationservice_23_VirtualMachineAppConfig_7_Payload;
           pub use super::r#Payload::r#Tag::r#Tag as _7_android_6_system_21_virtualizationservice_23_VirtualMachineAppConfig_7_Payload_3_Tag;
           pub use super::r#DebugLevel::r#DebugLevel as _7_android_6_system_21_virtualizationservice_23_VirtualMachineAppConfig_10_DebugLevel;
           pub use super::r#CustomConfig::r#CustomConfig as _7_android_6_system_21_virtualizationservice_23_VirtualMachineAppConfig_12_CustomConfig;
          }
        }
        pub mod VirtualMachineConfig {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen/android/system/virtualizationservice/VirtualMachineConfig.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice/VirtualMachineConfig.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug)]
          pub enum r#VirtualMachineConfig {
            AppConfig(crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineAppConfig),
            RawConfig(crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineRawConfig),
          }
          impl Default for r#VirtualMachineConfig {
            fn default() -> Self {
              Self::AppConfig(Default::default())
            }
          }
          impl binder::Parcelable for r#VirtualMachineConfig {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              match self {
                Self::AppConfig(v) => {
                  parcel.write(&0i32)?;
                  parcel.write(v)
                }
                Self::RawConfig(v) => {
                  parcel.write(&1i32)?;
                  parcel.write(v)
                }
              }
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              let tag: i32 = parcel.read()?;
              match tag {
                0 => {
                  let value: crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineAppConfig = parcel.read()?;
                  *self = Self::AppConfig(value);
                  Ok(())
                }
                1 => {
                  let value: crate::mangled::_7_android_6_system_21_virtualizationservice_23_VirtualMachineRawConfig = parcel.read()?;
                  *self = Self::RawConfig(value);
                  Ok(())
                }
                _ => {
                  Err(binder::StatusCode::BAD_VALUE)
                }
              }
            }
          }
          binder::impl_serialize_for_parcelable!(r#VirtualMachineConfig);
          binder::impl_deserialize_for_parcelable!(r#VirtualMachineConfig);
          impl binder::binder_impl::ParcelableMetadata for r#VirtualMachineConfig {
            fn get_descriptor() -> &'static str { "android.system.virtualizationservice.VirtualMachineConfig" }
          }
          pub mod r#Tag {
            #![allow(non_upper_case_globals)]
            use binder::declare_binder_enum;
            declare_binder_enum! {
              #[repr(C, align(4))]
              r#Tag : [i32; 2] {
                r#appConfig = 0,
                r#rawConfig = 1,
              }
            }
          }
          pub(crate) mod mangled {
           pub use super::r#VirtualMachineConfig as _7_android_6_system_21_virtualizationservice_20_VirtualMachineConfig;
           pub use super::r#Tag::r#Tag as _7_android_6_system_21_virtualizationservice_20_VirtualMachineConfig_3_Tag;
          }
        }
        pub mod VirtualMachineDebugInfo {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen/android/system/virtualizationservice/VirtualMachineDebugInfo.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice/VirtualMachineDebugInfo.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug)]
          pub struct r#VirtualMachineDebugInfo {
            pub r#cid: i32,
            pub r#temporaryDirectory: String,
            pub r#requesterUid: i32,
            pub r#requesterPid: i32,
            pub r#hostConsoleName: Option<String>,
          }
          impl Default for r#VirtualMachineDebugInfo {
            fn default() -> Self {
              Self {
                r#cid: 0,
                r#temporaryDirectory: Default::default(),
                r#requesterUid: 0,
                r#requesterPid: 0,
                r#hostConsoleName: Default::default(),
              }
            }
          }
          impl binder::Parcelable for r#VirtualMachineDebugInfo {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#cid)?;
                subparcel.write(&self.r#temporaryDirectory)?;
                subparcel.write(&self.r#requesterUid)?;
                subparcel.write(&self.r#requesterPid)?;
                subparcel.write(&self.r#hostConsoleName)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#cid = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#temporaryDirectory = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#requesterUid = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#requesterPid = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#hostConsoleName = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#VirtualMachineDebugInfo);
          binder::impl_deserialize_for_parcelable!(r#VirtualMachineDebugInfo);
          impl binder::binder_impl::ParcelableMetadata for r#VirtualMachineDebugInfo {
            fn get_descriptor() -> &'static str { "android.system.virtualizationservice.VirtualMachineDebugInfo" }
          }
          pub(crate) mod mangled {
           pub use super::r#VirtualMachineDebugInfo as _7_android_6_system_21_virtualizationservice_23_VirtualMachineDebugInfo;
          }
        }
        pub mod VirtualMachinePayloadConfig {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen/android/system/virtualizationservice/VirtualMachinePayloadConfig.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice/VirtualMachinePayloadConfig.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug)]
          pub struct r#VirtualMachinePayloadConfig {
            pub r#payloadBinaryName: String,
            pub r#extraApks: Vec<binder::ParcelFileDescriptor>,
          }
          impl Default for r#VirtualMachinePayloadConfig {
            fn default() -> Self {
              Self {
                r#payloadBinaryName: Default::default(),
                r#extraApks: Default::default(),
              }
            }
          }
          impl binder::Parcelable for r#VirtualMachinePayloadConfig {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#payloadBinaryName)?;
                subparcel.write(&self.r#extraApks)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#payloadBinaryName = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#extraApks = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#VirtualMachinePayloadConfig);
          binder::impl_deserialize_for_parcelable!(r#VirtualMachinePayloadConfig);
          impl binder::binder_impl::ParcelableMetadata for r#VirtualMachinePayloadConfig {
            fn get_descriptor() -> &'static str { "android.system.virtualizationservice.VirtualMachinePayloadConfig" }
          }
          pub(crate) mod mangled {
           pub use super::r#VirtualMachinePayloadConfig as _7_android_6_system_21_virtualizationservice_27_VirtualMachinePayloadConfig;
          }
        }
        pub mod VirtualMachineRawConfig {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen/android/system/virtualizationservice/VirtualMachineRawConfig.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice/VirtualMachineRawConfig.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug)]
          pub struct r#VirtualMachineRawConfig {
            pub r#name: String,
            pub r#instanceId: [u8; 64],
            pub r#kernel: Option<binder::ParcelFileDescriptor>,
            pub r#initrd: Option<binder::ParcelFileDescriptor>,
            pub r#params: Option<String>,
            pub r#bootloader: Option<binder::ParcelFileDescriptor>,
            pub r#disks: Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_9_DiskImage>,
            pub r#protectedVm: bool,
            pub r#memoryMib: i32,
            pub r#cpuTopology: crate::mangled::_7_android_6_system_21_virtualizationservice_11_CpuTopology,
            pub r#platformVersion: String,
            pub r#gdbPort: i32,
            pub r#hugePages: bool,
            pub r#devices: Vec<String>,
            pub r#displayConfig: Option<crate::mangled::_7_android_6_system_21_virtualizationservice_13_DisplayConfig>,
            pub r#inputDevices: Vec<crate::mangled::_7_android_6_system_21_virtualizationservice_11_InputDevice>,
            pub r#networkSupported: bool,
            pub r#consoleInputDevice: Option<String>,
            pub r#boostUclamp: bool,
            pub r#gpuConfig: Option<crate::mangled::_7_android_6_system_21_virtualizationservice_9_GpuConfig>,
            pub r#audioConfig: Option<crate::mangled::_7_android_6_system_21_virtualizationservice_11_AudioConfig>,
            pub r#noBalloon: bool,
            pub r#usbConfig: Option<crate::mangled::_7_android_6_system_21_virtualizationservice_9_UsbConfig>,
          }
          impl Default for r#VirtualMachineRawConfig {
            fn default() -> Self {
              Self {
                r#name: Default::default(),
                r#instanceId: [Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default()],
                r#kernel: Default::default(),
                r#initrd: Default::default(),
                r#params: Default::default(),
                r#bootloader: Default::default(),
                r#disks: Default::default(),
                r#protectedVm: false,
                r#memoryMib: 0,
                r#cpuTopology: crate::mangled::_7_android_6_system_21_virtualizationservice_11_CpuTopology::ONE_CPU,
                r#platformVersion: Default::default(),
                r#gdbPort: 0,
                r#hugePages: false,
                r#devices: Default::default(),
                r#displayConfig: Default::default(),
                r#inputDevices: Default::default(),
                r#networkSupported: false,
                r#consoleInputDevice: Default::default(),
                r#boostUclamp: false,
                r#gpuConfig: Default::default(),
                r#audioConfig: Default::default(),
                r#noBalloon: false,
                r#usbConfig: Default::default(),
              }
            }
          }
          impl binder::Parcelable for r#VirtualMachineRawConfig {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#name)?;
                subparcel.write(&self.r#instanceId)?;
                subparcel.write(&self.r#kernel)?;
                subparcel.write(&self.r#initrd)?;
                subparcel.write(&self.r#params)?;
                subparcel.write(&self.r#bootloader)?;
                subparcel.write(&self.r#disks)?;
                subparcel.write(&self.r#protectedVm)?;
                subparcel.write(&self.r#memoryMib)?;
                subparcel.write(&self.r#cpuTopology)?;
                subparcel.write(&self.r#platformVersion)?;
                subparcel.write(&self.r#gdbPort)?;
                subparcel.write(&self.r#hugePages)?;
                subparcel.write(&self.r#devices)?;
                subparcel.write(&self.r#displayConfig)?;
                subparcel.write(&self.r#inputDevices)?;
                subparcel.write(&self.r#networkSupported)?;
                subparcel.write(&self.r#consoleInputDevice)?;
                subparcel.write(&self.r#boostUclamp)?;
                subparcel.write(&self.r#gpuConfig)?;
                subparcel.write(&self.r#audioConfig)?;
                subparcel.write(&self.r#noBalloon)?;
                subparcel.write(&self.r#usbConfig)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#name = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#instanceId = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#kernel = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#initrd = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#params = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#bootloader = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#disks = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#protectedVm = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#memoryMib = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#cpuTopology = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#platformVersion = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#gdbPort = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#hugePages = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#devices = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#displayConfig = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#inputDevices = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#networkSupported = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#consoleInputDevice = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#boostUclamp = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#gpuConfig = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#audioConfig = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#noBalloon = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#usbConfig = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#VirtualMachineRawConfig);
          binder::impl_deserialize_for_parcelable!(r#VirtualMachineRawConfig);
          impl binder::binder_impl::ParcelableMetadata for r#VirtualMachineRawConfig {
            fn get_descriptor() -> &'static str { "android.system.virtualizationservice.VirtualMachineRawConfig" }
          }
          pub(crate) mod mangled {
           pub use super::r#VirtualMachineRawConfig as _7_android_6_system_21_virtualizationservice_23_VirtualMachineRawConfig;
          }
        }
        pub mod VirtualMachineState {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current -pout/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon_interface/preprocessed.aidl --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen/android/system/virtualizationservice/VirtualMachineState.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationservice-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationservice/VirtualMachineState.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #![allow(non_upper_case_globals)]
          use binder::declare_binder_enum;
          declare_binder_enum! {
            #[repr(C, align(4))]
            r#VirtualMachineState : [i32; 6] {
              r#NOT_STARTED = 0,
              r#STARTING = 1,
              r#STARTED = 2,
              r#READY = 3,
              r#FINISHED = 4,
              r#DEAD = 6,
            }
          }
          pub(crate) mod mangled {
           pub use super::r#VirtualMachineState as _7_android_6_system_21_virtualizationservice_19_VirtualMachineState;
          }
        }
      }
    }
  }
}
pub mod mangled {
  pub use super::aidl::android::system::virtualizationservice::AssignableDevice::mangled::*;
  pub use super::aidl::android::system::virtualizationservice::AudioConfig::mangled::*;
  pub use super::aidl::android::system::virtualizationservice::CpuTopology::mangled::*;
  pub use super::aidl::android::system::virtualizationservice::DiskImage::mangled::*;
  pub use super::aidl::android::system::virtualizationservice::DisplayConfig::mangled::*;
  pub use super::aidl::android::system::virtualizationservice::GpuConfig::mangled::*;
  pub use super::aidl::android::system::virtualizationservice::IVirtualMachine::mangled::*;
  pub use super::aidl::android::system::virtualizationservice::IVirtualMachineCallback::mangled::*;
  pub use super::aidl::android::system::virtualizationservice::IVirtualizationService::mangled::*;
  pub use super::aidl::android::system::virtualizationservice::InputDevice::mangled::*;
  pub use super::aidl::android::system::virtualizationservice::Partition::mangled::*;
  pub use super::aidl::android::system::virtualizationservice::PartitionType::mangled::*;
  pub use super::aidl::android::system::virtualizationservice::UsbConfig::mangled::*;
  pub use super::aidl::android::system::virtualizationservice::VirtualMachineAppConfig::mangled::*;
  pub use super::aidl::android::system::virtualizationservice::VirtualMachineConfig::mangled::*;
  pub use super::aidl::android::system::virtualizationservice::VirtualMachineDebugInfo::mangled::*;
  pub use super::aidl::android::system::virtualizationservice::VirtualMachinePayloadConfig::mangled::*;
  pub use super::aidl::android::system::virtualizationservice::VirtualMachineRawConfig::mangled::*;
  pub use super::aidl::android::system::virtualizationservice::VirtualMachineState::mangled::*;
  pub(crate) use android_system_virtualizationcommon::mangled::*;
}

#![allow(non_snake_case)]
#![allow(missing_docs)]
#[deprecated(note = "Please access via libbinder_rs binder::")]
pub use binder;
pub mod aidl {
  pub mod android {
    pub mod system {
      pub mod virtualizationcommon {
        pub mod Certificate {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon-rust-source/gen/android/system/virtualizationcommon/Certificate.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationcommon/Certificate.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug)]
          pub struct r#Certificate {
            pub r#encodedCertificate: Vec<u8>,
          }
          impl Default for r#Certificate {
            fn default() -> Self {
              Self {
                r#encodedCertificate: Default::default(),
              }
            }
          }
          impl binder::Parcelable for r#Certificate {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#encodedCertificate)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#encodedCertificate = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#Certificate);
          binder::impl_deserialize_for_parcelable!(r#Certificate);
          impl binder::binder_impl::ParcelableMetadata for r#Certificate {
            fn get_descriptor() -> &'static str { "android.system.virtualizationcommon.Certificate" }
          }
          pub(crate) mod mangled {
           pub use super::r#Certificate as _7_android_6_system_20_virtualizationcommon_11_Certificate;
          }
        }
        pub mod DeathReason {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon-rust-source/gen/android/system/virtualizationcommon/DeathReason.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationcommon/DeathReason.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #![allow(non_upper_case_globals)]
          use binder::declare_binder_enum;
          declare_binder_enum! {
            #[repr(C, align(4))]
            r#DeathReason : [i32; 16] {
              r#INFRASTRUCTURE_ERROR = 0,
              r#KILLED = 1,
              r#UNKNOWN = 2,
              r#SHUTDOWN = 3,
              r#START_FAILED = 4,
              r#REBOOT = 5,
              r#CRASH = 6,
              r#PVM_FIRMWARE_PUBLIC_KEY_MISMATCH = 7,
              r#PVM_FIRMWARE_INSTANCE_IMAGE_CHANGED = 8,
              r#MICRODROID_FAILED_TO_CONNECT_TO_VIRTUALIZATION_SERVICE = 11,
              r#MICRODROID_PAYLOAD_HAS_CHANGED = 12,
              r#MICRODROID_PAYLOAD_VERIFICATION_FAILED = 13,
              r#MICRODROID_INVALID_PAYLOAD_CONFIG = 14,
              r#MICRODROID_UNKNOWN_RUNTIME_ERROR = 15,
              r#HANGUP = 16,
              r#WATCHDOG_REBOOT = 17,
            }
          }
          pub(crate) mod mangled {
           pub use super::r#DeathReason as _7_android_6_system_20_virtualizationcommon_11_DeathReason;
          }
        }
        pub mod ErrorCode {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current --ninja -d out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon-rust-source/gen/android/system/virtualizationcommon/ErrorCode.rs.d -o out/soong/.intermediates/packages/modules/Virtualization/android/virtualizationservice/aidl/android.system.virtualizationcommon-rust-source/gen -Npackages/modules/Virtualization/android/virtualizationservice/aidl packages/modules/Virtualization/android/virtualizationservice/aidl/android/system/virtualizationcommon/ErrorCode.aidl
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #![allow(non_upper_case_globals)]
          use binder::declare_binder_enum;
          declare_binder_enum! {
            #[repr(C, align(4))]
            r#ErrorCode : [i32; 4] {
              r#UNKNOWN = 0,
              r#PAYLOAD_VERIFICATION_FAILED = 1,
              r#PAYLOAD_CHANGED = 2,
              r#PAYLOAD_INVALID_CONFIG = 3,
            }
          }
          pub(crate) mod mangled {
           pub use super::r#ErrorCode as _7_android_6_system_20_virtualizationcommon_9_ErrorCode;
          }
        }
      }
    }
  }
}
pub mod mangled {
  pub use super::aidl::android::system::virtualizationcommon::Certificate::mangled::*;
  pub use super::aidl::android::system::virtualizationcommon::DeathReason::mangled::*;
  pub use super::aidl::android::system::virtualizationcommon::ErrorCode::mangled::*;
}

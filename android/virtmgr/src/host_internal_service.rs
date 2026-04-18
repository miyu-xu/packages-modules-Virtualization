use android_system_virtualizationcommon::aidl::android::system::virtualizationcommon::Certificate::Certificate;
use android_system_virtualizationservice::aidl::android::system::virtualizationservice::{
    AssignableDevice::AssignableDevice,
    VirtualMachineDebugInfo::VirtualMachineDebugInfo,
};
use android_system_virtualizationservice_internal::aidl::android::system::virtualizationservice_internal::{
    IGlobalVmContext::BnGlobalVmContext,
    IBoundDevice::IBoundDevice,
    IGlobalVmContext::IGlobalVmContext,
    IVirtualizationServiceInternal::BnVirtualizationServiceInternal,
    IVirtualizationServiceInternal::IVirtualizationServiceInternal,
};
use binder::{
    BinderFeatures, ExceptionCode, Interface, ParcelFileDescriptor, SpIBinder, Status, Strong,
};
use std::collections::HashSet;
use std::fs::{create_dir_all, remove_dir_all};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{LazyLock, Mutex};

const HOST_CID_BASE: u32 = 2048;

pub fn global_service() -> Strong<dyn IVirtualizationServiceInternal> {
    static INSTANCE: LazyLock<Strong<dyn IVirtualizationServiceInternal>> = LazyLock::new(|| {
        BnVirtualizationServiceInternal::new_binder(
            HostVirtualizationServiceInternal::default(),
            BinderFeatures::default(),
        )
    });
    INSTANCE.clone()
}

#[derive(Default)]
struct HostVirtualizationServiceInternal {
    next_cid: AtomicU32,
    next_instance_id: AtomicU64,
    claimed_instance_ids: Mutex<HashSet<[u8; 64]>>,
}

impl HostVirtualizationServiceInternal {
    fn allocate_cid(&self) -> u32 {
        let next = self.next_cid.fetch_add(1, Ordering::Relaxed);
        if next == 0 {
            self.next_cid.store(HOST_CID_BASE + 1, Ordering::Relaxed);
            HOST_CID_BASE
        } else {
            HOST_CID_BASE + next - 1
        }
    }

    fn temp_dir_for_cid(cid: u32) -> binder::Result<PathBuf> {
        let path = std::env::temp_dir().join("virtmgr").join(cid.to_string());
        let _ = remove_dir_all(&path);
        create_dir_all(&path).map_err(|e| {
            Status::new_exception_str(
                ExceptionCode::ILLEGAL_STATE,
                Some(format!("Failed to create host temp dir {}: {e}", path.display())),
            )
        })?;
        Ok(path)
    }

    fn unsupported<T>(message: &str) -> binder::Result<T> {
        Err(Status::new_exception_str(
            ExceptionCode::UNSUPPORTED_OPERATION,
            Some(message.to_owned()),
        ))
    }
}

impl Interface for HostVirtualizationServiceInternal {}

impl IVirtualizationServiceInternal for HostVirtualizationServiceInternal {
    fn r#removeMemlockRlimit(&self) -> binder::Result<()> {
        Ok(())
    }

    fn r#allocateGlobalVmContext(
        &self,
        _arg_requesterDebugPid: i32,
    ) -> binder::Result<Strong<dyn IGlobalVmContext>> {
        let cid = self.allocate_cid();
        let temp_dir = Self::temp_dir_for_cid(cid)?;
        Ok(BnGlobalVmContext::new_binder(
            HostGlobalVmContext::new(cid, temp_dir),
            BinderFeatures::default(),
        ))
    }

    fn r#atomVmBooted(
        &self,
        _arg_atom: &android_system_virtualizationservice_internal::aidl::android::system::virtualizationservice_internal::AtomVmBooted::AtomVmBooted,
    ) -> binder::Result<()> {
        Ok(())
    }

    fn r#atomVmCreationRequested(
        &self,
        _arg_atom: &android_system_virtualizationservice_internal::aidl::android::system::virtualizationservice_internal::AtomVmCreationRequested::AtomVmCreationRequested,
    ) -> binder::Result<()> {
        Ok(())
    }

    fn r#atomVmExited(
        &self,
        _arg_atom: &android_system_virtualizationservice_internal::aidl::android::system::virtualizationservice_internal::AtomVmExited::AtomVmExited,
    ) -> binder::Result<()> {
        Ok(())
    }

    fn r#debugListVms(&self) -> binder::Result<Vec<VirtualMachineDebugInfo>> {
        Ok(vec![])
    }

    fn r#requestAttestation(
        &self,
        _arg_csr: &[u8],
        _arg_requesterUid: i32,
        _arg_testMode: bool,
    ) -> binder::Result<Vec<Certificate>> {
        Self::unsupported("Remote attestation is not implemented for the Windows host runtime")
    }

    fn r#enableTestAttestation(&self) -> binder::Result<()> {
        Ok(())
    }

    fn r#isRemoteAttestationSupported(&self) -> binder::Result<bool> {
        Ok(false)
    }

    fn r#getAssignableDevices(&self) -> binder::Result<Vec<AssignableDevice>> {
        Ok(vec![])
    }

    fn r#bindDevicesToVfioDriver(
        &self,
        _arg_devices: &[String],
    ) -> binder::Result<Vec<Strong<dyn IBoundDevice>>> {
        Self::unsupported("VFIO device assignment is not implemented for the Windows host runtime")
    }

    fn r#getDtboFile(&self) -> binder::Result<ParcelFileDescriptor> {
        Self::unsupported("DTBO access is not implemented for the Windows host runtime")
    }

    fn r#allocateInstanceId(&self) -> binder::Result<[u8; 64]> {
        let id = self.next_instance_id.fetch_add(1, Ordering::Relaxed) + 1;
        let mut instance_id = [0u8; 64];
        instance_id[..8].copy_from_slice(&id.to_le_bytes());
        self.claimed_instance_ids.lock().unwrap().insert(instance_id);
        Ok(instance_id)
    }

    fn r#removeVmInstance(&self, arg_instanceId: &[u8; 64]) -> binder::Result<()> {
        self.claimed_instance_ids.lock().unwrap().remove(arg_instanceId);
        Ok(())
    }

    fn r#claimVmInstance(&self, arg_instanceId: &[u8; 64]) -> binder::Result<()> {
        self.claimed_instance_ids.lock().unwrap().insert(*arg_instanceId);
        Ok(())
    }

    fn r#setDisplayService(&self, _arg_ibinder: &SpIBinder) -> binder::Result<()> {
        Self::unsupported(
            "Display service bridging is not implemented for the Windows host runtime",
        )
    }

    fn r#clearDisplayService(&self) -> binder::Result<()> {
        Ok(())
    }

    fn r#waitDisplayService(&self) -> binder::Result<SpIBinder> {
        Self::unsupported(
            "Display service bridging is not implemented for the Windows host runtime",
        )
    }

    fn r#createTapInterface(
        &self,
        _arg_ifaceNameSuffix: &str,
    ) -> binder::Result<ParcelFileDescriptor> {
        Self::unsupported("Tap networking is not implemented for the Windows host runtime")
    }

    fn r#deleteTapInterface(&self, _arg_tapFd: &ParcelFileDescriptor) -> binder::Result<()> {
        Ok(())
    }
}

struct HostGlobalVmContext {
    cid: u32,
    temp_dir: PathBuf,
    host_console_name: Mutex<Option<String>>,
}

impl HostGlobalVmContext {
    fn new(cid: u32, temp_dir: PathBuf) -> Self {
        Self { cid, temp_dir, host_console_name: Mutex::new(None) }
    }
}

impl Interface for HostGlobalVmContext {}

impl IGlobalVmContext for HostGlobalVmContext {
    fn r#getCid(&self) -> binder::Result<i32> {
        Ok(self.cid as i32)
    }

    fn r#getTemporaryDirectory(&self) -> binder::Result<String> {
        Ok(self.temp_dir.to_string_lossy().into_owned())
    }

    fn r#setHostConsoleName(&self, pathname: &str) -> binder::Result<()> {
        *self.host_console_name.lock().unwrap() = Some(pathname.to_owned());
        Ok(())
    }
}

impl Drop for HostGlobalVmContext {
    fn drop(&mut self) {
        if std::env::var("VIRTMGR_KEEP_TEMP")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
        {
            return;
        }
        let _ = remove_dir_all(&self.temp_dir);
    }
}

use kvm_bindings::RegList;
use kvm_ioctls::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Can not open kvm: {0}")]
    OpenKvm(kvm_ioctls::Error),
    #[error("Can not create vm: {0}")]
    CreateVm(kvm_ioctls::Error),
    #[error("Can not create vcpu: {0}")]
    CreateVcpu(kvm_ioctls::Error),
    #[error("Can not init vcpu: {0}")]
    InitVcpu(kvm_ioctls::Error),
    #[error("Can not finalize vcpu: {0}")]
    FinalizeVcpu(kvm_ioctls::Error),
    #[error("Can not get reg list: {0}")]
    GetRegList(kvm_ioctls::Error),
}

pub struct KvmVcpuWrapper {
    pub kvm: Kvm,
    pub vm: VmFd,
    pub vcpu: VcpuFd,
}

impl KvmVcpuWrapper {
    // This is a maximum allowed by kvm_bindings crate
    const REGISTERS_TO_QUERY: usize = 500;

    pub fn new() -> Result<Self, Error> {
        let kvm = Kvm::new().map_err(Error::OpenKvm)?;
        let vm = kvm.create_vm().map_err(Error::CreateVm)?;
        let vcpu = vm.create_vcpu(0).map_err(Error::CreateVcpu)?;

        let has_sve = vm.check_extension(Cap::ArmSve);

        let mut kvi = kvm_bindings::kvm_vcpu_init::default();
        vm.get_preferred_target(&mut kvi).unwrap();

        if has_sve {
            kvi.features[0] |= 1 << kvm_bindings::KVM_ARM_VCPU_SVE;
        }

        vcpu.vcpu_init(&kvi).map_err(Error::InitVcpu)?;

        if has_sve {
            let sve = kvm_bindings::KVM_ARM_VCPU_SVE as i32;
            vcpu.vcpu_finalize(&sve).map_err(Error::FinalizeVcpu)?;
        }

        Ok(Self { kvm, vm, vcpu })
    }

    pub fn query_registers(&self) -> Result<Vec<(u64, [u8; 2048])>, Error> {
        let mut reg_list = RegList::new(Self::REGISTERS_TO_QUERY).unwrap();
        let reg_list = match self.vcpu.get_reg_list(&mut reg_list) {
            Ok(_) => reg_list.as_slice(),
            Err(_) => {
                // if we fail to get Self::REGISTERS_TO_QUERY then the `n` in reg_list
                // will contain the correct number of registers to query
                reg_list = RegList::new(reg_list.as_fam_struct_ref().n as usize).unwrap();
                self.vcpu
                    .get_reg_list(&mut reg_list)
                    .map_err(Error::GetRegList)?;
                reg_list.as_slice()
            }
        };

        let ret = reg_list
            .iter()
            .map(|reg_id| {
                let mut reg_buf = [0u8; 2048];
                self.vcpu.get_one_reg(*reg_id, &mut reg_buf).unwrap();
                (*reg_id, reg_buf)
            })
            .collect();

        Ok(ret)
    }
}

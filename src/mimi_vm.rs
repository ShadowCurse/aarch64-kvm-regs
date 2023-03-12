use kvm_bindings::RegList;
use kvm_ioctls::*;

use crate::arm::{Aarch64KvmRegister, AARCH64_KVM_REGISTERS};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Can not open kvm: {0}")]
    OpenKvm(kvm_ioctls::Error),
    #[error("Can not create vm: {0}")]
    CreateVm(kvm_ioctls::Error),
    #[error("Can not create vcpu: {0}")]
    CreateVcpu(kvm_ioctls::Error),
    #[error("Can not get reg list: {0}")]
    GetRegList(kvm_ioctls::Error),
}

#[derive(Debug)]
pub struct KvmRegisterQuery {
    reg_id: u64,
    register: Option<Aarch64KvmRegister>,
}

pub struct KvmVcpuWrapper {
    pub kvm: Kvm,
    pub vm: VmFd,
    pub vcpu: VcpuFd,
}

impl KvmVcpuWrapper {
    const REGISTERS_TO_QUERY: usize = 1000;

    pub fn new() -> Result<Self, Error> {
        let kvm = Kvm::new().map_err(Error::OpenKvm)?;
        let vm = kvm.create_vm().map_err(Error::CreateVm)?;
        let vcpu = vm.create_vcpu(0).map_err(Error::CreateVcpu)?;

        Ok(Self { kvm, vm, vcpu })
    }

    pub fn query_registers(&self) -> Result<Vec<KvmRegisterQuery>, Error> {
        let mut reg_list = RegList::new(Self::REGISTERS_TO_QUERY).unwrap();
        self.vcpu
            .get_reg_list(&mut reg_list)
            .map_err(Error::GetRegList)?;
        let reg_list = reg_list.as_slice();

        assert!(Self::REGISTERS_TO_QUERY < reg_list.len());

        let regs = reg_list
            .iter()
            .map(|reg_id| KvmRegisterQuery {
                reg_id: *reg_id,
                register: AARCH64_KVM_REGISTERS
                    .iter()
                    .find(|kvm_reg| kvm_reg.reg_id == *reg_id)
                    .copied(),
            })
            .collect();
        Ok(regs)
    }
}

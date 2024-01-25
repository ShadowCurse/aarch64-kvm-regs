use serde::{Deserialize, Serialize};

pub const KVM_REG_SIZE_SHIFT: u32 = 52;
pub const KVM_REG_SIZE_MASK: u64 = 67553994410557440;

// Returns size of register in bits
pub fn reg_size(reg_id: u64) -> usize {
    2_usize.pow(((reg_id & KVM_REG_SIZE_MASK) >> KVM_REG_SIZE_SHIFT) as u32) * 8
}

#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aarch64KvmRegister {
    pub name: String,
    pub id: u64,
}

impl Aarch64KvmRegister {
    pub const fn new(name: String, op0: u64, op1: u64, crn: u64, crm: u64, op2: u64) -> Self {
        let id = Self::reg_id(op0, op1, crn, crm, op2);
        Self { name, id }
    }

    pub const fn reg_id(op0: u64, op1: u64, crn: u64, crm: u64, op2: u64) -> u64 {
        kvm_bindings::KVM_REG_ARM64
            | kvm_bindings::KVM_REG_SIZE_U64
            | kvm_bindings::KVM_REG_ARM64_SYSREG as u64
            | ((op0 << kvm_bindings::KVM_REG_ARM64_SYSREG_OP0_SHIFT)
                & kvm_bindings::KVM_REG_ARM64_SYSREG_OP0_MASK as u64)
            | ((op1 << kvm_bindings::KVM_REG_ARM64_SYSREG_OP1_SHIFT)
                & kvm_bindings::KVM_REG_ARM64_SYSREG_OP1_MASK as u64)
            | ((crn << kvm_bindings::KVM_REG_ARM64_SYSREG_CRN_SHIFT)
                & kvm_bindings::KVM_REG_ARM64_SYSREG_CRN_MASK as u64)
            | ((crm << kvm_bindings::KVM_REG_ARM64_SYSREG_CRM_SHIFT)
                & kvm_bindings::KVM_REG_ARM64_SYSREG_CRM_MASK as u64)
            | ((op2 << kvm_bindings::KVM_REG_ARM64_SYSREG_OP2_SHIFT)
                & kvm_bindings::KVM_REG_ARM64_SYSREG_OP2_MASK as u64)
    }
}

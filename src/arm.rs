#[derive(Debug)]
pub enum Access {
    RW,
    RO,
    WO,
}

#[allow(unused)]
#[derive(Debug)]
pub struct Aarch64KvmRegister {
    pub op0: u64,
    pub op1: u64,
    pub crn: u64,
    pub crm: u64,
    pub op2: u64,
    pub access: Access,
    pub mnemonic: &'static str,
    pub register: &'static str,
    pub reg_id: u64,
}

impl Aarch64KvmRegister {
    //https://elixir.bootlin.com/linux/v4.20.17/source/arch/arm64/include/uapi/asm/kvm.h#L203
    pub const KVM_REG_ARM64: u64 = 6917529027641081856;
    pub const KVM_REG_SIZE_U64: u64 = 13510798882111488;
    pub const KVM_REG_ARM64_SYSREG: u32 = 1245184;
    pub const KVM_REG_ARM64_SYSREG_OP0_MASK: u32 = 49152;
    pub const KVM_REG_ARM64_SYSREG_OP0_SHIFT: u32 = 14;
    pub const KVM_REG_ARM64_SYSREG_OP1_MASK: u32 = 14336;
    pub const KVM_REG_ARM64_SYSREG_OP1_SHIFT: u32 = 11;
    pub const KVM_REG_ARM64_SYSREG_CRN_MASK: u32 = 1920;
    pub const KVM_REG_ARM64_SYSREG_CRN_SHIFT: u32 = 7;
    pub const KVM_REG_ARM64_SYSREG_CRM_MASK: u32 = 120;
    pub const KVM_REG_ARM64_SYSREG_CRM_SHIFT: u32 = 3;
    pub const KVM_REG_ARM64_SYSREG_OP2_MASK: u32 = 7;
    pub const KVM_REG_ARM64_SYSREG_OP2_SHIFT: u32 = 0;

    pub const fn new(
        op0: u64,
        op1: u64,
        crn: u64,
        crm: u64,
        op2: u64,
        access: Access,
        mnemonic: &'static str,
        register: &'static str,
    ) -> Self {
        let reg_id = Self::reg_id(op0, op1, crn, crm, op2);
        Self {
            op0,
            op1,
            crn,
            crm,
            op2,
            access,
            mnemonic,
            register,
            reg_id,
        }
    }

    pub const fn reg_id(op0: u64, op1: u64, crn: u64, crm: u64, op2: u64) -> u64 {
        Self::KVM_REG_ARM64
            | Self::KVM_REG_SIZE_U64
            | Self::KVM_REG_ARM64_SYSREG as u64
            | ((op0 << Self::KVM_REG_ARM64_SYSREG_OP0_SHIFT)
                & Self::KVM_REG_ARM64_SYSREG_OP0_MASK as u64)
            | ((op1 << Self::KVM_REG_ARM64_SYSREG_OP1_SHIFT)
                & Self::KVM_REG_ARM64_SYSREG_OP1_MASK as u64)
            | ((crn << Self::KVM_REG_ARM64_SYSREG_CRN_SHIFT)
                & Self::KVM_REG_ARM64_SYSREG_CRN_MASK as u64)
            | ((crm << Self::KVM_REG_ARM64_SYSREG_CRM_SHIFT)
                & Self::KVM_REG_ARM64_SYSREG_CRM_MASK as u64)
            | ((op2 << Self::KVM_REG_ARM64_SYSREG_OP2_SHIFT)
                & Self::KVM_REG_ARM64_SYSREG_OP2_MASK as u64)
    }
}

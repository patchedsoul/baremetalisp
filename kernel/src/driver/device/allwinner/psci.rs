use super::cpu;
use super::memory;
use super::power;
use crate::driver::arm::scpi;
use crate::driver::psci::PsciResult;
use crate::driver::{psci, topology};

pub(crate) fn init() {
    cpu::init();
}

pub(crate) fn cpu_standby(_cpu_state: u8) {}

pub(crate) fn pwr_domain_on(mpidr: usize) -> PsciResult {
    // validation
    match topology::core_pos_by_mpidr(mpidr) {
        Some(_) => (),
        None => {
            return PsciResult::PsciEInternFail;
        }
    }

    if cpu::scpi_available() {
        scpi::set_css_power_state(
            mpidr,
            scpi::ScpiPowerState::PowerOn,
            scpi::ScpiPowerState::PowerOn,
            scpi::ScpiPowerState::PowerOn,
        );
    } else {
        cpu::cpu_on(mpidr);
    }

    PsciResult::PsciESuccess
}

pub(crate) fn pwr_domain_off(_target_state: &psci::PsciPowerState) {}

pub(crate) fn pwr_domain_suspend_pwrdown_early(_target_state: &psci::PsciPowerState) {}

pub(crate) fn pwr_domain_suspend(_target_state: &psci::PsciPowerState) {}

pub(crate) fn pwr_domain_on_finish(_target_state: &psci::PsciPowerState) {}

pub(crate) fn pwr_domain_on_finish_late(_target_state: &psci::PsciPowerState) {}

pub(crate) fn pwr_domain_suspend_finish(_target_state: &psci::PsciPowerState) {}

pub(crate) fn pwr_domain_pwr_down_wfi(_target_state: &psci::PsciPowerState) {}

pub(crate) fn system_off() {
    power::system_off();
}

pub(crate) fn system_reset() {}

pub(crate) fn validate_power_state(
    _power_state: usize,
    _req_state: &mut psci::PsciPowerState,
) -> isize {
    PsciResult::PsciENotSupported as isize
}

pub(crate) fn validate_ns_entrypoint(ns_entrypoint: usize) -> PsciResult {
    if ns_entrypoint >= memory::DRAM_BASE as usize {
        PsciResult::PsciESuccess
    } else {
        PsciResult::PsciEInvalidAddress
    }
}

pub(crate) fn get_sys_suspend_power_state(_target_state: &psci::PsciPowerState) {}

pub(crate) fn get_pwr_lvl_state_idx(_pwr_domain_state: u8, _pwrlvl: isize) -> isize {
    PsciResult::PsciENotSupported as isize
}

pub(crate) fn translate_power_state_by_mpidr(
    _mpidr: usize,
    _power_state: usize,
    _output_state: &mut psci::PsciPowerState,
) -> isize {
    PsciResult::PsciENotSupported as isize
}

pub(crate) fn get_node_hw_state(_mpidr: usize, _power_level: usize) -> isize {
    PsciResult::PsciENotSupported as isize
}

pub(crate) fn mem_protect_chk(_base: usize, _length: usize) -> isize {
    PsciResult::PsciENotSupported as isize
}

pub(crate) fn read_mem_protect(_val: &mut isize) -> isize {
    PsciResult::PsciENotSupported as isize
}

pub(crate) fn write_mem_protect(_val: isize) -> isize {
    PsciResult::PsciENotSupported as isize
}

pub(crate) fn system_reset2(_is_vendor: isize, _reset_type: isize, _cookie: u64) -> isize {
    PsciResult::PsciENotSupported as isize
}

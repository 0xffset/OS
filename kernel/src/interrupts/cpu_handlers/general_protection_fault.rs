use test_framework::serial_print;
use x86_64::structures::idt::InterruptStackFrame;

/// Handles a general protection fault.
pub extern "x86-interrupt" fn general_protection_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    serial_print!("GP Fault {},", _error_code);
}

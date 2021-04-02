pub mod interrupts;

pub fn init() {
    interrupts::init_idt();
}

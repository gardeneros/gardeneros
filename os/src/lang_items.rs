use crate::sbi::shutdown;
use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let err = info.message().unwrap();
    if let Some(location) = info.location() {
        println!("[Kernel] Panicked at {}:{} {}", location.file(), location.line(), err);
    } else {
        println!("[Kernel] Panicked: {}", err);
    }
    shutdown()
}

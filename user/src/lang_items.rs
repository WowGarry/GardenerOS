use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(panic_info: &PanicInfo) -> ! {
    let message = panic_info.message();
    let location = panic_info.location();

    if let Some(loc) = location {
        println!(
            "Panicked at {}:{}: {}",
            loc.file(),
            loc.line(),
            message
        );
    } else {
        println!("Panicked: {}", message);
    }

    loop {}
}

/// Until the bootloader is compiled separately from the kernel,
/// a panic handler is needed only for non-uefi boot as the uefi crate comes with its own panic handler.
#[cfg(not(feature = "boot_uefi"))]
mod not_boot_uefi_panic {
    use core::panic::PanicInfo;

    use crate::cpu::park_core;
    use crate::println;

    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        println!("{}", info);
        park_core();
    }
}

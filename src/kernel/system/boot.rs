use crate::main;

#[no_mangle]
pub extern "C" fn _start(mulitboot_information_address: usize) -> ! {
    main(mulitboot_information_address);

    loop {}
}

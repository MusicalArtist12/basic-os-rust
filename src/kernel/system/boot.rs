global_asm!(include_str!("bootloader.s"),  options(att_syntax));

static HELLO: &[u8] = b"Hello World!     ";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    main();

    loop {}
}

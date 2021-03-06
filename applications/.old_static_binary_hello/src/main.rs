// OFFICIAL REVONS OS REPO / REVONS OS
// revons company: contact.revons.co@support.co,
// fayssal chokri : contact.fayssal.revons@revons.co or
                //  contact.fayssal.chokri@developer.me

#![no_std]
#![no_main]
#![feature(lang_items)]


// extern crate alloc;
#[macro_use] extern crate log;
extern crate print;


fn main() {
    info!("Hello, world! (from hello application)");
    print::println("HELLO WORLD FROM HELLO APP!");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    main();

    loop { }
}





#[cfg(not(test))]
#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32, column: u32) -> ! {
    error!("\n\nPANIC in {} at line {}:{}:", file, line, column);
    error!("    {}", fmt);

    // TODO: check out Redox's unwind implementation: https://github.com/redox-os/kernel/blob/b364d052f20f1aa8bf4c756a0a1ea9caa6a8f381/src/arch/x86_64/interrupt/trace.rs#L9
    loop {}
}


#[cfg(not(test))]
#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {
    error!("\n\nin eh_personality, unimplemented!");
}


#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    error!("\n\nin _Unwind_Resume, unimplemented!");
    loop {}
}

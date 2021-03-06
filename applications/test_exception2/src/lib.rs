// OFFICIAL REVONS OS REPO / REVONS OS
// revons company: contact.revons.co@support.co,
// fayssal chokri : contact.fayssal.revons@revons.co or
                //  contact.fayssal.chokri@developer.me
#![no_std]


// extern crate alloc;
// #[macro_use] extern crate log;
extern crate logger;
// #[macro_use] extern crate terminal_print;
// extern crate task;


// use alloc::vec::Vec;
// use alloc::string::String;

#[derive(Debug)]
struct MyStruct2(pub usize);
impl MyStruct2 {
    #[inline(never)]
    pub fn new(val: usize) -> MyStruct2 {
        MyStruct2(val)
    }
}
impl Drop for MyStruct2 {
    #[inline(never)]
    fn drop(&mut self) {
        logger::write_str("\n*** DROPPING MYSTRUCT2 ***\n\n").unwrap();
    }
}

// pub fn main(_args: Vec<String>) -> isize {
pub fn main(val: usize) {

    // // dump some info about the this loaded app crate
    // {
    //     let curr_task = task::get_my_current_task().unwrap();
    //     let t = curr_task.lock();
    //     let app_crate = t.app_crate.as_ref().unwrap();
    //     let krate = app_crate.lock_as_ref();
    //     trace!("============== Crate {} =================", krate.crate_name);
    //     for s in krate.sections.values() {
    //         trace!("   {:?}", &*s.lock());
    //     }
    // }
    
    {
        let _my_struct2 = MyStruct2::new(val);

        {
            let _my_struct3 = MyStruct2::new(val + 10);
            let _res = logger::write_fmt(format_args!("{:?}\n", _my_struct3));
        }
        
        // cause page fault exception by dereferencing random memory value
        unsafe { *(0x5050DEADBEEF as *mut usize) = 0x5555_5555_5555; }
        let _res = logger::write_fmt(format_args!("{:?}\n", _my_struct2));
    }

    loop { }
    // 0
}

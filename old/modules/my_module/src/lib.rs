// my_module/src/lib.rs


pub trait Module {
    fn run(&self);

}

pub struct MyModule;

impl Module for MyModule {
    fn run(&self) {
        println!("Hello from the module!");
    }
}

#[no_mangle]
pub extern "C" fn create_module() -> *mut dyn Module {
    Box::into_raw(Box::new(MyModule)) as *mut dyn Module
}

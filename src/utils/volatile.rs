#[repr(transparent)]
pub struct Volatile<T>(T);

impl<T> Volatile<T>{
    pub fn new(val: T) -> Self { Volatile(val) }

    pub fn write(&mut self, val: T){
        // The value exists, so it's safe 
        unsafe {
            (&mut self.0 as *mut T).write_volatile(val);
        }
    }

    pub fn read(&self) -> T {
        // The value exists, so it's safe 
        unsafe {
            (&self.0 as *const T).read_volatile()
        }
    }

}

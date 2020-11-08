//! This file describes an abstraction over the InterruptDescriptorTable
//! 
//! Interrupts handler are setup in the `interrupt.rs` file

crate::import_commons!();

use core::marker::PhantomData;
use spin::Mutex;
pub static mut IDT: InterruptDescriptorTable = {
    InterruptDescriptorTable {
        divide_by_zero_excpt: Entry::empty(),
        debug_excpt: Entry::empty(),
        nmi_excpt: Entry::empty(),
        breakpoint_excpt: Entry::empty(),
        overflow_excpt: Entry::empty(),
        bound_range_excpt: Entry::empty(),
        invalid_opcode_excpt: Entry::empty(),
        device_not_available_excpt: Entry::empty(),
        double_fault_excpt: Entry::empty(),
        coproc_segment_overrun_excpt: Entry::empty(),
        invalid_tss_excpt: Entry::empty(),
        segment_not_present_excpt: Entry::empty(),
        stack_excpt: Entry::empty(),
        general_protection_excpt: Entry::empty(),
        page_fault_excpt: Entry::empty(),
        __reserved_excpt: Entry::empty(),
        floating_point_excpt: Entry::empty(),
        alignement_excpt: Entry::empty(),
        machine_check_excpt: Entry::empty(),
        simd_floating_point_excpt: Entry::empty(),
        control_protection_excpt: Entry::empty(),
        entries: [Entry::empty(); 235]
    }
};





#[derive(Debug)]
#[repr(C)]
pub struct Frame {
    pub ip: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segement: u64,
}


#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
/// https://wiki.osdev.org/Interrupt_Descriptor_Table#Structure_AMD64
pub struct Entry<T> {
    /// offset bits 0..15
    offset1:  u16,  
    /// code segment selector in GDT or LDT
    selector: u16,  
    /// bits 0..2 holds Interrupt Stack Table offset, rest of bits zero.
    ist: u8,         
    /// types and attributes: 
    /// - bits 0..3 : gate type, see `GateDescriptorType`
    /// - bit 4 : must be zero
    /// - bits 5..6 : descriptor privilege level
    /// - bit 7 : set if present
    type_attr: u8,  
    /// offset bits 16.31
    offset2: u16, 
    /// offset bits 32..63
    offset3: u32, 
    /// reserved
    zero: u32,
    __handler_type: core::marker::PhantomData<T>
}
#[repr(u8)]
pub enum GateDescriptorType {
    /// 
    Call = 0x0C,
    /// This is similar to a Trap 
    /// But interrupts will be disabled
    Interrupt = 0x0E,
    /// Interrupts are left enabled
    Trap = 0x0F
}

impl<T> Entry<T>  {


    pub const fn empty() -> Self {
        Self {
            offset1:0,
            selector:0,
            ist: 0,
            type_attr: 0b0000_1110,
            offset2: 0,
            offset3: 0,
            zero: 0,
            __handler_type: PhantomData
        }
    }
    /// Sets the gate descriptor's offset
    /// and mark the entry as present 
    /// 
    /// If the code segment selector is not set (=0)
    /// sets the active CS as the css
    pub fn set_addr(&mut self, offset: u64) -> &mut Self {
        self.offset1 = offset as u16;
        self.offset2 = (offset >> 16) as u16;
        self.offset3 = (offset >> 32) as u32;
        self.selector = if self.selector == 0 { 8 } else {0};
        self.set_present(true)
    }
    /// Marks whether the entry is present or not 
    pub fn set_present(&mut self, val: bool) -> &mut Self {
        self.type_attr = self.type_attr & 0x7F | (val as u8) << 7;
        self
    }
    
    /// Sets the segment descriptor
    pub fn set_cs(&mut self, selector: u16) -> &mut Self{
        self.selector = selector;
        self
    }
}

// This is just a cast from T to *const() for fn pointers
impl Entry<Handler> {
    pub fn set_handler(&mut self, func: Handler)-> &mut Self{
        self.set_addr(func as *const() as u64)
    }
}

// This is just a cast from T to *const() for fn pointers
impl Entry<HandlerEC> {
    pub fn set_handler(&mut self, func: HandlerEC)-> &mut Self{
        self.set_addr(func as *const() as u64)
    }
}

// This is just a cast from T to *const() for fn pointers
impl Entry<Abort> {
    pub fn set_handler(&mut self, func: Abort)-> &mut Self{
        self.set_addr(func as *const() as u64)
    }
}

// This is just a cast from T to *const() for fn pointers
impl Entry<AbortEC> {
    pub fn set_handler(&mut self, func: AbortEC)-> &mut Self{
        self.set_addr(func as *const() as u64)
    }
}

#[repr(C, packed)]
pub struct IDTLoader {
    size: u16,
    pointer: u64
}

pub type Handler = unsafe extern "x86-interrupt" fn(frame: &mut Frame) ;
pub type HandlerEC = unsafe extern "x86-interrupt" fn(frame: &mut Frame, error: u64);

pub type Abort = unsafe extern "x86-interrupt" fn(frame: &mut Frame) -> ! ;
pub type AbortEC = unsafe extern "x86-interrupt" fn(frame: &mut Frame, error: u64) -> !;


impl InterruptDescriptorTable {
    pub fn load(&self){
        let ptr = self as *const _ as u64;
        let size = core::mem::size_of::<Self>() as u16 -1;


        let loader =IDTLoader { size , pointer: ptr};
        
        unsafe { super::instructions::lidt(&loader as *const _ as u64 ); }
    }
}


#[repr(C, align(16))]
/// Represents an IDT
/// The first 21 
pub struct InterruptDescriptorTable {
    // see https://www.amd.com/system/files/TechDocs/24593.pdf
    // and https://wiki.osdev.org/Exceptions#Page_Fault for handler type
    // e.g : Fault or Trap -> Handler(EC if error code)
    //       abort -> Abort(EC if error code)
    ///Integer Divide-by-Zero Exception
    pub divide_by_zero_excpt: Entry<Handler>,
    ///Debug Exception
    pub debug_excpt: Entry<Handler>,
    ///Non-Maskable-Interrupt
    pub nmi_excpt: Entry<Handler>,
    ///Breakpoint Exception (INT 3)
    pub breakpoint_excpt: Entry<Handler>,
    ///Overflow Exception (INTO instruction)
    pub overflow_excpt: Entry<Handler>,
    ///Bound-Range Exception (BOUND instruction)
    pub bound_range_excpt: Entry<Handler>,
    ///Invalid-Opcode Exception
    pub invalid_opcode_excpt: Entry<Handler>,
    ///Device-Not-Available Exception
    pub device_not_available_excpt: Entry<Handler>,
    ///Double-Fault Exception
    pub double_fault_excpt: Entry<AbortEC>,
    ///Coprocessor-Segment-Overrun Exception (reserved in AMD64)
    pub coproc_segment_overrun_excpt: Entry<Handler>,
    ///Invalid-TSS Exception
    pub invalid_tss_excpt: Entry<HandlerEC>,
    ///Segment-Not-Present Exception
    pub segment_not_present_excpt: Entry<HandlerEC>,
    ///Stack Exception
    pub stack_excpt: Entry<HandlerEC>,
    ///General-Protection Exception
    pub general_protection_excpt: Entry<HandlerEC>,
    ///Page-Fault Exception
    pub page_fault_excpt: Entry<HandlerEC>,
    ///(Reserved)
    pub __reserved_excpt: Entry<Handler>,
    ///x87 Floating-Point Exception
    pub floating_point_excpt: Entry<Handler>,
    ///Alignment-Check Exception
    pub alignement_excpt: Entry<HandlerEC>,
    ///Machine-Check Exception
    pub machine_check_excpt: Entry<Abort>,
    ///SIMD Floating-Point Exception
    pub simd_floating_point_excpt: Entry<Handler>,
    ///Control-Protection Exception
    pub control_protection_excpt: Entry<Handler>,
    /// The rest of the entries (should be 256 total)
    pub entries: [Entry<Handler>; 256-21]
}


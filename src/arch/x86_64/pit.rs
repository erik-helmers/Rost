//! See https://wiki.osdev.org/Programmable_Interval_Timer#Outputs
//! 
//! The maths: 
//! The nominal frequency `nf` is 1.193182 Mhz
//! The reload value `rl` set specify how often an interrupt will be launched,
//! The resulting frequency is `nf`/rl, for a delay between interrupts of rl/`nf`*1000 ms 
//! 
//! To get a reload from a specified frequency 

use crate::arch::port::*;
use core::f64::*;

const CH0_DATA: Port<u8> = Port::new(0x40);
const CH1_DATA: Port<u8> = Port::new(0x41);
const CH2_DATA: Port<u8> = Port::new(0x42);
const MODE_PORT: Port<u8> = Port::new(0x43);

/// The nominal frequency of the PIT in hertz
const NOMINAL_FREQUENCY : f64 = 1_193_182 as f64;
const MINIMAL_FREQUENCY : f64 = NOMINAL_FREQUENCY / (u16::MAX as f64 + 1 as f64);
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum SelectChannel {
    Channel0 = 0x00,
    Channel1 = 0x40,
    Channel2 = 0x80,
    ReadBack = 0x0c,
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum AccessMode {
    LatchCountValue = 0x00,
    LobyteOnly = 0x10,
    HibyteOnly = 0x20,
    LobyteHibyte = 0x30,
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum OperatingMode {
    InterruptOnTerminalCount= 0x00 ,
    HardawareOneShot = 0x02,
    RateGenerator = 0x04,
    SquareWaveGenerator = 0x06,
    SoftwareTriggeredStrobe = 0x08,
    HardwareTriggeredStrobe = 0x0a,
    // RateGenerator (again)
    // SquareWaveGenerator (again)
}
#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub struct ModeCommand(u8);
impl ModeCommand {
    pub fn new(ch: SelectChannel, am: AccessMode, om: OperatingMode) -> Self {
        Self( ch as u8 | am as u8 | om as u8)
    }
}

pub struct Channel {
    /// The port linked to this channel
    port: Port<u8>,
    channel: SelectChannel,
    /// The last ModeCommand sent
    mode: ModeCommand,
    reload_value: Option<u16>
}

use crate::println;

impl Channel {

    /// Create a new channel struct by channel number 
    /// 
    /// Safety:
    /// Unsafe because the callee must ensure that the port is not written
    /// to anywhere else, because it assumes modes are not changed between calls
    pub  const unsafe fn new(ichannel: u8) -> Self {
        let (port, channel) = match ichannel {
            0 => (CH0_DATA, SelectChannel::Channel0),
            1 => (CH1_DATA, SelectChannel::Channel1),
            2 => (CH2_DATA, SelectChannel::Channel2),
            _ => panic!("Invalid channel number")
        };

        Self {
            port, 
            channel,
            mode: ModeCommand(0),
            reload_value: None,
            
        }
    }


    pub fn set_delay(&mut self, om:OperatingMode, delay:u32){
        todo!()        
    }

    /// Calculate a reload value given a frequency in hertz and set it
    /// 
    /// Also fuck it we use floating points
    pub fn set_frequency(&mut self, om:OperatingMode, freq: f64){

        assert!(freq > MINIMAL_FREQUENCY as f64, "Frequency is too low");
        assert!(freq < NOMINAL_FREQUENCY as f64, "Frequency is too high");
        // We calculate the reload value 
        let mut rl = (NOMINAL_FREQUENCY / freq) as u16;
        //let remainder = NOMINAL_FREQUENCY % freq;
        //if remainder > freq / 2 { rl += 1; }

        self.set_reload_value(om, rl as u16);
        
    }

    /// Sets the reload the value and 
    pub fn set_reload_value(&mut self, om: OperatingMode, mut rl: u16){
        self.reload_value = Some(rl);
        
        println!("rl set to {}, real freq {}Hz, delay {}ms", rl, NOMINAL_FREQUENCY as f64 / rl as f64, rl as f64 / NOMINAL_FREQUENCY as f64 * 1000 as f64);
        

        // On square wave generator we make sure the rl is pair 
        if let OperatingMode::SquareWaveGenerator = om {
            rl &= 0xFE; // Branchless programing !
        }
        unsafe {
            self.set_mode(AccessMode::LobyteHibyte, om);
            self.send_reload_value(rl);
        }
    }

    pub unsafe fn set_mode(&mut self, am: AccessMode, om: OperatingMode){
        let mode = ModeCommand::new(self.channel, am, om);
        if self.mode != mode {
            MODE_PORT.write(self.mode.0);
        }
    }

    /// Sets the reload value of the specified channel
    /// 
    /// Safety: 
    /// unsafe because it is expected that the channel
    /// is in lobyte/hibyte access mode
    pub unsafe fn send_reload_value(&self, rl: u16){
        // Write lobyte then hibyte
        self.port.write(rl as u8);
        self.port.write((rl >> 8) as u8);
    }
}





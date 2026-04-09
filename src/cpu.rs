use crate::byte;
use crate::cpu_memory;

pub struct Cpu
{
    pub memory: cpu_memory::CpuMemory,
    pub cycle: u32,
    pub extra_cycles: u32,
    pub a: Register8Bit,
    pub x: Register8Bit,
    pub y: Register8Bit,
    pub sp: Register8Bit,
    pub pc: Register16Bit,
    pub flags: FlagsRegister,
    //pub stack: Stack,
}

impl Cpu
{
    pub fn new(memory: cpu_memory::CpuMemory) -> Self
    {
        let sp = Register8Bit::new();
        Cpu
        {
            cycle: 0,
            extra_cycles: 0,
            a: Register8Bit::new(),
            x: Register8Bit::new(),
            y: Register8Bit::new(),
            //stack: Stack::new(sp),
            sp,
            pc: Register16Bit::new(),
            flags: FlagsRegister::new(),
            memory,
        }
    }
}

pub struct Register8Bit { value: u8, }

impl Register8Bit
{
    pub fn new() -> Self { Register8Bit { value: 0 } }

    pub fn get(&self) -> u8 { self.value }

    pub fn set(&mut self, value: u8) { self.value = value; }

    pub fn increment(&mut self)
    {
        self.value = self.value.wrapping_add(1);
    }
    pub fn decrement(&mut self)
    {
        self.value = self.value.wrapping_sub(1);
    }
}

pub struct Register16Bit { value: u16, }

impl Register16Bit
{
    pub fn new() -> Self { Register16Bit { value: 0 } }

    pub fn get(&self) -> u16 { self.value }

    pub fn set(&mut self, value: u16) { self.value = value; }

    pub fn increment(&mut self)
    {
        self.value = self.value.wrapping_add(1);
    }
    pub fn decrement(&mut self)
    {
        self.value = self.value.wrapping_sub(1);
    }
}

pub struct FlagsRegister
{
    pub c: bool,
    pub z: bool,
    pub i: bool,
    pub d: bool,
    pub v: bool,
    pub n: bool,
}

impl FlagsRegister
{
    pub fn new() -> Self
    {
        FlagsRegister
        {
            c: false,
            z: false,
            i: false,
            d: false,
            v: false,
            n: false,
        }
    }
    pub fn get(&self) -> u8
    {
        byte::bitfield(self.c as u8,self.z as u8,self.i as u8,self.d as u8,0,1,self.v as u8,self.n as u8)
    }
    pub fn set(&mut self, value: u8)
    {
        self.c = byte::get_flag(value,0);
        self.z = byte::get_flag(value,1);
        self.i = byte::get_flag(value,2);
        self.d = byte::get_flag(value,3);
        self.v = byte::get_flag(value,6);
        self.n = byte::get_flag(value,7);
    }
    pub fn update_zero(&mut self, value: u8)
    {
        self.z = value == 0;
    }
    pub fn update_negative(&mut self, value: u8)
    {
        self.n = byte::get_flag(value,7);
    }
    pub fn update_zero_and_negative(&mut self, value: u8)
    {
        self.update_zero(value);
        self.update_negative(value);
    }
}
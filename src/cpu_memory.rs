use crate::byte;
use crate::controller;

pub struct CpuMemory
{
    ram: [u8; 2048],
    //ppu:,
    //apu:,
    //mapper:,
    controllers: [Option<controller::Controller>; 2],
}

impl CpuMemory
{
    pub fn new() -> Self
    {
        CpuMemory 
        {
            ram: [0;2048],
            controllers: [None,None],
        }
    }
    pub fn read(&self, address: u16) -> u8
    {
        //WRAM (2 KiB)
        if address <= 0x07FF
        {
            return self.ram[address as usize];
        }

        // Mirrors of $0000-$07FF
        if address >= 0x0800 && address <= 0x1FFF
        {
            return self.read(0x0000 + (address - 0x0800) % 0x0800);
        }

        // PPU registers
        /* TODO: IMPLEMENT */

        // Mirrors of $2000-2007
        if address >= 0x2008 && address <= 0x3FFF
        {
            return self.read(0x2000 + (address - 0x2008) % 0x0008);
        }

        // APU registers
        /* TODO: IMPLEMENT */

        // PPU's OAMDMA register
        /* TODO: IMPLEMENT */

        // APUStatus register
        /* TODO: IMPLEMENT */

        // Controller port 1
        if address == 0x4016
        {
            //return self.controllers[0].as_mut().unwrap().onRead();
        }

        // Controller port 2
        if address == 0x4017
        {
            /* TODO: IMPLEMENT CONTROLLER */
        }

        // Cartridge space (PRG-ROM, mapper, etc.)
        if address >= 0x4020
        {
            /* TODO: IMPLEMENT MAPPER */
        }

        return 0;
    }

    pub fn write(&mut self, address: u16, value: u8)
    {
        //WRAM (2 KiB)
        if address <= 0x07FF
        {
            self.ram[address as usize] = value;
            return;
        }

        // Mirrors of $0000-$07FF
        if address >= 0x0800 && address <= 0x1FFF
        {
            return self.write(0x0000 + (address - 0x0800) % 0x0800,value);
        }

        // PPU registers
        /* TODO: IMPLEMENT */

        // Mirrors of $2000-2007
        if address >= 0x2008 && address <= 0x3FFF
        {
            return self.write(0x2000 + (address - 0x2008) % 0x0008,value);
        }

        // APU registers
        /* TODO: IMPLEMENT */

        // PPU's OAMDMA register
        /* TODO: IMPLEMENT */

        // APUStatus register
        /* TODO: IMPLEMENT */

        // Controller port 1
        if address == 0x4016
        {
            /* TODO: IMPLEMENT CONTROLLER */
        }

        // Controller port 2
        if address == 0x4017
        {
            /* TODO: IMPLEMENT CONTROLLER */
        }

        // Cartridge space (PRG-ROM, mapper, etc.)
        if address >= 0x4020
        {
            /* TODO: IMPLEMENT MAPPER */
        }
    }
    pub fn on_load(&mut self, controllers: [Option<controller::Controller>;2])
    {
        //self.ppu = ppu;
        //self.apu = apu;
        //self.mapper = mapper;
        self.controllers = controllers;
    }
    pub fn read16(&self, address: u16) -> u16
    {
        return byte::build_u16(self.read(address+1),self.read(address));
    }
}
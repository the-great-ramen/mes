use crate::byte;

enum MirroringType
{
    Vertical,
    Horizontal,
    FourScreen
}

pub struct NesHeader
{
    prg_rom_pages: u8,
    chr_rom_pages: u8,
    uses_chr_ram: bool,
    has_512_byte_padding: bool,
    has_prg_ram: bool,
    mirroring_id: MirroringType,
    mapper_id: u8,
}

pub struct Cartridge
{
    header: NesHeader,
    bytes: Vec<u8>,
}

impl Cartridge
{
    pub fn new(bytes: Vec<u8>) ->Result<Self, String>
    {
        let valid = [0x4E, 0x45, 0x53, 0x1A];
        if bytes[..4] != valid
        {
            return Err("Invalid ROM".to_string());
        }

        let prg_rom_pages = bytes[4];
        let chr_rom_pages = bytes[5];
        let uses_chr_ram = chr_rom_pages == 0;

        let flags6 = bytes[6];
        let flags7 = bytes[7];

        let has_512_byte_padding = (flags6 >> 2) & 1 == 1;
        let has_prg_ram = (flags6 >> 1) & 1 == 1;

        let mirroring_id = if(flags6 >> 3) & 1 == 1
        {
            MirroringType::FourScreen
        }
        else if flags6&1 == 0
        {
            MirroringType::Horizontal
        }
        else
        {
            MirroringType::Vertical
        };

        let mapper_id = (flags7 & 0xF0) | (flags6 >> 4);

        Ok(Self {
            header: NesHeader {
                prg_rom_pages,
                chr_rom_pages,
                uses_chr_ram,
                has_512_byte_padding,
                has_prg_ram,
                mirroring_id,
                mapper_id,
            },
            bytes,
        })
    }

    pub fn prg(&self) -> &[u8]
    {
        let start = if self.header.has_512_byte_padding {16+512} else {16};
        let size = 16384 * self.header.prg_rom_pages as usize;

        return &self.bytes[start..start+size];
    }
    pub fn chr(&self) -> Vec<u8>
    {
        if self.header.uses_chr_ram
        {
            return vec![0u8;8192];
        }
        let start = if self.header.has_512_byte_padding {16+512} else {16};
        let prg_size = 16384 * self.header.prg_rom_pages as usize;
        let chr_size = 8192 * self.header.chr_rom_pages as usize;
        return self.bytes[start + prg_size..start+prg_size+chr_size].to_vec();
    }
}
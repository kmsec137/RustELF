use std::{
	env::{self},
	fs::File,
	io::{self, Read},
	mem::{self},
	slice::{self},
	str::{self},
};

use byteorder::{
		LittleEndian, 
		BigEndian, 
		ReadBytesExt
};


struct EIdent {
	EI_MAG: [u8; 4],
	EI_CLASS: u8,
	EI_DATA: u8,
	EI_VERSION: u8,
	EI_OSABI: u8,
	EI_ABIVERSION: u8,
	EI_PAD: [u8; 7],
}

impl EIdent {
	fn default() -> EIdent {
		EIdent{
			EI_MAG:[0, 0, 0, 0],
			EI_CLASS:0,
			EI_DATA:0,
			EI_VERSION:0,
			EI_OSABI:0,
			EI_ABIVERSION:0,
			EI_PAD:[0; 7]}
	}
	fn from_reader(mut rdr: impl Read + Copy) -> io::Result<Self> {

		let mut _eident = EIdent::default();
		let mut _ei_mag = [0, 0, 0, 0];
		let mut _ei_pad = [0u8, 7];
		for _index in 0..(_ei_mag.len() as usize) {
			_ei_mag[_index] = rdr.read_u8()?;
		}
		
		_eident.EI_MAG = _ei_mag;	
		_eident.EI_CLASS = rdr.read_u8()?;
		_eident.EI_DATA = rdr.read_u8()?;
		_eident.EI_VERSION = rdr.read_u8()?;
		_eident.EI_OSABI = rdr.read_u8()?;
		_eident.EI_ABIVERSION = rdr.read_u8()?;

		for _index in 0..(_ei_pad.len() as usize){	
			_eident.EI_PAD[_index] = rdr.read_u8()?;
		}

		let mag_str: &str = str::from_utf8(&_eident.EI_MAG).unwrap();
		let pad_str: &str = str::from_utf8(&_eident.EI_PAD).unwrap();

		println!("[EIdent::from_reader] _eident.EI_MAG      '{}'",mag_str);
		println!("[EIdent::from_reader] _eident.EI_CLASS    '{:#02x}'",_eident.EI_CLASS);
		println!("[EIdent::from_reader] _eident.EI_DATA     '{:#02x}'",_eident.EI_DATA);
		println!("[EIdent::from_reader] _eident.EI_VERSION  '{:#02x}'",_eident.EI_VERSION);
		println!("[EIdent::from_reader] _eident.EI_OSABI  	 '{:#02x}'",_eident.EI_OSABI);
		println!("[EIdent::from_reader] _eident.EI_ABIVERSION  '{:#02x}'",_eident.EI_ABIVERSION);
		println!("[EIdent::from_reader] _eident.EI_PAD  '{}'",pad_str);

		Ok(_eident)
	}
}
enum EEntry {
	e_entry32(u32),
	e_entry64(u64)
}

enum EPHoff {
	phoff32(u32),
	phoff64(u64),
}

enum ESHoff {
	shoff32(u32),	
	shoff64(u64),
}


struct ELFHeader {
	e_ident: EIdent,
	e_type: u16,
	e_machine: u16,
	e_version: u32,
	e_entry: EEntry,
	e_phoff: EPHoff,
	e_shoff: ESHoff,
	e_flags: u32,
	e_ehsize: u16,
	e_phnum: u16,
	e_shentsize: u16,
	e_shnum: u16,
	e_shstrndx: u16,	
}
impl ELFHeader {
	fn default () -> ELFHeader {
			ELFHeader{
				e_ident: EIdent::default(),
				e_type: 0,
				e_machine: 0,
				e_version: 0,
				e_entry: EEntry::e_entry64(0),
				e_phoff: EPHoff::phoff64(0),
				e_shoff: ESHoff::shoff64(0),
				e_flags: 0,
				e_ehsize: 0,
				e_phnum: 0,
				e_shentsize: 0,
				e_shnum: 0,
				e_shstrndx: 0,	
			}
	}	
	fn from_reader(mut rdr : impl Read + Copy) -> io::Result<Self> {

		let _elf_header: ELFHeader = ELFHeader::default();
		let _e_ident: EIdent = EIdent::from_reader(rdr)?;
		let _e_type:    u16 = rdr.read_u16::<LittleEndian>()?;
		let _e_machine: u16 = rdr.read_u16::<LittleEndian>()?;
		let _e_version: u32 = rdr.read_u32::<LittleEndian>()?;

		Ok(_elf_header)
	}
}
fn main() {
	let mut _args: Vec<String> = env::args().collect(); 
	if _args.len() < 2 {
		return
	}
	const header_size: usize = mem::size_of::<ELFHeader>();
	let mut _read_buffer: Vec<u8> = vec![0u8; header_size];

	let file: std::fs::File = File::open(&_args[1]).unwrap();
	println!("Reading ELF... {}\n",&_args[1]);
	let mut _header = ELFHeader::from_reader(&file);
}

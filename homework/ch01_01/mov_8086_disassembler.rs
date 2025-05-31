//! This is a small program that demonstrates our ability to disassemble a register to register MOV oepration in 8086 machine code.

use std::fs::File;
use std::io::Read;
use std::process;
use std::mem::swap;

const _8086_OP_MASK:            u8 = 0b11111100; const _8086_OP_SHIFT: u8 = 2;

const _8086_MOV_OPCODE:         u8 = 0b100010;
const _8086_MOV_D_MASK:         u8 = 0b00000010;
const _8086_MOV_W_MASK:         u8 = 0b00000001;
const _8086_MOV_MOD_MASK:       u8 = 0b11000000; const _8086_MOV_MOD_SHIFT: u8 = 6;
const _8086_MOV_MOD_REG_TO_REG: u8 = 0b11;
const _8086_MOV_REG_MASK:       u8 = 0b00111000; const _8086_MOV_REG_SHIFT: u8 = 3;
const _8086_MOV_REG_AX_AL:      u8 = 0b000;
const _8086_MOV_REG_CX_CL:      u8 = 0b001;
const _8086_MOV_REG_DX_DL:      u8 = 0b010;
const _8086_MOV_REG_BX_BL:      u8 = 0b011;
const _8086_MOV_REG_SP_AH:      u8 = 0b100;
const _8086_MOV_REG_BP_CH:      u8 = 0b101;
const _8086_MOV_REG_SI_DH:      u8 = 0b110;
const _8086_MOV_REG_DI_BH:      u8 = 0b111;
const _8086_MOV_R_M_MASK:       u8 = 0b00000111; const _8086_MOV_R_M_SHIFT: u8 = 0;

fn get_reg_name<'a>(reg_operand: u8, is_wide: bool) -> &'a str
{
	if reg_operand == _8086_MOV_REG_AX_AL { return if is_wide {"ax"} else {"al"}; }
	if reg_operand == _8086_MOV_REG_CX_CL { return if is_wide {"cx"} else {"cl"}; }
	if reg_operand == _8086_MOV_REG_DX_DL { return if is_wide {"dx"} else {"dl"}; }
	if reg_operand == _8086_MOV_REG_BX_BL { return if is_wide {"bx"} else {"bl"}; }
	if reg_operand == _8086_MOV_REG_SP_AH { return if is_wide {"sp"} else {"ah"}; }
	if reg_operand == _8086_MOV_REG_BP_CH { return if is_wide {"bp"} else {"ch"}; }
	if reg_operand == _8086_MOV_REG_SI_DH { return if is_wide {"si"} else {"dh"}; }
	if reg_operand == _8086_MOV_REG_DI_BH { return if is_wide {"di"} else {"bh"}; }
	else { return "??"; }
}

#[allow(unused)]
#[allow(non_snake_case)]
fn main()
{
	let file_path = "listings\\listing38";
	
	let mut buffer = Vec::new();
	{
		let mut file = File::open(file_path).unwrap();
		if let Err(e) = file.read_to_end(&mut buffer)
		{
			eprintln!("Failed to read file '{}': {}", file_path, e);
			process::exit(1);
		}
	}
	
	// (Optional) Do something with the binary data
	// For example: print first few bytes in hex
	let mut bIndex = 0;
	while bIndex < buffer.len()
	{
		let byte = buffer[bIndex];
		let opcode: u8 = ((byte & _8086_OP_MASK) >> _8086_OP_SHIFT);
		let d: bool = ((byte & _8086_MOV_D_MASK) != 0);
		let w: bool = ((byte & _8086_MOV_W_MASK) != 0);
		// println!("opcode={:06b}, d={} w={}", opcode, if d {"1"} else {"0"}, if d {"1"} else {"0"});
		
		if (opcode == _8086_MOV_OPCODE)
		{
			bIndex += 1;
			let next_byte = buffer.get(bIndex).expect("MOV opcode expected 1 more byte, but we reached the end of the file!");
			let mode = ((next_byte & _8086_MOV_MOD_MASK) >> _8086_MOV_MOD_SHIFT);
			let mut reg = ((next_byte & _8086_MOV_REG_MASK) >> _8086_MOV_REG_SHIFT);
			let mut r_m = ((next_byte & _8086_MOV_R_M_MASK) >> _8086_MOV_R_M_SHIFT);
			if (!d) { swap(&mut reg, &mut r_m); }
			// println!("mode=0b{:02b} reg=0b{:03b} r_m=0b{:03b}", mode, reg, r_m);
			
			if (mode == _8086_MOV_MOD_REG_TO_REG)
			{
				// TODO: Idk why we have to swap the order of r_m and reg in order to match the input .asm file!
				println!("mov {}, {}", get_reg_name(reg, w), get_reg_name(r_m, w));
			}
			else
			{
				// Unhandled mod value, probably means to read/write to memory or immediate
				println!("mov({:02b}) {:03b}, {:03b}", mode, reg, r_m);
			}
		}
		else
		{
			println!("Unknown opcode: 0b{:06b} 0x{:02X}", opcode, opcode);
		}
		
		bIndex += 1;
	}
}
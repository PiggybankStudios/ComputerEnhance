//! This is a small program that demonstrates our ability to disassemble a register to register MOV operation in 8086 machine code.

use std::env;
use std::fs;
use std::io::Read;
use std::process::exit;

const DO_SECOND_LISTING: bool = true;

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

fn get_reg_name<'a>(reg_operand: u8, is_word_size: bool) -> &'a str
{
	if reg_operand == _8086_MOV_REG_AX_AL { return if is_word_size {"ax"} else {"al"}; }
	if reg_operand == _8086_MOV_REG_CX_CL { return if is_word_size {"cx"} else {"cl"}; }
	if reg_operand == _8086_MOV_REG_DX_DL { return if is_word_size {"dx"} else {"dl"}; }
	if reg_operand == _8086_MOV_REG_BX_BL { return if is_word_size {"bx"} else {"bl"}; }
	if reg_operand == _8086_MOV_REG_SP_AH { return if is_word_size {"sp"} else {"ah"}; }
	if reg_operand == _8086_MOV_REG_BP_CH { return if is_word_size {"bp"} else {"ch"}; }
	if reg_operand == _8086_MOV_REG_SI_DH { return if is_word_size {"si"} else {"dh"}; }
	if reg_operand == _8086_MOV_REG_DI_BH { return if is_word_size {"di"} else {"bh"}; }
	else { return "??"; }
}

#[allow(unused)]
#[allow(non_snake_case)]
fn main()
{
	let file_path = if DO_SECOND_LISTING {"listings\\listing38"} else {"listings\\listing37"};
	
	let mut buffer = Vec::new();
	{
		let mut file = fs::File::open(file_path).unwrap();
		if let Err(e) = file.read_to_end(&mut buffer)
		{
			eprintln!("Failed to read file '{}': {}", file_path, e);
			exit(1);
		}
	}
	
	let args: Vec<String> = env::args().collect();
	let program_path: &String = args.get(0).unwrap();
	println!("; ========================================================================");
	println!("; This file is disassembled by {}", program_path);
	println!("; ========================================================================");
	println!();
	println!("bits 16");
	println!();
	
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
			let reg = ((next_byte & _8086_MOV_REG_MASK) >> _8086_MOV_REG_SHIFT);
			let r_m = ((next_byte & _8086_MOV_R_M_MASK) >> _8086_MOV_R_M_SHIFT);
			let dst = if d {reg} else {r_m};
			let src = if d {r_m} else {reg};
			// println!("mode=0b{:02b} reg=0b{:03b} r_m=0b{:03b}", mode, reg, r_m);
			
			if (mode == _8086_MOV_MOD_REG_TO_REG)
			{
				println!("mov {}, {}", get_reg_name(dst, w), get_reg_name(src, w));
			}
			else
			{
				// Unhandled mod value, probably means to read/write to memory or immediate
				println!("mov({:02b}) {:03b}, {:03b}", mode, dst, src);
			}
		}
		else
		{
			eprintln!("Unknown opcode: 0b{:06b} 0x{:02X}", opcode, opcode);
			exit(2);
		}
		
		bIndex += 1;
	}
}
//! This is a small program that demonstrates our ability to disassemble a register to register MOV operation in 8086 machine code.

use std::env;
use std::fs;
use std::io::Read;
use std::process::exit;

const DO_SECOND_LISTING: bool = false;

// const BIT7_MASK:    u8 = 0b10000000;
// const BIT6_MASK:    u8 = 0b01000000;
// const BIT5_MASK:    u8 = 0b00100000;
// const BIT4_MASK:    u8 = 0b00010000;
const BIT3_MASK:    u8 = 0b00001000;
// const BIT2_MASK:    u8 = 0b00000100;
const BIT1_MASK:    u8 = 0b00000010;
const BIT0_MASK:    u8 = 0b00000001;
const BITS210_MASK: u8 = 0b00000111;

const _8086_OP_MASK4: u8 = 0b11110000; const _8086_OP_SHIFT4: u8 = 4;
const _8086_OP_MASK6: u8 = 0b11111100; const _8086_OP_SHIFT6: u8 = 2;
const _8086_OP_MASK7: u8 = 0b11111110; const _8086_OP_SHIFT7: u8 = 1;

const _8086_MOV_IM_REG_OPCODE:        u8 = 0b1011; //Immediate to register
const _8086_MOV_REGMEM_REGMEM_OPCODE: u8 = 0b100010; //Register/memory to/from register
const _8086_MOV_IM_MEM_OPCODE:        u8 = 0b1100011; //Immediate to register/memory
const _8086_MOV_MEM_ACCUM_OPCODE:     u8 = 0b1010000; //Memory to accumulator

const _8086_MOV_MOD_MASK:           u8 = 0b11000000; const _8086_MOV_MOD_SHIFT: u8 = 6;
const _8086_MOV_MOD_MEM_0BIT:       u8 = 0b00; //no offset bytes (except for if R/M is DIRECT ADDRESS)
const _8086_MOV_MOD_MEM_8BIT:       u8 = 0b01; //1-byte offset
const _8086_MOV_MOD_MEM_16BIT:      u8 = 0b10; //2-byte offset
const _8086_MOV_MOD_REG_TO_REG:     u8 = 0b11; //no offset bytes, because no memory address, mov between registers
const _8086_MOV_REG_MASK:           u8 = 0b00111000; const _8086_MOV_REG_SHIFT: u8 = 3;
const _8086_MOV_REG_AX_AL:          u8 = 0b000; //if w=1 ax else al
const _8086_MOV_REG_CX_CL:          u8 = 0b001; //if w=1 cx else cl
const _8086_MOV_REG_DX_DL:          u8 = 0b010; //if w=1 dx else dl
const _8086_MOV_REG_BX_BL:          u8 = 0b011; //if w=1 bx else bl
const _8086_MOV_REG_SP_AH:          u8 = 0b100; //if w=1 sp else ah
const _8086_MOV_REG_BP_CH:          u8 = 0b101; //if w=1 bp else ch
const _8086_MOV_REG_SI_DH:          u8 = 0b110; //if w=1 si else dh
const _8086_MOV_REG_DI_BH:          u8 = 0b111; //if w=1 di else bh
const _8086_MOV_R_M_MASK:           u8 = 0b00000111; const _8086_MOV_R_M_SHIFT: u8 = 0;
const _8086_MOV_EFF_ADDR_EQ_BX_SI:  u8 = 0b000;
const _8086_MOV_EFF_ADDR_EQ_BX_DI:  u8 = 0b001;
const _8086_MOV_EFF_ADDR_EQ_BP_SI:  u8 = 0b010;
const _8086_MOV_EFF_ADDR_EQ_BP_DI:  u8 = 0b011;
const _8086_MOV_EFF_ADDR_EQ_SI:     u8 = 0b100;
const _8086_MOV_EFF_ADDR_EQ_DI:     u8 = 0b101;
const _8086_MOV_EFF_ADDR_EQ_DIRECT: u8 = 0b110;
const _8086_MOV_EFF_ADDR_EQ_BX:     u8 = 0b111;

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

fn get_effective_address_equation_str<'a>(mem_operand: u8) -> &'a str
{
	if mem_operand == _8086_MOV_EFF_ADDR_EQ_BX_SI  { return "bx+si"; }
	if mem_operand == _8086_MOV_EFF_ADDR_EQ_BX_DI  { return "bx+di"; }
	if mem_operand == _8086_MOV_EFF_ADDR_EQ_BP_SI  { return "bp+si"; }
	if mem_operand == _8086_MOV_EFF_ADDR_EQ_BP_DI  { return "bp+di"; }
	if mem_operand == _8086_MOV_EFF_ADDR_EQ_SI     { return "si";    }
	if mem_operand == _8086_MOV_EFF_ADDR_EQ_DI     { return "di";    }
	if mem_operand == _8086_MOV_EFF_ADDR_EQ_DIRECT { return "bp";    }
	if mem_operand == _8086_MOV_EFF_ADDR_EQ_BX     { return "bx";    }
	else { return "??"; }
}

#[allow(unused)]
#[allow(non_snake_case)]
fn main()
{
	let file_path = if DO_SECOND_LISTING {"listings\\listing40"} else {"listings\\listing39"};
	
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
		let opcode4 = ((byte & _8086_OP_MASK4) >> _8086_OP_SHIFT4);
		let opcode6 = ((byte & _8086_OP_MASK6) >> _8086_OP_SHIFT6);
		let opcode7 = ((byte & _8086_OP_MASK7) >> _8086_OP_SHIFT7);
		
		if (opcode4 == _8086_MOV_IM_REG_OPCODE)
		{
			let w: bool = ((byte & BIT3_MASK) != 0);
			let reg = (byte & BITS210_MASK);
			
			let mut immediate_data: u16 = 0x00;
			if (w)
			{
				bIndex += 1; let data_lo: u8 = *buffer.get(bIndex).expect("MOV opcode expected 2 more byte, but we reached the end of the file!");
				bIndex += 1; let data_hi: u8 = *buffer.get(bIndex).expect("MOV opcode expected 2 more byte, but we reached the end of the file!");
				immediate_data = ((data_hi as u16) << 8) | (data_lo as u16);
			}
			else
			{
				bIndex += 1;
				immediate_data = (*buffer.get(bIndex).expect("MOV opcode expected 1 more byte, but we reached the end of the file!")) as u16;
			}
			
			println!("mov {}, {}", get_reg_name(reg, w), immediate_data);
		}
		else if (opcode6 == _8086_MOV_REGMEM_REGMEM_OPCODE)
		{
			let d: bool = ((byte & BIT1_MASK) != 0);
			let w: bool = ((byte & BIT0_MASK) != 0);
			// println!("opcode={:06b}, d={} w={}", opcode, if d {"1"} else {"0"}, if d {"1"} else {"0"});
			
			bIndex += 1;
			let next_byte = *buffer.get(bIndex).expect("MOV opcode expected 1 more byte, but we reached the end of the file!");
			let mode = ((next_byte & _8086_MOV_MOD_MASK) >> _8086_MOV_MOD_SHIFT);
			let reg = ((next_byte & _8086_MOV_REG_MASK) >> _8086_MOV_REG_SHIFT);
			let r_m = ((next_byte & _8086_MOV_R_M_MASK) >> _8086_MOV_R_M_SHIFT);
			// println!("mode=0b{:02b} reg=0b{:03b} r_m=0b{:03b}", mode, reg, r_m);
			
			let mut disp: u16 = 0x0000; //offset value in effective address calculation
			if (mode == _8086_MOV_MOD_MEM_8BIT)
			{
				bIndex += 1; disp = (*buffer.get(bIndex).expect("MOV opcode expected 2 more bytes, but we reached the end of the file!")) as u16;
			}
			else if (mode == _8086_MOV_MOD_MEM_16BIT || (mode == _8086_MOV_MOD_MEM_0BIT && r_m == _8086_MOV_EFF_ADDR_EQ_DIRECT))
			{
				bIndex += 1; let disp_lo: u8 = *buffer.get(bIndex).expect("MOV opcode expected 3 more bytes, but we reached the end of the file!");
				bIndex += 1; let disp_hi: u8 = *buffer.get(bIndex).expect("MOV opcode expected 3 more bytes, but we reached the end of the file!");
				disp = ((disp_hi as u16) << 8) | (disp_lo as u16);
			}
			
			if (mode == _8086_MOV_MOD_REG_TO_REG)
			{
				let dst = if d {reg} else {r_m};
				let src = if d {r_m} else {reg};
				println!("mov {}, {}", get_reg_name(dst, w), get_reg_name(src, w));
			}
			else if (mode == _8086_MOV_MOD_MEM_0BIT && r_m != _8086_MOV_EFF_ADDR_EQ_DIRECT)
			{
				//TODO: We probably want to merge all these if (d) else blocks by doing some sort of format on each operand and then printing them out one way or the other
				if (d) //if writing to register
				{
					println!("mov {}, [{}]", get_reg_name(reg, w), get_effective_address_equation_str(r_m));
				}
				else //else writing to memory
				{
					println!("mov [{}], {}", get_effective_address_equation_str(r_m), get_reg_name(reg, w));
				}
			}
			else if (mode == _8086_MOV_MOD_MEM_0BIT && r_m == _8086_MOV_EFF_ADDR_EQ_DIRECT)
			{
				if (d) //if writing to register
				{
					println!("mov {}, [{}]", get_reg_name(reg, w), disp);
				}
				else //else writing to memory
				{
					println!("mov [{}], {}", disp, get_reg_name(reg, w));
				}
			}
			else if (mode == _8086_MOV_MOD_MEM_8BIT || mode == _8086_MOV_MOD_MEM_16BIT)
			{
				if (d) //if writing to register
				{
					println!("mov {}, [{}+{}]", get_reg_name(reg, w), get_effective_address_equation_str(r_m), disp);
				}
				else //else writing to memory
				{
					println!("mov [{}+{}], {}", get_effective_address_equation_str(r_m), disp, get_reg_name(reg, w));
				}
			}
			else
			{
				// Unhandled mod value, probably means to read/write to memory or immediate
				println!("mov({:02b}) {:03b}, {:03b}", mode, reg, r_m);
			}
		}
		else if (opcode7 == _8086_MOV_IM_MEM_OPCODE)
		{
			println!("mov immediate to memory...");
		}
		else if (opcode7 == _8086_MOV_MEM_ACCUM_OPCODE)
		{
			println!("mov memory to accumulator...");
		}
		else
		{
			eprintln!("Unknown opcode: 0b{:07b} 0x{:02X}", opcode7, opcode7);
			exit(2);
		}
		
		bIndex += 1;
	}
}
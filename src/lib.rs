#![allow(warnings)]

pub mod decoders;
pub mod mem;
pub mod disassembler;

use std::sync::Arc;

use decoders::x86_decoder::*;
use decoders::x86_decoder::X86Instruction::*;

use decoders::decoder::Decoder;
use decoders::decoder::Instruction;

type Bytes = Vec<u8>;

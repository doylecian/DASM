pub mod decoders;

use decoders::x86_decoder::*;
use decoders::x86_decoder::X86Instruction::*;

use decoders::decoder::Decoder;
use decoders::decoder::Instruction;


#[cfg(test)]
mod tests {
    use decoders::decoder::Instruction;

    use super::*;

    #[test]
    fn decode_a_byte_array_x86() {
        let decoder = X86Decoder;
        let decoded: Vec<X86Instruction> = decoder.decode(&[0xE9, 0x90, 123]);
        println!("{:?}", decoded);
        assert_eq!(decoded, vec![JMP, NOP, ERR]);
    }
}

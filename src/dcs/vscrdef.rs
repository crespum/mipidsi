//! Module for the VSCRDEF visual scroll definition instruction constructors

use crate::{instruction::Instruction, Error};

use super::DcsCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vscrdef {
    tfa: u16,
    vsa: u16,
    bfa: u16,
}

impl Vscrdef {
    ///
    /// Construct a new Vscrdef with zero TFA and VSA
    /// and the specified vsa. This should default to the display's
    /// height (or width) framebuffer size.
    /// 
    pub fn new(vsa: u16) -> Self {
        Self {
            tfa: 0,
            vsa,
            bfa: 0,
        }
    }
}

impl DcsCommand for Vscrdef {
    fn instruction(&self) -> Instruction {
        Instruction::VSCRDEF
    }

    fn fill_params_buf(&self, buffer: &mut [u8]) -> Result<usize, Error> {
        let tfa_bytes = self.tfa.to_be_bytes();
        let vsa_bytes = self.vsa.to_be_bytes();
        let bfa_bytes = self.bfa.to_be_bytes();

        buffer[0] = tfa_bytes[0];
        buffer[1] = tfa_bytes[1];
        buffer[2] = vsa_bytes[0];
        buffer[3] = vsa_bytes[1];
        buffer[4] = bfa_bytes[0];
        buffer[5] = bfa_bytes[1];

        Ok(6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vscrdef_fills_buffer_properly() -> Result<(), Error> {
        let vscrdef = Vscrdef::new(320);

        let mut buffer = [0u8; 6];
        assert_eq!(vscrdef.fill_params_buf(&mut buffer)?, 6);
        assert_eq!(buffer, [0, 0, 0x1, 0x40, 0, 0]);

        Ok(())
    }
}
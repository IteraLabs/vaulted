#[cfg(test)]
mod tests {
    
    use super::*;
    use usdv_results::error::USDvError;


    #[test]
    fn test_error_conversion() {
        let error = USDvError::InvalidUSDCMint;
        let program_error: ProgramError = error.into();
        
        match program_error {
            ProgramError::Custom(code) => {
                assert_eq!(code, USDvError::InvalidUSDCMint as u32);
            }
            _ => panic!("Expected Custom error"),
        }
    }
}

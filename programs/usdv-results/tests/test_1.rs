#[cfg(test)]
mod tests {
    
    use usdv_results::USDvError;

    #[test]
    fn test_error_display() {
        let error = USDvError::InsufficientBalance {
        };
        
        let error_string = error.to_string();
        assert!(error_string.contains("Insufficient balance"));
        assert!(error_string.contains("1000"));
        assert!(error_string.contains("500"));
    }

}

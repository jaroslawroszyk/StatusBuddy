#[derive(Clone, Debug)]
pub struct Config {
    pub interval_time: u64,
}

impl Config {
    pub fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
        while let Some(arg) = args.next() {
            if arg == "--interval_time" {
                if let Some(val) = args.next() {
                    let interval_time = val.parse::<u64>().map_err(|_| {
                        format!(
                            "Invalid format for --interval_time: '{}'. Must be a positive integer.",
                            val
                        )
                    })?;
                    return Ok(Self { interval_time });
                } else {
                    return Err("Missing value for --interval_time argument".into());
                }
            }
        }
        Err("The --interval_time argument is required.".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args_success() {
        let args = vec![
            "program".to_string(),
            "--interval_time".to_string(),
            "10".to_string(),
        ];
        let config = Config::parse_args(args.into_iter()).unwrap();
        assert_eq!(config.interval_time, 10);
    }

    #[test]
    fn test_parse_args_missing_value() {
        let args = vec!["program".to_string(), "--interval_time".to_string()];
        let err = Config::parse_args(args.into_iter()).unwrap_err();
        assert_eq!(err, "Missing value for --interval_time argument");
    }

    #[test]
    fn test_parse_args_invalid_value() {
        let args = vec![
            "program".to_string(),
            "--interval_time".to_string(),
            "abc".to_string(),
        ];
        let err = Config::parse_args(args.into_iter()).unwrap_err();
        assert!(err.contains("Invalid format for --interval_time"));
    }

    #[test]
    fn test_parse_args_not_found() {
        let args = vec![
            "program".to_string(),
            "--other_arg".to_string(),
            "5".to_string(),
        ];
        let err = Config::parse_args(args.into_iter()).unwrap_err();
        assert_eq!(err, "The --interval_time argument is required.");
    }
}

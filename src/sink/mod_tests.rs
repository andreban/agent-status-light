use std::error::Error as StdError;

use super::*;

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

#[test]
fn test_default_constants() -> Result<()> {
    assert_eq!(DEFAULT_BAUD, 115_200);
    assert_eq!(DEFAULT_TIMEOUT_MS, 500);
    Ok(())
}

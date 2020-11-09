use crate::*;

#[test]
fn test_display() {
    assert_eq!(
        Game::new().display_board(),
        "  1  2  3  4  5  6  7 
1 -  -  -  -  -  -  - 
2 -  -  -  -  -  -  - 
3 -  -  -  -  -  -  - 
4 -  -  -  -  -  -  - 
5 -  -  -  -  -  -  - 
6 -  -  -  -  -  -  - 
"
    );
}

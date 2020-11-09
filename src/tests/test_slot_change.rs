use crate::*;

// Test pass

#[test]
fn test_slot_change() {
    let mut board = Game::new();
    board.change_slot(0, 3, 'e');
    assert_eq!(
        board.display_board(),
        "  1  2  3  4  5  6  7 
1 -  -  -  -  -  -  - 
2 -  -  -  -  -  -  - 
3 -  -  -  -  -  -  - 
4 e  -  -  -  -  -  - 
5 -  -  -  -  -  -  - 
6 -  -  -  -  -  -  - 
"
    );

    board.change_slot(3, 4, 'w');
    assert_eq!(
        board.display_board(),
        "  1  2  3  4  5  6  7 
1 -  -  -  -  -  -  - 
2 -  -  -  -  -  -  - 
3 -  -  -  -  -  -  - 
4 e  -  -  -  -  -  - 
5 -  -  -  w  -  -  - 
6 -  -  -  -  -  -  - 
"
    );

    board.change_slot(3, 5, 'j');
    assert_eq!(
        board.display_board(),
        "  1  2  3  4  5  6  7 
1 -  -  -  -  -  -  - 
2 -  -  -  -  -  -  - 
3 -  -  -  -  -  -  - 
4 e  -  -  -  -  -  - 
5 -  -  -  w  -  -  - 
6 -  -  -  j  -  -  - 
"
    );

    board.change_slot(4, 5, 'h');
    assert_eq!(
        board.display_board(),
        "  1  2  3  4  5  6  7 
1 -  -  -  -  -  -  - 
2 -  -  -  -  -  -  - 
3 -  -  -  -  -  -  - 
4 e  -  -  -  -  -  - 
5 -  -  -  w  -  -  - 
6 -  -  -  j  h  -  - 
"
    );
}

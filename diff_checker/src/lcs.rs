//Deals with the longest common subsequence problem

//Representing a 'final' line:

/*
  Pair of string and condition
  Like "abc" -> +
  Or   "def" -> - could be acheived with a pair of
    String and enum
*/

use std::vec;
use std::cmp;
use std::cmp::Ordering;
/*
AN EXAMPLE OF THE DYNAMIC TABLE USED FOR STORING THE VALUES OF THE LONGEST 
COMMON SUBSEQUENCE.

-----------------------------------------------------------------
|       |       |       |       |       |       |       |       |
|   0   |   0   |   0   |   0   |   0   |   0   |   0   |   0   |
|       |       |       |       |       |       |       |       |
|       |       |       |       |       |       |       |       |
-----------------------------------------------------------------
|       |       |       |       |       |       |       |       |
|   0   |   1   |   1   |   1   |       |       |       |       |
|       |   <   |   <   |   <   |       |       |       |       |
|       |       |       |       |       |       |       |       |
-----------------------------------------------------------------
|       |       |       |       |       |       |       |       |
|   0   |       |       |   2   |   2   |   2   |   2   |   2   |
|       |       |       |       |   <   |   <   |   <   |   <   |
|       |       |       |       |       |       |       |       |
-----------------------------------------------------------------
|       |       |       |       |       |       |       |       |
|   0   |       |       |       |       |       |   2^  |   3   |
|       |       |       |       |       |       |       |       |
|       |       |       |       |       |       |       |       |
-----------------------------------------------------------------
*/

fn create_lcs_of_strings(A: Vec<String>, B: Vec<String>) => Vec<String> {

    let i: u32 = 0;
    let j: u32 = 0;

    //PLUS ONE FOR THE ZEROES
    let table: Vec<Vec<u8>> = vec![vec![0; B.len() + 1]; A.len() + 1];

    //Filling table with LCS length
    for row in 0..A.len() {
        for col in 0..B.len() {
            if check_strings_equal(A.get(row), B.get(col)) {
                table[row][col] = table[row - 1][col - 1] + 1;
            } else {
                table[row][col] = cmp::max(table[row - 1][col],
                                            table[row][col - 1]);
            }
        }
    }

    let mut common_seq = Vec::new();
    let mut x:u32 = total_rows - 1;
    let mut y:u32 = total_columns - 1;

    while x != 0 && y != 0 {
        // Check element above is equal
        if table[x][y] == table[x - 1][y] {
            x = x - 1;
        }
        // check element to the left is equal
        else if table[x][y] == table[x][y - 1] {
            y = y - 1;
        }
        else {
            // check the two element at the respective x,y position is same
            assert_eq!(string1_chars[x-1], string2_chars[y-1]);
            let char = string1_chars[x - 1];
            common_seq.push(char);
            x = x - 1;
            y = y - 1;
        }
    }
    common_seq.reverse();
    (table[total_rows - 1][total_columns - 1],
    String::from_utf8(common_seq).unwrap())
}

fn check_strings_equal(String a, String b) => bool {
    let a_chars = a.as_chars();
    let b_chars = b.as_chars();

    match a.cmp(&b) {
        Ordering::Equal => true
        _ => false
    }
}


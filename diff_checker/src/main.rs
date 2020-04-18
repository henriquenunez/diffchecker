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

fn create_lcs_of_strings(str_a: String, str_b: String) -> String {

    let a = str_a.as_bytes();
    let b = str_b.as_bytes();

    //PLUS ONE FOR THE ZEROES
    let mut table: Vec<Vec<u8>> = vec![vec![0; b.len() + 1]; a.len() + 1];

    //Filling table with LCS length
    for row in 0..a.len() {
        for col in 0..b.len() {
            if a.get(row) == b.get(col) {
                table[row + 1][col + 1] = table[row][col] + 1;
            } else {
                table[row + 1][col + 1] = cmp::max(table[row][col + 1],
                                            table[row + 1][col]);
            }
        }
    }

    let mut common_seq = Vec::new();
    let mut x = a.len();
    let mut y = b.len();

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
            assert_eq!(a[x-1], b[y-1]);
            let char = a[x - 1];
            common_seq.push(char);
            x = x - 1;
            y = y - 1;
        }
    }
    common_seq.reverse();
    String::from_utf8(common_seq).unwrap()
}

use std::io;

fn main() {

    let mut string_one = String::new();
    let mut string_two = String::new();

    println!("Input first string:");
    let num_one = io::stdin().read_line(&mut string_one)
        .expect("Failed to read line!!");
    println!("Num of bytes is: {}", num_one);

    println!("Input second string:");
    let num_two = io::stdin().read_line(&mut string_two)
        .expect("Failed to read line!!");
    println!("Num of bytes is: {}", num_two);

    println!("Subsequence is: {}", create_lcs_of_strings(string_one,
                                                         string_two))
}

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
        0       A       B       C       D       E       F       G
    -----------------------------------------------------------------
    |       |       |       |       |       |       |       |       |
 0  |   0   |   0   |   0   |   0   |   0   |   0   |   0   |   0   |
    |       |       |       |       |       |       |       |       |
    |       |       |       |       |       |       |       |       |
    -----------------------------------------------------------------
    |       | \     |       |       |       |       |       |       |
 A  |   0   |   1   |   1   |   1   |   1   |   1   |   1   |   1   |
    |       |   <   |   <   |   <   |   <   |   <   |   <   |   <   |
    |       |       |       |       |       |       |       |       |
    -----------------------------------------------------------------
    |       |       |       |       | \     |       |       |       |
 D  |   0   |   1   |       |       |   2   |   2   |   2   |   2   |
    |       |   ^   |       |       |       |   <   |   <   |   <   |
    |       |       |       |       |       |       |       |       |
    -----------------------------------------------------------------
    |       |       |       |       |       |       |       |       |
 H  |   0   |   1   |   1   |   1   |   1   |   1   |   2^  |   3   |
    |       |   ^   |   <   |   <   |   <   |   <   |       |       |
    |       |       |       |       |       |       |       |       |
    -----------------------------------------------------------------
*/

fn create_lcs_of_strings(text_a: Vec<&str>,
                         text_b: Vec<&str>) -> Vec<String> {

    //PLUS ONE FOR THE ZEROES
    let mut table: Vec<Vec<u8>> =
                        vec![vec![0; text_b.len() + 1]; text_a.len() + 1];

    //Filling table with LCS length
    for (i_row, &row) in text_a.iter().enumerate() {
        for (i_col, &col) in text_b.iter().enumerate() {
            table[i_row + 1][i_col + 1] =
                match row.cmp(&col) {
                    Ordering::Equal => table[i_row][i_col] + 1,
                    _ => cmp::max(table[i_row + 1][i_col],
                                  table[i_row][i_col + 1]),
                };
        }
    }

    let mut common_seq = Vec::new();
    let mut x = text_a.len();
    let mut y = text_b.len();

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
            //I dont know how to assert it yet
            assert_eq!(text_a[x-1], text_b[y-1]);
            let this_str = String::from(text_a[x - 1]);
            common_seq.push(this_str);
            x = x - 1;
            y = y - 1;
        }
    }
    common_seq.reverse();
    common_seq
    //String::from_utf8(common_seq).unwrap()
}

use std::io;

fn main() {

    let string_one = vec!["aa", "bb", "cc", "cc", "cc"];//String::new();
    let string_two = vec!["aa",       "cc", "cc", "cc"];//String::new();

    let owo = create_lcs_of_strings(string_one, string_two);
    for grr in owo.iter() {
        println!("Part: {}", grr);
    }

}



use crate::Solution;
use std::collections:: HashSet;
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut columns = vec![vec![0; 9]; 9];
        let mut boxes = vec![vec![0; 9]; 9];
        let mut set = HashSet::new();
        for (i, row) in board.iter().enumerate() {
            let numbers = row.iter().map(|c| c.to_digit(10).unwrap_or(0)).enumerate();
            for (j, number) in numbers {
                if number !=0 && set.contains(&number) {
                    println!("{i} {j}");
                    return false;
                }
                set.insert(number);
                columns[j][i] = number;
                let z = i - (i % 3);
                let n = j / 3 + z;
                let k = j % 3 + i % 3 * 3;
                boxes[n][k] = number;
            }
            set.clear();
        }
        for i in 0 .. 9 {
            if Self::check_segment(&columns[i], &mut set) {return false;}
            if Self::check_segment(&boxes[i], &mut set) {return false;}
        }
        true
    }
    fn check_segment(segment: &[u32], cell_set: &mut HashSet<u32>) -> bool {
        for number in segment {
            if *number != 0 && cell_set.contains(&number) {return true;}
            cell_set.insert(*number);
        }
        cell_set.clear();
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let request = [
           // 00  01  02    03  04  05      06  07  08
           // 00  01  02    10  11  12      20  21  22
            ["5","3",".",   ".","7",".",    ".",".","."],
           // 10  11  12    13  14  15      16  17  18
           // 03  04  05    13  14  15      23  25  25
            ["6",".",".",   "1","9","5",    ".",".","."],
           // 20  21  12    23  24  25      26  27  28
           // 06  07  08    16  17  18      26  27  28
            [".","9","8",   ".",".",".",    ".","6","."], 
           // 30  31  12    33  34  15      36  37  38    
           // 30  31  32    40  41  42      50  51  52    
            ["8",".",".",   ".","6",".",    ".",".","3"],
            ["4",".",".",   "8",".","3",    ".",".","1"],
            ["7",".",".",   ".","2",".",    ".",".","6"],
            [".","6",".",".",".",".","2","8","."],
            [".",".",".","4","1","9",".",".","5"],
            [".",".",".",".","8",".",".","7","9"]
        ]
            .into_iter()
            .map(|row| row.into_iter().filter_map(|str| str.chars().next()).collect())
            .collect();
        let answer = Solution::is_valid_sudoku(request);
        let should_be = true;
        assert_eq!(should_be, answer);
    }

    #[test]
    fn test_2() {
        let request = [
            ["1","2",".",   ".",".",".",    "6",".","7"],
            [".",".",".",   ".",".",".",    ".",".","5"],
            [".",".","9",   ".","6",".",    "4",".","."],

            [".","6",".",   ".",".",".",    ".",".","."],
            [".",".",".",   ".","4",".",    ".","7","."],
            [".",".",".",   ".",".",".",    ".",".","."],

            [".",".",".",   "5",".",".",    ".",".","."],
            [".",".",".",   ".",".",".",    ".",".","2"],
            [".","9",".",   ".",".",".",    ".",".","7"]
        ]
            .into_iter()
            .map(|row| row.into_iter().filter_map(|str| str.chars().next()).collect())
            .collect();
        let answer = Solution::is_valid_sudoku(request);
        let should_be = false;
        assert_eq!(should_be, answer);
    }
}
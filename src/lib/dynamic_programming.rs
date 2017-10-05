use std::cmp;

pub fn longest_increasing_subsequence(input: &[u32]) -> Vec<u32> {
    let mut matrix: Vec<Vec<u32>> = input.into_iter().map(|x| vec![*x]).collect();

    for j in 1..input.len() {
        for i in 0..j {
            if input[j] > input[i] && matrix[j].len() < matrix[i].len()+1 {
                let mut res = matrix[i].clone();
                res.push(input[j]);
                matrix[j] = res;
            }
        }
    }

    matrix.iter().max_by_key(|x| x.len()).unwrap().clone()
}

pub fn longest_common_subsequence(input1: &[u32], input2: &[u32]) -> Vec<u32> {
    let size = input1.len() + 1; //pad this by one to account for initial value
    let mut matrix = vec![vec![0; size]; size]; //create an size x size matrix
    // generate optimal sub-solutions
    for row in 1..size {
        for col in 1..size {
            if input1[row-1] == input2[col-1] {
                matrix[row][col] = matrix[row-1][col-1] + 1;
            } else {
                matrix[row][col] = cmp::max(matrix[row][col-1], matrix[row-1][col]);
            }
        }
    }
    // recover solution
    let mut solution = Vec::new();
    let (mut i, mut j) = (size-1, size-1);
    while j != 0 && i != 0 {
        if matrix[i][j] != 0 && input1[i-1] == input2[j-1] {
            solution.push(input2[j-1]);
            i -= 1;
            j -= 1;
        } else if matrix[i-1][j] > matrix[i][j-1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    solution.reverse();
    solution
}
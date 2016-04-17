fn check_row(row: &Vec<i32>) -> bool {
    for i in 1..10 {
        let curr_value: &i32 = &i;
        if !row.contains(curr_value) {
            return false;
        }
    }

    return true;
}

fn check_rows(rows: Vec<Vec<i32>>) -> bool {
    for row in &rows {
        if !check_row(row) {
            return false
        }
    }

    return true;
}

fn main() {
    let sudoku: Vec<Vec<i32>> = vec![
        vec![5,3,4,6,7,8,9,1,2],
        vec![6,7,2,1,9,5,3,4,8],
        vec![1,9,8,3,4,2,5,6,7],
        vec![8,5,9,7,6,1,4,2,3],
        vec![4,2,6,8,5,3,7,9,1],
        vec![7,1,3,9,2,4,8,5,6],
        vec![9,6,1,5,3,7,2,8,4],
        vec![2,8,7,4,1,9,6,3,5],
        vec![3,4,5,2,8,6,1,7,9]
    ];

    if !check_rows(sudoku) {
        println!("The sudoku puzzle is invalid");
    }
    println!("The rows check out!");
}

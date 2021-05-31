use std::sync::Arc;
use std::sync::Mutex;

// gauss-seidel method implementation
pub fn solve_system_of_equations(
    coefficients_matrix: &Vec<Vec<f32>>, 
    y_vector: &Vec<Vec<f32>>, 
    iterations_number: i32, 
    thread_id: i32,
    all_threads_number: i32,
    x_results_mutex: Arc< Mutex< Vec<Vec<f32>> > >
){
    // preparing necessary data : N = D^(-1), L and U matrix
    let n_matrix: Vec<Vec<f32>> = prepare_n_matrix(coefficients_matrix); // D^(-1)
    let l_matrix: Vec<Vec<f32>> = prepare_l_matrix(coefficients_matrix); // L matrix
    let u_matrix: Vec<Vec<f32>> = prepare_u_matrix(coefficients_matrix); // U matrix

    let nu_matrix: Vec<Vec<f32>> = multiply_matrices(n_matrix.clone(), u_matrix.clone());
    let nl_matrix: Vec<Vec<f32>> = multiply_matrices(n_matrix.clone(), l_matrix.clone());

    // proper calculations
    for iteration_number in 1..iterations_number{
        let mut row_index = thread_id;
        while row_index < (coefficients_matrix.len() as i32) {
            let mut can_go: bool = false;
            while !can_go{
                let results = x_results_mutex.lock().unwrap();
                can_go = can_make_next_calculations(row_index, &results[iteration_number as usize], &results[(iteration_number - 1) as usize]);
            }

            let mut x_results = x_results_mutex.lock().unwrap();
            let db : f32 = n_matrix[row_index as usize][row_index as usize] * y_vector[row_index as usize][0]; //db

            let mut dlx = 0.0;
            for i in 0..row_index as usize{
                dlx += nl_matrix[row_index as usize][i as usize] * x_results[iteration_number as usize][i as usize]; //res -= l_matrix[row_index as usize][i as usize] * x_results[iteration_number as usize][i as usize];//
            }

            let mut dux = 0.0;
            for i in (row_index + 1)..u_matrix.len() as i32{
                dux += nu_matrix[row_index as usize][i as usize] * x_results[(iteration_number - 1) as usize][i as usize]; //res -= u_matrix[row_index as usize][i as usize] * x_results[iteration_number as usize][i as usize];
            }

            x_results[iteration_number as usize][row_index as usize] = db - dlx - dux;
            row_index += all_threads_number;
        }
    }
}

fn can_make_next_calculations(variable_id: i32, current_iteration_results: &Vec<f32>, previous_iteration_results: &Vec<f32>) -> bool {
    for i in 0..variable_id{
        if current_iteration_results[i as usize].is_nan(){
            return false;
        }
    }

    for i in (variable_id + 1)..previous_iteration_results.len() as i32{
        if previous_iteration_results[i as usize].is_nan(){
            return false;
        }
    }
    return true;
}

fn multiply_matrices(first_matrix: Vec<Vec<f32>>, second_matrix: Vec<Vec<f32>>) -> Vec<Vec<f32>>{
    let mut result_matrix = Vec::new();
    for row in 0..first_matrix.len(){
        let mut new_matrix_row = Vec::new();
        for column in 0..first_matrix.len(){
            let mut tmp = 0.0;
            for i in 0..first_matrix[row].len(){
                tmp += first_matrix[row][i] * second_matrix[i][column];
            }
            new_matrix_row.push(tmp);
        }
        result_matrix.push(new_matrix_row);
    }
    return result_matrix;
}

// U-matrix preparation
fn prepare_u_matrix(coefficients_matrix: &Vec<Vec<f32>>) -> Vec<Vec<f32>>{
    let mut u_matrix = Vec::new();
    for row_num in 0..coefficients_matrix.len(){
        let mut row = Vec::new();
        for column_num in 0..coefficients_matrix.len(){
            if row_num < column_num{
                row.push( coefficients_matrix[row_num][column_num] );
            }
            else{
                row.push(0.0);
            }
        }
        u_matrix.push(row);
    }
    return u_matrix;
}

// L-matrix preparation
fn prepare_l_matrix(coefficients_matrix: &Vec<Vec<f32>>) -> Vec<Vec<f32>>{
    let mut l_matrix = Vec::new();
    for row_num in 0..coefficients_matrix.len(){
        let mut row = Vec::new();
        for column_num in 0..coefficients_matrix.len(){
            if row_num > column_num{
                row.push( coefficients_matrix[row_num][column_num] );
            }
            else{
                row.push(0.0);
            }
        }
        l_matrix.push(row);
    }
    return l_matrix;
}

// Calculation of N = D^(-1)
fn prepare_n_matrix(coefficients_matrix: &Vec<Vec<f32>>) -> Vec<Vec<f32>>{
    /*let mut n_vector = Vec::new();
    for i in 0..coefficients_matrix.len(){
        n_vector.push( 1.0 / (coefficients_matrix[i][i] as f32) );
    }
    return n_vector;*/
    let mut n_matrix = Vec::new();
    for row_num in 0..coefficients_matrix.len(){
        let mut row = Vec::new();
        for column_num in 0..coefficients_matrix.len(){
            if row_num == column_num{
                row.push( 1.0 / coefficients_matrix[row_num][column_num] );
            }
            else{
                row.push(0.0);
            }
        }
        n_matrix.push(row);
    }
    return n_matrix;
}

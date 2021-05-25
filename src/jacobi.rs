use std::sync::Arc;
use std::sync::Mutex;

// jacobi implementation
pub fn solve_system_of_equations(
    coefficients_matrix: &Vec<Vec<f32>>, 
    y_vector: &Vec<Vec<f32>>, 
    iterations_number: i32, 
    thread_id: i32,
    all_threads_number: i32,
    x_results_mutex: Arc< Mutex< Vec<Vec<f32>> > >
){

    // preparing necessary data : matrix M and vector N
    let n_diagonal: Vec<f32> = prepare_n_vector(coefficients_matrix);
    let m_matrix: Vec<Vec<f32>> = prepare_m_matrix(coefficients_matrix, &n_diagonal);

    for iteration_number in 1..iterations_number{      // iterations loop
        let mut row_index = thread_id;  //
        while row_index < (coefficients_matrix.len() as i32) {
            let mut x_results = x_results_mutex.lock().unwrap();

            x_results[iteration_number as usize][row_index as usize] = n_diagonal[row_index as usize] * y_vector[row_index as usize][0];
            for j in 0..m_matrix.len(){
                x_results[iteration_number as usize][row_index as usize] += m_matrix[row_index as usize][j] * x_results[(iteration_number - 1)as usize][j];
            }
            row_index += all_threads_number;
        }
        
        let mut can_continue_calculations: bool = false;
        while !can_continue_calculations{
            let results = x_results_mutex.lock().unwrap();
            can_continue_calculations = can_start_next_iteration(&results[iteration_number as usize]);
        }
    }

    let results = x_results_mutex.lock().unwrap();
    println!("Iteration nr {:?},\nValues: {:?}\n", iterations_number, results[(iterations_number - 1) as usize]);
}

fn can_start_next_iteration(current_iteration_results: &Vec<f32>) -> bool{
    for i in 0..current_iteration_results.len(){
        if current_iteration_results[i].is_nan(){
            return false;
        }
    }
    return true;
}

// Calculation of M = -N * (L + U)
fn prepare_m_matrix(coefficients_matrix: &Vec<Vec<f32>>, n_vector: &Vec<f32>) -> Vec<Vec<f32>>{
    let mut m_matrix = Vec::new();
    for row_num in 0..coefficients_matrix.len(){
        let mut row = Vec::new();
        for column_num in 0..coefficients_matrix.len(){
            if row_num == column_num{
                row.push(0.0);
            }
            else{
                row.push( - coefficients_matrix[row_num][column_num] * n_vector[row_num] );
            }
        }
        m_matrix.push(row);
    }
    return m_matrix;
}

// Calculation of N = D^(-1)
fn prepare_n_vector(coefficients_matrix: &Vec<Vec<f32>>) -> Vec<f32>{
    let mut n_vector = Vec::new();
    for i in 0..coefficients_matrix.len(){
        n_vector.push( 1.0 / (coefficients_matrix[i][i] as f32) );
    }
    return n_vector;
}

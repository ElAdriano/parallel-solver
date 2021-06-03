use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Barrier;

pub fn solve_system_of_equations(
    iterations_number: i32,
    thread_id: i32,
    threads_number: i32,
    coefficients_matrix: &Vec< Vec<f32> >, 
    y_vector: &Vec< Vec<f32> >,
    results_mutex: Arc< Mutex< Vec<Vec<f32>> > >,
    barrier: Arc< Barrier >
){
    // preparing necessary data : matrix M and vector N
    let n_diagonal: Vec<f32> = prepare_n_vector(coefficients_matrix);
    let m_matrix: Vec<Vec<f32>> = prepare_m_matrix(coefficients_matrix, &n_diagonal);

    for iteration_number in 1..iterations_number{
        let mut variable_id = thread_id;
        while variable_id < y_vector.len() as i32{
            let mut iterations_results = results_mutex.lock().unwrap();

            let mut current_iteration_result: f32 = n_diagonal[variable_id as usize] * y_vector[variable_id as usize][0];
            for j in 0..m_matrix.len(){
                current_iteration_result += m_matrix[variable_id as usize][j] * iterations_results[(iteration_number - 1) as usize][j];
            }

            iterations_results[iteration_number as usize][variable_id as usize] = current_iteration_result;
            variable_id += threads_number;
        }
        barrier.wait();
        let iterations_results = results_mutex.lock().unwrap();
        let current_it_error = calculate_error(coefficients_matrix, &iterations_results[iteration_number as usize], y_vector);

        println!("Error value: {:?}", current_it_error);
        if current_it_error < 0.00001{
            return;
        }
    }
}

fn calculate_error(coefficients: &Vec< Vec<f32> >, x_values: &Vec<f32>, y_values: &Vec< Vec<f32> >) -> f32 {
    let mut error: f32 = 0.0;
    for row_id in 0..coefficients.len(){
        let mut row_value = 0.0;
        for i in 0..coefficients[row_id].len(){
            row_value += coefficients[row_id][i] * x_values[i];
        }
        error += (row_value - y_values[row_id][0]) * (row_value - y_values[row_id][0]); 
    }
    return error;
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

/*
pub fn iterate_new_value(
    coefficients_matrix: &Vec<Vec<f32>>, 
    y_vector: &Vec<Vec<f32>>, 
    variable_id: i32,
    x_results: &Vec<f32>,
) -> f32 {
    // preparing necessary data : matrix M and vector N
    let n_diagonal: Vec<f32> = prepare_n_vector(coefficients_matrix);
    let m_matrix: Vec<Vec<f32>> = prepare_m_matrix(coefficients_matrix, &n_diagonal);

    let mut result: f32 = n_diagonal[variable_id as usize] * y_vector[variable_id as usize][0];//x_results[iteration_number as usize][variable_id as usize] = n_diagonal[variable_id as usize] * y_vector[variable_id as usize][0];
    for j in 0..m_matrix.len(){
        result += m_matrix[variable_id as usize][j] * x_results[j];//x_results[iteration_number as usize][variable_id as usize] += m_matrix[variable_id as usize][j] * x_results[(iteration_number - 1)as usize][j];
    }
    return result;
}
=====================================================
for iteration_number in 1..iterations_number{
    let mut variable_id = thread_id;
    while variable_id < y_vector.len() as i32{
        let mut iteration_results = arc_mutex_clone.lock().unwrap();
        let new_value = jacobi::iterate_new_value(&a_matrix, &y_vector, variable_id, &iteration_results[(iteration_number - 1) as usize]);
        println!("Thread_id = {:?} | Value: {:?}", thread_id, new_value);
        iteration_results[iteration_number as usize][variable_id as usize] = new_value;
        variable_id += threads_number;
    }
    println!("Iteration {:?} ended...", iteration_number);
    barrier_copy.wait();
}
*/
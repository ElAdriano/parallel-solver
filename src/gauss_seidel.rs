use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Barrier;

// gauss-seidel method implementation
pub fn solve_system_of_equations(
    iterations_number: i32,
    thread_id: i32,
    all_threads_number: i32,
    coefficients_matrix: &Vec< Vec<f32> >, 
    y_vector: &Vec< Vec<f32> >,
    x_results_mutex: Arc< Mutex< Vec<Vec<f32>> > >,
    barrier: Arc< Barrier >
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
        while row_index < (y_vector.len() as i32) {
            //println!("Thread {:?} is waiting for others", thread_id);
            let mut can_compute = are_all_needed_results_available(iteration_number, &x_results_mutex, row_index);
            while !can_compute{
                can_compute = are_all_needed_results_available(iteration_number, &x_results_mutex, row_index);
            }

            //println!("Thread {:?} starts calculations for variable nr {:?}", thread_id, row_index);
            let db : f32 = n_matrix[row_index as usize][row_index as usize] * y_vector[row_index as usize][0]; //db

            let mut dlx = 0.0;
            for i in 0..row_index as usize{
                let x_results = x_results_mutex.lock().unwrap();
                dlx += nl_matrix[row_index as usize][i as usize] * x_results[iteration_number as usize][i as usize]; //res -= l_matrix[row_index as usize][i as usize] * x_results[iteration_number as usize][i as usize];//
            }

            let mut dux = 0.0;
            for i in (row_index + 1)..u_matrix.len() as i32{
                let x_results = x_results_mutex.lock().unwrap();
                dux += nu_matrix[row_index as usize][i as usize] * x_results[(iteration_number - 1) as usize][i as usize]; //res -= u_matrix[row_index as usize][i as usize] * x_results[iteration_number as usize][i as usize];
            }
            
            update_result_value(&x_results_mutex, iteration_number, db - dlx - dux, row_index);
            row_index += all_threads_number;
        }
        barrier.wait();

        let iterations_results = x_results_mutex.lock().unwrap();
        let current_it_error = calculate_error(coefficients_matrix, &iterations_results[iteration_number as usize], y_vector);
        println!("Error value: {:?}", current_it_error);

        if current_it_error < 0.00001{
            return;
        }
    }
}

fn are_all_needed_results_available(current_iteration_number: i32, mutex: &Arc< Mutex< Vec<Vec<f32>> > >, variable_id: i32) -> bool{
    let iterations_results = mutex.lock().unwrap();
    for i in 0..variable_id{
        if iterations_results[current_iteration_number as usize][i as usize].is_nan(){
            return false;
        }
    }

    let previous_it_num: usize = (current_iteration_number - 1) as usize;
    for i in (variable_id + 1)..iterations_results[previous_it_num].len() as i32{
        if iterations_results[previous_it_num][i as usize].is_nan(){
            return false;
        }
    }
    return true;
}

fn update_result_value(mutex: &Arc< Mutex< Vec<Vec<f32>> > >, current_iteration_number: i32, new_value: f32, variable_id: i32){
    let mut x_results = mutex.lock().unwrap();
    x_results[current_iteration_number as usize][variable_id as usize] = new_value;
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

fn multiply_matrices(first_matrix: Vec<Vec<f32>>, second_matrix: Vec<Vec<f32>>) -> Vec<Vec<f32>>{
    let mut result_matrix = Vec::new();
    for row in 0..first_matrix.len(){
        let mut new_matrix_row = Vec::new();
        for column in 0..first_matrix.len(){
            let mut tmp = 0.0;
            for i in 0..first_matrix[row].len(){
                tmp += first_matrix[row][i] * second_matrix[i][column];
            }
            tmp = (1000.0 * tmp).round() / 1000.0;
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
                let rounded: f32 = (1000.0 / coefficients_matrix[row_num][column_num]).round() / 1000.0;
                row.push(rounded);
            }
            else{
                row.push(0.0);
            }
        }
        n_matrix.push(row);
    }
    return n_matrix;
}

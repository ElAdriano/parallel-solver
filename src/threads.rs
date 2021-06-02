use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Barrier;

use jacobi;
use gauss_seidel;
use file_manager;

pub fn find_solutions(
    coefficients_matrix: Vec<Vec<f32>>, 
    y_values_vector: Vec<Vec<f32>>, 
    x_values: Vec<Vec<f32>>, 
    iterations_number: i32,
    threads_number: i32,
    output_file_name: &str,
    solving_method: i32
){
    create_threads_and_start_solving(coefficients_matrix, y_values_vector, x_values, iterations_number, threads_number, output_file_name, solving_method);
}

fn create_threads_and_start_solving(
    coefficients_matrix: Vec<Vec<f32>>, 
    y_values_vector: Vec<Vec<f32>>, 
    x_values: Vec<Vec<f32>>, 
    iterations_number: i32,
    threads_number: i32,
    output_file_name: &str,
    solving_method: i32
){
    let mut created_threads = vec![];

    let arc_barrier = Arc::new( Barrier::new(threads_number as usize) );
    let mutex = Mutex::new(x_values);
    let arc_mutex = Arc::new(mutex);

    for thread_id in 0..threads_number{
        let thread_name : String = "Thread".to_string() + &thread_id.to_string(); // powalone jakieś TODO
        let threads_builder = thread::Builder::new().name(thread_name);

        // making copy of coefficients matrix, results vector
        let a_matrix = Arc::new(coefficients_matrix.clone());
        let y_vector = Arc::new(y_values_vector.clone());
        let arc_mutex_clone = arc_mutex.clone();
        let barrier_clone = arc_barrier.clone();

        let new_thread = threads_builder.spawn(move || {
            if solving_method == 0{
                jacobi::solve_system_of_equations(
                    iterations_number,
                    thread_id,
                    threads_number,
                    &a_matrix,
                    &y_vector,
                    arc_mutex_clone,
                    barrier_clone
                );
            }
            else {
                gauss_seidel::solve_system_of_equations(
                    iterations_number,
                    thread_id,
                    threads_number,
                    &a_matrix,
                    &y_vector,
                    arc_mutex_clone,
                    barrier_clone
                );
            }
        }).unwrap();
        created_threads.push(new_thread);
    }

    let mut computation_ended: bool = false;
    while !computation_ended{
        let results = arc_mutex.lock().unwrap().to_vec();
        computation_ended = last_iteration_results_available(&results[(results.len() - 1) as usize]);
    }
    
    let results = arc_mutex.lock().unwrap().to_vec();
    file_manager::write(output_file_name, &results[(results.len() - 1) as usize]);
}

fn last_iteration_results_available(results: &Vec<f32>) -> bool {
    for i in 0..results.len(){
        if results[i].is_nan(){
            return false;
        }
    }
    return true;
}
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

    let mut sync_vector = Vec::new(); //contains info if i-th thread is still working
    for _i in 0..threads_number{
        sync_vector.push(true);
    }

    let arc_mutex_sync_vector = Arc::new( Mutex::new(sync_vector) );

    for thread_id in 0..threads_number{
        let thread_name : String = "Thread".to_string() + &thread_id.to_string(); // powalone jakieś TODO
        let threads_builder = thread::Builder::new().name(thread_name);

        // making copy of coefficients matrix, results vector, common mutex and threads barrier
        let a_matrix = Arc::new(coefficients_matrix.clone());
        let y_vector = Arc::new(y_values_vector.clone());
        let arc_mutex_clone = arc_mutex.clone();
        let barrier_clone = arc_barrier.clone();
        let sync_vector_clone = arc_mutex_sync_vector.clone();

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
            let mut sync_vector = sync_vector_clone.lock().unwrap();
            sync_vector[thread_id as usize] = false;
        }).unwrap();
        created_threads.push(new_thread);
    }

    let mut computation_ended: bool = false;
    while !computation_ended{
        let thread_work_status_vector = arc_mutex_sync_vector.lock().unwrap().to_vec();
        computation_ended = are_computations_ended(&thread_work_status_vector);
    }
    
    let results = arc_mutex.lock().unwrap().to_vec();
    let final_results_idx = find_results(&results);
    file_manager::write(output_file_name, &results[final_results_idx as usize]);
}

fn are_computations_ended(threads_status: &Vec<bool>) -> bool {
    for i in 0..threads_status.len(){
        if threads_status[i]{
            return false;
        }
    }
    return true;
}

fn find_results(results_history: &Vec<Vec<f32>>) -> i32{
    let mut iteration_id = (results_history.len() - 1) as i32;
    while iteration_id > 0 {
        let mut are_values_ok = true;
        for i in 0..results_history[iteration_id as usize].len(){
            if results_history[iteration_id as usize][i].is_nan(){
                are_values_ok = false;
                break;
            }
        }
        if are_values_ok{
            return iteration_id;
        }
        iteration_id -= 1;
    }
    return 0;
}
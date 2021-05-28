use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Barrier;

use jacobi;
use file_manager;

pub fn find_solutions(
    coefficients_matrix: Vec<Vec<f32>>, 
    y_values_vector: Vec<Vec<f32>>, 
    x_values: Vec<Vec<f32>>, 
    iterations_number: i32,
    threads_number: i32,
    output_file_name: &str
){
    create_threads_and_start_solving(coefficients_matrix, y_values_vector, x_values, iterations_number, threads_number, output_file_name);
}

fn create_threads_and_start_solving(
    coefficients_matrix: Vec<Vec<f32>>, 
    y_values_vector: Vec<Vec<f32>>, 
    x_values: Vec<Vec<f32>>, 
    iterations_number: i32,
    threads_number: i32,
    output_file_name: &str
){
    let mut created_threads = vec![];

    let mutex = Mutex::new(x_values);
    let arc_mutex = Arc::new(mutex);
    
    let thread_barrier = Arc::new(Barrier::new((threads_number + 1) as usize)); // creating a barrier to wait for other threads to end their work

    for i in 0..threads_number{
        let thread_name : String = "Thread".to_string() + &i.to_string(); // powalone jakieś TODO
        let threads_builder = thread::Builder::new().name(thread_name);

        // making copy of coefficients matrix, results vector
        let a_matrix = Arc::new(coefficients_matrix.clone());
        let y_vector = Arc::new(y_values_vector.clone());
        let arc_mutex_clone = arc_mutex.clone();

        let barrier_copy = thread_barrier.clone();

        let new_thread = threads_builder.spawn(move || {
            jacobi::solve_system_of_equations(&a_matrix, &y_vector, iterations_number, i, threads_number, arc_mutex_clone); // borrowing copied data to other threads (any other thread has no rights to borrowed data)
            barrier_copy.wait(); // wait for other siblings
        }).unwrap();
        created_threads.push(new_thread);
    }

    thread_barrier.wait(); // wait for all children
    let results = arc_mutex.lock().unwrap().to_vec();

    file_manager::write(output_file_name, &results[(results.len() - 1) as usize]);
}
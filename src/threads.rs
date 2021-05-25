use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

use jacobi;

pub fn find_solutions(
    coefficients_matrix: Vec<Vec<f32>>, 
    y_values_vector: Vec<Vec<f32>>, 
    x_values: Vec<Vec<f32>>, 
    iterations_number: i32,
    threads_number: i32
){
    create_threads(coefficients_matrix, y_values_vector, x_values, iterations_number, threads_number);
}

fn create_threads(
    coefficients_matrix: Vec<Vec<f32>>, 
    y_values_vector: Vec<Vec<f32>>, 
    x_values: Vec<Vec<f32>>, 
    iterations_number: i32,
    threads_number: i32
){
    let mut created_threads = vec![];

    let mutex = Mutex::new(x_values);
    let arc_mutex = Arc::new(mutex);
    
    for i in 0..threads_number{
        let thread_name : String = "Thread".to_string() + &i.to_string(); // powalone jakieś TODO
        let threads_builder = thread::Builder::new().name(thread_name);

        // making copy of coefficients matrix, results vector
        let a_matrix = Arc::new(coefficients_matrix.clone());
        let y_vector = Arc::new(y_values_vector.clone());
        let arc_mutex_clone = arc_mutex.clone();

        let new_thread = threads_builder.spawn(move || {
            jacobi::solve_system_of_equations(&a_matrix, &y_vector, iterations_number, i, threads_number, arc_mutex_clone); // borrowing copied data to other threads (any other thread has no rights to borrowed data)
        }).unwrap();
        created_threads.push(new_thread);
    }

    for new_thread in created_threads{
        match new_thread.join(){
            Ok(_) => {println!("Thread created properly...");},
            Err(e) => {println!("Could not create thread... {:?}", e);}
        };
    }
}
use std::env;
use std::process;

mod file_manager;
mod threads;
mod jacobi;

/**
 * Main function
 * 
 * Program execution codes:
 * ========================
 * Code |  0 | Program executed properly
 * Code | -1 | Incorrect number of arguments
 * Code | -2 | An error occurred while reading file
 */
fn main() {
    // collecting all input arguments
    let args: Vec<String> = env::args().collect();
    let validation_result = validate_input(&args);

    if !validation_result{
        process::exit(-1);
    }

    // reading coefficients_matrix and equations results data
    let coefficients_matrix: Vec<Vec<f32>> = file_manager::read(&args[1]);
    let y_values_vector: Vec<Vec<f32>> = file_manager::read(&args[2]);
    let threads_number: i32 = args[3].trim().parse().unwrap();

    // data validation
    let coefficients_matrix_correct = file_manager::validate_coefficients_matrix(&coefficients_matrix);
    if !coefficients_matrix_correct{
        process::exit(-2);
    }

    let results_vector_correct = file_manager::validate_results_vector(coefficients_matrix.len() as i32, &y_values_vector);
    if !results_vector_correct{
        process::exit(-2);
    }

    // creating history for results with initial values
    let mut x_values = Vec::new();
    let iterations_number: i32 = args[4].trim().parse().unwrap();

    for _it_num in 0..iterations_number{
        if _it_num == 0{
            let mut initial_values = Vec::new();
            for _i in 0..coefficients_matrix.len() as i32{
                initial_values.push(0.0);
            }
            x_values.push(initial_values);    
        }
        else{
            let mut initial_values = Vec::new();
            for _i in 0..coefficients_matrix.len() as i32{
                initial_values.push(f32::NAN);
            }
            x_values.push(initial_values);
        }
    }

    let output_file_name: &str = &args[5];

    // find solutions
    threads::find_solutions(coefficients_matrix, y_values_vector, x_values, iterations_number, threads_number, output_file_name);

    // end program
    process::exit(0);
}

fn validate_input(args: &Vec<String>) -> bool{
    if args.len() < 6{
        println!("There was too less arguments.\nExample for starting program: \n'.\\main.exe file_with_coefficients_matrix file_with_y_vector number_of_threads max_iterations_number output_file_name'");
        return false;
    }
    if args.len() > 6{
        println!("There was too many arguments.\nExample for starting program: \n'.\\main.exe file_with_coefficients_matrix file_with_y_vector number_of_threads max_iterations_number output_file_name'");
        return false;
    }
    return true;
}

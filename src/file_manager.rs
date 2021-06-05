use std::fs;
use std::process;

// code -2 means that data are incorrect
pub fn read(file_name: &str) -> Vec<Vec<f32>> {
    let is_file_existing = std::path::Path::new(file_name).exists();

    if !is_file_existing{
        println!("An error occurred while opening file '{}'.", file_name);
        process::exit(-2);
    }

    let file_content = fs::read_to_string(file_name).expect("");

    if file_content.len() == 0{
        process::exit(-2);
    }

    let rows : Vec<&str> = file_content.split('\n').collect();

    if rows.len() == 0 {
        process::exit(-2);
    }

    let mut matrix = Vec::new(); // create vector
    let mut row_length;
    let mut i = 0;

    for row in rows {
        let mut vector_row = Vec::new();

        let row_elements : Vec<&str> = row.split(' ').collect();
        row_length = row_elements.len();

        if i == 0{
            row_length = row_elements.len();
            i += 1;
        }

        if row_length != row_elements.len(){
            process::exit(-2);
        }

        for row_element in row_elements{
            if row_element == ""{
                println!("Empty line detected. Input file shouldn't contain any empty lines");
                process::exit(-2);
            }
            let converted: f32 = row_element.trim().parse().unwrap();
            vector_row.push(converted);
        }
        matrix.push(vector_row);
    }
    return matrix;
}

pub fn write(file_name: &str, content: &Vec<f32>){
    let mut file_content: String = "".to_string();
    for i in 0..content.len(){
        file_content += &(content[i].to_string() + "\n");
    }

    let saving_result = fs::write(file_name, file_content);
    match saving_result{
        Ok(_file) => { },
        Err(_error) => { 
            println!("An error occurred while saving results to file. That's all I know :("); 
            process::exit(-2);
        }
    }
}

pub fn validate_coefficients_matrix(coefficients_matrix: &Vec<Vec<f32>>) -> bool{
    for i in 0..coefficients_matrix.len(){
        if coefficients_matrix[i].len() < coefficients_matrix.len(){ // just in case
            println!("Coefficients matrix is not square matrix");
            return false;
        }
        if coefficients_matrix[i][i] == 0.0{
            println!("0 value detected on diagonal of matrix");
            return false;
        }
    }
    return true;
}

pub fn validate_results_vector(equations_number: i32, y_values_vector: &Vec<Vec<f32>>) -> bool{
    if equations_number != y_values_vector.len() as i32{
        println!("Incorrect number of elements in results vector");
        return false;
    }

    for i in 0..y_values_vector.len(){
        if y_values_vector[i].len() != 1{
            println!("Incorrect number of elements in results vector at line {:?}", (i + 1));
            return false;
        }
    }
    return true;
}
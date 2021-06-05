import subprocess
from timeit import default_timer as timer
import os
from xlwt import Workbook

stats = ['''
        {
            threads_number: {int},
            square_matrix_size: {int},
            time: {int} # in milliseconds,
            method: {str} # GAUSS/JACOBI
        }
        ''']

print("Compiling solver...\n")
subprocess.call('rustc main.rs', shell=True) #os.system('cmd /k "rustc main.rs"') # parallel solver compilation (just in case it was removed)
print("Solver compiled...\nStart processing tests...\n")

number_of_variables = 5
for test_id in range(1, 14):
    print("Processing test nr {}\n".format(test_id))
    test_package_path = "../tests/test_package_" + str(test_id)
    coefficients_file_path = test_package_path + "/coefficients.txt"
    y_values_file_path = test_package_path + "/y_values.txt"

    # jacobi time measure
    for threads_number in range(1, 5):
        start = timer()
        command = '.\main.exe {} {} {} {} {} {}'.format(
                coefficients_file_path, 
                y_values_file_path,
                str(threads_number),
                str(30),
                "results_jacobi_" + str(test_id + 4),
                "jacobi"
            )

        subprocess.call(command, shell=True)
        end = timer()
        execution_time_in_milliseconds = (end - start) * 1000
        
        stats.append(
            (threads_number, number_of_variables, execution_time_in_milliseconds, "JACOBI")
        )
    
    # gauss time measure
    for threads_number in range(1, 5):
        start = timer()
        command = '.\main.exe {} {} {} {} {} {}'.format(
                coefficients_file_path, 
                y_values_file_path,
                str(threads_number),
                str(30),
                "results_gauss_" + str(test_id + 4),
                "gauss"
            )

        subprocess.call(command, shell=True)
        end = timer()
        execution_time_in_milliseconds = (end - start) * 1000
        
        stats.append(
            (threads_number, number_of_variables, execution_time_in_milliseconds, "GAUSS")
        )
    number_of_variables += 2

print("Tests data processed...\n")
print("Saving data to file...\n")
results_output_file_path = "./solver_stats.txt"

THREADS_NUMBER_IDX = 0
MATRIX_SIZE_IDX = 1
TIME_IDX = 2
METHOD_IDX = 3

# Workbook is created
wb = Workbook()
  
# add_sheet is used to create sheet.
jacobi_sheet = wb.add_sheet('Jacobi')
jacobi_sheet.write(0, 0, 'Liczba wątków')
jacobi_sheet.write(0, 1, 'Liczba niewiadomych')
jacobi_sheet.write(0, 2, 'Czas rozwiązania układu [ms]')

gauss_sheet = wb.add_sheet('Gauss-Seidel')
gauss_sheet.write(0, 0, 'Liczba wątków')
gauss_sheet.write(0, 1, 'Liczba niewiadomych')
gauss_sheet.write(0, 2, 'Czas rozwiązania układu [ms]')

jacobi_idx = 1
gauss_idx = 1

for i in range(0, len(stats)):
    if stats[i][METHOD_IDX] == "JACOBI":
        jacobi_sheet.write(jacobi_idx, 0, stats[i][THREADS_NUMBER_IDX])
        jacobi_sheet.write(jacobi_idx, 1, stats[i][MATRIX_SIZE_IDX])
        jacobi_sheet.write(jacobi_idx, 2, stats[i][TIME_IDX])
        jacobi_idx += 1
    elif stats[i][METHOD_IDX] == "GAUSS":
        gauss_sheet.write(gauss_idx, 0, stats[i][THREADS_NUMBER_IDX])
        gauss_sheet.write(gauss_idx, 1, stats[i][MATRIX_SIZE_IDX])
        gauss_sheet.write(gauss_idx, 2, stats[i][TIME_IDX])
        gauss_idx += 1

wb.save('[PRIR]Projekt_wyniki.xls')

print("Data saved...\n")
subprocess.call('del main.exe', shell=True)
print("Generating data ended...\n")
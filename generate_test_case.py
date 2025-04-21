"""
Simple script to generate test/<name>.txt.

Format for each line:
<array_size> <elem> <elem>
"""
from random import randint, randrange

FILE_NAME = "bubble_sort.txt"
RANDOM_RANGE = [1, 10000]
ARRAY_SIZE_RANGE = [RANDOM_RANGE[1]//6, RANDOM_RANGE[1]//2]
NUM_ITERATIONS = 1000000

benchmark_case_content = ""

for _ in range(NUM_ITERATIONS):
    line = ""

    # Array size:
    array_size = randint(*ARRAY_SIZE_RANGE)
    line += str(array_size) + " "

    # Random numbers:
    for _ in range(array_size):
        line += str(randrange(*RANDOM_RANGE)) + " "
    line = line[:-1] + "\n"
    benchmark_case_content += line

with open("test/" + FILE_NAME, "w+") as file:
    benchmark_case_content = benchmark_case_content[:-1]
    file.write(benchmark_case_content)

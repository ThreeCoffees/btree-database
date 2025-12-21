import numpy as np
import pandas as pd
import os

dir = "./test_logs/"
full_file = pd.DataFrame()

for file_name in os.listdir(dir):
    file_path = os.path.join(dir, file_name)
    if os.path.isfile(file_path):
        opened_file = pd.read_csv(file_path)
        opened_file['file_of_origin'] = file_name
        full_file = pd.concat([full_file, opened_file])

full_file.to_csv("full_test_file.csv")






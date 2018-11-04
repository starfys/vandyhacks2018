import pandas as pd
import os
import time
from mstrio import microstrategy
import json

base_url="https://env-113279.customer.cloud.microstrategy.com/MicroStrategyLibrary/api"
username="swr2u@mtmail.mtsu.edu"
password="graei39lxn8"
project_name="MicroStrategy Tutorial"

DATA_DIR = "data"

###########

data = os.path.join(DATA_DIR, "initial_data.csv")
df = pd.read_csv(data, index_col=False)

dataset_name = 'base_dataset'
table_name = 'base_dataset_table'

conn = microstrategy.Connection(base_url=base_url,
                                username=username,
                                password=password,
                                project_name=project_name)
conn.connect()

print("Creating new base dataset on Microstrategy")
dataset_id, new_table_id = conn.create_dataset(data_frame=df,
                                               dataset_name=dataset_name,
                                               table_name=table_name,
                                               to_attribute=['work_id',
                                                             'task_id'])

with open(os.path.join(DATA_DIR, "base_dataset_ids.txt"), 'w') as f:
    f.write(dataset_id + '\n' + new_table_id)

###########


data = os.path.join(DATA_DIR, "predictions.csv")
df = pd.read_csv(data, index_col=False)

dataset_name = 'pred_dataset'
table_name = 'pred_dataset_table'

conn = microstrategy.Connection(base_url=base_url,
                                username=username,
                                password=password,
                                project_name=project_name)
conn.connect()

print("Creating new pred dataset on Microstrategy")
dataset_id, new_table_id = conn.create_dataset(data_frame=df,
                                               dataset_name=dataset_name,
                                               table_name=table_name,
                                               to_attribute=['work_id',
                                                             'task_id'])

with open(os.path.join(DATA_DIR, "pred_dataset_ids.txt"), 'w') as f:
    f.write(dataset_id + '\n' + new_table_id)

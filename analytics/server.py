from bottle import post, run, request
import pandas as pd
import os
import time
from mstrio import microstrategy
import requests
import json

@post('/work')
def get_new_work():
    
    df = pd.read_json(json.dumps(request.json), typ='series', convert_dates=False).to_frame().T

    print("Adding to dataset on Microstrategy")
    if len(df) == 11:
        conn.update_dataset(data_frame=df, 
                            update_policy='add', 
                            dataset_id=dataset_id, 
                            table_name=table_name, 
                            table_id=new_table_id)
    elif len(df) == 12:
        conn.update_dataset(data_frame=df, 
                            update_policy='add', 
                            dataset_id=pred_dataset_id, 
                            table_name=pred_table_name, 
                            table_id=pred_new_table_id)


base_url="https://env-113279.customer.cloud.microstrategy.com/MicroStrategyLibrary/api"
username="swr2u@mtmail.mtsu.edu"
password="graei39lxn8"
project_name="MicroStrategy Tutorial"

DATA_DIR = "data"
base_datasets_id_file = os.path.join(DATA_DIR, "base_datasets_ids.txt")
table_name = "base_dataset_table"
pred_datasets_id_file = os.path.join(DATA_DIR, "pred_datasets_ids.txt")
pred_table_name = "pred_dataset_table"

data = os.path.join(DATA_DIR, "quiet.csv")
df = pd.read_csv(data, index_col=False)


with open(base_datasets_id_file, 'r') as f:
    dataset_id, new_table_id = f.readlines().split()

with open(pred_datasets_id_file, 'r') as f:
    pred_dataset_id, pred_new_table_id = f.readlines().split()


conn = microstrategy.Connection(base_url=base_url,
                                username=username,
                                password=password,
                                project_name=project_name)
conn.connect()

run(host="localhost", port=8080, debug=True)

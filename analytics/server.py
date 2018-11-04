from bottle import post, run, request
import pandas as pd
import os
import time
from mstrio import microstrategy
import requests
import json

@post('/work')
def get_new_work():
    print(request.json)

    # take json from request
    # turn the list into a dict
    # turn dict into pandas dataframe
    # send to MS
    # return success=happy
    
    df = pd.read_json(json.dumps(request.json), typ='series', convert_dates=False).to_frame().T
    
    print("Adding to dataset on Microstrategy")
    conn.update_dataset(data_frame=df, 
                        update_policy='add', 
                        dataset_id=dataset_id, 
                        table_name=table_name, 
                        table_id=new_table_id)


base_url="https://env-113279.customer.cloud.microstrategy.com/MicroStrategyLibrary/api"
username="swr2u@mtmail.mtsu.edu"
password="graei39lxn8"
project_name="MicroStrategy Tutorial"

DATA_DIR = "data"
data = os.path.join(DATA_DIR, "quiet.csv")
df = pd.read_csv(data, index_col=False)

dataset_name = 'dataset_name'
table_name = 'table_name'

conn = microstrategy.Connection(base_url=base_url,
                                username=username,
                                password=password,
                                project_name=project_name)
conn.connect()

print("Creating new dataset on Microstrategy")
dataset_id, new_table_id = conn.create_dataset(data_frame=df,
                                               dataset_name=dataset_name,
                                               table_name=table_name,
                                               to_attribute=['work_id',
                                                             'task_id'])


run(host="localhost", port=8080, debug=True)

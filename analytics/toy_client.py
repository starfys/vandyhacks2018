import requests
import os
import json
import pandas as pd

########### Testing launch game ###########  
url = "http://localhost:8080"
path = "/work"

x = 100000


for _ in range(5):

    x += 1

    df = json.dumps({"end_time":1541290912929.0,"music":0.0,"num_breaks":8.0,"num_interruptions":7.0,"num_meetings":1.0,"progress":0.6969148132,"start_time":1541290912671.0,"task_id":x,"workspace_volume":0.2105564469})
    r = requests.post(url+path, json=df,) 

    input()

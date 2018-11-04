import requests
import os
import json
import pandas as pd

########### Testing launch game ###########  
url = "http://localhost:8080"
path = "/work"

x = 100000
y = 100001



for _ in range(5):

    x += 1

    df = json.dumps({"breaks":0.0,"end_time":1541303061918.0,"finished":0.0,"interruptions":6.0,"meetings":3.0,"music":0.0,"noise":0.4934773453,"progress":0.4520924592,"start_time":1541303061283.0,"task_id":y, "work_id":x})
    r = requests.post(url+path, json=df,) 

    input()

import requests
import os
import json
import pandas as pd

########### Testing launch game ###########  
url = "http://localhost:8080"
path = "/work"


df = pd.read_json(os.path.join("data", "quiet.json"),
                            convert_dates=False)

df = df.iloc[0].to_json(date_unit='us', orient='index')# first row


r = requests.post(url+path, json=df,) 

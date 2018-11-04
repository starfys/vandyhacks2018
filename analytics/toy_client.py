import requests
import json

########### Testing launch game ###########  
url = "http://localhost:8080"
path = "/work"
data = [{"id": "touhou_123"}, 
        {"id": "melty_blood"}]

r = requests.post(url+path, json=data,) 

import os
import pandas as pd
import numpy as np
import json
import uuid
import random
from time import time

# random task_id
cur_id = -1
def gen_task_id():
    global cur_id
    cur_id += 1
    return cur_id

def gen_work_id():
    return random.randint(0,int(2**32))

# random start time in num milliseconds
def gen_start_time(mu):
    return int(time()*1e3 + random.gauss(mu, 400))

def gen_end_time(start, mu):
    return int(start + random.gauss(mu, 400))

def gen_music(mu):
    roll = random.gauss(mu, 0.5)
    if roll <= 0.5:
        return 0
    else:
        return 1
    
def gen_interruptions(mu):
    return np.abs(int(random.gauss(mu, 3)))

def gen_workspace_volume(mu):
    roll = random.gauss(mu, 0.5)
    if roll > 1:
        return 1
    if roll < 0:
        return 0
    return roll
    
def gen_meetings(mu):
    return np.abs(int(random.gauss(mu, 3)))

def gen_breaks(mu):
    return np.abs(int(random.gauss(mu, 5)))

def gen_progress(mu):
    val = np.abs(random.gauss(mu, 0.2))
    if val > 1:
        val = 1
    return val

def gen_row(time_mu, 
            music_mu, 
            interruptions_mu, 
            volume_mu, 
            meetings_mu, 
            breaks_mu, 
            progress_mu):
    start = gen_start_time(time_mu)
    return np.array([
        gen_task_id(),
        gen_work_id(),
        start,
        gen_end_time(start, time_mu),
        gen_music(music_mu),
        0,
        gen_interruptions(interruptions_mu),
        gen_workspace_volume(volume_mu),
        gen_meetings(meetings_mu),
        gen_breaks(breaks_mu),
        gen_progress(progress_mu),
    ])

def gen_quiet_row():
    return gen_row(time_mu=500,
                    music_mu=0.2,
                    interruptions_mu=3,
                    volume_mu=0.1,
                    meetings_mu=2,
                    breaks_mu=6,
                    progress_mu=0.7
                   )

def gen_loud_row():
    return gen_row(time_mu=500,
                    music_mu=0.7,
                    interruptions_mu=7,
                    volume_mu=0.7,
                    meetings_mu=7,
                    breaks_mu=1,
                    progress_mu=0.2
                   )


columns = ['task_id',
           'work_id',
           'start_time',
           'end_time',
           'music',
           'finished',
           'interruptions',
           'noise', 
           'meetings',
           'breaks',
           'progress']


loud_dataset = np.array([gen_loud_row() for x in range(3000)])
loud_df = pd.DataFrame(data=loud_dataset, columns=columns)

quiet_dataset = np.array([gen_quiet_row() for x in range(7000)])
quiet_df = pd.DataFrame(data=quiet_dataset, columns=columns)

seeded_df = quiet_df.append(loud_df, ignore_index=True)
seeded_df.to_json("initial_data.json")
seeded_df.to_csv("initial_data.csv", index=False)

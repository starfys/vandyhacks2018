import os
import pandas as pd
import numpy as np
import json

from keras.utils import to_categorical
from keras.engine import Input, Model
from keras.layers import Dense, Flatten
from keras.layers.core import Activation
from keras.models import load_model
from keras.optimizers import Adam
import keras.backend as K

DATA_DIR = "data"
quiet_csv = os.path.join(DATA_DIR, "initial_data.csv")

df = pd.read_csv(quiet_csv)

# test regression: predict balance by age, job, marital, education, housing, duration of last contac
columns = df.columns
drop = ['task_id', 
        'work_id',
        'finished',
        'start_time', 
        'end_time',
        'progress']

X = df.drop(drop, axis=1)

# normalize X with column-wise linear downscale
X = X.divide(X.max(axis=0))

model_path = os.path.join("models", "weights.hdf5")
model = load_model(model_path)

preds = model.predict(X,
          batch_size=1024,
          verbose=1)

df_new = df.copy()
df_new['predicctions'] = preds 
df_new.to_csv(os.path.join(DATA_DIR, "predictions.csv"), index=False)

K.clear_session()

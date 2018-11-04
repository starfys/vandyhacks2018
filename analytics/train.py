import os
import pandas as pd
import numpy as np
import json

from sklearn.utils import shuffle

from keras.utils import to_categorical
from keras.engine import Input, Model
from keras.layers import Dense, Flatten
from keras.layers.core import Activation
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
        'end_time',]
# prune unused features
df = df.drop(drop, axis=1)

# set up variables
y = df['progress']
X = df.drop('progress', axis=1)

# feature engineering could go here
# set up duration, split start time into categorical or integer value
# that corresponds to time of day (morn/aft/night)
# and adjust the X datarrame appropriately

# normalize X with column-wise linear downscale
X = X.divide(X.max(axis=0))


X, y = shuffle(X, y, random_state=0)

# build basic dnn
def dnn(input_shape, model_path, lr=1e-4, verbose=0):
    inputs = Input(shape=input_shape[1:])

    x = Dense(64, activation='relu')(inputs)
    x = Dense(32, activation='relu')(x)
    x = Dense(1)(x)
    
    outputs = Activation('sigmoid')(x)
    
    model = Model(inputs=inputs, outputs=outputs)

    # dice as a human-readble metric 
    model.compile(optimizer=Adam(lr=lr),
                  metrics=['mae'],
                  loss='mse')

    # save json
    json_string = model.to_json()
    with open(model_path, 'w') as f:
        json.dump(json_string, f)

    # selectively print model
    if verbose:
        print(model.summary())

    return model

model_path = os.path.join("models", "dnn.json")
model = dnn(X.shape, model_path, verbose=1)

model.fit(X, y, 
          epochs=1000,
          batch_size=1024,
          validation_split=0.5,
          verbose=1)

model.save(os.path.join("models", "weights.hdf5"))

K.clear_session()

import matplotlib as mpl
mpl.use('Agg')

import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
import os

plt.rcParams["figure.figsize"] = [25,20]
plt.rcParams.update({'font.size': 35})
plt.rcParams['axes.xmargin'] = 0.01

data = pd.read_csv("out/")
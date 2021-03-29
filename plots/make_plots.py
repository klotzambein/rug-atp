import matplotlib as mpl
mpl.use('Agg')

import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
import os

plt.rcParams["figure.figsize"] = [25,20]
plt.rcParams.update({'font.size': 35})
plt.rcParams['axes.xmargin'] = 0.01

baseline = []
low = []
high = []

for root, dirs, files in os.walk("out"):
    for file_name in files:
        _data = pd.read_csv(os.path.join(root,file_name))
        if file_name.split("_")[0] == "baseline":
            baseline.append(_data)
        if file_name.split("_")[0] == "low":
            low.append(_data)
        if file_name.split("_")[0] == "high":
            high.append(_data)

baseline = pd.concat(baseline, axis = 0, ignore_index=True)
low = pd.concat(low, axis = 0, ignore_index=True)
high = pd.concat(high, axis = 0, ignore_index=True)

baseline = baseline.groupby("tick").mean()
low = low.groupby("tick").mean()
high = high.groupby("tick").mean()

data = [baseline, low, high]
data_name = ["baseline", "low", "high"]
i=0

for _data in data:
    _data["agent_count"] = (_data["agent_count"] / _data["agent_count"].max()) * 3000
    # Job Counts
    plt.figure()
    plt.plot(_data.index / 200, _data["job_counts_explorer"], linewidth=1.0, label="Explorer")
    plt.plot(_data.index / 200, _data["job_counts_farmer"], linewidth=1.0, label="Farmer")
    plt.plot(_data.index / 200, _data["job_counts_lumberer"], linewidth=1.0, label="Lumberer")
    plt.plot(_data.index / 200, _data["job_counts_fisher"], linewidth=1.0, label="Fisher")
    plt.plot(_data.index / 200, _data["job_counts_butcher"], linewidth=1.0, label="Butcher")
    plt.xlabel("Time (ticks)")
    plt.ylabel("Job Frequency")
    plt.title("Job Frequency Over 10 Runs")
    plt.legend()
    plt.savefig("plots/job_counts_{}.png".format(data_name[i]))

    # Prices
    plt.figure()
    plt.plot(_data.index, _data["prices_wheat"], linewidth=1.0, label="Wheat")
    plt.plot(_data.index, _data["prices_berry"], linewidth=1.0, label="Berry")
    plt.plot(_data.index, _data["prices_fish"], linewidth=1.0, label="Fish")
    plt.plot(_data.index, _data["prices_meat"], linewidth=1.0, label="Meat")
    plt.xlabel("Time (ticks)")
    plt.ylabel("Resource Price")
    plt.title("Mean Prices Over 10 Runs")
    plt.legend()
    plt.savefig("plots/prices_{}.png".format(data_name[i]))

    # Volume
    plt.figure()
    plt.plot(_data.index, _data["volume_wheat"], linewidth=1.0, label="Wheat")
    plt.plot(_data.index, _data["volume_berry"], linewidth=1.0, label="Berry")
    plt.plot(_data.index, _data["volume_fish"], linewidth=1.0, label="Fish")
    plt.plot(_data.index, _data["volume_meat"], linewidth=1.0, label="Meat")
    plt.xlabel("Time (ticks)")
    plt.ylabel("Resource Volume")
    plt.title("Mean Resource Volumes Over 10 Runs")
    plt.legend()
    plt.savefig("plots/volumes_{}.png".format(data_name[i]))

    i+=1

# Alive agents
plt.figure()
plt.plot(baseline.index, (baseline["agent_count"] / baseline["agent_count"].max()) * 3000 , linewidth=1.0, label="Baseline")
plt.plot(low.index, (low["agent_count"] / low["agent_count"].max()) * 3000, linewidth=1.0, label="Low Greed")
plt.plot(high.index, (high["agent_count"] / high["agent_count"].max()) * 3000, linewidth=1.0, label="High Greed")
plt.xlabel("Time (ticks)")
plt.ylabel("Alive Agents")
plt.title("Mean Number of Alive Agents Over 10 Runs")
plt.legend()
plt.savefig("plots/agent_count.png")

# Greed
plt.figure()
plt.scatter(baseline["alive_time"], baseline["greed"], linewidth=1.0, label="Baseline")
plt.scatter(low["alive_time"], low["greed"], linewidth=1.0, label="Low Greed")
plt.scatter(high["alive_time"], high["greed"], linewidth=1.0, label="High Greed")
plt.xlabel("Time alive (ticks)")
plt.ylabel("Greed")
plt.title("Mean Greed Over 10 Runs")
plt.legend()
plt.savefig("plots/greed_scatter.png")

# Cash $$$
plt.figure()
plt.scatter(baseline.index, baseline["cash"], linewidth=1.0, label="Baseline")
plt.scatter(low.index, low["cash"], linewidth=1.0, label="Low Greed")
plt.scatter(high.index, high["cash"], linewidth=1.0, label="High Greed")
plt.xlabel("Time (ticks)")
plt.ylabel("Cash")
plt.title("Mean Cash of All Agents Over 10 Runs")
plt.legend()
plt.savefig("plots/greed_scatter.png")

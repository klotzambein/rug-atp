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

baseline_agents = []
low_agents = []
high_agents = []

for root, dirs, files in os.walk("out"):
    for file_name in files:
        if file_name.split(".")[-1] == "zip":
            continue
        _data = pd.read_csv(os.path.join(root,file_name))
        if file_name.split("_")[0] == "baseline":
            if file_name.split(".")[1] == "agents":
                baseline_agents.append(_data)
            else:
                baseline.append(_data)
        if file_name.split("_")[0] == "low":
            if file_name.split(".")[1] == "agents":
                low_agents.append(_data)
            else:
                low.append(_data)
        if file_name.split("_")[0] == "high":
            if file_name.split(".")[1] == "agents":
                high_agents.append(_data)
            else:
                high.append(_data)

baseline = pd.concat(baseline, axis = 0, ignore_index=True)
low = pd.concat(low, axis = 0, ignore_index=True)
high = pd.concat(high, axis = 0, ignore_index=True)

baseline_agents = pd.concat(baseline_agents, axis = 0, ignore_index=True)
low_agents = pd.concat(low_agents, axis = 0, ignore_index=True)
high_agents = pd.concat(high_agents, axis = 0, ignore_index=True)

baseline = baseline.groupby("tick").mean()
low = low.groupby("tick").mean()
high = high.groupby("tick").mean()

data = [baseline, low, high]
data_agents = [baseline_agents, low_agents, high_agents]
data_name = ["baseline", "low", "high"]
i=0

for _data in data:
    _data["agent_count"] = (_data["agent_count"] / _data["agent_count"].max()) * 3000
    # Job Counts
    plt.figure()
    plt.plot(_data.index / 200, _data["job_counts_explorer"], label="Explorer")
    plt.plot(_data.index / 200, _data["job_counts_farmer"], label="Farmer")
    plt.plot(_data.index / 200, _data["job_counts_lumberer"], label="Lumberer")
    plt.plot(_data.index / 200, _data["job_counts_fisher"], label="Fisher")
    plt.plot(_data.index / 200, _data["job_counts_butcher"], label="Butcher")
    plt.xlabel("Time (days)")
    plt.ylabel("Job Frequency")
    plt.title("Job Frequency Over 10 Runs")
    plt.legend(markerscale=3.)
    plt.savefig("plots/job_counts_{}.png".format(data_name[i]))

    # Prices
    plt.figure()
    plt.plot(_data.index / 200, _data["prices_wheat"], label="Wheat")
    plt.plot(_data.index / 200, _data["prices_berry"], label="Berry")
    plt.plot(_data.index / 200, _data["prices_fish"], label="Fish")
    plt.plot(_data.index / 200, _data["prices_meat"], label="Meat")
    plt.xlabel("Time (days)")
    plt.ylabel("Resource Price")
    plt.title("Mean Prices Over 10 Runs")
    plt.legend(markerscale=3.)
    plt.savefig("plots/prices_{}.png".format(data_name[i]))

    # Volume
    plt.figure()
    plt.plot(_data.index / 200, _data["volume_wheat"], label="Wheat")
    plt.plot(_data.index / 200, _data["volume_berry"], label="Berry")
    plt.plot(_data.index / 200, _data["volume_fish"], label="Fish")
    plt.plot(_data.index / 200, _data["volume_meat"], label="Meat")
    plt.xlabel("Time (days)")
    plt.ylabel("Resource Volume")
    plt.title("Mean Resource Volumes Over 10 Runs")
    plt.legend(markerscale=3.)
    plt.savefig("plots/volumes_{}.png".format(data_name[i]))

    i+=1

# Alive agents
plt.figure()
plt.plot(baseline.index / 200, baseline["agent_count"], label="Baseline")
plt.plot(low.index / 200, low["agent_count"], label="Low Greed")
plt.plot(high.index / 200, high["agent_count"], label="High Greed")
plt.xlabel("Time (days)")
plt.ylabel("Alive Agents")
plt.title("Mean Number of Alive Agents Over 10 Runs")
plt.legend(markerscale=3.)
plt.savefig("plots/agent_count.png")

# Greed scatter
plt.figure()
plt.scatter(baseline_agents["lifetime"] / 200, baseline_agents["greed"], label="Baseline")
plt.scatter(low_agents["lifetime"] / 200, low_agents["greed"], label="Low Greed")
plt.scatter(high_agents["lifetime"] / 200, high_agents["greed"], label="High Greed")
plt.xlabel("Lifetime (days)")
plt.ylabel("Greed")
plt.title("Interaction between Agent Lifetime and Greed Over 10 Runs")
plt.legend(markerscale=3.)
plt.savefig("plots/greed_scatter.png")

# Greed over time
plt.figure()
plt.scatter(baseline.index / 200, baseline["agent_greed"], label="Baseline")
plt.scatter(low.index / 200, low["agent_greed"], label="Low Greed")
plt.scatter(high.index / 200, high["agent_greed"], label="High Greed")
plt.xlabel("Time (days)")
plt.ylabel("Greed")
plt.title("Mean Greed of Alive Agents Over 10 Runs")
plt.legend(markerscale=3.)
plt.savefig("plots/agent_greed.png")

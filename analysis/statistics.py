from scipy import stats
import pandas as pd

baseline = [1107, 1041, 858, 1280, 1242, 1186, 804, 1142, 1233, 1136]
high_greed = [413, 477, 343, 366, 429, 473, 466, 344, 408]
low_greed = [1105, 1605, 1146, 1118, 1163, 1102, 1129, 1341, 1456, 1371]

print("Low Greed vs High Greed")
t_value, p_value = stats.ttest_ind(low_greed, high_greed)
print("  T-value: {}\n  p-value: {}".format(t_value, p_value))

print("Baseline vs High Greed")
t_value, p_value = stats.ttest_ind(baseline, high_greed)
print("  T-value: {}\n  p-value: {}".format(t_value, p_value))

print("Baseline vs Low Greed")
t_value, p_value = stats.ttest_ind(baseline, low_greed)
print("  T-value: {}\n  p-value: {}".format(t_value, p_value))

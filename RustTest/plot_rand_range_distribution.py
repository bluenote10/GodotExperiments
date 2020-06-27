#!/usr/bin/env python

import os
import numpy as np
import matplotlib.pyplot as plt

data = open(os.path.expanduser("~/.local/share/godot/app_userdata/RustTest/debug_data.save")).read()
data = eval(data)

num_bins = 100

fig, ax = plt.subplots(1, 1, figsize=(16, 5))

counts, bin_edges = np.histogram(data, bins=num_bins)
bin_centers = 0.5 * (bin_edges[1:] + bin_edges[:-1])
errors = np.sqrt(counts)
width = (bin_edges[-1] - bin_edges[0]) / num_bins
ax.bar(bin_centers, counts, width=width, color='r', yerr=errors)

fig, ax = plt.subplots(1, 1, figsize=(16, 5))
plt.plot(data, range(len(data)), "o", ms=1)

plt.show()

# import IPython; IPython.embed()
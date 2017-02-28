#!/usr/bin/env python3

import csv
import argparse
import sys
import matplotlib.pyplot as plt
from common import *
from collections import defaultdict

setup()

with_year_in_password_per_year = defaultdict(lambda: 0)
all_per_year = defaultdict(lambda: 0)
all_count = 0

for row in DATA:
  try:
    year = int(row['YearOfBirth'])
    all_count += 1
    all_per_year[year] += 1
    if birth_year_in_password(row):
      with_year_in_password_per_year[year] += 1
  except ValueError:
    pass

def annotate(x_axis, y_axis):
  for i, year in enumerate(x_axis):
    plt.annotate(
          str(year), xy=(year, y_axis[i]),
          # xytext=(0, 10),
          # textcoords='offset points',
          # ha='right', va='bottom',
          # bbox=dict(boxstyle='round,pad=0.5', fc='yellow', alpha=0.5),
          # arrowprops=dict(arrowstyle = '->', connectionstyle='arc3,rad=0')
    )

x_axis = sorted(list(all_per_year.keys()))

print("Birth: min {}, max {}".format(x_axis[0], x_axis[len(x_axis) - 1]))

y_axis = [(with_year_in_password_per_year[k] * 100 / all_per_year[k]) for k in x_axis]

plt.scatter(x_axis, y_axis)

plt.title("Percentage of users with their birth year in the password per year");
plt.savefig('data/year-in-password-per-year.png')

annotate(x_axis, y_axis)
plt.savefig('data/year-in-password-per-year-annotated.png')

plt.figure(2)

y_axis = [with_year_in_password_per_year[k] for k in x_axis]
plt.scatter(x_axis, y_axis)
plt.title("Number of users with their birth year in the password per year");
plt.savefig('data/year-in-password-total.png')
annotate(x_axis, y_axis)
plt.savefig('data/year-in-password-total-annotated.png')

plt.figure(3)

y_axis = [(100 * with_year_in_password_per_year[k] / all_count) for k in x_axis]
plt.scatter(x_axis, y_axis)
plt.title("Percentage of users with their birth year in the password per year");
plt.savefig('data/year-in-password-total-percentage.png')
annotate(x_axis, y_axis)
plt.savefig('data/year-in-password-total-percentage-annotated.png')

plt.figure(4)

y_axis = [all_per_year[k] for k in x_axis]
plt.scatter(x_axis, y_axis)
plt.title("Number of users per year in the data");
plt.savefig('data/per-year-distribution.png')
annotate(x_axis, y_axis)
plt.savefig('data/per-year-distribution-annotated.png')

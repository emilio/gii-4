#!/usr/bin/env python3

import csv
from common import *


setup()

def filter_data(out_file, filter_fn):
  count = 0
  with open(out_file, 'w') as out:
    writer = csv.writer(out)
    writer.writerow(HEADER)
    for data in filter(filter_fn, DATA):
      row = [data[i] for i in HEADER]
      count += 1
      writer.writerow(row)

  print("{}:\t{} rows".format(out_file, count))



filter_data('./data/pws-numeric.csv', all_numbers)
filter_data('./data/pws-any-number.csv', any_number)
filter_data('./data/pws-date-like.csv', date_like)
filter_data('./data/pws-birth-year.csv', birth_year_in_password)

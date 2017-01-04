#!/usr/bin/env python3

import csv

DATA = []
HEADER = []

with open('./data/pws.csv') as f:
  reader = csv.reader(f)
  HEADER = next(reader)

  for row in reader:
    obj = {}
    for i, val in enumerate(row):
      obj[HEADER[i]] = val
    DATA.append(obj)


def filter_data(out_file, filter_fn):
  with open(out_file, 'w') as out:
    writer = csv.writer(out)
    writer.writerow(HEADER)
    for data in filter(filter_fn, DATA):
      row = [data[i] for i in HEADER]
      writer.writerow(row)


def all_numbers(row):
  return row['Password'].isdigit()


def any_number(row):
  return any(c.isdigit() for c in row['Password'])


filter_data('./data/pws-numeric.csv', all_numbers)
filter_data('./data/pws-any-number.csv', any_number)

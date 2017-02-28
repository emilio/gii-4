#!/usr/bin/env python3

import csv

DATA = []
HEADER = []

# XXXEmilio this is far from clean.
def setup():
  with open('./data/pws.csv') as f:
    reader = csv.reader(f)
    HEADER = next(reader)

    for row in reader:
      obj = {}
      for i, val in enumerate(row):
        obj[HEADER[i]] = val
      DATA.append(obj)

def all_numbers(row):
  return row['Password'].isdigit()


def any_number(row):
  return any(c.isdigit() for c in row['Password'])


def string_to_date(password):
  if len(password) == 6:
    return string_to_date_6(password)
  if len(password) == 8:
    return string_to_date_8(password)
  return None


def string_to_date_6(password):
  assert len(password) == 6
  pieces = [
      password[0:2],
      password[2:4],
      password[4:6],
  ]
  month = None
  day = None
  year = None
  for piece in pieces:
    if not month and int(piece) > 0 and int(piece) <= 12:
      month = piece
      continue
    if not day and int(piece) > 0 and int(piece) <= 31:
      day = piece
      continue
    if year:
      return None
    year = piece

  if not month or not day or not year:
    return None

  return [day, month, year]


def string_to_date_8(password):
  assert len(password) == 8
  combinations = [
    [
      password[0:2],
      password[2:4],
      password[4:8],
    ],
    [
      password[0:2],
      password[2:6],
      password[6:8],
    ],
    [
      password[0:4],
      password[4:6],
      password[6:8],
    ],
  ]

  for combination in combinations:
    month = None
    day = None
    year = None
    for piece in combination:
      if len(piece) == 4:
        assert not year
        year = int(piece)

      if not month and int(piece) > 0 and int(piece) <= 12:
        month = piece
        continue

      if not day and int(piece) > 0 and int(piece) <= 31:
        day = piece
        continue
      return None

    if not month or not day or not year:
      return None

    return [day, month, year]

  return None


def date_like(row):
  if not all_numbers(row):
    return False
  password = row['Password']
  # TODO(emilio): Handle 8-digit passwords.
  if len(password) != 6:
    return False
  return string_to_date(password) is not None


def birth_year_in_password(row):
  if not all_numbers(row):
    return False
  password = row['Password']
  ret = string_to_date(password)
  if ret is None:
    return False
  year = ret[2]
  assert len(year) == 2 or len(year) == 4
  if len(year) == 2:
    return year == row['YearOfBirth'][2:4]
  return year == row['YearOfBirth']



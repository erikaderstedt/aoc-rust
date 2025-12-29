# get_input.py

import datetime
import os
import sys

f = open("cookie.txt", "r")
cookie = f.read().strip()
f.close()
if len(sys.argv) < 3:
    day_of_month = datetime.date.today().day
    year = datetime.date.today().year
else:
    year = int(sys.argv[1])
    day_of_month = int(sys.argv[2])

cmd = f"curl https://adventofcode.com/{year}/day/{day_of_month}/input --cookie 'session={cookie}' -o inputs/{year}/day{day_of_month:02d}.in"

print(cmd)
os.system(cmd)
print(f"zed inputs/{year}/day{day_of_month:02d}.in")

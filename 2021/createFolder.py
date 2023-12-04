import os
import datetime
import shutil

day = datetime.date.today().day
dirpath = os.getcwd() + "/Tag " + str(day)
os.mkdir(dirpath)

pypath = dirpath+ "/Tag" + str(day) + ".py"
shutil.copy(os.getcwd() + "/Template.py", pypath)

f = open(dirpath + "/Input", "x")
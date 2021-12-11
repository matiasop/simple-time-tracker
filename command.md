# Command examples

## linux

cargo run "data/days.json" "data/weeks.json" "data/months.json"

/home/matiasop98/tmp/simple-time-tracker "/home/matiasop98/tmp/days.json" "/home/matiasop98/tmp/weeks.json" "/home/matiasop98/tmp/months.json"

#!/bin/bash

/home/matisop/tmp/time-tracker/simple-time-tracker "/home/matisop/tmp/time-tracker/days.json" "/home/matisop/tmp/time-tracker/months.json"

## windows

"C:\tmp\time-tracker\simple-time-tracker.exe" "C:\tmp\time-tracker\data\days.json" "C:\tmp\time-tracker\data\weeks.json" "C:\tmp\time-tracker\data\months.json"

Add it to task Scheduler

## windows bat file

Create a file named `time-tracker.bat` with this content:

```
@echo off 


start /d "C:\tmp\time-tracker\" simple-time-tracker.exe "C:\tmp\time-tracker\days.json" "C:\tmp\time-tracker\weeks.json" "C:\tmp\time-tracker\months.json"
```

Create a shortcut to this file

Run Windows + R, type shell:startup and add the shortcut to this folder.

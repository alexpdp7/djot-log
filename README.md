Parses a log in Markdown format, printing the balance of hours against a target, and the logs for the current day.

```
$ djot-log example.md
Balance:

day: 2023-12-04 8h 0m, delta minutes 0
day: 2023-12-03 8h 0m, delta minutes 0

Logs for 2023-12-04:

2023-12-04 09:00:00-13:00:00 Coding // Work / MyOrg / MyDept / MyProj
2023-12-04 14:00:00-18:00:00 Coding // Work / MyOrg / MyDept / MyProj
```

```
$ djot-log example_unbalanced.md
Balance:

day: 2023-12-06 7h 50m, delta minutes 0
day: 2023-12-05 8h 10m, delta minutes 10
day: 2023-12-04 8h 0m, delta minutes 0

Logs for 2023-12-06:

2023-12-06 09:00:00-16:50:00 Work
```

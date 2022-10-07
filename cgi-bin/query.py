# -*- coding: UTF-8 -*-
import sys
import sqlite3

print("Content-type:text/html")
print("陈林峰")
print()

argv = sys.argv[1:]
id = argv[0].split("=")[1]  # 学号

conn = sqlite3.connect("data/database.db")
cursor = conn.cursor()
cursor.execute('''
    CREATE TABLE IF NOT EXISTS student(
    id text PRIMARY KEY NOT NULL,
    name text NOT NULL,
    class text NOT NULL
    );''')

info = cursor.execute("SELECT id, name, class  from student where id = ?", (id,))
count = 0

print("<!DOCTYPE html>")
print("<html>")
print("<head>")
print("<meta charset=\"UTF-8\">")
print("<title>上传</title>")
print("</head>")

print("<body>")

for row in info:
    count += 1
    print("ID: ", row[0])
    print("<br>")
    print("Name: ", row[1])
    print("<br>")
    print("Class: ", row[2])

if count == 0:
    print("<h1>没有此学生信息</h1>")

print("</body>")

# print("<script>")
# print("alert(\"{}\")".format(info))
# print("</script>")
conn.commit()
conn.close()

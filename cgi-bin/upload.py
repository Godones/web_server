# -*- coding: UTF-8 -*-
import sqlite3
import sys

argv = sys.argv[1:]
id = argv[0].split("=")[1]  # 学号
name = argv[1].split("=")[1]  # 姓名
s_class = argv[2].split("=")[1]  # 班级

conn = sqlite3.connect("data/database.db")
cursor = conn.cursor()
cursor.execute('''
    CREATE TABLE IF NOT EXISTS student(
    id text PRIMARY KEY NOT NULL,
    name text NOT NULL,
    class text NOT NULL
    );''')

cursor.execute("INSERT OR IGNORE INTO student VALUES(?,?,?)", (id, name, s_class))

conn.commit()
conn.close()

print("Content-type:text/html")
print()

print("<!DOCTYPE html>")
print("<html>")
print("<head>")
print("<meta charset=\"UTF-8\">")
print("<title>上传</title>")
print("</head>")
print("<body>")
print("<h1>学生信息已上传</h1>")
print("</body>")


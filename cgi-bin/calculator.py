import sys
print("Content-type:text/html")
print()

argv = sys.argv[1:]

first_number = float(argv[0].split("=")[1])
second_number = float(argv[1].split("=")[1])

print("<h1>")
print("The answer is: ")
print(first_number+second_number)
print("</h1>")

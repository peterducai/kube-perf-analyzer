#!/usr/bin/python3

import csv

etcd_warning_list = []


class EtcdWarning:

    def __init__(self, name, description, color, threshold, grep):
        self.name = name
        self.count = 0
        self.description = description
        self.color = color
        self.threshold = threshold
        self.grep = grep
        #self.threshold_error = threshold_error
        #self.color = color


with open('warnings.csv', newline='') as f:
    reader = csv.reader(f, delimiter=':', quoting=csv.QUOTE_NONE)
    for row in reader:
        print(row)
        #for i in etcd_warning_list:
        etcd_warning_list.append(EtcdWarning(row))

 

 
# # Accessing object value using a for loop
# for obj in list:
#     print(obj.name, obj.roll, sep=' ')
 
# print("")
# # Accessing individual elements
# print(list[0].name)
# print(list[1].name)
# print(list[2].name)
# print(list[3].name)


#define text file to open
##my_file = open('my_data.txt', 'r')

#read text file into list 
##data = my_file.read()

#display content of text file
##print(data)
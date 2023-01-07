"""
Script that turns all files in the data/ngrams folder into a sqlite database,
where the filenames are used as the table names
"""

import sqlite3
import os

def fill_db(name, data):
    """
    Creates a 2-column table in the database. 
    Column 1 is the word, column 2 is a space-separated list of years and counts
    i.e.
    ('airport', '2007 175702 2008 173294')
    ('request', '2005 646179 2006 677820 2007 697645 2008 795265')
    ('wandered', '2005 83769 2006 87688 2007 108634 2008 171015')

    You might be wondering, "why not a 3 column table, with columns 
    (word, year, freq)?". Unfortunately, that schema makes SELECT queries slow
    as shiitake mushrooms. This schema lets the word be used as a primary key,
    speeding up SELECT queries dramatically.
    """
    con = sqlite3.connect("ngordnet.db")
    cur = con.cursor()

    cur.execute(f"DROP TABLE IF EXISTS {name}")
    cur.execute(f"CREATE TABLE {name}(word TEXT NOT NULL PRIMARY KEY, data TEXT NOT NULL)")
    cur.executemany(f"INSERT INTO {name} VALUES(?, ?)", data)
    con.commit()


def read_file(path):
    mappings = {}
    with open(path) as f:
        for line in f.readlines():
            s = line.split('\t')
            if len(s) >= 3:
                ar = mappings.setdefault(s[0], [])
                ar.append(s[1])
                ar.append(s[2])
    
    return [(k, " ".join(v)) for k, v in mappings.items()]

def convert_file_to_db(path):
    filename = os.path.basename(path).split(".")[0]
    data = read_file(path)
    fill_db(filename, data)


if __name__ == "__main__":
    for filename in os.listdir("data/ngrams"):
        convert_file_to_db("data/ngrams/" + filename)



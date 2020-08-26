#!/bin/bash

if [ "$#" -ne 4 ]; then
  echo "Illegal number of parameters"
  echo "Usage is: loadWild.sh name programLink platform primaryDomain"
  exit 3
fi

# Step 1: Insert the program, this will become part of the shid binary in the near future
echo "INSERT INTO programs (name, link, platform, created_at, updated_at) VALUES ('$1', '$2', '$3', NOW(), NOW());" | docker exec -i pg-docker psql -U postgres -d shid

# Step 2: Use Shid to add domain to the new program
# If shid isn't in your $PATH what are you even doing?
echo $4 | shid addDomain -p $1 -s seed

# Step 3: Run subdomain enumeration and store results
# 3a - findomain
~/tools/findomain/findomain -t $4 -q | shid addDomain -p $1 -s findomain;

# 3b - amass
amass enum -d $4 | shid addDomain -p $1 -s amass;

# 3c - subfinder
subfinder -d $4 | shid addDomain -p $1 -s subfinder;

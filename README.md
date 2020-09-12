# Shid - ABANDONED

I've decided to stop development on this project. Using Rust for this was a bad call since this project is mainly gluing things together between the command line and a database, which is far better suited to a scripting language.

## Overview

This is my first crack at a recon framework that will assist in my Bug Bounty Hunting. The goal of this
repo is going to be to automatically find domains, servers, and endpoints that are in scope for a bug
bounty program, and categorize them in a way that is useful later. This could take the form of looking
for entites which match a particular signature, taking screenshots of endpoints with a given set of 
parameters, or just storing responses for later.


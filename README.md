# Fast Filters Library

A Rust library containing several competitive variants of Approximate Membership Query Data Structures (AMQDS) also known as Filters. 
This project was made as part of the [Individual Project](https://www.bris.ac.uk/unit-programme-catalogue/UnitDetails.jsa?ayrCode=23%2F24&unitCode=COMS30045) at the University of Bristol.

## Table of Contents

1. [Overview](#overview)
2. [Manual Setup](#manual-setup)
3. [Documentation](#documentation)
4. [Contributing](#contributing)
5. [Implementations](#implementations)


## Overview

Our goal is to produce a competitive library of configurable AMQDS that can be easily applied by other software engineers without the need for additional optimisations.
An AMQDS, F, is an over approximation of a set, S, that answers membership queries. There is a small false positive, \(\epsilon\) that F returns True when an element is not in the set. However,
if an element is in the set, then it is guaranteed to return True. They are space efficient and offer very fast membership queries and are used to avoid expensive operations like disk or network accesses.



## Manual Setup

### Requirements

- [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
- [Rust Rover](https://www.jetbrains.com/rust/nextversion/) with Rust extensions is optional but recommended.


## Documentation

Look through our [wiki]() that we will be updating throughout production.



## Contributing

[We have a simple guide](/CONTRIBUTING.md) on how to use `git` with this project.

## Implementations
The following AMQDS have been implemented. Note that as of the last update, the Blocked Bloom Filter and Register Aligned Bloom Filter are not vectorised.

1. Bloom Filter
2. Blocked Bloom Filter
3. Counting Bloom Filter
4. Register Aligned Bloom Filter
5. Cuckoo Filter
6. XOR Filter
7. Binary Fuse Filter
8. Morton Filter
9. Quotient Filter




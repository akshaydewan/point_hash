#Comparison of Hashers

A comparison of hashers when applied to a `Point` struct

Output from my machine:
```
Default HashSet
Insert completed in 1.0386837s
Retrieve completed in 0.42566398s
----------
FXHash
Insert completed in 0.9031651s
Retrieve completed in 0.40001178s
----------
Fnv 64
Insert completed in 0.8247534s
Retrieve completed in 0.36593392s
----------
Fnv 32
Insert completed in 1.1962667s
Retrieve completed in 0.37191176s
----------
Spooky
Insert completed in 7.174129s
Retrieve completed in 2.9850314s
----------
Lookup3
Insert completed in 2.009412s
Retrieve completed in 0.8284081s
----------
OAATHasher
Insert completed in 1.3962045s
Retrieve completed in 0.6042114s
----------
DJB2Hasher
Insert completed in 1.3288014s
Retrieve completed in 0.42249614s
----------
SDBMHasher
Insert completed in 1.5735025s
Retrieve completed in 0.60605407s
----------
Bricolage
Insert completed in 2.1879466s
Retrieve completed in 0.8767516s
```

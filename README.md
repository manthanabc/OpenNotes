# OpenNotes
This is a rust client for a protocol i am making for sharing of notes and documents for archiving purposes
this is designed to be very ressilent and and decentralised

## Installing
This can be easily installed by just running released binery file 
if you dont have go-ipfs client installed then run
### linux
```./OpenNotes setup```
### windows
```OpenNotes.exe setup```

Once the setup is done you can run without any args

Note: if you already have ifps installed no need to run the setup

## using
Once the binary is running you can open localhost:8000


## How does it work ?
We can have some signing servers which publish an state file to ipns. 
peers can request them to append their ipfs hash of file to the signing server's
list and other clients dont even need to request the server as they alerady have the ipns
hash added they will just resolv the ipns to the file and use the list 

the file itself contains the list of hashes of files other peers have uploaded.

Now: support for protocol 15 



LATER: 
-add support for quakeworld protocol,  protocol 28 
https://nyov.github.io/qstat-svn/old/qprotocol.html
send a getchallenge packet, wait for the response, send a connect, wait for a positive response, start sending game packets periodically.


quakeworld net protocol 
client: net_chan.c

server: sv_main.c & sv_nchan.c

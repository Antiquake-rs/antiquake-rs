
##Collision 


https://www.gamedev.net/forums/topic/119342-half-life-or-q1-bsp-and-collisions/


Each hull is for collision for a different size of 'entity' 

The way it works is that there's a point-sized BSP tree which is generated from the original brushes, a humanoid-sized BSP tree generated from the brushes expanded by a humanoid-sized box, and a Shambler-sized one used only for Shamblers.

https://webpages.charlotte.edu/krs/courses/3050/lectures/cd2.pdf

https://developer.valvesoftware.com/wiki/BSP
http://web.archive.org/web/20050428082221/http://www.planetquake.com/qxx/bsp/
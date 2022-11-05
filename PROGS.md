
### Rebuild progs dat system as JSON config files  -- called SLIME system 


use rhai scripts ?? 
https://docs.rs/rhai/latest/rhai/

https://quakewiki.org/wiki/Quake_QuakeC_source
https://www.gamers.org/dEngine/quake/spec/quake-spec34/qc-menu.htm

slime.json 

{
    imports: [ ""  ] //paths to other slime json files 
 



}






need the slime to call 
ub fn builtin_precache_model(&mut self)


like the progs dat was ! 
https://quakewiki.org/wiki/world.qc



## REFERENCE 
QUAKE C Source 
https://quakewiki.org/wiki/Quake_QuakeC_source
 


the map file is loaded and it loops through all the entities in the map file
and for each one, it executes the progs.dat program with the name of the entity
and that execution is precaching sounds and models !!
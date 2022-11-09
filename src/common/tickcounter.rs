

use std::time::Duration;

pub struct TickCounter {

    pub tick_period: Duration,
    pub tick_accumulator: Duration,

}

impl TickCounter {

    pub fn new(duration:Duration) -> TickCounter {
       
        TickCounter { tick_period: duration, tick_accumulator: Duration::zero()}
    }


    pub fn update( frame_time: Duration ){


        
    }


    pub fn register_callback( tick_method:  FnMut  ){

    }


}
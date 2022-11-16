

use chrono::Duration;

  

//type Callback = Fn(&Tickable);

pub struct TickCounter {

    tick_period: Duration,
    tick_accumulator: Duration, 

}

impl TickCounter {

    pub fn new(duration:Duration) -> TickCounter {
       
        TickCounter { 
            tick_period: duration, 
            tick_accumulator: Duration::zero(),
            
        }
    }


    pub fn update(&mut self, frame_time: Duration) -> (Duration, bool)
     
    { 
        self.tick_accumulator = self.tick_accumulator + frame_time;
        if self.tick_accumulator > self.tick_period {

            match self.tick_accumulator.checked_sub( &self.tick_period ){
                Some(difference) => {
                    self.tick_accumulator = difference;
                     
                 
                    return (self.tick_accumulator, true)
                },
                None =>  return (self.tick_accumulator, false)
            }
        }else{
            return (self.tick_accumulator, false)
        }
 
       
    }


    pub fn get_progress_pct(&self)-> f32 {
        return self.tick_accumulator.num_milliseconds() as f32 / self.tick_period.num_milliseconds() as f32;
    }

 


}
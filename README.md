# jblomlof-simulation
This will be a pendulum simulation (a bad one)  
I wanted to do a double Pendulum, realized it's way to complicated, and then saw TheCodingTrain already had an episode with all the correct maths that was needed. Instead I wanted to make a budget version. Be much more liberal and approximate how a pendulum on a pendulum would function. It's quite the cool approximation for being so simple. But that feelt like to little, so instead I made it easy to set explore how it will behave by simply changing amount of pendulums and their starting angle. Also focused a bit more on visuals that is compatible with resizing window and such, trying to avoid hard magic numbers.

## To run
To run the program simply clone this repo and run ```cargo run```.

### Features
Most of its is commented but I'll explain

const SLOWDOWN_RATE is a factor which we multiply the speed by each cycle. I.E = 1 does nothing, < 1 makes it stops evantually, > 1 increases movement.  
Simple to add or remove pendulums, simply change const AMOUNT_PENDULUM. Also able to set angles.    
It traces the tip. Change size and amount of trace points with constants.  
Change width of pendulum with const SCALE_SIZE. 

Have fun and play around with it. ITs weird. Feel free to change physics to make it more realistic, or more crazy.
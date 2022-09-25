
Conf directory for provider : ./conf
Custom directory for provider configurations not watched by Git (for custom UDP IP/Port) : create a directory "mkdir ./custom-conf" and copy ./conf/xplane11.yaml into.

Start with xplane 11 provider : cargo run ./conf/xplane11.yaml
Start with Sim Mock provider : cargo run ./conf/sim-mock.yaml
Run tests : cargo test

Threads : 
* Main Tread : (config/init)
* Adc Thread : Get sensors datas and forward to AP and GUI
* Autopilot (AP) Thread : Process ADC DATA, Handle GUI AP commands and send AP state to GUI (if changed).
* GUI Thread : Receive ADC Data and AP State and set to shared memory via Arc/Mutex.
* GuiApp Thread : Egui APP (frontend), Read shared memory state via Arc/Mutex, handle user's inputs and send AP configuration command to AP.


MPSC channels configuration : 

<pre>
ADC-----[AdcDataMessage]------->AP------------------------
 |                              ^                        |
 |                              |                        |
 |                         [APCmdMessage]         [APStateMessage]
 |                              |                        |
 |                            GuiApp (Egui)              |
 |                              |                        |
 |                   Arc::&lt;Mutex&lt;GuiState&gt;&gt;              |
 |                              ^                        |
 |                              |                        |
 |------[AdcDataMessage]------>Gui<----------------------|

</pre>

X-Plane 11 configuration

Network configuration (UDP)
UDP Output Data screen
IP Address : what you want.
Port : 49003 (or others port non used by X-Plane)

UDP Read/Write DataRef
We use only write DataRef here.
IP Address: A least the same as the UDP Output Data screen
Port : Same port as the UDP Output Data screen.

NOTE: The write port is 49000 and is the default write port in X-Plane 11.


Output Data screen required config for Autopilot to get data (enable UDP checkbox for each data below).

Index, Data, Data fields (8 data field for each data index, value = -999 => no data for the field).

<pre>
|---------------------------------------|-------------|--------------|--------------|-------------|--------------|--------------|--------------|-------------|
|  Data index                           | 0           | 1            | 2            | 3           | 4            | 5            | 6            | 7           |
|---------------------------------------|-------------|--------------|--------------|-------------|--------------|--------------|--------------|-------------|
| 0, Frame                              | f-act       | f-sim        |              | frame       | cpu (time)   | cpu (time)   | grnd (ratio) | flit (ratio)|
| 3, Speeds                             | Vind (kias) | Vind (keas)  | Vtrue (ktas) | Vtrue (ktgs)|              | Vind (mph)   | Vtrue (mphas)| Vtrue (mphg)|
| 4, Mach, VVI, g-load                  | Mach (ratio)|              | VVI (fpm)    |             | Gload (norml)| Gload (axial)| Gload (side) |             |
| 8, Joystick aileron/elevator/rudder   | elev (stick)| ailrn (stick)| ruddr (stick)|             |              |              |              |             |
| 17, Pitch, roll & headings            | pitch (deg) | roll (deg)   | hding (true) | hding (mag) |              |              |              |             | 
| 18, Angle of attack, sideslip, & paths| alpha (deg) | beta (deg)   | hpath (deg)  | vpath (deg) |              |              |              | slip (deg)  |
| 19, Magnetic compass                  | mag (comp)  | mavar (deg)  |              |             |              |              |              |             |
| 20, Latitude, longitude, & altitude   | lat (deg)   | lon (deg)    | alt (ftmsl)  | alt (ftagl) | on (runwy)   | alt (ind)    | lat (origin) | lon (origin)| 
| 25, Throttle (commanded)              | thro1 (part)| thro2 (part) |              |             |              |              |              |             |
| 26, Throttle (actual)                 | thro1 (part)| thro2 (part) |              |             |              |              |              |             |
| 41, N1                                | N1  1 (pcnt)| N1  2 (pcnt) |              |             |              |              |              |             |
| 42, N2                                | N2  1 (pcnt)| N2  2 (pcnt) |              |             |              |              |              |             |
| 132, Climb stats                      | h-spd (kt)  | v-spd (fpm)  |              | mult (VxVVI)|              |              |              |             |
|---------------------------------------|-------------|--------------|--------------|-------------|--------------|--------------|--------------|-------------|
</pre>
Write DataRef : 

sim/joystick/yoke_heading_ratio
sim/joystick/yoke_pitch_ratio
sim/joystick/yoke_roll_ratio

sim/cockpit2/engine/actuators/throttle_ratio
sim/cockpit2/engine/actuators/throttle_ratio_all


Cockpit mapping : 
PFD speed : #3 Vind (kias ?)
PDF Alt : #20  Alt (ind) or Alt (ftmsl) ?
PFD Heading : #19 Mag (comp)

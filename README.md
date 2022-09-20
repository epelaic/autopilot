
Conf directory for provider : ./conf
Custom directory for provider configurations not watched by Git (for custom UDP IP/Port) : create a directory "mkdir ./custom-conf" and copy ./conf/xplane11.yaml into.

Start with xplane 11 provider : cargo run ./conf/xplane11.yaml
Start with Sim Mock provider : cargo run ./conf/sim-mock.yaml
Run tests : cargo test

Threads : 
* Main Tread : (config/init)
* Adc Thread : Get sensors datas and forward to AP and GUI
* Autopilot (AP) Thread : Process ADC DATA, Handle GUI AP commands and send AP state to GUI (if changed).
* GUI Thread : Receive ADC Data and AP State end set to shared memory via Arc/Mutex.
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

Conf directory for provider : ./conf
Custom directory for provider configurations not watched by Git (for custom UDP IP/Port) : create a directory "mkdir ./custom-conf" and copy ./conf/xplane11.yaml into.

Start with xplane 11 provider : cargo run ./conf/xplane11.yaml
Start with Sim Mock provider : cargo run ./conf/sim-mock.yaml
Run tests : cargo test
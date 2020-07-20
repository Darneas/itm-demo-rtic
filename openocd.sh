openocd \
-c 'interface jlink; transport select swd; source [find target/nrf52.cfg]'
-c 'itm port 0 on'
-c 'tpui config internal /tmp/swo.log uart off 800000'

# waybar-network-traffic

## Build & Installation

```shell
cargo build --release
mkdir -p ~/.config/waybar/scripts
cp target/release/waybar-network-traffic ~/.config/waybar/scripts/waybar-network-traffic
```

## USAGE

`~/.config/waybar/config`

```json
   "custom/network_traffic": {
        "exec": "~/.config/waybar/scripts/waybar-network-traffic",
        "return-type": "json",
   },
```

## Contributions and further development

PR Welcome!

## TODO

[ ] custom formatting
[ ] different color when disconnected
[ ] tooltip - graph last N seconds nicely

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

Feel free to add percentage (to get nice color when bandwidth is saturated) or any other feature if it makes sense. Network interface filter, customized formatting, tooltip...

#!/bin/sh
options="play\npause\nnext\nprevious\n"

selection="$(echo -e $options | \
             dmenu -i -p "Do something")"

result=$(playerctl $selection)
$HOME/.local/share/dwm/dwm-bar-refresh

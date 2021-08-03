## Scripts introduction

- `dwm-bar`

Output status to the bar. Mulitple color is supported:

```text
^rx,y,w,h^ Draw a rectangle of width w and height h, with its top left corner at (x,y) relative the X drawing cursor.

^c#FF0000^ Set foreground color.

^b#55cdfc^ Set background color, only applies to text, simply use the ^r^ command to change the background while drawing.

^f<px>^ Forward the X drawing cursor by <px> pixel. Please bear in mind that you have to move the cursor enough to display your drawing (by the with of your drawing).

^d^ Reset colors to SchemeNorm.
```

- dwm-volume-ctl

Used amixer which in the alsa-utils packages to changed system sound volume.
You will need to modified the sound card and controls variable to fit your case.

- chwp

Randomly filled background with the image in `$HOME/Pictures/Wallpapers`


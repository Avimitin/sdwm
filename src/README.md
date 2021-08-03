## Build Instruction

The Makefile will automatically create a directory at `/home/YOURUSERNAME/.local/share/dwm`
and put the script there. So it's necessary to change the `HOMEDIR` variable in the Makefile.

```makefile
# change name here
HOMEDIR = /home/YOURUSERNAME
```

And the script's keyboard shortcut is also hardcoded. 
Changed the variable listed below to successfully execute the script.

```c
static const char *powercmd[] = { "/home/YOURUSERNAME/.local/share/dwm/power", NULL };
static const char *chwpcmd[] = { "/home/YOURUSERNAME/.local/share/dwm/chwp", NULL };
static const char *volupcmd[] = { "/home/YOURUSERNAME/.local/share/dwm/dwm-volume-ctl", "up", NULL };
static const char *voldowncmd[] = { "/home/YOURUSERNAME/.local/share/dwm/dwm-volume-ctl", "down", NULL };
```

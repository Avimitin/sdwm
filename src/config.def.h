/* See LICENSE file for copyright and license details. */

/* appearance */
static const unsigned int borderpx  = 0;        /* border pixel of windows */
static const Gap default_gap        = {.isgap = 1, .realgap = 10, .gappx = 10};
static const unsigned int snap      = 32;       /* snap pixel */
static const int showbar            = 1;        /* 0 means no bar */
enum showtab_modes { showtab_never, showtab_auto, showtab_nmodes, showtab_always };
static const int showtab            = showtab_auto;
static const int toptab             = True;
static const int topbar             = 0;        /* 0 means bottom bar */
static const unsigned int systraypinning = 0;   /* 0: sloppy systray follows selected monitor, >0: pin systray to monitor X */
static const unsigned int systrayspacing = 2;   /* systray spacing */
static const int systraypinningfailfirst = 1;   /* 1: if pinning fails, display systray on the first monitor, False: display systray on the last monitor*/
static const int showsystray             = 1;   /* 0 means no systray */
static const int horizpadbar        = 15;        /* horizontal padding for statusbar */
static const int vertpadbar         = 20;        /* vertical padding for statusbar */
static const int vertpadtab         = 37;       /* Vertical padding for tab */
static const int horizpadtabi       = 15;       /* Horizon padding in tab */
static const int horizpadtabo       = 15;       /* Horizon padding at the tab edge */
static const Bool viewontag         = True;     /* Switch view on tag switch */
static const char *fonts[]          = { 
	"Ubuntu Nerd Font:size=10",
	"Noto Sans CJK SC:size=10",
	"Noto Sans CJK JP:size=10",
};
static const char col_cyan[]        = "#70c0ba";
static const char bluegray[]        = "#2e3440";
static const char white[]           = "#eeeeee";
static const char black[]           = "#232831";
static const char bg_normal[]           = "#1C1F24";
static const char bg_focus[]           = "#282C34";
static const char fg_normal[]           = "#606672";
static const char selected_border_color[] = "#6A6A6B";

/* More about exadecimal color code for transparency can check:
 * https://gist.github.com/lopspower/03fb1cc0ac9f32ef38f4 */
static const unsigned int baralpha = 0xBF; /* 75% */
static const unsigned int tabalpha = 0xBF; /* 90% */
static const unsigned int borderalpha = 0xE6;

static const char *colors[][3]      = {
	/*               fg         bg         border   */
	[SchemeNorm] = { fg_normal,     bg_normal, black},
	[SchemeSel]  = { white,     bg_focus,  selected_border_color  },
	[TabSel]     = { white,     bg_focus,  selected_border_color  },
	[TabNorm]    = { fg_normal, bg_normal,  black },
};

static const unsigned int alphas[][3]      = {
	/*               fg      bg        border     */
	[SchemeNorm] = { OPAQUE, baralpha, borderalpha },
	[SchemeSel]  = { OPAQUE, baralpha, borderalpha },
	[TabSel]     = { OPAQUE, tabalpha, borderalpha },
	[TabNorm]    = { OPAQUE, tabalpha, borderalpha },
};

/* tagging */
/* static const char *tags[] = { "", "", "", */ 
/*                               "", "", "", */ 
/*                               "", "", "" }; */
static const char *tags[] = {"", "", "", "", "", "", "","", ""};

static const Rule rules[] = {
	/* xprop(1):
	 *	WM_CLASS(STRING) = instance, class
	 *	WM_NAME(STRING) = title
	 */
	/* class                 instance    title       tags mask     isfloating   monitor */
	{ "firefox",             NULL,       NULL,       0,       0,           -1 },
	{ "Waterfox",             NULL,       NULL,       0,       0,           -1 },
};

/* layout(s) */
static const float mfact     = 0.55; /* factor of master area size [0.05..0.95] */
static const int nmaster     = 1;    /* number of clients in master area */
static const int resizehints = 1;    /* 1 means respect size hints in tiled resizals */
static const int lockfullscreen = 1; /* 1 will force focus on the fullscreen window */

#include "horizgrid.c"
static const Layout layouts[] = {
/* first entry is default */
	/* symbol     arrange function */
	{ "\ufab1", NULL },    /* no layout function means floating behavior */
	{ "",      tile },
	{ "𧻓",     monocle },
	{ "ﱖ",      horizgrid },
};

static const char* monocle_windows_count_tags[] = {
  "", "", "", "", "", "", "", "", "", "",
};

/* key definitions */
#define MODKEY Mod4Mask
#define TAGKEYS(KEY,TAG) \
	{ MODKEY,                       KEY,      view,           {.ui = 1 << TAG} }, \
	{ MODKEY|ControlMask,           KEY,      toggleview,     {.ui = 1 << TAG} }, \
	{ MODKEY|ShiftMask,             KEY,      tag,            {.ui = 1 << TAG} }, \
	{ MODKEY|ControlMask|ShiftMask, KEY,      toggletag,      {.ui = 1 << TAG} },

/* helper for spawning shell commands in the pre dwm-5.0 fashion */
#define SHCMD(cmd) { .v = (const char*[]){ "/bin/sh", "-c", cmd, NULL } }

/* commands */
static char dmenumon[2] = "0"; /* component of dmenucmd, manipulated in spawn() */
static const char *dmenucmd[] = { "fish", "-c", "dmenu_run",  NULL };
static const char *scscmd[] = { "flameshot", "gui",  NULL };
static const char *termcmd[]  = { "st", "-c", "simp-term", NULL };
/* static const char *termcmd[]  = { "tabbed", "-d", "-r", "2", "st", "-w", "\"\"", NULL }; */

/* changed all sh1marin to your name */
static const char *powercmd[] = { "/home/sh1marin/.local/share/dwm/power", NULL };
static const char *chwpcmd[] = { "/home/sh1marin/.local/share/dwm/chwp", NULL };
static const char *volupcmd[] = { "/home/sh1marin/.local/share/dwm/dwm-volume-ctl", "up", NULL };
static const char *voldowncmd[] = { "/home/sh1marin/.local/share/dwm/dwm-volume-ctl", "down", NULL };
static const char *playerctlcmd[] = { "/home/sh1marin/.local/share/dwm/playerctl.sh", NULL };

static Key keys[] = {
	/* modifier                     key        function        argument */
	{ MODKEY,                       XK_o, spawn,          {.v = dmenucmd } },
	{ MODKEY|ShiftMask,             XK_Return, spawn,          {.v = termcmd } },
	{ MODKEY|ShiftMask,             XK_s,      spawn,          {.v = scscmd } },
	{ MODKEY|ShiftMask,             XK_b,      spawn,          {.v = chwpcmd } },
	{ MODKEY|ShiftMask,             XK_e,      spawn,          {.v = powercmd } },
	{ MODKEY|ShiftMask,             XK_u,      spawn,          {.v = volupcmd } },
	{ MODKEY|ShiftMask,             XK_d,      spawn,          {.v = voldowncmd } },
	{ MODKEY|ShiftMask,             XK_p,      spawn,          {.v = playerctlcmd } },
	{ MODKEY,                       XK_w,      hidewin,        {0} },
	{ MODKEY|ShiftMask,             XK_w,      restorewin,     {0} },
	{ MODKEY,                       XK_b,      togglebar,      {0} },
	{ MODKEY,                       XK_j,      focusstack,     {.i = +1 } },
	{ MODKEY,                       XK_k,      focusstack,     {.i = -1 } },
	{ MODKEY,                       XK_i,      incnmaster,     {.i = +1 } },
	{ MODKEY,                       XK_d,      incnmaster,     {.i = -1 } },
	{ MODKEY,                       XK_h,      setmfact,       {.f = -0.05} },
	{ MODKEY,                       XK_l,      setmfact,       {.f = +0.05} },
	{ MODKEY,                       XK_Return, zoom,           {0} },
	{ MODKEY,                       XK_Tab,    view,           {0} },
	{ MODKEY|ShiftMask,             XK_q,      killclient,     {0} },
	{ MODKEY,                       XK_t,      setlayout,      {.v = &layouts[0]} },
	{ MODKEY,                       XK_f,      setlayout,      {.v = &layouts[1]} },
	{ MODKEY,                       XK_m,      setlayout,      {.v = &layouts[2]} },
	{ MODKEY,                       XK_g,      setlayout,      {.v = &layouts[3]} },
	{ MODKEY|ShiftMask,             XK_f,      togglefullscr,  {0} },
	{ MODKEY,                       XK_space,  setlayout,      {0} },
	{ MODKEY|ShiftMask,             XK_space,  togglefloating, {0} },
	{ MODKEY,                       XK_0,      view,           {.ui = ~0 } },
	{ MODKEY|ShiftMask,             XK_0,      tag,            {.ui = ~0 } },
	{ MODKEY,                       XK_comma,  focusmon,       {.i = -1 } },
	{ MODKEY,                       XK_period, focusmon,       {.i = +1 } },
	{ MODKEY|ShiftMask,             XK_comma,  tagmon,         {.i = -1 } },
	{ MODKEY|ShiftMask,             XK_period, tagmon,         {.i = +1 } },
	{ MODKEY,                       XK_minus,  setgaps,        {.i = -5 } },
	{ MODKEY,                       XK_equal,  setgaps,        {.i = +5 } },
	{ MODKEY|ShiftMask,             XK_minus,  setgaps,        {.i = GAP_RESET } },
	{ MODKEY|ShiftMask,             XK_equal,  setgaps,        {.i = GAP_TOGGLE} },
	{ MODKEY|ShiftMask,             XK_t,      tabmode,        { -1 } },
	TAGKEYS(                        XK_1,                      0)
	TAGKEYS(                        XK_2,                      1)
	TAGKEYS(                        XK_3,                      2)
	TAGKEYS(                        XK_4,                      3)
	TAGKEYS(                        XK_5,                      4)
	TAGKEYS(                        XK_6,                      5)
	TAGKEYS(                        XK_7,                      6)
	TAGKEYS(                        XK_8,                      7)
	TAGKEYS(                        XK_9,                      8)
	{ MODKEY|ShiftMask,             XK_c,      quit,           {0} },
};

/* button definitions */
/* click can be ClkTagBar, ClkLtSymbol, ClkStatusText, ClkWinTitle, ClkClientWin, or ClkRootWin */
static Button buttons[] = {
	/* click                event mask      button          function        argument */
	{ ClkLtSymbol,          0,              Button1,        setlayout,      {0} },
	{ ClkLtSymbol,          0,              Button3,        setlayout,      {.v = &layouts[2]} },
	{ ClkStatusText,        0,              Button2,        spawn,          {.v = termcmd } },
	{ ClkClientWin,         MODKEY,         Button1,        movemouse,      {0} },
	{ ClkClientWin,         MODKEY,         Button2,        togglefloating, {0} },
	{ ClkClientWin,         MODKEY|ShiftMask, Button1,        resizemouse,    {0} },
	{ ClkTagBar,            0,              Button1,        view,           {0} },
	{ ClkTagBar,            0,              Button3,        toggleview,     {0} },
	{ ClkTagBar,            MODKEY,         Button1,        tag,            {0} },
	{ ClkTagBar,            MODKEY,         Button3,        toggletag,      {0} },
	{ ClkTabBar,            0,              Button1,        focuswin,       {0} },
};


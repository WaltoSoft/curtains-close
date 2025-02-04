curtains-close(1)

# NAME

curtains-close - A Wayland logout menu written for the Curtains desktop environment but should work in other Wayland environments.

# SYNOPSIS

*curtains-close* [options]

# OPTIONS

*-h, --help*
	Show help message and stop

*-c, --css-path* <path>
	Specify a path to a file that will output the CSS used for styling the curtains-close window
	Ignored if -C, --css-content, -l, or --css-loader-path options are set

*-C, --css-content* <content>
	Specify, via a command line string, the CSS used for styling the curtains-close window
	Ignored if -l, or --css-loader-path options are set

*-l, --css-loader-path* <path>
	Specify a path to a shell script that will output the CSS used for styling the curtains-close window
	If this option is present, then the -c, --css-path, -C, AND --css-content options are ignored

*-s, --settings-path* <path>
	Specify a path to a json file containing configuration settings.
	Ignored if -S, --settings-content, -L, or --settings-loader-path options are set 

*-S, --settings-content* <content>
	Specify, via a command line string, the Json used for configuration settings
	Ignored if -L, or --settings-loader-path options are set

*-L, --settings-loader-path* <path>
	Sepcify a path to a shell script that will output the json used to configure curtains-close
	If this option is present, then the -s, --settings-path, -S, AND --settings-content options are ignored

*-n, --buttons-per-row* <number>
	Set the number of buttons per row
	Setting this option will override the "buttons_per_row" setting loaded from the configuration json

*-x, --column-spacing* <number>
	Set the space between button columns
	Setting this option will override the "column_spacing" setting loaded from the configuration json

*-y, --row-spacing* <number>
	Set the space between button rows
	Setting this option will override the "row_spacing" setting loaded from the configuration json

*-d, --delay-before-closing* <number>
	Set the delay after clicking a button before closing curtains-close
	Setting this option will override the "delay_before_closing" setting loaded from the configuration json

# DESCRIPTION

curtains-close is a GTK4 Wayland native UI for locking, logging out, shutting down, rebooting, etc, the curtains desktop environment. 
Written in Rust, curtains-clsoe is inspired by both wlogout and wleave and has the following features.

.  Uses text based icons using an Icon Font instead of an image file.
.  Allows setting configuration and button layout via json configuration file.
.  Allows for css content or settings json to be loaded from a shell script.
.  Loads overlay window with buttons on active monitor, and a blank overlay window on all other monitors.

# CONFIGURATION

Configuration settings for curtains-close will be loaded from one of the following locations and searched
in order (first match wins).  However, any one of the settings can also be specified via a command line argument 
which will override the corresponding value from the matched source.

1. JSON content passed directly to curtains-close as a string using the -S, or --settings-content CLI parameters.
2. Executing a shell script file specified in the -L, or --settings-content-loader CLI parameters.  It is expected
   that this script return the proper JSON format.
3. Reading a file path specified in the -s or --settings path CLI parameters
4. Executing '$XDG_CONFIG_HOME/curtains/close/settings.sh' if it exists
5. Reading '$XDG_CONFIG_HOME/curtains/close/settings.json' if it exists
6. Reading '/usr/local/etc/curtains-close/settings.json' if it exists 
7. Reading '/etc/curtains-close/settings.json'

The CSS used for styling the curtains-close GUI will be loaded from one of the following locations and searched
in order (first match wins)

1. CSS content passed directly to curtain-close as a string using the -C, or --css-content CLI parameters.
2. Executing a shell script file specified in the -l, or --css-content-loader CLI parameters.  It is expected
   that this script return proper CSS.
3. Reading a file path specified in the -c or --css-path CLI parameters
4. Executing '$XDG_CONFIG_HOME/curtains/close/style.sh' if it exists
5. Reading '$XDG_CONFIG_HOME/curtains/close/style.css' if it exists
6. Reading '/usr/local/etc/curtains-close/style.css' if it exists
7. Reading '/etc/curtains-close/style.css'

An error is raised when no configuration settings json is found; However, the css is optional. 

# AUTHORS

Inspired By wlogout by Haden Collins <collinshaden@gmail.com>. For more information about wlogout, see <https://github.com/ArtsyMacaw/wlogout>.
Inspired By wleave by Natty <natty.sh.git@gmail.com>.  For more information about wleave, see <https://github.com/AMNatty/wleave>.

curtains-close is written by
Billy Walton, see <https://github.com/waltosoft/curtains-close>.

# SEE ALSO

*curtains-close*(5)
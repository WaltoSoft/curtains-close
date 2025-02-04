curtains-close(5)

# NAME

curtains-close - A Wayland logout menu written for the Curtains desktop environment but should work in other Wayland environments.

# CONFIGURATION

Configuration settings json has the following example format:
{
  "buttons_per_row": <number>,
  "column_spacing": <pixels>,
  "row_spacing": <pixels>,
  "delay_before_closing": <millseconds>,
  "buttons": []
}

. buttons_per_row:      The number of buttons that will be displayed on 1 row of buttons before starting a new row
. colump_spacing:       Number of pixels between columns of buttons
. row_spacing:          Number of pixels between rows of buttons
. delay_before_closing: The delay in milliseconds to wait after executing the command on the button before terminating curtains-close
. buttons:              An array of button objects that will be displayed (see below)

The buttons setting is an array of objects with the following example format:
{
  "id": <string>,
  "command": <string>,
  "icon": <string>,
  "keybind": <char>,
  "text": <string>
}

. id:      Selector that would be used to identify the button in CSS
. command: Shell command to be executed when the button is clicked
. icon:    Meant to specify a unicode character representing an icon in an Icon Font (i.e. Material Design Icons or Font Awesome)
. text:    Text displayed on the Button
. keybind: The keyboard character mapped to the button that will execute it's command (note the escape key is reserved for exiting the application)

# AUTHOR

Inspired By wlogout by Haden Collins <collinshaden@gmail.com>. For more information about wlogout, see <https://github.com/ArtsyMacaw/wlogout>.
Inspired By wleave by Natty <natty.sh.git@gmail.com>.  For more information about wleave, see <https://github.com/AMNatty/wleave>.

curtains-close is written by
Billy Walton, see <https://github.com/waltosoft/curtains-close>.

# SEE ALSO

*curtains-close*(1)
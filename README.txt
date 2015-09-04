|Chronia|
+=======+

Your little hated friend who kills processes that distracts you.
Use install.sh to install.

Chronia looks at the open processes every N milliseconds and kills
those which distracts you (custom defined, see `config`), if a
certain condition is met.

WARNING: Configure it before you install. Especially if you use
         termite as you terminal emulator, as this is default.

Note that it uses systemd. If your system doesn't support systemd,
just compile the repos with cargo and then daemonize the process,
using you favorite tool.

Config
------
The config is hard-coded. See main.rs. To load the new config run
install.sh.

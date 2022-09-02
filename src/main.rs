extern crate console;
extern crate sysinfo;
use std::fs::{File, create_dir_all, metadata};
use std::io::{BufRead, BufReader, Write};
#[allow(deprecated)]
use std::env::{home_dir, consts, args};

use console::style;
use sysinfo::{System, SystemExt, RefreshKind};

#[allow(deprecated)]
fn main() {
    let mut distro = "";
    let argv: Vec<String> = args().collect();
    if argv.len() > 1 {
        if argv[1] == "linux" || argv[1] == "freebsd" || argv[1] == "solaris" || argv[1] == "dragonfly" || argv[1] == "windows" || argv[1] == "macos" || argv[1] == "ios" || argv[1] == "openbsd" || argv[1] == "netbsd" ||argv[1] == "android" {
            distro = &argv[1];
        } else {
            println!("unknown distro: {}", &argv[1]);
        }
    }
    else {
        distro = consts::OS;
    }
    let mut sys = System::new_with_specifics(RefreshKind::everything().without_components_list().without_networks_list().without_disks_list());
    sys.refresh_memory();
    sys.refresh_cpu();
    if let Some(home) = home_dir() {
        let configdir = format!("{}/.config/rfetch", home.display());
        if !metadata(&configdir).is_ok() {
            create_dir_all(&configdir).unwrap();
        }
        if !metadata(format!("{}/rfetch.config", configdir)).is_ok() {
            let defaultconfig = "[linux]
color=189
   __
-=(o '.
   '.-.\\
   /|  \\\\
   '|  ||
    _\\_):,_
[windows]
color=111
11111  11111
11111  11111
11111  11111
         
11111  11111
11111  11111
11111  11111
[freebsd]
color=9
/\\,-'''''-,/\\
\\_)       (_/
|           |
|           |
 ;         ;
  '-_____-'
[macos]
color=245
       .:'
    _ :'_
 .'`_`-'_``.
:________.-'
:_______:
:_______:
 :_______`-;
  `._.-._.'
[ios]
color=245
       .:'
    _ :'_
 .'`_`-'_``.
:________.-'
:_______:
:_______:
 :_______`-;
  `._.-._.'
[dragonfly]
color=141
   ,_,
('-_|_-')
 >--|--<
(_-'|'-_)
    |
    |
    |
[netbsd]
color=130
\\\\\\`-______,----__
 \\\\        __,---\\`_
  \\\\       \\`.____
   \\\\-______,----\\`-
    \\\\
     \\\\
      \\\\
[openbsd]
color=3
      _____
    \\-     -/
 \\_/         \\
 |        O O |
 |_  <   )  3 )
 / \\         /
    /-_____-\\
[solaris]
color=3
       .   .;   .
   .   :;  ::  ;:   .
   .;. ..      .. .;.
..  ..             ..  ..
 .;,                 ,;.
[android]
color=76
  ;,           ,;
   ';,.-----.,;'
  ,'           ',
 /    O     O    \\
|                 |
'-----------------'";
            let mut configfile = File::create(format!("{}/rfetch.config", configdir)).unwrap();
            write!(configfile, "{}", defaultconfig).unwrap();
        }
        let lines = BufReader::new(File::open(format!("{}/rfetch.config", configdir)).unwrap()).lines();
        let mut print = false;
        let mut asciiart = String::new();
        let mut longest_length = 0;
        for line in lines {
            if let Ok(linecontents) = line {
                if print == true {
                    if linecontents.starts_with("[") && linecontents.ends_with("]") {
                        break;
                    }
                    asciiart.push_str(linecontents.trim_end());
                    asciiart.push('\n');
                    let length = linecontents.trim_end().len();
                    if length > longest_length {
                        longest_length = length;
                    }
                }
                else if linecontents == format!("[{}]", distro) {
                    print = true
                }
            }
        }
        #[allow(unused_assignments)]
        let mut user = String::new();
        let pcname = match sys.host_name() {
            Some(n) => n,
            None => "null".to_string()
        };
        let corecount = match sys.physical_core_count() {
            Some(n) => n,
            None => 0
        };
        if consts::OS == "windows" {
            user = std::env::var("USERNAME").unwrap();
        } else {
            user = std::env::var("USER").unwrap();
        }
        let mut info: Vec<String> = Vec::new();

        info.push(format!("{}: {}{}{}", style("USER").color256(208), user, style("@").cyan(), pcname));
        info.push(format!("{}: {}", style("OS").color256(135), consts::OS));
        info.push(format!("{}: {}M / {}M", style("MEM").color256(119), sys.used_memory()/1024, sys.total_memory()/1024));
        info.push(format!("{}: {}minutes", style("UP").color256(111), sys.uptime()/60));
        info.push(format!("{}: {}", style("CORES").color256(210), corecount));
        #[allow(unused_assignments)]
        let mut color = 0;
        for line in asciiart.lines() {
            if line.starts_with("color=") {
                color = line.replace("color=", "").trim().parse().unwrap();
                break;
            }
        }
        let mut counter= 0;
        for line in asciiart.lines() {
            if line.starts_with("color="){
                continue;
            }
            let mut ascii_line = line.to_string();
            for _ in line.len()..longest_length+3 {
                ascii_line.push(' ');
            }
            if counter < 5 {
                println!("{}{}", style(ascii_line).color256(color), info[counter]);
            }
            else {
                println!("{}", style(ascii_line).color256(color));
            }
            counter += 1;
        }
    }
}
use std::env::consts::OS;

use sysinfo::System;

pub fn get_image() -> String {
    return match OS {
        "windows" => windows(),
        "macos" => macos(),
        "linux" => match System::distribution_id().as_str() {
            "ubuntu" => ubuntu(),
            "fedora" => fedora(),
            "debian" => debian(),
            "proxmox" => proxmox(),
            "arch" => arch(),
            _ => linux_default(),
        },
        _ => rust_fetch(),
    };
}

fn windows() -> String {
    "\x1b[94mLLLLLLLLLLLLLLLL   LLLLLLLLLLLLLLLL\x1b[0m
\x1b[94mLLLLLLLLLLLLLLLL   LLLLLLLLLLLLLLLL\x1b[0m
\x1b[94mLLLLLLLLLLLLLLLL   LLLLLLLLLLLLLLLL\x1b[0m
\x1b[94mLLLLLLLLLLLLLLLL   LLLLLLLLLLLLLLLL\x1b[0m
\x1b[94mLLLLLLLLLLLLLLLL   LLLLLLLLLLLLLLLL\x1b[0m
\x1b[94mLLLLLLLLLLLLLLLL   LLLLLLLLLLLLLLLL\x1b[0m
\x1b[94mLLLLLLLLLLLLLLLL   LLLLLLLLLLLLLLLL\x1b[0m
\x1b[94m                                   \x1b[0m
\x1b[94mLLLLLLLLLLLLLLLL   LLLLLLLLLLLLLLLL\x1b[0m
\x1b[94mLLLLLLLLLLLLLLLL   LLLLLLLLLLLLLLLL\x1b[0m
\x1b[94mLLLLLLLLLLLLLLLL   LLLLLLLLLLLLLLLL\x1b[0m
\x1b[94mLLLLLLLLLLLLLLLL   LLLLLLLLLLLLLLLL\x1b[0m
\x1b[94mLLLLLLLLLLLLLLLL   LLLLLLLLLLLLLLLL\x1b[0m
\x1b[94mLLLLLLLLLLLLLLLL   LLLLLLLLLLLLLLLL\x1b[0m
\x1b[94mLLLLLLLLLLLLLLLL   LLLLLLLLLLLLLLLL\x1b[0m"
        .to_string()
}

fn rust_fetch() -> String {
    "RRRRRRRRRRRRRRRRRRRRRR
RRRRRRRRRRRRRRRRRRRRRRRRRR
RRRRRRRR        RRRRRRRRRRR
RRRRRRRR           RRRRRRRRR
RRRRRRRR            RRRRRRRR
RRRRRRRR       \x1b[31m(((((((((((((\x1b[0m
RRRRRRRRRRRRRRR\x1b[31m((((\x1b[0m
RRRRRRRRRRRRRRR\x1b[31m((((\x1b[0m
RRRRRRRR       \x1b[31m(((((((((((((\x1b[0m
RRRRRRRR       \x1b[31m((((\x1b[0mRRRR
RRRRRRRR       \x1b[31m((((\x1b[0mRRRRRR
RRRRRRRR       \x1b[31m((((\x1b[0mRRRRRRRR"
        .to_string()
}

fn ubuntu() -> String {
    "              \x1b[31mUUUUUUUUUUUUUUUUUUU\x1b[0m
          \x1b[31mUUUUUUUUUUUUUUUUUUUUUUUUUUU\x1b[0m
       \x1b[31mUUUUUUUUUUUUUUUUUUUUUUU    UUUUUU\x1b[0m
     \x1b[31mUUUUUUUUUUUUUUUUUUUUUUUU      UUUUUUU\x1b[0m
   \x1b[31mUUUUUUUUUUUUUU           U     UUUUUUUUUU\x1b[0m
  \x1b[31mUUUUUUUUUUU    UU               UUUUUUUUUUU\x1b[0m
 \x1b[31mUUUUUUUUUU      UUUUUUUUUUUUU      UUUUUUUUUU\x1b[0m
\x1b[31mUUUUUUUUUU     UUUUUUUUUUUUUUUUU     UUUUUUUUUU\x1b[0m
\x1b[31mUUUU     UU   UUUUUUUUUUUUUUUUUUU     UUUUUUUUU\x1b[0m
\x1b[31mUUU       U   UUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU\x1b[0m
\x1b[31mUUUU     U.   UUUUUUUUUUUUUUUUUUU     UUUUUUUUU\x1b[0m
\x1b[31mUUUUUUUUUU     UUUUUUUUUUUUUUUUU     UUUUUUUUUU\x1b[0m
 \x1b[31mUUUUUUUUUU      UUUUUUUUUUUUU      UUUUUUUUUU\x1b[0m
  \x1b[31mUUUUUUUUUUU    UU            U* UUUUUUUUUUU\x1b[0m
   \x1b[31mUUUUUUUUUUUUUU           U     pUUUUUUUUU\x1b[0m
     \x1b[31mUUUUUUUUUUUUUUUUUUUUUUUU      UUUUUUU\x1b[0m
       \x1b[31mUUUUUUUUUUUUUUUUUUUUUUU    UUUUUU\x1b[0m
          \x1b[31mUUUUUUUUUUUUUUUUUUUUUUUUUUU\x1b[0m
              \x1b[31mUUUUUUUUUUUUUUUUUUU\x1b[0m"
        .to_string()
}

fn debian() -> String {
    "                       ,##p,
              (DDDDDDDDDDDDDDDDDDDDD*
          #DDDDDDDDDDDDDDDDDDDDDDDDDDDDDp
       ,DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD
     #DDDDDDDDDDDDD,             DDDDDDDDDDDDp
    DDDDDDDDDDDD     *DDDDDDDDD(    (DDDDDDDDDD
  (DDDDDDDDDDD,  .DDDDDDDDDDDDDDDD    DDDDDDDDDD
 .DDDDDDDDDDD  DDDDDDDDDDD   DDDDDD   pDDDDDDDDDD
 DDDDDDDDDDD  DD#DDDDDD DDDDDDDD#DDD  DDDDDDDDDD#D
 DDDDDDDDDDD  DDDDDDDD DDDDDDDDDDDDD ,DDDDDDDDDDDD
#DDDDDDDDDDD  DDDDDDDD pDDDDDDDDDDD  DDDDDDDDDDDDD
 DDDDDDDDDDD  DDDDDDDDDD DDDDDDD# .DDDDDDDDDDDDDDD
 DDDDDDDDDDDD  DDDDDDDDDDD DDDDD#DDDDDDDDDDDDDDDDD
  DDDDDDDDDDDD  DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD
  ,DDDDDDDDDDDD  (DDDDDDDDDDDDDDDDDDDDDDDDDDDDDD
    DDDDDDDDDDDDD  pDDDDDDDDDDDDDDDDDDDDDDDDDDD
     pDD#DDDDDDD#DDD*  D#DDDDDDD#DDDDDDD#DDDD
        DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD
          *DDDDDDDDDDDDDDDDDDDDDDDDDDDDD
              .DDDDDDDDDDDDDDDDDDDDD
"
    .to_string()
}

fn proxmox() -> String {
    "         PPPPPPPPPP             PPPPPPPPPP
          PPPPPPPPPP         PPPPPPPPPP
,pppppppp   PPPPPPPPPP    .PPPPPPPPPP   pppppppp
.ppppppppppp  PPPPPPPPPPPPPPPPPPPPP  ppppppppppp
   ppppppppppp  .PPPPPPPPPPPPPPP   ppppppppppp
     ppppppppppp   PPPPPPPPPPP  *pppppppppp*
       *ppppppppppp  PPPPPPP  ppppppppppp
          ppppppppppp  PPp  ppppppppppp
            ppppppppppp   ppppppppppp
          ppppppppppp  PP%  ppppppppppp
       ppppppppppp*  PPPPPPP  pppppppppppp
     ppppppppppp  *PPPPPPPPPPP   ppppppppppp
   ppppppppppp  PPPPPPPPPPPPPPPP,  ppppppppppp
,ppppppppppp  PPPPPPPPPPPPPPPPPPPPP  ppppppppppp
 pppppppp   PPPPPPPPPP     PPPPPPPPPP   pppppppp
          PPPPPPPPPP         PPPPPPPPPP
        PPPPPPPPP,             PPPPPPPPPP"
        .to_string()
}

fn fedora() -> String {
    "                    (########(
            #######################
         #############################
      ##################         ########
    #################   #######      #####/
   ################  *###             ######
  ################. *###  #####(       ######
 #################  ###  #######       ######(
 #################  ###  #####         #######
(#######        ############/        (########
(#####          ############(      ###########
(####        ,####  ###  ####################.
(###       #######  ###  ###################(
(####       ######  ###  (##################
(####             ####  ##################
(######     #######(   ################(
.#########         .#################
 *###############################*
     /##################("
        .to_string()
}

fn macos() -> String {
    "                                 @@@
                           @@@@@@@
                         @@@@@@@@
                        @@@@@@@@
                       @@@@@@
          @@@@@@@@     @     @@@@@@@@@@
     *@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
   @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
 @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
 @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%
 @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
 &@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
   @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
     @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
      @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
        @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
          @@@@@@@@@@@@    @@@@@@@@@@@@"
        .to_string()
}

fn linux_default() -> String {
    "                .88888888:.
               88888888.88888.
             .8888888888888888.
             888888888888888888
             88' _`88'_  `88888
             88 88 88 88  88888
             88_88_::_88_:88888
             88:::,::,:::::8888
             88`:::::::::'`8888
            .88  `::::'    8:88.
           8888            `8:888.
         .8888'             `888888.
        .8888:..  .::.  ...:'8888888:.
       .8888.'     :'     `'::`88:88888
      .8888        '         `.888:8888.
     888:8         .           888:88888
   .888:88        .:           888:88888:
   8888888.       ::           88:888888
   `.::.888.      ::          .88888888
  .::::::.888.    ::         :::`8888'.:.
 ::::::::::.888   '         .::::::::::::
 ::::::::::::.8    '      .:8::::::::::::.
.::::::::::::::.        .:888:::::::::::::
:::::::::::::::88:.__..:88888:::::::::::'
 `'.:::::::::::88888888888.88:::::::::'
    `':::_:' -- '' -'-' `':_::::'`"
        .to_string()
}

fn arch() -> String {
    "                             (
                      (((
                     (((((
                    (((((((
                   (((((((((
                  (((((((((((
                  ((((((((((((
                (((((((((((((((
               ((((((((((((((((((
             (((((((((((((((((((((
            (((((((((((((((((((((((
           (((((((((((((((((((((((((
         ((((((((((((     ((((((((((((
        (((((((((((         (((((((((((
       (((((((((((           (((((((((((
     (((((((((((((           (((((((((((
    ((((((((((((((           (((((((((((((
   (((((((((((                   (((((((((((
 (((((((                               (((((((
(((                                         ((( "
        .to_string()
}

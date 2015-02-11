
#![allow(non_upper_case_globals)]

use libc;

pub type SDL_Keycode = libc::int32_t;
pub const SDLK_UNKNOWN            :SDL_Keycode = 0;
pub const SDLK_BACKSPACE          :SDL_Keycode = 8;
pub const SDLK_TAB                :SDL_Keycode = 9;
pub const SDLK_RETURN             :SDL_Keycode = 13;
pub const SDLK_ESCAPE             :SDL_Keycode = 27;
pub const SDLK_SPACE              :SDL_Keycode = 32;
pub const SDLK_EXCLAIM            :SDL_Keycode = 33;
pub const SDLK_QUOTEDBL           :SDL_Keycode = 34;
pub const SDLK_HASH               :SDL_Keycode = 35;
pub const SDLK_DOLLAR             :SDL_Keycode = 36;
pub const SDLK_PERCENT            :SDL_Keycode = 37;
pub const SDLK_AMPERSAND          :SDL_Keycode = 38;
pub const SDLK_QUOTE              :SDL_Keycode = 39;
pub const SDLK_LEFTPAREN          :SDL_Keycode = 40;
pub const SDLK_RIGHTPAREN         :SDL_Keycode = 41;
pub const SDLK_ASTERISK           :SDL_Keycode = 42;
pub const SDLK_PLUS               :SDL_Keycode = 43;
pub const SDLK_COMMA              :SDL_Keycode = 44;
pub const SDLK_MINUS              :SDL_Keycode = 45;
pub const SDLK_PERIOD             :SDL_Keycode = 46;
pub const SDLK_SLASH              :SDL_Keycode = 47;
pub const SDLK_0                  :SDL_Keycode = 48;
pub const SDLK_1                  :SDL_Keycode = 49;
pub const SDLK_2                  :SDL_Keycode = 50;
pub const SDLK_3                  :SDL_Keycode = 51;
pub const SDLK_4                  :SDL_Keycode = 52;
pub const SDLK_5                  :SDL_Keycode = 53;
pub const SDLK_6                  :SDL_Keycode = 54;
pub const SDLK_7                  :SDL_Keycode = 55;
pub const SDLK_8                  :SDL_Keycode = 56;
pub const SDLK_9                  :SDL_Keycode = 57;
pub const SDLK_COLON              :SDL_Keycode = 58;
pub const SDLK_SEMICOLON          :SDL_Keycode = 59;
pub const SDLK_LESS               :SDL_Keycode = 60;
pub const SDLK_EQUALS             :SDL_Keycode = 61;
pub const SDLK_GREATER            :SDL_Keycode = 62;
pub const SDLK_QUESTION           :SDL_Keycode = 63;
pub const SDLK_AT                 :SDL_Keycode = 64;
pub const SDLK_LEFTBRACKET        :SDL_Keycode = 91;
pub const SDLK_BACKSLASH          :SDL_Keycode = 92;
pub const SDLK_RIGHTBRACKET       :SDL_Keycode = 93;
pub const SDLK_CARET              :SDL_Keycode = 94;
pub const SDLK_UNDERSCORE         :SDL_Keycode = 95;
pub const SDLK_BACKQUOTE          :SDL_Keycode = 96;
pub const SDLK_a                  :SDL_Keycode = 97;
pub const SDLK_b                  :SDL_Keycode = 98;
pub const SDLK_c                  :SDL_Keycode = 99;
pub const SDLK_d                  :SDL_Keycode = 100;
pub const SDLK_e                  :SDL_Keycode = 101;
pub const SDLK_f                  :SDL_Keycode = 102;
pub const SDLK_g                  :SDL_Keycode = 103;
pub const SDLK_h                  :SDL_Keycode = 104;
pub const SDLK_i                  :SDL_Keycode = 105;
pub const SDLK_j                  :SDL_Keycode = 106;
pub const SDLK_k                  :SDL_Keycode = 107;
pub const SDLK_l                  :SDL_Keycode = 108;
pub const SDLK_m                  :SDL_Keycode = 109;
pub const SDLK_n                  :SDL_Keycode = 110;
pub const SDLK_o                  :SDL_Keycode = 111;
pub const SDLK_p                  :SDL_Keycode = 112;
pub const SDLK_q                  :SDL_Keycode = 113;
pub const SDLK_r                  :SDL_Keycode = 114;
pub const SDLK_s                  :SDL_Keycode = 115;
pub const SDLK_t                  :SDL_Keycode = 116;
pub const SDLK_u                  :SDL_Keycode = 117;
pub const SDLK_v                  :SDL_Keycode = 118;
pub const SDLK_w                  :SDL_Keycode = 119;
pub const SDLK_x                  :SDL_Keycode = 120;
pub const SDLK_y                  :SDL_Keycode = 121;
pub const SDLK_z                  :SDL_Keycode = 122;
pub const SDLK_DELETE             :SDL_Keycode = 127;
pub const SDLK_CAPSLOCK           :SDL_Keycode = 1073741881;
pub const SDLK_F1                 :SDL_Keycode = 1073741882;
pub const SDLK_F2                 :SDL_Keycode = 1073741883;
pub const SDLK_F3                 :SDL_Keycode = 1073741884;
pub const SDLK_F4                 :SDL_Keycode = 1073741885;
pub const SDLK_F5                 :SDL_Keycode = 1073741886;
pub const SDLK_F6                 :SDL_Keycode = 1073741887;
pub const SDLK_F7                 :SDL_Keycode = 1073741888;
pub const SDLK_F8                 :SDL_Keycode = 1073741889;
pub const SDLK_F9                 :SDL_Keycode = 1073741890;
pub const SDLK_F10                :SDL_Keycode = 1073741891;
pub const SDLK_F11                :SDL_Keycode = 1073741892;
pub const SDLK_F12                :SDL_Keycode = 1073741893;
pub const SDLK_PRINTSCREEN        :SDL_Keycode = 1073741894;
pub const SDLK_SCROLLLOCK         :SDL_Keycode = 1073741895;
pub const SDLK_PAUSE              :SDL_Keycode = 1073741896;
pub const SDLK_INSERT             :SDL_Keycode = 1073741897;
pub const SDLK_HOME               :SDL_Keycode = 1073741898;
pub const SDLK_PAGEUP             :SDL_Keycode = 1073741899;
pub const SDLK_END                :SDL_Keycode = 1073741901;
pub const SDLK_PAGEDOWN           :SDL_Keycode = 1073741902;
pub const SDLK_RIGHT              :SDL_Keycode = 1073741903;
pub const SDLK_LEFT               :SDL_Keycode = 1073741904;
pub const SDLK_DOWN               :SDL_Keycode = 1073741905;
pub const SDLK_UP                 :SDL_Keycode = 1073741906;
pub const SDLK_NUMLOCKCLEAR       :SDL_Keycode = 1073741907;
pub const SDLK_KPDIVIDE           :SDL_Keycode = 1073741908;
pub const SDLK_KPMULTIPLY         :SDL_Keycode = 1073741909;
pub const SDLK_KPMINUS            :SDL_Keycode = 1073741910;
pub const SDLK_KPPLUS             :SDL_Keycode = 1073741911;
pub const SDLK_KPENTER            :SDL_Keycode = 1073741912;
pub const SDLK_KP1                :SDL_Keycode = 1073741913;
pub const SDLK_KP2                :SDL_Keycode = 1073741914;
pub const SDLK_KP3                :SDL_Keycode = 1073741915;
pub const SDLK_KP4                :SDL_Keycode = 1073741916;
pub const SDLK_KP5                :SDL_Keycode = 1073741917;
pub const SDLK_KP6                :SDL_Keycode = 1073741918;
pub const SDLK_KP7                :SDL_Keycode = 1073741919;
pub const SDLK_KP8                :SDL_Keycode = 1073741920;
pub const SDLK_KP9                :SDL_Keycode = 1073741921;
pub const SDLK_KP0                :SDL_Keycode = 1073741922;
pub const SDLK_KPPERIOD           :SDL_Keycode = 1073741923;
pub const SDLK_APPLICATION        :SDL_Keycode = 1073741925;
pub const SDLK_POWER              :SDL_Keycode = 1073741926;
pub const SDLK_KPEQUALS           :SDL_Keycode = 1073741927;
pub const SDLK_F13                :SDL_Keycode = 1073741928;
pub const SDLK_F14                :SDL_Keycode = 1073741929;
pub const SDLK_F15                :SDL_Keycode = 1073741930;
pub const SDLK_F16                :SDL_Keycode = 1073741931;
pub const SDLK_F17                :SDL_Keycode = 1073741932;
pub const SDLK_F18                :SDL_Keycode = 1073741933;
pub const SDLK_F19                :SDL_Keycode = 1073741934;
pub const SDLK_F20                :SDL_Keycode = 1073741935;
pub const SDLK_F21                :SDL_Keycode = 1073741936;
pub const SDLK_F22                :SDL_Keycode = 1073741937;
pub const SDLK_F23                :SDL_Keycode = 1073741938;
pub const SDLK_F24                :SDL_Keycode = 1073741939;
pub const SDLK_EXECUTE            :SDL_Keycode = 1073741940;
pub const SDLK_HELP               :SDL_Keycode = 1073741941;
pub const SDLK_MENU               :SDL_Keycode = 1073741942;
pub const SDLK_SELECT             :SDL_Keycode = 1073741943;
pub const SDLK_STOP               :SDL_Keycode = 1073741944;
pub const SDLK_AGAIN              :SDL_Keycode = 1073741945;
pub const SDLK_UNDO               :SDL_Keycode = 1073741946;
pub const SDLK_CUT                :SDL_Keycode = 1073741947;
pub const SDLK_COPY               :SDL_Keycode = 1073741948;
pub const SDLK_PASTE              :SDL_Keycode = 1073741949;
pub const SDLK_FIND               :SDL_Keycode = 1073741950;
pub const SDLK_MUTE               :SDL_Keycode = 1073741951;
pub const SDLK_VOLUMEUP           :SDL_Keycode = 1073741952;
pub const SDLK_VOLUMEDOWN         :SDL_Keycode = 1073741953;
pub const SDLK_KPCOMMA            :SDL_Keycode = 1073741957;
pub const SDLK_KPEQUALSAS400      :SDL_Keycode = 1073741958;
pub const SDLK_ALTERASE           :SDL_Keycode = 1073741977;
pub const SDLK_SYSREQ             :SDL_Keycode = 1073741978;
pub const SDLK_CANCEL             :SDL_Keycode = 1073741979;
pub const SDLK_CLEAR              :SDL_Keycode = 1073741980;
pub const SDLK_PRIOR              :SDL_Keycode = 1073741981;
pub const SDLK_RETURN2            :SDL_Keycode = 1073741982;
pub const SDLK_SEPARATOR          :SDL_Keycode = 1073741983;
pub const SDLK_OUT                :SDL_Keycode = 1073741984;
pub const SDLK_OPER               :SDL_Keycode = 1073741985;
pub const SDLK_CLEARAGAIN         :SDL_Keycode = 1073741986;
pub const SDLK_CRSEL              :SDL_Keycode = 1073741987;
pub const SDLK_EXSEL              :SDL_Keycode = 1073741988;
pub const SDLK_KP00               :SDL_Keycode = 1073742000;
pub const SDLK_KP000              :SDL_Keycode = 1073742001;
pub const SDLK_THOUSANDSSEPARATOR :SDL_Keycode = 1073742002;
pub const SDLK_DECIMALSEPARATOR   :SDL_Keycode = 1073742003;
pub const SDLK_CURRENCYUNIT       :SDL_Keycode = 1073742004;
pub const SDLK_CURRENCYSUBUNIT    :SDL_Keycode = 1073742005;
pub const SDLK_KPLEFTPAREN        :SDL_Keycode = 1073742006;
pub const SDLK_KPRIGHTPAREN       :SDL_Keycode = 1073742007;
pub const SDLK_KPLEFTBRACE        :SDL_Keycode = 1073742008;
pub const SDLK_KPRIGHTBRACE       :SDL_Keycode = 1073742009;
pub const SDLK_KPTAB              :SDL_Keycode = 1073742010;
pub const SDLK_KPBACKSPACE        :SDL_Keycode = 1073742011;
pub const SDLK_KPA                :SDL_Keycode = 1073742012;
pub const SDLK_KPB                :SDL_Keycode = 1073742013;
pub const SDLK_KPC                :SDL_Keycode = 1073742014;
pub const SDLK_KPD                :SDL_Keycode = 1073742015;
pub const SDLK_KPE                :SDL_Keycode = 1073742016;
pub const SDLK_KPF                :SDL_Keycode = 1073742017;
pub const SDLK_KPXOR              :SDL_Keycode = 1073742018;
pub const SDLK_KPPOWER            :SDL_Keycode = 1073742019;
pub const SDLK_KPPERCENT          :SDL_Keycode = 1073742020;
pub const SDLK_KPLESS             :SDL_Keycode = 1073742021;
pub const SDLK_KPGREATER          :SDL_Keycode = 1073742022;
pub const SDLK_KPAMPERSAND        :SDL_Keycode = 1073742023;
pub const SDLK_KPDBLAMPERSAND     :SDL_Keycode = 1073742024;
pub const SDLK_KPVERTICALBAR      :SDL_Keycode = 1073742025;
pub const SDLK_KPDBLVERTICALBAR   :SDL_Keycode = 1073742026;
pub const SDLK_KPCOLON            :SDL_Keycode = 1073742027;
pub const SDLK_KPHASH             :SDL_Keycode = 1073742028;
pub const SDLK_KPSPACE            :SDL_Keycode = 1073742029;
pub const SDLK_KPAT               :SDL_Keycode = 1073742030;
pub const SDLK_KPEXCLAM           :SDL_Keycode = 1073742031;
pub const SDLK_KPMEMSTORE         :SDL_Keycode = 1073742032;
pub const SDLK_KPMEMRECALL        :SDL_Keycode = 1073742033;
pub const SDLK_KPMEMCLEAR         :SDL_Keycode = 1073742034;
pub const SDLK_KPMEMADD           :SDL_Keycode = 1073742035;
pub const SDLK_KPMEMSUBTRACT      :SDL_Keycode = 1073742036;
pub const SDLK_KPMEMMULTIPLY      :SDL_Keycode = 1073742037;
pub const SDLK_KPMEMDIVIDE        :SDL_Keycode = 1073742038;
pub const SDLK_KPPLUSMINUS        :SDL_Keycode = 1073742039;
pub const SDLK_KPCEAR             :SDL_Keycode = 1073742040;
pub const SDLK_KPCLEARENTRY       :SDL_Keycode = 1073742041;
pub const SDLK_KPBINARY           :SDL_Keycode = 1073742042;
pub const SDLK_KPOCTAL            :SDL_Keycode = 1073742043;
pub const SDLK_KPDECIMAL          :SDL_Keycode = 1073742044;
pub const SDLK_KPHEXADECIMAL      :SDL_Keycode = 1073742045;
pub const SDLK_LCTRL              :SDL_Keycode = 1073742048;
pub const SDLK_LSHIFT             :SDL_Keycode = 1073742049;
pub const SDLK_LALT               :SDL_Keycode = 1073742050;
pub const SDLK_LGUI               :SDL_Keycode = 1073742051;
pub const SDLK_RCTRL              :SDL_Keycode = 1073742052;
pub const SDLK_RSHIFT             :SDL_Keycode = 1073742053;
pub const SDLK_RALT               :SDL_Keycode = 1073742054;
pub const SDLK_RGUI               :SDL_Keycode = 1073742055;
pub const SDLK_MODE               :SDL_Keycode = 1073742081;
pub const SDLK_AUDIONEXT          :SDL_Keycode = 1073742082;
pub const SDLK_AUDIOPREV          :SDL_Keycode = 1073742083;
pub const SDLK_AUDIOSTOP          :SDL_Keycode = 1073742084;
pub const SDLK_AUDIOPLAY          :SDL_Keycode = 1073742085;
pub const SDLK_AUDIOMUTE          :SDL_Keycode = 1073742086;
pub const SDLK_MEDIASELECT        :SDL_Keycode = 1073742087;
pub const SDLK_WWW                :SDL_Keycode = 1073742088;
pub const SDLK_MAIL               :SDL_Keycode = 1073742089;
pub const SDLK_CALCULATOR         :SDL_Keycode = 1073742090;
pub const SDLK_COMPUTER           :SDL_Keycode = 1073742091;
pub const SDLK_ACSEARCH           :SDL_Keycode = 1073742092;
pub const SDLK_ACHOME             :SDL_Keycode = 1073742093;
pub const SDLK_ACBACK             :SDL_Keycode = 1073742094;
pub const SDLK_ACFORWARD          :SDL_Keycode = 1073742095;
pub const SDLK_ACSTOP             :SDL_Keycode = 1073742096;
pub const SDLK_ACREFRESH          :SDL_Keycode = 1073742097;
pub const SDLK_ACBOOKMARKS        :SDL_Keycode = 1073742098;
pub const SDLK_BRIGHTNESSDOWN     :SDL_Keycode = 1073742099;
pub const SDLK_BRIGHTNESSUP       :SDL_Keycode = 1073742100;
pub const SDLK_DISPLAYSWITCH      :SDL_Keycode = 1073742101;
pub const SDLK_KBDILLUMTOGGLE     :SDL_Keycode = 1073742102;
pub const SDLK_KBDILLUMDOWN       :SDL_Keycode = 1073742103;
pub const SDLK_KBDILLUMUP         :SDL_Keycode = 1073742104;
pub const SDLK_EJECT              :SDL_Keycode = 1073742105;
pub const SDLK_SLEEP              :SDL_Keycode = 1073742106;

pub type SDL_Keymod = libc::c_uint;
pub const KMOD_NONE     :SDL_Keymod = 0x0000;
pub const KMOD_LSHIFT   :SDL_Keymod = 0x0001;
pub const KMOD_RSHIFT   :SDL_Keymod = 0x0002;
pub const KMOD_LCTRL    :SDL_Keymod = 0x0040;
pub const KMOD_RCTRL    :SDL_Keymod = 0x0080;
pub const KMOD_LALT     :SDL_Keymod = 0x0100;
pub const KMOD_RALT     :SDL_Keymod = 0x0200;
pub const KMOD_LGUI     :SDL_Keymod = 0x0400;
pub const KMOD_RGUI     :SDL_Keymod = 0x0800;
pub const KMOD_NUM      :SDL_Keymod = 0x1000;
pub const KMOD_CAPS     :SDL_Keymod = 0x2000;
pub const KMOD_MODE     :SDL_Keymod = 0x4000;
pub const KMOD_RESERVED :SDL_Keymod = 0x8000;


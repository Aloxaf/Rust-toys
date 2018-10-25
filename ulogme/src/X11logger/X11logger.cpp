//
// Created by aloxaf on 18-10-25.
//
// raw: https://github.com/UltimateHackingKeyboard/current-window-linux/blob/master/get-current-window.c

#include <X11/Xlib.h>
#include <X11/Xatom.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "common.h"

#define MAXSTR 1000

void check_status(int status, unsigned long window) {
    if (status == BadWindow) {
        throw "window id # 0x%lx does not exists!"; //  window);
    }

    if (status != Success) {
        throw "XGetWindowProperty failed!";
    }
}

unsigned char *get_string_property(Display *display, Window window, char const *property_name) {
    Atom actual_type, filter_atom;
    int actual_format, status;
    unsigned long nitems, bytes_after;
    unsigned char *prop;

    filter_atom = XInternAtom(display, property_name, True);
    status = XGetWindowProperty(display, window, filter_atom, 0, MAXSTR, False, AnyPropertyType,
                                &actual_type, &actual_format, &nitems, &bytes_after, &prop);
    check_status(status, window);
    return prop;
}

unsigned long get_long_property(Display *display, Window window, char const *property_name) {
    unsigned char *prop = get_string_property(display, window, property_name);
    unsigned long long_property = prop[0] + (prop[1] << 8) + (prop[2] << 16) + (prop[3] << 24);
    return long_property;
}

extern "C" DLL_PUBLIC WindowInfo get_active_window() {
    Display *display = XOpenDisplay(NULL);
    int screen = XDefaultScreen(display);
    Window window = RootWindow(display, screen);
    window = get_long_property(display, window, "_NET_ACTIVE_WINDOW");

    WindowInfo info;
    if (window == 0) {
        info.PID = 0;
        info.wm_class = NULL;
        info.wm_name = NULL;
    } else {
        info.PID = get_long_property(display, window, "_NET_WM_PID");
        info.wm_class = (char *) get_string_property(display, window, "WM_CLASS");
        info.wm_name = (char *) get_string_property(display, window, "_NET_WM_NAME");
    }
    XCloseDisplay(display);
    return info;
}

DLL_PUBLIC KeyEvent get_keyboard_event() {
    return (KeyEvent) {0, 0};
}